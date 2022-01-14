#[cfg(all(feature = "Devices_Bluetooth_Advertisement", feature = "implement_exclusive"))]
pub trait IBluetoothLEAdvertisementPublisherTriggerDetails_Impl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<super::Advertisement::BluetoothLEAdvertisementPublisherStatus>;
    fn Error(&mut self) -> ::windows::core::Result<super::BluetoothError>;
}
#[cfg(all(feature = "Devices_Bluetooth_Advertisement", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBluetoothLEAdvertisementPublisherTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.IBluetoothLEAdvertisementPublisherTriggerDetails";
}
#[cfg(all(feature = "Devices_Bluetooth_Advertisement", feature = "implement_exclusive"))]
impl IBluetoothLEAdvertisementPublisherTriggerDetails_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBluetoothLEAdvertisementPublisherTriggerDetails_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBluetoothLEAdvertisementPublisherTriggerDetails_Vtbl {
        unsafe extern "system" fn Status<Impl: IBluetoothLEAdvertisementPublisherTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Advertisement::BluetoothLEAdvertisementPublisherStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Error<Impl: IBluetoothLEAdvertisementPublisherTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::BluetoothError) -> ::windows::core::HRESULT {
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
pub trait IBluetoothLEAdvertisementPublisherTriggerDetails2_Impl: Sized {
    fn SelectedTransmitPowerLevelInDBm(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<i16>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBluetoothLEAdvertisementPublisherTriggerDetails2 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.IBluetoothLEAdvertisementPublisherTriggerDetails2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IBluetoothLEAdvertisementPublisherTriggerDetails2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBluetoothLEAdvertisementPublisherTriggerDetails2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBluetoothLEAdvertisementPublisherTriggerDetails2_Vtbl {
        unsafe extern "system" fn SelectedTransmitPowerLevelInDBm<Impl: IBluetoothLEAdvertisementPublisherTriggerDetails2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IBluetoothLEAdvertisementWatcherTriggerDetails_Impl: Sized {
    fn Error(&mut self) -> ::windows::core::Result<super::BluetoothError>;
    fn Advertisements(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::Advertisement::BluetoothLEAdvertisementReceivedEventArgs>>;
    fn SignalStrengthFilter(&mut self) -> ::windows::core::Result<super::BluetoothSignalStrengthFilter>;
}
#[cfg(all(feature = "Devices_Bluetooth_Advertisement", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBluetoothLEAdvertisementWatcherTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.IBluetoothLEAdvertisementWatcherTriggerDetails";
}
#[cfg(all(feature = "Devices_Bluetooth_Advertisement", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IBluetoothLEAdvertisementWatcherTriggerDetails_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBluetoothLEAdvertisementWatcherTriggerDetails_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBluetoothLEAdvertisementWatcherTriggerDetails_Vtbl {
        unsafe extern "system" fn Error<Impl: IBluetoothLEAdvertisementWatcherTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::BluetoothError) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Advertisements<Impl: IBluetoothLEAdvertisementWatcherTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SignalStrengthFilter<Impl: IBluetoothLEAdvertisementWatcherTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IGattCharacteristicNotificationTriggerDetails_Impl: Sized {
    fn Characteristic(&mut self) -> ::windows::core::Result<super::GenericAttributeProfile::GattCharacteristic>;
    fn Value(&mut self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
}
#[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGattCharacteristicNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.IGattCharacteristicNotificationTriggerDetails";
}
#[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IGattCharacteristicNotificationTriggerDetails_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattCharacteristicNotificationTriggerDetails_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattCharacteristicNotificationTriggerDetails_Vtbl {
        unsafe extern "system" fn Characteristic<Impl: IGattCharacteristicNotificationTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Value<Impl: IGattCharacteristicNotificationTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IGattCharacteristicNotificationTriggerDetails2_Impl: Sized {
    fn Error(&mut self) -> ::windows::core::Result<super::BluetoothError>;
    fn EventTriggeringMode(&mut self) -> ::windows::core::Result<BluetoothEventTriggeringMode>;
    fn ValueChangedEvents(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::GenericAttributeProfile::GattValueChangedEventArgs>>;
}
#[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGattCharacteristicNotificationTriggerDetails2 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.IGattCharacteristicNotificationTriggerDetails2";
}
#[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IGattCharacteristicNotificationTriggerDetails2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattCharacteristicNotificationTriggerDetails2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattCharacteristicNotificationTriggerDetails2_Vtbl {
        unsafe extern "system" fn Error<Impl: IGattCharacteristicNotificationTriggerDetails2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::BluetoothError) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn EventTriggeringMode<Impl: IGattCharacteristicNotificationTriggerDetails2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut BluetoothEventTriggeringMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ValueChangedEvents<Impl: IGattCharacteristicNotificationTriggerDetails2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IGattServiceProviderConnection_Impl: Sized {
    fn TriggerId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Service(&mut self) -> ::windows::core::Result<super::GenericAttributeProfile::GattLocalService>;
    fn Start(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGattServiceProviderConnection {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.IGattServiceProviderConnection";
}
#[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "implement_exclusive"))]
impl IGattServiceProviderConnection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattServiceProviderConnection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattServiceProviderConnection_Vtbl {
        unsafe extern "system" fn TriggerId<Impl: IGattServiceProviderConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Service<Impl: IGattServiceProviderConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Start<Impl: IGattServiceProviderConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait IGattServiceProviderConnectionStatics_Impl: Sized {
    fn AllServices(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, GattServiceProviderConnection>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGattServiceProviderConnectionStatics {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.IGattServiceProviderConnectionStatics";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IGattServiceProviderConnectionStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattServiceProviderConnectionStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattServiceProviderConnectionStatics_Vtbl {
        unsafe extern "system" fn AllServices<Impl: IGattServiceProviderConnectionStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IGattServiceProviderTriggerDetails_Impl: Sized {
    fn Connection(&mut self) -> ::windows::core::Result<GattServiceProviderConnection>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattServiceProviderTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.IGattServiceProviderTriggerDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IGattServiceProviderTriggerDetails_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattServiceProviderTriggerDetails_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattServiceProviderTriggerDetails_Vtbl {
        unsafe extern "system" fn Connection<Impl: IGattServiceProviderTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IRfcommConnectionTriggerDetails_Impl: Sized {
    fn Socket(&mut self) -> ::windows::core::Result<super::super::super::Networking::Sockets::StreamSocket>;
    fn Incoming(&mut self) -> ::windows::core::Result<bool>;
    fn RemoteDevice(&mut self) -> ::windows::core::Result<super::BluetoothDevice>;
}
#[cfg(all(feature = "Networking_Sockets", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRfcommConnectionTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.IRfcommConnectionTriggerDetails";
}
#[cfg(all(feature = "Networking_Sockets", feature = "implement_exclusive"))]
impl IRfcommConnectionTriggerDetails_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRfcommConnectionTriggerDetails_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRfcommConnectionTriggerDetails_Vtbl {
        unsafe extern "system" fn Socket<Impl: IRfcommConnectionTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Incoming<Impl: IRfcommConnectionTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoteDevice<Impl: IRfcommConnectionTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IRfcommInboundConnectionInformation_Impl: Sized {
    fn SdpRecord(&mut self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn SetSdpRecord(&mut self, value: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn LocalServiceId(&mut self) -> ::windows::core::Result<super::Rfcomm::RfcommServiceId>;
    fn SetLocalServiceId(&mut self, value: &::core::option::Option<super::Rfcomm::RfcommServiceId>) -> ::windows::core::Result<()>;
    fn ServiceCapabilities(&mut self) -> ::windows::core::Result<super::BluetoothServiceCapabilities>;
    fn SetServiceCapabilities(&mut self, value: super::BluetoothServiceCapabilities) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRfcommInboundConnectionInformation {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.IRfcommInboundConnectionInformation";
}
#[cfg(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IRfcommInboundConnectionInformation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRfcommInboundConnectionInformation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRfcommInboundConnectionInformation_Vtbl {
        unsafe extern "system" fn SdpRecord<Impl: IRfcommInboundConnectionInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSdpRecord<Impl: IRfcommInboundConnectionInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSdpRecord(&*(&value as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LocalServiceId<Impl: IRfcommInboundConnectionInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetLocalServiceId<Impl: IRfcommInboundConnectionInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLocalServiceId(&*(&value as *const <super::Rfcomm::RfcommServiceId as ::windows::core::Abi>::Abi as *const <super::Rfcomm::RfcommServiceId as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ServiceCapabilities<Impl: IRfcommInboundConnectionInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::BluetoothServiceCapabilities) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetServiceCapabilities<Impl: IRfcommInboundConnectionInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::BluetoothServiceCapabilities) -> ::windows::core::HRESULT {
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
pub trait IRfcommOutboundConnectionInformation_Impl: Sized {
    fn RemoteServiceId(&mut self) -> ::windows::core::Result<super::Rfcomm::RfcommServiceId>;
    fn SetRemoteServiceId(&mut self, value: &::core::option::Option<super::Rfcomm::RfcommServiceId>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Devices_Bluetooth_Rfcomm", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRfcommOutboundConnectionInformation {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Background.IRfcommOutboundConnectionInformation";
}
#[cfg(all(feature = "Devices_Bluetooth_Rfcomm", feature = "implement_exclusive"))]
impl IRfcommOutboundConnectionInformation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRfcommOutboundConnectionInformation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRfcommOutboundConnectionInformation_Vtbl {
        unsafe extern "system" fn RemoteServiceId<Impl: IRfcommOutboundConnectionInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRemoteServiceId<Impl: IRfcommOutboundConnectionInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
