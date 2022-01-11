#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Networking", feature = "Networking_Sockets", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IRfcommDeviceServiceImpl: Sized {
    fn ConnectionHostName(&self) -> ::windows::core::Result<super::super::super::Networking::HostName>;
    fn ConnectionServiceName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ServiceId(&self) -> ::windows::core::Result<RfcommServiceId>;
    fn ProtectionLevel(&self) -> ::windows::core::Result<super::super::super::Networking::Sockets::SocketProtectionLevel>;
    fn MaxProtectionLevel(&self) -> ::windows::core::Result<super::super::super::Networking::Sockets::SocketProtectionLevel>;
    fn GetSdpRawAttributesAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IMapView<u32, super::super::super::Storage::Streams::IBuffer>>>;
    fn GetSdpRawAttributesWithCacheModeAsync(&self, cachemode: super::BluetoothCacheMode) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IMapView<u32, super::super::super::Storage::Streams::IBuffer>>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Networking", feature = "Networking_Sockets", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRfcommDeviceService {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Rfcomm.IRfcommDeviceService";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Networking", feature = "Networking_Sockets", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IRfcommDeviceServiceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRfcommDeviceServiceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRfcommDeviceServiceVtbl {
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
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IRfcommDeviceService>,
            ::windows::core::GetTrustLevel,
            ConnectionHostName::<Impl, IMPL_OFFSET>,
            ConnectionServiceName::<Impl, IMPL_OFFSET>,
            ServiceId::<Impl, IMPL_OFFSET>,
            ProtectionLevel::<Impl, IMPL_OFFSET>,
            MaxProtectionLevel::<Impl, IMPL_OFFSET>,
            GetSdpRawAttributesAsync::<Impl, IMPL_OFFSET>,
            GetSdpRawAttributesWithCacheModeAsync::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRfcommDeviceService as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Networking", feature = "Networking_Sockets", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IRfcommDeviceService2Impl: Sized + IRfcommDeviceServiceImpl {
    fn Device(&self) -> ::windows::core::Result<super::BluetoothDevice>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Networking", feature = "Networking_Sockets", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRfcommDeviceService2 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Rfcomm.IRfcommDeviceService2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Networking", feature = "Networking_Sockets", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IRfcommDeviceService2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRfcommDeviceService2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRfcommDeviceService2Vtbl {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRfcommDeviceService2>, ::windows::core::GetTrustLevel, Device::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRfcommDeviceService2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "Networking", feature = "Networking_Sockets", feature = "implement_exclusive"))]
