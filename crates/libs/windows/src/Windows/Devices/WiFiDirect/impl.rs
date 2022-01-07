#[cfg(feature = "implement_exclusive")]
pub trait IWiFiDirectAdvertisementImpl: Sized {
    fn InformationElements(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<WiFiDirectInformationElement>>;
    fn SetInformationElements(&self, value: &::core::option::Option<super::super::Foundation::Collections::IVector<WiFiDirectInformationElement>>) -> ::windows::core::Result<()>;
    fn ListenStateDiscoverability(&self) -> ::windows::core::Result<WiFiDirectAdvertisementListenStateDiscoverability>;
    fn SetListenStateDiscoverability(&self, value: WiFiDirectAdvertisementListenStateDiscoverability) -> ::windows::core::Result<()>;
    fn IsAutonomousGroupOwnerEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsAutonomousGroupOwnerEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn LegacySettings(&self) -> ::windows::core::Result<WiFiDirectLegacySettings>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWiFiDirectAdvertisement {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.IWiFiDirectAdvertisement";
}
#[cfg(feature = "implement_exclusive")]
impl IWiFiDirectAdvertisementVtbl {
    pub const fn new<Impl: IWiFiDirectAdvertisementImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWiFiDirectAdvertisementVtbl {
        unsafe extern "system" fn InformationElements<Impl: IWiFiDirectAdvertisementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InformationElements() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInformationElements<Impl: IWiFiDirectAdvertisementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetInformationElements(&*(&value as *const <super::super::Foundation::Collections::IVector<WiFiDirectInformationElement> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IVector<WiFiDirectInformationElement> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ListenStateDiscoverability<Impl: IWiFiDirectAdvertisementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut WiFiDirectAdvertisementListenStateDiscoverability) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ListenStateDiscoverability() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetListenStateDiscoverability<Impl: IWiFiDirectAdvertisementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: WiFiDirectAdvertisementListenStateDiscoverability) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetListenStateDiscoverability(value).into()
        }
        unsafe extern "system" fn IsAutonomousGroupOwnerEnabled<Impl: IWiFiDirectAdvertisementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsAutonomousGroupOwnerEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsAutonomousGroupOwnerEnabled<Impl: IWiFiDirectAdvertisementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsAutonomousGroupOwnerEnabled(value).into()
        }
        unsafe extern "system" fn LegacySettings<Impl: IWiFiDirectAdvertisementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LegacySettings() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWiFiDirectAdvertisement>, base.5, InformationElements::<Impl, OFFSET>, SetInformationElements::<Impl, OFFSET>, ListenStateDiscoverability::<Impl, OFFSET>, SetListenStateDiscoverability::<Impl, OFFSET>, IsAutonomousGroupOwnerEnabled::<Impl, OFFSET>, SetIsAutonomousGroupOwnerEnabled::<Impl, OFFSET>, LegacySettings::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWiFiDirectAdvertisement2Impl: Sized {
    fn SupportedConfigurationMethods(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<WiFiDirectConfigurationMethod>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWiFiDirectAdvertisement2 {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.IWiFiDirectAdvertisement2";
}
#[cfg(feature = "implement_exclusive")]
impl IWiFiDirectAdvertisement2Vtbl {
    pub const fn new<Impl: IWiFiDirectAdvertisement2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWiFiDirectAdvertisement2Vtbl {
        unsafe extern "system" fn SupportedConfigurationMethods<Impl: IWiFiDirectAdvertisement2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SupportedConfigurationMethods() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWiFiDirectAdvertisement2>, base.5, SupportedConfigurationMethods::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWiFiDirectAdvertisementPublisherImpl: Sized {
    fn Advertisement(&self) -> ::windows::core::Result<WiFiDirectAdvertisement>;
    fn Status(&self) -> ::windows::core::Result<WiFiDirectAdvertisementPublisherStatus>;
    fn StatusChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<WiFiDirectAdvertisementPublisher, WiFiDirectAdvertisementPublisherStatusChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStatusChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWiFiDirectAdvertisementPublisher {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.IWiFiDirectAdvertisementPublisher";
}
#[cfg(feature = "implement_exclusive")]
impl IWiFiDirectAdvertisementPublisherVtbl {
    pub const fn new<Impl: IWiFiDirectAdvertisementPublisherImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWiFiDirectAdvertisementPublisherVtbl {
        unsafe extern "system" fn Advertisement<Impl: IWiFiDirectAdvertisementPublisherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Advertisement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: IWiFiDirectAdvertisementPublisherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut WiFiDirectAdvertisementPublisherStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StatusChanged<Impl: IWiFiDirectAdvertisementPublisherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StatusChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<WiFiDirectAdvertisementPublisher, WiFiDirectAdvertisementPublisherStatusChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<WiFiDirectAdvertisementPublisher, WiFiDirectAdvertisementPublisherStatusChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStatusChanged<Impl: IWiFiDirectAdvertisementPublisherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveStatusChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Start<Impl: IWiFiDirectAdvertisementPublisherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Stop<Impl: IWiFiDirectAdvertisementPublisherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWiFiDirectAdvertisementPublisher>, base.5, Advertisement::<Impl, OFFSET>, Status::<Impl, OFFSET>, StatusChanged::<Impl, OFFSET>, RemoveStatusChanged::<Impl, OFFSET>, Start::<Impl, OFFSET>, Stop::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWiFiDirectAdvertisementPublisherStatusChangedEventArgsImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<WiFiDirectAdvertisementPublisherStatus>;
    fn Error(&self) -> ::windows::core::Result<WiFiDirectError>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWiFiDirectAdvertisementPublisherStatusChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.IWiFiDirectAdvertisementPublisherStatusChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IWiFiDirectAdvertisementPublisherStatusChangedEventArgsVtbl {
    pub const fn new<Impl: IWiFiDirectAdvertisementPublisherStatusChangedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWiFiDirectAdvertisementPublisherStatusChangedEventArgsVtbl {
        unsafe extern "system" fn Status<Impl: IWiFiDirectAdvertisementPublisherStatusChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut WiFiDirectAdvertisementPublisherStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Error<Impl: IWiFiDirectAdvertisementPublisherStatusChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut WiFiDirectError) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Error() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWiFiDirectAdvertisementPublisherStatusChangedEventArgs>, base.5, Status::<Impl, OFFSET>, Error::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWiFiDirectConnectionListenerImpl: Sized {
    fn ConnectionRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<WiFiDirectConnectionListener, WiFiDirectConnectionRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveConnectionRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWiFiDirectConnectionListener {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.IWiFiDirectConnectionListener";
}
#[cfg(feature = "implement_exclusive")]
impl IWiFiDirectConnectionListenerVtbl {
    pub const fn new<Impl: IWiFiDirectConnectionListenerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWiFiDirectConnectionListenerVtbl {
        unsafe extern "system" fn ConnectionRequested<Impl: IWiFiDirectConnectionListenerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ConnectionRequested(&*(&handler as *const <super::super::Foundation::TypedEventHandler<WiFiDirectConnectionListener, WiFiDirectConnectionRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<WiFiDirectConnectionListener, WiFiDirectConnectionRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveConnectionRequested<Impl: IWiFiDirectConnectionListenerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveConnectionRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWiFiDirectConnectionListener>, base.5, ConnectionRequested::<Impl, OFFSET>, RemoveConnectionRequested::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWiFiDirectConnectionParametersImpl: Sized {
    fn GroupOwnerIntent(&self) -> ::windows::core::Result<i16>;
    fn SetGroupOwnerIntent(&self, value: i16) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWiFiDirectConnectionParameters {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.IWiFiDirectConnectionParameters";
}
#[cfg(feature = "implement_exclusive")]
impl IWiFiDirectConnectionParametersVtbl {
    pub const fn new<Impl: IWiFiDirectConnectionParametersImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWiFiDirectConnectionParametersVtbl {
        unsafe extern "system" fn GroupOwnerIntent<Impl: IWiFiDirectConnectionParametersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GroupOwnerIntent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGroupOwnerIntent<Impl: IWiFiDirectConnectionParametersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetGroupOwnerIntent(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWiFiDirectConnectionParameters>, base.5, GroupOwnerIntent::<Impl, OFFSET>, SetGroupOwnerIntent::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWiFiDirectConnectionParameters2Impl: Sized {
    fn PreferenceOrderedConfigurationMethods(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<WiFiDirectConfigurationMethod>>;
    fn PreferredPairingProcedure(&self) -> ::windows::core::Result<WiFiDirectPairingProcedure>;
    fn SetPreferredPairingProcedure(&self, value: WiFiDirectPairingProcedure) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWiFiDirectConnectionParameters2 {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.IWiFiDirectConnectionParameters2";
}
#[cfg(feature = "implement_exclusive")]
impl IWiFiDirectConnectionParameters2Vtbl {
    pub const fn new<Impl: IWiFiDirectConnectionParameters2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWiFiDirectConnectionParameters2Vtbl {
        unsafe extern "system" fn PreferenceOrderedConfigurationMethods<Impl: IWiFiDirectConnectionParameters2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PreferenceOrderedConfigurationMethods() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreferredPairingProcedure<Impl: IWiFiDirectConnectionParameters2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut WiFiDirectPairingProcedure) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PreferredPairingProcedure() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreferredPairingProcedure<Impl: IWiFiDirectConnectionParameters2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: WiFiDirectPairingProcedure) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPreferredPairingProcedure(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWiFiDirectConnectionParameters2>, base.5, PreferenceOrderedConfigurationMethods::<Impl, OFFSET>, PreferredPairingProcedure::<Impl, OFFSET>, SetPreferredPairingProcedure::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWiFiDirectConnectionParametersStaticsImpl: Sized {
    fn GetDevicePairingKinds(&self, configurationmethod: WiFiDirectConfigurationMethod) -> ::windows::core::Result<super::Enumeration::DevicePairingKinds>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWiFiDirectConnectionParametersStatics {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.IWiFiDirectConnectionParametersStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IWiFiDirectConnectionParametersStaticsVtbl {
    pub const fn new<Impl: IWiFiDirectConnectionParametersStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWiFiDirectConnectionParametersStaticsVtbl {
        unsafe extern "system" fn GetDevicePairingKinds<Impl: IWiFiDirectConnectionParametersStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, configurationmethod: WiFiDirectConfigurationMethod, result__: *mut super::Enumeration::DevicePairingKinds) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDevicePairingKinds(configurationmethod) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWiFiDirectConnectionParametersStatics>, base.5, GetDevicePairingKinds::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IWiFiDirectConnectionRequestImpl: Sized + IClosableImpl {
    fn DeviceInformation(&self) -> ::windows::core::Result<super::Enumeration::DeviceInformation>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWiFiDirectConnectionRequest {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.IWiFiDirectConnectionRequest";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IWiFiDirectConnectionRequestVtbl {
    pub const fn new<Impl: IWiFiDirectConnectionRequestImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWiFiDirectConnectionRequestVtbl {
        unsafe extern "system" fn DeviceInformation<Impl: IWiFiDirectConnectionRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeviceInformation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWiFiDirectConnectionRequest>, base.5, DeviceInformation::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWiFiDirectConnectionRequestedEventArgsImpl: Sized {
    fn GetConnectionRequest(&self) -> ::windows::core::Result<WiFiDirectConnectionRequest>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWiFiDirectConnectionRequestedEventArgs {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.IWiFiDirectConnectionRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IWiFiDirectConnectionRequestedEventArgsVtbl {
    pub const fn new<Impl: IWiFiDirectConnectionRequestedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWiFiDirectConnectionRequestedEventArgsVtbl {
        unsafe extern "system" fn GetConnectionRequest<Impl: IWiFiDirectConnectionRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetConnectionRequest() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWiFiDirectConnectionRequestedEventArgs>, base.5, GetConnectionRequest::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IWiFiDirectDeviceImpl: Sized + IClosableImpl {
    fn ConnectionStatus(&self) -> ::windows::core::Result<WiFiDirectConnectionStatus>;
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ConnectionStatusChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<WiFiDirectDevice, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveConnectionStatusChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetConnectionEndpointPairs(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Networking::EndpointPair>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWiFiDirectDevice {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.IWiFiDirectDevice";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IWiFiDirectDeviceVtbl {
    pub const fn new<Impl: IWiFiDirectDeviceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWiFiDirectDeviceVtbl {
        unsafe extern "system" fn ConnectionStatus<Impl: IWiFiDirectDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut WiFiDirectConnectionStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ConnectionStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceId<Impl: IWiFiDirectDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectionStatusChanged<Impl: IWiFiDirectDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ConnectionStatusChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<WiFiDirectDevice, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<WiFiDirectDevice, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveConnectionStatusChanged<Impl: IWiFiDirectDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveConnectionStatusChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetConnectionEndpointPairs<Impl: IWiFiDirectDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetConnectionEndpointPairs() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWiFiDirectDevice>, base.5, ConnectionStatus::<Impl, OFFSET>, DeviceId::<Impl, OFFSET>, ConnectionStatusChanged::<Impl, OFFSET>, RemoveConnectionStatusChanged::<Impl, OFFSET>, GetConnectionEndpointPairs::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWiFiDirectDeviceStaticsImpl: Sized {
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<WiFiDirectDevice>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWiFiDirectDeviceStatics {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.IWiFiDirectDeviceStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IWiFiDirectDeviceStaticsVtbl {
    pub const fn new<Impl: IWiFiDirectDeviceStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWiFiDirectDeviceStaticsVtbl {
        unsafe extern "system" fn GetDeviceSelector<Impl: IWiFiDirectDeviceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelector() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromIdAsync<Impl: IWiFiDirectDeviceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromIdAsync(&*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWiFiDirectDeviceStatics>, base.5, GetDeviceSelector::<Impl, OFFSET>, FromIdAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWiFiDirectDeviceStatics2Impl: Sized {
    fn GetDeviceSelector(&self, r#type: WiFiDirectDeviceSelectorType) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING, connectionparameters: &::core::option::Option<WiFiDirectConnectionParameters>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<WiFiDirectDevice>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWiFiDirectDeviceStatics2 {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.IWiFiDirectDeviceStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IWiFiDirectDeviceStatics2Vtbl {
    pub const fn new<Impl: IWiFiDirectDeviceStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWiFiDirectDeviceStatics2Vtbl {
        unsafe extern "system" fn GetDeviceSelector<Impl: IWiFiDirectDeviceStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: WiFiDirectDeviceSelectorType, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelector(r#type) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromIdAsync<Impl: IWiFiDirectDeviceStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, connectionparameters: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromIdAsync(&*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&connectionparameters as *const <WiFiDirectConnectionParameters as ::windows::core::Abi>::Abi as *const <WiFiDirectConnectionParameters as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWiFiDirectDeviceStatics2>, base.5, GetDeviceSelector::<Impl, OFFSET>, FromIdAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWiFiDirectInformationElementImpl: Sized {
    fn Oui(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn SetOui(&self, value: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn OuiType(&self) -> ::windows::core::Result<u8>;
    fn SetOuiType(&self, value: u8) -> ::windows::core::Result<()>;
    fn Value(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn SetValue(&self, value: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWiFiDirectInformationElement {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.IWiFiDirectInformationElement";
}
#[cfg(feature = "implement_exclusive")]
impl IWiFiDirectInformationElementVtbl {
    pub const fn new<Impl: IWiFiDirectInformationElementImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWiFiDirectInformationElementVtbl {
        unsafe extern "system" fn Oui<Impl: IWiFiDirectInformationElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Oui() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOui<Impl: IWiFiDirectInformationElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetOui(&*(&value as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OuiType<Impl: IWiFiDirectInformationElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OuiType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOuiType<Impl: IWiFiDirectInformationElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetOuiType(value).into()
        }
        unsafe extern "system" fn Value<Impl: IWiFiDirectInformationElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: IWiFiDirectInformationElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetValue(&*(&value as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWiFiDirectInformationElement>, base.5, Oui::<Impl, OFFSET>, SetOui::<Impl, OFFSET>, OuiType::<Impl, OFFSET>, SetOuiType::<Impl, OFFSET>, Value::<Impl, OFFSET>, SetValue::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWiFiDirectInformationElementStaticsImpl: Sized {
    fn CreateFromBuffer(&self, buffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<WiFiDirectInformationElement>>;
    fn CreateFromDeviceInformation(&self, deviceinformation: &::core::option::Option<super::Enumeration::DeviceInformation>) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<WiFiDirectInformationElement>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWiFiDirectInformationElementStatics {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.IWiFiDirectInformationElementStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IWiFiDirectInformationElementStaticsVtbl {
    pub const fn new<Impl: IWiFiDirectInformationElementStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWiFiDirectInformationElementStaticsVtbl {
        unsafe extern "system" fn CreateFromBuffer<Impl: IWiFiDirectInformationElementStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, buffer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFromBuffer(&*(&buffer as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromDeviceInformation<Impl: IWiFiDirectInformationElementStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deviceinformation: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFromDeviceInformation(&*(&deviceinformation as *const <super::Enumeration::DeviceInformation as ::windows::core::Abi>::Abi as *const <super::Enumeration::DeviceInformation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWiFiDirectInformationElementStatics>, base.5, CreateFromBuffer::<Impl, OFFSET>, CreateFromDeviceInformation::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWiFiDirectLegacySettingsImpl: Sized {
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn Ssid(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSsid(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Passphrase(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential>;
    fn SetPassphrase(&self, value: &::core::option::Option<super::super::Security::Credentials::PasswordCredential>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWiFiDirectLegacySettings {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.IWiFiDirectLegacySettings";
}
#[cfg(feature = "implement_exclusive")]
impl IWiFiDirectLegacySettingsVtbl {
    pub const fn new<Impl: IWiFiDirectLegacySettingsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWiFiDirectLegacySettingsVtbl {
        unsafe extern "system" fn IsEnabled<Impl: IWiFiDirectLegacySettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsEnabled<Impl: IWiFiDirectLegacySettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsEnabled(value).into()
        }
        unsafe extern "system" fn Ssid<Impl: IWiFiDirectLegacySettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Ssid() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSsid<Impl: IWiFiDirectLegacySettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSsid(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Passphrase<Impl: IWiFiDirectLegacySettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Passphrase() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPassphrase<Impl: IWiFiDirectLegacySettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPassphrase(&*(&value as *const <super::super::Security::Credentials::PasswordCredential as ::windows::core::Abi>::Abi as *const <super::super::Security::Credentials::PasswordCredential as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWiFiDirectLegacySettings>, base.5, IsEnabled::<Impl, OFFSET>, SetIsEnabled::<Impl, OFFSET>, Ssid::<Impl, OFFSET>, SetSsid::<Impl, OFFSET>, Passphrase::<Impl, OFFSET>, SetPassphrase::<Impl, OFFSET>)
    }
}
