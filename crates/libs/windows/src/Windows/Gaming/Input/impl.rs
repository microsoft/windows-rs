#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
pub trait IArcadeStickImpl: Sized + IGameControllerImpl {
    fn GetButtonLabel(&self, button: ArcadeStickButtons) -> ::windows::core::Result<GameControllerButtonLabel>;
    fn GetCurrentReading(&self) -> ::windows::core::Result<ArcadeStickReading>;
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IArcadeStick {
    const NAME: &'static str = "Windows.Gaming.Input.IArcadeStick";
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl IArcadeStickVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IArcadeStickImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IArcadeStickVtbl {
        unsafe extern "system" fn GetButtonLabel<Impl: IArcadeStickImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, button: ArcadeStickButtons, result__: *mut GameControllerButtonLabel) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetCurrentReading<Impl: IArcadeStickImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ArcadeStickReading) -> ::windows::core::HRESULT {
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
pub trait IArcadeStickStaticsImpl: Sized {
    fn ArcadeStickAdded(&self, value: &::core::option::Option<super::super::Foundation::EventHandler<ArcadeStick>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveArcadeStickAdded(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ArcadeStickRemoved(&self, value: &::core::option::Option<super::super::Foundation::EventHandler<ArcadeStick>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveArcadeStickRemoved(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ArcadeSticks(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ArcadeStick>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IArcadeStickStatics {
    const NAME: &'static str = "Windows.Gaming.Input.IArcadeStickStatics";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IArcadeStickStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IArcadeStickStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IArcadeStickStaticsVtbl {
        unsafe extern "system" fn ArcadeStickAdded<Impl: IArcadeStickStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveArcadeStickAdded<Impl: IArcadeStickStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveArcadeStickAdded(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ArcadeStickRemoved<Impl: IArcadeStickStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveArcadeStickRemoved<Impl: IArcadeStickStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveArcadeStickRemoved(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ArcadeSticks<Impl: IArcadeStickStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IArcadeStickStatics2Impl: Sized + IArcadeStickStaticsImpl {
    fn FromGameController(&self, gamecontroller: &::core::option::Option<IGameController>) -> ::windows::core::Result<ArcadeStick>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IArcadeStickStatics2 {
    const NAME: &'static str = "Windows.Gaming.Input.IArcadeStickStatics2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IArcadeStickStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IArcadeStickStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IArcadeStickStatics2Vtbl {
        unsafe extern "system" fn FromGameController<Impl: IArcadeStickStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gamecontroller: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IFlightStickImpl: Sized + IGameControllerImpl {
    fn HatSwitchKind(&self) -> ::windows::core::Result<GameControllerSwitchKind>;
    fn GetButtonLabel(&self, button: FlightStickButtons) -> ::windows::core::Result<GameControllerButtonLabel>;
    fn GetCurrentReading(&self) -> ::windows::core::Result<FlightStickReading>;
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFlightStick {
    const NAME: &'static str = "Windows.Gaming.Input.IFlightStick";
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl IFlightStickVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFlightStickImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFlightStickVtbl {
        unsafe extern "system" fn HatSwitchKind<Impl: IFlightStickImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GameControllerSwitchKind) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetButtonLabel<Impl: IFlightStickImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, button: FlightStickButtons, result__: *mut GameControllerButtonLabel) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetCurrentReading<Impl: IFlightStickImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FlightStickReading) -> ::windows::core::HRESULT {
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
pub trait IFlightStickStaticsImpl: Sized {
    fn FlightStickAdded(&self, value: &::core::option::Option<super::super::Foundation::EventHandler<FlightStick>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveFlightStickAdded(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn FlightStickRemoved(&self, value: &::core::option::Option<super::super::Foundation::EventHandler<FlightStick>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveFlightStickRemoved(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn FlightSticks(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<FlightStick>>;
    fn FromGameController(&self, gamecontroller: &::core::option::Option<IGameController>) -> ::windows::core::Result<FlightStick>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFlightStickStatics {
    const NAME: &'static str = "Windows.Gaming.Input.IFlightStickStatics";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IFlightStickStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFlightStickStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFlightStickStaticsVtbl {
        unsafe extern "system" fn FlightStickAdded<Impl: IFlightStickStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveFlightStickAdded<Impl: IFlightStickStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveFlightStickAdded(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FlightStickRemoved<Impl: IFlightStickStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveFlightStickRemoved<Impl: IFlightStickStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveFlightStickRemoved(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FlightSticks<Impl: IFlightStickStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FromGameController<Impl: IFlightStickStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gamecontroller: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IGameControllerImpl: Sized {
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
impl IGameControllerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameControllerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGameControllerVtbl {
        unsafe extern "system" fn HeadsetConnected<Impl: IGameControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveHeadsetConnected<Impl: IGameControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveHeadsetConnected(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn HeadsetDisconnected<Impl: IGameControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveHeadsetDisconnected<Impl: IGameControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveHeadsetDisconnected(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UserChanged<Impl: IGameControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveUserChanged<Impl: IGameControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveUserChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Headset<Impl: IGameControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsWireless<Impl: IGameControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn User<Impl: IGameControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IGameControllerBatteryInfoImpl: Sized {
    fn TryGetBatteryReport(&self) -> ::windows::core::Result<super::super::Devices::Power::BatteryReport>;
}
#[cfg(feature = "Devices_Power")]
impl ::windows::core::RuntimeName for IGameControllerBatteryInfo {
    const NAME: &'static str = "Windows.Gaming.Input.IGameControllerBatteryInfo";
}
#[cfg(feature = "Devices_Power")]
impl IGameControllerBatteryInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGameControllerBatteryInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGameControllerBatteryInfoVtbl {
        unsafe extern "system" fn TryGetBatteryReport<Impl: IGameControllerBatteryInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IGamepadImpl: Sized + IGameControllerImpl {
    fn Vibration(&self) -> ::windows::core::Result<GamepadVibration>;
    fn SetVibration(&self, value: &GamepadVibration) -> ::windows::core::Result<()>;
    fn GetCurrentReading(&self) -> ::windows::core::Result<GamepadReading>;
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGamepad {
    const NAME: &'static str = "Windows.Gaming.Input.IGamepad";
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl IGamepadVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGamepadImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGamepadVtbl {
        unsafe extern "system" fn Vibration<Impl: IGamepadImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GamepadVibration) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetVibration<Impl: IGamepadImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: GamepadVibration) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVibration(&*(&value as *const <GamepadVibration as ::windows::core::Abi>::Abi as *const <GamepadVibration as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetCurrentReading<Impl: IGamepadImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GamepadReading) -> ::windows::core::HRESULT {
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
pub trait IGamepad2Impl: Sized + IGameControllerImpl + IGamepadImpl {
    fn GetButtonLabel(&self, button: GamepadButtons) -> ::windows::core::Result<GameControllerButtonLabel>;
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGamepad2 {
    const NAME: &'static str = "Windows.Gaming.Input.IGamepad2";
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl IGamepad2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGamepad2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGamepad2Vtbl {
        unsafe extern "system" fn GetButtonLabel<Impl: IGamepad2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, button: GamepadButtons, result__: *mut GameControllerButtonLabel) -> ::windows::core::HRESULT {
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
pub trait IGamepadStaticsImpl: Sized {
    fn GamepadAdded(&self, value: &::core::option::Option<super::super::Foundation::EventHandler<Gamepad>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveGamepadAdded(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GamepadRemoved(&self, value: &::core::option::Option<super::super::Foundation::EventHandler<Gamepad>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveGamepadRemoved(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Gamepads(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<Gamepad>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGamepadStatics {
    const NAME: &'static str = "Windows.Gaming.Input.IGamepadStatics";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IGamepadStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGamepadStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGamepadStaticsVtbl {
        unsafe extern "system" fn GamepadAdded<Impl: IGamepadStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveGamepadAdded<Impl: IGamepadStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveGamepadAdded(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GamepadRemoved<Impl: IGamepadStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveGamepadRemoved<Impl: IGamepadStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveGamepadRemoved(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Gamepads<Impl: IGamepadStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IGamepadStatics2Impl: Sized + IGamepadStaticsImpl {
    fn FromGameController(&self, gamecontroller: &::core::option::Option<IGameController>) -> ::windows::core::Result<Gamepad>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGamepadStatics2 {
    const NAME: &'static str = "Windows.Gaming.Input.IGamepadStatics2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IGamepadStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGamepadStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGamepadStatics2Vtbl {
        unsafe extern "system" fn FromGameController<Impl: IGamepadStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gamecontroller: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IHeadsetImpl: Sized {
    fn CaptureDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RenderDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHeadset {
    const NAME: &'static str = "Windows.Gaming.Input.IHeadset";
}
#[cfg(feature = "implement_exclusive")]
impl IHeadsetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHeadsetImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHeadsetVtbl {
        unsafe extern "system" fn CaptureDeviceId<Impl: IHeadsetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RenderDeviceId<Impl: IHeadsetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
pub trait IRacingWheelImpl: Sized + IGameControllerImpl {
    fn HasClutch(&self) -> ::windows::core::Result<bool>;
    fn HasHandbrake(&self) -> ::windows::core::Result<bool>;
    fn HasPatternShifter(&self) -> ::windows::core::Result<bool>;
    fn MaxPatternShifterGear(&self) -> ::windows::core::Result<i32>;
    fn MaxWheelAngle(&self) -> ::windows::core::Result<f64>;
    fn WheelMotor(&self) -> ::windows::core::Result<ForceFeedback::ForceFeedbackMotor>;
    fn GetButtonLabel(&self, button: RacingWheelButtons) -> ::windows::core::Result<GameControllerButtonLabel>;
    fn GetCurrentReading(&self) -> ::windows::core::Result<RacingWheelReading>;
}
#[cfg(all(feature = "Foundation", feature = "Gaming_Input_ForceFeedback", feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRacingWheel {
    const NAME: &'static str = "Windows.Gaming.Input.IRacingWheel";
}
#[cfg(all(feature = "Foundation", feature = "Gaming_Input_ForceFeedback", feature = "System", feature = "implement_exclusive"))]
impl IRacingWheelVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRacingWheelImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRacingWheelVtbl {
        unsafe extern "system" fn HasClutch<Impl: IRacingWheelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HasHandbrake<Impl: IRacingWheelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HasPatternShifter<Impl: IRacingWheelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MaxPatternShifterGear<Impl: IRacingWheelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MaxWheelAngle<Impl: IRacingWheelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn WheelMotor<Impl: IRacingWheelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetButtonLabel<Impl: IRacingWheelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, button: RacingWheelButtons, result__: *mut GameControllerButtonLabel) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetCurrentReading<Impl: IRacingWheelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut RacingWheelReading) -> ::windows::core::HRESULT {
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
pub trait IRacingWheelStaticsImpl: Sized {
    fn RacingWheelAdded(&self, value: &::core::option::Option<super::super::Foundation::EventHandler<RacingWheel>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRacingWheelAdded(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RacingWheelRemoved(&self, value: &::core::option::Option<super::super::Foundation::EventHandler<RacingWheel>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRacingWheelRemoved(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RacingWheels(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<RacingWheel>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRacingWheelStatics {
    const NAME: &'static str = "Windows.Gaming.Input.IRacingWheelStatics";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IRacingWheelStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRacingWheelStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRacingWheelStaticsVtbl {
        unsafe extern "system" fn RacingWheelAdded<Impl: IRacingWheelStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveRacingWheelAdded<Impl: IRacingWheelStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRacingWheelAdded(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RacingWheelRemoved<Impl: IRacingWheelStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveRacingWheelRemoved<Impl: IRacingWheelStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRacingWheelRemoved(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RacingWheels<Impl: IRacingWheelStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IRacingWheelStatics2Impl: Sized + IRacingWheelStaticsImpl {
    fn FromGameController(&self, gamecontroller: &::core::option::Option<IGameController>) -> ::windows::core::Result<RacingWheel>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRacingWheelStatics2 {
    const NAME: &'static str = "Windows.Gaming.Input.IRacingWheelStatics2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IRacingWheelStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRacingWheelStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRacingWheelStatics2Vtbl {
        unsafe extern "system" fn FromGameController<Impl: IRacingWheelStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gamecontroller: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IRawGameControllerImpl: Sized + IGameControllerImpl {
    fn AxisCount(&self) -> ::windows::core::Result<i32>;
    fn ButtonCount(&self) -> ::windows::core::Result<i32>;
    fn ForceFeedbackMotors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ForceFeedback::ForceFeedbackMotor>>;
    fn HardwareProductId(&self) -> ::windows::core::Result<u16>;
    fn HardwareVendorId(&self) -> ::windows::core::Result<u16>;
    fn SwitchCount(&self) -> ::windows::core::Result<i32>;
    fn GetButtonLabel(&self, buttonindex: i32) -> ::windows::core::Result<GameControllerButtonLabel>;
    fn GetCurrentReading(&self, buttonarray: &mut [<bool as ::windows::core::DefaultType>::DefaultType], switcharray: &mut [<GameControllerSwitchPosition as ::windows::core::DefaultType>::DefaultType], axisarray: &mut [<f64 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u64>;
    fn GetSwitchKind(&self, switchindex: i32) -> ::windows::core::Result<GameControllerSwitchKind>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Gaming_Input_ForceFeedback", feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRawGameController {
    const NAME: &'static str = "Windows.Gaming.Input.IRawGameController";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Gaming_Input_ForceFeedback", feature = "System", feature = "implement_exclusive"))]
impl IRawGameControllerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRawGameControllerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRawGameControllerVtbl {
        unsafe extern "system" fn AxisCount<Impl: IRawGameControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ButtonCount<Impl: IRawGameControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ForceFeedbackMotors<Impl: IRawGameControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HardwareProductId<Impl: IRawGameControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HardwareVendorId<Impl: IRawGameControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SwitchCount<Impl: IRawGameControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetButtonLabel<Impl: IRawGameControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buttonindex: i32, result__: *mut GameControllerButtonLabel) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetCurrentReading<Impl: IRawGameControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buttonArray_array_size: u32, buttonarray: *mut bool, switchArray_array_size: u32, switcharray: *mut GameControllerSwitchPosition, axisArray_array_size: u32, axisarray: *mut f64, result__: *mut u64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetSwitchKind<Impl: IRawGameControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, switchindex: i32, result__: *mut GameControllerSwitchKind) -> ::windows::core::HRESULT {
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
pub trait IRawGameController2Impl: Sized + IGameControllerImpl + IRawGameControllerImpl {
    fn SimpleHapticsControllers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Devices::Haptics::SimpleHapticsController>>;
    fn NonRoamableId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Devices_Haptics", feature = "Foundation", feature = "Foundation_Collections", feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRawGameController2 {
    const NAME: &'static str = "Windows.Gaming.Input.IRawGameController2";
}
#[cfg(all(feature = "Devices_Haptics", feature = "Foundation", feature = "Foundation_Collections", feature = "System", feature = "implement_exclusive"))]
impl IRawGameController2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRawGameController2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRawGameController2Vtbl {
        unsafe extern "system" fn SimpleHapticsControllers<Impl: IRawGameController2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn NonRoamableId<Impl: IRawGameController2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DisplayName<Impl: IRawGameController2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
pub trait IRawGameControllerStaticsImpl: Sized {
    fn RawGameControllerAdded(&self, value: &::core::option::Option<super::super::Foundation::EventHandler<RawGameController>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRawGameControllerAdded(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RawGameControllerRemoved(&self, value: &::core::option::Option<super::super::Foundation::EventHandler<RawGameController>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRawGameControllerRemoved(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RawGameControllers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<RawGameController>>;
    fn FromGameController(&self, gamecontroller: &::core::option::Option<IGameController>) -> ::windows::core::Result<RawGameController>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRawGameControllerStatics {
    const NAME: &'static str = "Windows.Gaming.Input.IRawGameControllerStatics";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IRawGameControllerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRawGameControllerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRawGameControllerStaticsVtbl {
        unsafe extern "system" fn RawGameControllerAdded<Impl: IRawGameControllerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveRawGameControllerAdded<Impl: IRawGameControllerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRawGameControllerAdded(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RawGameControllerRemoved<Impl: IRawGameControllerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveRawGameControllerRemoved<Impl: IRawGameControllerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRawGameControllerRemoved(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RawGameControllers<Impl: IRawGameControllerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FromGameController<Impl: IRawGameControllerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gamecontroller: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IUINavigationControllerImpl: Sized + IGameControllerImpl {
    fn GetCurrentReading(&self) -> ::windows::core::Result<UINavigationReading>;
    fn GetOptionalButtonLabel(&self, button: OptionalUINavigationButtons) -> ::windows::core::Result<GameControllerButtonLabel>;
    fn GetRequiredButtonLabel(&self, button: RequiredUINavigationButtons) -> ::windows::core::Result<GameControllerButtonLabel>;
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUINavigationController {
    const NAME: &'static str = "Windows.Gaming.Input.IUINavigationController";
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl IUINavigationControllerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUINavigationControllerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUINavigationControllerVtbl {
        unsafe extern "system" fn GetCurrentReading<Impl: IUINavigationControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UINavigationReading) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetOptionalButtonLabel<Impl: IUINavigationControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, button: OptionalUINavigationButtons, result__: *mut GameControllerButtonLabel) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetRequiredButtonLabel<Impl: IUINavigationControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, button: RequiredUINavigationButtons, result__: *mut GameControllerButtonLabel) -> ::windows::core::HRESULT {
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
pub trait IUINavigationControllerStaticsImpl: Sized {
    fn UINavigationControllerAdded(&self, value: &::core::option::Option<super::super::Foundation::EventHandler<UINavigationController>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUINavigationControllerAdded(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn UINavigationControllerRemoved(&self, value: &::core::option::Option<super::super::Foundation::EventHandler<UINavigationController>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUINavigationControllerRemoved(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn UINavigationControllers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<UINavigationController>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUINavigationControllerStatics {
    const NAME: &'static str = "Windows.Gaming.Input.IUINavigationControllerStatics";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IUINavigationControllerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUINavigationControllerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUINavigationControllerStaticsVtbl {
        unsafe extern "system" fn UINavigationControllerAdded<Impl: IUINavigationControllerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveUINavigationControllerAdded<Impl: IUINavigationControllerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveUINavigationControllerAdded(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UINavigationControllerRemoved<Impl: IUINavigationControllerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveUINavigationControllerRemoved<Impl: IUINavigationControllerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveUINavigationControllerRemoved(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UINavigationControllers<Impl: IUINavigationControllerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IUINavigationControllerStatics2Impl: Sized + IUINavigationControllerStaticsImpl {
    fn FromGameController(&self, gamecontroller: &::core::option::Option<IGameController>) -> ::windows::core::Result<UINavigationController>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUINavigationControllerStatics2 {
    const NAME: &'static str = "Windows.Gaming.Input.IUINavigationControllerStatics2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IUINavigationControllerStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUINavigationControllerStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUINavigationControllerStatics2Vtbl {
        unsafe extern "system" fn FromGameController<Impl: IUINavigationControllerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gamecontroller: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
