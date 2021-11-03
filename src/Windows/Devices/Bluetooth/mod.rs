#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Devices_Bluetooth_Advertisement")]
pub mod Advertisement;
#[cfg(feature = "Devices_Bluetooth_Background")]
pub mod Background;
#[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
pub mod GenericAttributeProfile;
#[cfg(feature = "Devices_Bluetooth_Rfcomm")]
pub mod Rfcomm;
#[doc = "*Required features: `Devices_Bluetooth`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct BluetoothAdapter(::windows::runtime::IInspectable);
impl BluetoothAdapter {
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn DeviceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn BluetoothAddress(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn IsClassicSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn IsLowEnergySupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn IsPeripheralRoleSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn IsCentralRoleSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn IsAdvertisementOffloadSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(all(feature = "Devices_Radios", feature = "Foundation"))]
    #[doc = "*Required features: `Devices_Bluetooth`, `Devices_Radios`, `Foundation`*"]
    pub fn GetRadioAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::Radios::Radio>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::Radios::Radio>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn GetDeviceSelector() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IBluetoothAdapterStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`*"]
    pub fn FromIdAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(deviceid: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<BluetoothAdapter>> {
        Self::IBluetoothAdapterStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<BluetoothAdapter>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`*"]
    pub fn GetDefaultAsync() -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<BluetoothAdapter>> {
        Self::IBluetoothAdapterStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<BluetoothAdapter>>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn AreClassicSecureConnectionsSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IBluetoothAdapter2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn AreLowEnergySecureConnectionsSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IBluetoothAdapter2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn IsExtendedAdvertisingSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IBluetoothAdapter3>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn MaxAdvertisementDataLength(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IBluetoothAdapter3>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn IBluetoothAdapterStatics<R, F: FnOnce(&IBluetoothAdapterStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<BluetoothAdapter, IBluetoothAdapterStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BluetoothAdapter {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.BluetoothAdapter;{7974f04c-5f7a-4a34-9225-a855f84b1a8b})");
}
unsafe impl ::windows::runtime::Interface for BluetoothAdapter {
    type Vtable = IBluetoothAdapter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2037706828, 24442, 18996, [146, 37, 168, 85, 248, 75, 26, 139]);
}
impl ::windows::runtime::RuntimeName for BluetoothAdapter {
    const NAME: &'static str = "Windows.Devices.Bluetooth.BluetoothAdapter";
}
unsafe impl ::std::marker::Send for BluetoothAdapter {}
unsafe impl ::std::marker::Sync for BluetoothAdapter {}
#[doc = "*Required features: `Devices_Bluetooth`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct BluetoothAddressType(pub i32);
impl BluetoothAddressType {
    pub const Public: BluetoothAddressType = BluetoothAddressType(0i32);
    pub const Random: BluetoothAddressType = BluetoothAddressType(1i32);
    pub const Unspecified: BluetoothAddressType = BluetoothAddressType(2i32);
}
impl ::std::convert::From<i32> for BluetoothAddressType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for BluetoothAddressType {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for BluetoothAddressType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.BluetoothAddressType;i4)");
}
impl ::windows::runtime::DefaultType for BluetoothAddressType {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Bluetooth`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct BluetoothCacheMode(pub i32);
impl BluetoothCacheMode {
    pub const Cached: BluetoothCacheMode = BluetoothCacheMode(0i32);
    pub const Uncached: BluetoothCacheMode = BluetoothCacheMode(1i32);
}
impl ::std::convert::From<i32> for BluetoothCacheMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for BluetoothCacheMode {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for BluetoothCacheMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.BluetoothCacheMode;i4)");
}
impl ::windows::runtime::DefaultType for BluetoothCacheMode {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Bluetooth`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct BluetoothClassOfDevice(::windows::runtime::IInspectable);
impl BluetoothClassOfDevice {
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn RawValue(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn MajorClass(&self) -> ::windows::runtime::Result<BluetoothMajorClass> {
        let this = self;
        unsafe {
            let mut result__: BluetoothMajorClass = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothMajorClass>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn MinorClass(&self) -> ::windows::runtime::Result<BluetoothMinorClass> {
        let this = self;
        unsafe {
            let mut result__: BluetoothMinorClass = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothMinorClass>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn ServiceCapabilities(&self) -> ::windows::runtime::Result<BluetoothServiceCapabilities> {
        let this = self;
        unsafe {
            let mut result__: BluetoothServiceCapabilities = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothServiceCapabilities>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn FromRawValue(rawvalue: u32) -> ::windows::runtime::Result<BluetoothClassOfDevice> {
        Self::IBluetoothClassOfDeviceStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), rawvalue, &mut result__).from_abi::<BluetoothClassOfDevice>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn FromParts(majorclass: BluetoothMajorClass, minorclass: BluetoothMinorClass, servicecapabilities: BluetoothServiceCapabilities) -> ::windows::runtime::Result<BluetoothClassOfDevice> {
        Self::IBluetoothClassOfDeviceStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), majorclass, minorclass, servicecapabilities, &mut result__).from_abi::<BluetoothClassOfDevice>(result__)
        })
    }
    pub fn IBluetoothClassOfDeviceStatics<R, F: FnOnce(&IBluetoothClassOfDeviceStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<BluetoothClassOfDevice, IBluetoothClassOfDeviceStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BluetoothClassOfDevice {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.BluetoothClassOfDevice;{d640227e-d7d7-4661-9454-65039ca17a2b})");
}
unsafe impl ::windows::runtime::Interface for BluetoothClassOfDevice {
    type Vtable = IBluetoothClassOfDevice_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3594527358, 55255, 18017, [148, 84, 101, 3, 156, 161, 122, 43]);
}
impl ::windows::runtime::RuntimeName for BluetoothClassOfDevice {
    const NAME: &'static str = "Windows.Devices.Bluetooth.BluetoothClassOfDevice";
}
unsafe impl ::std::marker::Send for BluetoothClassOfDevice {}
unsafe impl ::std::marker::Sync for BluetoothClassOfDevice {}
#[doc = "*Required features: `Devices_Bluetooth`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct BluetoothConnectionStatus(pub i32);
impl BluetoothConnectionStatus {
    pub const Disconnected: BluetoothConnectionStatus = BluetoothConnectionStatus(0i32);
    pub const Connected: BluetoothConnectionStatus = BluetoothConnectionStatus(1i32);
}
impl ::std::convert::From<i32> for BluetoothConnectionStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for BluetoothConnectionStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for BluetoothConnectionStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.BluetoothConnectionStatus;i4)");
}
impl ::windows::runtime::DefaultType for BluetoothConnectionStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Bluetooth`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct BluetoothDevice(::windows::runtime::IInspectable);
impl BluetoothDevice {
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn DeviceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Networking")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Networking`*"]
    pub fn HostName(&self) -> ::windows::runtime::Result<super::super::Networking::HostName> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Networking::HostName>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn ClassOfDevice(&self) -> ::windows::runtime::Result<BluetoothClassOfDevice> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothClassOfDevice>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation_Collections`, `Storage_Streams`*"]
    pub fn SdpRecords(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<super::super::Storage::Streams::IBuffer>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Storage::Streams::IBuffer>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_Bluetooth`, `Devices_Bluetooth_Rfcomm`, `Foundation_Collections`*"]
    pub fn RfcommServices(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<Rfcomm::RfcommDeviceService>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<Rfcomm::RfcommDeviceService>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn ConnectionStatus(&self) -> ::windows::runtime::Result<BluetoothConnectionStatus> {
        let this = self;
        unsafe {
            let mut result__: BluetoothConnectionStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothConnectionStatus>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn BluetoothAddress(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`*"]
    pub fn NameChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<BluetoothDevice, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`*"]
    pub fn RemoveNameChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`*"]
    pub fn SdpRecordsChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<BluetoothDevice, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`*"]
    pub fn RemoveSdpRecordsChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`*"]
    pub fn ConnectionStatusChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<BluetoothDevice, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`*"]
    pub fn RemoveConnectionStatusChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`*"]
    pub fn FromIdAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(deviceid: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<BluetoothDevice>> {
        Self::IBluetoothDeviceStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<BluetoothDevice>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Networking"))]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`, `Networking`*"]
    pub fn FromHostNameAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Networking::HostName>>(hostname: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<BluetoothDevice>> {
        Self::IBluetoothDeviceStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), hostname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<BluetoothDevice>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`*"]
    pub fn FromBluetoothAddressAsync(address: u64) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<BluetoothDevice>> {
        Self::IBluetoothDeviceStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), address, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<BluetoothDevice>>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn GetDeviceSelector() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IBluetoothDeviceStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Devices_Enumeration")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Devices_Enumeration`*"]
    pub fn DeviceInformation(&self) -> ::windows::runtime::Result<super::Enumeration::DeviceInformation> {
        let this = &::windows::runtime::Interface::cast::<IBluetoothDevice2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::Enumeration::DeviceInformation>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn GetDeviceSelectorFromPairingState(pairingstate: bool) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IBluetoothDeviceStatics2(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), pairingstate, &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn GetDeviceSelectorFromConnectionStatus(connectionstatus: BluetoothConnectionStatus) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IBluetoothDeviceStatics2(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), connectionstatus, &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn GetDeviceSelectorFromDeviceName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(devicename: Param0) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IBluetoothDeviceStatics2(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), devicename.into_param().abi(), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn GetDeviceSelectorFromBluetoothAddress(bluetoothaddress: u64) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IBluetoothDeviceStatics2(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), bluetoothaddress, &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn GetDeviceSelectorFromClassOfDevice<'a, Param0: ::windows::runtime::IntoParam<'a, BluetoothClassOfDevice>>(classofdevice: Param0) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IBluetoothDeviceStatics2(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), classofdevice.into_param().abi(), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Devices_Enumeration")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Devices_Enumeration`*"]
    pub fn DeviceAccessInformation(&self) -> ::windows::runtime::Result<super::Enumeration::DeviceAccessInformation> {
        let this = &::windows::runtime::Interface::cast::<IBluetoothDevice3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::Enumeration::DeviceAccessInformation>(result__)
        }
    }
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))]
    #[doc = "*Required features: `Devices_Bluetooth`, `Devices_Enumeration`, `Foundation`*"]
    pub fn RequestAccessAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::Enumeration::DeviceAccessStatus>> {
        let this = &::windows::runtime::Interface::cast::<IBluetoothDevice3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::Enumeration::DeviceAccessStatus>>(result__)
        }
    }
    #[cfg(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Foundation"))]
    #[doc = "*Required features: `Devices_Bluetooth`, `Devices_Bluetooth_Rfcomm`, `Foundation`*"]
    pub fn GetRfcommServicesAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<Rfcomm::RfcommDeviceServicesResult>> {
        let this = &::windows::runtime::Interface::cast::<IBluetoothDevice3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<Rfcomm::RfcommDeviceServicesResult>>(result__)
        }
    }
    #[cfg(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Foundation"))]
    #[doc = "*Required features: `Devices_Bluetooth`, `Devices_Bluetooth_Rfcomm`, `Foundation`*"]
    pub fn GetRfcommServicesWithCacheModeAsync(&self, cachemode: BluetoothCacheMode) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<Rfcomm::RfcommDeviceServicesResult>> {
        let this = &::windows::runtime::Interface::cast::<IBluetoothDevice3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), cachemode, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<Rfcomm::RfcommDeviceServicesResult>>(result__)
        }
    }
    #[cfg(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Foundation"))]
    #[doc = "*Required features: `Devices_Bluetooth`, `Devices_Bluetooth_Rfcomm`, `Foundation`*"]
    pub fn GetRfcommServicesForIdAsync<'a, Param0: ::windows::runtime::IntoParam<'a, Rfcomm::RfcommServiceId>>(&self, serviceid: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<Rfcomm::RfcommDeviceServicesResult>> {
        let this = &::windows::runtime::Interface::cast::<IBluetoothDevice3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), serviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<Rfcomm::RfcommDeviceServicesResult>>(result__)
        }
    }
    #[cfg(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Foundation"))]
    #[doc = "*Required features: `Devices_Bluetooth`, `Devices_Bluetooth_Rfcomm`, `Foundation`*"]
    pub fn GetRfcommServicesForIdWithCacheModeAsync<'a, Param0: ::windows::runtime::IntoParam<'a, Rfcomm::RfcommServiceId>>(&self, serviceid: Param0, cachemode: BluetoothCacheMode) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<Rfcomm::RfcommDeviceServicesResult>> {
        let this = &::windows::runtime::Interface::cast::<IBluetoothDevice3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), serviceid.into_param().abi(), cachemode, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<Rfcomm::RfcommDeviceServicesResult>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn BluetoothDeviceId(&self) -> ::windows::runtime::Result<BluetoothDeviceId> {
        let this = &::windows::runtime::Interface::cast::<IBluetoothDevice4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothDeviceId>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn WasSecureConnectionUsedForPairing(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IBluetoothDevice5>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IBluetoothDeviceStatics<R, F: FnOnce(&IBluetoothDeviceStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<BluetoothDevice, IBluetoothDeviceStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IBluetoothDeviceStatics2<R, F: FnOnce(&IBluetoothDeviceStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<BluetoothDevice, IBluetoothDeviceStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BluetoothDevice {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.BluetoothDevice;{2335b156-90d2-4a04-aef5-0e20b9e6b707})");
}
unsafe impl ::windows::runtime::Interface for BluetoothDevice {
    type Vtable = IBluetoothDevice_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(590721366, 37074, 18948, [174, 245, 14, 32, 185, 230, 183, 7]);
}
impl ::windows::runtime::RuntimeName for BluetoothDevice {
    const NAME: &'static str = "Windows.Devices.Bluetooth.BluetoothDevice";
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<BluetoothDevice> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: BluetoothDevice) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&BluetoothDevice> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &BluetoothDevice) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for BluetoothDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &BluetoothDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for BluetoothDevice {}
unsafe impl ::std::marker::Sync for BluetoothDevice {}
#[doc = "*Required features: `Devices_Bluetooth`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct BluetoothDeviceId(::windows::runtime::IInspectable);
impl BluetoothDeviceId {
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn IsClassicDevice(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn IsLowEnergyDevice(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn FromId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(deviceid: Param0) -> ::windows::runtime::Result<BluetoothDeviceId> {
        Self::IBluetoothDeviceIdStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<BluetoothDeviceId>(result__)
        })
    }
    pub fn IBluetoothDeviceIdStatics<R, F: FnOnce(&IBluetoothDeviceIdStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<BluetoothDeviceId, IBluetoothDeviceIdStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BluetoothDeviceId {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.BluetoothDeviceId;{c17949af-57c1-4642-bcce-e6c06b20ae76})");
}
unsafe impl ::windows::runtime::Interface for BluetoothDeviceId {
    type Vtable = IBluetoothDeviceId_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3245951407, 22465, 17986, [188, 206, 230, 192, 107, 32, 174, 118]);
}
impl ::windows::runtime::RuntimeName for BluetoothDeviceId {
    const NAME: &'static str = "Windows.Devices.Bluetooth.BluetoothDeviceId";
}
unsafe impl ::std::marker::Send for BluetoothDeviceId {}
unsafe impl ::std::marker::Sync for BluetoothDeviceId {}
#[doc = "*Required features: `Devices_Bluetooth`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct BluetoothError(pub i32);
impl BluetoothError {
    pub const Success: BluetoothError = BluetoothError(0i32);
    pub const RadioNotAvailable: BluetoothError = BluetoothError(1i32);
    pub const ResourceInUse: BluetoothError = BluetoothError(2i32);
    pub const DeviceNotConnected: BluetoothError = BluetoothError(3i32);
    pub const OtherError: BluetoothError = BluetoothError(4i32);
    pub const DisabledByPolicy: BluetoothError = BluetoothError(5i32);
    pub const NotSupported: BluetoothError = BluetoothError(6i32);
    pub const DisabledByUser: BluetoothError = BluetoothError(7i32);
    pub const ConsentRequired: BluetoothError = BluetoothError(8i32);
    pub const TransportNotSupported: BluetoothError = BluetoothError(9i32);
}
impl ::std::convert::From<i32> for BluetoothError {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for BluetoothError {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for BluetoothError {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.BluetoothError;i4)");
}
impl ::windows::runtime::DefaultType for BluetoothError {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Bluetooth`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct BluetoothLEAppearance(::windows::runtime::IInspectable);
impl BluetoothLEAppearance {
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn RawValue(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn Category(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn SubCategory(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn FromRawValue(rawvalue: u16) -> ::windows::runtime::Result<BluetoothLEAppearance> {
        Self::IBluetoothLEAppearanceStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), rawvalue, &mut result__).from_abi::<BluetoothLEAppearance>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn FromParts(appearancecategory: u16, appearancesubcategory: u16) -> ::windows::runtime::Result<BluetoothLEAppearance> {
        Self::IBluetoothLEAppearanceStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), appearancecategory, appearancesubcategory, &mut result__).from_abi::<BluetoothLEAppearance>(result__)
        })
    }
    pub fn IBluetoothLEAppearanceStatics<R, F: FnOnce(&IBluetoothLEAppearanceStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<BluetoothLEAppearance, IBluetoothLEAppearanceStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BluetoothLEAppearance {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.BluetoothLEAppearance;{5d2079f2-66a8-4258-985e-02b4d9509f18})");
}
unsafe impl ::windows::runtime::Interface for BluetoothLEAppearance {
    type Vtable = IBluetoothLEAppearance_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1562409458, 26280, 16984, [152, 94, 2, 180, 217, 80, 159, 24]);
}
impl ::windows::runtime::RuntimeName for BluetoothLEAppearance {
    const NAME: &'static str = "Windows.Devices.Bluetooth.BluetoothLEAppearance";
}
unsafe impl ::std::marker::Send for BluetoothLEAppearance {}
unsafe impl ::std::marker::Sync for BluetoothLEAppearance {}
#[doc = "*Required features: `Devices_Bluetooth`*"]
pub struct BluetoothLEAppearanceCategories {}
impl BluetoothLEAppearanceCategories {
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn Uncategorized() -> ::windows::runtime::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn Phone() -> ::windows::runtime::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn Computer() -> ::windows::runtime::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn Watch() -> ::windows::runtime::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn Clock() -> ::windows::runtime::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn Display() -> ::windows::runtime::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn RemoteControl() -> ::windows::runtime::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn EyeGlasses() -> ::windows::runtime::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn Tag() -> ::windows::runtime::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn Keyring() -> ::windows::runtime::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn MediaPlayer() -> ::windows::runtime::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn BarcodeScanner() -> ::windows::runtime::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn Thermometer() -> ::windows::runtime::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn HeartRate() -> ::windows::runtime::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn BloodPressure() -> ::windows::runtime::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn HumanInterfaceDevice() -> ::windows::runtime::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn GlucoseMeter() -> ::windows::runtime::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn RunningWalking() -> ::windows::runtime::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn Cycling() -> ::windows::runtime::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn PulseOximeter() -> ::windows::runtime::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn WeightScale() -> ::windows::runtime::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn OutdoorSportActivity() -> ::windows::runtime::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    pub fn IBluetoothLEAppearanceCategoriesStatics<R, F: FnOnce(&IBluetoothLEAppearanceCategoriesStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<BluetoothLEAppearanceCategories, IBluetoothLEAppearanceCategoriesStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for BluetoothLEAppearanceCategories {
    const NAME: &'static str = "Windows.Devices.Bluetooth.BluetoothLEAppearanceCategories";
}
#[doc = "*Required features: `Devices_Bluetooth`*"]
pub struct BluetoothLEAppearanceSubcategories {}
impl BluetoothLEAppearanceSubcategories {
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn Generic() -> ::windows::runtime::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn SportsWatch() -> ::windows::runtime::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn ThermometerEar() -> ::windows::runtime::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn HeartRateBelt() -> ::windows::runtime::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn BloodPressureArm() -> ::windows::runtime::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn BloodPressureWrist() -> ::windows::runtime::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn Keyboard() -> ::windows::runtime::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn Mouse() -> ::windows::runtime::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn Joystick() -> ::windows::runtime::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn Gamepad() -> ::windows::runtime::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn DigitizerTablet() -> ::windows::runtime::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn CardReader() -> ::windows::runtime::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn DigitalPen() -> ::windows::runtime::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn BarcodeScanner() -> ::windows::runtime::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn RunningWalkingInShoe() -> ::windows::runtime::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn RunningWalkingOnShoe() -> ::windows::runtime::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn RunningWalkingOnHip() -> ::windows::runtime::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn CyclingComputer() -> ::windows::runtime::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn CyclingSpeedSensor() -> ::windows::runtime::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn CyclingCadenceSensor() -> ::windows::runtime::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn CyclingPowerSensor() -> ::windows::runtime::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn CyclingSpeedCadenceSensor() -> ::windows::runtime::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn OximeterFingertip() -> ::windows::runtime::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn OximeterWristWorn() -> ::windows::runtime::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn LocationDisplay() -> ::windows::runtime::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn LocationNavigationDisplay() -> ::windows::runtime::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn LocationPod() -> ::windows::runtime::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).32)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn LocationNavigationPod() -> ::windows::runtime::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).33)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    pub fn IBluetoothLEAppearanceSubcategoriesStatics<R, F: FnOnce(&IBluetoothLEAppearanceSubcategoriesStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<BluetoothLEAppearanceSubcategories, IBluetoothLEAppearanceSubcategoriesStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for BluetoothLEAppearanceSubcategories {
    const NAME: &'static str = "Windows.Devices.Bluetooth.BluetoothLEAppearanceSubcategories";
}
#[doc = "*Required features: `Devices_Bluetooth`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct BluetoothLEConnectionParameters(::windows::runtime::IInspectable);
impl BluetoothLEConnectionParameters {
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn LinkTimeout(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn ConnectionLatency(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn ConnectionInterval(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BluetoothLEConnectionParameters {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.BluetoothLEConnectionParameters;{33cb0771-8da9-508f-a366-1ca388c929ab})");
}
unsafe impl ::windows::runtime::Interface for BluetoothLEConnectionParameters {
    type Vtable = IBluetoothLEConnectionParameters_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(868943729, 36265, 20623, [163, 102, 28, 163, 136, 201, 41, 171]);
}
impl ::windows::runtime::RuntimeName for BluetoothLEConnectionParameters {
    const NAME: &'static str = "Windows.Devices.Bluetooth.BluetoothLEConnectionParameters";
}
unsafe impl ::std::marker::Send for BluetoothLEConnectionParameters {}
unsafe impl ::std::marker::Sync for BluetoothLEConnectionParameters {}
#[doc = "*Required features: `Devices_Bluetooth`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct BluetoothLEConnectionPhy(::windows::runtime::IInspectable);
impl BluetoothLEConnectionPhy {
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn TransmitInfo(&self) -> ::windows::runtime::Result<BluetoothLEConnectionPhyInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothLEConnectionPhyInfo>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn ReceiveInfo(&self) -> ::windows::runtime::Result<BluetoothLEConnectionPhyInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothLEConnectionPhyInfo>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BluetoothLEConnectionPhy {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.BluetoothLEConnectionPhy;{781e5e48-621e-5a7e-8be6-1b9561ff63c9})");
}
unsafe impl ::windows::runtime::Interface for BluetoothLEConnectionPhy {
    type Vtable = IBluetoothLEConnectionPhy_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2015256136, 25118, 23166, [139, 230, 27, 149, 97, 255, 99, 201]);
}
impl ::windows::runtime::RuntimeName for BluetoothLEConnectionPhy {
    const NAME: &'static str = "Windows.Devices.Bluetooth.BluetoothLEConnectionPhy";
}
unsafe impl ::std::marker::Send for BluetoothLEConnectionPhy {}
unsafe impl ::std::marker::Sync for BluetoothLEConnectionPhy {}
#[doc = "*Required features: `Devices_Bluetooth`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct BluetoothLEConnectionPhyInfo(::windows::runtime::IInspectable);
impl BluetoothLEConnectionPhyInfo {
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn IsUncoded1MPhy(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn IsUncoded2MPhy(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn IsCodedPhy(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BluetoothLEConnectionPhyInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.BluetoothLEConnectionPhyInfo;{9a100bdd-602e-5c27-a1ae-b230015a6394})");
}
unsafe impl ::windows::runtime::Interface for BluetoothLEConnectionPhyInfo {
    type Vtable = IBluetoothLEConnectionPhyInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2584742877, 24622, 23591, [161, 174, 178, 48, 1, 90, 99, 148]);
}
impl ::windows::runtime::RuntimeName for BluetoothLEConnectionPhyInfo {
    const NAME: &'static str = "Windows.Devices.Bluetooth.BluetoothLEConnectionPhyInfo";
}
unsafe impl ::std::marker::Send for BluetoothLEConnectionPhyInfo {}
unsafe impl ::std::marker::Sync for BluetoothLEConnectionPhyInfo {}
#[doc = "*Required features: `Devices_Bluetooth`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct BluetoothLEDevice(::windows::runtime::IInspectable);
impl BluetoothLEDevice {
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn DeviceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_Bluetooth`, `Devices_Bluetooth_GenericAttributeProfile`, `Foundation_Collections`*"]
    pub fn GattServices(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<GenericAttributeProfile::GattDeviceService>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<GenericAttributeProfile::GattDeviceService>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn ConnectionStatus(&self) -> ::windows::runtime::Result<BluetoothConnectionStatus> {
        let this = self;
        unsafe {
            let mut result__: BluetoothConnectionStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothConnectionStatus>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn BluetoothAddress(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn GetGattService<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, serviceuuid: Param0) -> ::windows::runtime::Result<GenericAttributeProfile::GattDeviceService> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), serviceuuid.into_param().abi(), &mut result__).from_abi::<GenericAttributeProfile::GattDeviceService>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`*"]
    pub fn NameChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<BluetoothLEDevice, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`*"]
    pub fn RemoveNameChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`*"]
    pub fn GattServicesChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<BluetoothLEDevice, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`*"]
    pub fn RemoveGattServicesChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`*"]
    pub fn ConnectionStatusChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<BluetoothLEDevice, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`*"]
    pub fn RemoveConnectionStatusChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`*"]
    pub fn FromIdAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(deviceid: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<BluetoothLEDevice>> {
        Self::IBluetoothLEDeviceStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<BluetoothLEDevice>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`*"]
    pub fn FromBluetoothAddressAsync(bluetoothaddress: u64) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<BluetoothLEDevice>> {
        Self::IBluetoothLEDeviceStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), bluetoothaddress, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<BluetoothLEDevice>>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn GetDeviceSelector() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IBluetoothLEDeviceStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Devices_Enumeration")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Devices_Enumeration`*"]
    pub fn DeviceInformation(&self) -> ::windows::runtime::Result<super::Enumeration::DeviceInformation> {
        let this = &::windows::runtime::Interface::cast::<IBluetoothLEDevice2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::Enumeration::DeviceInformation>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn Appearance(&self) -> ::windows::runtime::Result<BluetoothLEAppearance> {
        let this = &::windows::runtime::Interface::cast::<IBluetoothLEDevice2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothLEAppearance>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn BluetoothAddressType(&self) -> ::windows::runtime::Result<BluetoothAddressType> {
        let this = &::windows::runtime::Interface::cast::<IBluetoothLEDevice2>(self)?;
        unsafe {
            let mut result__: BluetoothAddressType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothAddressType>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn GetDeviceSelectorFromPairingState(pairingstate: bool) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IBluetoothLEDeviceStatics2(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), pairingstate, &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn GetDeviceSelectorFromConnectionStatus(connectionstatus: BluetoothConnectionStatus) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IBluetoothLEDeviceStatics2(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), connectionstatus, &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn GetDeviceSelectorFromDeviceName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(devicename: Param0) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IBluetoothLEDeviceStatics2(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), devicename.into_param().abi(), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn GetDeviceSelectorFromBluetoothAddress(bluetoothaddress: u64) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IBluetoothLEDeviceStatics2(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), bluetoothaddress, &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn GetDeviceSelectorFromBluetoothAddressWithBluetoothAddressType(bluetoothaddress: u64, bluetoothaddresstype: BluetoothAddressType) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IBluetoothLEDeviceStatics2(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), bluetoothaddress, bluetoothaddresstype, &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn GetDeviceSelectorFromAppearance<'a, Param0: ::windows::runtime::IntoParam<'a, BluetoothLEAppearance>>(appearance: Param0) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IBluetoothLEDeviceStatics2(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), appearance.into_param().abi(), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`*"]
    pub fn FromBluetoothAddressWithBluetoothAddressTypeAsync(bluetoothaddress: u64, bluetoothaddresstype: BluetoothAddressType) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<BluetoothLEDevice>> {
        Self::IBluetoothLEDeviceStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), bluetoothaddress, bluetoothaddresstype, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<BluetoothLEDevice>>(result__)
        })
    }
    #[cfg(feature = "Devices_Enumeration")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Devices_Enumeration`*"]
    pub fn DeviceAccessInformation(&self) -> ::windows::runtime::Result<super::Enumeration::DeviceAccessInformation> {
        let this = &::windows::runtime::Interface::cast::<IBluetoothLEDevice3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::Enumeration::DeviceAccessInformation>(result__)
        }
    }
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))]
    #[doc = "*Required features: `Devices_Bluetooth`, `Devices_Enumeration`, `Foundation`*"]
    pub fn RequestAccessAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::Enumeration::DeviceAccessStatus>> {
        let this = &::windows::runtime::Interface::cast::<IBluetoothLEDevice3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::Enumeration::DeviceAccessStatus>>(result__)
        }
    }
    #[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation"))]
    #[doc = "*Required features: `Devices_Bluetooth`, `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn GetGattServicesAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<GenericAttributeProfile::GattDeviceServicesResult>> {
        let this = &::windows::runtime::Interface::cast::<IBluetoothLEDevice3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<GenericAttributeProfile::GattDeviceServicesResult>>(result__)
        }
    }
    #[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation"))]
    #[doc = "*Required features: `Devices_Bluetooth`, `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn GetGattServicesWithCacheModeAsync(&self, cachemode: BluetoothCacheMode) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<GenericAttributeProfile::GattDeviceServicesResult>> {
        let this = &::windows::runtime::Interface::cast::<IBluetoothLEDevice3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), cachemode, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<GenericAttributeProfile::GattDeviceServicesResult>>(result__)
        }
    }
    #[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation"))]
    #[doc = "*Required features: `Devices_Bluetooth`, `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn GetGattServicesForUuidAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, serviceuuid: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<GenericAttributeProfile::GattDeviceServicesResult>> {
        let this = &::windows::runtime::Interface::cast::<IBluetoothLEDevice3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), serviceuuid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<GenericAttributeProfile::GattDeviceServicesResult>>(result__)
        }
    }
    #[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation"))]
    #[doc = "*Required features: `Devices_Bluetooth`, `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn GetGattServicesForUuidWithCacheModeAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, serviceuuid: Param0, cachemode: BluetoothCacheMode) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<GenericAttributeProfile::GattDeviceServicesResult>> {
        let this = &::windows::runtime::Interface::cast::<IBluetoothLEDevice3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), serviceuuid.into_param().abi(), cachemode, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<GenericAttributeProfile::GattDeviceServicesResult>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn BluetoothDeviceId(&self) -> ::windows::runtime::Result<BluetoothDeviceId> {
        let this = &::windows::runtime::Interface::cast::<IBluetoothLEDevice4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothDeviceId>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn WasSecureConnectionUsedForPairing(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IBluetoothLEDevice5>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn GetConnectionParameters(&self) -> ::windows::runtime::Result<BluetoothLEConnectionParameters> {
        let this = &::windows::runtime::Interface::cast::<IBluetoothLEDevice6>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothLEConnectionParameters>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn GetConnectionPhy(&self) -> ::windows::runtime::Result<BluetoothLEConnectionPhy> {
        let this = &::windows::runtime::Interface::cast::<IBluetoothLEDevice6>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothLEConnectionPhy>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn RequestPreferredConnectionParameters<'a, Param0: ::windows::runtime::IntoParam<'a, BluetoothLEPreferredConnectionParameters>>(&self, preferredconnectionparameters: Param0) -> ::windows::runtime::Result<BluetoothLEPreferredConnectionParametersRequest> {
        let this = &::windows::runtime::Interface::cast::<IBluetoothLEDevice6>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), preferredconnectionparameters.into_param().abi(), &mut result__).from_abi::<BluetoothLEPreferredConnectionParametersRequest>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`*"]
    pub fn ConnectionParametersChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<BluetoothLEDevice, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IBluetoothLEDevice6>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`*"]
    pub fn RemoveConnectionParametersChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBluetoothLEDevice6>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`*"]
    pub fn ConnectionPhyChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<BluetoothLEDevice, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IBluetoothLEDevice6>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`*"]
    pub fn RemoveConnectionPhyChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBluetoothLEDevice6>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn IBluetoothLEDeviceStatics<R, F: FnOnce(&IBluetoothLEDeviceStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<BluetoothLEDevice, IBluetoothLEDeviceStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IBluetoothLEDeviceStatics2<R, F: FnOnce(&IBluetoothLEDeviceStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<BluetoothLEDevice, IBluetoothLEDeviceStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BluetoothLEDevice {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.BluetoothLEDevice;{b5ee2f7b-4ad8-4642-ac48-80a0b500e887})");
}
unsafe impl ::windows::runtime::Interface for BluetoothLEDevice {
    type Vtable = IBluetoothLEDevice_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3052285819, 19160, 17986, [172, 72, 128, 160, 181, 0, 232, 135]);
}
impl ::windows::runtime::RuntimeName for BluetoothLEDevice {
    const NAME: &'static str = "Windows.Devices.Bluetooth.BluetoothLEDevice";
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<BluetoothLEDevice> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: BluetoothLEDevice) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&BluetoothLEDevice> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &BluetoothLEDevice) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for BluetoothLEDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &BluetoothLEDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for BluetoothLEDevice {}
unsafe impl ::std::marker::Sync for BluetoothLEDevice {}
#[doc = "*Required features: `Devices_Bluetooth`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct BluetoothLEPreferredConnectionParameters(::windows::runtime::IInspectable);
impl BluetoothLEPreferredConnectionParameters {
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn LinkTimeout(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn ConnectionLatency(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn MinConnectionInterval(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn MaxConnectionInterval(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn Balanced() -> ::windows::runtime::Result<BluetoothLEPreferredConnectionParameters> {
        Self::IBluetoothLEPreferredConnectionParametersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothLEPreferredConnectionParameters>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn ThroughputOptimized() -> ::windows::runtime::Result<BluetoothLEPreferredConnectionParameters> {
        Self::IBluetoothLEPreferredConnectionParametersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothLEPreferredConnectionParameters>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn PowerOptimized() -> ::windows::runtime::Result<BluetoothLEPreferredConnectionParameters> {
        Self::IBluetoothLEPreferredConnectionParametersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothLEPreferredConnectionParameters>(result__)
        })
    }
    pub fn IBluetoothLEPreferredConnectionParametersStatics<R, F: FnOnce(&IBluetoothLEPreferredConnectionParametersStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<BluetoothLEPreferredConnectionParameters, IBluetoothLEPreferredConnectionParametersStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BluetoothLEPreferredConnectionParameters {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.BluetoothLEPreferredConnectionParameters;{f2f44344-7372-5f7b-9b34-29c944f5a715})");
}
unsafe impl ::windows::runtime::Interface for BluetoothLEPreferredConnectionParameters {
    type Vtable = IBluetoothLEPreferredConnectionParameters_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4076094276, 29554, 24443, [155, 52, 41, 201, 68, 245, 167, 21]);
}
impl ::windows::runtime::RuntimeName for BluetoothLEPreferredConnectionParameters {
    const NAME: &'static str = "Windows.Devices.Bluetooth.BluetoothLEPreferredConnectionParameters";
}
unsafe impl ::std::marker::Send for BluetoothLEPreferredConnectionParameters {}
unsafe impl ::std::marker::Sync for BluetoothLEPreferredConnectionParameters {}
#[doc = "*Required features: `Devices_Bluetooth`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct BluetoothLEPreferredConnectionParametersRequest(::windows::runtime::IInspectable);
impl BluetoothLEPreferredConnectionParametersRequest {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<BluetoothLEPreferredConnectionParametersRequestStatus> {
        let this = self;
        unsafe {
            let mut result__: BluetoothLEPreferredConnectionParametersRequestStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothLEPreferredConnectionParametersRequestStatus>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BluetoothLEPreferredConnectionParametersRequest {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.BluetoothLEPreferredConnectionParametersRequest;{8a375276-a528-5266-b661-cce6a5ff9739})");
}
unsafe impl ::windows::runtime::Interface for BluetoothLEPreferredConnectionParametersRequest {
    type Vtable = IBluetoothLEPreferredConnectionParametersRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2318881398, 42280, 21094, [182, 97, 204, 230, 165, 255, 151, 57]);
}
impl ::windows::runtime::RuntimeName for BluetoothLEPreferredConnectionParametersRequest {
    const NAME: &'static str = "Windows.Devices.Bluetooth.BluetoothLEPreferredConnectionParametersRequest";
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<BluetoothLEPreferredConnectionParametersRequest> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: BluetoothLEPreferredConnectionParametersRequest) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&BluetoothLEPreferredConnectionParametersRequest> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &BluetoothLEPreferredConnectionParametersRequest) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for BluetoothLEPreferredConnectionParametersRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &BluetoothLEPreferredConnectionParametersRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for BluetoothLEPreferredConnectionParametersRequest {}
unsafe impl ::std::marker::Sync for BluetoothLEPreferredConnectionParametersRequest {}
#[doc = "*Required features: `Devices_Bluetooth`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct BluetoothLEPreferredConnectionParametersRequestStatus(pub i32);
impl BluetoothLEPreferredConnectionParametersRequestStatus {
    pub const Unspecified: BluetoothLEPreferredConnectionParametersRequestStatus = BluetoothLEPreferredConnectionParametersRequestStatus(0i32);
    pub const Success: BluetoothLEPreferredConnectionParametersRequestStatus = BluetoothLEPreferredConnectionParametersRequestStatus(1i32);
    pub const DeviceNotAvailable: BluetoothLEPreferredConnectionParametersRequestStatus = BluetoothLEPreferredConnectionParametersRequestStatus(2i32);
    pub const AccessDenied: BluetoothLEPreferredConnectionParametersRequestStatus = BluetoothLEPreferredConnectionParametersRequestStatus(3i32);
}
impl ::std::convert::From<i32> for BluetoothLEPreferredConnectionParametersRequestStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for BluetoothLEPreferredConnectionParametersRequestStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for BluetoothLEPreferredConnectionParametersRequestStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.BluetoothLEPreferredConnectionParametersRequestStatus;i4)");
}
impl ::windows::runtime::DefaultType for BluetoothLEPreferredConnectionParametersRequestStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Bluetooth`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct BluetoothMajorClass(pub i32);
impl BluetoothMajorClass {
    pub const Miscellaneous: BluetoothMajorClass = BluetoothMajorClass(0i32);
    pub const Computer: BluetoothMajorClass = BluetoothMajorClass(1i32);
    pub const Phone: BluetoothMajorClass = BluetoothMajorClass(2i32);
    pub const NetworkAccessPoint: BluetoothMajorClass = BluetoothMajorClass(3i32);
    pub const AudioVideo: BluetoothMajorClass = BluetoothMajorClass(4i32);
    pub const Peripheral: BluetoothMajorClass = BluetoothMajorClass(5i32);
    pub const Imaging: BluetoothMajorClass = BluetoothMajorClass(6i32);
    pub const Wearable: BluetoothMajorClass = BluetoothMajorClass(7i32);
    pub const Toy: BluetoothMajorClass = BluetoothMajorClass(8i32);
    pub const Health: BluetoothMajorClass = BluetoothMajorClass(9i32);
}
impl ::std::convert::From<i32> for BluetoothMajorClass {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for BluetoothMajorClass {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for BluetoothMajorClass {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.BluetoothMajorClass;i4)");
}
impl ::windows::runtime::DefaultType for BluetoothMajorClass {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Bluetooth`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct BluetoothMinorClass(pub i32);
impl BluetoothMinorClass {
    pub const Uncategorized: BluetoothMinorClass = BluetoothMinorClass(0i32);
    pub const ComputerDesktop: BluetoothMinorClass = BluetoothMinorClass(1i32);
    pub const ComputerServer: BluetoothMinorClass = BluetoothMinorClass(2i32);
    pub const ComputerLaptop: BluetoothMinorClass = BluetoothMinorClass(3i32);
    pub const ComputerHandheld: BluetoothMinorClass = BluetoothMinorClass(4i32);
    pub const ComputerPalmSize: BluetoothMinorClass = BluetoothMinorClass(5i32);
    pub const ComputerWearable: BluetoothMinorClass = BluetoothMinorClass(6i32);
    pub const ComputerTablet: BluetoothMinorClass = BluetoothMinorClass(7i32);
    pub const PhoneCellular: BluetoothMinorClass = BluetoothMinorClass(1i32);
    pub const PhoneCordless: BluetoothMinorClass = BluetoothMinorClass(2i32);
    pub const PhoneSmartPhone: BluetoothMinorClass = BluetoothMinorClass(3i32);
    pub const PhoneWired: BluetoothMinorClass = BluetoothMinorClass(4i32);
    pub const PhoneIsdn: BluetoothMinorClass = BluetoothMinorClass(5i32);
    pub const NetworkFullyAvailable: BluetoothMinorClass = BluetoothMinorClass(0i32);
    pub const NetworkUsed01To17Percent: BluetoothMinorClass = BluetoothMinorClass(8i32);
    pub const NetworkUsed17To33Percent: BluetoothMinorClass = BluetoothMinorClass(16i32);
    pub const NetworkUsed33To50Percent: BluetoothMinorClass = BluetoothMinorClass(24i32);
    pub const NetworkUsed50To67Percent: BluetoothMinorClass = BluetoothMinorClass(32i32);
    pub const NetworkUsed67To83Percent: BluetoothMinorClass = BluetoothMinorClass(40i32);
    pub const NetworkUsed83To99Percent: BluetoothMinorClass = BluetoothMinorClass(48i32);
    pub const NetworkNoServiceAvailable: BluetoothMinorClass = BluetoothMinorClass(56i32);
    pub const AudioVideoWearableHeadset: BluetoothMinorClass = BluetoothMinorClass(1i32);
    pub const AudioVideoHandsFree: BluetoothMinorClass = BluetoothMinorClass(2i32);
    pub const AudioVideoMicrophone: BluetoothMinorClass = BluetoothMinorClass(4i32);
    pub const AudioVideoLoudspeaker: BluetoothMinorClass = BluetoothMinorClass(5i32);
    pub const AudioVideoHeadphones: BluetoothMinorClass = BluetoothMinorClass(6i32);
    pub const AudioVideoPortableAudio: BluetoothMinorClass = BluetoothMinorClass(7i32);
    pub const AudioVideoCarAudio: BluetoothMinorClass = BluetoothMinorClass(8i32);
    pub const AudioVideoSetTopBox: BluetoothMinorClass = BluetoothMinorClass(9i32);
    pub const AudioVideoHifiAudioDevice: BluetoothMinorClass = BluetoothMinorClass(10i32);
    pub const AudioVideoVcr: BluetoothMinorClass = BluetoothMinorClass(11i32);
    pub const AudioVideoVideoCamera: BluetoothMinorClass = BluetoothMinorClass(12i32);
    pub const AudioVideoCamcorder: BluetoothMinorClass = BluetoothMinorClass(13i32);
    pub const AudioVideoVideoMonitor: BluetoothMinorClass = BluetoothMinorClass(14i32);
    pub const AudioVideoVideoDisplayAndLoudspeaker: BluetoothMinorClass = BluetoothMinorClass(15i32);
    pub const AudioVideoVideoConferencing: BluetoothMinorClass = BluetoothMinorClass(16i32);
    pub const AudioVideoGamingOrToy: BluetoothMinorClass = BluetoothMinorClass(18i32);
    pub const PeripheralJoystick: BluetoothMinorClass = BluetoothMinorClass(1i32);
    pub const PeripheralGamepad: BluetoothMinorClass = BluetoothMinorClass(2i32);
    pub const PeripheralRemoteControl: BluetoothMinorClass = BluetoothMinorClass(3i32);
    pub const PeripheralSensing: BluetoothMinorClass = BluetoothMinorClass(4i32);
    pub const PeripheralDigitizerTablet: BluetoothMinorClass = BluetoothMinorClass(5i32);
    pub const PeripheralCardReader: BluetoothMinorClass = BluetoothMinorClass(6i32);
    pub const PeripheralDigitalPen: BluetoothMinorClass = BluetoothMinorClass(7i32);
    pub const PeripheralHandheldScanner: BluetoothMinorClass = BluetoothMinorClass(8i32);
    pub const PeripheralHandheldGesture: BluetoothMinorClass = BluetoothMinorClass(9i32);
    pub const WearableWristwatch: BluetoothMinorClass = BluetoothMinorClass(1i32);
    pub const WearablePager: BluetoothMinorClass = BluetoothMinorClass(2i32);
    pub const WearableJacket: BluetoothMinorClass = BluetoothMinorClass(3i32);
    pub const WearableHelmet: BluetoothMinorClass = BluetoothMinorClass(4i32);
    pub const WearableGlasses: BluetoothMinorClass = BluetoothMinorClass(5i32);
    pub const ToyRobot: BluetoothMinorClass = BluetoothMinorClass(1i32);
    pub const ToyVehicle: BluetoothMinorClass = BluetoothMinorClass(2i32);
    pub const ToyDoll: BluetoothMinorClass = BluetoothMinorClass(3i32);
    pub const ToyController: BluetoothMinorClass = BluetoothMinorClass(4i32);
    pub const ToyGame: BluetoothMinorClass = BluetoothMinorClass(5i32);
    pub const HealthBloodPressureMonitor: BluetoothMinorClass = BluetoothMinorClass(1i32);
    pub const HealthThermometer: BluetoothMinorClass = BluetoothMinorClass(2i32);
    pub const HealthWeighingScale: BluetoothMinorClass = BluetoothMinorClass(3i32);
    pub const HealthGlucoseMeter: BluetoothMinorClass = BluetoothMinorClass(4i32);
    pub const HealthPulseOximeter: BluetoothMinorClass = BluetoothMinorClass(5i32);
    pub const HealthHeartRateMonitor: BluetoothMinorClass = BluetoothMinorClass(6i32);
    pub const HealthHealthDataDisplay: BluetoothMinorClass = BluetoothMinorClass(7i32);
    pub const HealthStepCounter: BluetoothMinorClass = BluetoothMinorClass(8i32);
    pub const HealthBodyCompositionAnalyzer: BluetoothMinorClass = BluetoothMinorClass(9i32);
    pub const HealthPeakFlowMonitor: BluetoothMinorClass = BluetoothMinorClass(10i32);
    pub const HealthMedicationMonitor: BluetoothMinorClass = BluetoothMinorClass(11i32);
    pub const HealthKneeProsthesis: BluetoothMinorClass = BluetoothMinorClass(12i32);
    pub const HealthAnkleProsthesis: BluetoothMinorClass = BluetoothMinorClass(13i32);
    pub const HealthGenericHealthManager: BluetoothMinorClass = BluetoothMinorClass(14i32);
    pub const HealthPersonalMobilityDevice: BluetoothMinorClass = BluetoothMinorClass(15i32);
}
impl ::std::convert::From<i32> for BluetoothMinorClass {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for BluetoothMinorClass {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for BluetoothMinorClass {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.BluetoothMinorClass;i4)");
}
impl ::windows::runtime::DefaultType for BluetoothMinorClass {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Bluetooth`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct BluetoothServiceCapabilities(pub u32);
impl BluetoothServiceCapabilities {
    pub const None: BluetoothServiceCapabilities = BluetoothServiceCapabilities(0u32);
    pub const LimitedDiscoverableMode: BluetoothServiceCapabilities = BluetoothServiceCapabilities(1u32);
    pub const PositioningService: BluetoothServiceCapabilities = BluetoothServiceCapabilities(8u32);
    pub const NetworkingService: BluetoothServiceCapabilities = BluetoothServiceCapabilities(16u32);
    pub const RenderingService: BluetoothServiceCapabilities = BluetoothServiceCapabilities(32u32);
    pub const CapturingService: BluetoothServiceCapabilities = BluetoothServiceCapabilities(64u32);
    pub const ObjectTransferService: BluetoothServiceCapabilities = BluetoothServiceCapabilities(128u32);
    pub const AudioService: BluetoothServiceCapabilities = BluetoothServiceCapabilities(256u32);
    pub const TelephoneService: BluetoothServiceCapabilities = BluetoothServiceCapabilities(512u32);
    pub const InformationService: BluetoothServiceCapabilities = BluetoothServiceCapabilities(1024u32);
}
impl ::std::convert::From<u32> for BluetoothServiceCapabilities {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for BluetoothServiceCapabilities {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for BluetoothServiceCapabilities {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.BluetoothServiceCapabilities;u4)");
}
impl ::windows::runtime::DefaultType for BluetoothServiceCapabilities {
    type DefaultType = Self;
}
impl ::std::ops::BitOr for BluetoothServiceCapabilities {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for BluetoothServiceCapabilities {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for BluetoothServiceCapabilities {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for BluetoothServiceCapabilities {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for BluetoothServiceCapabilities {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Devices_Bluetooth`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct BluetoothSignalStrengthFilter(::windows::runtime::IInspectable);
impl BluetoothSignalStrengthFilter {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<BluetoothSignalStrengthFilter, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`*"]
    pub fn InRangeThresholdInDBm(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<i16>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i16>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`*"]
    pub fn SetInRangeThresholdInDBm<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::IReference<i16>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`*"]
    pub fn OutOfRangeThresholdInDBm(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<i16>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i16>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`*"]
    pub fn SetOutOfRangeThresholdInDBm<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::IReference<i16>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`*"]
    pub fn OutOfRangeTimeout(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`*"]
    pub fn SetOutOfRangeTimeout<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`*"]
    pub fn SamplingInterval(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`*"]
    pub fn SetSamplingInterval<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BluetoothSignalStrengthFilter {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.BluetoothSignalStrengthFilter;{df7b7391-6bb5-4cfe-90b1-5d7324edcf7f})");
}
unsafe impl ::windows::runtime::Interface for BluetoothSignalStrengthFilter {
    type Vtable = IBluetoothSignalStrengthFilter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3749409681, 27573, 19710, [144, 177, 93, 115, 36, 237, 207, 127]);
}
impl ::windows::runtime::RuntimeName for BluetoothSignalStrengthFilter {
    const NAME: &'static str = "Windows.Devices.Bluetooth.BluetoothSignalStrengthFilter";
}
unsafe impl ::std::marker::Send for BluetoothSignalStrengthFilter {}
unsafe impl ::std::marker::Sync for BluetoothSignalStrengthFilter {}
#[doc = "*Required features: `Devices_Bluetooth`*"]
pub struct BluetoothUuidHelper {}
impl BluetoothUuidHelper {
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn FromShortId(shortid: u32) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        Self::IBluetoothUuidHelperStatics(|this| unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), shortid, &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`*"]
    pub fn TryGetShortId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(uuid: Param0) -> ::windows::runtime::Result<super::super::Foundation::IReference<u32>> {
        Self::IBluetoothUuidHelperStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), uuid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IReference<u32>>(result__)
        })
    }
    pub fn IBluetoothUuidHelperStatics<R, F: FnOnce(&IBluetoothUuidHelperStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<BluetoothUuidHelper, IBluetoothUuidHelperStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for BluetoothUuidHelper {
    const NAME: &'static str = "Windows.Devices.Bluetooth.BluetoothUuidHelper";
}
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothAdapter(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBluetoothAdapter {
    type Vtable = IBluetoothAdapter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2037706828, 24442, 18996, [146, 37, 168, 85, 248, 75, 26, 139]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothAdapter_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Devices_Radios", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Devices_Radios", feature = "Foundation")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothAdapter2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBluetoothAdapter2 {
    type Vtable = IBluetoothAdapter2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2895433420, 9429, 16819, [145, 109, 16, 151, 197, 11, 16, 43]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothAdapter2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothAdapter3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBluetoothAdapter3 {
    type Vtable = IBluetoothAdapter3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2407933152, 52137, 21009, [159, 137, 58, 172, 98, 180, 198, 184]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothAdapter3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothAdapterStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBluetoothAdapterStatics {
    type Vtable = IBluetoothAdapterStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2332228458, 44108, 18241, [134, 97, 142, 171, 125, 23, 234, 159]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothAdapterStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deviceid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothClassOfDevice(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBluetoothClassOfDevice {
    type Vtable = IBluetoothClassOfDevice_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3594527358, 55255, 18017, [148, 84, 101, 3, 156, 161, 122, 43]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothClassOfDevice_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut BluetoothMajorClass) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut BluetoothMinorClass) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut BluetoothServiceCapabilities) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothClassOfDeviceStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBluetoothClassOfDeviceStatics {
    type Vtable = IBluetoothClassOfDeviceStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3831575997, 4002, 16748, [145, 180, 193, 228, 140, 160, 97, 193]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothClassOfDeviceStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rawvalue: u32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, majorclass: BluetoothMajorClass, minorclass: BluetoothMinorClass, servicecapabilities: BluetoothServiceCapabilities, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothDevice(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBluetoothDevice {
    type Vtable = IBluetoothDevice_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(590721366, 37074, 18948, [174, 245, 14, 32, 185, 230, 183, 7]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothDevice_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Networking")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Networking"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Foundation_Collections")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut BluetoothConnectionStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothDevice2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBluetoothDevice2 {
    type Vtable = IBluetoothDevice2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(20183380, 45398, 19920, [177, 245, 193, 27, 195, 26, 81, 99]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothDevice2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Enumeration")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothDevice3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBluetoothDevice3 {
    type Vtable = IBluetoothDevice3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1476392843, 25882, 17492, [185, 15, 235, 33, 239, 11, 13, 113]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothDevice3_abi(
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
    #[cfg(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Foundation")))] usize,
    #[cfg(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cachemode: BluetoothCacheMode, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Foundation")))] usize,
    #[cfg(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, serviceid: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Foundation")))] usize,
    #[cfg(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, serviceid: ::windows::runtime::RawPtr, cachemode: BluetoothCacheMode, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Foundation")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothDevice4(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBluetoothDevice4 {
    type Vtable = IBluetoothDevice4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2172400813, 3740, 17074, [168, 220, 62, 128, 148, 148, 13, 18]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothDevice4_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothDevice5(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBluetoothDevice5 {
    type Vtable = IBluetoothDevice5_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3051402117, 24197, 17753, [161, 13, 28, 114, 129, 55, 159, 150]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothDevice5_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothDeviceId(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBluetoothDeviceId {
    type Vtable = IBluetoothDeviceId_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3245951407, 22465, 17986, [188, 206, 230, 192, 107, 32, 174, 118]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothDeviceId_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothDeviceIdStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBluetoothDeviceIdStatics {
    type Vtable = IBluetoothDeviceIdStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2810728039, 16123, 20273, [187, 194, 129, 14, 9, 151, 116, 4]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothDeviceIdStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deviceid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothDeviceStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBluetoothDeviceStatics {
    type Vtable = IBluetoothDeviceStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(160554833, 22491, 18213, [187, 215, 132, 246, 67, 39, 236, 44]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothDeviceStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deviceid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Networking"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hostname: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Networking")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, address: u64, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothDeviceStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBluetoothDeviceStatics2 {
    type Vtable = IBluetoothDeviceStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3265170991, 19988, 17527, [170, 27, 184, 180, 126, 91, 126, 206]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothDeviceStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pairingstate: bool, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, connectionstatus: BluetoothConnectionStatus, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, devicename: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bluetoothaddress: u64, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, classofdevice: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEAppearance(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBluetoothLEAppearance {
    type Vtable = IBluetoothLEAppearance_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1562409458, 26280, 16984, [152, 94, 2, 180, 217, 80, 159, 24]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAppearance_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEAppearanceCategoriesStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBluetoothLEAppearanceCategoriesStatics {
    type Vtable = IBluetoothLEAppearanceCategoriesStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1833784574, 1130, 16773, [170, 182, 130, 76, 240, 97, 8, 97]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAppearanceCategoriesStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEAppearanceStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBluetoothLEAppearanceStatics {
    type Vtable = IBluetoothLEAppearanceStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2710814919, 17668, 20298, [155, 165, 205, 16, 84, 229, 224, 101]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAppearanceStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rawvalue: u16, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, appearancecategory: u16, appearancesubcategory: u16, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEAppearanceSubcategoriesStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBluetoothLEAppearanceSubcategoriesStatics {
    type Vtable = IBluetoothLEAppearanceSubcategoriesStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3850085894, 8516, 16730, [131, 18, 113, 204, 242, 145, 248, 209]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAppearanceSubcategoriesStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEConnectionParameters(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBluetoothLEConnectionParameters {
    type Vtable = IBluetoothLEConnectionParameters_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(868943729, 36265, 20623, [163, 102, 28, 163, 136, 201, 41, 171]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEConnectionParameters_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEConnectionPhy(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBluetoothLEConnectionPhy {
    type Vtable = IBluetoothLEConnectionPhy_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2015256136, 25118, 23166, [139, 230, 27, 149, 97, 255, 99, 201]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEConnectionPhy_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEConnectionPhyInfo(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBluetoothLEConnectionPhyInfo {
    type Vtable = IBluetoothLEConnectionPhyInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2584742877, 24622, 23591, [161, 174, 178, 48, 1, 90, 99, 148]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEConnectionPhyInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEDevice(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBluetoothLEDevice {
    type Vtable = IBluetoothLEDevice_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3052285819, 19160, 17986, [172, 72, 128, 160, 181, 0, 232, 135]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEDevice_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation_Collections")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut BluetoothConnectionStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, serviceuuid: ::windows::runtime::GUID, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_GenericAttributeProfile"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEDevice2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBluetoothLEDevice2 {
    type Vtable = IBluetoothLEDevice2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(653288115, 31470, 19761, [186, 186, 177, 185, 119, 95, 89, 22]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEDevice2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Enumeration")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut BluetoothAddressType) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEDevice3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBluetoothLEDevice3 {
    type Vtable = IBluetoothLEDevice3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2934563987, 17580, 16604, [175, 51, 178, 193, 60, 1, 202, 70]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEDevice3_abi(
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
    #[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation")))] usize,
    #[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cachemode: BluetoothCacheMode, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation")))] usize,
    #[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, serviceuuid: ::windows::runtime::GUID, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation")))] usize,
    #[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, serviceuuid: ::windows::runtime::GUID, cachemode: BluetoothCacheMode, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEDevice4(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBluetoothLEDevice4 {
    type Vtable = IBluetoothLEDevice4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(727732273, 8776, 19247, [172, 240, 124, 238, 54, 252, 88, 112]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEDevice4_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEDevice5(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBluetoothLEDevice5 {
    type Vtable = IBluetoothLEDevice5_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2640974432, 21127, 17806, [149, 186, 23, 200, 183, 187, 50, 110]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEDevice5_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEDevice6(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBluetoothLEDevice6 {
    type Vtable = IBluetoothLEDevice6_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3396440303, 3246, 22332, [161, 202, 225, 252, 91, 252, 57, 226]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEDevice6_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, preferredconnectionparameters: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEDeviceStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBluetoothLEDeviceStatics {
    type Vtable = IBluetoothLEDeviceStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3369015833, 61622, 19440, [134, 137, 65, 48, 61, 226, 217, 244]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEDeviceStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deviceid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bluetoothaddress: u64, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEDeviceStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBluetoothLEDeviceStatics2 {
    type Vtable = IBluetoothLEDeviceStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1595064427, 15276, 17384, [173, 22, 86, 50, 113, 189, 65, 194]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEDeviceStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pairingstate: bool, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, connectionstatus: BluetoothConnectionStatus, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, devicename: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bluetoothaddress: u64, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bluetoothaddress: u64, bluetoothaddresstype: BluetoothAddressType, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, appearance: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bluetoothaddress: u64, bluetoothaddresstype: BluetoothAddressType, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEPreferredConnectionParameters(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBluetoothLEPreferredConnectionParameters {
    type Vtable = IBluetoothLEPreferredConnectionParameters_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4076094276, 29554, 24443, [155, 52, 41, 201, 68, 245, 167, 21]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEPreferredConnectionParameters_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEPreferredConnectionParametersRequest(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBluetoothLEPreferredConnectionParametersRequest {
    type Vtable = IBluetoothLEPreferredConnectionParametersRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2318881398, 42280, 21094, [182, 97, 204, 230, 165, 255, 151, 57]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEPreferredConnectionParametersRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut BluetoothLEPreferredConnectionParametersRequestStatus) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEPreferredConnectionParametersStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBluetoothLEPreferredConnectionParametersStatics {
    type Vtable = IBluetoothLEPreferredConnectionParametersStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(238980828, 10065, 21930, [168, 56, 143, 174, 238, 129, 141, 114]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEPreferredConnectionParametersStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothSignalStrengthFilter(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBluetoothSignalStrengthFilter {
    type Vtable = IBluetoothSignalStrengthFilter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3749409681, 27573, 19710, [144, 177, 93, 115, 36, 237, 207, 127]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothSignalStrengthFilter_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothUuidHelperStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBluetoothUuidHelperStatics {
    type Vtable = IBluetoothUuidHelperStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(400493784, 53108, 19233, [175, 230, 245, 122, 17, 188, 222, 160]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothUuidHelperStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shortid: u32, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uuid: ::windows::runtime::GUID, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
