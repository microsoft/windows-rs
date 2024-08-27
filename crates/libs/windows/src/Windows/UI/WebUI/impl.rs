pub trait IActivatedEventArgsDeferral_Impl: Sized + windows_core::IUnknownImpl {
    fn ActivatedOperation(&self) -> windows_core::Result<ActivatedOperation>;
}
impl windows_core::RuntimeName for IActivatedEventArgsDeferral {
    const NAME: &'static str = "Windows.UI.WebUI.IActivatedEventArgsDeferral";
}
impl IActivatedEventArgsDeferral_Vtbl {
    pub const fn new<Identity: IActivatedEventArgsDeferral_Impl, const OFFSET: isize>() -> IActivatedEventArgsDeferral_Vtbl {
        unsafe extern "system" fn ActivatedOperation<Identity: IActivatedEventArgsDeferral_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IActivatedEventArgsDeferral_Impl::ActivatedOperation(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IActivatedEventArgsDeferral, OFFSET>(),
            ActivatedOperation: ActivatedOperation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActivatedEventArgsDeferral as windows_core::Interface>::IID
    }
}
pub trait IWebUIBackgroundTaskInstance_Impl: Sized + windows_core::IUnknownImpl {
    fn Succeeded(&self) -> windows_core::Result<bool>;
    fn SetSucceeded(&self, succeeded: bool) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWebUIBackgroundTaskInstance {
    const NAME: &'static str = "Windows.UI.WebUI.IWebUIBackgroundTaskInstance";
}
impl IWebUIBackgroundTaskInstance_Vtbl {
    pub const fn new<Identity: IWebUIBackgroundTaskInstance_Impl, const OFFSET: isize>() -> IWebUIBackgroundTaskInstance_Vtbl {
        unsafe extern "system" fn Succeeded<Identity: IWebUIBackgroundTaskInstance_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWebUIBackgroundTaskInstance_Impl::Succeeded(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSucceeded<Identity: IWebUIBackgroundTaskInstance_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, succeeded: bool) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWebUIBackgroundTaskInstance_Impl::SetSucceeded(this, succeeded).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IWebUIBackgroundTaskInstance, OFFSET>(),
            Succeeded: Succeeded::<Identity, OFFSET>,
            SetSucceeded: SetSucceeded::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWebUIBackgroundTaskInstance as windows_core::Interface>::IID
    }
}
pub trait IWebUINavigatedEventArgs_Impl: Sized + windows_core::IUnknownImpl {
    fn NavigatedOperation(&self) -> windows_core::Result<WebUINavigatedOperation>;
}
impl windows_core::RuntimeName for IWebUINavigatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.IWebUINavigatedEventArgs";
}
impl IWebUINavigatedEventArgs_Vtbl {
    pub const fn new<Identity: IWebUINavigatedEventArgs_Impl, const OFFSET: isize>() -> IWebUINavigatedEventArgs_Vtbl {
        unsafe extern "system" fn NavigatedOperation<Identity: IWebUINavigatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWebUINavigatedEventArgs_Impl::NavigatedOperation(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IWebUINavigatedEventArgs, OFFSET>(),
            NavigatedOperation: NavigatedOperation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWebUINavigatedEventArgs as windows_core::Interface>::IID
    }
}
