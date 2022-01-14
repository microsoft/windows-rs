#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAppBroadcastingMonitor_Impl: Sized {
    fn IsCurrentAppBroadcasting(&mut self) -> ::windows::core::Result<bool>;
    fn IsCurrentAppBroadcastingChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppBroadcastingMonitor, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveIsCurrentAppBroadcastingChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppBroadcastingMonitor {
    const NAME: &'static str = "Windows.Media.AppBroadcasting.IAppBroadcastingMonitor";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAppBroadcastingMonitor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBroadcastingMonitor_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppBroadcastingMonitor_Vtbl {
        unsafe extern "system" fn IsCurrentAppBroadcasting<Impl: IAppBroadcastingMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsCurrentAppBroadcastingChanged<Impl: IAppBroadcastingMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveIsCurrentAppBroadcastingChanged<Impl: IAppBroadcastingMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
pub trait IAppBroadcastingStatus_Impl: Sized {
    fn CanStartBroadcast(&mut self) -> ::windows::core::Result<bool>;
    fn Details(&mut self) -> ::windows::core::Result<AppBroadcastingStatusDetails>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBroadcastingStatus {
    const NAME: &'static str = "Windows.Media.AppBroadcasting.IAppBroadcastingStatus";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBroadcastingStatus_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBroadcastingStatus_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppBroadcastingStatus_Vtbl {
        unsafe extern "system" fn CanStartBroadcast<Impl: IAppBroadcastingStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Details<Impl: IAppBroadcastingStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IAppBroadcastingStatusDetails_Impl: Sized {
    fn IsAnyAppBroadcasting(&mut self) -> ::windows::core::Result<bool>;
    fn IsCaptureResourceUnavailable(&mut self) -> ::windows::core::Result<bool>;
    fn IsGameStreamInProgress(&mut self) -> ::windows::core::Result<bool>;
    fn IsGpuConstrained(&mut self) -> ::windows::core::Result<bool>;
    fn IsAppInactive(&mut self) -> ::windows::core::Result<bool>;
    fn IsBlockedForApp(&mut self) -> ::windows::core::Result<bool>;
    fn IsDisabledByUser(&mut self) -> ::windows::core::Result<bool>;
    fn IsDisabledBySystem(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBroadcastingStatusDetails {
    const NAME: &'static str = "Windows.Media.AppBroadcasting.IAppBroadcastingStatusDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBroadcastingStatusDetails_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBroadcastingStatusDetails_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppBroadcastingStatusDetails_Vtbl {
        unsafe extern "system" fn IsAnyAppBroadcasting<Impl: IAppBroadcastingStatusDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsCaptureResourceUnavailable<Impl: IAppBroadcastingStatusDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsGameStreamInProgress<Impl: IAppBroadcastingStatusDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsGpuConstrained<Impl: IAppBroadcastingStatusDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsAppInactive<Impl: IAppBroadcastingStatusDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsBlockedForApp<Impl: IAppBroadcastingStatusDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsDisabledByUser<Impl: IAppBroadcastingStatusDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsDisabledBySystem<Impl: IAppBroadcastingStatusDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
pub trait IAppBroadcastingUI_Impl: Sized {
    fn GetStatus(&mut self) -> ::windows::core::Result<AppBroadcastingStatus>;
    fn ShowBroadcastUI(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBroadcastingUI {
    const NAME: &'static str = "Windows.Media.AppBroadcasting.IAppBroadcastingUI";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBroadcastingUI_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBroadcastingUI_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppBroadcastingUI_Vtbl {
        unsafe extern "system" fn GetStatus<Impl: IAppBroadcastingUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ShowBroadcastUI<Impl: IAppBroadcastingUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait IAppBroadcastingUIStatics_Impl: Sized {
    fn GetDefault(&mut self) -> ::windows::core::Result<AppBroadcastingUI>;
    fn GetForUser(&mut self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<AppBroadcastingUI>;
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppBroadcastingUIStatics {
    const NAME: &'static str = "Windows.Media.AppBroadcasting.IAppBroadcastingUIStatics";
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl IAppBroadcastingUIStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBroadcastingUIStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppBroadcastingUIStatics_Vtbl {
        unsafe extern "system" fn GetDefault<Impl: IAppBroadcastingUIStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetForUser<Impl: IAppBroadcastingUIStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
