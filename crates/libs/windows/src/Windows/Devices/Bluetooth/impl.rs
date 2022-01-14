#[cfg(all(feature = "Devices_Radios", feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBluetoothAdapter_Impl: Sized {
    fn DeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn BluetoothAddress(&mut self) -> ::windows::core::Result<u64>;
    fn IsClassicSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsLowEnergySupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsPeripheralRoleSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsCentralRoleSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsAdvertisementOffloadSupported(&mut self) -> ::windows::core::Result<bool>;
    fn GetRadioAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::Radios::Radio>>;
}
#[cfg(all(feature = "Devices_Radios", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBluetoothAdapter {
    const NAME: &'static str = "Windows.Devices.Bluetooth.IBluetoothAdapter";
}
#[cfg(all(feature = "Devices_Radios", feature = "Foundation", feature = "implement_exclusive"))]
impl IBluetoothAdapter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBluetoothAdapter_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBluetoothAdapter_Vtbl {
        unsafe extern "system" fn DeviceId<Impl: IBluetoothAdapter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn BluetoothAddress<Impl: IBluetoothAdapter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BluetoothAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsClassicSupported<Impl: IBluetoothAdapter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsClassicSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLowEnergySupported<Impl: IBluetoothAdapter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsLowEnergySupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPeripheralRoleSupported<Impl: IBluetoothAdapter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPeripheralRoleSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCentralRoleSupported<Impl: IBluetoothAdapter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCentralRoleSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAdvertisementOffloadSupported<Impl: IBluetoothAdapter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAdvertisementOffloadSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRadioAsync<Impl: IBluetoothAdapter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRadioAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBluetoothAdapter, BASE_OFFSET>(),
            DeviceId: DeviceId::<Impl, IMPL_OFFSET>,
            BluetoothAddress: BluetoothAddress::<Impl, IMPL_OFFSET>,
            IsClassicSupported: IsClassicSupported::<Impl, IMPL_OFFSET>,
            IsLowEnergySupported: IsLowEnergySupported::<Impl, IMPL_OFFSET>,
            IsPeripheralRoleSupported: IsPeripheralRoleSupported::<Impl, IMPL_OFFSET>,
            IsCentralRoleSupported: IsCentralRoleSupported::<Impl, IMPL_OFFSET>,
            IsAdvertisementOffloadSupported: IsAdvertisementOffloadSupported::<Impl, IMPL_OFFSET>,
            GetRadioAsync: GetRadioAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBluetoothAdapter as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothAdapter2_Impl: Sized {
    fn AreClassicSecureConnectionsSupported(&mut self) -> ::windows::core::Result<bool>;
    fn AreLowEnergySecureConnectionsSupported(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBluetoothAdapter2 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.IBluetoothAdapter2";
}
#[cfg(feature = "implement_exclusive")]
impl IBluetoothAdapter2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBluetoothAdapter2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBluetoothAdapter2_Vtbl {
        unsafe extern "system" fn AreClassicSecureConnectionsSupported<Impl: IBluetoothAdapter2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AreClassicSecureConnectionsSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AreLowEnergySecureConnectionsSupported<Impl: IBluetoothAdapter2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AreLowEnergySecureConnectionsSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBluetoothAdapter2, BASE_OFFSET>(),
            AreClassicSecureConnectionsSupported: AreClassicSecureConnectionsSupported::<Impl, IMPL_OFFSET>,
            AreLowEnergySecureConnectionsSupported: AreLowEnergySecureConnectionsSupported::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBluetoothAdapter2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothAdapter3_Impl: Sized {
    fn IsExtendedAdvertisingSupported(&mut self) -> ::windows::core::Result<bool>;
    fn MaxAdvertisementDataLength(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBluetoothAdapter3 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.IBluetoothAdapter3";
}
#[cfg(feature = "implement_exclusive")]
impl IBluetoothAdapter3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBluetoothAdapter3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBluetoothAdapter3_Vtbl {
        unsafe extern "system" fn IsExtendedAdvertisingSupported<Impl: IBluetoothAdapter3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsExtendedAdvertisingSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxAdvertisementDataLength<Impl: IBluetoothAdapter3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxAdvertisementDataLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBluetoothAdapter3, BASE_OFFSET>(),
            IsExtendedAdvertisingSupported: IsExtendedAdvertisingSupported::<Impl, IMPL_OFFSET>,
            MaxAdvertisementDataLength: MaxAdvertisementDataLength::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBluetoothAdapter3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBluetoothAdapterStatics_Impl: Sized {
    fn GetDeviceSelector(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromIdAsync(&mut self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BluetoothAdapter>>;
    fn GetDefaultAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BluetoothAdapter>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBluetoothAdapterStatics {
    const NAME: &'static str = "Windows.Devices.Bluetooth.IBluetoothAdapterStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IBluetoothAdapterStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBluetoothAdapterStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBluetoothAdapterStatics_Vtbl {
        unsafe extern "system" fn GetDeviceSelector<Impl: IBluetoothAdapterStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FromIdAsync<Impl: IBluetoothAdapterStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDefaultAsync<Impl: IBluetoothAdapterStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefaultAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBluetoothAdapterStatics, BASE_OFFSET>(),
            GetDeviceSelector: GetDeviceSelector::<Impl, IMPL_OFFSET>,
            FromIdAsync: FromIdAsync::<Impl, IMPL_OFFSET>,
            GetDefaultAsync: GetDefaultAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBluetoothAdapterStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothClassOfDevice_Impl: Sized {
    fn RawValue(&mut self) -> ::windows::core::Result<u32>;
    fn MajorClass(&mut self) -> ::windows::core::Result<BluetoothMajorClass>;
    fn MinorClass(&mut self) -> ::windows::core::Result<BluetoothMinorClass>;
    fn ServiceCapabilities(&mut self) -> ::windows::core::Result<BluetoothServiceCapabilities>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBluetoothClassOfDevice {
    const NAME: &'static str = "Windows.Devices.Bluetooth.IBluetoothClassOfDevice";
}
#[cfg(feature = "implement_exclusive")]
impl IBluetoothClassOfDevice_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBluetoothClassOfDevice_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBluetoothClassOfDevice_Vtbl {
        unsafe extern "system" fn RawValue<Impl: IBluetoothClassOfDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RawValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MajorClass<Impl: IBluetoothClassOfDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut BluetoothMajorClass) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MajorClass() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinorClass<Impl: IBluetoothClassOfDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut BluetoothMinorClass) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinorClass() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceCapabilities<Impl: IBluetoothClassOfDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut BluetoothServiceCapabilities) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBluetoothClassOfDevice, BASE_OFFSET>(),
            RawValue: RawValue::<Impl, IMPL_OFFSET>,
            MajorClass: MajorClass::<Impl, IMPL_OFFSET>,
            MinorClass: MinorClass::<Impl, IMPL_OFFSET>,
            ServiceCapabilities: ServiceCapabilities::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBluetoothClassOfDevice as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothClassOfDeviceStatics_Impl: Sized {
    fn FromRawValue(&mut self, rawvalue: u32) -> ::windows::core::Result<BluetoothClassOfDevice>;
    fn FromParts(&mut self, majorclass: BluetoothMajorClass, minorclass: BluetoothMinorClass, servicecapabilities: BluetoothServiceCapabilities) -> ::windows::core::Result<BluetoothClassOfDevice>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBluetoothClassOfDeviceStatics {
    const NAME: &'static str = "Windows.Devices.Bluetooth.IBluetoothClassOfDeviceStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IBluetoothClassOfDeviceStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBluetoothClassOfDeviceStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBluetoothClassOfDeviceStatics_Vtbl {
        unsafe extern "system" fn FromRawValue<Impl: IBluetoothClassOfDeviceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rawvalue: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromRawValue(rawvalue) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromParts<Impl: IBluetoothClassOfDeviceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, majorclass: BluetoothMajorClass, minorclass: BluetoothMinorClass, servicecapabilities: BluetoothServiceCapabilities, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromParts(majorclass, minorclass, servicecapabilities) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBluetoothClassOfDeviceStatics, BASE_OFFSET>(),
            FromRawValue: FromRawValue::<Impl, IMPL_OFFSET>,
            FromParts: FromParts::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBluetoothClassOfDeviceStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Networking", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IBluetoothDevice_Impl: Sized {
    fn DeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HostName(&mut self) -> ::windows::core::Result<super::super::Networking::HostName>;
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ClassOfDevice(&mut self) -> ::windows::core::Result<BluetoothClassOfDevice>;
    fn SdpRecords(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Storage::Streams::IBuffer>>;
    fn RfcommServices(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<Rfcomm::RfcommDeviceService>>;
    fn ConnectionStatus(&mut self) -> ::windows::core::Result<BluetoothConnectionStatus>;
    fn BluetoothAddress(&mut self) -> ::windows::core::Result<u64>;
    fn NameChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<BluetoothDevice, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveNameChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SdpRecordsChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<BluetoothDevice, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSdpRecordsChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ConnectionStatusChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<BluetoothDevice, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveConnectionStatusChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Networking", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBluetoothDevice {
    const NAME: &'static str = "Windows.Devices.Bluetooth.IBluetoothDevice";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Networking", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IBluetoothDevice_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBluetoothDevice_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBluetoothDevice_Vtbl {
        unsafe extern "system" fn DeviceId<Impl: IBluetoothDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HostName<Impl: IBluetoothDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HostName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: IBluetoothDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClassOfDevice<Impl: IBluetoothDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClassOfDevice() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SdpRecords<Impl: IBluetoothDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SdpRecords() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RfcommServices<Impl: IBluetoothDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RfcommServices() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectionStatus<Impl: IBluetoothDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut BluetoothConnectionStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn BluetoothAddress<Impl: IBluetoothDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BluetoothAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NameChanged<Impl: IBluetoothDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NameChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<BluetoothDevice, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<BluetoothDevice, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveNameChanged<Impl: IBluetoothDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveNameChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SdpRecordsChanged<Impl: IBluetoothDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SdpRecordsChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<BluetoothDevice, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<BluetoothDevice, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSdpRecordsChanged<Impl: IBluetoothDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSdpRecordsChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ConnectionStatusChanged<Impl: IBluetoothDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectionStatusChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<BluetoothDevice, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<BluetoothDevice, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveConnectionStatusChanged<Impl: IBluetoothDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveConnectionStatusChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBluetoothDevice, BASE_OFFSET>(),
            DeviceId: DeviceId::<Impl, IMPL_OFFSET>,
            HostName: HostName::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            ClassOfDevice: ClassOfDevice::<Impl, IMPL_OFFSET>,
            SdpRecords: SdpRecords::<Impl, IMPL_OFFSET>,
            RfcommServices: RfcommServices::<Impl, IMPL_OFFSET>,
            ConnectionStatus: ConnectionStatus::<Impl, IMPL_OFFSET>,
            BluetoothAddress: BluetoothAddress::<Impl, IMPL_OFFSET>,
            NameChanged: NameChanged::<Impl, IMPL_OFFSET>,
            RemoveNameChanged: RemoveNameChanged::<Impl, IMPL_OFFSET>,
            SdpRecordsChanged: SdpRecordsChanged::<Impl, IMPL_OFFSET>,
            RemoveSdpRecordsChanged: RemoveSdpRecordsChanged::<Impl, IMPL_OFFSET>,
            ConnectionStatusChanged: ConnectionStatusChanged::<Impl, IMPL_OFFSET>,
            RemoveConnectionStatusChanged: RemoveConnectionStatusChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBluetoothDevice as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Enumeration", feature = "implement_exclusive"))]
pub trait IBluetoothDevice2_Impl: Sized {
    fn DeviceInformation(&mut self) -> ::windows::core::Result<super::Enumeration::DeviceInformation>;
}
#[cfg(all(feature = "Devices_Enumeration", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBluetoothDevice2 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.IBluetoothDevice2";
}
#[cfg(all(feature = "Devices_Enumeration", feature = "implement_exclusive"))]
impl IBluetoothDevice2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBluetoothDevice2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBluetoothDevice2_Vtbl {
        unsafe extern "system" fn DeviceInformation<Impl: IBluetoothDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBluetoothDevice2, BASE_OFFSET>(),
            DeviceInformation: DeviceInformation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBluetoothDevice2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Devices_Enumeration", feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBluetoothDevice3_Impl: Sized {
    fn DeviceAccessInformation(&mut self) -> ::windows::core::Result<super::Enumeration::DeviceAccessInformation>;
    fn RequestAccessAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::Enumeration::DeviceAccessStatus>>;
    fn GetRfcommServicesAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Rfcomm::RfcommDeviceServicesResult>>;
    fn GetRfcommServicesWithCacheModeAsync(&mut self, cachemode: BluetoothCacheMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Rfcomm::RfcommDeviceServicesResult>>;
    fn GetRfcommServicesForIdAsync(&mut self, serviceid: &::core::option::Option<Rfcomm::RfcommServiceId>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Rfcomm::RfcommDeviceServicesResult>>;
    fn GetRfcommServicesForIdWithCacheModeAsync(&mut self, serviceid: &::core::option::Option<Rfcomm::RfcommServiceId>, cachemode: BluetoothCacheMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Rfcomm::RfcommDeviceServicesResult>>;
}
#[cfg(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Devices_Enumeration", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBluetoothDevice3 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.IBluetoothDevice3";
}
#[cfg(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Devices_Enumeration", feature = "Foundation", feature = "implement_exclusive"))]
impl IBluetoothDevice3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBluetoothDevice3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBluetoothDevice3_Vtbl {
        unsafe extern "system" fn DeviceAccessInformation<Impl: IBluetoothDevice3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceAccessInformation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestAccessAsync<Impl: IBluetoothDevice3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestAccessAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRfcommServicesAsync<Impl: IBluetoothDevice3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRfcommServicesAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRfcommServicesWithCacheModeAsync<Impl: IBluetoothDevice3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cachemode: BluetoothCacheMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRfcommServicesWithCacheModeAsync(cachemode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRfcommServicesForIdAsync<Impl: IBluetoothDevice3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, serviceid: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRfcommServicesForIdAsync(&*(&serviceid as *const <Rfcomm::RfcommServiceId as ::windows::core::Abi>::Abi as *const <Rfcomm::RfcommServiceId as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRfcommServicesForIdWithCacheModeAsync<Impl: IBluetoothDevice3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, serviceid: ::windows::core::RawPtr, cachemode: BluetoothCacheMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRfcommServicesForIdWithCacheModeAsync(&*(&serviceid as *const <Rfcomm::RfcommServiceId as ::windows::core::Abi>::Abi as *const <Rfcomm::RfcommServiceId as ::windows::core::DefaultType>::DefaultType), cachemode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBluetoothDevice3, BASE_OFFSET>(),
            DeviceAccessInformation: DeviceAccessInformation::<Impl, IMPL_OFFSET>,
            RequestAccessAsync: RequestAccessAsync::<Impl, IMPL_OFFSET>,
            GetRfcommServicesAsync: GetRfcommServicesAsync::<Impl, IMPL_OFFSET>,
            GetRfcommServicesWithCacheModeAsync: GetRfcommServicesWithCacheModeAsync::<Impl, IMPL_OFFSET>,
            GetRfcommServicesForIdAsync: GetRfcommServicesForIdAsync::<Impl, IMPL_OFFSET>,
            GetRfcommServicesForIdWithCacheModeAsync: GetRfcommServicesForIdWithCacheModeAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBluetoothDevice3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothDevice4_Impl: Sized {
    fn BluetoothDeviceId(&mut self) -> ::windows::core::Result<BluetoothDeviceId>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBluetoothDevice4 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.IBluetoothDevice4";
}
#[cfg(feature = "implement_exclusive")]
impl IBluetoothDevice4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBluetoothDevice4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBluetoothDevice4_Vtbl {
        unsafe extern "system" fn BluetoothDeviceId<Impl: IBluetoothDevice4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BluetoothDeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBluetoothDevice4, BASE_OFFSET>(),
            BluetoothDeviceId: BluetoothDeviceId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBluetoothDevice4 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothDevice5_Impl: Sized {
    fn WasSecureConnectionUsedForPairing(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBluetoothDevice5 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.IBluetoothDevice5";
}
#[cfg(feature = "implement_exclusive")]
impl IBluetoothDevice5_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBluetoothDevice5_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBluetoothDevice5_Vtbl {
        unsafe extern "system" fn WasSecureConnectionUsedForPairing<Impl: IBluetoothDevice5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WasSecureConnectionUsedForPairing() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBluetoothDevice5, BASE_OFFSET>(),
            WasSecureConnectionUsedForPairing: WasSecureConnectionUsedForPairing::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBluetoothDevice5 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothDeviceId_Impl: Sized {
    fn Id(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsClassicDevice(&mut self) -> ::windows::core::Result<bool>;
    fn IsLowEnergyDevice(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBluetoothDeviceId {
    const NAME: &'static str = "Windows.Devices.Bluetooth.IBluetoothDeviceId";
}
#[cfg(feature = "implement_exclusive")]
impl IBluetoothDeviceId_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBluetoothDeviceId_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBluetoothDeviceId_Vtbl {
        unsafe extern "system" fn Id<Impl: IBluetoothDeviceId_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsClassicDevice<Impl: IBluetoothDeviceId_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsClassicDevice() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLowEnergyDevice<Impl: IBluetoothDeviceId_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsLowEnergyDevice() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBluetoothDeviceId, BASE_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            IsClassicDevice: IsClassicDevice::<Impl, IMPL_OFFSET>,
            IsLowEnergyDevice: IsLowEnergyDevice::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBluetoothDeviceId as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothDeviceIdStatics_Impl: Sized {
    fn FromId(&mut self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<BluetoothDeviceId>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBluetoothDeviceIdStatics {
    const NAME: &'static str = "Windows.Devices.Bluetooth.IBluetoothDeviceIdStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IBluetoothDeviceIdStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBluetoothDeviceIdStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBluetoothDeviceIdStatics_Vtbl {
        unsafe extern "system" fn FromId<Impl: IBluetoothDeviceIdStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromId(&*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IBluetoothDeviceIdStatics, BASE_OFFSET>(), FromId: FromId::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBluetoothDeviceIdStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Networking", feature = "implement_exclusive"))]
pub trait IBluetoothDeviceStatics_Impl: Sized {
    fn FromIdAsync(&mut self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BluetoothDevice>>;
    fn FromHostNameAsync(&mut self, hostname: &::core::option::Option<super::super::Networking::HostName>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BluetoothDevice>>;
    fn FromBluetoothAddressAsync(&mut self, address: u64) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BluetoothDevice>>;
    fn GetDeviceSelector(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "Networking", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBluetoothDeviceStatics {
    const NAME: &'static str = "Windows.Devices.Bluetooth.IBluetoothDeviceStatics";
}
#[cfg(all(feature = "Foundation", feature = "Networking", feature = "implement_exclusive"))]
impl IBluetoothDeviceStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBluetoothDeviceStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBluetoothDeviceStatics_Vtbl {
        unsafe extern "system" fn FromIdAsync<Impl: IBluetoothDeviceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FromHostNameAsync<Impl: IBluetoothDeviceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hostname: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromHostNameAsync(&*(&hostname as *const <super::super::Networking::HostName as ::windows::core::Abi>::Abi as *const <super::super::Networking::HostName as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromBluetoothAddressAsync<Impl: IBluetoothDeviceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, address: u64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromBluetoothAddressAsync(address) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceSelector<Impl: IBluetoothDeviceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBluetoothDeviceStatics, BASE_OFFSET>(),
            FromIdAsync: FromIdAsync::<Impl, IMPL_OFFSET>,
            FromHostNameAsync: FromHostNameAsync::<Impl, IMPL_OFFSET>,
            FromBluetoothAddressAsync: FromBluetoothAddressAsync::<Impl, IMPL_OFFSET>,
            GetDeviceSelector: GetDeviceSelector::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBluetoothDeviceStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothDeviceStatics2_Impl: Sized {
    fn GetDeviceSelectorFromPairingState(&mut self, pairingstate: bool) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorFromConnectionStatus(&mut self, connectionstatus: BluetoothConnectionStatus) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorFromDeviceName(&mut self, devicename: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorFromBluetoothAddress(&mut self, bluetoothaddress: u64) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorFromClassOfDevice(&mut self, classofdevice: &::core::option::Option<BluetoothClassOfDevice>) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBluetoothDeviceStatics2 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.IBluetoothDeviceStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IBluetoothDeviceStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBluetoothDeviceStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBluetoothDeviceStatics2_Vtbl {
        unsafe extern "system" fn GetDeviceSelectorFromPairingState<Impl: IBluetoothDeviceStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pairingstate: bool, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelectorFromPairingState(pairingstate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceSelectorFromConnectionStatus<Impl: IBluetoothDeviceStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectionstatus: BluetoothConnectionStatus, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelectorFromConnectionStatus(connectionstatus) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceSelectorFromDeviceName<Impl: IBluetoothDeviceStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, devicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelectorFromDeviceName(&*(&devicename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceSelectorFromBluetoothAddress<Impl: IBluetoothDeviceStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bluetoothaddress: u64, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelectorFromBluetoothAddress(bluetoothaddress) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceSelectorFromClassOfDevice<Impl: IBluetoothDeviceStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, classofdevice: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelectorFromClassOfDevice(&*(&classofdevice as *const <BluetoothClassOfDevice as ::windows::core::Abi>::Abi as *const <BluetoothClassOfDevice as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBluetoothDeviceStatics2, BASE_OFFSET>(),
            GetDeviceSelectorFromPairingState: GetDeviceSelectorFromPairingState::<Impl, IMPL_OFFSET>,
            GetDeviceSelectorFromConnectionStatus: GetDeviceSelectorFromConnectionStatus::<Impl, IMPL_OFFSET>,
            GetDeviceSelectorFromDeviceName: GetDeviceSelectorFromDeviceName::<Impl, IMPL_OFFSET>,
            GetDeviceSelectorFromBluetoothAddress: GetDeviceSelectorFromBluetoothAddress::<Impl, IMPL_OFFSET>,
            GetDeviceSelectorFromClassOfDevice: GetDeviceSelectorFromClassOfDevice::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBluetoothDeviceStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEAppearance_Impl: Sized {
    fn RawValue(&mut self) -> ::windows::core::Result<u16>;
    fn Category(&mut self) -> ::windows::core::Result<u16>;
    fn SubCategory(&mut self) -> ::windows::core::Result<u16>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBluetoothLEAppearance {
    const NAME: &'static str = "Windows.Devices.Bluetooth.IBluetoothLEAppearance";
}
#[cfg(feature = "implement_exclusive")]
impl IBluetoothLEAppearance_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBluetoothLEAppearance_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBluetoothLEAppearance_Vtbl {
        unsafe extern "system" fn RawValue<Impl: IBluetoothLEAppearance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RawValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Category<Impl: IBluetoothLEAppearance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Category() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubCategory<Impl: IBluetoothLEAppearance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SubCategory() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBluetoothLEAppearance, BASE_OFFSET>(),
            RawValue: RawValue::<Impl, IMPL_OFFSET>,
            Category: Category::<Impl, IMPL_OFFSET>,
            SubCategory: SubCategory::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBluetoothLEAppearance as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEAppearanceCategoriesStatics_Impl: Sized {
    fn Uncategorized(&mut self) -> ::windows::core::Result<u16>;
    fn Phone(&mut self) -> ::windows::core::Result<u16>;
    fn Computer(&mut self) -> ::windows::core::Result<u16>;
    fn Watch(&mut self) -> ::windows::core::Result<u16>;
    fn Clock(&mut self) -> ::windows::core::Result<u16>;
    fn Display(&mut self) -> ::windows::core::Result<u16>;
    fn RemoteControl(&mut self) -> ::windows::core::Result<u16>;
    fn EyeGlasses(&mut self) -> ::windows::core::Result<u16>;
    fn Tag(&mut self) -> ::windows::core::Result<u16>;
    fn Keyring(&mut self) -> ::windows::core::Result<u16>;
    fn MediaPlayer(&mut self) -> ::windows::core::Result<u16>;
    fn BarcodeScanner(&mut self) -> ::windows::core::Result<u16>;
    fn Thermometer(&mut self) -> ::windows::core::Result<u16>;
    fn HeartRate(&mut self) -> ::windows::core::Result<u16>;
    fn BloodPressure(&mut self) -> ::windows::core::Result<u16>;
    fn HumanInterfaceDevice(&mut self) -> ::windows::core::Result<u16>;
    fn GlucoseMeter(&mut self) -> ::windows::core::Result<u16>;
    fn RunningWalking(&mut self) -> ::windows::core::Result<u16>;
    fn Cycling(&mut self) -> ::windows::core::Result<u16>;
    fn PulseOximeter(&mut self) -> ::windows::core::Result<u16>;
    fn WeightScale(&mut self) -> ::windows::core::Result<u16>;
    fn OutdoorSportActivity(&mut self) -> ::windows::core::Result<u16>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBluetoothLEAppearanceCategoriesStatics {
    const NAME: &'static str = "Windows.Devices.Bluetooth.IBluetoothLEAppearanceCategoriesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IBluetoothLEAppearanceCategoriesStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBluetoothLEAppearanceCategoriesStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBluetoothLEAppearanceCategoriesStatics_Vtbl {
        unsafe extern "system" fn Uncategorized<Impl: IBluetoothLEAppearanceCategoriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Uncategorized() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Phone<Impl: IBluetoothLEAppearanceCategoriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Phone() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Computer<Impl: IBluetoothLEAppearanceCategoriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Computer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Watch<Impl: IBluetoothLEAppearanceCategoriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Watch() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clock<Impl: IBluetoothLEAppearanceCategoriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clock() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Display<Impl: IBluetoothLEAppearanceCategoriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Display() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoteControl<Impl: IBluetoothLEAppearanceCategoriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoteControl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EyeGlasses<Impl: IBluetoothLEAppearanceCategoriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EyeGlasses() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Tag<Impl: IBluetoothLEAppearanceCategoriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Tag() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Keyring<Impl: IBluetoothLEAppearanceCategoriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Keyring() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaPlayer<Impl: IBluetoothLEAppearanceCategoriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaPlayer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BarcodeScanner<Impl: IBluetoothLEAppearanceCategoriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BarcodeScanner() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Thermometer<Impl: IBluetoothLEAppearanceCategoriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Thermometer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HeartRate<Impl: IBluetoothLEAppearanceCategoriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HeartRate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BloodPressure<Impl: IBluetoothLEAppearanceCategoriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BloodPressure() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HumanInterfaceDevice<Impl: IBluetoothLEAppearanceCategoriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HumanInterfaceDevice() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GlucoseMeter<Impl: IBluetoothLEAppearanceCategoriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GlucoseMeter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RunningWalking<Impl: IBluetoothLEAppearanceCategoriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RunningWalking() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cycling<Impl: IBluetoothLEAppearanceCategoriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cycling() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PulseOximeter<Impl: IBluetoothLEAppearanceCategoriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PulseOximeter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WeightScale<Impl: IBluetoothLEAppearanceCategoriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WeightScale() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OutdoorSportActivity<Impl: IBluetoothLEAppearanceCategoriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OutdoorSportActivity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBluetoothLEAppearanceCategoriesStatics, BASE_OFFSET>(),
            Uncategorized: Uncategorized::<Impl, IMPL_OFFSET>,
            Phone: Phone::<Impl, IMPL_OFFSET>,
            Computer: Computer::<Impl, IMPL_OFFSET>,
            Watch: Watch::<Impl, IMPL_OFFSET>,
            Clock: Clock::<Impl, IMPL_OFFSET>,
            Display: Display::<Impl, IMPL_OFFSET>,
            RemoteControl: RemoteControl::<Impl, IMPL_OFFSET>,
            EyeGlasses: EyeGlasses::<Impl, IMPL_OFFSET>,
            Tag: Tag::<Impl, IMPL_OFFSET>,
            Keyring: Keyring::<Impl, IMPL_OFFSET>,
            MediaPlayer: MediaPlayer::<Impl, IMPL_OFFSET>,
            BarcodeScanner: BarcodeScanner::<Impl, IMPL_OFFSET>,
            Thermometer: Thermometer::<Impl, IMPL_OFFSET>,
            HeartRate: HeartRate::<Impl, IMPL_OFFSET>,
            BloodPressure: BloodPressure::<Impl, IMPL_OFFSET>,
            HumanInterfaceDevice: HumanInterfaceDevice::<Impl, IMPL_OFFSET>,
            GlucoseMeter: GlucoseMeter::<Impl, IMPL_OFFSET>,
            RunningWalking: RunningWalking::<Impl, IMPL_OFFSET>,
            Cycling: Cycling::<Impl, IMPL_OFFSET>,
            PulseOximeter: PulseOximeter::<Impl, IMPL_OFFSET>,
            WeightScale: WeightScale::<Impl, IMPL_OFFSET>,
            OutdoorSportActivity: OutdoorSportActivity::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBluetoothLEAppearanceCategoriesStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEAppearanceStatics_Impl: Sized {
    fn FromRawValue(&mut self, rawvalue: u16) -> ::windows::core::Result<BluetoothLEAppearance>;
    fn FromParts(&mut self, appearancecategory: u16, appearancesubcategory: u16) -> ::windows::core::Result<BluetoothLEAppearance>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBluetoothLEAppearanceStatics {
    const NAME: &'static str = "Windows.Devices.Bluetooth.IBluetoothLEAppearanceStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IBluetoothLEAppearanceStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBluetoothLEAppearanceStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBluetoothLEAppearanceStatics_Vtbl {
        unsafe extern "system" fn FromRawValue<Impl: IBluetoothLEAppearanceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rawvalue: u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromRawValue(rawvalue) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromParts<Impl: IBluetoothLEAppearanceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appearancecategory: u16, appearancesubcategory: u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromParts(appearancecategory, appearancesubcategory) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBluetoothLEAppearanceStatics, BASE_OFFSET>(),
            FromRawValue: FromRawValue::<Impl, IMPL_OFFSET>,
            FromParts: FromParts::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBluetoothLEAppearanceStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEAppearanceSubcategoriesStatics_Impl: Sized {
    fn Generic(&mut self) -> ::windows::core::Result<u16>;
    fn SportsWatch(&mut self) -> ::windows::core::Result<u16>;
    fn ThermometerEar(&mut self) -> ::windows::core::Result<u16>;
    fn HeartRateBelt(&mut self) -> ::windows::core::Result<u16>;
    fn BloodPressureArm(&mut self) -> ::windows::core::Result<u16>;
    fn BloodPressureWrist(&mut self) -> ::windows::core::Result<u16>;
    fn Keyboard(&mut self) -> ::windows::core::Result<u16>;
    fn Mouse(&mut self) -> ::windows::core::Result<u16>;
    fn Joystick(&mut self) -> ::windows::core::Result<u16>;
    fn Gamepad(&mut self) -> ::windows::core::Result<u16>;
    fn DigitizerTablet(&mut self) -> ::windows::core::Result<u16>;
    fn CardReader(&mut self) -> ::windows::core::Result<u16>;
    fn DigitalPen(&mut self) -> ::windows::core::Result<u16>;
    fn BarcodeScanner(&mut self) -> ::windows::core::Result<u16>;
    fn RunningWalkingInShoe(&mut self) -> ::windows::core::Result<u16>;
    fn RunningWalkingOnShoe(&mut self) -> ::windows::core::Result<u16>;
    fn RunningWalkingOnHip(&mut self) -> ::windows::core::Result<u16>;
    fn CyclingComputer(&mut self) -> ::windows::core::Result<u16>;
    fn CyclingSpeedSensor(&mut self) -> ::windows::core::Result<u16>;
    fn CyclingCadenceSensor(&mut self) -> ::windows::core::Result<u16>;
    fn CyclingPowerSensor(&mut self) -> ::windows::core::Result<u16>;
    fn CyclingSpeedCadenceSensor(&mut self) -> ::windows::core::Result<u16>;
    fn OximeterFingertip(&mut self) -> ::windows::core::Result<u16>;
    fn OximeterWristWorn(&mut self) -> ::windows::core::Result<u16>;
    fn LocationDisplay(&mut self) -> ::windows::core::Result<u16>;
    fn LocationNavigationDisplay(&mut self) -> ::windows::core::Result<u16>;
    fn LocationPod(&mut self) -> ::windows::core::Result<u16>;
    fn LocationNavigationPod(&mut self) -> ::windows::core::Result<u16>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBluetoothLEAppearanceSubcategoriesStatics {
    const NAME: &'static str = "Windows.Devices.Bluetooth.IBluetoothLEAppearanceSubcategoriesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IBluetoothLEAppearanceSubcategoriesStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBluetoothLEAppearanceSubcategoriesStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBluetoothLEAppearanceSubcategoriesStatics_Vtbl {
        unsafe extern "system" fn Generic<Impl: IBluetoothLEAppearanceSubcategoriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Generic() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SportsWatch<Impl: IBluetoothLEAppearanceSubcategoriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SportsWatch() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ThermometerEar<Impl: IBluetoothLEAppearanceSubcategoriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ThermometerEar() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HeartRateBelt<Impl: IBluetoothLEAppearanceSubcategoriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HeartRateBelt() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BloodPressureArm<Impl: IBluetoothLEAppearanceSubcategoriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BloodPressureArm() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BloodPressureWrist<Impl: IBluetoothLEAppearanceSubcategoriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BloodPressureWrist() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Keyboard<Impl: IBluetoothLEAppearanceSubcategoriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Keyboard() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Mouse<Impl: IBluetoothLEAppearanceSubcategoriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Mouse() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Joystick<Impl: IBluetoothLEAppearanceSubcategoriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Joystick() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Gamepad<Impl: IBluetoothLEAppearanceSubcategoriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Gamepad() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DigitizerTablet<Impl: IBluetoothLEAppearanceSubcategoriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DigitizerTablet() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CardReader<Impl: IBluetoothLEAppearanceSubcategoriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CardReader() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DigitalPen<Impl: IBluetoothLEAppearanceSubcategoriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DigitalPen() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BarcodeScanner<Impl: IBluetoothLEAppearanceSubcategoriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BarcodeScanner() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RunningWalkingInShoe<Impl: IBluetoothLEAppearanceSubcategoriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RunningWalkingInShoe() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RunningWalkingOnShoe<Impl: IBluetoothLEAppearanceSubcategoriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RunningWalkingOnShoe() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RunningWalkingOnHip<Impl: IBluetoothLEAppearanceSubcategoriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RunningWalkingOnHip() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CyclingComputer<Impl: IBluetoothLEAppearanceSubcategoriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CyclingComputer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CyclingSpeedSensor<Impl: IBluetoothLEAppearanceSubcategoriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CyclingSpeedSensor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CyclingCadenceSensor<Impl: IBluetoothLEAppearanceSubcategoriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CyclingCadenceSensor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CyclingPowerSensor<Impl: IBluetoothLEAppearanceSubcategoriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CyclingPowerSensor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CyclingSpeedCadenceSensor<Impl: IBluetoothLEAppearanceSubcategoriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CyclingSpeedCadenceSensor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OximeterFingertip<Impl: IBluetoothLEAppearanceSubcategoriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OximeterFingertip() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OximeterWristWorn<Impl: IBluetoothLEAppearanceSubcategoriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OximeterWristWorn() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocationDisplay<Impl: IBluetoothLEAppearanceSubcategoriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocationDisplay() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocationNavigationDisplay<Impl: IBluetoothLEAppearanceSubcategoriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocationNavigationDisplay() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocationPod<Impl: IBluetoothLEAppearanceSubcategoriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocationPod() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocationNavigationPod<Impl: IBluetoothLEAppearanceSubcategoriesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocationNavigationPod() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBluetoothLEAppearanceSubcategoriesStatics, BASE_OFFSET>(),
            Generic: Generic::<Impl, IMPL_OFFSET>,
            SportsWatch: SportsWatch::<Impl, IMPL_OFFSET>,
            ThermometerEar: ThermometerEar::<Impl, IMPL_OFFSET>,
            HeartRateBelt: HeartRateBelt::<Impl, IMPL_OFFSET>,
            BloodPressureArm: BloodPressureArm::<Impl, IMPL_OFFSET>,
            BloodPressureWrist: BloodPressureWrist::<Impl, IMPL_OFFSET>,
            Keyboard: Keyboard::<Impl, IMPL_OFFSET>,
            Mouse: Mouse::<Impl, IMPL_OFFSET>,
            Joystick: Joystick::<Impl, IMPL_OFFSET>,
            Gamepad: Gamepad::<Impl, IMPL_OFFSET>,
            DigitizerTablet: DigitizerTablet::<Impl, IMPL_OFFSET>,
            CardReader: CardReader::<Impl, IMPL_OFFSET>,
            DigitalPen: DigitalPen::<Impl, IMPL_OFFSET>,
            BarcodeScanner: BarcodeScanner::<Impl, IMPL_OFFSET>,
            RunningWalkingInShoe: RunningWalkingInShoe::<Impl, IMPL_OFFSET>,
            RunningWalkingOnShoe: RunningWalkingOnShoe::<Impl, IMPL_OFFSET>,
            RunningWalkingOnHip: RunningWalkingOnHip::<Impl, IMPL_OFFSET>,
            CyclingComputer: CyclingComputer::<Impl, IMPL_OFFSET>,
            CyclingSpeedSensor: CyclingSpeedSensor::<Impl, IMPL_OFFSET>,
            CyclingCadenceSensor: CyclingCadenceSensor::<Impl, IMPL_OFFSET>,
            CyclingPowerSensor: CyclingPowerSensor::<Impl, IMPL_OFFSET>,
            CyclingSpeedCadenceSensor: CyclingSpeedCadenceSensor::<Impl, IMPL_OFFSET>,
            OximeterFingertip: OximeterFingertip::<Impl, IMPL_OFFSET>,
            OximeterWristWorn: OximeterWristWorn::<Impl, IMPL_OFFSET>,
            LocationDisplay: LocationDisplay::<Impl, IMPL_OFFSET>,
            LocationNavigationDisplay: LocationNavigationDisplay::<Impl, IMPL_OFFSET>,
            LocationPod: LocationPod::<Impl, IMPL_OFFSET>,
            LocationNavigationPod: LocationNavigationPod::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBluetoothLEAppearanceSubcategoriesStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEConnectionParameters_Impl: Sized {
    fn LinkTimeout(&mut self) -> ::windows::core::Result<u16>;
    fn ConnectionLatency(&mut self) -> ::windows::core::Result<u16>;
    fn ConnectionInterval(&mut self) -> ::windows::core::Result<u16>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBluetoothLEConnectionParameters {
    const NAME: &'static str = "Windows.Devices.Bluetooth.IBluetoothLEConnectionParameters";
}
#[cfg(feature = "implement_exclusive")]
impl IBluetoothLEConnectionParameters_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBluetoothLEConnectionParameters_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBluetoothLEConnectionParameters_Vtbl {
        unsafe extern "system" fn LinkTimeout<Impl: IBluetoothLEConnectionParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LinkTimeout() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectionLatency<Impl: IBluetoothLEConnectionParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectionLatency() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectionInterval<Impl: IBluetoothLEConnectionParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectionInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBluetoothLEConnectionParameters, BASE_OFFSET>(),
            LinkTimeout: LinkTimeout::<Impl, IMPL_OFFSET>,
            ConnectionLatency: ConnectionLatency::<Impl, IMPL_OFFSET>,
            ConnectionInterval: ConnectionInterval::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBluetoothLEConnectionParameters as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEConnectionPhy_Impl: Sized {
    fn TransmitInfo(&mut self) -> ::windows::core::Result<BluetoothLEConnectionPhyInfo>;
    fn ReceiveInfo(&mut self) -> ::windows::core::Result<BluetoothLEConnectionPhyInfo>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBluetoothLEConnectionPhy {
    const NAME: &'static str = "Windows.Devices.Bluetooth.IBluetoothLEConnectionPhy";
}
#[cfg(feature = "implement_exclusive")]
impl IBluetoothLEConnectionPhy_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBluetoothLEConnectionPhy_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBluetoothLEConnectionPhy_Vtbl {
        unsafe extern "system" fn TransmitInfo<Impl: IBluetoothLEConnectionPhy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransmitInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveInfo<Impl: IBluetoothLEConnectionPhy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceiveInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBluetoothLEConnectionPhy, BASE_OFFSET>(),
            TransmitInfo: TransmitInfo::<Impl, IMPL_OFFSET>,
            ReceiveInfo: ReceiveInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBluetoothLEConnectionPhy as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEConnectionPhyInfo_Impl: Sized {
    fn IsUncoded1MPhy(&mut self) -> ::windows::core::Result<bool>;
    fn IsUncoded2MPhy(&mut self) -> ::windows::core::Result<bool>;
    fn IsCodedPhy(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBluetoothLEConnectionPhyInfo {
    const NAME: &'static str = "Windows.Devices.Bluetooth.IBluetoothLEConnectionPhyInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IBluetoothLEConnectionPhyInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBluetoothLEConnectionPhyInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBluetoothLEConnectionPhyInfo_Vtbl {
        unsafe extern "system" fn IsUncoded1MPhy<Impl: IBluetoothLEConnectionPhyInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsUncoded1MPhy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsUncoded2MPhy<Impl: IBluetoothLEConnectionPhyInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsUncoded2MPhy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCodedPhy<Impl: IBluetoothLEConnectionPhyInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCodedPhy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBluetoothLEConnectionPhyInfo, BASE_OFFSET>(),
            IsUncoded1MPhy: IsUncoded1MPhy::<Impl, IMPL_OFFSET>,
            IsUncoded2MPhy: IsUncoded2MPhy::<Impl, IMPL_OFFSET>,
            IsCodedPhy: IsCodedPhy::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBluetoothLEConnectionPhyInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IBluetoothLEDevice_Impl: Sized {
    fn DeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GattServices(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<GenericAttributeProfile::GattDeviceService>>;
    fn ConnectionStatus(&mut self) -> ::windows::core::Result<BluetoothConnectionStatus>;
    fn BluetoothAddress(&mut self) -> ::windows::core::Result<u64>;
    fn GetGattService(&mut self, serviceuuid: &::windows::core::GUID) -> ::windows::core::Result<GenericAttributeProfile::GattDeviceService>;
    fn NameChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<BluetoothLEDevice, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveNameChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GattServicesChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<BluetoothLEDevice, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveGattServicesChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ConnectionStatusChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<BluetoothLEDevice, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveConnectionStatusChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBluetoothLEDevice {
    const NAME: &'static str = "Windows.Devices.Bluetooth.IBluetoothLEDevice";
}
#[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IBluetoothLEDevice_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBluetoothLEDevice_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBluetoothLEDevice_Vtbl {
        unsafe extern "system" fn DeviceId<Impl: IBluetoothLEDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Name<Impl: IBluetoothLEDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GattServices<Impl: IBluetoothLEDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GattServices() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectionStatus<Impl: IBluetoothLEDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut BluetoothConnectionStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn BluetoothAddress<Impl: IBluetoothLEDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BluetoothAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGattService<Impl: IBluetoothLEDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, serviceuuid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGattService(&*(&serviceuuid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NameChanged<Impl: IBluetoothLEDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NameChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<BluetoothLEDevice, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<BluetoothLEDevice, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveNameChanged<Impl: IBluetoothLEDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveNameChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GattServicesChanged<Impl: IBluetoothLEDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GattServicesChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<BluetoothLEDevice, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<BluetoothLEDevice, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveGattServicesChanged<Impl: IBluetoothLEDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveGattServicesChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ConnectionStatusChanged<Impl: IBluetoothLEDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectionStatusChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<BluetoothLEDevice, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<BluetoothLEDevice, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveConnectionStatusChanged<Impl: IBluetoothLEDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveConnectionStatusChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBluetoothLEDevice, BASE_OFFSET>(),
            DeviceId: DeviceId::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            GattServices: GattServices::<Impl, IMPL_OFFSET>,
            ConnectionStatus: ConnectionStatus::<Impl, IMPL_OFFSET>,
            BluetoothAddress: BluetoothAddress::<Impl, IMPL_OFFSET>,
            GetGattService: GetGattService::<Impl, IMPL_OFFSET>,
            NameChanged: NameChanged::<Impl, IMPL_OFFSET>,
            RemoveNameChanged: RemoveNameChanged::<Impl, IMPL_OFFSET>,
            GattServicesChanged: GattServicesChanged::<Impl, IMPL_OFFSET>,
            RemoveGattServicesChanged: RemoveGattServicesChanged::<Impl, IMPL_OFFSET>,
            ConnectionStatusChanged: ConnectionStatusChanged::<Impl, IMPL_OFFSET>,
            RemoveConnectionStatusChanged: RemoveConnectionStatusChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBluetoothLEDevice as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Enumeration", feature = "implement_exclusive"))]
pub trait IBluetoothLEDevice2_Impl: Sized {
    fn DeviceInformation(&mut self) -> ::windows::core::Result<super::Enumeration::DeviceInformation>;
    fn Appearance(&mut self) -> ::windows::core::Result<BluetoothLEAppearance>;
    fn BluetoothAddressType(&mut self) -> ::windows::core::Result<BluetoothAddressType>;
}
#[cfg(all(feature = "Devices_Enumeration", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBluetoothLEDevice2 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.IBluetoothLEDevice2";
}
#[cfg(all(feature = "Devices_Enumeration", feature = "implement_exclusive"))]
impl IBluetoothLEDevice2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBluetoothLEDevice2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBluetoothLEDevice2_Vtbl {
        unsafe extern "system" fn DeviceInformation<Impl: IBluetoothLEDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Appearance<Impl: IBluetoothLEDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Appearance() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BluetoothAddressType<Impl: IBluetoothLEDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut BluetoothAddressType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BluetoothAddressType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBluetoothLEDevice2, BASE_OFFSET>(),
            DeviceInformation: DeviceInformation::<Impl, IMPL_OFFSET>,
            Appearance: Appearance::<Impl, IMPL_OFFSET>,
            BluetoothAddressType: BluetoothAddressType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBluetoothLEDevice2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBluetoothLEDevice3_Impl: Sized {
    fn DeviceAccessInformation(&mut self) -> ::windows::core::Result<super::Enumeration::DeviceAccessInformation>;
    fn RequestAccessAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::Enumeration::DeviceAccessStatus>>;
    fn GetGattServicesAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<GenericAttributeProfile::GattDeviceServicesResult>>;
    fn GetGattServicesWithCacheModeAsync(&mut self, cachemode: BluetoothCacheMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<GenericAttributeProfile::GattDeviceServicesResult>>;
    fn GetGattServicesForUuidAsync(&mut self, serviceuuid: &::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<GenericAttributeProfile::GattDeviceServicesResult>>;
    fn GetGattServicesForUuidWithCacheModeAsync(&mut self, serviceuuid: &::windows::core::GUID, cachemode: BluetoothCacheMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<GenericAttributeProfile::GattDeviceServicesResult>>;
}
#[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBluetoothLEDevice3 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.IBluetoothLEDevice3";
}
#[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "implement_exclusive"))]
impl IBluetoothLEDevice3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBluetoothLEDevice3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBluetoothLEDevice3_Vtbl {
        unsafe extern "system" fn DeviceAccessInformation<Impl: IBluetoothLEDevice3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceAccessInformation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestAccessAsync<Impl: IBluetoothLEDevice3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestAccessAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGattServicesAsync<Impl: IBluetoothLEDevice3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGattServicesAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGattServicesWithCacheModeAsync<Impl: IBluetoothLEDevice3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cachemode: BluetoothCacheMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGattServicesWithCacheModeAsync(cachemode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGattServicesForUuidAsync<Impl: IBluetoothLEDevice3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, serviceuuid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGattServicesForUuidAsync(&*(&serviceuuid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGattServicesForUuidWithCacheModeAsync<Impl: IBluetoothLEDevice3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, serviceuuid: ::windows::core::GUID, cachemode: BluetoothCacheMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGattServicesForUuidWithCacheModeAsync(&*(&serviceuuid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), cachemode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBluetoothLEDevice3, BASE_OFFSET>(),
            DeviceAccessInformation: DeviceAccessInformation::<Impl, IMPL_OFFSET>,
            RequestAccessAsync: RequestAccessAsync::<Impl, IMPL_OFFSET>,
            GetGattServicesAsync: GetGattServicesAsync::<Impl, IMPL_OFFSET>,
            GetGattServicesWithCacheModeAsync: GetGattServicesWithCacheModeAsync::<Impl, IMPL_OFFSET>,
            GetGattServicesForUuidAsync: GetGattServicesForUuidAsync::<Impl, IMPL_OFFSET>,
            GetGattServicesForUuidWithCacheModeAsync: GetGattServicesForUuidWithCacheModeAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBluetoothLEDevice3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEDevice4_Impl: Sized {
    fn BluetoothDeviceId(&mut self) -> ::windows::core::Result<BluetoothDeviceId>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBluetoothLEDevice4 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.IBluetoothLEDevice4";
}
#[cfg(feature = "implement_exclusive")]
impl IBluetoothLEDevice4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBluetoothLEDevice4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBluetoothLEDevice4_Vtbl {
        unsafe extern "system" fn BluetoothDeviceId<Impl: IBluetoothLEDevice4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BluetoothDeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBluetoothLEDevice4, BASE_OFFSET>(),
            BluetoothDeviceId: BluetoothDeviceId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBluetoothLEDevice4 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEDevice5_Impl: Sized {
    fn WasSecureConnectionUsedForPairing(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBluetoothLEDevice5 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.IBluetoothLEDevice5";
}
#[cfg(feature = "implement_exclusive")]
impl IBluetoothLEDevice5_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBluetoothLEDevice5_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBluetoothLEDevice5_Vtbl {
        unsafe extern "system" fn WasSecureConnectionUsedForPairing<Impl: IBluetoothLEDevice5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WasSecureConnectionUsedForPairing() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBluetoothLEDevice5, BASE_OFFSET>(),
            WasSecureConnectionUsedForPairing: WasSecureConnectionUsedForPairing::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBluetoothLEDevice5 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBluetoothLEDevice6_Impl: Sized {
    fn GetConnectionParameters(&mut self) -> ::windows::core::Result<BluetoothLEConnectionParameters>;
    fn GetConnectionPhy(&mut self) -> ::windows::core::Result<BluetoothLEConnectionPhy>;
    fn RequestPreferredConnectionParameters(&mut self, preferredconnectionparameters: &::core::option::Option<BluetoothLEPreferredConnectionParameters>) -> ::windows::core::Result<BluetoothLEPreferredConnectionParametersRequest>;
    fn ConnectionParametersChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<BluetoothLEDevice, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveConnectionParametersChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ConnectionPhyChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<BluetoothLEDevice, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveConnectionPhyChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBluetoothLEDevice6 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.IBluetoothLEDevice6";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IBluetoothLEDevice6_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBluetoothLEDevice6_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBluetoothLEDevice6_Vtbl {
        unsafe extern "system" fn GetConnectionParameters<Impl: IBluetoothLEDevice6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConnectionParameters() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnectionPhy<Impl: IBluetoothLEDevice6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConnectionPhy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestPreferredConnectionParameters<Impl: IBluetoothLEDevice6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preferredconnectionparameters: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestPreferredConnectionParameters(&*(&preferredconnectionparameters as *const <BluetoothLEPreferredConnectionParameters as ::windows::core::Abi>::Abi as *const <BluetoothLEPreferredConnectionParameters as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectionParametersChanged<Impl: IBluetoothLEDevice6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectionParametersChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<BluetoothLEDevice, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<BluetoothLEDevice, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveConnectionParametersChanged<Impl: IBluetoothLEDevice6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveConnectionParametersChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ConnectionPhyChanged<Impl: IBluetoothLEDevice6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectionPhyChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<BluetoothLEDevice, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<BluetoothLEDevice, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveConnectionPhyChanged<Impl: IBluetoothLEDevice6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveConnectionPhyChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBluetoothLEDevice6, BASE_OFFSET>(),
            GetConnectionParameters: GetConnectionParameters::<Impl, IMPL_OFFSET>,
            GetConnectionPhy: GetConnectionPhy::<Impl, IMPL_OFFSET>,
            RequestPreferredConnectionParameters: RequestPreferredConnectionParameters::<Impl, IMPL_OFFSET>,
            ConnectionParametersChanged: ConnectionParametersChanged::<Impl, IMPL_OFFSET>,
            RemoveConnectionParametersChanged: RemoveConnectionParametersChanged::<Impl, IMPL_OFFSET>,
            ConnectionPhyChanged: ConnectionPhyChanged::<Impl, IMPL_OFFSET>,
            RemoveConnectionPhyChanged: RemoveConnectionPhyChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBluetoothLEDevice6 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBluetoothLEDeviceStatics_Impl: Sized {
    fn FromIdAsync(&mut self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BluetoothLEDevice>>;
    fn FromBluetoothAddressAsync(&mut self, bluetoothaddress: u64) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BluetoothLEDevice>>;
    fn GetDeviceSelector(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBluetoothLEDeviceStatics {
    const NAME: &'static str = "Windows.Devices.Bluetooth.IBluetoothLEDeviceStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IBluetoothLEDeviceStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBluetoothLEDeviceStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBluetoothLEDeviceStatics_Vtbl {
        unsafe extern "system" fn FromIdAsync<Impl: IBluetoothLEDeviceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FromBluetoothAddressAsync<Impl: IBluetoothLEDeviceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bluetoothaddress: u64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromBluetoothAddressAsync(bluetoothaddress) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceSelector<Impl: IBluetoothLEDeviceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBluetoothLEDeviceStatics, BASE_OFFSET>(),
            FromIdAsync: FromIdAsync::<Impl, IMPL_OFFSET>,
            FromBluetoothAddressAsync: FromBluetoothAddressAsync::<Impl, IMPL_OFFSET>,
            GetDeviceSelector: GetDeviceSelector::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBluetoothLEDeviceStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBluetoothLEDeviceStatics2_Impl: Sized {
    fn GetDeviceSelectorFromPairingState(&mut self, pairingstate: bool) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorFromConnectionStatus(&mut self, connectionstatus: BluetoothConnectionStatus) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorFromDeviceName(&mut self, devicename: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorFromBluetoothAddress(&mut self, bluetoothaddress: u64) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorFromBluetoothAddressWithBluetoothAddressType(&mut self, bluetoothaddress: u64, bluetoothaddresstype: BluetoothAddressType) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorFromAppearance(&mut self, appearance: &::core::option::Option<BluetoothLEAppearance>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromBluetoothAddressWithBluetoothAddressTypeAsync(&mut self, bluetoothaddress: u64, bluetoothaddresstype: BluetoothAddressType) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BluetoothLEDevice>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBluetoothLEDeviceStatics2 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.IBluetoothLEDeviceStatics2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IBluetoothLEDeviceStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBluetoothLEDeviceStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBluetoothLEDeviceStatics2_Vtbl {
        unsafe extern "system" fn GetDeviceSelectorFromPairingState<Impl: IBluetoothLEDeviceStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pairingstate: bool, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelectorFromPairingState(pairingstate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceSelectorFromConnectionStatus<Impl: IBluetoothLEDeviceStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectionstatus: BluetoothConnectionStatus, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelectorFromConnectionStatus(connectionstatus) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceSelectorFromDeviceName<Impl: IBluetoothLEDeviceStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, devicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelectorFromDeviceName(&*(&devicename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceSelectorFromBluetoothAddress<Impl: IBluetoothLEDeviceStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bluetoothaddress: u64, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelectorFromBluetoothAddress(bluetoothaddress) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceSelectorFromBluetoothAddressWithBluetoothAddressType<Impl: IBluetoothLEDeviceStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bluetoothaddress: u64, bluetoothaddresstype: BluetoothAddressType, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelectorFromBluetoothAddressWithBluetoothAddressType(bluetoothaddress, bluetoothaddresstype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceSelectorFromAppearance<Impl: IBluetoothLEDeviceStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appearance: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelectorFromAppearance(&*(&appearance as *const <BluetoothLEAppearance as ::windows::core::Abi>::Abi as *const <BluetoothLEAppearance as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromBluetoothAddressWithBluetoothAddressTypeAsync<Impl: IBluetoothLEDeviceStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bluetoothaddress: u64, bluetoothaddresstype: BluetoothAddressType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromBluetoothAddressWithBluetoothAddressTypeAsync(bluetoothaddress, bluetoothaddresstype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBluetoothLEDeviceStatics2, BASE_OFFSET>(),
            GetDeviceSelectorFromPairingState: GetDeviceSelectorFromPairingState::<Impl, IMPL_OFFSET>,
            GetDeviceSelectorFromConnectionStatus: GetDeviceSelectorFromConnectionStatus::<Impl, IMPL_OFFSET>,
            GetDeviceSelectorFromDeviceName: GetDeviceSelectorFromDeviceName::<Impl, IMPL_OFFSET>,
            GetDeviceSelectorFromBluetoothAddress: GetDeviceSelectorFromBluetoothAddress::<Impl, IMPL_OFFSET>,
            GetDeviceSelectorFromBluetoothAddressWithBluetoothAddressType: GetDeviceSelectorFromBluetoothAddressWithBluetoothAddressType::<Impl, IMPL_OFFSET>,
            GetDeviceSelectorFromAppearance: GetDeviceSelectorFromAppearance::<Impl, IMPL_OFFSET>,
            FromBluetoothAddressWithBluetoothAddressTypeAsync: FromBluetoothAddressWithBluetoothAddressTypeAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBluetoothLEDeviceStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEPreferredConnectionParameters_Impl: Sized {
    fn LinkTimeout(&mut self) -> ::windows::core::Result<u16>;
    fn ConnectionLatency(&mut self) -> ::windows::core::Result<u16>;
    fn MinConnectionInterval(&mut self) -> ::windows::core::Result<u16>;
    fn MaxConnectionInterval(&mut self) -> ::windows::core::Result<u16>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBluetoothLEPreferredConnectionParameters {
    const NAME: &'static str = "Windows.Devices.Bluetooth.IBluetoothLEPreferredConnectionParameters";
}
#[cfg(feature = "implement_exclusive")]
impl IBluetoothLEPreferredConnectionParameters_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBluetoothLEPreferredConnectionParameters_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBluetoothLEPreferredConnectionParameters_Vtbl {
        unsafe extern "system" fn LinkTimeout<Impl: IBluetoothLEPreferredConnectionParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LinkTimeout() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectionLatency<Impl: IBluetoothLEPreferredConnectionParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectionLatency() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinConnectionInterval<Impl: IBluetoothLEPreferredConnectionParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinConnectionInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxConnectionInterval<Impl: IBluetoothLEPreferredConnectionParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxConnectionInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBluetoothLEPreferredConnectionParameters, BASE_OFFSET>(),
            LinkTimeout: LinkTimeout::<Impl, IMPL_OFFSET>,
            ConnectionLatency: ConnectionLatency::<Impl, IMPL_OFFSET>,
            MinConnectionInterval: MinConnectionInterval::<Impl, IMPL_OFFSET>,
            MaxConnectionInterval: MaxConnectionInterval::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBluetoothLEPreferredConnectionParameters as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEPreferredConnectionParametersRequest_Impl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<BluetoothLEPreferredConnectionParametersRequestStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBluetoothLEPreferredConnectionParametersRequest {
    const NAME: &'static str = "Windows.Devices.Bluetooth.IBluetoothLEPreferredConnectionParametersRequest";
}
#[cfg(feature = "implement_exclusive")]
impl IBluetoothLEPreferredConnectionParametersRequest_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBluetoothLEPreferredConnectionParametersRequest_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBluetoothLEPreferredConnectionParametersRequest_Vtbl {
        unsafe extern "system" fn Status<Impl: IBluetoothLEPreferredConnectionParametersRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut BluetoothLEPreferredConnectionParametersRequestStatus) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBluetoothLEPreferredConnectionParametersRequest, BASE_OFFSET>(),
            Status: Status::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBluetoothLEPreferredConnectionParametersRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEPreferredConnectionParametersStatics_Impl: Sized {
    fn Balanced(&mut self) -> ::windows::core::Result<BluetoothLEPreferredConnectionParameters>;
    fn ThroughputOptimized(&mut self) -> ::windows::core::Result<BluetoothLEPreferredConnectionParameters>;
    fn PowerOptimized(&mut self) -> ::windows::core::Result<BluetoothLEPreferredConnectionParameters>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBluetoothLEPreferredConnectionParametersStatics {
    const NAME: &'static str = "Windows.Devices.Bluetooth.IBluetoothLEPreferredConnectionParametersStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IBluetoothLEPreferredConnectionParametersStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBluetoothLEPreferredConnectionParametersStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBluetoothLEPreferredConnectionParametersStatics_Vtbl {
        unsafe extern "system" fn Balanced<Impl: IBluetoothLEPreferredConnectionParametersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Balanced() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ThroughputOptimized<Impl: IBluetoothLEPreferredConnectionParametersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ThroughputOptimized() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PowerOptimized<Impl: IBluetoothLEPreferredConnectionParametersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PowerOptimized() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBluetoothLEPreferredConnectionParametersStatics, BASE_OFFSET>(),
            Balanced: Balanced::<Impl, IMPL_OFFSET>,
            ThroughputOptimized: ThroughputOptimized::<Impl, IMPL_OFFSET>,
            PowerOptimized: PowerOptimized::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBluetoothLEPreferredConnectionParametersStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBluetoothSignalStrengthFilter_Impl: Sized {
    fn InRangeThresholdInDBm(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<i16>>;
    fn SetInRangeThresholdInDBm(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<i16>>) -> ::windows::core::Result<()>;
    fn OutOfRangeThresholdInDBm(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<i16>>;
    fn SetOutOfRangeThresholdInDBm(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<i16>>) -> ::windows::core::Result<()>;
    fn OutOfRangeTimeout(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn SetOutOfRangeTimeout(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
    fn SamplingInterval(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn SetSamplingInterval(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBluetoothSignalStrengthFilter {
    const NAME: &'static str = "Windows.Devices.Bluetooth.IBluetoothSignalStrengthFilter";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IBluetoothSignalStrengthFilter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBluetoothSignalStrengthFilter_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBluetoothSignalStrengthFilter_Vtbl {
        unsafe extern "system" fn InRangeThresholdInDBm<Impl: IBluetoothSignalStrengthFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InRangeThresholdInDBm() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInRangeThresholdInDBm<Impl: IBluetoothSignalStrengthFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInRangeThresholdInDBm(&*(&value as *const <super::super::Foundation::IReference<i16> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<i16> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OutOfRangeThresholdInDBm<Impl: IBluetoothSignalStrengthFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OutOfRangeThresholdInDBm() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutOfRangeThresholdInDBm<Impl: IBluetoothSignalStrengthFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOutOfRangeThresholdInDBm(&*(&value as *const <super::super::Foundation::IReference<i16> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<i16> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OutOfRangeTimeout<Impl: IBluetoothSignalStrengthFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OutOfRangeTimeout() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutOfRangeTimeout<Impl: IBluetoothSignalStrengthFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOutOfRangeTimeout(&*(&value as *const <super::super::Foundation::IReference<super::super::Foundation::TimeSpan> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::super::Foundation::TimeSpan> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SamplingInterval<Impl: IBluetoothSignalStrengthFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SamplingInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSamplingInterval<Impl: IBluetoothSignalStrengthFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSamplingInterval(&*(&value as *const <super::super::Foundation::IReference<super::super::Foundation::TimeSpan> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::super::Foundation::TimeSpan> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBluetoothSignalStrengthFilter, BASE_OFFSET>(),
            InRangeThresholdInDBm: InRangeThresholdInDBm::<Impl, IMPL_OFFSET>,
            SetInRangeThresholdInDBm: SetInRangeThresholdInDBm::<Impl, IMPL_OFFSET>,
            OutOfRangeThresholdInDBm: OutOfRangeThresholdInDBm::<Impl, IMPL_OFFSET>,
            SetOutOfRangeThresholdInDBm: SetOutOfRangeThresholdInDBm::<Impl, IMPL_OFFSET>,
            OutOfRangeTimeout: OutOfRangeTimeout::<Impl, IMPL_OFFSET>,
            SetOutOfRangeTimeout: SetOutOfRangeTimeout::<Impl, IMPL_OFFSET>,
            SamplingInterval: SamplingInterval::<Impl, IMPL_OFFSET>,
            SetSamplingInterval: SetSamplingInterval::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBluetoothSignalStrengthFilter as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBluetoothUuidHelperStatics_Impl: Sized {
    fn FromShortId(&mut self, shortid: u32) -> ::windows::core::Result<::windows::core::GUID>;
    fn TryGetShortId(&mut self, uuid: &::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBluetoothUuidHelperStatics {
    const NAME: &'static str = "Windows.Devices.Bluetooth.IBluetoothUuidHelperStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IBluetoothUuidHelperStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBluetoothUuidHelperStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBluetoothUuidHelperStatics_Vtbl {
        unsafe extern "system" fn FromShortId<Impl: IBluetoothUuidHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shortid: u32, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromShortId(shortid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetShortId<Impl: IBluetoothUuidHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uuid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetShortId(&*(&uuid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBluetoothUuidHelperStatics, BASE_OFFSET>(),
            FromShortId: FromShortId::<Impl, IMPL_OFFSET>,
            TryGetShortId: TryGetShortId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBluetoothUuidHelperStatics as ::windows::core::Interface>::IID
    }
}
