#[cfg(all(feature = "System", feature = "UI_Core", feature = "implement_exclusive"))]
pub trait IWebViewControlAcceleratorKeyPressedEventArgsImpl: Sized {
    fn EventType(&self) -> ::windows::core::Result<super::super::super::UI::Core::CoreAcceleratorKeyEventType>;
    fn VirtualKey(&self) -> ::windows::core::Result<super::super::super::System::VirtualKey>;
    fn KeyStatus(&self) -> ::windows::core::Result<super::super::super::UI::Core::CorePhysicalKeyStatus>;
    fn RoutingStage(&self) -> ::windows::core::Result<WebViewControlAcceleratorKeyRoutingStage>;
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "System", feature = "UI_Core", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebViewControlAcceleratorKeyPressedEventArgs {
    const NAME: &'static str = "Windows.Web.UI.Interop.IWebViewControlAcceleratorKeyPressedEventArgs";
}
#[cfg(all(feature = "System", feature = "UI_Core", feature = "implement_exclusive"))]
impl IWebViewControlAcceleratorKeyPressedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControlAcceleratorKeyPressedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebViewControlAcceleratorKeyPressedEventArgsVtbl {
        unsafe extern "system" fn EventType<Impl: IWebViewControlAcceleratorKeyPressedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::UI::Core::CoreAcceleratorKeyEventType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EventType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VirtualKey<Impl: IWebViewControlAcceleratorKeyPressedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::System::VirtualKey) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VirtualKey() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyStatus<Impl: IWebViewControlAcceleratorKeyPressedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::UI::Core::CorePhysicalKeyStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoutingStage<Impl: IWebViewControlAcceleratorKeyPressedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut WebViewControlAcceleratorKeyRoutingStage) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RoutingStage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handled<Impl: IWebViewControlAcceleratorKeyPressedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetHandled<Impl: IWebViewControlAcceleratorKeyPressedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebViewControlAcceleratorKeyPressedEventArgs, BASE_OFFSET>(),
            EventType: EventType::<Impl, IMPL_OFFSET>,
            VirtualKey: VirtualKey::<Impl, IMPL_OFFSET>,
            KeyStatus: KeyStatus::<Impl, IMPL_OFFSET>,
            RoutingStage: RoutingStage::<Impl, IMPL_OFFSET>,
            Handled: Handled::<Impl, IMPL_OFFSET>,
            SetHandled: SetHandled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebViewControlAcceleratorKeyPressedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebViewControlMoveFocusRequestedEventArgsImpl: Sized {
    fn Reason(&self) -> ::windows::core::Result<WebViewControlMoveFocusReason>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebViewControlMoveFocusRequestedEventArgs {
    const NAME: &'static str = "Windows.Web.UI.Interop.IWebViewControlMoveFocusRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IWebViewControlMoveFocusRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControlMoveFocusRequestedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebViewControlMoveFocusRequestedEventArgsVtbl {
        unsafe extern "system" fn Reason<Impl: IWebViewControlMoveFocusRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut WebViewControlMoveFocusReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reason() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebViewControlMoveFocusRequestedEventArgs, BASE_OFFSET>(),
            Reason: Reason::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebViewControlMoveFocusRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IWebViewControlProcessImpl: Sized {
    fn ProcessId(&self) -> ::windows::core::Result<u32>;
    fn EnterpriseId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsPrivateNetworkClientServerCapabilityEnabled(&self) -> ::windows::core::Result<bool>;
    fn CreateWebViewControlAsync(&self, hostwindowhandle: i64, bounds: &super::super::super::Foundation::Rect) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<WebViewControl>>;
    fn GetWebViewControls(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<WebViewControl>>;
    fn Terminate(&self) -> ::windows::core::Result<()>;
    fn ProcessExited(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<WebViewControlProcess, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveProcessExited(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebViewControlProcess {
    const NAME: &'static str = "Windows.Web.UI.Interop.IWebViewControlProcess";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IWebViewControlProcessVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControlProcessImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebViewControlProcessVtbl {
        unsafe extern "system" fn ProcessId<Impl: IWebViewControlProcessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProcessId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnterpriseId<Impl: IWebViewControlProcessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnterpriseId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPrivateNetworkClientServerCapabilityEnabled<Impl: IWebViewControlProcessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPrivateNetworkClientServerCapabilityEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWebViewControlAsync<Impl: IWebViewControlProcessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hostwindowhandle: i64, bounds: super::super::super::Foundation::Rect, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWebViewControlAsync(hostwindowhandle, &*(&bounds as *const <super::super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWebViewControls<Impl: IWebViewControlProcessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWebViewControls() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Terminate<Impl: IWebViewControlProcessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Terminate().into()
        }
        unsafe extern "system" fn ProcessExited<Impl: IWebViewControlProcessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProcessExited(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<WebViewControlProcess, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<WebViewControlProcess, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveProcessExited<Impl: IWebViewControlProcessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveProcessExited(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebViewControlProcess, BASE_OFFSET>(),
            ProcessId: ProcessId::<Impl, IMPL_OFFSET>,
            EnterpriseId: EnterpriseId::<Impl, IMPL_OFFSET>,
            IsPrivateNetworkClientServerCapabilityEnabled: IsPrivateNetworkClientServerCapabilityEnabled::<Impl, IMPL_OFFSET>,
            CreateWebViewControlAsync: CreateWebViewControlAsync::<Impl, IMPL_OFFSET>,
            GetWebViewControls: GetWebViewControls::<Impl, IMPL_OFFSET>,
            Terminate: Terminate::<Impl, IMPL_OFFSET>,
            ProcessExited: ProcessExited::<Impl, IMPL_OFFSET>,
            RemoveProcessExited: RemoveProcessExited::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebViewControlProcess as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebViewControlProcessFactoryImpl: Sized {
    fn CreateWithOptions(&self, processoptions: &::core::option::Option<WebViewControlProcessOptions>) -> ::windows::core::Result<WebViewControlProcess>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebViewControlProcessFactory {
    const NAME: &'static str = "Windows.Web.UI.Interop.IWebViewControlProcessFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IWebViewControlProcessFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControlProcessFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebViewControlProcessFactoryVtbl {
        unsafe extern "system" fn CreateWithOptions<Impl: IWebViewControlProcessFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, processoptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithOptions(&*(&processoptions as *const <WebViewControlProcessOptions as ::windows::core::Abi>::Abi as *const <WebViewControlProcessOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebViewControlProcessFactory, BASE_OFFSET>(),
            CreateWithOptions: CreateWithOptions::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebViewControlProcessFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebViewControlProcessOptionsImpl: Sized {
    fn SetEnterpriseId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn EnterpriseId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPrivateNetworkClientServerCapability(&self, value: WebViewControlProcessCapabilityState) -> ::windows::core::Result<()>;
    fn PrivateNetworkClientServerCapability(&self) -> ::windows::core::Result<WebViewControlProcessCapabilityState>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebViewControlProcessOptions {
    const NAME: &'static str = "Windows.Web.UI.Interop.IWebViewControlProcessOptions";
}
#[cfg(feature = "implement_exclusive")]
impl IWebViewControlProcessOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControlProcessOptionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebViewControlProcessOptionsVtbl {
        unsafe extern "system" fn SetEnterpriseId<Impl: IWebViewControlProcessOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnterpriseId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EnterpriseId<Impl: IWebViewControlProcessOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnterpriseId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivateNetworkClientServerCapability<Impl: IWebViewControlProcessOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: WebViewControlProcessCapabilityState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrivateNetworkClientServerCapability(value).into()
        }
        unsafe extern "system" fn PrivateNetworkClientServerCapability<Impl: IWebViewControlProcessOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut WebViewControlProcessCapabilityState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrivateNetworkClientServerCapability() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebViewControlProcessOptions, BASE_OFFSET>(),
            SetEnterpriseId: SetEnterpriseId::<Impl, IMPL_OFFSET>,
            EnterpriseId: EnterpriseId::<Impl, IMPL_OFFSET>,
            SetPrivateNetworkClientServerCapability: SetPrivateNetworkClientServerCapability::<Impl, IMPL_OFFSET>,
            PrivateNetworkClientServerCapability: PrivateNetworkClientServerCapability::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebViewControlProcessOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IWebViewControlSiteImpl: Sized {
    fn Process(&self) -> ::windows::core::Result<WebViewControlProcess>;
    fn SetScale(&self, value: f64) -> ::windows::core::Result<()>;
    fn Scale(&self) -> ::windows::core::Result<f64>;
    fn SetBounds(&self, value: &super::super::super::Foundation::Rect) -> ::windows::core::Result<()>;
    fn Bounds(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
    fn SetIsVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsVisible(&self) -> ::windows::core::Result<bool>;
    fn Close(&self) -> ::windows::core::Result<()>;
    fn MoveFocus(&self, reason: WebViewControlMoveFocusReason) -> ::windows::core::Result<()>;
    fn MoveFocusRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<WebViewControl, WebViewControlMoveFocusRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveMoveFocusRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AcceleratorKeyPressed(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<WebViewControl, WebViewControlAcceleratorKeyPressedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveAcceleratorKeyPressed(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebViewControlSite {
    const NAME: &'static str = "Windows.Web.UI.Interop.IWebViewControlSite";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IWebViewControlSiteVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControlSiteImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebViewControlSiteVtbl {
        unsafe extern "system" fn Process<Impl: IWebViewControlSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Process() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScale<Impl: IWebViewControlSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScale(value).into()
        }
        unsafe extern "system" fn Scale<Impl: IWebViewControlSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Scale() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBounds<Impl: IWebViewControlSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBounds(&*(&value as *const <super::super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Bounds<Impl: IWebViewControlSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Bounds() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsVisible<Impl: IWebViewControlSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsVisible(value).into()
        }
        unsafe extern "system" fn IsVisible<Impl: IWebViewControlSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsVisible() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: IWebViewControlSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        unsafe extern "system" fn MoveFocus<Impl: IWebViewControlSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reason: WebViewControlMoveFocusReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MoveFocus(reason).into()
        }
        unsafe extern "system" fn MoveFocusRequested<Impl: IWebViewControlSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveFocusRequested(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<WebViewControl, WebViewControlMoveFocusRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<WebViewControl, WebViewControlMoveFocusRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMoveFocusRequested<Impl: IWebViewControlSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMoveFocusRequested(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AcceleratorKeyPressed<Impl: IWebViewControlSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AcceleratorKeyPressed(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<WebViewControl, WebViewControlAcceleratorKeyPressedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<WebViewControl, WebViewControlAcceleratorKeyPressedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAcceleratorKeyPressed<Impl: IWebViewControlSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAcceleratorKeyPressed(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebViewControlSite, BASE_OFFSET>(),
            Process: Process::<Impl, IMPL_OFFSET>,
            SetScale: SetScale::<Impl, IMPL_OFFSET>,
            Scale: Scale::<Impl, IMPL_OFFSET>,
            SetBounds: SetBounds::<Impl, IMPL_OFFSET>,
            Bounds: Bounds::<Impl, IMPL_OFFSET>,
            SetIsVisible: SetIsVisible::<Impl, IMPL_OFFSET>,
            IsVisible: IsVisible::<Impl, IMPL_OFFSET>,
            Close: Close::<Impl, IMPL_OFFSET>,
            MoveFocus: MoveFocus::<Impl, IMPL_OFFSET>,
            MoveFocusRequested: MoveFocusRequested::<Impl, IMPL_OFFSET>,
            RemoveMoveFocusRequested: RemoveMoveFocusRequested::<Impl, IMPL_OFFSET>,
            AcceleratorKeyPressed: AcceleratorKeyPressed::<Impl, IMPL_OFFSET>,
            RemoveAcceleratorKeyPressed: RemoveAcceleratorKeyPressed::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebViewControlSite as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IWebViewControlSite2Impl: Sized {
    fn GotFocus(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<WebViewControl, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveGotFocus(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn LostFocus(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<WebViewControl, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveLostFocus(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebViewControlSite2 {
    const NAME: &'static str = "Windows.Web.UI.Interop.IWebViewControlSite2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IWebViewControlSite2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControlSite2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebViewControlSite2Vtbl {
        unsafe extern "system" fn GotFocus<Impl: IWebViewControlSite2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GotFocus(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<WebViewControl, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<WebViewControl, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveGotFocus<Impl: IWebViewControlSite2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveGotFocus(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LostFocus<Impl: IWebViewControlSite2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LostFocus(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<WebViewControl, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<WebViewControl, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveLostFocus<Impl: IWebViewControlSite2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveLostFocus(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebViewControlSite2, BASE_OFFSET>(),
            GotFocus: GotFocus::<Impl, IMPL_OFFSET>,
            RemoveGotFocus: RemoveGotFocus::<Impl, IMPL_OFFSET>,
            LostFocus: LostFocus::<Impl, IMPL_OFFSET>,
            RemoveLostFocus: RemoveLostFocus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebViewControlSite2 as ::windows::core::Interface>::IID
    }
}
