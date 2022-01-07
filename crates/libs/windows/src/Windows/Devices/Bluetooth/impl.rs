#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothAdapterImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn BluetoothAddress(&self) -> ::windows::core::Result<u64>;
    fn IsClassicSupported(&self) -> ::windows::core::Result<bool>;
    fn IsLowEnergySupported(&self) -> ::windows::core::Result<bool>;
    fn IsPeripheralRoleSupported(&self) -> ::windows::core::Result<bool>;
    fn IsCentralRoleSupported(&self) -> ::windows::core::Result<bool>;
    fn IsAdvertisementOffloadSupported(&self) -> ::windows::core::Result<bool>;
    fn GetRadioAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::Radios::Radio>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBluetoothAdapter {
    const NAME: &'static str = "Windows.Devices.Bluetooth.IBluetoothAdapter";
}
#[cfg(feature = "implement_exclusive")]
impl IBluetoothAdapterVtbl {
    pub const fn new<Impl: IBluetoothAdapterImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBluetoothAdapterVtbl {
        unsafe extern "system" fn DeviceId<Impl: IBluetoothAdapterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn BluetoothAddress<Impl: IBluetoothAdapterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BluetoothAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsClassicSupported<Impl: IBluetoothAdapterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsClassicSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLowEnergySupported<Impl: IBluetoothAdapterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsLowEnergySupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPeripheralRoleSupported<Impl: IBluetoothAdapterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsPeripheralRoleSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCentralRoleSupported<Impl: IBluetoothAdapterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsCentralRoleSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAdvertisementOffloadSupported<Impl: IBluetoothAdapterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsAdvertisementOffloadSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRadioAsync<Impl: IBluetoothAdapterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRadioAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBluetoothAdapter>, base.5, DeviceId::<Impl, OFFSET>, BluetoothAddress::<Impl, OFFSET>, IsClassicSupported::<Impl, OFFSET>, IsLowEnergySupported::<Impl, OFFSET>, IsPeripheralRoleSupported::<Impl, OFFSET>, IsCentralRoleSupported::<Impl, OFFSET>, IsAdvertisementOffloadSupported::<Impl, OFFSET>, GetRadioAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothAdapter2Impl: Sized {
    fn AreClassicSecureConnectionsSupported(&self) -> ::windows::core::Result<bool>;
    fn AreLowEnergySecureConnectionsSupported(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBluetoothAdapter2 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.IBluetoothAdapter2";
}
#[cfg(feature = "implement_exclusive")]
impl IBluetoothAdapter2Vtbl {
    pub const fn new<Impl: IBluetoothAdapter2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBluetoothAdapter2Vtbl {
        unsafe extern "system" fn AreClassicSecureConnectionsSupported<Impl: IBluetoothAdapter2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AreClassicSecureConnectionsSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AreLowEnergySecureConnectionsSupported<Impl: IBluetoothAdapter2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AreLowEnergySecureConnectionsSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBluetoothAdapter2>, base.5, AreClassicSecureConnectionsSupported::<Impl, OFFSET>, AreLowEnergySecureConnectionsSupported::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothAdapter3Impl: Sized {
    fn IsExtendedAdvertisingSupported(&self) -> ::windows::core::Result<bool>;
    fn MaxAdvertisementDataLength(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBluetoothAdapter3 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.IBluetoothAdapter3";
}
#[cfg(feature = "implement_exclusive")]
impl IBluetoothAdapter3Vtbl {
    pub const fn new<Impl: IBluetoothAdapter3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBluetoothAdapter3Vtbl {
        unsafe extern "system" fn IsExtendedAdvertisingSupported<Impl: IBluetoothAdapter3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsExtendedAdvertisingSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxAdvertisementDataLength<Impl: IBluetoothAdapter3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxAdvertisementDataLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBluetoothAdapter3>, base.5, IsExtendedAdvertisingSupported::<Impl, OFFSET>, MaxAdvertisementDataLength::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothAdapterStaticsImpl: Sized {
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BluetoothAdapter>>;
    fn GetDefaultAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BluetoothAdapter>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBluetoothAdapterStatics {
    const NAME: &'static str = "Windows.Devices.Bluetooth.IBluetoothAdapterStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IBluetoothAdapterStaticsVtbl {
    pub const fn new<Impl: IBluetoothAdapterStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBluetoothAdapterStaticsVtbl {
        unsafe extern "system" fn GetDeviceSelector<Impl: IBluetoothAdapterStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FromIdAsync<Impl: IBluetoothAdapterStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDefaultAsync<Impl: IBluetoothAdapterStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDefaultAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBluetoothAdapterStatics>, base.5, GetDeviceSelector::<Impl, OFFSET>, FromIdAsync::<Impl, OFFSET>, GetDefaultAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothClassOfDeviceImpl: Sized {
    fn RawValue(&self) -> ::windows::core::Result<u32>;
    fn MajorClass(&self) -> ::windows::core::Result<BluetoothMajorClass>;
    fn MinorClass(&self) -> ::windows::core::Result<BluetoothMinorClass>;
    fn ServiceCapabilities(&self) -> ::windows::core::Result<BluetoothServiceCapabilities>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBluetoothClassOfDevice {
    const NAME: &'static str = "Windows.Devices.Bluetooth.IBluetoothClassOfDevice";
}
#[cfg(feature = "implement_exclusive")]
impl IBluetoothClassOfDeviceVtbl {
    pub const fn new<Impl: IBluetoothClassOfDeviceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBluetoothClassOfDeviceVtbl {
        unsafe extern "system" fn RawValue<Impl: IBluetoothClassOfDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RawValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MajorClass<Impl: IBluetoothClassOfDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut BluetoothMajorClass) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MajorClass() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinorClass<Impl: IBluetoothClassOfDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut BluetoothMinorClass) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MinorClass() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceCapabilities<Impl: IBluetoothClassOfDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut BluetoothServiceCapabilities) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ServiceCapabilities() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBluetoothClassOfDevice>, base.5, RawValue::<Impl, OFFSET>, MajorClass::<Impl, OFFSET>, MinorClass::<Impl, OFFSET>, ServiceCapabilities::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothClassOfDeviceStaticsImpl: Sized {
    fn FromRawValue(&self, rawvalue: u32) -> ::windows::core::Result<BluetoothClassOfDevice>;
    fn FromParts(&self, majorclass: BluetoothMajorClass, minorclass: BluetoothMinorClass, servicecapabilities: BluetoothServiceCapabilities) -> ::windows::core::Result<BluetoothClassOfDevice>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBluetoothClassOfDeviceStatics {
    const NAME: &'static str = "Windows.Devices.Bluetooth.IBluetoothClassOfDeviceStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IBluetoothClassOfDeviceStaticsVtbl {
    pub const fn new<Impl: IBluetoothClassOfDeviceStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBluetoothClassOfDeviceStaticsVtbl {
        unsafe extern "system" fn FromRawValue<Impl: IBluetoothClassOfDeviceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rawvalue: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromRawValue(rawvalue) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromParts<Impl: IBluetoothClassOfDeviceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, majorclass: BluetoothMajorClass, minorclass: BluetoothMinorClass, servicecapabilities: BluetoothServiceCapabilities, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromParts(majorclass, minorclass, servicecapabilities) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBluetoothClassOfDeviceStatics>, base.5, FromRawValue::<Impl, OFFSET>, FromParts::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothDeviceImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HostName(&self) -> ::windows::core::Result<super::super::Networking::HostName>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ClassOfDevice(&self) -> ::windows::core::Result<BluetoothClassOfDevice>;
    fn SdpRecords(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Storage::Streams::IBuffer>>;
    fn RfcommServices(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<Rfcomm::RfcommDeviceService>>;
    fn ConnectionStatus(&self) -> ::windows::core::Result<BluetoothConnectionStatus>;
    fn BluetoothAddress(&self) -> ::windows::core::Result<u64>;
    fn NameChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<BluetoothDevice, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveNameChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SdpRecordsChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<BluetoothDevice, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSdpRecordsChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ConnectionStatusChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<BluetoothDevice, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveConnectionStatusChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBluetoothDevice {
    const NAME: &'static str = "Windows.Devices.Bluetooth.IBluetoothDevice";
}
#[cfg(feature = "implement_exclusive")]
impl IBluetoothDeviceVtbl {
    pub const fn new<Impl: IBluetoothDeviceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBluetoothDeviceVtbl {
        unsafe extern "system" fn DeviceId<Impl: IBluetoothDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HostName<Impl: IBluetoothDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HostName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: IBluetoothDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClassOfDevice<Impl: IBluetoothDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ClassOfDevice() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SdpRecords<Impl: IBluetoothDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SdpRecords() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RfcommServices<Impl: IBluetoothDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RfcommServices() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectionStatus<Impl: IBluetoothDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut BluetoothConnectionStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn BluetoothAddress<Impl: IBluetoothDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BluetoothAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NameChanged<Impl: IBluetoothDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NameChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<BluetoothDevice, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<BluetoothDevice, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveNameChanged<Impl: IBluetoothDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveNameChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SdpRecordsChanged<Impl: IBluetoothDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SdpRecordsChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<BluetoothDevice, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<BluetoothDevice, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSdpRecordsChanged<Impl: IBluetoothDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveSdpRecordsChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ConnectionStatusChanged<Impl: IBluetoothDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ConnectionStatusChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<BluetoothDevice, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<BluetoothDevice, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveConnectionStatusChanged<Impl: IBluetoothDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveConnectionStatusChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IBluetoothDevice>,
            base.5,
            DeviceId::<Impl, OFFSET>,
            HostName::<Impl, OFFSET>,
            Name::<Impl, OFFSET>,
            ClassOfDevice::<Impl, OFFSET>,
            SdpRecords::<Impl, OFFSET>,
            RfcommServices::<Impl, OFFSET>,
            ConnectionStatus::<Impl, OFFSET>,
            BluetoothAddress::<Impl, OFFSET>,
            NameChanged::<Impl, OFFSET>,
            RemoveNameChanged::<Impl, OFFSET>,
            SdpRecordsChanged::<Impl, OFFSET>,
            RemoveSdpRecordsChanged::<Impl, OFFSET>,
            ConnectionStatusChanged::<Impl, OFFSET>,
            RemoveConnectionStatusChanged::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothDevice2Impl: Sized {
    fn DeviceInformation(&self) -> ::windows::core::Result<super::Enumeration::DeviceInformation>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBluetoothDevice2 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.IBluetoothDevice2";
}
#[cfg(feature = "implement_exclusive")]
impl IBluetoothDevice2Vtbl {
    pub const fn new<Impl: IBluetoothDevice2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBluetoothDevice2Vtbl {
        unsafe extern "system" fn DeviceInformation<Impl: IBluetoothDevice2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBluetoothDevice2>, base.5, DeviceInformation::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothDevice3Impl: Sized {
    fn DeviceAccessInformation(&self) -> ::windows::core::Result<super::Enumeration::DeviceAccessInformation>;
    fn RequestAccessAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::Enumeration::DeviceAccessStatus>>;
    fn GetRfcommServicesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Rfcomm::RfcommDeviceServicesResult>>;
    fn GetRfcommServicesWithCacheModeAsync(&self, cachemode: BluetoothCacheMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Rfcomm::RfcommDeviceServicesResult>>;
    fn GetRfcommServicesForIdAsync(&self, serviceid: &::core::option::Option<Rfcomm::RfcommServiceId>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Rfcomm::RfcommDeviceServicesResult>>;
    fn GetRfcommServicesForIdWithCacheModeAsync(&self, serviceid: &::core::option::Option<Rfcomm::RfcommServiceId>, cachemode: BluetoothCacheMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Rfcomm::RfcommDeviceServicesResult>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBluetoothDevice3 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.IBluetoothDevice3";
}
#[cfg(feature = "implement_exclusive")]
impl IBluetoothDevice3Vtbl {
    pub const fn new<Impl: IBluetoothDevice3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBluetoothDevice3Vtbl {
        unsafe extern "system" fn DeviceAccessInformation<Impl: IBluetoothDevice3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeviceAccessInformation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestAccessAsync<Impl: IBluetoothDevice3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestAccessAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRfcommServicesAsync<Impl: IBluetoothDevice3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRfcommServicesAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRfcommServicesWithCacheModeAsync<Impl: IBluetoothDevice3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cachemode: BluetoothCacheMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRfcommServicesWithCacheModeAsync(cachemode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRfcommServicesForIdAsync<Impl: IBluetoothDevice3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, serviceid: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRfcommServicesForIdAsync(&*(&serviceid as *const <Rfcomm::RfcommServiceId as ::windows::core::Abi>::Abi as *const <Rfcomm::RfcommServiceId as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRfcommServicesForIdWithCacheModeAsync<Impl: IBluetoothDevice3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, serviceid: ::windows::core::RawPtr, cachemode: BluetoothCacheMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRfcommServicesForIdWithCacheModeAsync(&*(&serviceid as *const <Rfcomm::RfcommServiceId as ::windows::core::Abi>::Abi as *const <Rfcomm::RfcommServiceId as ::windows::core::DefaultType>::DefaultType), cachemode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBluetoothDevice3>, base.5, DeviceAccessInformation::<Impl, OFFSET>, RequestAccessAsync::<Impl, OFFSET>, GetRfcommServicesAsync::<Impl, OFFSET>, GetRfcommServicesWithCacheModeAsync::<Impl, OFFSET>, GetRfcommServicesForIdAsync::<Impl, OFFSET>, GetRfcommServicesForIdWithCacheModeAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothDevice4Impl: Sized {
    fn BluetoothDeviceId(&self) -> ::windows::core::Result<BluetoothDeviceId>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBluetoothDevice4 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.IBluetoothDevice4";
}
#[cfg(feature = "implement_exclusive")]
impl IBluetoothDevice4Vtbl {
    pub const fn new<Impl: IBluetoothDevice4Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBluetoothDevice4Vtbl {
        unsafe extern "system" fn BluetoothDeviceId<Impl: IBluetoothDevice4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BluetoothDeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBluetoothDevice4>, base.5, BluetoothDeviceId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothDevice5Impl: Sized {
    fn WasSecureConnectionUsedForPairing(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBluetoothDevice5 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.IBluetoothDevice5";
}
#[cfg(feature = "implement_exclusive")]
impl IBluetoothDevice5Vtbl {
    pub const fn new<Impl: IBluetoothDevice5Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBluetoothDevice5Vtbl {
        unsafe extern "system" fn WasSecureConnectionUsedForPairing<Impl: IBluetoothDevice5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WasSecureConnectionUsedForPairing() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBluetoothDevice5>, base.5, WasSecureConnectionUsedForPairing::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothDeviceIdImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsClassicDevice(&self) -> ::windows::core::Result<bool>;
    fn IsLowEnergyDevice(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBluetoothDeviceId {
    const NAME: &'static str = "Windows.Devices.Bluetooth.IBluetoothDeviceId";
}
#[cfg(feature = "implement_exclusive")]
impl IBluetoothDeviceIdVtbl {
    pub const fn new<Impl: IBluetoothDeviceIdImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBluetoothDeviceIdVtbl {
        unsafe extern "system" fn Id<Impl: IBluetoothDeviceIdImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsClassicDevice<Impl: IBluetoothDeviceIdImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsClassicDevice() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLowEnergyDevice<Impl: IBluetoothDeviceIdImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsLowEnergyDevice() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBluetoothDeviceId>, base.5, Id::<Impl, OFFSET>, IsClassicDevice::<Impl, OFFSET>, IsLowEnergyDevice::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothDeviceIdStaticsImpl: Sized {
    fn FromId(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<BluetoothDeviceId>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBluetoothDeviceIdStatics {
    const NAME: &'static str = "Windows.Devices.Bluetooth.IBluetoothDeviceIdStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IBluetoothDeviceIdStaticsVtbl {
    pub const fn new<Impl: IBluetoothDeviceIdStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBluetoothDeviceIdStaticsVtbl {
        unsafe extern "system" fn FromId<Impl: IBluetoothDeviceIdStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromId(&*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBluetoothDeviceIdStatics>, base.5, FromId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothDeviceStaticsImpl: Sized {
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BluetoothDevice>>;
    fn FromHostNameAsync(&self, hostname: &::core::option::Option<super::super::Networking::HostName>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BluetoothDevice>>;
    fn FromBluetoothAddressAsync(&self, address: u64) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BluetoothDevice>>;
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBluetoothDeviceStatics {
    const NAME: &'static str = "Windows.Devices.Bluetooth.IBluetoothDeviceStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IBluetoothDeviceStaticsVtbl {
    pub const fn new<Impl: IBluetoothDeviceStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBluetoothDeviceStaticsVtbl {
        unsafe extern "system" fn FromIdAsync<Impl: IBluetoothDeviceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FromHostNameAsync<Impl: IBluetoothDeviceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hostname: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromHostNameAsync(&*(&hostname as *const <super::super::Networking::HostName as ::windows::core::Abi>::Abi as *const <super::super::Networking::HostName as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromBluetoothAddressAsync<Impl: IBluetoothDeviceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, address: u64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromBluetoothAddressAsync(address) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceSelector<Impl: IBluetoothDeviceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBluetoothDeviceStatics>, base.5, FromIdAsync::<Impl, OFFSET>, FromHostNameAsync::<Impl, OFFSET>, FromBluetoothAddressAsync::<Impl, OFFSET>, GetDeviceSelector::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothDeviceStatics2Impl: Sized {
    fn GetDeviceSelectorFromPairingState(&self, pairingstate: bool) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorFromConnectionStatus(&self, connectionstatus: BluetoothConnectionStatus) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorFromDeviceName(&self, devicename: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorFromBluetoothAddress(&self, bluetoothaddress: u64) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorFromClassOfDevice(&self, classofdevice: &::core::option::Option<BluetoothClassOfDevice>) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBluetoothDeviceStatics2 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.IBluetoothDeviceStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IBluetoothDeviceStatics2Vtbl {
    pub const fn new<Impl: IBluetoothDeviceStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBluetoothDeviceStatics2Vtbl {
        unsafe extern "system" fn GetDeviceSelectorFromPairingState<Impl: IBluetoothDeviceStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pairingstate: bool, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelectorFromPairingState(pairingstate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceSelectorFromConnectionStatus<Impl: IBluetoothDeviceStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, connectionstatus: BluetoothConnectionStatus, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelectorFromConnectionStatus(connectionstatus) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceSelectorFromDeviceName<Impl: IBluetoothDeviceStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, devicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelectorFromDeviceName(&*(&devicename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceSelectorFromBluetoothAddress<Impl: IBluetoothDeviceStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bluetoothaddress: u64, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelectorFromBluetoothAddress(bluetoothaddress) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceSelectorFromClassOfDevice<Impl: IBluetoothDeviceStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, classofdevice: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelectorFromClassOfDevice(&*(&classofdevice as *const <BluetoothClassOfDevice as ::windows::core::Abi>::Abi as *const <BluetoothClassOfDevice as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBluetoothDeviceStatics2>, base.5, GetDeviceSelectorFromPairingState::<Impl, OFFSET>, GetDeviceSelectorFromConnectionStatus::<Impl, OFFSET>, GetDeviceSelectorFromDeviceName::<Impl, OFFSET>, GetDeviceSelectorFromBluetoothAddress::<Impl, OFFSET>, GetDeviceSelectorFromClassOfDevice::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEAppearanceImpl: Sized {
    fn RawValue(&self) -> ::windows::core::Result<u16>;
    fn Category(&self) -> ::windows::core::Result<u16>;
    fn SubCategory(&self) -> ::windows::core::Result<u16>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBluetoothLEAppearance {
    const NAME: &'static str = "Windows.Devices.Bluetooth.IBluetoothLEAppearance";
}
#[cfg(feature = "implement_exclusive")]
impl IBluetoothLEAppearanceVtbl {
    pub const fn new<Impl: IBluetoothLEAppearanceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBluetoothLEAppearanceVtbl {
        unsafe extern "system" fn RawValue<Impl: IBluetoothLEAppearanceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RawValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Category<Impl: IBluetoothLEAppearanceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Category() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubCategory<Impl: IBluetoothLEAppearanceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SubCategory() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBluetoothLEAppearance>, base.5, RawValue::<Impl, OFFSET>, Category::<Impl, OFFSET>, SubCategory::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEAppearanceCategoriesStaticsImpl: Sized {
    fn Uncategorized(&self) -> ::windows::core::Result<u16>;
    fn Phone(&self) -> ::windows::core::Result<u16>;
    fn Computer(&self) -> ::windows::core::Result<u16>;
    fn Watch(&self) -> ::windows::core::Result<u16>;
    fn Clock(&self) -> ::windows::core::Result<u16>;
    fn Display(&self) -> ::windows::core::Result<u16>;
    fn RemoteControl(&self) -> ::windows::core::Result<u16>;
    fn EyeGlasses(&self) -> ::windows::core::Result<u16>;
    fn Tag(&self) -> ::windows::core::Result<u16>;
    fn Keyring(&self) -> ::windows::core::Result<u16>;
    fn MediaPlayer(&self) -> ::windows::core::Result<u16>;
    fn BarcodeScanner(&self) -> ::windows::core::Result<u16>;
    fn Thermometer(&self) -> ::windows::core::Result<u16>;
    fn HeartRate(&self) -> ::windows::core::Result<u16>;
    fn BloodPressure(&self) -> ::windows::core::Result<u16>;
    fn HumanInterfaceDevice(&self) -> ::windows::core::Result<u16>;
    fn GlucoseMeter(&self) -> ::windows::core::Result<u16>;
    fn RunningWalking(&self) -> ::windows::core::Result<u16>;
    fn Cycling(&self) -> ::windows::core::Result<u16>;
    fn PulseOximeter(&self) -> ::windows::core::Result<u16>;
    fn WeightScale(&self) -> ::windows::core::Result<u16>;
    fn OutdoorSportActivity(&self) -> ::windows::core::Result<u16>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBluetoothLEAppearanceCategoriesStatics {
    const NAME: &'static str = "Windows.Devices.Bluetooth.IBluetoothLEAppearanceCategoriesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IBluetoothLEAppearanceCategoriesStaticsVtbl {
    pub const fn new<Impl: IBluetoothLEAppearanceCategoriesStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBluetoothLEAppearanceCategoriesStaticsVtbl {
        unsafe extern "system" fn Uncategorized<Impl: IBluetoothLEAppearanceCategoriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Uncategorized() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Phone<Impl: IBluetoothLEAppearanceCategoriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Phone() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Computer<Impl: IBluetoothLEAppearanceCategoriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Computer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Watch<Impl: IBluetoothLEAppearanceCategoriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Watch() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clock<Impl: IBluetoothLEAppearanceCategoriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clock() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Display<Impl: IBluetoothLEAppearanceCategoriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Display() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoteControl<Impl: IBluetoothLEAppearanceCategoriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoteControl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EyeGlasses<Impl: IBluetoothLEAppearanceCategoriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EyeGlasses() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Tag<Impl: IBluetoothLEAppearanceCategoriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Tag() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Keyring<Impl: IBluetoothLEAppearanceCategoriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Keyring() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaPlayer<Impl: IBluetoothLEAppearanceCategoriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MediaPlayer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BarcodeScanner<Impl: IBluetoothLEAppearanceCategoriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BarcodeScanner() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Thermometer<Impl: IBluetoothLEAppearanceCategoriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Thermometer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HeartRate<Impl: IBluetoothLEAppearanceCategoriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HeartRate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BloodPressure<Impl: IBluetoothLEAppearanceCategoriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BloodPressure() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HumanInterfaceDevice<Impl: IBluetoothLEAppearanceCategoriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HumanInterfaceDevice() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GlucoseMeter<Impl: IBluetoothLEAppearanceCategoriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GlucoseMeter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RunningWalking<Impl: IBluetoothLEAppearanceCategoriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RunningWalking() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cycling<Impl: IBluetoothLEAppearanceCategoriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Cycling() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PulseOximeter<Impl: IBluetoothLEAppearanceCategoriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PulseOximeter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WeightScale<Impl: IBluetoothLEAppearanceCategoriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WeightScale() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OutdoorSportActivity<Impl: IBluetoothLEAppearanceCategoriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OutdoorSportActivity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IBluetoothLEAppearanceCategoriesStatics>,
            base.5,
            Uncategorized::<Impl, OFFSET>,
            Phone::<Impl, OFFSET>,
            Computer::<Impl, OFFSET>,
            Watch::<Impl, OFFSET>,
            Clock::<Impl, OFFSET>,
            Display::<Impl, OFFSET>,
            RemoteControl::<Impl, OFFSET>,
            EyeGlasses::<Impl, OFFSET>,
            Tag::<Impl, OFFSET>,
            Keyring::<Impl, OFFSET>,
            MediaPlayer::<Impl, OFFSET>,
            BarcodeScanner::<Impl, OFFSET>,
            Thermometer::<Impl, OFFSET>,
            HeartRate::<Impl, OFFSET>,
            BloodPressure::<Impl, OFFSET>,
            HumanInterfaceDevice::<Impl, OFFSET>,
            GlucoseMeter::<Impl, OFFSET>,
            RunningWalking::<Impl, OFFSET>,
            Cycling::<Impl, OFFSET>,
            PulseOximeter::<Impl, OFFSET>,
            WeightScale::<Impl, OFFSET>,
            OutdoorSportActivity::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEAppearanceStaticsImpl: Sized {
    fn FromRawValue(&self, rawvalue: u16) -> ::windows::core::Result<BluetoothLEAppearance>;
    fn FromParts(&self, appearancecategory: u16, appearancesubcategory: u16) -> ::windows::core::Result<BluetoothLEAppearance>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBluetoothLEAppearanceStatics {
    const NAME: &'static str = "Windows.Devices.Bluetooth.IBluetoothLEAppearanceStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IBluetoothLEAppearanceStaticsVtbl {
    pub const fn new<Impl: IBluetoothLEAppearanceStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBluetoothLEAppearanceStaticsVtbl {
        unsafe extern "system" fn FromRawValue<Impl: IBluetoothLEAppearanceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rawvalue: u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromRawValue(rawvalue) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromParts<Impl: IBluetoothLEAppearanceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, appearancecategory: u16, appearancesubcategory: u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromParts(appearancecategory, appearancesubcategory) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBluetoothLEAppearanceStatics>, base.5, FromRawValue::<Impl, OFFSET>, FromParts::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEAppearanceSubcategoriesStaticsImpl: Sized {
    fn Generic(&self) -> ::windows::core::Result<u16>;
    fn SportsWatch(&self) -> ::windows::core::Result<u16>;
    fn ThermometerEar(&self) -> ::windows::core::Result<u16>;
    fn HeartRateBelt(&self) -> ::windows::core::Result<u16>;
    fn BloodPressureArm(&self) -> ::windows::core::Result<u16>;
    fn BloodPressureWrist(&self) -> ::windows::core::Result<u16>;
    fn Keyboard(&self) -> ::windows::core::Result<u16>;
    fn Mouse(&self) -> ::windows::core::Result<u16>;
    fn Joystick(&self) -> ::windows::core::Result<u16>;
    fn Gamepad(&self) -> ::windows::core::Result<u16>;
    fn DigitizerTablet(&self) -> ::windows::core::Result<u16>;
    fn CardReader(&self) -> ::windows::core::Result<u16>;
    fn DigitalPen(&self) -> ::windows::core::Result<u16>;
    fn BarcodeScanner(&self) -> ::windows::core::Result<u16>;
    fn RunningWalkingInShoe(&self) -> ::windows::core::Result<u16>;
    fn RunningWalkingOnShoe(&self) -> ::windows::core::Result<u16>;
    fn RunningWalkingOnHip(&self) -> ::windows::core::Result<u16>;
    fn CyclingComputer(&self) -> ::windows::core::Result<u16>;
    fn CyclingSpeedSensor(&self) -> ::windows::core::Result<u16>;
    fn CyclingCadenceSensor(&self) -> ::windows::core::Result<u16>;
    fn CyclingPowerSensor(&self) -> ::windows::core::Result<u16>;
    fn CyclingSpeedCadenceSensor(&self) -> ::windows::core::Result<u16>;
    fn OximeterFingertip(&self) -> ::windows::core::Result<u16>;
    fn OximeterWristWorn(&self) -> ::windows::core::Result<u16>;
    fn LocationDisplay(&self) -> ::windows::core::Result<u16>;
    fn LocationNavigationDisplay(&self) -> ::windows::core::Result<u16>;
    fn LocationPod(&self) -> ::windows::core::Result<u16>;
    fn LocationNavigationPod(&self) -> ::windows::core::Result<u16>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBluetoothLEAppearanceSubcategoriesStatics {
    const NAME: &'static str = "Windows.Devices.Bluetooth.IBluetoothLEAppearanceSubcategoriesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IBluetoothLEAppearanceSubcategoriesStaticsVtbl {
    pub const fn new<Impl: IBluetoothLEAppearanceSubcategoriesStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBluetoothLEAppearanceSubcategoriesStaticsVtbl {
        unsafe extern "system" fn Generic<Impl: IBluetoothLEAppearanceSubcategoriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Generic() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SportsWatch<Impl: IBluetoothLEAppearanceSubcategoriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SportsWatch() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ThermometerEar<Impl: IBluetoothLEAppearanceSubcategoriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ThermometerEar() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HeartRateBelt<Impl: IBluetoothLEAppearanceSubcategoriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HeartRateBelt() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BloodPressureArm<Impl: IBluetoothLEAppearanceSubcategoriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BloodPressureArm() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BloodPressureWrist<Impl: IBluetoothLEAppearanceSubcategoriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BloodPressureWrist() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Keyboard<Impl: IBluetoothLEAppearanceSubcategoriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Keyboard() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Mouse<Impl: IBluetoothLEAppearanceSubcategoriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Mouse() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Joystick<Impl: IBluetoothLEAppearanceSubcategoriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Joystick() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Gamepad<Impl: IBluetoothLEAppearanceSubcategoriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Gamepad() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DigitizerTablet<Impl: IBluetoothLEAppearanceSubcategoriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DigitizerTablet() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CardReader<Impl: IBluetoothLEAppearanceSubcategoriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CardReader() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DigitalPen<Impl: IBluetoothLEAppearanceSubcategoriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DigitalPen() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BarcodeScanner<Impl: IBluetoothLEAppearanceSubcategoriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BarcodeScanner() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RunningWalkingInShoe<Impl: IBluetoothLEAppearanceSubcategoriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RunningWalkingInShoe() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RunningWalkingOnShoe<Impl: IBluetoothLEAppearanceSubcategoriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RunningWalkingOnShoe() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RunningWalkingOnHip<Impl: IBluetoothLEAppearanceSubcategoriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RunningWalkingOnHip() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CyclingComputer<Impl: IBluetoothLEAppearanceSubcategoriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CyclingComputer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CyclingSpeedSensor<Impl: IBluetoothLEAppearanceSubcategoriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CyclingSpeedSensor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CyclingCadenceSensor<Impl: IBluetoothLEAppearanceSubcategoriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CyclingCadenceSensor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CyclingPowerSensor<Impl: IBluetoothLEAppearanceSubcategoriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CyclingPowerSensor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CyclingSpeedCadenceSensor<Impl: IBluetoothLEAppearanceSubcategoriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CyclingSpeedCadenceSensor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OximeterFingertip<Impl: IBluetoothLEAppearanceSubcategoriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OximeterFingertip() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OximeterWristWorn<Impl: IBluetoothLEAppearanceSubcategoriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OximeterWristWorn() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocationDisplay<Impl: IBluetoothLEAppearanceSubcategoriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LocationDisplay() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocationNavigationDisplay<Impl: IBluetoothLEAppearanceSubcategoriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LocationNavigationDisplay() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocationPod<Impl: IBluetoothLEAppearanceSubcategoriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LocationPod() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocationNavigationPod<Impl: IBluetoothLEAppearanceSubcategoriesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LocationNavigationPod() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IBluetoothLEAppearanceSubcategoriesStatics>,
            base.5,
            Generic::<Impl, OFFSET>,
            SportsWatch::<Impl, OFFSET>,
            ThermometerEar::<Impl, OFFSET>,
            HeartRateBelt::<Impl, OFFSET>,
            BloodPressureArm::<Impl, OFFSET>,
            BloodPressureWrist::<Impl, OFFSET>,
            Keyboard::<Impl, OFFSET>,
            Mouse::<Impl, OFFSET>,
            Joystick::<Impl, OFFSET>,
            Gamepad::<Impl, OFFSET>,
            DigitizerTablet::<Impl, OFFSET>,
            CardReader::<Impl, OFFSET>,
            DigitalPen::<Impl, OFFSET>,
            BarcodeScanner::<Impl, OFFSET>,
            RunningWalkingInShoe::<Impl, OFFSET>,
            RunningWalkingOnShoe::<Impl, OFFSET>,
            RunningWalkingOnHip::<Impl, OFFSET>,
            CyclingComputer::<Impl, OFFSET>,
            CyclingSpeedSensor::<Impl, OFFSET>,
            CyclingCadenceSensor::<Impl, OFFSET>,
            CyclingPowerSensor::<Impl, OFFSET>,
            CyclingSpeedCadenceSensor::<Impl, OFFSET>,
            OximeterFingertip::<Impl, OFFSET>,
            OximeterWristWorn::<Impl, OFFSET>,
            LocationDisplay::<Impl, OFFSET>,
            LocationNavigationDisplay::<Impl, OFFSET>,
            LocationPod::<Impl, OFFSET>,
            LocationNavigationPod::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEConnectionParametersImpl: Sized {
    fn LinkTimeout(&self) -> ::windows::core::Result<u16>;
    fn ConnectionLatency(&self) -> ::windows::core::Result<u16>;
    fn ConnectionInterval(&self) -> ::windows::core::Result<u16>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBluetoothLEConnectionParameters {
    const NAME: &'static str = "Windows.Devices.Bluetooth.IBluetoothLEConnectionParameters";
}
#[cfg(feature = "implement_exclusive")]
impl IBluetoothLEConnectionParametersVtbl {
    pub const fn new<Impl: IBluetoothLEConnectionParametersImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBluetoothLEConnectionParametersVtbl {
        unsafe extern "system" fn LinkTimeout<Impl: IBluetoothLEConnectionParametersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LinkTimeout() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectionLatency<Impl: IBluetoothLEConnectionParametersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ConnectionLatency() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectionInterval<Impl: IBluetoothLEConnectionParametersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ConnectionInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBluetoothLEConnectionParameters>, base.5, LinkTimeout::<Impl, OFFSET>, ConnectionLatency::<Impl, OFFSET>, ConnectionInterval::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEConnectionPhyImpl: Sized {
    fn TransmitInfo(&self) -> ::windows::core::Result<BluetoothLEConnectionPhyInfo>;
    fn ReceiveInfo(&self) -> ::windows::core::Result<BluetoothLEConnectionPhyInfo>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBluetoothLEConnectionPhy {
    const NAME: &'static str = "Windows.Devices.Bluetooth.IBluetoothLEConnectionPhy";
}
#[cfg(feature = "implement_exclusive")]
impl IBluetoothLEConnectionPhyVtbl {
    pub const fn new<Impl: IBluetoothLEConnectionPhyImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBluetoothLEConnectionPhyVtbl {
        unsafe extern "system" fn TransmitInfo<Impl: IBluetoothLEConnectionPhyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TransmitInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveInfo<Impl: IBluetoothLEConnectionPhyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReceiveInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBluetoothLEConnectionPhy>, base.5, TransmitInfo::<Impl, OFFSET>, ReceiveInfo::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEConnectionPhyInfoImpl: Sized {
    fn IsUncoded1MPhy(&self) -> ::windows::core::Result<bool>;
    fn IsUncoded2MPhy(&self) -> ::windows::core::Result<bool>;
    fn IsCodedPhy(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBluetoothLEConnectionPhyInfo {
    const NAME: &'static str = "Windows.Devices.Bluetooth.IBluetoothLEConnectionPhyInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IBluetoothLEConnectionPhyInfoVtbl {
    pub const fn new<Impl: IBluetoothLEConnectionPhyInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBluetoothLEConnectionPhyInfoVtbl {
        unsafe extern "system" fn IsUncoded1MPhy<Impl: IBluetoothLEConnectionPhyInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsUncoded1MPhy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsUncoded2MPhy<Impl: IBluetoothLEConnectionPhyInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsUncoded2MPhy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCodedPhy<Impl: IBluetoothLEConnectionPhyInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsCodedPhy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBluetoothLEConnectionPhyInfo>, base.5, IsUncoded1MPhy::<Impl, OFFSET>, IsUncoded2MPhy::<Impl, OFFSET>, IsCodedPhy::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEDeviceImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GattServices(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<GenericAttributeProfile::GattDeviceService>>;
    fn ConnectionStatus(&self) -> ::windows::core::Result<BluetoothConnectionStatus>;
    fn BluetoothAddress(&self) -> ::windows::core::Result<u64>;
    fn GetGattService(&self, serviceuuid: &::windows::core::GUID) -> ::windows::core::Result<GenericAttributeProfile::GattDeviceService>;
    fn NameChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<BluetoothLEDevice, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveNameChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GattServicesChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<BluetoothLEDevice, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveGattServicesChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ConnectionStatusChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<BluetoothLEDevice, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveConnectionStatusChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBluetoothLEDevice {
    const NAME: &'static str = "Windows.Devices.Bluetooth.IBluetoothLEDevice";
}
#[cfg(feature = "implement_exclusive")]
impl IBluetoothLEDeviceVtbl {
    pub const fn new<Impl: IBluetoothLEDeviceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBluetoothLEDeviceVtbl {
        unsafe extern "system" fn DeviceId<Impl: IBluetoothLEDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Name<Impl: IBluetoothLEDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GattServices<Impl: IBluetoothLEDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GattServices() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectionStatus<Impl: IBluetoothLEDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut BluetoothConnectionStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn BluetoothAddress<Impl: IBluetoothLEDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BluetoothAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGattService<Impl: IBluetoothLEDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, serviceuuid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetGattService(&*(&serviceuuid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NameChanged<Impl: IBluetoothLEDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NameChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<BluetoothLEDevice, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<BluetoothLEDevice, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveNameChanged<Impl: IBluetoothLEDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveNameChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GattServicesChanged<Impl: IBluetoothLEDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GattServicesChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<BluetoothLEDevice, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<BluetoothLEDevice, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveGattServicesChanged<Impl: IBluetoothLEDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveGattServicesChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ConnectionStatusChanged<Impl: IBluetoothLEDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ConnectionStatusChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<BluetoothLEDevice, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<BluetoothLEDevice, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveConnectionStatusChanged<Impl: IBluetoothLEDeviceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveConnectionStatusChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IBluetoothLEDevice>,
            base.5,
            DeviceId::<Impl, OFFSET>,
            Name::<Impl, OFFSET>,
            GattServices::<Impl, OFFSET>,
            ConnectionStatus::<Impl, OFFSET>,
            BluetoothAddress::<Impl, OFFSET>,
            GetGattService::<Impl, OFFSET>,
            NameChanged::<Impl, OFFSET>,
            RemoveNameChanged::<Impl, OFFSET>,
            GattServicesChanged::<Impl, OFFSET>,
            RemoveGattServicesChanged::<Impl, OFFSET>,
            ConnectionStatusChanged::<Impl, OFFSET>,
            RemoveConnectionStatusChanged::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEDevice2Impl: Sized {
    fn DeviceInformation(&self) -> ::windows::core::Result<super::Enumeration::DeviceInformation>;
    fn Appearance(&self) -> ::windows::core::Result<BluetoothLEAppearance>;
    fn BluetoothAddressType(&self) -> ::windows::core::Result<BluetoothAddressType>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBluetoothLEDevice2 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.IBluetoothLEDevice2";
}
#[cfg(feature = "implement_exclusive")]
impl IBluetoothLEDevice2Vtbl {
    pub const fn new<Impl: IBluetoothLEDevice2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBluetoothLEDevice2Vtbl {
        unsafe extern "system" fn DeviceInformation<Impl: IBluetoothLEDevice2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Appearance<Impl: IBluetoothLEDevice2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Appearance() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BluetoothAddressType<Impl: IBluetoothLEDevice2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut BluetoothAddressType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BluetoothAddressType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBluetoothLEDevice2>, base.5, DeviceInformation::<Impl, OFFSET>, Appearance::<Impl, OFFSET>, BluetoothAddressType::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEDevice3Impl: Sized {
    fn DeviceAccessInformation(&self) -> ::windows::core::Result<super::Enumeration::DeviceAccessInformation>;
    fn RequestAccessAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::Enumeration::DeviceAccessStatus>>;
    fn GetGattServicesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<GenericAttributeProfile::GattDeviceServicesResult>>;
    fn GetGattServicesWithCacheModeAsync(&self, cachemode: BluetoothCacheMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<GenericAttributeProfile::GattDeviceServicesResult>>;
    fn GetGattServicesForUuidAsync(&self, serviceuuid: &::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<GenericAttributeProfile::GattDeviceServicesResult>>;
    fn GetGattServicesForUuidWithCacheModeAsync(&self, serviceuuid: &::windows::core::GUID, cachemode: BluetoothCacheMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<GenericAttributeProfile::GattDeviceServicesResult>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBluetoothLEDevice3 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.IBluetoothLEDevice3";
}
#[cfg(feature = "implement_exclusive")]
impl IBluetoothLEDevice3Vtbl {
    pub const fn new<Impl: IBluetoothLEDevice3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBluetoothLEDevice3Vtbl {
        unsafe extern "system" fn DeviceAccessInformation<Impl: IBluetoothLEDevice3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeviceAccessInformation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestAccessAsync<Impl: IBluetoothLEDevice3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestAccessAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGattServicesAsync<Impl: IBluetoothLEDevice3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetGattServicesAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGattServicesWithCacheModeAsync<Impl: IBluetoothLEDevice3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cachemode: BluetoothCacheMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetGattServicesWithCacheModeAsync(cachemode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGattServicesForUuidAsync<Impl: IBluetoothLEDevice3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, serviceuuid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetGattServicesForUuidAsync(&*(&serviceuuid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGattServicesForUuidWithCacheModeAsync<Impl: IBluetoothLEDevice3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, serviceuuid: ::windows::core::GUID, cachemode: BluetoothCacheMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetGattServicesForUuidWithCacheModeAsync(&*(&serviceuuid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), cachemode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBluetoothLEDevice3>, base.5, DeviceAccessInformation::<Impl, OFFSET>, RequestAccessAsync::<Impl, OFFSET>, GetGattServicesAsync::<Impl, OFFSET>, GetGattServicesWithCacheModeAsync::<Impl, OFFSET>, GetGattServicesForUuidAsync::<Impl, OFFSET>, GetGattServicesForUuidWithCacheModeAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEDevice4Impl: Sized {
    fn BluetoothDeviceId(&self) -> ::windows::core::Result<BluetoothDeviceId>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBluetoothLEDevice4 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.IBluetoothLEDevice4";
}
#[cfg(feature = "implement_exclusive")]
impl IBluetoothLEDevice4Vtbl {
    pub const fn new<Impl: IBluetoothLEDevice4Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBluetoothLEDevice4Vtbl {
        unsafe extern "system" fn BluetoothDeviceId<Impl: IBluetoothLEDevice4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BluetoothDeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBluetoothLEDevice4>, base.5, BluetoothDeviceId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEDevice5Impl: Sized {
    fn WasSecureConnectionUsedForPairing(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBluetoothLEDevice5 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.IBluetoothLEDevice5";
}
#[cfg(feature = "implement_exclusive")]
impl IBluetoothLEDevice5Vtbl {
    pub const fn new<Impl: IBluetoothLEDevice5Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBluetoothLEDevice5Vtbl {
        unsafe extern "system" fn WasSecureConnectionUsedForPairing<Impl: IBluetoothLEDevice5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WasSecureConnectionUsedForPairing() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBluetoothLEDevice5>, base.5, WasSecureConnectionUsedForPairing::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEDevice6Impl: Sized {
    fn GetConnectionParameters(&self) -> ::windows::core::Result<BluetoothLEConnectionParameters>;
    fn GetConnectionPhy(&self) -> ::windows::core::Result<BluetoothLEConnectionPhy>;
    fn RequestPreferredConnectionParameters(&self, preferredconnectionparameters: &::core::option::Option<BluetoothLEPreferredConnectionParameters>) -> ::windows::core::Result<BluetoothLEPreferredConnectionParametersRequest>;
    fn ConnectionParametersChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<BluetoothLEDevice, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveConnectionParametersChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ConnectionPhyChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<BluetoothLEDevice, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveConnectionPhyChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBluetoothLEDevice6 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.IBluetoothLEDevice6";
}
#[cfg(feature = "implement_exclusive")]
impl IBluetoothLEDevice6Vtbl {
    pub const fn new<Impl: IBluetoothLEDevice6Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBluetoothLEDevice6Vtbl {
        unsafe extern "system" fn GetConnectionParameters<Impl: IBluetoothLEDevice6Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetConnectionParameters() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnectionPhy<Impl: IBluetoothLEDevice6Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetConnectionPhy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestPreferredConnectionParameters<Impl: IBluetoothLEDevice6Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, preferredconnectionparameters: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestPreferredConnectionParameters(&*(&preferredconnectionparameters as *const <BluetoothLEPreferredConnectionParameters as ::windows::core::Abi>::Abi as *const <BluetoothLEPreferredConnectionParameters as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectionParametersChanged<Impl: IBluetoothLEDevice6Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ConnectionParametersChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<BluetoothLEDevice, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<BluetoothLEDevice, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveConnectionParametersChanged<Impl: IBluetoothLEDevice6Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveConnectionParametersChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ConnectionPhyChanged<Impl: IBluetoothLEDevice6Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ConnectionPhyChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<BluetoothLEDevice, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<BluetoothLEDevice, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveConnectionPhyChanged<Impl: IBluetoothLEDevice6Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveConnectionPhyChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBluetoothLEDevice6>, base.5, GetConnectionParameters::<Impl, OFFSET>, GetConnectionPhy::<Impl, OFFSET>, RequestPreferredConnectionParameters::<Impl, OFFSET>, ConnectionParametersChanged::<Impl, OFFSET>, RemoveConnectionParametersChanged::<Impl, OFFSET>, ConnectionPhyChanged::<Impl, OFFSET>, RemoveConnectionPhyChanged::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEDeviceStaticsImpl: Sized {
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BluetoothLEDevice>>;
    fn FromBluetoothAddressAsync(&self, bluetoothaddress: u64) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BluetoothLEDevice>>;
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBluetoothLEDeviceStatics {
    const NAME: &'static str = "Windows.Devices.Bluetooth.IBluetoothLEDeviceStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IBluetoothLEDeviceStaticsVtbl {
    pub const fn new<Impl: IBluetoothLEDeviceStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBluetoothLEDeviceStaticsVtbl {
        unsafe extern "system" fn FromIdAsync<Impl: IBluetoothLEDeviceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FromBluetoothAddressAsync<Impl: IBluetoothLEDeviceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bluetoothaddress: u64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromBluetoothAddressAsync(bluetoothaddress) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceSelector<Impl: IBluetoothLEDeviceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBluetoothLEDeviceStatics>, base.5, FromIdAsync::<Impl, OFFSET>, FromBluetoothAddressAsync::<Impl, OFFSET>, GetDeviceSelector::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEDeviceStatics2Impl: Sized {
    fn GetDeviceSelectorFromPairingState(&self, pairingstate: bool) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorFromConnectionStatus(&self, connectionstatus: BluetoothConnectionStatus) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorFromDeviceName(&self, devicename: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorFromBluetoothAddress(&self, bluetoothaddress: u64) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorFromBluetoothAddressWithBluetoothAddressType(&self, bluetoothaddress: u64, bluetoothaddresstype: BluetoothAddressType) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorFromAppearance(&self, appearance: &::core::option::Option<BluetoothLEAppearance>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromBluetoothAddressWithBluetoothAddressTypeAsync(&self, bluetoothaddress: u64, bluetoothaddresstype: BluetoothAddressType) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BluetoothLEDevice>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBluetoothLEDeviceStatics2 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.IBluetoothLEDeviceStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IBluetoothLEDeviceStatics2Vtbl {
    pub const fn new<Impl: IBluetoothLEDeviceStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBluetoothLEDeviceStatics2Vtbl {
        unsafe extern "system" fn GetDeviceSelectorFromPairingState<Impl: IBluetoothLEDeviceStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pairingstate: bool, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelectorFromPairingState(pairingstate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceSelectorFromConnectionStatus<Impl: IBluetoothLEDeviceStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, connectionstatus: BluetoothConnectionStatus, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelectorFromConnectionStatus(connectionstatus) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceSelectorFromDeviceName<Impl: IBluetoothLEDeviceStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, devicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelectorFromDeviceName(&*(&devicename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceSelectorFromBluetoothAddress<Impl: IBluetoothLEDeviceStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bluetoothaddress: u64, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelectorFromBluetoothAddress(bluetoothaddress) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceSelectorFromBluetoothAddressWithBluetoothAddressType<Impl: IBluetoothLEDeviceStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bluetoothaddress: u64, bluetoothaddresstype: BluetoothAddressType, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelectorFromBluetoothAddressWithBluetoothAddressType(bluetoothaddress, bluetoothaddresstype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceSelectorFromAppearance<Impl: IBluetoothLEDeviceStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, appearance: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelectorFromAppearance(&*(&appearance as *const <BluetoothLEAppearance as ::windows::core::Abi>::Abi as *const <BluetoothLEAppearance as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromBluetoothAddressWithBluetoothAddressTypeAsync<Impl: IBluetoothLEDeviceStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bluetoothaddress: u64, bluetoothaddresstype: BluetoothAddressType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromBluetoothAddressWithBluetoothAddressTypeAsync(bluetoothaddress, bluetoothaddresstype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IBluetoothLEDeviceStatics2>,
            base.5,
            GetDeviceSelectorFromPairingState::<Impl, OFFSET>,
            GetDeviceSelectorFromConnectionStatus::<Impl, OFFSET>,
            GetDeviceSelectorFromDeviceName::<Impl, OFFSET>,
            GetDeviceSelectorFromBluetoothAddress::<Impl, OFFSET>,
            GetDeviceSelectorFromBluetoothAddressWithBluetoothAddressType::<Impl, OFFSET>,
            GetDeviceSelectorFromAppearance::<Impl, OFFSET>,
            FromBluetoothAddressWithBluetoothAddressTypeAsync::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEPreferredConnectionParametersImpl: Sized {
    fn LinkTimeout(&self) -> ::windows::core::Result<u16>;
    fn ConnectionLatency(&self) -> ::windows::core::Result<u16>;
    fn MinConnectionInterval(&self) -> ::windows::core::Result<u16>;
    fn MaxConnectionInterval(&self) -> ::windows::core::Result<u16>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBluetoothLEPreferredConnectionParameters {
    const NAME: &'static str = "Windows.Devices.Bluetooth.IBluetoothLEPreferredConnectionParameters";
}
#[cfg(feature = "implement_exclusive")]
impl IBluetoothLEPreferredConnectionParametersVtbl {
    pub const fn new<Impl: IBluetoothLEPreferredConnectionParametersImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBluetoothLEPreferredConnectionParametersVtbl {
        unsafe extern "system" fn LinkTimeout<Impl: IBluetoothLEPreferredConnectionParametersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LinkTimeout() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectionLatency<Impl: IBluetoothLEPreferredConnectionParametersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ConnectionLatency() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinConnectionInterval<Impl: IBluetoothLEPreferredConnectionParametersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MinConnectionInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxConnectionInterval<Impl: IBluetoothLEPreferredConnectionParametersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxConnectionInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBluetoothLEPreferredConnectionParameters>, base.5, LinkTimeout::<Impl, OFFSET>, ConnectionLatency::<Impl, OFFSET>, MinConnectionInterval::<Impl, OFFSET>, MaxConnectionInterval::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEPreferredConnectionParametersRequestImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<BluetoothLEPreferredConnectionParametersRequestStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBluetoothLEPreferredConnectionParametersRequest {
    const NAME: &'static str = "Windows.Devices.Bluetooth.IBluetoothLEPreferredConnectionParametersRequest";
}
#[cfg(feature = "implement_exclusive")]
impl IBluetoothLEPreferredConnectionParametersRequestVtbl {
    pub const fn new<Impl: IBluetoothLEPreferredConnectionParametersRequestImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBluetoothLEPreferredConnectionParametersRequestVtbl {
        unsafe extern "system" fn Status<Impl: IBluetoothLEPreferredConnectionParametersRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut BluetoothLEPreferredConnectionParametersRequestStatus) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBluetoothLEPreferredConnectionParametersRequest>, base.5, Status::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEPreferredConnectionParametersStaticsImpl: Sized {
    fn Balanced(&self) -> ::windows::core::Result<BluetoothLEPreferredConnectionParameters>;
    fn ThroughputOptimized(&self) -> ::windows::core::Result<BluetoothLEPreferredConnectionParameters>;
    fn PowerOptimized(&self) -> ::windows::core::Result<BluetoothLEPreferredConnectionParameters>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBluetoothLEPreferredConnectionParametersStatics {
    const NAME: &'static str = "Windows.Devices.Bluetooth.IBluetoothLEPreferredConnectionParametersStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IBluetoothLEPreferredConnectionParametersStaticsVtbl {
    pub const fn new<Impl: IBluetoothLEPreferredConnectionParametersStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBluetoothLEPreferredConnectionParametersStaticsVtbl {
        unsafe extern "system" fn Balanced<Impl: IBluetoothLEPreferredConnectionParametersStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Balanced() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ThroughputOptimized<Impl: IBluetoothLEPreferredConnectionParametersStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ThroughputOptimized() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PowerOptimized<Impl: IBluetoothLEPreferredConnectionParametersStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PowerOptimized() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBluetoothLEPreferredConnectionParametersStatics>, base.5, Balanced::<Impl, OFFSET>, ThroughputOptimized::<Impl, OFFSET>, PowerOptimized::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothSignalStrengthFilterImpl: Sized {
    fn InRangeThresholdInDBm(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i16>>;
    fn SetInRangeThresholdInDBm(&self, value: &::core::option::Option<super::super::Foundation::IReference<i16>>) -> ::windows::core::Result<()>;
    fn OutOfRangeThresholdInDBm(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i16>>;
    fn SetOutOfRangeThresholdInDBm(&self, value: &::core::option::Option<super::super::Foundation::IReference<i16>>) -> ::windows::core::Result<()>;
    fn OutOfRangeTimeout(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn SetOutOfRangeTimeout(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
    fn SamplingInterval(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn SetSamplingInterval(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBluetoothSignalStrengthFilter {
    const NAME: &'static str = "Windows.Devices.Bluetooth.IBluetoothSignalStrengthFilter";
}
#[cfg(feature = "implement_exclusive")]
impl IBluetoothSignalStrengthFilterVtbl {
    pub const fn new<Impl: IBluetoothSignalStrengthFilterImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBluetoothSignalStrengthFilterVtbl {
        unsafe extern "system" fn InRangeThresholdInDBm<Impl: IBluetoothSignalStrengthFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InRangeThresholdInDBm() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInRangeThresholdInDBm<Impl: IBluetoothSignalStrengthFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetInRangeThresholdInDBm(&*(&value as *const <super::super::Foundation::IReference<i16> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<i16> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OutOfRangeThresholdInDBm<Impl: IBluetoothSignalStrengthFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OutOfRangeThresholdInDBm() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutOfRangeThresholdInDBm<Impl: IBluetoothSignalStrengthFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetOutOfRangeThresholdInDBm(&*(&value as *const <super::super::Foundation::IReference<i16> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<i16> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OutOfRangeTimeout<Impl: IBluetoothSignalStrengthFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OutOfRangeTimeout() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutOfRangeTimeout<Impl: IBluetoothSignalStrengthFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetOutOfRangeTimeout(&*(&value as *const <super::super::Foundation::IReference<super::super::Foundation::TimeSpan> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::super::Foundation::TimeSpan> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SamplingInterval<Impl: IBluetoothSignalStrengthFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SamplingInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSamplingInterval<Impl: IBluetoothSignalStrengthFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSamplingInterval(&*(&value as *const <super::super::Foundation::IReference<super::super::Foundation::TimeSpan> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::super::Foundation::TimeSpan> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBluetoothSignalStrengthFilter>, base.5, InRangeThresholdInDBm::<Impl, OFFSET>, SetInRangeThresholdInDBm::<Impl, OFFSET>, OutOfRangeThresholdInDBm::<Impl, OFFSET>, SetOutOfRangeThresholdInDBm::<Impl, OFFSET>, OutOfRangeTimeout::<Impl, OFFSET>, SetOutOfRangeTimeout::<Impl, OFFSET>, SamplingInterval::<Impl, OFFSET>, SetSamplingInterval::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothUuidHelperStaticsImpl: Sized {
    fn FromShortId(&self, shortid: u32) -> ::windows::core::Result<::windows::core::GUID>;
    fn TryGetShortId(&self, uuid: &::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBluetoothUuidHelperStatics {
    const NAME: &'static str = "Windows.Devices.Bluetooth.IBluetoothUuidHelperStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IBluetoothUuidHelperStaticsVtbl {
    pub const fn new<Impl: IBluetoothUuidHelperStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBluetoothUuidHelperStaticsVtbl {
        unsafe extern "system" fn FromShortId<Impl: IBluetoothUuidHelperStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, shortid: u32, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromShortId(shortid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetShortId<Impl: IBluetoothUuidHelperStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uuid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryGetShortId(&*(&uuid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBluetoothUuidHelperStatics>, base.5, FromShortId::<Impl, OFFSET>, TryGetShortId::<Impl, OFFSET>)
    }
}
