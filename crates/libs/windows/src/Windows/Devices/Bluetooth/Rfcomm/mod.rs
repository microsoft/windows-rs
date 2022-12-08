#[doc(hidden)]
#[repr(transparent)]
pub struct IRfcommDeviceService(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRfcommDeviceService {
    type Vtable = IRfcommDeviceService_Vtbl;
}
unsafe impl ::windows::core::Interface for IRfcommDeviceService {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae81ff1f_c5a1_4c40_8c28_f3efd69062f3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRfcommDeviceService_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Networking")]
    pub ConnectionHostName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Networking"))]
    ConnectionHostName: usize,
    pub ConnectionServiceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ServiceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Networking_Sockets")]
    pub ProtectionLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Networking::Sockets::SocketProtectionLevel) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Networking_Sockets"))]
    ProtectionLevel: usize,
    #[cfg(feature = "Networking_Sockets")]
    pub MaxProtectionLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Networking::Sockets::SocketProtectionLevel) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Networking_Sockets"))]
    MaxProtectionLevel: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub GetSdpRawAttributesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    GetSdpRawAttributesAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub GetSdpRawAttributesWithCacheModeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cachemode: super::BluetoothCacheMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    GetSdpRawAttributesWithCacheModeAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRfcommDeviceService2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRfcommDeviceService2 {
    type Vtable = IRfcommDeviceService2_Vtbl;
}
unsafe impl ::windows::core::Interface for IRfcommDeviceService2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x536ced14_ebcd_49fe_bf9f_40efc689b20d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRfcommDeviceService2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Device: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRfcommDeviceService3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRfcommDeviceService3 {
    type Vtable = IRfcommDeviceService3_Vtbl;
}
unsafe impl ::windows::core::Interface for IRfcommDeviceService3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1c22ace6_dd44_4d23_866d_8f3486ee6490);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRfcommDeviceService3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Enumeration")]
    pub DeviceAccessInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    DeviceAccessInformation: usize,
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))]
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Enumeration", feature = "Foundation")))]
    RequestAccessAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRfcommDeviceServiceStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRfcommDeviceServiceStatics {
    type Vtable = IRfcommDeviceServiceStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IRfcommDeviceServiceStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa4a149ef_626d_41ac_b253_87ac5c27e28a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRfcommDeviceServiceStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, serviceid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRfcommDeviceServiceStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRfcommDeviceServiceStatics2 {
    type Vtable = IRfcommDeviceServiceStatics2_Vtbl;
}
unsafe impl ::windows::core::Interface for IRfcommDeviceServiceStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa8cb1c9_e78d_4be4_8076_0a3d87a0a05f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRfcommDeviceServiceStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDeviceSelectorForBluetoothDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bluetoothdevice: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDeviceSelectorForBluetoothDeviceWithCacheMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bluetoothdevice: *mut ::core::ffi::c_void, cachemode: super::BluetoothCacheMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDeviceSelectorForBluetoothDeviceAndServiceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bluetoothdevice: *mut ::core::ffi::c_void, serviceid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDeviceSelectorForBluetoothDeviceAndServiceIdWithCacheMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bluetoothdevice: *mut ::core::ffi::c_void, serviceid: *mut ::core::ffi::c_void, cachemode: super::BluetoothCacheMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRfcommDeviceServicesResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRfcommDeviceServicesResult {
    type Vtable = IRfcommDeviceServicesResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IRfcommDeviceServicesResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3b48388c_7ccf_488e_9625_d259a5732d55);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRfcommDeviceServicesResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Error: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::BluetoothError) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Services: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Services: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRfcommServiceId(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRfcommServiceId {
    type Vtable = IRfcommServiceId_Vtbl;
}
unsafe impl ::windows::core::Interface for IRfcommServiceId {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x22629204_7e02_4017_8136_da1b6a1b9bbf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRfcommServiceId_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Uuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub AsShortId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub AsString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRfcommServiceIdStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRfcommServiceIdStatics {
    type Vtable = IRfcommServiceIdStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IRfcommServiceIdStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2a179eba_a975_46e3_b56b_08ffd783a5fe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRfcommServiceIdStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub FromUuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uuid: ::windows::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FromShortId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shortid: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SerialPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ObexObjectPush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ObexFileTransfer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PhoneBookAccessPce: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PhoneBookAccessPse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GenericFileTransfer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRfcommServiceProvider(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRfcommServiceProvider {
    type Vtable = IRfcommServiceProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for IRfcommServiceProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeadbfdc4_b1f6_44ff_9f7c_e7a82ab86821);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRfcommServiceProvider_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ServiceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub SdpRawAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    SdpRawAttributes: usize,
    #[cfg(feature = "Networking_Sockets")]
    pub StartAdvertising: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, listener: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Networking_Sockets"))]
    StartAdvertising: usize,
    pub StopAdvertising: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRfcommServiceProvider2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRfcommServiceProvider2 {
    type Vtable = IRfcommServiceProvider2_Vtbl;
}
unsafe impl ::windows::core::Interface for IRfcommServiceProvider2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x736bdfc6_3c81_4d1e_baf2_ddbb81284512);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRfcommServiceProvider2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Networking_Sockets")]
    pub StartAdvertisingWithRadioDiscoverability: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, listener: *mut ::core::ffi::c_void, radiodiscoverable: bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Networking_Sockets"))]
    StartAdvertisingWithRadioDiscoverability: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRfcommServiceProviderStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRfcommServiceProviderStatics {
    type Vtable = IRfcommServiceProviderStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IRfcommServiceProviderStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x98888303_69ca_413a_84f7_4344c7292997);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRfcommServiceProviderStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CreateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, serviceid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateAsync: usize,
}
#[doc = "*Required features: `\"Devices_Bluetooth_Rfcomm\"`*"]
#[repr(transparent)]
pub struct RfcommDeviceService(::windows::core::IUnknown);
impl RfcommDeviceService {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Networking\"`*"]
    #[cfg(feature = "Networking")]
    pub fn ConnectionHostName(&self) -> ::windows::core::Result<super::super::super::Networking::HostName> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ConnectionHostName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ConnectionServiceName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ConnectionServiceName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ServiceId(&self) -> ::windows::core::Result<RfcommServiceId> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ServiceId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Sockets\"`*"]
    #[cfg(feature = "Networking_Sockets")]
    pub fn ProtectionLevel(&self) -> ::windows::core::Result<super::super::super::Networking::Sockets::SocketProtectionLevel> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ProtectionLevel)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Sockets\"`*"]
    #[cfg(feature = "Networking_Sockets")]
    pub fn MaxProtectionLevel(&self) -> ::windows::core::Result<super::super::super::Networking::Sockets::SocketProtectionLevel> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxProtectionLevel)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn GetSdpRawAttributesAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IMapView<u32, super::super::super::Storage::Streams::IBuffer>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetSdpRawAttributesAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn GetSdpRawAttributesWithCacheModeAsync(&self, cachemode: super::BluetoothCacheMode) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IMapView<u32, super::super::super::Storage::Streams::IBuffer>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetSdpRawAttributesWithCacheModeAsync)(::windows::core::Vtable::as_raw(this), cachemode, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Device(&self) -> ::windows::core::Result<super::BluetoothDevice> {
        let this = &::windows::core::Interface::cast::<IRfcommDeviceService2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Device)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Enumeration\"`*"]
    #[cfg(feature = "Devices_Enumeration")]
    pub fn DeviceAccessInformation(&self) -> ::windows::core::Result<super::super::Enumeration::DeviceAccessInformation> {
        let this = &::windows::core::Interface::cast::<IRfcommDeviceService3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DeviceAccessInformation)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Enumeration\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))]
    pub fn RequestAccessAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::Enumeration::DeviceAccessStatus>> {
        let this = &::windows::core::Interface::cast::<IRfcommDeviceService3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestAccessAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync(deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<RfcommDeviceService>> {
        Self::IRfcommDeviceServiceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FromIdAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(deviceid), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn GetDeviceSelector(serviceid: &RfcommServiceId) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IRfcommDeviceServiceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeviceSelector)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(serviceid), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn GetDeviceSelectorForBluetoothDevice(bluetoothdevice: &super::BluetoothDevice) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IRfcommDeviceServiceStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeviceSelectorForBluetoothDevice)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(bluetoothdevice), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn GetDeviceSelectorForBluetoothDeviceWithCacheMode(bluetoothdevice: &super::BluetoothDevice, cachemode: super::BluetoothCacheMode) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IRfcommDeviceServiceStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeviceSelectorForBluetoothDeviceWithCacheMode)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(bluetoothdevice), cachemode, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn GetDeviceSelectorForBluetoothDeviceAndServiceId(bluetoothdevice: &super::BluetoothDevice, serviceid: &RfcommServiceId) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IRfcommDeviceServiceStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeviceSelectorForBluetoothDeviceAndServiceId)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(bluetoothdevice), ::core::mem::transmute_copy(serviceid), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn GetDeviceSelectorForBluetoothDeviceAndServiceIdWithCacheMode(bluetoothdevice: &super::BluetoothDevice, serviceid: &RfcommServiceId, cachemode: super::BluetoothCacheMode) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IRfcommDeviceServiceStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeviceSelectorForBluetoothDeviceAndServiceIdWithCacheMode)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(bluetoothdevice), ::core::mem::transmute_copy(serviceid), cachemode, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IRfcommDeviceServiceStatics<R, F: FnOnce(&IRfcommDeviceServiceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<RfcommDeviceService, IRfcommDeviceServiceStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IRfcommDeviceServiceStatics2<R, F: FnOnce(&IRfcommDeviceServiceStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<RfcommDeviceService, IRfcommDeviceServiceStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for RfcommDeviceService {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RfcommDeviceService {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RfcommDeviceService {}
impl ::core::fmt::Debug for RfcommDeviceService {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RfcommDeviceService").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RfcommDeviceService {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Rfcomm.RfcommDeviceService;{ae81ff1f-c5a1-4c40-8c28-f3efd69062f3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for RfcommDeviceService {
    type Vtable = IRfcommDeviceService_Vtbl;
}
unsafe impl ::windows::core::Interface for RfcommDeviceService {
    const IID: ::windows::core::GUID = <IRfcommDeviceService as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RfcommDeviceService {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Rfcomm.RfcommDeviceService";
}
::windows::core::interface_hierarchy!(RfcommDeviceService, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<RfcommDeviceService> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: RfcommDeviceService) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&RfcommDeviceService> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &RfcommDeviceService) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&RfcommDeviceService> for ::windows::core::InParam<super::super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &RfcommDeviceService) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for RfcommDeviceService {}
unsafe impl ::core::marker::Sync for RfcommDeviceService {}
#[doc = "*Required features: `\"Devices_Bluetooth_Rfcomm\"`*"]
#[repr(transparent)]
pub struct RfcommDeviceServicesResult(::windows::core::IUnknown);
impl RfcommDeviceServicesResult {
    pub fn Error(&self) -> ::windows::core::Result<super::BluetoothError> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Error)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Services(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<RfcommDeviceService>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Services)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for RfcommDeviceServicesResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RfcommDeviceServicesResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RfcommDeviceServicesResult {}
impl ::core::fmt::Debug for RfcommDeviceServicesResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RfcommDeviceServicesResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RfcommDeviceServicesResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Rfcomm.RfcommDeviceServicesResult;{3b48388c-7ccf-488e-9625-d259a5732d55})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for RfcommDeviceServicesResult {
    type Vtable = IRfcommDeviceServicesResult_Vtbl;
}
unsafe impl ::windows::core::Interface for RfcommDeviceServicesResult {
    const IID: ::windows::core::GUID = <IRfcommDeviceServicesResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RfcommDeviceServicesResult {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Rfcomm.RfcommDeviceServicesResult";
}
::windows::core::interface_hierarchy!(RfcommDeviceServicesResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for RfcommDeviceServicesResult {}
unsafe impl ::core::marker::Sync for RfcommDeviceServicesResult {}
#[doc = "*Required features: `\"Devices_Bluetooth_Rfcomm\"`*"]
#[repr(transparent)]
pub struct RfcommServiceId(::windows::core::IUnknown);
impl RfcommServiceId {
    pub fn Uuid(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Uuid)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AsShortId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AsShortId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AsString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AsString)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn FromUuid(uuid: ::windows::core::GUID) -> ::windows::core::Result<RfcommServiceId> {
        Self::IRfcommServiceIdStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FromUuid)(::windows::core::Vtable::as_raw(this), uuid, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn FromShortId(shortid: u32) -> ::windows::core::Result<RfcommServiceId> {
        Self::IRfcommServiceIdStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FromShortId)(::windows::core::Vtable::as_raw(this), shortid, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn SerialPort() -> ::windows::core::Result<RfcommServiceId> {
        Self::IRfcommServiceIdStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SerialPort)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn ObexObjectPush() -> ::windows::core::Result<RfcommServiceId> {
        Self::IRfcommServiceIdStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ObexObjectPush)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn ObexFileTransfer() -> ::windows::core::Result<RfcommServiceId> {
        Self::IRfcommServiceIdStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ObexFileTransfer)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn PhoneBookAccessPce() -> ::windows::core::Result<RfcommServiceId> {
        Self::IRfcommServiceIdStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PhoneBookAccessPce)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn PhoneBookAccessPse() -> ::windows::core::Result<RfcommServiceId> {
        Self::IRfcommServiceIdStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PhoneBookAccessPse)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn GenericFileTransfer() -> ::windows::core::Result<RfcommServiceId> {
        Self::IRfcommServiceIdStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GenericFileTransfer)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IRfcommServiceIdStatics<R, F: FnOnce(&IRfcommServiceIdStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<RfcommServiceId, IRfcommServiceIdStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for RfcommServiceId {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RfcommServiceId {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RfcommServiceId {}
impl ::core::fmt::Debug for RfcommServiceId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RfcommServiceId").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RfcommServiceId {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Rfcomm.RfcommServiceId;{22629204-7e02-4017-8136-da1b6a1b9bbf})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for RfcommServiceId {
    type Vtable = IRfcommServiceId_Vtbl;
}
unsafe impl ::windows::core::Interface for RfcommServiceId {
    const IID: ::windows::core::GUID = <IRfcommServiceId as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RfcommServiceId {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Rfcomm.RfcommServiceId";
}
::windows::core::interface_hierarchy!(RfcommServiceId, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for RfcommServiceId {}
unsafe impl ::core::marker::Sync for RfcommServiceId {}
#[doc = "*Required features: `\"Devices_Bluetooth_Rfcomm\"`*"]
#[repr(transparent)]
pub struct RfcommServiceProvider(::windows::core::IUnknown);
impl RfcommServiceProvider {
    pub fn ServiceId(&self) -> ::windows::core::Result<RfcommServiceId> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ServiceId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn SdpRawAttributes(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMap<u32, super::super::super::Storage::Streams::IBuffer>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SdpRawAttributes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Sockets\"`*"]
    #[cfg(feature = "Networking_Sockets")]
    pub fn StartAdvertising(&self, listener: &super::super::super::Networking::Sockets::StreamSocketListener) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).StartAdvertising)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(listener)).ok() }
    }
    pub fn StopAdvertising(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).StopAdvertising)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Networking_Sockets\"`*"]
    #[cfg(feature = "Networking_Sockets")]
    pub fn StartAdvertisingWithRadioDiscoverability(&self, listener: &super::super::super::Networking::Sockets::StreamSocketListener, radiodiscoverable: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IRfcommServiceProvider2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).StartAdvertisingWithRadioDiscoverability)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(listener), radiodiscoverable).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateAsync(serviceid: &RfcommServiceId) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<RfcommServiceProvider>> {
        Self::IRfcommServiceProviderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(serviceid), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IRfcommServiceProviderStatics<R, F: FnOnce(&IRfcommServiceProviderStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<RfcommServiceProvider, IRfcommServiceProviderStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for RfcommServiceProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RfcommServiceProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RfcommServiceProvider {}
impl ::core::fmt::Debug for RfcommServiceProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RfcommServiceProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RfcommServiceProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Rfcomm.RfcommServiceProvider;{eadbfdc4-b1f6-44ff-9f7c-e7a82ab86821})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for RfcommServiceProvider {
    type Vtable = IRfcommServiceProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for RfcommServiceProvider {
    const IID: ::windows::core::GUID = <IRfcommServiceProvider as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RfcommServiceProvider {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Rfcomm.RfcommServiceProvider";
}
::windows::core::interface_hierarchy!(RfcommServiceProvider, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for RfcommServiceProvider {}
unsafe impl ::core::marker::Sync for RfcommServiceProvider {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
