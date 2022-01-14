#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
pub trait IArcadeStick_Impl: Sized + IGameController_Impl {
    fn GetButtonLabel(&mut self, button: ArcadeStickButtons) -> ::windows::core::Result<GameControllerButtonLabel>;
    fn GetCurrentReading(&mut self) -> ::windows::core::Result<ArcadeStickReading>;
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IArcadeStick {
    const NAME: &'static str = "Windows.Gaming.Input.IArcadeStick";
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl IArcadeStick_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IArcadeStick_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IArcadeStick_Vtbl {
        unsafe extern "system" fn GetButtonLabel<Impl: IArcadeStick_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, button: ArcadeStickButtons, result__: *mut GameControllerButtonLabel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetButtonLabel(button) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentReading<Impl: IArcadeStick_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ArcadeStickReading) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentReading() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IArcadeStick, BASE_OFFSET>(),
            GetButtonLabel: GetButtonLabel::<Impl, IMPL_OFFSET>,
            GetCurrentReading: GetCurrentReading::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IArcadeStick as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IArcadeStickStatics_Impl: Sized {
    fn ArcadeStickAdded(&mut self, value: &::core::option::Option<super::super::Foundation::EventHandler<ArcadeStick>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveArcadeStickAdded(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ArcadeStickRemoved(&mut self, value: &::core::option::Option<super::super::Foundation::EventHandler<ArcadeStick>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveArcadeStickRemoved(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ArcadeSticks(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ArcadeStick>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IArcadeStickStatics {
    const NAME: &'static str = "Windows.Gaming.Input.IArcadeStickStatics";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IArcadeStickStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IArcadeStickStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IArcadeStickStatics_Vtbl {
        unsafe extern "system" fn ArcadeStickAdded<Impl: IArcadeStickStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ArcadeStickAdded(&*(&value as *const <super::super::Foundation::EventHandler<ArcadeStick> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<ArcadeStick> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveArcadeStickAdded<Impl: IArcadeStickStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveArcadeStickAdded(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ArcadeStickRemoved<Impl: IArcadeStickStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ArcadeStickRemoved(&*(&value as *const <super::super::Foundation::EventHandler<ArcadeStick> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<ArcadeStick> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveArcadeStickRemoved<Impl: IArcadeStickStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveArcadeStickRemoved(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ArcadeSticks<Impl: IArcadeStickStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ArcadeSticks() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IArcadeStickStatics, BASE_OFFSET>(),
            ArcadeStickAdded: ArcadeStickAdded::<Impl, IMPL_OFFSET>,
            RemoveArcadeStickAdded: RemoveArcadeStickAdded::<Impl, IMPL_OFFSET>,
            ArcadeStickRemoved: ArcadeStickRemoved::<Impl, IMPL_OFFSET>,
            RemoveArcadeStickRemoved: RemoveArcadeStickRemoved::<Impl, IMPL_OFFSET>,
            ArcadeSticks: ArcadeSticks::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IArcadeStickStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IArcadeStickStatics2_Impl: Sized + IArcadeStickStatics_Impl {
    fn FromGameController(&mut self, gamecontroller: &::core::option::Option<IGameController>) -> ::windows::core::Result<ArcadeStick>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IArcadeStickStatics2 {
    const NAME: &'static str = "Windows.Gaming.Input.IArcadeStickStatics2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IArcadeStickStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IArcadeStickStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IArcadeStickStatics2_Vtbl {
        unsafe extern "system" fn FromGameController<Impl: IArcadeStickStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gamecontroller: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromGameController(&*(&gamecontroller as *const <IGameController as ::windows::core::Abi>::Abi as *const <IGameController as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IArcadeStickStatics2, BASE_OFFSET>(),
            FromGameController: FromGameController::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IArcadeStickStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
pub trait IFlightStick_Impl: Sized + IGameController_Impl {
    fn HatSwitchKind(&mut self) -> ::windows::core::Result<GameControllerSwitchKind>;
    fn GetButtonLabel(&mut self, button: FlightStickButtons) -> ::windows::core::Result<GameControllerButtonLabel>;
    fn GetCurrentReading(&mut self) -> ::windows::core::Result<FlightStickReading>;
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFlightStick {
    const NAME: &'static str = "Windows.Gaming.Input.IFlightStick";
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl IFlightStick_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFlightStick_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFlightStick_Vtbl {
        unsafe extern "system" fn HatSwitchKind<Impl: IFlightStick_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GameControllerSwitchKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HatSwitchKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetButtonLabel<Impl: IFlightStick_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, button: FlightStickButtons, result__: *mut GameControllerButtonLabel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetButtonLabel(button) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentReading<Impl: IFlightStick_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FlightStickReading) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentReading() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFlightStick, BASE_OFFSET>(),
            HatSwitchKind: HatSwitchKind::<Impl, IMPL_OFFSET>,
            GetButtonLabel: GetButtonLabel::<Impl, IMPL_OFFSET>,
            GetCurrentReading: GetCurrentReading::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFlightStick as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IFlightStickStatics_Impl: Sized {
    fn FlightStickAdded(&mut self, value: &::core::option::Option<super::super::Foundation::EventHandler<FlightStick>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveFlightStickAdded(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn FlightStickRemoved(&mut self, value: &::core::option::Option<super::super::Foundation::EventHandler<FlightStick>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveFlightStickRemoved(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn FlightSticks(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<FlightStick>>;
    fn FromGameController(&mut self, gamecontroller: &::core::option::Option<IGameController>) -> ::windows::core::Result<FlightStick>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFlightStickStatics {
    const NAME: &'static str = "Windows.Gaming.Input.IFlightStickStatics";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IFlightStickStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFlightStickStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFlightStickStatics_Vtbl {
        unsafe extern "system" fn FlightStickAdded<Impl: IFlightStickStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FlightStickAdded(&*(&value as *const <super::super::Foundation::EventHandler<FlightStick> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<FlightStick> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFlightStickAdded<Impl: IFlightStickStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveFlightStickAdded(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FlightStickRemoved<Impl: IFlightStickStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FlightStickRemoved(&*(&value as *const <super::super::Foundation::EventHandler<FlightStick> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<FlightStick> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFlightStickRemoved<Impl: IFlightStickStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveFlightStickRemoved(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FlightSticks<Impl: IFlightStickStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FlightSticks() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromGameController<Impl: IFlightStickStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gamecontroller: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromGameController(&*(&gamecontroller as *const <IGameController as ::windows::core::Abi>::Abi as *const <IGameController as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFlightStickStatics, BASE_OFFSET>(),
            FlightStickAdded: FlightStickAdded::<Impl, IMPL_OFFSET>,
            RemoveFlightStickAdded: RemoveFlightStickAdded::<Impl, IMPL_OFFSET>,
            FlightStickRemoved: FlightStickRemoved::<Impl, IMPL_OFFSET>,
            RemoveFlightStickRemoved: RemoveFlightStickRemoved::<Impl, IMPL_OFFSET>,
            FlightSticks: FlightSticks::<Impl, IMPL_OFFSET>,
            FromGameController: FromGameController::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFlightStickStatics as ::windows::core::Interface>::IID
    }
}
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameController_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGameController_Vtbl {
        unsafe extern "system" fn HeadsetConnected<Impl: IGameController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HeadsetConnected(&*(&value as *const <super::super::Foundation::TypedEventHandler<IGameController, Headset> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<IGameController, Headset> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveHeadsetConnected<Impl: IGameController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveHeadsetConnected(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn HeadsetDisconnected<Impl: IGameController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HeadsetDisconnected(&*(&value as *const <super::super::Foundation::TypedEventHandler<IGameController, Headset> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<IGameController, Headset> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveHeadsetDisconnected<Impl: IGameController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveHeadsetDisconnected(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UserChanged<Impl: IGameController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserChanged(&*(&value as *const <super::super::Foundation::TypedEventHandler<IGameController, super::super::System::UserChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<IGameController, super::super::System::UserChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveUserChanged<Impl: IGameController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveUserChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Headset<Impl: IGameController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Headset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsWireless<Impl: IGameController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsWireless() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn User<Impl: IGameController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGameController, BASE_OFFSET>(),
            HeadsetConnected: HeadsetConnected::<Impl, IMPL_OFFSET>,
            RemoveHeadsetConnected: RemoveHeadsetConnected::<Impl, IMPL_OFFSET>,
            HeadsetDisconnected: HeadsetDisconnected::<Impl, IMPL_OFFSET>,
            RemoveHeadsetDisconnected: RemoveHeadsetDisconnected::<Impl, IMPL_OFFSET>,
            UserChanged: UserChanged::<Impl, IMPL_OFFSET>,
            RemoveUserChanged: RemoveUserChanged::<Impl, IMPL_OFFSET>,
            Headset: Headset::<Impl, IMPL_OFFSET>,
            IsWireless: IsWireless::<Impl, IMPL_OFFSET>,
            User: User::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameControllerBatteryInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGameControllerBatteryInfo_Vtbl {
        unsafe extern "system" fn TryGetBatteryReport<Impl: IGameControllerBatteryInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGameControllerBatteryInfo, BASE_OFFSET>(),
            TryGetBatteryReport: TryGetBatteryReport::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGameControllerBatteryInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
pub trait IGamepad_Impl: Sized + IGameController_Impl {
    fn Vibration(&mut self) -> ::windows::core::Result<GamepadVibration>;
    fn SetVibration(&mut self, value: &GamepadVibration) -> ::windows::core::Result<()>;
    fn GetCurrentReading(&mut self) -> ::windows::core::Result<GamepadReading>;
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGamepad {
    const NAME: &'static str = "Windows.Gaming.Input.IGamepad";
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl IGamepad_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGamepad_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGamepad_Vtbl {
        unsafe extern "system" fn Vibration<Impl: IGamepad_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GamepadVibration) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Vibration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVibration<Impl: IGamepad_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: GamepadVibration) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVibration(&*(&value as *const <GamepadVibration as ::windows::core::Abi>::Abi as *const <GamepadVibration as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetCurrentReading<Impl: IGamepad_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GamepadReading) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentReading() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGamepad, BASE_OFFSET>(),
            Vibration: Vibration::<Impl, IMPL_OFFSET>,
            SetVibration: SetVibration::<Impl, IMPL_OFFSET>,
            GetCurrentReading: GetCurrentReading::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGamepad as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
pub trait IGamepad2_Impl: Sized + IGameController_Impl + IGamepad_Impl {
    fn GetButtonLabel(&mut self, button: GamepadButtons) -> ::windows::core::Result<GameControllerButtonLabel>;
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGamepad2 {
    const NAME: &'static str = "Windows.Gaming.Input.IGamepad2";
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl IGamepad2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGamepad2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGamepad2_Vtbl {
        unsafe extern "system" fn GetButtonLabel<Impl: IGamepad2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, button: GamepadButtons, result__: *mut GameControllerButtonLabel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetButtonLabel(button) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IGamepad2, BASE_OFFSET>(), GetButtonLabel: GetButtonLabel::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGamepad2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IGamepadStatics_Impl: Sized {
    fn GamepadAdded(&mut self, value: &::core::option::Option<super::super::Foundation::EventHandler<Gamepad>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveGamepadAdded(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GamepadRemoved(&mut self, value: &::core::option::Option<super::super::Foundation::EventHandler<Gamepad>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveGamepadRemoved(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Gamepads(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<Gamepad>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGamepadStatics {
    const NAME: &'static str = "Windows.Gaming.Input.IGamepadStatics";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IGamepadStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGamepadStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGamepadStatics_Vtbl {
        unsafe extern "system" fn GamepadAdded<Impl: IGamepadStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GamepadAdded(&*(&value as *const <super::super::Foundation::EventHandler<Gamepad> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<Gamepad> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveGamepadAdded<Impl: IGamepadStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveGamepadAdded(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GamepadRemoved<Impl: IGamepadStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GamepadRemoved(&*(&value as *const <super::super::Foundation::EventHandler<Gamepad> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<Gamepad> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveGamepadRemoved<Impl: IGamepadStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveGamepadRemoved(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Gamepads<Impl: IGamepadStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Gamepads() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGamepadStatics, BASE_OFFSET>(),
            GamepadAdded: GamepadAdded::<Impl, IMPL_OFFSET>,
            RemoveGamepadAdded: RemoveGamepadAdded::<Impl, IMPL_OFFSET>,
            GamepadRemoved: GamepadRemoved::<Impl, IMPL_OFFSET>,
            RemoveGamepadRemoved: RemoveGamepadRemoved::<Impl, IMPL_OFFSET>,
            Gamepads: Gamepads::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGamepadStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IGamepadStatics2_Impl: Sized + IGamepadStatics_Impl {
    fn FromGameController(&mut self, gamecontroller: &::core::option::Option<IGameController>) -> ::windows::core::Result<Gamepad>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGamepadStatics2 {
    const NAME: &'static str = "Windows.Gaming.Input.IGamepadStatics2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IGamepadStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGamepadStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGamepadStatics2_Vtbl {
        unsafe extern "system" fn FromGameController<Impl: IGamepadStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gamecontroller: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromGameController(&*(&gamecontroller as *const <IGameController as ::windows::core::Abi>::Abi as *const <IGameController as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGamepadStatics2, BASE_OFFSET>(),
            FromGameController: FromGameController::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGamepadStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHeadset_Impl: Sized {
    fn CaptureDeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RenderDeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHeadset {
    const NAME: &'static str = "Windows.Gaming.Input.IHeadset";
}
#[cfg(feature = "implement_exclusive")]
impl IHeadset_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHeadset_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHeadset_Vtbl {
        unsafe extern "system" fn CaptureDeviceId<Impl: IHeadset_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CaptureDeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RenderDeviceId<Impl: IHeadset_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RenderDeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHeadset, BASE_OFFSET>(),
            CaptureDeviceId: CaptureDeviceId::<Impl, IMPL_OFFSET>,
            RenderDeviceId: RenderDeviceId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHeadset as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Gaming_Input_ForceFeedback", feature = "System", feature = "implement_exclusive"))]
pub trait IRacingWheel_Impl: Sized + IGameController_Impl {
    fn HasClutch(&mut self) -> ::windows::core::Result<bool>;
    fn HasHandbrake(&mut self) -> ::windows::core::Result<bool>;
    fn HasPatternShifter(&mut self) -> ::windows::core::Result<bool>;
    fn MaxPatternShifterGear(&mut self) -> ::windows::core::Result<i32>;
    fn MaxWheelAngle(&mut self) -> ::windows::core::Result<f64>;
    fn WheelMotor(&mut self) -> ::windows::core::Result<ForceFeedback::ForceFeedbackMotor>;
    fn GetButtonLabel(&mut self, button: RacingWheelButtons) -> ::windows::core::Result<GameControllerButtonLabel>;
    fn GetCurrentReading(&mut self) -> ::windows::core::Result<RacingWheelReading>;
}
#[cfg(all(feature = "Foundation", feature = "Gaming_Input_ForceFeedback", feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRacingWheel {
    const NAME: &'static str = "Windows.Gaming.Input.IRacingWheel";
}
#[cfg(all(feature = "Foundation", feature = "Gaming_Input_ForceFeedback", feature = "System", feature = "implement_exclusive"))]
impl IRacingWheel_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRacingWheel_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRacingWheel_Vtbl {
        unsafe extern "system" fn HasClutch<Impl: IRacingWheel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasClutch() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasHandbrake<Impl: IRacingWheel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasHandbrake() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasPatternShifter<Impl: IRacingWheel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasPatternShifter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxPatternShifterGear<Impl: IRacingWheel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxPatternShifterGear() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxWheelAngle<Impl: IRacingWheel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxWheelAngle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WheelMotor<Impl: IRacingWheel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WheelMotor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetButtonLabel<Impl: IRacingWheel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, button: RacingWheelButtons, result__: *mut GameControllerButtonLabel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetButtonLabel(button) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentReading<Impl: IRacingWheel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut RacingWheelReading) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentReading() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRacingWheel, BASE_OFFSET>(),
            HasClutch: HasClutch::<Impl, IMPL_OFFSET>,
            HasHandbrake: HasHandbrake::<Impl, IMPL_OFFSET>,
            HasPatternShifter: HasPatternShifter::<Impl, IMPL_OFFSET>,
            MaxPatternShifterGear: MaxPatternShifterGear::<Impl, IMPL_OFFSET>,
            MaxWheelAngle: MaxWheelAngle::<Impl, IMPL_OFFSET>,
            WheelMotor: WheelMotor::<Impl, IMPL_OFFSET>,
            GetButtonLabel: GetButtonLabel::<Impl, IMPL_OFFSET>,
            GetCurrentReading: GetCurrentReading::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRacingWheel as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IRacingWheelStatics_Impl: Sized {
    fn RacingWheelAdded(&mut self, value: &::core::option::Option<super::super::Foundation::EventHandler<RacingWheel>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRacingWheelAdded(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RacingWheelRemoved(&mut self, value: &::core::option::Option<super::super::Foundation::EventHandler<RacingWheel>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRacingWheelRemoved(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RacingWheels(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<RacingWheel>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRacingWheelStatics {
    const NAME: &'static str = "Windows.Gaming.Input.IRacingWheelStatics";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IRacingWheelStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRacingWheelStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRacingWheelStatics_Vtbl {
        unsafe extern "system" fn RacingWheelAdded<Impl: IRacingWheelStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RacingWheelAdded(&*(&value as *const <super::super::Foundation::EventHandler<RacingWheel> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<RacingWheel> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRacingWheelAdded<Impl: IRacingWheelStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRacingWheelAdded(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RacingWheelRemoved<Impl: IRacingWheelStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RacingWheelRemoved(&*(&value as *const <super::super::Foundation::EventHandler<RacingWheel> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<RacingWheel> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRacingWheelRemoved<Impl: IRacingWheelStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRacingWheelRemoved(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RacingWheels<Impl: IRacingWheelStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RacingWheels() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRacingWheelStatics, BASE_OFFSET>(),
            RacingWheelAdded: RacingWheelAdded::<Impl, IMPL_OFFSET>,
            RemoveRacingWheelAdded: RemoveRacingWheelAdded::<Impl, IMPL_OFFSET>,
            RacingWheelRemoved: RacingWheelRemoved::<Impl, IMPL_OFFSET>,
            RemoveRacingWheelRemoved: RemoveRacingWheelRemoved::<Impl, IMPL_OFFSET>,
            RacingWheels: RacingWheels::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRacingWheelStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IRacingWheelStatics2_Impl: Sized + IRacingWheelStatics_Impl {
    fn FromGameController(&mut self, gamecontroller: &::core::option::Option<IGameController>) -> ::windows::core::Result<RacingWheel>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRacingWheelStatics2 {
    const NAME: &'static str = "Windows.Gaming.Input.IRacingWheelStatics2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IRacingWheelStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRacingWheelStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRacingWheelStatics2_Vtbl {
        unsafe extern "system" fn FromGameController<Impl: IRacingWheelStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gamecontroller: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromGameController(&*(&gamecontroller as *const <IGameController as ::windows::core::Abi>::Abi as *const <IGameController as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRacingWheelStatics2, BASE_OFFSET>(),
            FromGameController: FromGameController::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRacingWheelStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Gaming_Input_ForceFeedback", feature = "System", feature = "implement_exclusive"))]
pub trait IRawGameController_Impl: Sized + IGameController_Impl {
    fn AxisCount(&mut self) -> ::windows::core::Result<i32>;
    fn ButtonCount(&mut self) -> ::windows::core::Result<i32>;
    fn ForceFeedbackMotors(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ForceFeedback::ForceFeedbackMotor>>;
    fn HardwareProductId(&mut self) -> ::windows::core::Result<u16>;
    fn HardwareVendorId(&mut self) -> ::windows::core::Result<u16>;
    fn SwitchCount(&mut self) -> ::windows::core::Result<i32>;
    fn GetButtonLabel(&mut self, buttonindex: i32) -> ::windows::core::Result<GameControllerButtonLabel>;
    fn GetCurrentReading(&mut self, buttonarray: &mut [<bool as ::windows::core::DefaultType>::DefaultType], switcharray: &mut [<GameControllerSwitchPosition as ::windows::core::DefaultType>::DefaultType], axisarray: &mut [<f64 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u64>;
    fn GetSwitchKind(&mut self, switchindex: i32) -> ::windows::core::Result<GameControllerSwitchKind>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Gaming_Input_ForceFeedback", feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRawGameController {
    const NAME: &'static str = "Windows.Gaming.Input.IRawGameController";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Gaming_Input_ForceFeedback", feature = "System", feature = "implement_exclusive"))]
impl IRawGameController_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRawGameController_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRawGameController_Vtbl {
        unsafe extern "system" fn AxisCount<Impl: IRawGameController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AxisCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ButtonCount<Impl: IRawGameController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ButtonCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ForceFeedbackMotors<Impl: IRawGameController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ForceFeedbackMotors() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HardwareProductId<Impl: IRawGameController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HardwareProductId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HardwareVendorId<Impl: IRawGameController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HardwareVendorId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SwitchCount<Impl: IRawGameController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SwitchCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetButtonLabel<Impl: IRawGameController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buttonindex: i32, result__: *mut GameControllerButtonLabel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetButtonLabel(buttonindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentReading<Impl: IRawGameController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buttonArray_array_size: u32, buttonarray: *mut bool, switchArray_array_size: u32, switcharray: *mut GameControllerSwitchPosition, axisArray_array_size: u32, axisarray: *mut f64, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentReading(::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&buttonarray), buttonArray_array_size as _), ::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&switcharray), switchArray_array_size as _), ::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&axisarray), axisArray_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSwitchKind<Impl: IRawGameController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, switchindex: i32, result__: *mut GameControllerSwitchKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSwitchKind(switchindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRawGameController, BASE_OFFSET>(),
            AxisCount: AxisCount::<Impl, IMPL_OFFSET>,
            ButtonCount: ButtonCount::<Impl, IMPL_OFFSET>,
            ForceFeedbackMotors: ForceFeedbackMotors::<Impl, IMPL_OFFSET>,
            HardwareProductId: HardwareProductId::<Impl, IMPL_OFFSET>,
            HardwareVendorId: HardwareVendorId::<Impl, IMPL_OFFSET>,
            SwitchCount: SwitchCount::<Impl, IMPL_OFFSET>,
            GetButtonLabel: GetButtonLabel::<Impl, IMPL_OFFSET>,
            GetCurrentReading: GetCurrentReading::<Impl, IMPL_OFFSET>,
            GetSwitchKind: GetSwitchKind::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRawGameController as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Haptics", feature = "Foundation", feature = "Foundation_Collections", feature = "System", feature = "implement_exclusive"))]
pub trait IRawGameController2_Impl: Sized + IGameController_Impl + IRawGameController_Impl {
    fn SimpleHapticsControllers(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Devices::Haptics::SimpleHapticsController>>;
    fn NonRoamableId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Devices_Haptics", feature = "Foundation", feature = "Foundation_Collections", feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRawGameController2 {
    const NAME: &'static str = "Windows.Gaming.Input.IRawGameController2";
}
#[cfg(all(feature = "Devices_Haptics", feature = "Foundation", feature = "Foundation_Collections", feature = "System", feature = "implement_exclusive"))]
impl IRawGameController2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRawGameController2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRawGameController2_Vtbl {
        unsafe extern "system" fn SimpleHapticsControllers<Impl: IRawGameController2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SimpleHapticsControllers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NonRoamableId<Impl: IRawGameController2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NonRoamableId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Impl: IRawGameController2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRawGameController2, BASE_OFFSET>(),
            SimpleHapticsControllers: SimpleHapticsControllers::<Impl, IMPL_OFFSET>,
            NonRoamableId: NonRoamableId::<Impl, IMPL_OFFSET>,
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRawGameController2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IRawGameControllerStatics_Impl: Sized {
    fn RawGameControllerAdded(&mut self, value: &::core::option::Option<super::super::Foundation::EventHandler<RawGameController>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRawGameControllerAdded(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RawGameControllerRemoved(&mut self, value: &::core::option::Option<super::super::Foundation::EventHandler<RawGameController>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRawGameControllerRemoved(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RawGameControllers(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<RawGameController>>;
    fn FromGameController(&mut self, gamecontroller: &::core::option::Option<IGameController>) -> ::windows::core::Result<RawGameController>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRawGameControllerStatics {
    const NAME: &'static str = "Windows.Gaming.Input.IRawGameControllerStatics";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IRawGameControllerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRawGameControllerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRawGameControllerStatics_Vtbl {
        unsafe extern "system" fn RawGameControllerAdded<Impl: IRawGameControllerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RawGameControllerAdded(&*(&value as *const <super::super::Foundation::EventHandler<RawGameController> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<RawGameController> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRawGameControllerAdded<Impl: IRawGameControllerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRawGameControllerAdded(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RawGameControllerRemoved<Impl: IRawGameControllerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RawGameControllerRemoved(&*(&value as *const <super::super::Foundation::EventHandler<RawGameController> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<RawGameController> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRawGameControllerRemoved<Impl: IRawGameControllerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRawGameControllerRemoved(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RawGameControllers<Impl: IRawGameControllerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RawGameControllers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromGameController<Impl: IRawGameControllerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gamecontroller: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromGameController(&*(&gamecontroller as *const <IGameController as ::windows::core::Abi>::Abi as *const <IGameController as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRawGameControllerStatics, BASE_OFFSET>(),
            RawGameControllerAdded: RawGameControllerAdded::<Impl, IMPL_OFFSET>,
            RemoveRawGameControllerAdded: RemoveRawGameControllerAdded::<Impl, IMPL_OFFSET>,
            RawGameControllerRemoved: RawGameControllerRemoved::<Impl, IMPL_OFFSET>,
            RemoveRawGameControllerRemoved: RemoveRawGameControllerRemoved::<Impl, IMPL_OFFSET>,
            RawGameControllers: RawGameControllers::<Impl, IMPL_OFFSET>,
            FromGameController: FromGameController::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRawGameControllerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
pub trait IUINavigationController_Impl: Sized + IGameController_Impl {
    fn GetCurrentReading(&mut self) -> ::windows::core::Result<UINavigationReading>;
    fn GetOptionalButtonLabel(&mut self, button: OptionalUINavigationButtons) -> ::windows::core::Result<GameControllerButtonLabel>;
    fn GetRequiredButtonLabel(&mut self, button: RequiredUINavigationButtons) -> ::windows::core::Result<GameControllerButtonLabel>;
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUINavigationController {
    const NAME: &'static str = "Windows.Gaming.Input.IUINavigationController";
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl IUINavigationController_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUINavigationController_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUINavigationController_Vtbl {
        unsafe extern "system" fn GetCurrentReading<Impl: IUINavigationController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UINavigationReading) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentReading() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOptionalButtonLabel<Impl: IUINavigationController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, button: OptionalUINavigationButtons, result__: *mut GameControllerButtonLabel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOptionalButtonLabel(button) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRequiredButtonLabel<Impl: IUINavigationController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, button: RequiredUINavigationButtons, result__: *mut GameControllerButtonLabel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRequiredButtonLabel(button) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUINavigationController, BASE_OFFSET>(),
            GetCurrentReading: GetCurrentReading::<Impl, IMPL_OFFSET>,
            GetOptionalButtonLabel: GetOptionalButtonLabel::<Impl, IMPL_OFFSET>,
            GetRequiredButtonLabel: GetRequiredButtonLabel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUINavigationController as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IUINavigationControllerStatics_Impl: Sized {
    fn UINavigationControllerAdded(&mut self, value: &::core::option::Option<super::super::Foundation::EventHandler<UINavigationController>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUINavigationControllerAdded(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn UINavigationControllerRemoved(&mut self, value: &::core::option::Option<super::super::Foundation::EventHandler<UINavigationController>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUINavigationControllerRemoved(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn UINavigationControllers(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<UINavigationController>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUINavigationControllerStatics {
    const NAME: &'static str = "Windows.Gaming.Input.IUINavigationControllerStatics";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IUINavigationControllerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUINavigationControllerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUINavigationControllerStatics_Vtbl {
        unsafe extern "system" fn UINavigationControllerAdded<Impl: IUINavigationControllerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UINavigationControllerAdded(&*(&value as *const <super::super::Foundation::EventHandler<UINavigationController> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<UINavigationController> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveUINavigationControllerAdded<Impl: IUINavigationControllerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveUINavigationControllerAdded(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UINavigationControllerRemoved<Impl: IUINavigationControllerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UINavigationControllerRemoved(&*(&value as *const <super::super::Foundation::EventHandler<UINavigationController> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<UINavigationController> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveUINavigationControllerRemoved<Impl: IUINavigationControllerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveUINavigationControllerRemoved(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UINavigationControllers<Impl: IUINavigationControllerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UINavigationControllers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUINavigationControllerStatics, BASE_OFFSET>(),
            UINavigationControllerAdded: UINavigationControllerAdded::<Impl, IMPL_OFFSET>,
            RemoveUINavigationControllerAdded: RemoveUINavigationControllerAdded::<Impl, IMPL_OFFSET>,
            UINavigationControllerRemoved: UINavigationControllerRemoved::<Impl, IMPL_OFFSET>,
            RemoveUINavigationControllerRemoved: RemoveUINavigationControllerRemoved::<Impl, IMPL_OFFSET>,
            UINavigationControllers: UINavigationControllers::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUINavigationControllerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IUINavigationControllerStatics2_Impl: Sized + IUINavigationControllerStatics_Impl {
    fn FromGameController(&mut self, gamecontroller: &::core::option::Option<IGameController>) -> ::windows::core::Result<UINavigationController>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUINavigationControllerStatics2 {
    const NAME: &'static str = "Windows.Gaming.Input.IUINavigationControllerStatics2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IUINavigationControllerStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUINavigationControllerStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUINavigationControllerStatics2_Vtbl {
        unsafe extern "system" fn FromGameController<Impl: IUINavigationControllerStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gamecontroller: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromGameController(&*(&gamecontroller as *const <IGameController as ::windows::core::Abi>::Abi as *const <IGameController as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUINavigationControllerStatics2, BASE_OFFSET>(),
            FromGameController: FromGameController::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUINavigationControllerStatics2 as ::windows::core::Interface>::IID
    }
}
