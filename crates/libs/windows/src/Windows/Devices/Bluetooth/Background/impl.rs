#[cfg(all(feature = "Devices_Bluetooth_Advertisement", feature = "implement_exclusive"))]
pub trait IBluetoothLEAdvertisementPublisherTriggerDetailsImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<super::Advertisement::BluetoothLEAdvertisementPublisherStatus>;
    fn Error(&self) -> ::windows::core::Result<super::BluetoothError>;
}
#[cfg(all(feature = "Devices_Bluetooth_Advertisement", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBluetoothLEAdvertisementPublisherTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.IBluetoothLEAdvertisementPublisherTriggerDetails";
}
#[cfg(all(feature = "Devices_Bluetooth_Advertisement", feature = "implement_exclusive"))]
impl IBluetoothLEAdvertisementPublisherTriggerDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBluetoothLEAdvertisementPublisherTriggerDetailsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBluetoothLEAdvertisementPublisherTriggerDetailsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBluetoothLEAdvertisementPublisherTriggerDetails, BASE_OFFSET>(),
            Status: Status::<Impl, IMPL_OFFSET>,
            Error: Error::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBluetoothLEAdvertisementPublisherTriggerDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBluetoothLEAdvertisementPublisherTriggerDetails2Impl: Sized {
    fn SelectedTransmitPowerLevelInDBm(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<i16>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBluetoothLEAdvertisementPublisherTriggerDetails2 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.IBluetoothLEAdvertisementPublisherTriggerDetails2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IBluetoothLEAdvertisementPublisherTriggerDetails2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBluetoothLEAdvertisementPublisherTriggerDetails2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBluetoothLEAdvertisementPublisherTriggerDetails2Vtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBluetoothLEAdvertisementPublisherTriggerDetails2, BASE_OFFSET>(),
            SelectedTransmitPowerLevelInDBm: SelectedTransmitPowerLevelInDBm::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBluetoothLEAdvertisementPublisherTriggerDetails2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Bluetooth_Advertisement", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IBluetoothLEAdvertisementWatcherTriggerDetailsImpl: Sized {
    fn Error(&self) -> ::windows::core::Result<super::BluetoothError>;
    fn Advertisements(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::Advertisement::BluetoothLEAdvertisementReceivedEventArgs>>;
    fn SignalStrengthFilter(&self) -> ::windows::core::Result<super::BluetoothSignalStrengthFilter>;
}
#[cfg(all(feature = "Devices_Bluetooth_Advertisement", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBluetoothLEAdvertisementWatcherTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.IBluetoothLEAdvertisementWatcherTriggerDetails";
}
#[cfg(all(feature = "Devices_Bluetooth_Advertisement", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IBluetoothLEAdvertisementWatcherTriggerDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBluetoothLEAdvertisementWatcherTriggerDetailsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBluetoothLEAdvertisementWatcherTriggerDetailsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBluetoothLEAdvertisementWatcherTriggerDetails, BASE_OFFSET>(),
            Error: Error::<Impl, IMPL_OFFSET>,
            Advertisements: Advertisements::<Impl, IMPL_OFFSET>,
            SignalStrengthFilter: SignalStrengthFilter::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBluetoothLEAdvertisementWatcherTriggerDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IGattCharacteristicNotificationTriggerDetailsImpl: Sized {
    fn Characteristic(&self) -> ::windows::core::Result<super::GenericAttributeProfile::GattCharacteristic>;
    fn Value(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
}
#[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGattCharacteristicNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.IGattCharacteristicNotificationTriggerDetails";
}
#[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IGattCharacteristicNotificationTriggerDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattCharacteristicNotificationTriggerDetailsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattCharacteristicNotificationTriggerDetailsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGattCharacteristicNotificationTriggerDetails, BASE_OFFSET>(),
            Characteristic: Characteristic::<Impl, IMPL_OFFSET>,
            Value: Value::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGattCharacteristicNotificationTriggerDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IGattCharacteristicNotificationTriggerDetails2Impl: Sized {
    fn Error(&self) -> ::windows::core::Result<super::BluetoothError>;
    fn EventTriggeringMode(&self) -> ::windows::core::Result<BluetoothEventTriggeringMode>;
    fn ValueChangedEvents(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::GenericAttributeProfile::GattValueChangedEventArgs>>;
}
#[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGattCharacteristicNotificationTriggerDetails2 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.IGattCharacteristicNotificationTriggerDetails2";
}
#[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IGattCharacteristicNotificationTriggerDetails2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattCharacteristicNotificationTriggerDetails2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattCharacteristicNotificationTriggerDetails2Vtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGattCharacteristicNotificationTriggerDetails2, BASE_OFFSET>(),
            Error: Error::<Impl, IMPL_OFFSET>,
            EventTriggeringMode: EventTriggeringMode::<Impl, IMPL_OFFSET>,
            ValueChangedEvents: ValueChangedEvents::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGattCharacteristicNotificationTriggerDetails2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "implement_exclusive"))]
