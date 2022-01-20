#[cfg(all(feature = "Foundation", feature = "System"))]
pub trait IGameController_Impl: Sized {
    fn HeadsetConnected(&mut self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<IGameController, Headset>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveHeadsetConnected(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn HeadsetDisconnected(&mut self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<IGameController, Headset>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveHeadsetDisconnected(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn UserChanged(&mut self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<IGameController, super::super::System::UserChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUserChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Headset(&mut self) -> ::windows::core::Result<Headset>;
    fn IsWireless(&mut self) -> ::windows::core::Result<bool>;
    fn User(&mut self) -> ::windows::core::Result<super::super::System::User>;
}
#[cfg(all(feature = "Foundation", feature = "System"))]
impl ::windows::core::RuntimeName for IGameController {
    const NAME: &'static str = "Windows.Gaming.Input.IGameController";
}
#[cfg(all(feature = "Foundation", feature = "System"))]
impl IGameController_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameController_Impl, const OFFSET: isize>() -> IGameController_Vtbl {
        unsafe extern "system" fn HeadsetConnected<Identity: ::windows::core::IUnknownImpl, Impl: IGameController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).HeadsetConnected(&*(&value as *const <super::super::Foundation::TypedEventHandler<IGameController, Headset> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<IGameController, Headset> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveHeadsetConnected<Identity: ::windows::core::IUnknownImpl, Impl: IGameController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveHeadsetConnected(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn HeadsetDisconnected<Identity: ::windows::core::IUnknownImpl, Impl: IGameController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).HeadsetDisconnected(&*(&value as *const <super::super::Foundation::TypedEventHandler<IGameController, Headset> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<IGameController, Headset> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveHeadsetDisconnected<Identity: ::windows::core::IUnknownImpl, Impl: IGameController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveHeadsetDisconnected(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UserChanged<Identity: ::windows::core::IUnknownImpl, Impl: IGameController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UserChanged(&*(&value as *const <super::super::Foundation::TypedEventHandler<IGameController, super::super::System::UserChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<IGameController, super::super::System::UserChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveUserChanged<Identity: ::windows::core::IUnknownImpl, Impl: IGameController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveUserChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Headset<Identity: ::windows::core::IUnknownImpl, Impl: IGameController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Headset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsWireless<Identity: ::windows::core::IUnknownImpl, Impl: IGameController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsWireless() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn User<Identity: ::windows::core::IUnknownImpl, Impl: IGameController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).User() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGameController, OFFSET>(),
            HeadsetConnected: HeadsetConnected::<Identity, Impl, OFFSET>,
            RemoveHeadsetConnected: RemoveHeadsetConnected::<Identity, Impl, OFFSET>,
            HeadsetDisconnected: HeadsetDisconnected::<Identity, Impl, OFFSET>,
            RemoveHeadsetDisconnected: RemoveHeadsetDisconnected::<Identity, Impl, OFFSET>,
            UserChanged: UserChanged::<Identity, Impl, OFFSET>,
            RemoveUserChanged: RemoveUserChanged::<Identity, Impl, OFFSET>,
            Headset: Headset::<Identity, Impl, OFFSET>,
            IsWireless: IsWireless::<Identity, Impl, OFFSET>,
            User: User::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGameController as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Devices_Power")]
pub trait IGameControllerBatteryInfo_Impl: Sized {
    fn TryGetBatteryReport(&mut self) -> ::windows::core::Result<super::super::Devices::Power::BatteryReport>;
}
#[cfg(feature = "Devices_Power")]
impl ::windows::core::RuntimeName for IGameControllerBatteryInfo {
    const NAME: &'static str = "Windows.Gaming.Input.IGameControllerBatteryInfo";
}
#[cfg(feature = "Devices_Power")]
impl IGameControllerBatteryInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameControllerBatteryInfo_Impl, const OFFSET: isize>() -> IGameControllerBatteryInfo_Vtbl {
        unsafe extern "system" fn TryGetBatteryReport<Identity: ::windows::core::IUnknownImpl, Impl: IGameControllerBatteryInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TryGetBatteryReport() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGameControllerBatteryInfo, OFFSET>(),
            TryGetBatteryReport: TryGetBatteryReport::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGameControllerBatteryInfo as ::windows::core::Interface>::IID
    }
}
