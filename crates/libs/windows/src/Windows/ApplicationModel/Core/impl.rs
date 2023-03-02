#[doc = "*Required features: `\"ApplicationModel_Core\"`, `\"Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Foundation")]
pub trait ICoreApplicationUnhandledError_Impl: Sized {
    fn UnhandledErrorDetected(&self, handler: ::core::option::Option<&super::super::Foundation::EventHandler<UnhandledErrorDetectedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUnhandledErrorDetected(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for ICoreApplicationUnhandledError {
    const NAME: &'static str = "Windows.ApplicationModel.Core.ICoreApplicationUnhandledError";
}
#[cfg(feature = "Foundation")]
impl ICoreApplicationUnhandledError_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreApplicationUnhandledError_Impl, const OFFSET: isize>() -> ICoreApplicationUnhandledError_Vtbl {
        unsafe extern "system" fn UnhandledErrorDetected<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreApplicationUnhandledError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UnhandledErrorDetected(::windows::core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveUnhandledErrorDetected<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreApplicationUnhandledError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveUnhandledErrorDetected(::core::mem::transmute(&token)).into()
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, ICoreApplicationUnhandledError, OFFSET>(),
            UnhandledErrorDetected: UnhandledErrorDetected::<Identity, Impl, OFFSET>,
            RemoveUnhandledErrorDetected: RemoveUnhandledErrorDetected::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreApplicationUnhandledError as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel_Core\"`, `\"UI_Core\"`, `\"implement\"`*"]
#[cfg(feature = "UI_Core")]
pub trait IFrameworkView_Impl: Sized {
    fn Initialize(&self, applicationview: ::core::option::Option<&CoreApplicationView>) -> ::windows::core::Result<()>;
    fn SetWindow(&self, window: ::core::option::Option<&super::super::UI::Core::CoreWindow>) -> ::windows::core::Result<()>;
    fn Load(&self, entrypoint: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Run(&self) -> ::windows::core::Result<()>;
    fn Uninitialize(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "UI_Core")]
impl ::windows::core::RuntimeName for IFrameworkView {
    const NAME: &'static str = "Windows.ApplicationModel.Core.IFrameworkView";
}
#[cfg(feature = "UI_Core")]
impl IFrameworkView_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFrameworkView_Impl, const OFFSET: isize>() -> IFrameworkView_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFrameworkView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, applicationview: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::windows::core::from_raw_borrowed(&applicationview)).into()
        }
        unsafe extern "system" fn SetWindow<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFrameworkView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, window: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetWindow(::windows::core::from_raw_borrowed(&window)).into()
        }
        unsafe extern "system" fn Load<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFrameworkView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, entrypoint: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Load(::core::mem::transmute(&entrypoint)).into()
        }
        unsafe extern "system" fn Run<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFrameworkView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Run().into()
        }
        unsafe extern "system" fn Uninitialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFrameworkView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Uninitialize().into()
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IFrameworkView, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            SetWindow: SetWindow::<Identity, Impl, OFFSET>,
            Load: Load::<Identity, Impl, OFFSET>,
            Run: Run::<Identity, Impl, OFFSET>,
            Uninitialize: Uninitialize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFrameworkView as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel_Core\"`, `\"implement\"`*"]
pub trait IFrameworkViewSource_Impl: Sized {
    fn CreateView(&self) -> ::windows::core::Result<IFrameworkView>;
}
impl ::windows::core::RuntimeName for IFrameworkViewSource {
    const NAME: &'static str = "Windows.ApplicationModel.Core.IFrameworkViewSource";
}
impl IFrameworkViewSource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFrameworkViewSource_Impl, const OFFSET: isize>() -> IFrameworkViewSource_Vtbl {
        unsafe extern "system" fn CreateView<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFrameworkViewSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateView() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IFrameworkViewSource, OFFSET>(), CreateView: CreateView::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFrameworkViewSource as ::windows::core::ComInterface>::IID
    }
}
