#[cfg(feature = "Foundation")]
pub trait ICoreApplicationUnhandledError_Impl: Sized {
    fn UnhandledErrorDetected(&mut self, handler: &::core::option::Option<super::super::Foundation::EventHandler<UnhandledErrorDetectedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUnhandledErrorDetected(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for ICoreApplicationUnhandledError {
    const NAME: &'static str = "Windows.ApplicationModel.Core.ICoreApplicationUnhandledError";
}
#[cfg(feature = "Foundation")]
impl ICoreApplicationUnhandledError_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreApplicationUnhandledError_Impl, const OFFSET: isize>() -> ICoreApplicationUnhandledError_Vtbl {
        unsafe extern "system" fn UnhandledErrorDetected<Identity: ::windows::core::IUnknownImpl, Impl: ICoreApplicationUnhandledError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UnhandledErrorDetected(&*(&handler as *const <super::super::Foundation::EventHandler<UnhandledErrorDetectedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<UnhandledErrorDetectedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveUnhandledErrorDetected<Identity: ::windows::core::IUnknownImpl, Impl: ICoreApplicationUnhandledError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveUnhandledErrorDetected(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreApplicationUnhandledError, OFFSET>(),
            UnhandledErrorDetected: UnhandledErrorDetected::<Identity, Impl, OFFSET>,
            RemoveUnhandledErrorDetected: RemoveUnhandledErrorDetected::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreApplicationUnhandledError as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "UI_Core")]
pub trait IFrameworkView_Impl: Sized {
    fn Initialize(&mut self, applicationview: &::core::option::Option<CoreApplicationView>) -> ::windows::core::Result<()>;
    fn SetWindow(&mut self, window: &::core::option::Option<super::super::UI::Core::CoreWindow>) -> ::windows::core::Result<()>;
    fn Load(&mut self, entrypoint: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Run(&mut self) -> ::windows::core::Result<()>;
    fn Uninitialize(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "UI_Core")]
impl ::windows::core::RuntimeName for IFrameworkView {
    const NAME: &'static str = "Windows.ApplicationModel.Core.IFrameworkView";
}
#[cfg(feature = "UI_Core")]
impl IFrameworkView_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFrameworkView_Impl, const OFFSET: isize>() -> IFrameworkView_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl, Impl: IFrameworkView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, applicationview: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Initialize(&*(&applicationview as *const <CoreApplicationView as ::windows::core::Abi>::Abi as *const <CoreApplicationView as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetWindow<Identity: ::windows::core::IUnknownImpl, Impl: IFrameworkView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, window: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetWindow(&*(&window as *const <super::super::UI::Core::CoreWindow as ::windows::core::Abi>::Abi as *const <super::super::UI::Core::CoreWindow as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Load<Identity: ::windows::core::IUnknownImpl, Impl: IFrameworkView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, entrypoint: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Load(&*(&entrypoint as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Run<Identity: ::windows::core::IUnknownImpl, Impl: IFrameworkView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Run().into()
        }
        unsafe extern "system" fn Uninitialize<Identity: ::windows::core::IUnknownImpl, Impl: IFrameworkView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Uninitialize().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFrameworkView, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            SetWindow: SetWindow::<Identity, Impl, OFFSET>,
            Load: Load::<Identity, Impl, OFFSET>,
            Run: Run::<Identity, Impl, OFFSET>,
            Uninitialize: Uninitialize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFrameworkView as ::windows::core::Interface>::IID
    }
}
pub trait IFrameworkViewSource_Impl: Sized {
    fn CreateView(&mut self) -> ::windows::core::Result<IFrameworkView>;
}
impl ::windows::core::RuntimeName for IFrameworkViewSource {
    const NAME: &'static str = "Windows.ApplicationModel.Core.IFrameworkViewSource";
}
impl IFrameworkViewSource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFrameworkViewSource_Impl, const OFFSET: isize>() -> IFrameworkViewSource_Vtbl {
        unsafe extern "system" fn CreateView<Identity: ::windows::core::IUnknownImpl, Impl: IFrameworkViewSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IFrameworkViewSource, OFFSET>(), CreateView: CreateView::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFrameworkViewSource as ::windows::core::Interface>::IID
    }
}
