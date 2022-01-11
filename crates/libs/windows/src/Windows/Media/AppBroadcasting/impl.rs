#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAppBroadcastingMonitorImpl: Sized {
    fn IsCurrentAppBroadcasting(&self) -> ::windows::core::Result<bool>;
    fn IsCurrentAppBroadcastingChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppBroadcastingMonitor, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveIsCurrentAppBroadcastingChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppBroadcastingMonitor {
    const NAME: &'static str = "Windows.Media.AppBroadcasting.IAppBroadcastingMonitor";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAppBroadcastingMonitorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBroadcastingMonitorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppBroadcastingMonitorVtbl {
        unsafe extern "system" fn IsCurrentAppBroadcasting<Impl: IAppBroadcastingMonitorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCurrentAppBroadcasting() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCurrentAppBroadcastingChanged<Impl: IAppBroadcastingMonitorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCurrentAppBroadcastingChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<AppBroadcastingMonitor, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AppBroadcastingMonitor, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveIsCurrentAppBroadcastingChanged<Impl: IAppBroadcastingMonitorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveIsCurrentAppBroadcastingChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppBroadcastingMonitor, BASE_OFFSET>(),
            IsCurrentAppBroadcasting: IsCurrentAppBroadcasting::<Impl, IMPL_OFFSET>,
            IsCurrentAppBroadcastingChanged: IsCurrentAppBroadcastingChanged::<Impl, IMPL_OFFSET>,
            RemoveIsCurrentAppBroadcastingChanged: RemoveIsCurrentAppBroadcastingChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppBroadcastingMonitor as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastingStatusImpl: Sized {
    fn CanStartBroadcast(&self) -> ::windows::core::Result<bool>;
    fn Details(&self) -> ::windows::core::Result<AppBroadcastingStatusDetails>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBroadcastingStatus {
    const NAME: &'static str = "Windows.Media.AppBroadcasting.IAppBroadcastingStatus";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBroadcastingStatusVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBroadcastingStatusImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppBroadcastingStatusVtbl {
        unsafe extern "system" fn CanStartBroadcast<Impl: IAppBroadcastingStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanStartBroadcast() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Details<Impl: IAppBroadcastingStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Details() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppBroadcastingStatus, BASE_OFFSET>(),
            CanStartBroadcast: CanStartBroadcast::<Impl, IMPL_OFFSET>,
            Details: Details::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppBroadcastingStatus as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastingStatusDetailsImpl: Sized {
    fn IsAnyAppBroadcasting(&self) -> ::windows::core::Result<bool>;
    fn IsCaptureResourceUnavailable(&self) -> ::windows::core::Result<bool>;
    fn IsGameStreamInProgress(&self) -> ::windows::core::Result<bool>;
    fn IsGpuConstrained(&self) -> ::windows::core::Result<bool>;
    fn IsAppInactive(&self) -> ::windows::core::Result<bool>;
    fn IsBlockedForApp(&self) -> ::windows::core::Result<bool>;
    fn IsDisabledByUser(&self) -> ::windows::core::Result<bool>;
    fn IsDisabledBySystem(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBroadcastingStatusDetails {
    const NAME: &'static str = "Windows.Media.AppBroadcasting.IAppBroadcastingStatusDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBroadcastingStatusDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBroadcastingStatusDetailsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppBroadcastingStatusDetailsVtbl {
        unsafe extern "system" fn IsAnyAppBroadcasting<Impl: IAppBroadcastingStatusDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAnyAppBroadcasting() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCaptureResourceUnavailable<Impl: IAppBroadcastingStatusDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCaptureResourceUnavailable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsGameStreamInProgress<Impl: IAppBroadcastingStatusDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsGameStreamInProgress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsGpuConstrained<Impl: IAppBroadcastingStatusDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsGpuConstrained() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAppInactive<Impl: IAppBroadcastingStatusDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAppInactive() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsBlockedForApp<Impl: IAppBroadcastingStatusDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsBlockedForApp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDisabledByUser<Impl: IAppBroadcastingStatusDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDisabledByUser() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDisabledBySystem<Impl: IAppBroadcastingStatusDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDisabledBySystem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppBroadcastingStatusDetails, BASE_OFFSET>(),
            IsAnyAppBroadcasting: IsAnyAppBroadcasting::<Impl, IMPL_OFFSET>,
            IsCaptureResourceUnavailable: IsCaptureResourceUnavailable::<Impl, IMPL_OFFSET>,
            IsGameStreamInProgress: IsGameStreamInProgress::<Impl, IMPL_OFFSET>,
            IsGpuConstrained: IsGpuConstrained::<Impl, IMPL_OFFSET>,
            IsAppInactive: IsAppInactive::<Impl, IMPL_OFFSET>,
            IsBlockedForApp: IsBlockedForApp::<Impl, IMPL_OFFSET>,
            IsDisabledByUser: IsDisabledByUser::<Impl, IMPL_OFFSET>,
            IsDisabledBySystem: IsDisabledBySystem::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppBroadcastingStatusDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastingUIImpl: Sized {
    fn GetStatus(&self) -> ::windows::core::Result<AppBroadcastingStatus>;
    fn ShowBroadcastUI(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBroadcastingUI {
    const NAME: &'static str = "Windows.Media.AppBroadcasting.IAppBroadcastingUI";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBroadcastingUIVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBroadcastingUIImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppBroadcastingUIVtbl {
        unsafe extern "system" fn GetStatus<Impl: IAppBroadcastingUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowBroadcastUI<Impl: IAppBroadcastingUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowBroadcastUI().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppBroadcastingUI, BASE_OFFSET>(),
            GetStatus: GetStatus::<Impl, IMPL_OFFSET>,
            ShowBroadcastUI: ShowBroadcastUI::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppBroadcastingUI as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
pub trait IAppBroadcastingUIStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<AppBroadcastingUI>;
    fn GetForUser(&self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<AppBroadcastingUI>;
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppBroadcastingUIStatics {
    const NAME: &'static str = "Windows.Media.AppBroadcasting.IAppBroadcastingUIStatics";
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl IAppBroadcastingUIStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBroadcastingUIStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppBroadcastingUIStaticsVtbl {
        unsafe extern "system" fn GetDefault<Impl: IAppBroadcastingUIStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetForUser<Impl: IAppBroadcastingUIStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForUser(&*(&user as *const <super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::System::User as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppBroadcastingUIStatics, BASE_OFFSET>(),
            GetDefault: GetDefault::<Impl, IMPL_OFFSET>,
            GetForUser: GetForUser::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppBroadcastingUIStatics as ::windows::core::Interface>::IID
    }
}