pub trait IRfcommDeviceService3Impl: Sized + IRfcommDeviceServiceImpl + IRfcommDeviceService2Impl {
    fn DeviceAccessInformation(&self) -> ::windows::core::Result<super::super::Enumeration::DeviceAccessInformation>;
    fn RequestAccessAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::Enumeration::DeviceAccessStatus>>;
}
#[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "Networking", feature = "Networking_Sockets", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRfcommDeviceService3 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Rfcomm.IRfcommDeviceService3";
}
#[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "Networking", feature = "Networking_Sockets", feature = "implement_exclusive"))]
impl IRfcommDeviceService3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRfcommDeviceService3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRfcommDeviceService3Vtbl {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRfcommDeviceService3>, ::windows::core::GetTrustLevel, DeviceAccessInformation::<Impl, IMPL_OFFSET>, RequestAccessAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRfcommDeviceService3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IRfcommDeviceServiceStaticsImpl: Sized {
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<RfcommDeviceService>>;
    fn GetDeviceSelector(&self, serviceid: &::core::option::Option<RfcommServiceId>) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRfcommDeviceServiceStatics {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Rfcomm.IRfcommDeviceServiceStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IRfcommDeviceServiceStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRfcommDeviceServiceStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRfcommDeviceServiceStaticsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRfcommDeviceServiceStatics>, ::windows::core::GetTrustLevel, FromIdAsync::<Impl, IMPL_OFFSET>, GetDeviceSelector::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRfcommDeviceServiceStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IRfcommDeviceServiceStatics2Impl: Sized + IRfcommDeviceServiceStaticsImpl {
    fn GetDeviceSelectorForBluetoothDevice(&self, bluetoothdevice: &::core::option::Option<super::BluetoothDevice>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorForBluetoothDeviceWithCacheMode(&self, bluetoothdevice: &::core::option::Option<super::BluetoothDevice>, cachemode: super::BluetoothCacheMode) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorForBluetoothDeviceAndServiceId(&self, bluetoothdevice: &::core::option::Option<super::BluetoothDevice>, serviceid: &::core::option::Option<RfcommServiceId>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorForBluetoothDeviceAndServiceIdWithCacheMode(&self, bluetoothdevice: &::core::option::Option<super::BluetoothDevice>, serviceid: &::core::option::Option<RfcommServiceId>, cachemode: super::BluetoothCacheMode) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRfcommDeviceServiceStatics2 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Rfcomm.IRfcommDeviceServiceStatics2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IRfcommDeviceServiceStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRfcommDeviceServiceStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRfcommDeviceServiceStatics2Vtbl {
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
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IRfcommDeviceServiceStatics2>,
            ::windows::core::GetTrustLevel,
            GetDeviceSelectorForBluetoothDevice::<Impl, IMPL_OFFSET>,
            GetDeviceSelectorForBluetoothDeviceWithCacheMode::<Impl, IMPL_OFFSET>,
            GetDeviceSelectorForBluetoothDeviceAndServiceId::<Impl, IMPL_OFFSET>,
            GetDeviceSelectorForBluetoothDeviceAndServiceIdWithCacheMode::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRfcommDeviceServiceStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IRfcommDeviceServicesResultImpl: Sized {
    fn Error(&self) -> ::windows::core::Result<super::BluetoothError>;
    fn Services(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<RfcommDeviceService>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRfcommDeviceServicesResult {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Rfcomm.IRfcommDeviceServicesResult";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IRfcommDeviceServicesResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRfcommDeviceServicesResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRfcommDeviceServicesResultVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRfcommDeviceServicesResult>, ::windows::core::GetTrustLevel, Error::<Impl, IMPL_OFFSET>, Services::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRfcommDeviceServicesResult as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRfcommServiceIdImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRfcommServiceIdVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRfcommServiceId>, ::windows::core::GetTrustLevel, Uuid::<Impl, IMPL_OFFSET>, AsShortId::<Impl, IMPL_OFFSET>, AsString::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRfcommServiceId as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRfcommServiceIdStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRfcommServiceIdStaticsVtbl {
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
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IRfcommServiceIdStatics>,
            ::windows::core::GetTrustLevel,
            FromUuid::<Impl, IMPL_OFFSET>,
            FromShortId::<Impl, IMPL_OFFSET>,
            SerialPort::<Impl, IMPL_OFFSET>,
            ObexObjectPush::<Impl, IMPL_OFFSET>,
            ObexFileTransfer::<Impl, IMPL_OFFSET>,
            PhoneBookAccessPce::<Impl, IMPL_OFFSET>,
            PhoneBookAccessPse::<Impl, IMPL_OFFSET>,
            GenericFileTransfer::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRfcommServiceIdStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Networking_Sockets", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IRfcommServiceProviderImpl: Sized {
    fn ServiceId(&self) -> ::windows::core::Result<RfcommServiceId>;
    fn SdpRawAttributes(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMap<u32, super::super::super::Storage::Streams::IBuffer>>;
    fn StartAdvertising(&self, listener: &::core::option::Option<super::super::super::Networking::Sockets::StreamSocketListener>) -> ::windows::core::Result<()>;
    fn StopAdvertising(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Networking_Sockets", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRfcommServiceProvider {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Rfcomm.IRfcommServiceProvider";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Networking_Sockets", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IRfcommServiceProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRfcommServiceProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRfcommServiceProviderVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRfcommServiceProvider>, ::windows::core::GetTrustLevel, ServiceId::<Impl, IMPL_OFFSET>, SdpRawAttributes::<Impl, IMPL_OFFSET>, StartAdvertising::<Impl, IMPL_OFFSET>, StopAdvertising::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRfcommServiceProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Networking_Sockets", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IRfcommServiceProvider2Impl: Sized + IRfcommServiceProviderImpl {
    fn StartAdvertisingWithRadioDiscoverability(&self, listener: &::core::option::Option<super::super::super::Networking::Sockets::StreamSocketListener>, radiodiscoverable: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Networking_Sockets", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRfcommServiceProvider2 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Rfcomm.IRfcommServiceProvider2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Networking_Sockets", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IRfcommServiceProvider2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRfcommServiceProvider2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRfcommServiceProvider2Vtbl {
        unsafe extern "system" fn StartAdvertisingWithRadioDiscoverability<Impl: IRfcommServiceProvider2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listener: ::windows::core::RawPtr, radiodiscoverable: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartAdvertisingWithRadioDiscoverability(&*(&listener as *const <super::super::super::Networking::Sockets::StreamSocketListener as ::windows::core::Abi>::Abi as *const <super::super::super::Networking::Sockets::StreamSocketListener as ::windows::core::DefaultType>::DefaultType), radiodiscoverable).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRfcommServiceProvider2>, ::windows::core::GetTrustLevel, StartAdvertisingWithRadioDiscoverability::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRfcommServiceProvider2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IRfcommServiceProviderStaticsImpl: Sized {
    fn CreateAsync(&self, serviceid: &::core::option::Option<RfcommServiceId>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<RfcommServiceProvider>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRfcommServiceProviderStatics {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Rfcomm.IRfcommServiceProviderStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IRfcommServiceProviderStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRfcommServiceProviderStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRfcommServiceProviderStaticsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRfcommServiceProviderStatics>, ::windows::core::GetTrustLevel, CreateAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRfcommServiceProviderStatics as ::windows::core::Interface>::IID
    }
}
