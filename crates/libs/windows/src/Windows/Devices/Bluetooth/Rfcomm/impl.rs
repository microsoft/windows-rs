#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Networking", feature = "Networking_Sockets", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IRfcommDeviceService_Impl: Sized {
    fn ConnectionHostName(&mut self) -> ::windows::core::Result<super::super::super::Networking::HostName>;
    fn ConnectionServiceName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ServiceId(&mut self) -> ::windows::core::Result<RfcommServiceId>;
    fn ProtectionLevel(&mut self) -> ::windows::core::Result<super::super::super::Networking::Sockets::SocketProtectionLevel>;
    fn MaxProtectionLevel(&mut self) -> ::windows::core::Result<super::super::super::Networking::Sockets::SocketProtectionLevel>;
    fn GetSdpRawAttributesAsync(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IMapView<u32, super::super::super::Storage::Streams::IBuffer>>>;
    fn GetSdpRawAttributesWithCacheModeAsync(&mut self, cachemode: super::BluetoothCacheMode) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IMapView<u32, super::super::super::Storage::Streams::IBuffer>>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Networking", feature = "Networking_Sockets", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRfcommDeviceService {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Rfcomm.IRfcommDeviceService";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Networking", feature = "Networking_Sockets", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IRfcommDeviceService_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRfcommDeviceService_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRfcommDeviceService_Vtbl {
        unsafe extern "system" fn ConnectionHostName<Impl: IRfcommDeviceService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ConnectionServiceName<Impl: IRfcommDeviceService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ServiceId<Impl: IRfcommDeviceService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ProtectionLevel<Impl: IRfcommDeviceService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Networking::Sockets::SocketProtectionLevel) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MaxProtectionLevel<Impl: IRfcommDeviceService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Networking::Sockets::SocketProtectionLevel) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetSdpRawAttributesAsync<Impl: IRfcommDeviceService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetSdpRawAttributesWithCacheModeAsync<Impl: IRfcommDeviceService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cachemode: super::BluetoothCacheMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRfcommDeviceService, BASE_OFFSET>(),
            ConnectionHostName: ConnectionHostName::<Impl, IMPL_OFFSET>,
            ConnectionServiceName: ConnectionServiceName::<Impl, IMPL_OFFSET>,
            ServiceId: ServiceId::<Impl, IMPL_OFFSET>,
            ProtectionLevel: ProtectionLevel::<Impl, IMPL_OFFSET>,
            MaxProtectionLevel: MaxProtectionLevel::<Impl, IMPL_OFFSET>,
            GetSdpRawAttributesAsync: GetSdpRawAttributesAsync::<Impl, IMPL_OFFSET>,
            GetSdpRawAttributesWithCacheModeAsync: GetSdpRawAttributesWithCacheModeAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRfcommDeviceService as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Networking", feature = "Networking_Sockets", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IRfcommDeviceService2_Impl: Sized + IRfcommDeviceService_Impl {
    fn Device(&mut self) -> ::windows::core::Result<super::BluetoothDevice>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Networking", feature = "Networking_Sockets", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRfcommDeviceService2 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Rfcomm.IRfcommDeviceService2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Networking", feature = "Networking_Sockets", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IRfcommDeviceService2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRfcommDeviceService2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRfcommDeviceService2_Vtbl {
        unsafe extern "system" fn Device<Impl: IRfcommDeviceService2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IRfcommDeviceService2, BASE_OFFSET>(), Device: Device::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRfcommDeviceService2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "Networking", feature = "Networking_Sockets", feature = "implement_exclusive"))]
