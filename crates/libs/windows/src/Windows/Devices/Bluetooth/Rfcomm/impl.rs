#[cfg(feature = "implement_exclusive")]
pub trait IRfcommDeviceServiceImpl: Sized {
    fn ConnectionHostName(&self) -> ::windows::core::Result<super::super::super::Networking::HostName>;
    fn ConnectionServiceName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ServiceId(&self) -> ::windows::core::Result<RfcommServiceId>;
    fn ProtectionLevel(&self) -> ::windows::core::Result<super::super::super::Networking::Sockets::SocketProtectionLevel>;
    fn MaxProtectionLevel(&self) -> ::windows::core::Result<super::super::super::Networking::Sockets::SocketProtectionLevel>;
    fn GetSdpRawAttributesAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IMapView<u32, super::super::super::Storage::Streams::IBuffer>>>;
    fn GetSdpRawAttributesWithCacheModeAsync(&self, cachemode: super::BluetoothCacheMode) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IMapView<u32, super::super::super::Storage::Streams::IBuffer>>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRfcommDeviceService {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Rfcomm.IRfcommDeviceService";
}
#[cfg(feature = "implement_exclusive")]
impl IRfcommDeviceServiceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRfcommDeviceServiceImpl, const OFFSET: isize>() -> IRfcommDeviceServiceVtbl {
        unsafe extern "system" fn ConnectionHostName<Impl: IRfcommDeviceServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectionHostName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectionServiceName<Impl: IRfcommDeviceServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectionServiceName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceId<Impl: IRfcommDeviceServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProtectionLevel<Impl: IRfcommDeviceServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Networking::Sockets::SocketProtectionLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProtectionLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxProtectionLevel<Impl: IRfcommDeviceServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Networking::Sockets::SocketProtectionLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxProtectionLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSdpRawAttributesAsync<Impl: IRfcommDeviceServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSdpRawAttributesAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSdpRawAttributesWithCacheModeAsync<Impl: IRfcommDeviceServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cachemode: super::BluetoothCacheMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSdpRawAttributesWithCacheModeAsync(cachemode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IRfcommDeviceService>,
            ::windows::core::GetTrustLevel,
            ConnectionHostName::<Impl, OFFSET>,
            ConnectionServiceName::<Impl, OFFSET>,
            ServiceId::<Impl, OFFSET>,
            ProtectionLevel::<Impl, OFFSET>,
            MaxProtectionLevel::<Impl, OFFSET>,
            GetSdpRawAttributesAsync::<Impl, OFFSET>,
            GetSdpRawAttributesWithCacheModeAsync::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRfcommDeviceService2Impl: Sized + IRfcommDeviceServiceImpl {
    fn Device(&self) -> ::windows::core::Result<super::BluetoothDevice>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRfcommDeviceService2 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Rfcomm.IRfcommDeviceService2";
}
#[cfg(feature = "implement_exclusive")]
impl IRfcommDeviceService2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRfcommDeviceService2Impl, const OFFSET: isize>() -> IRfcommDeviceService2Vtbl {
        unsafe extern "system" fn Device<Impl: IRfcommDeviceService2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Device() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRfcommDeviceService2>, ::windows::core::GetTrustLevel, Device::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRfcommDeviceService3Impl: Sized + IRfcommDeviceServiceImpl + IRfcommDeviceService2Impl {
    fn DeviceAccessInformation(&self) -> ::windows::core::Result<super::super::Enumeration::DeviceAccessInformation>;
    fn RequestAccessAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::Enumeration::DeviceAccessStatus>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRfcommDeviceService3 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Rfcomm.IRfcommDeviceService3";
}
#[cfg(feature = "implement_exclusive")]
impl IRfcommDeviceService3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRfcommDeviceService3Impl, const OFFSET: isize>() -> IRfcommDeviceService3Vtbl {
        unsafe extern "system" fn DeviceAccessInformation<Impl: IRfcommDeviceService3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RequestAccessAsync<Impl: IRfcommDeviceService3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRfcommDeviceService3>, ::windows::core::GetTrustLevel, DeviceAccessInformation::<Impl, OFFSET>, RequestAccessAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRfcommDeviceServiceStaticsImpl: Sized {
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<RfcommDeviceService>>;
    fn GetDeviceSelector(&self, serviceid: &::core::option::Option<RfcommServiceId>) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRfcommDeviceServiceStatics {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Rfcomm.IRfcommDeviceServiceStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IRfcommDeviceServiceStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRfcommDeviceServiceStaticsImpl, const OFFSET: isize>() -> IRfcommDeviceServiceStaticsVtbl {
        unsafe extern "system" fn FromIdAsync<Impl: IRfcommDeviceServiceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDeviceSelector<Impl: IRfcommDeviceServiceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, serviceid: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelector(&*(&serviceid as *const <RfcommServiceId as ::windows::core::Abi>::Abi as *const <RfcommServiceId as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRfcommDeviceServiceStatics>, ::windows::core::GetTrustLevel, FromIdAsync::<Impl, OFFSET>, GetDeviceSelector::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRfcommDeviceServiceStatics2Impl: Sized + IRfcommDeviceServiceStaticsImpl {
    fn GetDeviceSelectorForBluetoothDevice(&self, bluetoothdevice: &::core::option::Option<super::BluetoothDevice>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorForBluetoothDeviceWithCacheMode(&self, bluetoothdevice: &::core::option::Option<super::BluetoothDevice>, cachemode: super::BluetoothCacheMode) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorForBluetoothDeviceAndServiceId(&self, bluetoothdevice: &::core::option::Option<super::BluetoothDevice>, serviceid: &::core::option::Option<RfcommServiceId>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorForBluetoothDeviceAndServiceIdWithCacheMode(&self, bluetoothdevice: &::core::option::Option<super::BluetoothDevice>, serviceid: &::core::option::Option<RfcommServiceId>, cachemode: super::BluetoothCacheMode) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRfcommDeviceServiceStatics2 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Rfcomm.IRfcommDeviceServiceStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IRfcommDeviceServiceStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRfcommDeviceServiceStatics2Impl, const OFFSET: isize>() -> IRfcommDeviceServiceStatics2Vtbl {
        unsafe extern "system" fn GetDeviceSelectorForBluetoothDevice<Impl: IRfcommDeviceServiceStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bluetoothdevice: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelectorForBluetoothDevice(&*(&bluetoothdevice as *const <super::BluetoothDevice as ::windows::core::Abi>::Abi as *const <super::BluetoothDevice as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceSelectorForBluetoothDeviceWithCacheMode<Impl: IRfcommDeviceServiceStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bluetoothdevice: ::windows::core::RawPtr, cachemode: super::BluetoothCacheMode, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelectorForBluetoothDeviceWithCacheMode(&*(&bluetoothdevice as *const <super::BluetoothDevice as ::windows::core::Abi>::Abi as *const <super::BluetoothDevice as ::windows::core::DefaultType>::DefaultType), cachemode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceSelectorForBluetoothDeviceAndServiceId<Impl: IRfcommDeviceServiceStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bluetoothdevice: ::windows::core::RawPtr, serviceid: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelectorForBluetoothDeviceAndServiceId(&*(&bluetoothdevice as *const <super::BluetoothDevice as ::windows::core::Abi>::Abi as *const <super::BluetoothDevice as ::windows::core::DefaultType>::DefaultType), &*(&serviceid as *const <RfcommServiceId as ::windows::core::Abi>::Abi as *const <RfcommServiceId as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceSelectorForBluetoothDeviceAndServiceIdWithCacheMode<Impl: IRfcommDeviceServiceStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bluetoothdevice: ::windows::core::RawPtr, serviceid: ::windows::core::RawPtr, cachemode: super::BluetoothCacheMode, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelectorForBluetoothDeviceAndServiceIdWithCacheMode(&*(&bluetoothdevice as *const <super::BluetoothDevice as ::windows::core::Abi>::Abi as *const <super::BluetoothDevice as ::windows::core::DefaultType>::DefaultType), &*(&serviceid as *const <RfcommServiceId as ::windows::core::Abi>::Abi as *const <RfcommServiceId as ::windows::core::DefaultType>::DefaultType), cachemode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IRfcommDeviceServiceStatics2>,
            ::windows::core::GetTrustLevel,
            GetDeviceSelectorForBluetoothDevice::<Impl, OFFSET>,
            GetDeviceSelectorForBluetoothDeviceWithCacheMode::<Impl, OFFSET>,
            GetDeviceSelectorForBluetoothDeviceAndServiceId::<Impl, OFFSET>,
            GetDeviceSelectorForBluetoothDeviceAndServiceIdWithCacheMode::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRfcommDeviceServicesResultImpl: Sized {
    fn Error(&self) -> ::windows::core::Result<super::BluetoothError>;
    fn Services(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<RfcommDeviceService>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRfcommDeviceServicesResult {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Rfcomm.IRfcommDeviceServicesResult";
}
#[cfg(feature = "implement_exclusive")]
impl IRfcommDeviceServicesResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRfcommDeviceServicesResultImpl, const OFFSET: isize>() -> IRfcommDeviceServicesResultVtbl {
        unsafe extern "system" fn Error<Impl: IRfcommDeviceServicesResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::BluetoothError) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Services<Impl: IRfcommDeviceServicesResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Services() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRfcommDeviceServicesResult>, ::windows::core::GetTrustLevel, Error::<Impl, OFFSET>, Services::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRfcommServiceIdImpl: Sized {
    fn Uuid(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn AsShortId(&self) -> ::windows::core::Result<u32>;
    fn AsString(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRfcommServiceId {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Rfcomm.IRfcommServiceId";
}
#[cfg(feature = "implement_exclusive")]
impl IRfcommServiceIdVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRfcommServiceIdImpl, const OFFSET: isize>() -> IRfcommServiceIdVtbl {
        unsafe extern "system" fn Uuid<Impl: IRfcommServiceIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Uuid() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AsShortId<Impl: IRfcommServiceIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AsShortId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AsString<Impl: IRfcommServiceIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AsString() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRfcommServiceId>, ::windows::core::GetTrustLevel, Uuid::<Impl, OFFSET>, AsShortId::<Impl, OFFSET>, AsString::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRfcommServiceIdStaticsImpl: Sized {
    fn FromUuid(&self, uuid: &::windows::core::GUID) -> ::windows::core::Result<RfcommServiceId>;
    fn FromShortId(&self, shortid: u32) -> ::windows::core::Result<RfcommServiceId>;
    fn SerialPort(&self) -> ::windows::core::Result<RfcommServiceId>;
    fn ObexObjectPush(&self) -> ::windows::core::Result<RfcommServiceId>;
    fn ObexFileTransfer(&self) -> ::windows::core::Result<RfcommServiceId>;
    fn PhoneBookAccessPce(&self) -> ::windows::core::Result<RfcommServiceId>;
    fn PhoneBookAccessPse(&self) -> ::windows::core::Result<RfcommServiceId>;
    fn GenericFileTransfer(&self) -> ::windows::core::Result<RfcommServiceId>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRfcommServiceIdStatics {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Rfcomm.IRfcommServiceIdStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IRfcommServiceIdStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRfcommServiceIdStaticsImpl, const OFFSET: isize>() -> IRfcommServiceIdStaticsVtbl {
        unsafe extern "system" fn FromUuid<Impl: IRfcommServiceIdStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uuid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromUuid(&*(&uuid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromShortId<Impl: IRfcommServiceIdStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shortid: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SerialPort<Impl: IRfcommServiceIdStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SerialPort() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ObexObjectPush<Impl: IRfcommServiceIdStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ObexObjectPush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ObexFileTransfer<Impl: IRfcommServiceIdStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ObexFileTransfer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhoneBookAccessPce<Impl: IRfcommServiceIdStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PhoneBookAccessPce() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhoneBookAccessPse<Impl: IRfcommServiceIdStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PhoneBookAccessPse() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GenericFileTransfer<Impl: IRfcommServiceIdStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GenericFileTransfer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IRfcommServiceIdStatics>,
            ::windows::core::GetTrustLevel,
            FromUuid::<Impl, OFFSET>,
            FromShortId::<Impl, OFFSET>,
            SerialPort::<Impl, OFFSET>,
            ObexObjectPush::<Impl, OFFSET>,
            ObexFileTransfer::<Impl, OFFSET>,
            PhoneBookAccessPce::<Impl, OFFSET>,
            PhoneBookAccessPse::<Impl, OFFSET>,
            GenericFileTransfer::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRfcommServiceProviderImpl: Sized {
    fn ServiceId(&self) -> ::windows::core::Result<RfcommServiceId>;
    fn SdpRawAttributes(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMap<u32, super::super::super::Storage::Streams::IBuffer>>;
    fn StartAdvertising(&self, listener: &::core::option::Option<super::super::super::Networking::Sockets::StreamSocketListener>) -> ::windows::core::Result<()>;
    fn StopAdvertising(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRfcommServiceProvider {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Rfcomm.IRfcommServiceProvider";
}
#[cfg(feature = "implement_exclusive")]
impl IRfcommServiceProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRfcommServiceProviderImpl, const OFFSET: isize>() -> IRfcommServiceProviderVtbl {
        unsafe extern "system" fn ServiceId<Impl: IRfcommServiceProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SdpRawAttributes<Impl: IRfcommServiceProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SdpRawAttributes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartAdvertising<Impl: IRfcommServiceProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listener: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartAdvertising(&*(&listener as *const <super::super::super::Networking::Sockets::StreamSocketListener as ::windows::core::Abi>::Abi as *const <super::super::super::Networking::Sockets::StreamSocketListener as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StopAdvertising<Impl: IRfcommServiceProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StopAdvertising().into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRfcommServiceProvider>, ::windows::core::GetTrustLevel, ServiceId::<Impl, OFFSET>, SdpRawAttributes::<Impl, OFFSET>, StartAdvertising::<Impl, OFFSET>, StopAdvertising::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRfcommServiceProvider2Impl: Sized + IRfcommServiceProviderImpl {
    fn StartAdvertisingWithRadioDiscoverability(&self, listener: &::core::option::Option<super::super::super::Networking::Sockets::StreamSocketListener>, radiodiscoverable: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRfcommServiceProvider2 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Rfcomm.IRfcommServiceProvider2";
}
#[cfg(feature = "implement_exclusive")]
impl IRfcommServiceProvider2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRfcommServiceProvider2Impl, const OFFSET: isize>() -> IRfcommServiceProvider2Vtbl {
        unsafe extern "system" fn StartAdvertisingWithRadioDiscoverability<Impl: IRfcommServiceProvider2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listener: ::windows::core::RawPtr, radiodiscoverable: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartAdvertisingWithRadioDiscoverability(&*(&listener as *const <super::super::super::Networking::Sockets::StreamSocketListener as ::windows::core::Abi>::Abi as *const <super::super::super::Networking::Sockets::StreamSocketListener as ::windows::core::DefaultType>::DefaultType), radiodiscoverable).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRfcommServiceProvider2>, ::windows::core::GetTrustLevel, StartAdvertisingWithRadioDiscoverability::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRfcommServiceProviderStaticsImpl: Sized {
    fn CreateAsync(&self, serviceid: &::core::option::Option<RfcommServiceId>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<RfcommServiceProvider>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRfcommServiceProviderStatics {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Rfcomm.IRfcommServiceProviderStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IRfcommServiceProviderStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRfcommServiceProviderStaticsImpl, const OFFSET: isize>() -> IRfcommServiceProviderStaticsVtbl {
        unsafe extern "system" fn CreateAsync<Impl: IRfcommServiceProviderStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, serviceid: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAsync(&*(&serviceid as *const <RfcommServiceId as ::windows::core::Abi>::Abi as *const <RfcommServiceId as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRfcommServiceProviderStatics>, ::windows::core::GetTrustLevel, CreateAsync::<Impl, OFFSET>)
    }
}
