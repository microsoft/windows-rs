#[cfg(all(feature = "Foundation", feature = "System"))]
pub trait IGameController_Impl: Sized {
    fn HeadsetConnected(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<IGameController, Headset>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveHeadsetConnected(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn HeadsetDisconnected(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<IGameController, Headset>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveHeadsetDisconnected(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn UserChanged(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<IGameController, super::super::System::UserChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUserChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Headset(&self) -> ::windows::core::Result<Headset>;
    fn IsWireless(&self) -> ::windows::core::Result<bool>;
    fn User(&self) -> ::windows::core::Result<super::super::System::User>;
}
#[cfg(all(feature = "Foundation", feature = "System"))]
impl ::windows::core::RuntimeName for IGameController {
    const NAME: &'static str = "Windows.Gaming.Input.IGameController";
}
#[cfg(all(feature = "Foundation", feature = "System"))]
impl IGameController_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGameController_Impl, const OFFSET: isize>() -> IGameController_Vtbl {
        unsafe extern "system" fn HeadsetConnected<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGameController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HeadsetConnected(::core::mem::transmute(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveHeadsetConnected<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGameController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveHeadsetConnected(::core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn HeadsetDisconnected<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGameController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HeadsetDisconnected(::core::mem::transmute(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveHeadsetDisconnected<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGameController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveHeadsetDisconnected(::core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn UserChanged<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGameController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UserChanged(::core::mem::transmute(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveUserChanged<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGameController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveUserChanged(::core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn Headset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGameController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Headset() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsWireless<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGameController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsWireless() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn User<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGameController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.User() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectableVtbl::new::<Identity, IGameController, OFFSET>(),
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
    fn TryGetBatteryReport(&self) -> ::windows::core::Result<super::super::Devices::Power::BatteryReport>;
}
#[cfg(feature = "Devices_Power")]
impl ::windows::core::RuntimeName for IGameControllerBatteryInfo {
    const NAME: &'static str = "Windows.Gaming.Input.IGameControllerBatteryInfo";
}
#[cfg(feature = "Devices_Power")]
impl IGameControllerBatteryInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGameControllerBatteryInfo_Impl, const OFFSET: isize>() -> IGameControllerBatteryInfo_Vtbl {
        unsafe extern "system" fn TryGetBatteryReport<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGameControllerBatteryInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TryGetBatteryReport() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectableVtbl::new::<Identity, IGameControllerBatteryInfo, OFFSET>(),
            TryGetBatteryReport: TryGetBatteryReport::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGameControllerBatteryInfo as ::windows::core::Interface>::IID
    }
}
