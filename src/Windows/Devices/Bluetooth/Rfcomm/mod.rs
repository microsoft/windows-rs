#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IRfcommDeviceService(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRfcommDeviceService {
    type Vtable = IRfcommDeviceService_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2927755039, 50593, 19520, [140, 40, 243, 239, 214, 144, 98, 243]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRfcommDeviceService_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Networking")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Networking"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Networking_Sockets")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Networking::Sockets::SocketProtectionLevel) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Networking_Sockets"))] usize,
    #[cfg(feature = "Networking_Sockets")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Networking::Sockets::SocketProtectionLevel) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Networking_Sockets"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cachemode: super::BluetoothCacheMode, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IRfcommDeviceService2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRfcommDeviceService2 {
    type Vtable = IRfcommDeviceService2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1399647508, 60365, 18942, [191, 159, 64, 239, 198, 137, 178, 13]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRfcommDeviceService2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IRfcommDeviceService3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRfcommDeviceService3 {
    type Vtable = IRfcommDeviceService3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(472034534, 56644, 19747, [134, 109, 143, 52, 134, 238, 100, 144]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRfcommDeviceService3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Enumeration")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))] usize,
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Devices_Enumeration", feature = "Foundation")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IRfcommDeviceServiceStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRfcommDeviceServiceStatics {
    type Vtable = IRfcommDeviceServiceStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2762033647, 25197, 16812, [178, 83, 135, 172, 92, 39, 226, 138]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRfcommDeviceServiceStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deviceid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, serviceid: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IRfcommDeviceServiceStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRfcommDeviceServiceStatics2 {
    type Vtable = IRfcommDeviceServiceStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2861347273, 59277, 19428, [128, 118, 10, 61, 135, 160, 160, 95]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRfcommDeviceServiceStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bluetoothdevice: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bluetoothdevice: ::windows::runtime::RawPtr, cachemode: super::BluetoothCacheMode, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bluetoothdevice: ::windows::runtime::RawPtr, serviceid: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bluetoothdevice: ::windows::runtime::RawPtr, serviceid: ::windows::runtime::RawPtr, cachemode: super::BluetoothCacheMode, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IRfcommDeviceServicesResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRfcommDeviceServicesResult {
    type Vtable = IRfcommDeviceServicesResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(994588812, 31951, 18574, [150, 37, 210, 89, 165, 115, 45, 85]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRfcommDeviceServicesResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::BluetoothError) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IRfcommServiceId(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRfcommServiceId {
    type Vtable = IRfcommServiceId_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(576885252, 32258, 16407, [129, 54, 218, 27, 106, 27, 155, 191]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRfcommServiceId_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IRfcommServiceIdStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRfcommServiceIdStatics {
    type Vtable = IRfcommServiceIdStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(706191034, 43381, 18147, [181, 107, 8, 255, 215, 131, 165, 254]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRfcommServiceIdStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uuid: ::windows::runtime::GUID, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shortid: u32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IRfcommServiceProvider(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRfcommServiceProvider {
    type Vtable = IRfcommServiceProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3940285892, 45558, 17663, [159, 124, 231, 168, 42, 184, 104, 33]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRfcommServiceProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))] usize,
    #[cfg(feature = "Networking_Sockets")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, listener: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Networking_Sockets"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IRfcommServiceProvider2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRfcommServiceProvider2 {
    type Vtable = IRfcommServiceProvider2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1936449478, 15489, 19742, [186, 242, 221, 187, 129, 40, 69, 18]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRfcommServiceProvider2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Networking_Sockets")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, listener: ::windows::runtime::RawPtr, radiodiscoverable: bool) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Networking_Sockets"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IRfcommServiceProviderStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRfcommServiceProviderStatics {
    type Vtable = IRfcommServiceProviderStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2559083267, 27082, 16698, [132, 247, 67, 68, 199, 41, 41, 151]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRfcommServiceProviderStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, serviceid: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc = "*Required features: `Devices_Bluetooth_Rfcomm`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct RfcommDeviceService(::windows::runtime::IInspectable);
impl RfcommDeviceService {
    #[cfg(feature = "Networking")]
    #[doc = "*Required features: `Devices_Bluetooth_Rfcomm`, `Networking`*"]
    pub fn ConnectionHostName(&self) -> ::windows::runtime::Result<super::super::super::Networking::HostName> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Networking::HostName>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_Rfcomm`*"]
    pub fn ConnectionServiceName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_Rfcomm`*"]
    pub fn ServiceId(&self) -> ::windows::runtime::Result<RfcommServiceId> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<RfcommServiceId>(result__)
        }
    }
    #[cfg(feature = "Networking_Sockets")]
    #[doc = "*Required features: `Devices_Bluetooth_Rfcomm`, `Networking_Sockets`*"]
    pub fn ProtectionLevel(&self) -> ::windows::runtime::Result<super::super::super::Networking::Sockets::SocketProtectionLevel> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Networking::Sockets::SocketProtectionLevel = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Networking::Sockets::SocketProtectionLevel>(result__)
        }
    }
    #[cfg(feature = "Networking_Sockets")]
    #[doc = "*Required features: `Devices_Bluetooth_Rfcomm`, `Networking_Sockets`*"]
    pub fn MaxProtectionLevel(&self) -> ::windows::runtime::Result<super::super::super::Networking::Sockets::SocketProtectionLevel> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Networking::Sockets::SocketProtectionLevel = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Networking::Sockets::SocketProtectionLevel>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Devices_Bluetooth_Rfcomm`, `Foundation`, `Foundation_Collections`, `Storage_Streams`*"]
    pub fn GetSdpRawAttributesAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IMapView<u32, super::super::super::Storage::Streams::IBuffer>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IMapView<u32, super::super::super::Storage::Streams::IBuffer>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Devices_Bluetooth_Rfcomm`, `Foundation`, `Foundation_Collections`, `Storage_Streams`*"]
    pub fn GetSdpRawAttributesWithCacheModeAsync(&self, cachemode: super::BluetoothCacheMode) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IMapView<u32, super::super::super::Storage::Streams::IBuffer>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), cachemode, &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IMapView<u32, super::super::super::Storage::Streams::IBuffer>>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_Rfcomm`*"]
    pub fn Device(&self) -> ::windows::runtime::Result<super::BluetoothDevice> {
        let this = &::windows::runtime::Interface::cast::<IRfcommDeviceService2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::BluetoothDevice>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_Rfcomm`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_Rfcomm`, `Foundation`*"]
    pub fn FromIdAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(deviceid: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<RfcommDeviceService>> {
        Self::IRfcommDeviceServiceStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<RfcommDeviceService>>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_Rfcomm`*"]
    pub fn GetDeviceSelector<'a, Param0: ::windows::runtime::IntoParam<'a, RfcommServiceId>>(serviceid: Param0) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IRfcommDeviceServiceStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), serviceid.into_param().abi(), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Devices_Enumeration")]
    #[doc = "*Required features: `Devices_Bluetooth_Rfcomm`, `Devices_Enumeration`*"]
    pub fn DeviceAccessInformation(&self) -> ::windows::runtime::Result<super::super::Enumeration::DeviceAccessInformation> {
        let this = &::windows::runtime::Interface::cast::<IRfcommDeviceService3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Enumeration::DeviceAccessInformation>(result__)
        }
    }
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))]
    #[doc = "*Required features: `Devices_Bluetooth_Rfcomm`, `Devices_Enumeration`, `Foundation`*"]
    pub fn RequestAccessAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<super::super::Enumeration::DeviceAccessStatus>> {
        let this = &::windows::runtime::Interface::cast::<IRfcommDeviceService3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::Enumeration::DeviceAccessStatus>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_Rfcomm`*"]
    pub fn GetDeviceSelectorForBluetoothDevice<'a, Param0: ::windows::runtime::IntoParam<'a, super::BluetoothDevice>>(bluetoothdevice: Param0) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IRfcommDeviceServiceStatics2(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), bluetoothdevice.into_param().abi(), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_Rfcomm`*"]
    pub fn GetDeviceSelectorForBluetoothDeviceWithCacheMode<'a, Param0: ::windows::runtime::IntoParam<'a, super::BluetoothDevice>>(bluetoothdevice: Param0, cachemode: super::BluetoothCacheMode) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IRfcommDeviceServiceStatics2(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), bluetoothdevice.into_param().abi(), cachemode, &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_Rfcomm`*"]
    pub fn GetDeviceSelectorForBluetoothDeviceAndServiceId<'a, Param0: ::windows::runtime::IntoParam<'a, super::BluetoothDevice>, Param1: ::windows::runtime::IntoParam<'a, RfcommServiceId>>(bluetoothdevice: Param0, serviceid: Param1) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IRfcommDeviceServiceStatics2(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), bluetoothdevice.into_param().abi(), serviceid.into_param().abi(), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_Rfcomm`*"]
    pub fn GetDeviceSelectorForBluetoothDeviceAndServiceIdWithCacheMode<'a, Param0: ::windows::runtime::IntoParam<'a, super::BluetoothDevice>, Param1: ::windows::runtime::IntoParam<'a, RfcommServiceId>>(bluetoothdevice: Param0, serviceid: Param1, cachemode: super::BluetoothCacheMode) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IRfcommDeviceServiceStatics2(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), bluetoothdevice.into_param().abi(), serviceid.into_param().abi(), cachemode, &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn IRfcommDeviceServiceStatics<R, F: FnOnce(&IRfcommDeviceServiceStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<RfcommDeviceService, IRfcommDeviceServiceStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IRfcommDeviceServiceStatics2<R, F: FnOnce(&IRfcommDeviceServiceStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<RfcommDeviceService, IRfcommDeviceServiceStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RfcommDeviceService {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Rfcomm.RfcommDeviceService;{ae81ff1f-c5a1-4c40-8c28-f3efd69062f3})");
}
unsafe impl ::windows::runtime::Interface for RfcommDeviceService {
    type Vtable = IRfcommDeviceService_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2927755039, 50593, 19520, [140, 40, 243, 239, 214, 144, 98, 243]);
}
impl ::windows::runtime::RuntimeName for RfcommDeviceService {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Rfcomm.RfcommDeviceService";
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<RfcommDeviceService> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: RfcommDeviceService) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&RfcommDeviceService> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &RfcommDeviceService) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for RfcommDeviceService {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for &RfcommDeviceService {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for RfcommDeviceService {}
unsafe impl ::std::marker::Sync for RfcommDeviceService {}
#[doc = "*Required features: `Devices_Bluetooth_Rfcomm`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct RfcommDeviceServicesResult(::windows::runtime::IInspectable);
impl RfcommDeviceServicesResult {
    #[doc = "*Required features: `Devices_Bluetooth_Rfcomm`*"]
    pub fn Error(&self) -> ::windows::runtime::Result<super::BluetoothError> {
        let this = self;
        unsafe {
            let mut result__: super::BluetoothError = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::BluetoothError>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Bluetooth_Rfcomm`, `Foundation_Collections`*"]
    pub fn Services(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<RfcommDeviceService>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<RfcommDeviceService>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RfcommDeviceServicesResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Rfcomm.RfcommDeviceServicesResult;{3b48388c-7ccf-488e-9625-d259a5732d55})");
}
unsafe impl ::windows::runtime::Interface for RfcommDeviceServicesResult {
    type Vtable = IRfcommDeviceServicesResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(994588812, 31951, 18574, [150, 37, 210, 89, 165, 115, 45, 85]);
}
impl ::windows::runtime::RuntimeName for RfcommDeviceServicesResult {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Rfcomm.RfcommDeviceServicesResult";
}
unsafe impl ::std::marker::Send for RfcommDeviceServicesResult {}
unsafe impl ::std::marker::Sync for RfcommDeviceServicesResult {}
#[doc = "*Required features: `Devices_Bluetooth_Rfcomm`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct RfcommServiceId(::windows::runtime::IInspectable);
impl RfcommServiceId {
    #[doc = "*Required features: `Devices_Bluetooth_Rfcomm`*"]
    pub fn Uuid(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_Rfcomm`*"]
    pub fn AsShortId(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_Rfcomm`*"]
    pub fn AsString(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_Rfcomm`*"]
    pub fn FromUuid<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(uuid: Param0) -> ::windows::runtime::Result<RfcommServiceId> {
        Self::IRfcommServiceIdStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), uuid.into_param().abi(), &mut result__).from_abi::<RfcommServiceId>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_Rfcomm`*"]
    pub fn FromShortId(shortid: u32) -> ::windows::runtime::Result<RfcommServiceId> {
        Self::IRfcommServiceIdStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), shortid, &mut result__).from_abi::<RfcommServiceId>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_Rfcomm`*"]
    pub fn SerialPort() -> ::windows::runtime::Result<RfcommServiceId> {
        Self::IRfcommServiceIdStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<RfcommServiceId>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_Rfcomm`*"]
    pub fn ObexObjectPush() -> ::windows::runtime::Result<RfcommServiceId> {
        Self::IRfcommServiceIdStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<RfcommServiceId>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_Rfcomm`*"]
    pub fn ObexFileTransfer() -> ::windows::runtime::Result<RfcommServiceId> {
        Self::IRfcommServiceIdStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<RfcommServiceId>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_Rfcomm`*"]
    pub fn PhoneBookAccessPce() -> ::windows::runtime::Result<RfcommServiceId> {
        Self::IRfcommServiceIdStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<RfcommServiceId>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_Rfcomm`*"]
    pub fn PhoneBookAccessPse() -> ::windows::runtime::Result<RfcommServiceId> {
        Self::IRfcommServiceIdStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<RfcommServiceId>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_Rfcomm`*"]
    pub fn GenericFileTransfer() -> ::windows::runtime::Result<RfcommServiceId> {
        Self::IRfcommServiceIdStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<RfcommServiceId>(result__)
        })
    }
    pub fn IRfcommServiceIdStatics<R, F: FnOnce(&IRfcommServiceIdStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<RfcommServiceId, IRfcommServiceIdStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RfcommServiceId {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Rfcomm.RfcommServiceId;{22629204-7e02-4017-8136-da1b6a1b9bbf})");
}
unsafe impl ::windows::runtime::Interface for RfcommServiceId {
    type Vtable = IRfcommServiceId_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(576885252, 32258, 16407, [129, 54, 218, 27, 106, 27, 155, 191]);
}
impl ::windows::runtime::RuntimeName for RfcommServiceId {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Rfcomm.RfcommServiceId";
}
unsafe impl ::std::marker::Send for RfcommServiceId {}
unsafe impl ::std::marker::Sync for RfcommServiceId {}
#[doc = "*Required features: `Devices_Bluetooth_Rfcomm`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct RfcommServiceProvider(::windows::runtime::IInspectable);
impl RfcommServiceProvider {
    #[doc = "*Required features: `Devices_Bluetooth_Rfcomm`*"]
    pub fn ServiceId(&self) -> ::windows::runtime::Result<RfcommServiceId> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<RfcommServiceId>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Devices_Bluetooth_Rfcomm`, `Foundation_Collections`, `Storage_Streams`*"]
    pub fn SdpRawAttributes(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IMap<u32, super::super::super::Storage::Streams::IBuffer>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IMap<u32, super::super::super::Storage::Streams::IBuffer>>(result__)
        }
    }
    #[cfg(feature = "Networking_Sockets")]
    #[doc = "*Required features: `Devices_Bluetooth_Rfcomm`, `Networking_Sockets`*"]
    pub fn StartAdvertising<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Networking::Sockets::StreamSocketListener>>(&self, listener: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), listener.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_Bluetooth_Rfcomm`*"]
    pub fn StopAdvertising(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_Rfcomm`, `Foundation`*"]
    pub fn CreateAsync<'a, Param0: ::windows::runtime::IntoParam<'a, RfcommServiceId>>(serviceid: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<RfcommServiceProvider>> {
        Self::IRfcommServiceProviderStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), serviceid.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<RfcommServiceProvider>>(result__)
        })
    }
    #[cfg(feature = "Networking_Sockets")]
    #[doc = "*Required features: `Devices_Bluetooth_Rfcomm`, `Networking_Sockets`*"]
    pub fn StartAdvertisingWithRadioDiscoverability<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Networking::Sockets::StreamSocketListener>>(&self, listener: Param0, radiodiscoverable: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IRfcommServiceProvider2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), listener.into_param().abi(), radiodiscoverable).ok() }
    }
    pub fn IRfcommServiceProviderStatics<R, F: FnOnce(&IRfcommServiceProviderStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<RfcommServiceProvider, IRfcommServiceProviderStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RfcommServiceProvider {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Rfcomm.RfcommServiceProvider;{eadbfdc4-b1f6-44ff-9f7c-e7a82ab86821})");
}
unsafe impl ::windows::runtime::Interface for RfcommServiceProvider {
    type Vtable = IRfcommServiceProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3940285892, 45558, 17663, [159, 124, 231, 168, 42, 184, 104, 33]);
}
impl ::windows::runtime::RuntimeName for RfcommServiceProvider {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Rfcomm.RfcommServiceProvider";
}
unsafe impl ::std::marker::Send for RfcommServiceProvider {}
unsafe impl ::std::marker::Sync for RfcommServiceProvider {}
