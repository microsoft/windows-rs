#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IWiFiDirectAdvertisementImpl: Sized {
    fn InformationElements(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<WiFiDirectInformationElement>>;
    fn SetInformationElements(&mut self, value: &::core::option::Option<super::super::Foundation::Collections::IVector<WiFiDirectInformationElement>>) -> ::windows::core::Result<()>;
    fn ListenStateDiscoverability(&mut self) -> ::windows::core::Result<WiFiDirectAdvertisementListenStateDiscoverability>;
    fn SetListenStateDiscoverability(&mut self, value: WiFiDirectAdvertisementListenStateDiscoverability) -> ::windows::core::Result<()>;
    fn IsAutonomousGroupOwnerEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsAutonomousGroupOwnerEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn LegacySettings(&mut self) -> ::windows::core::Result<WiFiDirectLegacySettings>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWiFiDirectAdvertisement {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.IWiFiDirectAdvertisement";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IWiFiDirectAdvertisementVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiFiDirectAdvertisementImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWiFiDirectAdvertisementVtbl {
        unsafe extern "system" fn InformationElements<Impl: IWiFiDirectAdvertisementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InformationElements() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInformationElements<Impl: IWiFiDirectAdvertisementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInformationElements(&*(&value as *const <super::super::Foundation::Collections::IVector<WiFiDirectInformationElement> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IVector<WiFiDirectInformationElement> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ListenStateDiscoverability<Impl: IWiFiDirectAdvertisementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut WiFiDirectAdvertisementListenStateDiscoverability) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ListenStateDiscoverability() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetListenStateDiscoverability<Impl: IWiFiDirectAdvertisementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: WiFiDirectAdvertisementListenStateDiscoverability) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetListenStateDiscoverability(value).into()
        }
        unsafe extern "system" fn IsAutonomousGroupOwnerEnabled<Impl: IWiFiDirectAdvertisementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAutonomousGroupOwnerEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsAutonomousGroupOwnerEnabled<Impl: IWiFiDirectAdvertisementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsAutonomousGroupOwnerEnabled(value).into()
        }
        unsafe extern "system" fn LegacySettings<Impl: IWiFiDirectAdvertisementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LegacySettings() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWiFiDirectAdvertisement, BASE_OFFSET>(),
            InformationElements: InformationElements::<Impl, IMPL_OFFSET>,
            SetInformationElements: SetInformationElements::<Impl, IMPL_OFFSET>,
            ListenStateDiscoverability: ListenStateDiscoverability::<Impl, IMPL_OFFSET>,
            SetListenStateDiscoverability: SetListenStateDiscoverability::<Impl, IMPL_OFFSET>,
            IsAutonomousGroupOwnerEnabled: IsAutonomousGroupOwnerEnabled::<Impl, IMPL_OFFSET>,
            SetIsAutonomousGroupOwnerEnabled: SetIsAutonomousGroupOwnerEnabled::<Impl, IMPL_OFFSET>,
            LegacySettings: LegacySettings::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiFiDirectAdvertisement as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IWiFiDirectAdvertisement2Impl: Sized {
    fn SupportedConfigurationMethods(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<WiFiDirectConfigurationMethod>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWiFiDirectAdvertisement2 {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.IWiFiDirectAdvertisement2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IWiFiDirectAdvertisement2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiFiDirectAdvertisement2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWiFiDirectAdvertisement2Vtbl {
        unsafe extern "system" fn SupportedConfigurationMethods<Impl: IWiFiDirectAdvertisement2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedConfigurationMethods() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWiFiDirectAdvertisement2, BASE_OFFSET>(),
            SupportedConfigurationMethods: SupportedConfigurationMethods::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiFiDirectAdvertisement2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IWiFiDirectAdvertisementPublisherImpl: Sized {
    fn Advertisement(&mut self) -> ::windows::core::Result<WiFiDirectAdvertisement>;
    fn Status(&mut self) -> ::windows::core::Result<WiFiDirectAdvertisementPublisherStatus>;
    fn StatusChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<WiFiDirectAdvertisementPublisher, WiFiDirectAdvertisementPublisherStatusChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStatusChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Start(&mut self) -> ::windows::core::Result<()>;
    fn Stop(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWiFiDirectAdvertisementPublisher {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.IWiFiDirectAdvertisementPublisher";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IWiFiDirectAdvertisementPublisherVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiFiDirectAdvertisementPublisherImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWiFiDirectAdvertisementPublisherVtbl {
        unsafe extern "system" fn Advertisement<Impl: IWiFiDirectAdvertisementPublisherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Advertisement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: IWiFiDirectAdvertisementPublisherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut WiFiDirectAdvertisementPublisherStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusChanged<Impl: IWiFiDirectAdvertisementPublisherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<WiFiDirectAdvertisementPublisher, WiFiDirectAdvertisementPublisherStatusChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<WiFiDirectAdvertisementPublisher, WiFiDirectAdvertisementPublisherStatusChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStatusChanged<Impl: IWiFiDirectAdvertisementPublisherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStatusChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Start<Impl: IWiFiDirectAdvertisementPublisherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Stop<Impl: IWiFiDirectAdvertisementPublisherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWiFiDirectAdvertisementPublisher, BASE_OFFSET>(),
            Advertisement: Advertisement::<Impl, IMPL_OFFSET>,
            Status: Status::<Impl, IMPL_OFFSET>,
            StatusChanged: StatusChanged::<Impl, IMPL_OFFSET>,
            RemoveStatusChanged: RemoveStatusChanged::<Impl, IMPL_OFFSET>,
            Start: Start::<Impl, IMPL_OFFSET>,
            Stop: Stop::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiFiDirectAdvertisementPublisher as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWiFiDirectAdvertisementPublisherStatusChangedEventArgsImpl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<WiFiDirectAdvertisementPublisherStatus>;
    fn Error(&mut self) -> ::windows::core::Result<WiFiDirectError>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWiFiDirectAdvertisementPublisherStatusChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.IWiFiDirectAdvertisementPublisherStatusChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IWiFiDirectAdvertisementPublisherStatusChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiFiDirectAdvertisementPublisherStatusChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWiFiDirectAdvertisementPublisherStatusChangedEventArgsVtbl {
        unsafe extern "system" fn Status<Impl: IWiFiDirectAdvertisementPublisherStatusChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut WiFiDirectAdvertisementPublisherStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Error<Impl: IWiFiDirectAdvertisementPublisherStatusChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut WiFiDirectError) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Error() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWiFiDirectAdvertisementPublisherStatusChangedEventArgs, BASE_OFFSET>(),
            Status: Status::<Impl, IMPL_OFFSET>,
            Error: Error::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiFiDirectAdvertisementPublisherStatusChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IWiFiDirectConnectionListenerImpl: Sized {
    fn ConnectionRequested(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<WiFiDirectConnectionListener, WiFiDirectConnectionRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveConnectionRequested(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWiFiDirectConnectionListener {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.IWiFiDirectConnectionListener";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IWiFiDirectConnectionListenerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiFiDirectConnectionListenerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWiFiDirectConnectionListenerVtbl {
        unsafe extern "system" fn ConnectionRequested<Impl: IWiFiDirectConnectionListenerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectionRequested(&*(&handler as *const <super::super::Foundation::TypedEventHandler<WiFiDirectConnectionListener, WiFiDirectConnectionRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<WiFiDirectConnectionListener, WiFiDirectConnectionRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveConnectionRequested<Impl: IWiFiDirectConnectionListenerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveConnectionRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWiFiDirectConnectionListener, BASE_OFFSET>(),
            ConnectionRequested: ConnectionRequested::<Impl, IMPL_OFFSET>,
            RemoveConnectionRequested: RemoveConnectionRequested::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiFiDirectConnectionListener as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWiFiDirectConnectionParametersImpl: Sized {
    fn GroupOwnerIntent(&mut self) -> ::windows::core::Result<i16>;
    fn SetGroupOwnerIntent(&mut self, value: i16) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWiFiDirectConnectionParameters {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.IWiFiDirectConnectionParameters";
}
#[cfg(feature = "implement_exclusive")]
impl IWiFiDirectConnectionParametersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiFiDirectConnectionParametersImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWiFiDirectConnectionParametersVtbl {
        unsafe extern "system" fn GroupOwnerIntent<Impl: IWiFiDirectConnectionParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GroupOwnerIntent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGroupOwnerIntent<Impl: IWiFiDirectConnectionParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGroupOwnerIntent(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWiFiDirectConnectionParameters, BASE_OFFSET>(),
            GroupOwnerIntent: GroupOwnerIntent::<Impl, IMPL_OFFSET>,
            SetGroupOwnerIntent: SetGroupOwnerIntent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiFiDirectConnectionParameters as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IWiFiDirectConnectionParameters2Impl: Sized {
    fn PreferenceOrderedConfigurationMethods(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<WiFiDirectConfigurationMethod>>;
    fn PreferredPairingProcedure(&mut self) -> ::windows::core::Result<WiFiDirectPairingProcedure>;
    fn SetPreferredPairingProcedure(&mut self, value: WiFiDirectPairingProcedure) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWiFiDirectConnectionParameters2 {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.IWiFiDirectConnectionParameters2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IWiFiDirectConnectionParameters2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiFiDirectConnectionParameters2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWiFiDirectConnectionParameters2Vtbl {
        unsafe extern "system" fn PreferenceOrderedConfigurationMethods<Impl: IWiFiDirectConnectionParameters2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreferenceOrderedConfigurationMethods() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreferredPairingProcedure<Impl: IWiFiDirectConnectionParameters2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut WiFiDirectPairingProcedure) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreferredPairingProcedure() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreferredPairingProcedure<Impl: IWiFiDirectConnectionParameters2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: WiFiDirectPairingProcedure) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPreferredPairingProcedure(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWiFiDirectConnectionParameters2, BASE_OFFSET>(),
            PreferenceOrderedConfigurationMethods: PreferenceOrderedConfigurationMethods::<Impl, IMPL_OFFSET>,
            PreferredPairingProcedure: PreferredPairingProcedure::<Impl, IMPL_OFFSET>,
            SetPreferredPairingProcedure: SetPreferredPairingProcedure::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiFiDirectConnectionParameters2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Enumeration", feature = "implement_exclusive"))]
pub trait IWiFiDirectConnectionParametersStaticsImpl: Sized {
    fn GetDevicePairingKinds(&mut self, configurationmethod: WiFiDirectConfigurationMethod) -> ::windows::core::Result<super::Enumeration::DevicePairingKinds>;
}
#[cfg(all(feature = "Devices_Enumeration", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWiFiDirectConnectionParametersStatics {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.IWiFiDirectConnectionParametersStatics";
}
#[cfg(all(feature = "Devices_Enumeration", feature = "implement_exclusive"))]
impl IWiFiDirectConnectionParametersStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiFiDirectConnectionParametersStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWiFiDirectConnectionParametersStaticsVtbl {
        unsafe extern "system" fn GetDevicePairingKinds<Impl: IWiFiDirectConnectionParametersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, configurationmethod: WiFiDirectConfigurationMethod, result__: *mut super::Enumeration::DevicePairingKinds) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDevicePairingKinds(configurationmethod) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWiFiDirectConnectionParametersStatics, BASE_OFFSET>(),
            GetDevicePairingKinds: GetDevicePairingKinds::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiFiDirectConnectionParametersStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "implement_exclusive"))]
pub trait IWiFiDirectConnectionRequestImpl: Sized + IClosableImpl {
    fn DeviceInformation(&mut self) -> ::windows::core::Result<super::Enumeration::DeviceInformation>;
}
#[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWiFiDirectConnectionRequest {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.IWiFiDirectConnectionRequest";
}
#[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "implement_exclusive"))]
impl IWiFiDirectConnectionRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiFiDirectConnectionRequestImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWiFiDirectConnectionRequestVtbl {
        unsafe extern "system" fn DeviceInformation<Impl: IWiFiDirectConnectionRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceInformation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWiFiDirectConnectionRequest, BASE_OFFSET>(),
            DeviceInformation: DeviceInformation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiFiDirectConnectionRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWiFiDirectConnectionRequestedEventArgsImpl: Sized {
    fn GetConnectionRequest(&mut self) -> ::windows::core::Result<WiFiDirectConnectionRequest>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWiFiDirectConnectionRequestedEventArgs {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.IWiFiDirectConnectionRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IWiFiDirectConnectionRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiFiDirectConnectionRequestedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWiFiDirectConnectionRequestedEventArgsVtbl {
        unsafe extern "system" fn GetConnectionRequest<Impl: IWiFiDirectConnectionRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConnectionRequest() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWiFiDirectConnectionRequestedEventArgs, BASE_OFFSET>(),
            GetConnectionRequest: GetConnectionRequest::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiFiDirectConnectionRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Networking", feature = "implement_exclusive"))]
pub trait IWiFiDirectDeviceImpl: Sized + IClosableImpl {
    fn ConnectionStatus(&mut self) -> ::windows::core::Result<WiFiDirectConnectionStatus>;
    fn DeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ConnectionStatusChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<WiFiDirectDevice, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveConnectionStatusChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetConnectionEndpointPairs(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Networking::EndpointPair>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Networking", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWiFiDirectDevice {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.IWiFiDirectDevice";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Networking", feature = "implement_exclusive"))]
impl IWiFiDirectDeviceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiFiDirectDeviceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWiFiDirectDeviceVtbl {
        unsafe extern "system" fn ConnectionStatus<Impl: IWiFiDirectDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut WiFiDirectConnectionStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectionStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceId<Impl: IWiFiDirectDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectionStatusChanged<Impl: IWiFiDirectDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectionStatusChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<WiFiDirectDevice, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<WiFiDirectDevice, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveConnectionStatusChanged<Impl: IWiFiDirectDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveConnectionStatusChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetConnectionEndpointPairs<Impl: IWiFiDirectDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConnectionEndpointPairs() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWiFiDirectDevice, BASE_OFFSET>(),
            ConnectionStatus: ConnectionStatus::<Impl, IMPL_OFFSET>,
            DeviceId: DeviceId::<Impl, IMPL_OFFSET>,
            ConnectionStatusChanged: ConnectionStatusChanged::<Impl, IMPL_OFFSET>,
            RemoveConnectionStatusChanged: RemoveConnectionStatusChanged::<Impl, IMPL_OFFSET>,
            GetConnectionEndpointPairs: GetConnectionEndpointPairs::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiFiDirectDevice as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IWiFiDirectDeviceStaticsImpl: Sized {
    fn GetDeviceSelector(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromIdAsync(&mut self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<WiFiDirectDevice>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWiFiDirectDeviceStatics {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.IWiFiDirectDeviceStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IWiFiDirectDeviceStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiFiDirectDeviceStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWiFiDirectDeviceStaticsVtbl {
        unsafe extern "system" fn GetDeviceSelector<Impl: IWiFiDirectDeviceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelector() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromIdAsync<Impl: IWiFiDirectDeviceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromIdAsync(&*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWiFiDirectDeviceStatics, BASE_OFFSET>(),
            GetDeviceSelector: GetDeviceSelector::<Impl, IMPL_OFFSET>,
            FromIdAsync: FromIdAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiFiDirectDeviceStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IWiFiDirectDeviceStatics2Impl: Sized {
    fn GetDeviceSelector(&mut self, r#type: WiFiDirectDeviceSelectorType) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromIdAsync(&mut self, deviceid: &::windows::core::HSTRING, connectionparameters: &::core::option::Option<WiFiDirectConnectionParameters>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<WiFiDirectDevice>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWiFiDirectDeviceStatics2 {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.IWiFiDirectDeviceStatics2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IWiFiDirectDeviceStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiFiDirectDeviceStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWiFiDirectDeviceStatics2Vtbl {
        unsafe extern "system" fn GetDeviceSelector<Impl: IWiFiDirectDeviceStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: WiFiDirectDeviceSelectorType, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelector(r#type) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromIdAsync<Impl: IWiFiDirectDeviceStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, connectionparameters: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromIdAsync(&*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&connectionparameters as *const <WiFiDirectConnectionParameters as ::windows::core::Abi>::Abi as *const <WiFiDirectConnectionParameters as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWiFiDirectDeviceStatics2, BASE_OFFSET>(),
            GetDeviceSelector: GetDeviceSelector::<Impl, IMPL_OFFSET>,
            FromIdAsync: FromIdAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiFiDirectDeviceStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IWiFiDirectInformationElementImpl: Sized {
    fn Oui(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn SetOui(&mut self, value: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn OuiType(&mut self) -> ::windows::core::Result<u8>;
    fn SetOuiType(&mut self, value: u8) -> ::windows::core::Result<()>;
    fn Value(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn SetValue(&mut self, value: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWiFiDirectInformationElement {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.IWiFiDirectInformationElement";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IWiFiDirectInformationElementVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiFiDirectInformationElementImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWiFiDirectInformationElementVtbl {
        unsafe extern "system" fn Oui<Impl: IWiFiDirectInformationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Oui() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOui<Impl: IWiFiDirectInformationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOui(&*(&value as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OuiType<Impl: IWiFiDirectInformationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OuiType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOuiType<Impl: IWiFiDirectInformationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOuiType(value).into()
        }
        unsafe extern "system" fn Value<Impl: IWiFiDirectInformationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: IWiFiDirectInformationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(&*(&value as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWiFiDirectInformationElement, BASE_OFFSET>(),
            Oui: Oui::<Impl, IMPL_OFFSET>,
            SetOui: SetOui::<Impl, IMPL_OFFSET>,
            OuiType: OuiType::<Impl, IMPL_OFFSET>,
            SetOuiType: SetOuiType::<Impl, IMPL_OFFSET>,
            Value: Value::<Impl, IMPL_OFFSET>,
            SetValue: SetValue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiFiDirectInformationElement as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Enumeration", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IWiFiDirectInformationElementStaticsImpl: Sized {
    fn CreateFromBuffer(&mut self, buffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<WiFiDirectInformationElement>>;
    fn CreateFromDeviceInformation(&mut self, deviceinformation: &::core::option::Option<super::Enumeration::DeviceInformation>) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<WiFiDirectInformationElement>>;
}
#[cfg(all(feature = "Devices_Enumeration", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWiFiDirectInformationElementStatics {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.IWiFiDirectInformationElementStatics";
}
#[cfg(all(feature = "Devices_Enumeration", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IWiFiDirectInformationElementStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiFiDirectInformationElementStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWiFiDirectInformationElementStaticsVtbl {
        unsafe extern "system" fn CreateFromBuffer<Impl: IWiFiDirectInformationElementStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromBuffer(&*(&buffer as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromDeviceInformation<Impl: IWiFiDirectInformationElementStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceinformation: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromDeviceInformation(&*(&deviceinformation as *const <super::Enumeration::DeviceInformation as ::windows::core::Abi>::Abi as *const <super::Enumeration::DeviceInformation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWiFiDirectInformationElementStatics, BASE_OFFSET>(),
            CreateFromBuffer: CreateFromBuffer::<Impl, IMPL_OFFSET>,
            CreateFromDeviceInformation: CreateFromDeviceInformation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiFiDirectInformationElementStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Security_Credentials", feature = "implement_exclusive"))]
pub trait IWiFiDirectLegacySettingsImpl: Sized {
    fn IsEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn Ssid(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSsid(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Passphrase(&mut self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential>;
    fn SetPassphrase(&mut self, value: &::core::option::Option<super::super::Security::Credentials::PasswordCredential>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Security_Credentials", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWiFiDirectLegacySettings {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.IWiFiDirectLegacySettings";
}
#[cfg(all(feature = "Security_Credentials", feature = "implement_exclusive"))]
impl IWiFiDirectLegacySettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiFiDirectLegacySettingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWiFiDirectLegacySettingsVtbl {
        unsafe extern "system" fn IsEnabled<Impl: IWiFiDirectLegacySettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsEnabled<Impl: IWiFiDirectLegacySettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsEnabled(value).into()
        }
        unsafe extern "system" fn Ssid<Impl: IWiFiDirectLegacySettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Ssid() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSsid<Impl: IWiFiDirectLegacySettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSsid(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Passphrase<Impl: IWiFiDirectLegacySettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Passphrase() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPassphrase<Impl: IWiFiDirectLegacySettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPassphrase(&*(&value as *const <super::super::Security::Credentials::PasswordCredential as ::windows::core::Abi>::Abi as *const <super::super::Security::Credentials::PasswordCredential as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWiFiDirectLegacySettings, BASE_OFFSET>(),
            IsEnabled: IsEnabled::<Impl, IMPL_OFFSET>,
            SetIsEnabled: SetIsEnabled::<Impl, IMPL_OFFSET>,
            Ssid: Ssid::<Impl, IMPL_OFFSET>,
            SetSsid: SetSsid::<Impl, IMPL_OFFSET>,
            Passphrase: Passphrase::<Impl, IMPL_OFFSET>,
            SetPassphrase: SetPassphrase::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiFiDirectLegacySettings as ::windows::core::Interface>::IID
    }
}