pub trait IRfcommDeviceService3_Impl: Sized + IRfcommDeviceService_Impl + IRfcommDeviceService2_Impl {
    fn DeviceAccessInformation(&mut self) -> ::windows::core::Result<super::super::Enumeration::DeviceAccessInformation>;
    fn RequestAccessAsync(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::Enumeration::DeviceAccessStatus>>;
}
#[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "Networking", feature = "Networking_Sockets", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRfcommDeviceService3 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Rfcomm.IRfcommDeviceService3";
}
#[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "Networking", feature = "Networking_Sockets", feature = "implement_exclusive"))]
impl IRfcommDeviceService3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRfcommDeviceService3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRfcommDeviceService3_Vtbl {
        unsafe extern "system" fn DeviceAccessInformation<Impl: IRfcommDeviceService3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RequestAccessAsync<Impl: IRfcommDeviceService3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRfcommDeviceService3, BASE_OFFSET>(),
            DeviceAccessInformation: DeviceAccessInformation::<Impl, IMPL_OFFSET>,
            RequestAccessAsync: RequestAccessAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRfcommDeviceService3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IRfcommDeviceServiceStatics_Impl: Sized {
    fn FromIdAsync(&mut self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<RfcommDeviceService>>;
    fn GetDeviceSelector(&mut self, serviceid: &::core::option::Option<RfcommServiceId>) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRfcommDeviceServiceStatics {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Rfcomm.IRfcommDeviceServiceStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IRfcommDeviceServiceStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRfcommDeviceServiceStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRfcommDeviceServiceStatics_Vtbl {
        unsafe extern "system" fn FromIdAsync<Impl: IRfcommDeviceServiceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDeviceSelector<Impl: IRfcommDeviceServiceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, serviceid: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRfcommDeviceServiceStatics, BASE_OFFSET>(),
            FromIdAsync: FromIdAsync::<Impl, IMPL_OFFSET>,
            GetDeviceSelector: GetDeviceSelector::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRfcommDeviceServiceStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IRfcommDeviceServiceStatics2_Impl: Sized + IRfcommDeviceServiceStatics_Impl {
    fn GetDeviceSelectorForBluetoothDevice(&mut self, bluetoothdevice: &::core::option::Option<super::BluetoothDevice>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorForBluetoothDeviceWithCacheMode(&mut self, bluetoothdevice: &::core::option::Option<super::BluetoothDevice>, cachemode: super::BluetoothCacheMode) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorForBluetoothDeviceAndServiceId(&mut self, bluetoothdevice: &::core::option::Option<super::BluetoothDevice>, serviceid: &::core::option::Option<RfcommServiceId>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorForBluetoothDeviceAndServiceIdWithCacheMode(&mut self, bluetoothdevice: &::core::option::Option<super::BluetoothDevice>, serviceid: &::core::option::Option<RfcommServiceId>, cachemode: super::BluetoothCacheMode) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRfcommDeviceServiceStatics2 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Rfcomm.IRfcommDeviceServiceStatics2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IRfcommDeviceServiceStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRfcommDeviceServiceStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRfcommDeviceServiceStatics2_Vtbl {
        unsafe extern "system" fn GetDeviceSelectorForBluetoothDevice<Impl: IRfcommDeviceServiceStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bluetoothdevice: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDeviceSelectorForBluetoothDeviceWithCacheMode<Impl: IRfcommDeviceServiceStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bluetoothdevice: ::windows::core::RawPtr, cachemode: super::BluetoothCacheMode, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDeviceSelectorForBluetoothDeviceAndServiceId<Impl: IRfcommDeviceServiceStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bluetoothdevice: ::windows::core::RawPtr, serviceid: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDeviceSelectorForBluetoothDeviceAndServiceIdWithCacheMode<Impl: IRfcommDeviceServiceStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bluetoothdevice: ::windows::core::RawPtr, serviceid: ::windows::core::RawPtr, cachemode: super::BluetoothCacheMode, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRfcommDeviceServiceStatics2, BASE_OFFSET>(),
            GetDeviceSelectorForBluetoothDevice: GetDeviceSelectorForBluetoothDevice::<Impl, IMPL_OFFSET>,
            GetDeviceSelectorForBluetoothDeviceWithCacheMode: GetDeviceSelectorForBluetoothDeviceWithCacheMode::<Impl, IMPL_OFFSET>,
            GetDeviceSelectorForBluetoothDeviceAndServiceId: GetDeviceSelectorForBluetoothDeviceAndServiceId::<Impl, IMPL_OFFSET>,
            GetDeviceSelectorForBluetoothDeviceAndServiceIdWithCacheMode: GetDeviceSelectorForBluetoothDeviceAndServiceIdWithCacheMode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRfcommDeviceServiceStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IRfcommDeviceServicesResult_Impl: Sized {
    fn Error(&mut self) -> ::windows::core::Result<super::BluetoothError>;
    fn Services(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<RfcommDeviceService>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRfcommDeviceServicesResult {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Rfcomm.IRfcommDeviceServicesResult";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IRfcommDeviceServicesResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRfcommDeviceServicesResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRfcommDeviceServicesResult_Vtbl {
        unsafe extern "system" fn Error<Impl: IRfcommDeviceServicesResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::BluetoothError) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Services<Impl: IRfcommDeviceServicesResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRfcommDeviceServicesResult, BASE_OFFSET>(),
            Error: Error::<Impl, IMPL_OFFSET>,
            Services: Services::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRfcommDeviceServicesResult as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRfcommServiceId_Impl: Sized {
    fn Uuid(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn AsShortId(&mut self) -> ::windows::core::Result<u32>;
    fn AsString(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRfcommServiceId {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Rfcomm.IRfcommServiceId";
}
#[cfg(feature = "implement_exclusive")]
impl IRfcommServiceId_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRfcommServiceId_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRfcommServiceId_Vtbl {
        unsafe extern "system" fn Uuid<Impl: IRfcommServiceId_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AsShortId<Impl: IRfcommServiceId_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AsString<Impl: IRfcommServiceId_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRfcommServiceId, BASE_OFFSET>(),
            Uuid: Uuid::<Impl, IMPL_OFFSET>,
            AsShortId: AsShortId::<Impl, IMPL_OFFSET>,
            AsString: AsString::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRfcommServiceId as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRfcommServiceIdStatics_Impl: Sized {
    fn FromUuid(&mut self, uuid: &::windows::core::GUID) -> ::windows::core::Result<RfcommServiceId>;
    fn FromShortId(&mut self, shortid: u32) -> ::windows::core::Result<RfcommServiceId>;
    fn SerialPort(&mut self) -> ::windows::core::Result<RfcommServiceId>;
    fn ObexObjectPush(&mut self) -> ::windows::core::Result<RfcommServiceId>;
    fn ObexFileTransfer(&mut self) -> ::windows::core::Result<RfcommServiceId>;
    fn PhoneBookAccessPce(&mut self) -> ::windows::core::Result<RfcommServiceId>;
    fn PhoneBookAccessPse(&mut self) -> ::windows::core::Result<RfcommServiceId>;
    fn GenericFileTransfer(&mut self) -> ::windows::core::Result<RfcommServiceId>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRfcommServiceIdStatics {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Rfcomm.IRfcommServiceIdStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IRfcommServiceIdStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRfcommServiceIdStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRfcommServiceIdStatics_Vtbl {
        unsafe extern "system" fn FromUuid<Impl: IRfcommServiceIdStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uuid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FromShortId<Impl: IRfcommServiceIdStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shortid: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SerialPort<Impl: IRfcommServiceIdStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ObexObjectPush<Impl: IRfcommServiceIdStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ObexFileTransfer<Impl: IRfcommServiceIdStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PhoneBookAccessPce<Impl: IRfcommServiceIdStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PhoneBookAccessPse<Impl: IRfcommServiceIdStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GenericFileTransfer<Impl: IRfcommServiceIdStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRfcommServiceIdStatics, BASE_OFFSET>(),
            FromUuid: FromUuid::<Impl, IMPL_OFFSET>,
            FromShortId: FromShortId::<Impl, IMPL_OFFSET>,
            SerialPort: SerialPort::<Impl, IMPL_OFFSET>,
            ObexObjectPush: ObexObjectPush::<Impl, IMPL_OFFSET>,
            ObexFileTransfer: ObexFileTransfer::<Impl, IMPL_OFFSET>,
            PhoneBookAccessPce: PhoneBookAccessPce::<Impl, IMPL_OFFSET>,
            PhoneBookAccessPse: PhoneBookAccessPse::<Impl, IMPL_OFFSET>,
            GenericFileTransfer: GenericFileTransfer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRfcommServiceIdStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Networking_Sockets", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IRfcommServiceProvider_Impl: Sized {
    fn ServiceId(&mut self) -> ::windows::core::Result<RfcommServiceId>;
    fn SdpRawAttributes(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMap<u32, super::super::super::Storage::Streams::IBuffer>>;
    fn StartAdvertising(&mut self, listener: &::core::option::Option<super::super::super::Networking::Sockets::StreamSocketListener>) -> ::windows::core::Result<()>;
    fn StopAdvertising(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Networking_Sockets", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRfcommServiceProvider {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Rfcomm.IRfcommServiceProvider";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Networking_Sockets", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IRfcommServiceProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRfcommServiceProvider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRfcommServiceProvider_Vtbl {
        unsafe extern "system" fn ServiceId<Impl: IRfcommServiceProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SdpRawAttributes<Impl: IRfcommServiceProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn StartAdvertising<Impl: IRfcommServiceProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listener: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartAdvertising(&*(&listener as *const <super::super::super::Networking::Sockets::StreamSocketListener as ::windows::core::Abi>::Abi as *const <super::super::super::Networking::Sockets::StreamSocketListener as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StopAdvertising<Impl: IRfcommServiceProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StopAdvertising().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRfcommServiceProvider, BASE_OFFSET>(),
            ServiceId: ServiceId::<Impl, IMPL_OFFSET>,
            SdpRawAttributes: SdpRawAttributes::<Impl, IMPL_OFFSET>,
            StartAdvertising: StartAdvertising::<Impl, IMPL_OFFSET>,
            StopAdvertising: StopAdvertising::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRfcommServiceProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Networking_Sockets", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IRfcommServiceProvider2_Impl: Sized + IRfcommServiceProvider_Impl {
    fn StartAdvertisingWithRadioDiscoverability(&mut self, listener: &::core::option::Option<super::super::super::Networking::Sockets::StreamSocketListener>, radiodiscoverable: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Networking_Sockets", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRfcommServiceProvider2 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Rfcomm.IRfcommServiceProvider2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Networking_Sockets", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IRfcommServiceProvider2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRfcommServiceProvider2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRfcommServiceProvider2_Vtbl {
        unsafe extern "system" fn StartAdvertisingWithRadioDiscoverability<Impl: IRfcommServiceProvider2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listener: ::windows::core::RawPtr, radiodiscoverable: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartAdvertisingWithRadioDiscoverability(&*(&listener as *const <super::super::super::Networking::Sockets::StreamSocketListener as ::windows::core::Abi>::Abi as *const <super::super::super::Networking::Sockets::StreamSocketListener as ::windows::core::DefaultType>::DefaultType), radiodiscoverable).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRfcommServiceProvider2, BASE_OFFSET>(),
            StartAdvertisingWithRadioDiscoverability: StartAdvertisingWithRadioDiscoverability::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRfcommServiceProvider2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IRfcommServiceProviderStatics_Impl: Sized {
    fn CreateAsync(&mut self, serviceid: &::core::option::Option<RfcommServiceId>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<RfcommServiceProvider>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRfcommServiceProviderStatics {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Rfcomm.IRfcommServiceProviderStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IRfcommServiceProviderStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRfcommServiceProviderStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRfcommServiceProviderStatics_Vtbl {
        unsafe extern "system" fn CreateAsync<Impl: IRfcommServiceProviderStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, serviceid: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRfcommServiceProviderStatics, BASE_OFFSET>(),
            CreateAsync: CreateAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRfcommServiceProviderStatics as ::windows::core::Interface>::IID
    }
}