pub trait IGattServiceProviderConnectionImpl: Sized {
    fn TriggerId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Service(&self) -> ::windows::core::Result<super::GenericAttributeProfile::GattLocalService>;
    fn Start(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGattServiceProviderConnection {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.IGattServiceProviderConnection";
}
#[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "implement_exclusive"))]
impl IGattServiceProviderConnectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattServiceProviderConnectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattServiceProviderConnectionVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGattServiceProviderConnection, BASE_OFFSET>(),
            TriggerId: TriggerId::<Impl, IMPL_OFFSET>,
            Service: Service::<Impl, IMPL_OFFSET>,
            Start: Start::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGattServiceProviderConnection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IGattServiceProviderConnectionStaticsImpl: Sized {
    fn AllServices(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, GattServiceProviderConnection>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGattServiceProviderConnectionStatics {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.IGattServiceProviderConnectionStatics";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IGattServiceProviderConnectionStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattServiceProviderConnectionStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattServiceProviderConnectionStaticsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGattServiceProviderConnectionStatics, BASE_OFFSET>(),
            AllServices: AllServices::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGattServiceProviderConnectionStatics as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattServiceProviderTriggerDetailsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattServiceProviderTriggerDetailsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGattServiceProviderTriggerDetails, BASE_OFFSET>(),
            Connection: Connection::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGattServiceProviderTriggerDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Networking_Sockets", feature = "implement_exclusive"))]
pub trait IRfcommConnectionTriggerDetailsImpl: Sized {
    fn Socket(&self) -> ::windows::core::Result<super::super::super::Networking::Sockets::StreamSocket>;
    fn Incoming(&self) -> ::windows::core::Result<bool>;
    fn RemoteDevice(&self) -> ::windows::core::Result<super::BluetoothDevice>;
}
#[cfg(all(feature = "Networking_Sockets", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRfcommConnectionTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.IRfcommConnectionTriggerDetails";
}
#[cfg(all(feature = "Networking_Sockets", feature = "implement_exclusive"))]
impl IRfcommConnectionTriggerDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRfcommConnectionTriggerDetailsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRfcommConnectionTriggerDetailsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRfcommConnectionTriggerDetails, BASE_OFFSET>(),
            Socket: Socket::<Impl, IMPL_OFFSET>,
            Incoming: Incoming::<Impl, IMPL_OFFSET>,
            RemoteDevice: RemoteDevice::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRfcommConnectionTriggerDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IRfcommInboundConnectionInformationImpl: Sized {
    fn SdpRecord(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn SetSdpRecord(&self, value: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn LocalServiceId(&self) -> ::windows::core::Result<super::Rfcomm::RfcommServiceId>;
    fn SetLocalServiceId(&self, value: &::core::option::Option<super::Rfcomm::RfcommServiceId>) -> ::windows::core::Result<()>;
    fn ServiceCapabilities(&self) -> ::windows::core::Result<super::BluetoothServiceCapabilities>;
    fn SetServiceCapabilities(&self, value: super::BluetoothServiceCapabilities) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRfcommInboundConnectionInformation {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.IRfcommInboundConnectionInformation";
}
#[cfg(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IRfcommInboundConnectionInformationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRfcommInboundConnectionInformationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRfcommInboundConnectionInformationVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRfcommInboundConnectionInformation, BASE_OFFSET>(),
            SdpRecord: SdpRecord::<Impl, IMPL_OFFSET>,
            SetSdpRecord: SetSdpRecord::<Impl, IMPL_OFFSET>,
            LocalServiceId: LocalServiceId::<Impl, IMPL_OFFSET>,
            SetLocalServiceId: SetLocalServiceId::<Impl, IMPL_OFFSET>,
            ServiceCapabilities: ServiceCapabilities::<Impl, IMPL_OFFSET>,
            SetServiceCapabilities: SetServiceCapabilities::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRfcommInboundConnectionInformation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Bluetooth_Rfcomm", feature = "implement_exclusive"))]
pub trait IRfcommOutboundConnectionInformationImpl: Sized {
    fn RemoteServiceId(&self) -> ::windows::core::Result<super::Rfcomm::RfcommServiceId>;
    fn SetRemoteServiceId(&self, value: &::core::option::Option<super::Rfcomm::RfcommServiceId>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Devices_Bluetooth_Rfcomm", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRfcommOutboundConnectionInformation {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.IRfcommOutboundConnectionInformation";
}
#[cfg(all(feature = "Devices_Bluetooth_Rfcomm", feature = "implement_exclusive"))]
impl IRfcommOutboundConnectionInformationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRfcommOutboundConnectionInformationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRfcommOutboundConnectionInformationVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRfcommOutboundConnectionInformation, BASE_OFFSET>(),
            RemoteServiceId: RemoteServiceId::<Impl, IMPL_OFFSET>,
            SetRemoteServiceId: SetRemoteServiceId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRfcommOutboundConnectionInformation as ::windows::core::Interface>::IID
    }
}
