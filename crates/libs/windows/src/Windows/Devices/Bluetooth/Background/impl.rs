#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEAdvertisementPublisherTriggerDetailsImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<super::Advertisement::BluetoothLEAdvertisementPublisherStatus>;
    fn Error(&self) -> ::windows::core::Result<super::BluetoothError>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBluetoothLEAdvertisementPublisherTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.IBluetoothLEAdvertisementPublisherTriggerDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IBluetoothLEAdvertisementPublisherTriggerDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBluetoothLEAdvertisementPublisherTriggerDetailsImpl, const OFFSET: isize>() -> IBluetoothLEAdvertisementPublisherTriggerDetailsVtbl {
        unsafe extern "system" fn Status<Impl: IBluetoothLEAdvertisementPublisherTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Advertisement::BluetoothLEAdvertisementPublisherStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Error<Impl: IBluetoothLEAdvertisementPublisherTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::BluetoothError) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBluetoothLEAdvertisementPublisherTriggerDetails>, ::windows::core::GetTrustLevel, Status::<Impl, OFFSET>, Error::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEAdvertisementPublisherTriggerDetails2Impl: Sized {
    fn SelectedTransmitPowerLevelInDBm(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<i16>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBluetoothLEAdvertisementPublisherTriggerDetails2 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.IBluetoothLEAdvertisementPublisherTriggerDetails2";
}
#[cfg(feature = "implement_exclusive")]
impl IBluetoothLEAdvertisementPublisherTriggerDetails2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBluetoothLEAdvertisementPublisherTriggerDetails2Impl, const OFFSET: isize>() -> IBluetoothLEAdvertisementPublisherTriggerDetails2Vtbl {
        unsafe extern "system" fn SelectedTransmitPowerLevelInDBm<Impl: IBluetoothLEAdvertisementPublisherTriggerDetails2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedTransmitPowerLevelInDBm() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBluetoothLEAdvertisementPublisherTriggerDetails2>, ::windows::core::GetTrustLevel, SelectedTransmitPowerLevelInDBm::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEAdvertisementWatcherTriggerDetailsImpl: Sized {
    fn Error(&self) -> ::windows::core::Result<super::BluetoothError>;
    fn Advertisements(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::Advertisement::BluetoothLEAdvertisementReceivedEventArgs>>;
    fn SignalStrengthFilter(&self) -> ::windows::core::Result<super::BluetoothSignalStrengthFilter>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBluetoothLEAdvertisementWatcherTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.IBluetoothLEAdvertisementWatcherTriggerDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IBluetoothLEAdvertisementWatcherTriggerDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBluetoothLEAdvertisementWatcherTriggerDetailsImpl, const OFFSET: isize>() -> IBluetoothLEAdvertisementWatcherTriggerDetailsVtbl {
        unsafe extern "system" fn Error<Impl: IBluetoothLEAdvertisementWatcherTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::BluetoothError) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Advertisements<Impl: IBluetoothLEAdvertisementWatcherTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Advertisements() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SignalStrengthFilter<Impl: IBluetoothLEAdvertisementWatcherTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SignalStrengthFilter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBluetoothLEAdvertisementWatcherTriggerDetails>, ::windows::core::GetTrustLevel, Error::<Impl, OFFSET>, Advertisements::<Impl, OFFSET>, SignalStrengthFilter::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattCharacteristicNotificationTriggerDetailsImpl: Sized {
    fn Characteristic(&self) -> ::windows::core::Result<super::GenericAttributeProfile::GattCharacteristic>;
    fn Value(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattCharacteristicNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.IGattCharacteristicNotificationTriggerDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IGattCharacteristicNotificationTriggerDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattCharacteristicNotificationTriggerDetailsImpl, const OFFSET: isize>() -> IGattCharacteristicNotificationTriggerDetailsVtbl {
        unsafe extern "system" fn Characteristic<Impl: IGattCharacteristicNotificationTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Characteristic() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Impl: IGattCharacteristicNotificationTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGattCharacteristicNotificationTriggerDetails>, ::windows::core::GetTrustLevel, Characteristic::<Impl, OFFSET>, Value::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattCharacteristicNotificationTriggerDetails2Impl: Sized {
    fn Error(&self) -> ::windows::core::Result<super::BluetoothError>;
    fn EventTriggeringMode(&self) -> ::windows::core::Result<BluetoothEventTriggeringMode>;
    fn ValueChangedEvents(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::GenericAttributeProfile::GattValueChangedEventArgs>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattCharacteristicNotificationTriggerDetails2 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.IGattCharacteristicNotificationTriggerDetails2";
}
#[cfg(feature = "implement_exclusive")]
impl IGattCharacteristicNotificationTriggerDetails2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattCharacteristicNotificationTriggerDetails2Impl, const OFFSET: isize>() -> IGattCharacteristicNotificationTriggerDetails2Vtbl {
        unsafe extern "system" fn Error<Impl: IGattCharacteristicNotificationTriggerDetails2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::BluetoothError) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn EventTriggeringMode<Impl: IGattCharacteristicNotificationTriggerDetails2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut BluetoothEventTriggeringMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EventTriggeringMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ValueChangedEvents<Impl: IGattCharacteristicNotificationTriggerDetails2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ValueChangedEvents() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGattCharacteristicNotificationTriggerDetails2>, ::windows::core::GetTrustLevel, Error::<Impl, OFFSET>, EventTriggeringMode::<Impl, OFFSET>, ValueChangedEvents::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattServiceProviderConnectionImpl: Sized {
    fn TriggerId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Service(&self) -> ::windows::core::Result<super::GenericAttributeProfile::GattLocalService>;
    fn Start(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattServiceProviderConnection {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.IGattServiceProviderConnection";
}
#[cfg(feature = "implement_exclusive")]
impl IGattServiceProviderConnectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattServiceProviderConnectionImpl, const OFFSET: isize>() -> IGattServiceProviderConnectionVtbl {
        unsafe extern "system" fn TriggerId<Impl: IGattServiceProviderConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TriggerId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Service<Impl: IGattServiceProviderConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Service() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Start<Impl: IGattServiceProviderConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGattServiceProviderConnection>, ::windows::core::GetTrustLevel, TriggerId::<Impl, OFFSET>, Service::<Impl, OFFSET>, Start::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattServiceProviderConnectionStaticsImpl: Sized {
    fn AllServices(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, GattServiceProviderConnection>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattServiceProviderConnectionStatics {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.IGattServiceProviderConnectionStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IGattServiceProviderConnectionStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattServiceProviderConnectionStaticsImpl, const OFFSET: isize>() -> IGattServiceProviderConnectionStaticsVtbl {
        unsafe extern "system" fn AllServices<Impl: IGattServiceProviderConnectionStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllServices() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGattServiceProviderConnectionStatics>, ::windows::core::GetTrustLevel, AllServices::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattServiceProviderTriggerDetailsImpl: Sized {
    fn Connection(&self) -> ::windows::core::Result<GattServiceProviderConnection>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattServiceProviderTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.IGattServiceProviderTriggerDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IGattServiceProviderTriggerDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattServiceProviderTriggerDetailsImpl, const OFFSET: isize>() -> IGattServiceProviderTriggerDetailsVtbl {
        unsafe extern "system" fn Connection<Impl: IGattServiceProviderTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Connection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGattServiceProviderTriggerDetails>, ::windows::core::GetTrustLevel, Connection::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRfcommConnectionTriggerDetailsImpl: Sized {
    fn Socket(&self) -> ::windows::core::Result<super::super::super::Networking::Sockets::StreamSocket>;
    fn Incoming(&self) -> ::windows::core::Result<bool>;
    fn RemoteDevice(&self) -> ::windows::core::Result<super::BluetoothDevice>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRfcommConnectionTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.IRfcommConnectionTriggerDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IRfcommConnectionTriggerDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRfcommConnectionTriggerDetailsImpl, const OFFSET: isize>() -> IRfcommConnectionTriggerDetailsVtbl {
        unsafe extern "system" fn Socket<Impl: IRfcommConnectionTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Socket() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Incoming<Impl: IRfcommConnectionTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Incoming() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoteDevice<Impl: IRfcommConnectionTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoteDevice() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRfcommConnectionTriggerDetails>, ::windows::core::GetTrustLevel, Socket::<Impl, OFFSET>, Incoming::<Impl, OFFSET>, RemoteDevice::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRfcommInboundConnectionInformationImpl: Sized {
    fn SdpRecord(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn SetSdpRecord(&self, value: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn LocalServiceId(&self) -> ::windows::core::Result<super::Rfcomm::RfcommServiceId>;
    fn SetLocalServiceId(&self, value: &::core::option::Option<super::Rfcomm::RfcommServiceId>) -> ::windows::core::Result<()>;
    fn ServiceCapabilities(&self) -> ::windows::core::Result<super::BluetoothServiceCapabilities>;
    fn SetServiceCapabilities(&self, value: super::BluetoothServiceCapabilities) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRfcommInboundConnectionInformation {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.IRfcommInboundConnectionInformation";
}
#[cfg(feature = "implement_exclusive")]
impl IRfcommInboundConnectionInformationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRfcommInboundConnectionInformationImpl, const OFFSET: isize>() -> IRfcommInboundConnectionInformationVtbl {
        unsafe extern "system" fn SdpRecord<Impl: IRfcommInboundConnectionInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SdpRecord() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSdpRecord<Impl: IRfcommInboundConnectionInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSdpRecord(&*(&value as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LocalServiceId<Impl: IRfcommInboundConnectionInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalServiceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalServiceId<Impl: IRfcommInboundConnectionInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLocalServiceId(&*(&value as *const <super::Rfcomm::RfcommServiceId as ::windows::core::Abi>::Abi as *const <super::Rfcomm::RfcommServiceId as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ServiceCapabilities<Impl: IRfcommInboundConnectionInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::BluetoothServiceCapabilities) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceCapabilities() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServiceCapabilities<Impl: IRfcommInboundConnectionInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::BluetoothServiceCapabilities) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetServiceCapabilities(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRfcommInboundConnectionInformation>, ::windows::core::GetTrustLevel, SdpRecord::<Impl, OFFSET>, SetSdpRecord::<Impl, OFFSET>, LocalServiceId::<Impl, OFFSET>, SetLocalServiceId::<Impl, OFFSET>, ServiceCapabilities::<Impl, OFFSET>, SetServiceCapabilities::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRfcommOutboundConnectionInformationImpl: Sized {
    fn RemoteServiceId(&self) -> ::windows::core::Result<super::Rfcomm::RfcommServiceId>;
    fn SetRemoteServiceId(&self, value: &::core::option::Option<super::Rfcomm::RfcommServiceId>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRfcommOutboundConnectionInformation {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.IRfcommOutboundConnectionInformation";
}
#[cfg(feature = "implement_exclusive")]
impl IRfcommOutboundConnectionInformationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRfcommOutboundConnectionInformationImpl, const OFFSET: isize>() -> IRfcommOutboundConnectionInformationVtbl {
        unsafe extern "system" fn RemoteServiceId<Impl: IRfcommOutboundConnectionInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoteServiceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemoteServiceId<Impl: IRfcommOutboundConnectionInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRemoteServiceId(&*(&value as *const <super::Rfcomm::RfcommServiceId as ::windows::core::Abi>::Abi as *const <super::Rfcomm::RfcommServiceId as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRfcommOutboundConnectionInformation>, ::windows::core::GetTrustLevel, RemoteServiceId::<Impl, OFFSET>, SetRemoteServiceId::<Impl, OFFSET>)
    }
}
