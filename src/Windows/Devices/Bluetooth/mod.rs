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
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BluetoothAdapter(pub ::windows::core::IInspectable);
impl BluetoothAdapter {
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn BluetoothAddress(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn IsClassicSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn IsLowEnergySupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn IsPeripheralRoleSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn IsCentralRoleSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn IsAdvertisementOffloadSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(all(feature = "Devices_Radios", feature = "Foundation"))]
    #[doc = "*Required features: `Devices_Bluetooth`, `Devices_Radios`, `Foundation`*"]
    pub fn GetRadioAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::Radios::Radio>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::Radios::Radio>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IBluetoothAdapterStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`*"]
    pub fn FromIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BluetoothAdapter>> {
        Self::IBluetoothAdapterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<BluetoothAdapter>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`*"]
    pub fn GetDefaultAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BluetoothAdapter>> {
        Self::IBluetoothAdapterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<BluetoothAdapter>>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn AreClassicSecureConnectionsSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IBluetoothAdapter2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn AreLowEnergySecureConnectionsSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IBluetoothAdapter2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn IsExtendedAdvertisingSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IBluetoothAdapter3>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn MaxAdvertisementDataLength(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IBluetoothAdapter3>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn IBluetoothAdapterStatics<R, F: FnOnce(&IBluetoothAdapterStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BluetoothAdapter, IBluetoothAdapterStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for BluetoothAdapter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.BluetoothAdapter;{7974f04c-5f7a-4a34-9225-a855f84b1a8b})");
}
unsafe impl ::windows::core::Interface for BluetoothAdapter {
    type Vtable = IBluetoothAdapter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7974f04c_5f7a_4a34_9225_a855f84b1a8b);
}
impl ::windows::core::RuntimeName for BluetoothAdapter {
    const NAME: &'static str = "Windows.Devices.Bluetooth.BluetoothAdapter";
}
impl ::core::convert::From<BluetoothAdapter> for ::windows::core::IUnknown {
    fn from(value: BluetoothAdapter) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BluetoothAdapter> for ::windows::core::IUnknown {
    fn from(value: &BluetoothAdapter) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BluetoothAdapter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BluetoothAdapter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BluetoothAdapter> for ::windows::core::IInspectable {
    fn from(value: BluetoothAdapter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BluetoothAdapter> for ::windows::core::IInspectable {
    fn from(value: &BluetoothAdapter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BluetoothAdapter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BluetoothAdapter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for BluetoothAdapter {}
unsafe impl ::core::marker::Sync for BluetoothAdapter {}
#[doc = "*Required features: `Devices_Bluetooth`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct BluetoothAddressType(pub i32);
impl BluetoothAddressType {
    pub const Public: BluetoothAddressType = BluetoothAddressType(0i32);
    pub const Random: BluetoothAddressType = BluetoothAddressType(1i32);
    pub const Unspecified: BluetoothAddressType = BluetoothAddressType(2i32);
}
impl ::core::convert::From<i32> for BluetoothAddressType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for BluetoothAddressType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for BluetoothAddressType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.BluetoothAddressType;i4)");
}
impl ::windows::core::DefaultType for BluetoothAddressType {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Bluetooth`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct BluetoothCacheMode(pub i32);
impl BluetoothCacheMode {
    pub const Cached: BluetoothCacheMode = BluetoothCacheMode(0i32);
    pub const Uncached: BluetoothCacheMode = BluetoothCacheMode(1i32);
}
impl ::core::convert::From<i32> for BluetoothCacheMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for BluetoothCacheMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for BluetoothCacheMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.BluetoothCacheMode;i4)");
}
impl ::windows::core::DefaultType for BluetoothCacheMode {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Bluetooth`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BluetoothClassOfDevice(pub ::windows::core::IInspectable);
impl BluetoothClassOfDevice {
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn RawValue(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn MajorClass(&self) -> ::windows::core::Result<BluetoothMajorClass> {
        let this = self;
        unsafe {
            let mut result__: BluetoothMajorClass = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothMajorClass>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn MinorClass(&self) -> ::windows::core::Result<BluetoothMinorClass> {
        let this = self;
        unsafe {
            let mut result__: BluetoothMinorClass = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothMinorClass>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn ServiceCapabilities(&self) -> ::windows::core::Result<BluetoothServiceCapabilities> {
        let this = self;
        unsafe {
            let mut result__: BluetoothServiceCapabilities = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothServiceCapabilities>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn FromRawValue(rawvalue: u32) -> ::windows::core::Result<BluetoothClassOfDevice> {
        Self::IBluetoothClassOfDeviceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), rawvalue, &mut result__).from_abi::<BluetoothClassOfDevice>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn FromParts(majorclass: BluetoothMajorClass, minorclass: BluetoothMinorClass, servicecapabilities: BluetoothServiceCapabilities) -> ::windows::core::Result<BluetoothClassOfDevice> {
        Self::IBluetoothClassOfDeviceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), majorclass, minorclass, servicecapabilities, &mut result__).from_abi::<BluetoothClassOfDevice>(result__)
        })
    }
    pub fn IBluetoothClassOfDeviceStatics<R, F: FnOnce(&IBluetoothClassOfDeviceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BluetoothClassOfDevice, IBluetoothClassOfDeviceStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for BluetoothClassOfDevice {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.BluetoothClassOfDevice;{d640227e-d7d7-4661-9454-65039ca17a2b})");
}
unsafe impl ::windows::core::Interface for BluetoothClassOfDevice {
    type Vtable = IBluetoothClassOfDevice_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd640227e_d7d7_4661_9454_65039ca17a2b);
}
impl ::windows::core::RuntimeName for BluetoothClassOfDevice {
    const NAME: &'static str = "Windows.Devices.Bluetooth.BluetoothClassOfDevice";
}
impl ::core::convert::From<BluetoothClassOfDevice> for ::windows::core::IUnknown {
    fn from(value: BluetoothClassOfDevice) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BluetoothClassOfDevice> for ::windows::core::IUnknown {
    fn from(value: &BluetoothClassOfDevice) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BluetoothClassOfDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BluetoothClassOfDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BluetoothClassOfDevice> for ::windows::core::IInspectable {
    fn from(value: BluetoothClassOfDevice) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BluetoothClassOfDevice> for ::windows::core::IInspectable {
    fn from(value: &BluetoothClassOfDevice) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BluetoothClassOfDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BluetoothClassOfDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for BluetoothClassOfDevice {}
unsafe impl ::core::marker::Sync for BluetoothClassOfDevice {}
#[doc = "*Required features: `Devices_Bluetooth`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct BluetoothConnectionStatus(pub i32);
impl BluetoothConnectionStatus {
    pub const Disconnected: BluetoothConnectionStatus = BluetoothConnectionStatus(0i32);
    pub const Connected: BluetoothConnectionStatus = BluetoothConnectionStatus(1i32);
}
impl ::core::convert::From<i32> for BluetoothConnectionStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for BluetoothConnectionStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for BluetoothConnectionStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.BluetoothConnectionStatus;i4)");
}
impl ::windows::core::DefaultType for BluetoothConnectionStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Bluetooth`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BluetoothDevice(pub ::windows::core::IInspectable);
impl BluetoothDevice {
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Networking")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Networking`*"]
    pub fn HostName(&self) -> ::windows::core::Result<super::super::Networking::HostName> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Networking::HostName>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn ClassOfDevice(&self) -> ::windows::core::Result<BluetoothClassOfDevice> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothClassOfDevice>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation_Collections`, `Storage_Streams`*"]
    pub fn SdpRecords(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Storage::Streams::IBuffer>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Storage::Streams::IBuffer>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_Bluetooth`, `Devices_Bluetooth_Rfcomm`, `Foundation_Collections`*"]
    pub fn RfcommServices(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<Rfcomm::RfcommDeviceService>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<Rfcomm::RfcommDeviceService>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn ConnectionStatus(&self) -> ::windows::core::Result<BluetoothConnectionStatus> {
        let this = self;
        unsafe {
            let mut result__: BluetoothConnectionStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothConnectionStatus>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn BluetoothAddress(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`*"]
    pub fn NameChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<BluetoothDevice, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`*"]
    pub fn RemoveNameChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`*"]
    pub fn SdpRecordsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<BluetoothDevice, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`*"]
    pub fn RemoveSdpRecordsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`*"]
    pub fn ConnectionStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<BluetoothDevice, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`*"]
    pub fn RemoveConnectionStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`*"]
    pub fn FromIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BluetoothDevice>> {
        Self::IBluetoothDeviceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<BluetoothDevice>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Networking"))]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`, `Networking`*"]
    pub fn FromHostNameAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Networking::HostName>>(hostname: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BluetoothDevice>> {
        Self::IBluetoothDeviceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), hostname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<BluetoothDevice>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`*"]
    pub fn FromBluetoothAddressAsync(address: u64) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BluetoothDevice>> {
        Self::IBluetoothDeviceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), address, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<BluetoothDevice>>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IBluetoothDeviceStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Devices_Enumeration")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Devices_Enumeration`*"]
    pub fn DeviceInformation(&self) -> ::windows::core::Result<super::Enumeration::DeviceInformation> {
        let this = &::windows::core::Interface::cast::<IBluetoothDevice2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Enumeration::DeviceInformation>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn GetDeviceSelectorFromPairingState(pairingstate: bool) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IBluetoothDeviceStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), pairingstate, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn GetDeviceSelectorFromConnectionStatus(connectionstatus: BluetoothConnectionStatus) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IBluetoothDeviceStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), connectionstatus, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn GetDeviceSelectorFromDeviceName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(devicename: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IBluetoothDeviceStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), devicename.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn GetDeviceSelectorFromBluetoothAddress(bluetoothaddress: u64) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IBluetoothDeviceStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), bluetoothaddress, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn GetDeviceSelectorFromClassOfDevice<'a, Param0: ::windows::core::IntoParam<'a, BluetoothClassOfDevice>>(classofdevice: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IBluetoothDeviceStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), classofdevice.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Devices_Enumeration")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Devices_Enumeration`*"]
    pub fn DeviceAccessInformation(&self) -> ::windows::core::Result<super::Enumeration::DeviceAccessInformation> {
        let this = &::windows::core::Interface::cast::<IBluetoothDevice3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Enumeration::DeviceAccessInformation>(result__)
        }
    }
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))]
    #[doc = "*Required features: `Devices_Bluetooth`, `Devices_Enumeration`, `Foundation`*"]
    pub fn RequestAccessAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::Enumeration::DeviceAccessStatus>> {
        let this = &::windows::core::Interface::cast::<IBluetoothDevice3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::Enumeration::DeviceAccessStatus>>(result__)
        }
    }
    #[cfg(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Foundation"))]
    #[doc = "*Required features: `Devices_Bluetooth`, `Devices_Bluetooth_Rfcomm`, `Foundation`*"]
    pub fn GetRfcommServicesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Rfcomm::RfcommDeviceServicesResult>> {
        let this = &::windows::core::Interface::cast::<IBluetoothDevice3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<Rfcomm::RfcommDeviceServicesResult>>(result__)
        }
    }
    #[cfg(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Foundation"))]
    #[doc = "*Required features: `Devices_Bluetooth`, `Devices_Bluetooth_Rfcomm`, `Foundation`*"]
    pub fn GetRfcommServicesWithCacheModeAsync(&self, cachemode: BluetoothCacheMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Rfcomm::RfcommDeviceServicesResult>> {
        let this = &::windows::core::Interface::cast::<IBluetoothDevice3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), cachemode, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<Rfcomm::RfcommDeviceServicesResult>>(result__)
        }
    }
    #[cfg(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Foundation"))]
    #[doc = "*Required features: `Devices_Bluetooth`, `Devices_Bluetooth_Rfcomm`, `Foundation`*"]
    pub fn GetRfcommServicesForIdAsync<'a, Param0: ::windows::core::IntoParam<'a, Rfcomm::RfcommServiceId>>(&self, serviceid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Rfcomm::RfcommDeviceServicesResult>> {
        let this = &::windows::core::Interface::cast::<IBluetoothDevice3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), serviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<Rfcomm::RfcommDeviceServicesResult>>(result__)
        }
    }
    #[cfg(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Foundation"))]
    #[doc = "*Required features: `Devices_Bluetooth`, `Devices_Bluetooth_Rfcomm`, `Foundation`*"]
    pub fn GetRfcommServicesForIdWithCacheModeAsync<'a, Param0: ::windows::core::IntoParam<'a, Rfcomm::RfcommServiceId>>(&self, serviceid: Param0, cachemode: BluetoothCacheMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Rfcomm::RfcommDeviceServicesResult>> {
        let this = &::windows::core::Interface::cast::<IBluetoothDevice3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), serviceid.into_param().abi(), cachemode, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<Rfcomm::RfcommDeviceServicesResult>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn BluetoothDeviceId(&self) -> ::windows::core::Result<BluetoothDeviceId> {
        let this = &::windows::core::Interface::cast::<IBluetoothDevice4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothDeviceId>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn WasSecureConnectionUsedForPairing(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IBluetoothDevice5>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IBluetoothDeviceStatics<R, F: FnOnce(&IBluetoothDeviceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BluetoothDevice, IBluetoothDeviceStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IBluetoothDeviceStatics2<R, F: FnOnce(&IBluetoothDeviceStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BluetoothDevice, IBluetoothDeviceStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for BluetoothDevice {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.BluetoothDevice;{2335b156-90d2-4a04-aef5-0e20b9e6b707})");
}
unsafe impl ::windows::core::Interface for BluetoothDevice {
    type Vtable = IBluetoothDevice_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2335b156_90d2_4a04_aef5_0e20b9e6b707);
}
impl ::windows::core::RuntimeName for BluetoothDevice {
    const NAME: &'static str = "Windows.Devices.Bluetooth.BluetoothDevice";
}
impl ::core::convert::From<BluetoothDevice> for ::windows::core::IUnknown {
    fn from(value: BluetoothDevice) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BluetoothDevice> for ::windows::core::IUnknown {
    fn from(value: &BluetoothDevice) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BluetoothDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BluetoothDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BluetoothDevice> for ::windows::core::IInspectable {
    fn from(value: BluetoothDevice) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BluetoothDevice> for ::windows::core::IInspectable {
    fn from(value: &BluetoothDevice) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BluetoothDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BluetoothDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<BluetoothDevice> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: BluetoothDevice) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&BluetoothDevice> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &BluetoothDevice) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for BluetoothDevice {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &BluetoothDevice {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for BluetoothDevice {}
unsafe impl ::core::marker::Sync for BluetoothDevice {}
#[doc = "*Required features: `Devices_Bluetooth`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BluetoothDeviceId(pub ::windows::core::IInspectable);
impl BluetoothDeviceId {
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn IsClassicDevice(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn IsLowEnergyDevice(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn FromId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0) -> ::windows::core::Result<BluetoothDeviceId> {
        Self::IBluetoothDeviceIdStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<BluetoothDeviceId>(result__)
        })
    }
    pub fn IBluetoothDeviceIdStatics<R, F: FnOnce(&IBluetoothDeviceIdStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BluetoothDeviceId, IBluetoothDeviceIdStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for BluetoothDeviceId {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.BluetoothDeviceId;{c17949af-57c1-4642-bcce-e6c06b20ae76})");
}
unsafe impl ::windows::core::Interface for BluetoothDeviceId {
    type Vtable = IBluetoothDeviceId_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc17949af_57c1_4642_bcce_e6c06b20ae76);
}
impl ::windows::core::RuntimeName for BluetoothDeviceId {
    const NAME: &'static str = "Windows.Devices.Bluetooth.BluetoothDeviceId";
}
impl ::core::convert::From<BluetoothDeviceId> for ::windows::core::IUnknown {
    fn from(value: BluetoothDeviceId) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BluetoothDeviceId> for ::windows::core::IUnknown {
    fn from(value: &BluetoothDeviceId) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BluetoothDeviceId {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BluetoothDeviceId {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BluetoothDeviceId> for ::windows::core::IInspectable {
    fn from(value: BluetoothDeviceId) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BluetoothDeviceId> for ::windows::core::IInspectable {
    fn from(value: &BluetoothDeviceId) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BluetoothDeviceId {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BluetoothDeviceId {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for BluetoothDeviceId {}
unsafe impl ::core::marker::Sync for BluetoothDeviceId {}
#[doc = "*Required features: `Devices_Bluetooth`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
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
impl ::core::convert::From<i32> for BluetoothError {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for BluetoothError {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for BluetoothError {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.BluetoothError;i4)");
}
impl ::windows::core::DefaultType for BluetoothError {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Bluetooth`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BluetoothLEAppearance(pub ::windows::core::IInspectable);
impl BluetoothLEAppearance {
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn RawValue(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn Category(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn SubCategory(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn FromRawValue(rawvalue: u16) -> ::windows::core::Result<BluetoothLEAppearance> {
        Self::IBluetoothLEAppearanceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), rawvalue, &mut result__).from_abi::<BluetoothLEAppearance>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn FromParts(appearancecategory: u16, appearancesubcategory: u16) -> ::windows::core::Result<BluetoothLEAppearance> {
        Self::IBluetoothLEAppearanceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), appearancecategory, appearancesubcategory, &mut result__).from_abi::<BluetoothLEAppearance>(result__)
        })
    }
    pub fn IBluetoothLEAppearanceStatics<R, F: FnOnce(&IBluetoothLEAppearanceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BluetoothLEAppearance, IBluetoothLEAppearanceStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for BluetoothLEAppearance {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.BluetoothLEAppearance;{5d2079f2-66a8-4258-985e-02b4d9509f18})");
}
unsafe impl ::windows::core::Interface for BluetoothLEAppearance {
    type Vtable = IBluetoothLEAppearance_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d2079f2_66a8_4258_985e_02b4d9509f18);
}
impl ::windows::core::RuntimeName for BluetoothLEAppearance {
    const NAME: &'static str = "Windows.Devices.Bluetooth.BluetoothLEAppearance";
}
impl ::core::convert::From<BluetoothLEAppearance> for ::windows::core::IUnknown {
    fn from(value: BluetoothLEAppearance) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BluetoothLEAppearance> for ::windows::core::IUnknown {
    fn from(value: &BluetoothLEAppearance) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BluetoothLEAppearance {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BluetoothLEAppearance {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BluetoothLEAppearance> for ::windows::core::IInspectable {
    fn from(value: BluetoothLEAppearance) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BluetoothLEAppearance> for ::windows::core::IInspectable {
    fn from(value: &BluetoothLEAppearance) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BluetoothLEAppearance {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BluetoothLEAppearance {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for BluetoothLEAppearance {}
unsafe impl ::core::marker::Sync for BluetoothLEAppearance {}
#[doc = "*Required features: `Devices_Bluetooth`*"]
pub struct BluetoothLEAppearanceCategories {}
impl BluetoothLEAppearanceCategories {
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn Uncategorized() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn Phone() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn Computer() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn Watch() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn Clock() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn Display() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn RemoteControl() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn EyeGlasses() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn Tag() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn Keyring() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn MediaPlayer() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn BarcodeScanner() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn Thermometer() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn HeartRate() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn BloodPressure() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn HumanInterfaceDevice() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn GlucoseMeter() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn RunningWalking() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn Cycling() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn PulseOximeter() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn WeightScale() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn OutdoorSportActivity() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    pub fn IBluetoothLEAppearanceCategoriesStatics<R, F: FnOnce(&IBluetoothLEAppearanceCategoriesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BluetoothLEAppearanceCategories, IBluetoothLEAppearanceCategoriesStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for BluetoothLEAppearanceCategories {
    const NAME: &'static str = "Windows.Devices.Bluetooth.BluetoothLEAppearanceCategories";
}
#[doc = "*Required features: `Devices_Bluetooth`*"]
pub struct BluetoothLEAppearanceSubcategories {}
impl BluetoothLEAppearanceSubcategories {
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn Generic() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn SportsWatch() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn ThermometerEar() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn HeartRateBelt() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn BloodPressureArm() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn BloodPressureWrist() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn Keyboard() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn Mouse() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn Joystick() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn Gamepad() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn DigitizerTablet() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn CardReader() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn DigitalPen() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn BarcodeScanner() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn RunningWalkingInShoe() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn RunningWalkingOnShoe() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn RunningWalkingOnHip() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn CyclingComputer() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn CyclingSpeedSensor() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn CyclingCadenceSensor() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn CyclingPowerSensor() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn CyclingSpeedCadenceSensor() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn OximeterFingertip() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn OximeterWristWorn() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn LocationDisplay() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).30)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn LocationNavigationDisplay() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).31)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn LocationPod() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).32)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn LocationNavigationPod() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).33)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    pub fn IBluetoothLEAppearanceSubcategoriesStatics<R, F: FnOnce(&IBluetoothLEAppearanceSubcategoriesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BluetoothLEAppearanceSubcategories, IBluetoothLEAppearanceSubcategoriesStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for BluetoothLEAppearanceSubcategories {
    const NAME: &'static str = "Windows.Devices.Bluetooth.BluetoothLEAppearanceSubcategories";
}
#[doc = "*Required features: `Devices_Bluetooth`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BluetoothLEConnectionParameters(pub ::windows::core::IInspectable);
impl BluetoothLEConnectionParameters {
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn LinkTimeout(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn ConnectionLatency(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn ConnectionInterval(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for BluetoothLEConnectionParameters {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.BluetoothLEConnectionParameters;{33cb0771-8da9-508f-a366-1ca388c929ab})");
}
unsafe impl ::windows::core::Interface for BluetoothLEConnectionParameters {
    type Vtable = IBluetoothLEConnectionParameters_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33cb0771_8da9_508f_a366_1ca388c929ab);
}
impl ::windows::core::RuntimeName for BluetoothLEConnectionParameters {
    const NAME: &'static str = "Windows.Devices.Bluetooth.BluetoothLEConnectionParameters";
}
impl ::core::convert::From<BluetoothLEConnectionParameters> for ::windows::core::IUnknown {
    fn from(value: BluetoothLEConnectionParameters) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BluetoothLEConnectionParameters> for ::windows::core::IUnknown {
    fn from(value: &BluetoothLEConnectionParameters) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BluetoothLEConnectionParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BluetoothLEConnectionParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BluetoothLEConnectionParameters> for ::windows::core::IInspectable {
    fn from(value: BluetoothLEConnectionParameters) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BluetoothLEConnectionParameters> for ::windows::core::IInspectable {
    fn from(value: &BluetoothLEConnectionParameters) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BluetoothLEConnectionParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BluetoothLEConnectionParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for BluetoothLEConnectionParameters {}
unsafe impl ::core::marker::Sync for BluetoothLEConnectionParameters {}
#[doc = "*Required features: `Devices_Bluetooth`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BluetoothLEConnectionPhy(pub ::windows::core::IInspectable);
impl BluetoothLEConnectionPhy {
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn TransmitInfo(&self) -> ::windows::core::Result<BluetoothLEConnectionPhyInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothLEConnectionPhyInfo>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn ReceiveInfo(&self) -> ::windows::core::Result<BluetoothLEConnectionPhyInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothLEConnectionPhyInfo>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for BluetoothLEConnectionPhy {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.BluetoothLEConnectionPhy;{781e5e48-621e-5a7e-8be6-1b9561ff63c9})");
}
unsafe impl ::windows::core::Interface for BluetoothLEConnectionPhy {
    type Vtable = IBluetoothLEConnectionPhy_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x781e5e48_621e_5a7e_8be6_1b9561ff63c9);
}
impl ::windows::core::RuntimeName for BluetoothLEConnectionPhy {
    const NAME: &'static str = "Windows.Devices.Bluetooth.BluetoothLEConnectionPhy";
}
impl ::core::convert::From<BluetoothLEConnectionPhy> for ::windows::core::IUnknown {
    fn from(value: BluetoothLEConnectionPhy) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BluetoothLEConnectionPhy> for ::windows::core::IUnknown {
    fn from(value: &BluetoothLEConnectionPhy) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BluetoothLEConnectionPhy {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BluetoothLEConnectionPhy {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BluetoothLEConnectionPhy> for ::windows::core::IInspectable {
    fn from(value: BluetoothLEConnectionPhy) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BluetoothLEConnectionPhy> for ::windows::core::IInspectable {
    fn from(value: &BluetoothLEConnectionPhy) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BluetoothLEConnectionPhy {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BluetoothLEConnectionPhy {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for BluetoothLEConnectionPhy {}
unsafe impl ::core::marker::Sync for BluetoothLEConnectionPhy {}
#[doc = "*Required features: `Devices_Bluetooth`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BluetoothLEConnectionPhyInfo(pub ::windows::core::IInspectable);
impl BluetoothLEConnectionPhyInfo {
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn IsUncoded1MPhy(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn IsUncoded2MPhy(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn IsCodedPhy(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for BluetoothLEConnectionPhyInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.BluetoothLEConnectionPhyInfo;{9a100bdd-602e-5c27-a1ae-b230015a6394})");
}
unsafe impl ::windows::core::Interface for BluetoothLEConnectionPhyInfo {
    type Vtable = IBluetoothLEConnectionPhyInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a100bdd_602e_5c27_a1ae_b230015a6394);
}
impl ::windows::core::RuntimeName for BluetoothLEConnectionPhyInfo {
    const NAME: &'static str = "Windows.Devices.Bluetooth.BluetoothLEConnectionPhyInfo";
}
impl ::core::convert::From<BluetoothLEConnectionPhyInfo> for ::windows::core::IUnknown {
    fn from(value: BluetoothLEConnectionPhyInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BluetoothLEConnectionPhyInfo> for ::windows::core::IUnknown {
    fn from(value: &BluetoothLEConnectionPhyInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BluetoothLEConnectionPhyInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BluetoothLEConnectionPhyInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BluetoothLEConnectionPhyInfo> for ::windows::core::IInspectable {
    fn from(value: BluetoothLEConnectionPhyInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BluetoothLEConnectionPhyInfo> for ::windows::core::IInspectable {
    fn from(value: &BluetoothLEConnectionPhyInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BluetoothLEConnectionPhyInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BluetoothLEConnectionPhyInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for BluetoothLEConnectionPhyInfo {}
unsafe impl ::core::marker::Sync for BluetoothLEConnectionPhyInfo {}
#[doc = "*Required features: `Devices_Bluetooth`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BluetoothLEDevice(pub ::windows::core::IInspectable);
impl BluetoothLEDevice {
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_Bluetooth`, `Devices_Bluetooth_GenericAttributeProfile`, `Foundation_Collections`*"]
    pub fn GattServices(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<GenericAttributeProfile::GattDeviceService>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<GenericAttributeProfile::GattDeviceService>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn ConnectionStatus(&self) -> ::windows::core::Result<BluetoothConnectionStatus> {
        let this = self;
        unsafe {
            let mut result__: BluetoothConnectionStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothConnectionStatus>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn BluetoothAddress(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn GetGattService<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, serviceuuid: Param0) -> ::windows::core::Result<GenericAttributeProfile::GattDeviceService> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), serviceuuid.into_param().abi(), &mut result__).from_abi::<GenericAttributeProfile::GattDeviceService>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`*"]
    pub fn NameChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<BluetoothLEDevice, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`*"]
    pub fn RemoveNameChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`*"]
    pub fn GattServicesChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<BluetoothLEDevice, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`*"]
    pub fn RemoveGattServicesChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`*"]
    pub fn ConnectionStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<BluetoothLEDevice, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`*"]
    pub fn RemoveConnectionStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`*"]
    pub fn FromIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BluetoothLEDevice>> {
        Self::IBluetoothLEDeviceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<BluetoothLEDevice>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`*"]
    pub fn FromBluetoothAddressAsync(bluetoothaddress: u64) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BluetoothLEDevice>> {
        Self::IBluetoothLEDeviceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), bluetoothaddress, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<BluetoothLEDevice>>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IBluetoothLEDeviceStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Devices_Enumeration")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Devices_Enumeration`*"]
    pub fn DeviceInformation(&self) -> ::windows::core::Result<super::Enumeration::DeviceInformation> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEDevice2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Enumeration::DeviceInformation>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn Appearance(&self) -> ::windows::core::Result<BluetoothLEAppearance> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEDevice2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothLEAppearance>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn BluetoothAddressType(&self) -> ::windows::core::Result<BluetoothAddressType> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEDevice2>(self)?;
        unsafe {
            let mut result__: BluetoothAddressType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothAddressType>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn GetDeviceSelectorFromPairingState(pairingstate: bool) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IBluetoothLEDeviceStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), pairingstate, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn GetDeviceSelectorFromConnectionStatus(connectionstatus: BluetoothConnectionStatus) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IBluetoothLEDeviceStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), connectionstatus, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn GetDeviceSelectorFromDeviceName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(devicename: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IBluetoothLEDeviceStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), devicename.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn GetDeviceSelectorFromBluetoothAddress(bluetoothaddress: u64) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IBluetoothLEDeviceStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), bluetoothaddress, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn GetDeviceSelectorFromBluetoothAddressWithBluetoothAddressType(bluetoothaddress: u64, bluetoothaddresstype: BluetoothAddressType) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IBluetoothLEDeviceStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), bluetoothaddress, bluetoothaddresstype, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn GetDeviceSelectorFromAppearance<'a, Param0: ::windows::core::IntoParam<'a, BluetoothLEAppearance>>(appearance: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IBluetoothLEDeviceStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), appearance.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`*"]
    pub fn FromBluetoothAddressWithBluetoothAddressTypeAsync(bluetoothaddress: u64, bluetoothaddresstype: BluetoothAddressType) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BluetoothLEDevice>> {
        Self::IBluetoothLEDeviceStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), bluetoothaddress, bluetoothaddresstype, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<BluetoothLEDevice>>(result__)
        })
    }
    #[cfg(feature = "Devices_Enumeration")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Devices_Enumeration`*"]
    pub fn DeviceAccessInformation(&self) -> ::windows::core::Result<super::Enumeration::DeviceAccessInformation> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEDevice3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Enumeration::DeviceAccessInformation>(result__)
        }
    }
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))]
    #[doc = "*Required features: `Devices_Bluetooth`, `Devices_Enumeration`, `Foundation`*"]
    pub fn RequestAccessAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::Enumeration::DeviceAccessStatus>> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEDevice3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::Enumeration::DeviceAccessStatus>>(result__)
        }
    }
    #[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation"))]
    #[doc = "*Required features: `Devices_Bluetooth`, `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn GetGattServicesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<GenericAttributeProfile::GattDeviceServicesResult>> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEDevice3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<GenericAttributeProfile::GattDeviceServicesResult>>(result__)
        }
    }
    #[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation"))]
    #[doc = "*Required features: `Devices_Bluetooth`, `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn GetGattServicesWithCacheModeAsync(&self, cachemode: BluetoothCacheMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<GenericAttributeProfile::GattDeviceServicesResult>> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEDevice3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), cachemode, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<GenericAttributeProfile::GattDeviceServicesResult>>(result__)
        }
    }
    #[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation"))]
    #[doc = "*Required features: `Devices_Bluetooth`, `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn GetGattServicesForUuidAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, serviceuuid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<GenericAttributeProfile::GattDeviceServicesResult>> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEDevice3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), serviceuuid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<GenericAttributeProfile::GattDeviceServicesResult>>(result__)
        }
    }
    #[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation"))]
    #[doc = "*Required features: `Devices_Bluetooth`, `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn GetGattServicesForUuidWithCacheModeAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, serviceuuid: Param0, cachemode: BluetoothCacheMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<GenericAttributeProfile::GattDeviceServicesResult>> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEDevice3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), serviceuuid.into_param().abi(), cachemode, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<GenericAttributeProfile::GattDeviceServicesResult>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn BluetoothDeviceId(&self) -> ::windows::core::Result<BluetoothDeviceId> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEDevice4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothDeviceId>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn WasSecureConnectionUsedForPairing(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEDevice5>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn GetConnectionParameters(&self) -> ::windows::core::Result<BluetoothLEConnectionParameters> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEDevice6>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothLEConnectionParameters>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn GetConnectionPhy(&self) -> ::windows::core::Result<BluetoothLEConnectionPhy> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEDevice6>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothLEConnectionPhy>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn RequestPreferredConnectionParameters<'a, Param0: ::windows::core::IntoParam<'a, BluetoothLEPreferredConnectionParameters>>(&self, preferredconnectionparameters: Param0) -> ::windows::core::Result<BluetoothLEPreferredConnectionParametersRequest> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEDevice6>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), preferredconnectionparameters.into_param().abi(), &mut result__).from_abi::<BluetoothLEPreferredConnectionParametersRequest>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`*"]
    pub fn ConnectionParametersChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<BluetoothLEDevice, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEDevice6>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`*"]
    pub fn RemoveConnectionParametersChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEDevice6>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`*"]
    pub fn ConnectionPhyChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<BluetoothLEDevice, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEDevice6>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`*"]
    pub fn RemoveConnectionPhyChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEDevice6>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn IBluetoothLEDeviceStatics<R, F: FnOnce(&IBluetoothLEDeviceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BluetoothLEDevice, IBluetoothLEDeviceStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IBluetoothLEDeviceStatics2<R, F: FnOnce(&IBluetoothLEDeviceStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BluetoothLEDevice, IBluetoothLEDeviceStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for BluetoothLEDevice {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.BluetoothLEDevice;{b5ee2f7b-4ad8-4642-ac48-80a0b500e887})");
}
unsafe impl ::windows::core::Interface for BluetoothLEDevice {
    type Vtable = IBluetoothLEDevice_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5ee2f7b_4ad8_4642_ac48_80a0b500e887);
}
impl ::windows::core::RuntimeName for BluetoothLEDevice {
    const NAME: &'static str = "Windows.Devices.Bluetooth.BluetoothLEDevice";
}
impl ::core::convert::From<BluetoothLEDevice> for ::windows::core::IUnknown {
    fn from(value: BluetoothLEDevice) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BluetoothLEDevice> for ::windows::core::IUnknown {
    fn from(value: &BluetoothLEDevice) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BluetoothLEDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BluetoothLEDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BluetoothLEDevice> for ::windows::core::IInspectable {
    fn from(value: BluetoothLEDevice) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BluetoothLEDevice> for ::windows::core::IInspectable {
    fn from(value: &BluetoothLEDevice) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BluetoothLEDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BluetoothLEDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<BluetoothLEDevice> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: BluetoothLEDevice) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&BluetoothLEDevice> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &BluetoothLEDevice) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for BluetoothLEDevice {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &BluetoothLEDevice {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for BluetoothLEDevice {}
unsafe impl ::core::marker::Sync for BluetoothLEDevice {}
#[doc = "*Required features: `Devices_Bluetooth`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BluetoothLEPreferredConnectionParameters(pub ::windows::core::IInspectable);
impl BluetoothLEPreferredConnectionParameters {
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn LinkTimeout(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn ConnectionLatency(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn MinConnectionInterval(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn MaxConnectionInterval(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn Balanced() -> ::windows::core::Result<BluetoothLEPreferredConnectionParameters> {
        Self::IBluetoothLEPreferredConnectionParametersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothLEPreferredConnectionParameters>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn ThroughputOptimized() -> ::windows::core::Result<BluetoothLEPreferredConnectionParameters> {
        Self::IBluetoothLEPreferredConnectionParametersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothLEPreferredConnectionParameters>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn PowerOptimized() -> ::windows::core::Result<BluetoothLEPreferredConnectionParameters> {
        Self::IBluetoothLEPreferredConnectionParametersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothLEPreferredConnectionParameters>(result__)
        })
    }
    pub fn IBluetoothLEPreferredConnectionParametersStatics<R, F: FnOnce(&IBluetoothLEPreferredConnectionParametersStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BluetoothLEPreferredConnectionParameters, IBluetoothLEPreferredConnectionParametersStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for BluetoothLEPreferredConnectionParameters {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.BluetoothLEPreferredConnectionParameters;{f2f44344-7372-5f7b-9b34-29c944f5a715})");
}
unsafe impl ::windows::core::Interface for BluetoothLEPreferredConnectionParameters {
    type Vtable = IBluetoothLEPreferredConnectionParameters_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2f44344_7372_5f7b_9b34_29c944f5a715);
}
impl ::windows::core::RuntimeName for BluetoothLEPreferredConnectionParameters {
    const NAME: &'static str = "Windows.Devices.Bluetooth.BluetoothLEPreferredConnectionParameters";
}
impl ::core::convert::From<BluetoothLEPreferredConnectionParameters> for ::windows::core::IUnknown {
    fn from(value: BluetoothLEPreferredConnectionParameters) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BluetoothLEPreferredConnectionParameters> for ::windows::core::IUnknown {
    fn from(value: &BluetoothLEPreferredConnectionParameters) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BluetoothLEPreferredConnectionParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BluetoothLEPreferredConnectionParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BluetoothLEPreferredConnectionParameters> for ::windows::core::IInspectable {
    fn from(value: BluetoothLEPreferredConnectionParameters) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BluetoothLEPreferredConnectionParameters> for ::windows::core::IInspectable {
    fn from(value: &BluetoothLEPreferredConnectionParameters) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BluetoothLEPreferredConnectionParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BluetoothLEPreferredConnectionParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for BluetoothLEPreferredConnectionParameters {}
unsafe impl ::core::marker::Sync for BluetoothLEPreferredConnectionParameters {}
#[doc = "*Required features: `Devices_Bluetooth`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BluetoothLEPreferredConnectionParametersRequest(pub ::windows::core::IInspectable);
impl BluetoothLEPreferredConnectionParametersRequest {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn Status(&self) -> ::windows::core::Result<BluetoothLEPreferredConnectionParametersRequestStatus> {
        let this = self;
        unsafe {
            let mut result__: BluetoothLEPreferredConnectionParametersRequestStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothLEPreferredConnectionParametersRequestStatus>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for BluetoothLEPreferredConnectionParametersRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.BluetoothLEPreferredConnectionParametersRequest;{8a375276-a528-5266-b661-cce6a5ff9739})");
}
unsafe impl ::windows::core::Interface for BluetoothLEPreferredConnectionParametersRequest {
    type Vtable = IBluetoothLEPreferredConnectionParametersRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8a375276_a528_5266_b661_cce6a5ff9739);
}
impl ::windows::core::RuntimeName for BluetoothLEPreferredConnectionParametersRequest {
    const NAME: &'static str = "Windows.Devices.Bluetooth.BluetoothLEPreferredConnectionParametersRequest";
}
impl ::core::convert::From<BluetoothLEPreferredConnectionParametersRequest> for ::windows::core::IUnknown {
    fn from(value: BluetoothLEPreferredConnectionParametersRequest) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BluetoothLEPreferredConnectionParametersRequest> for ::windows::core::IUnknown {
    fn from(value: &BluetoothLEPreferredConnectionParametersRequest) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BluetoothLEPreferredConnectionParametersRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BluetoothLEPreferredConnectionParametersRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BluetoothLEPreferredConnectionParametersRequest> for ::windows::core::IInspectable {
    fn from(value: BluetoothLEPreferredConnectionParametersRequest) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BluetoothLEPreferredConnectionParametersRequest> for ::windows::core::IInspectable {
    fn from(value: &BluetoothLEPreferredConnectionParametersRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BluetoothLEPreferredConnectionParametersRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BluetoothLEPreferredConnectionParametersRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<BluetoothLEPreferredConnectionParametersRequest> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: BluetoothLEPreferredConnectionParametersRequest) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&BluetoothLEPreferredConnectionParametersRequest> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &BluetoothLEPreferredConnectionParametersRequest) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for BluetoothLEPreferredConnectionParametersRequest {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &BluetoothLEPreferredConnectionParametersRequest {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for BluetoothLEPreferredConnectionParametersRequest {}
unsafe impl ::core::marker::Sync for BluetoothLEPreferredConnectionParametersRequest {}
#[doc = "*Required features: `Devices_Bluetooth`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct BluetoothLEPreferredConnectionParametersRequestStatus(pub i32);
impl BluetoothLEPreferredConnectionParametersRequestStatus {
    pub const Unspecified: BluetoothLEPreferredConnectionParametersRequestStatus = BluetoothLEPreferredConnectionParametersRequestStatus(0i32);
    pub const Success: BluetoothLEPreferredConnectionParametersRequestStatus = BluetoothLEPreferredConnectionParametersRequestStatus(1i32);
    pub const DeviceNotAvailable: BluetoothLEPreferredConnectionParametersRequestStatus = BluetoothLEPreferredConnectionParametersRequestStatus(2i32);
    pub const AccessDenied: BluetoothLEPreferredConnectionParametersRequestStatus = BluetoothLEPreferredConnectionParametersRequestStatus(3i32);
}
impl ::core::convert::From<i32> for BluetoothLEPreferredConnectionParametersRequestStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for BluetoothLEPreferredConnectionParametersRequestStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for BluetoothLEPreferredConnectionParametersRequestStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.BluetoothLEPreferredConnectionParametersRequestStatus;i4)");
}
impl ::windows::core::DefaultType for BluetoothLEPreferredConnectionParametersRequestStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Bluetooth`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
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
impl ::core::convert::From<i32> for BluetoothMajorClass {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for BluetoothMajorClass {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for BluetoothMajorClass {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.BluetoothMajorClass;i4)");
}
impl ::windows::core::DefaultType for BluetoothMajorClass {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Bluetooth`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
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
impl ::core::convert::From<i32> for BluetoothMinorClass {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for BluetoothMinorClass {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for BluetoothMinorClass {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.BluetoothMinorClass;i4)");
}
impl ::windows::core::DefaultType for BluetoothMinorClass {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Bluetooth`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
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
impl ::core::convert::From<u32> for BluetoothServiceCapabilities {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for BluetoothServiceCapabilities {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for BluetoothServiceCapabilities {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.BluetoothServiceCapabilities;u4)");
}
impl ::windows::core::DefaultType for BluetoothServiceCapabilities {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for BluetoothServiceCapabilities {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for BluetoothServiceCapabilities {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for BluetoothServiceCapabilities {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for BluetoothServiceCapabilities {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for BluetoothServiceCapabilities {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Devices_Bluetooth`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BluetoothSignalStrengthFilter(pub ::windows::core::IInspectable);
impl BluetoothSignalStrengthFilter {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BluetoothSignalStrengthFilter, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`*"]
    pub fn InRangeThresholdInDBm(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i16>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i16>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`*"]
    pub fn SetInRangeThresholdInDBm<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<i16>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`*"]
    pub fn OutOfRangeThresholdInDBm(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i16>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i16>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`*"]
    pub fn SetOutOfRangeThresholdInDBm<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<i16>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`*"]
    pub fn OutOfRangeTimeout(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`*"]
    pub fn SetOutOfRangeTimeout<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`*"]
    pub fn SamplingInterval(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`*"]
    pub fn SetSamplingInterval<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for BluetoothSignalStrengthFilter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.BluetoothSignalStrengthFilter;{df7b7391-6bb5-4cfe-90b1-5d7324edcf7f})");
}
unsafe impl ::windows::core::Interface for BluetoothSignalStrengthFilter {
    type Vtable = IBluetoothSignalStrengthFilter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdf7b7391_6bb5_4cfe_90b1_5d7324edcf7f);
}
impl ::windows::core::RuntimeName for BluetoothSignalStrengthFilter {
    const NAME: &'static str = "Windows.Devices.Bluetooth.BluetoothSignalStrengthFilter";
}
impl ::core::convert::From<BluetoothSignalStrengthFilter> for ::windows::core::IUnknown {
    fn from(value: BluetoothSignalStrengthFilter) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BluetoothSignalStrengthFilter> for ::windows::core::IUnknown {
    fn from(value: &BluetoothSignalStrengthFilter) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BluetoothSignalStrengthFilter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BluetoothSignalStrengthFilter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BluetoothSignalStrengthFilter> for ::windows::core::IInspectable {
    fn from(value: BluetoothSignalStrengthFilter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BluetoothSignalStrengthFilter> for ::windows::core::IInspectable {
    fn from(value: &BluetoothSignalStrengthFilter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BluetoothSignalStrengthFilter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BluetoothSignalStrengthFilter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for BluetoothSignalStrengthFilter {}
unsafe impl ::core::marker::Sync for BluetoothSignalStrengthFilter {}
#[doc = "*Required features: `Devices_Bluetooth`*"]
pub struct BluetoothUuidHelper {}
impl BluetoothUuidHelper {
    #[doc = "*Required features: `Devices_Bluetooth`*"]
    pub fn FromShortId(shortid: u32) -> ::windows::core::Result<::windows::core::GUID> {
        Self::IBluetoothUuidHelperStatics(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), shortid, &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth`, `Foundation`*"]
    pub fn TryGetShortId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(uuid: Param0) -> ::windows::core::Result<super::super::Foundation::IReference<u32>> {
        Self::IBluetoothUuidHelperStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), uuid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IReference<u32>>(result__)
        })
    }
    pub fn IBluetoothUuidHelperStatics<R, F: FnOnce(&IBluetoothUuidHelperStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BluetoothUuidHelper, IBluetoothUuidHelperStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for BluetoothUuidHelper {
    const NAME: &'static str = "Windows.Devices.Bluetooth.BluetoothUuidHelper";
}
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothAdapter(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBluetoothAdapter {
    type Vtable = IBluetoothAdapter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7974f04c_5f7a_4a34_9225_a855f84b1a8b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothAdapter_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Devices_Radios", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Radios", feature = "Foundation")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothAdapter2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBluetoothAdapter2 {
    type Vtable = IBluetoothAdapter2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xac94cecc_24d5_41b3_916d_1097c50b102b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothAdapter2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothAdapter3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBluetoothAdapter3 {
    type Vtable = IBluetoothAdapter3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f8624e0_cba9_5211_9f89_3aac62b4c6b8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothAdapter3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothAdapterStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBluetoothAdapterStatics {
    type Vtable = IBluetoothAdapterStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8b02fb6a_ac4c_4741_8661_8eab7d17ea9f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothAdapterStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothClassOfDevice(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBluetoothClassOfDevice {
    type Vtable = IBluetoothClassOfDevice_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd640227e_d7d7_4661_9454_65039ca17a2b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothClassOfDevice_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut BluetoothMajorClass) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut BluetoothMinorClass) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut BluetoothServiceCapabilities) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothClassOfDeviceStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBluetoothClassOfDeviceStatics {
    type Vtable = IBluetoothClassOfDeviceStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe46135bd_0fa2_416c_91b4_c1e48ca061c1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothClassOfDeviceStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, rawvalue: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, majorclass: BluetoothMajorClass, minorclass: BluetoothMinorClass, servicecapabilities: BluetoothServiceCapabilities, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothDevice(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBluetoothDevice {
    type Vtable = IBluetoothDevice_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2335b156_90d2_4a04_aef5_0e20b9e6b707);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothDevice_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Networking")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Networking"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Foundation_Collections")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut BluetoothConnectionStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothDevice2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBluetoothDevice2 {
    type Vtable = IBluetoothDevice2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0133f954_b156_4dd0_b1f5_c11bc31a5163);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothDevice2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Enumeration")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothDevice3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBluetoothDevice3 {
    type Vtable = IBluetoothDevice3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x57fff78b_651a_4454_b90f_eb21ef0b0d71);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothDevice3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Enumeration")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))] usize,
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Enumeration", feature = "Foundation")))] usize,
    #[cfg(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Foundation")))] usize,
    #[cfg(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cachemode: BluetoothCacheMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Foundation")))] usize,
    #[cfg(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, serviceid: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Foundation")))] usize,
    #[cfg(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, serviceid: ::windows::core::RawPtr, cachemode: BluetoothCacheMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Foundation")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothDevice4(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBluetoothDevice4 {
    type Vtable = IBluetoothDevice4_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x817c34ad_0e9c_42b2_a8dc_3e8094940d12);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothDevice4_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothDevice5(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBluetoothDevice5 {
    type Vtable = IBluetoothDevice5_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5e0b385_5e85_4559_a10d_1c7281379f96);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothDevice5_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothDeviceId(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBluetoothDeviceId {
    type Vtable = IBluetoothDeviceId_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc17949af_57c1_4642_bcce_e6c06b20ae76);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothDeviceId_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothDeviceIdStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBluetoothDeviceIdStatics {
    type Vtable = IBluetoothDeviceIdStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa7884e67_3efb_4f31_bbc2_810e09977404);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothDeviceIdStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothDeviceStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBluetoothDeviceStatics {
    type Vtable = IBluetoothDeviceStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0991df51_57db_4725_bbd7_84f64327ec2c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothDeviceStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Networking"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hostname: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Networking")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, address: u64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothDeviceStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBluetoothDeviceStatics2 {
    type Vtable = IBluetoothDeviceStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc29e8e2f_4e14_4477_aa1b_b8b47e5b7ece);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothDeviceStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pairingstate: bool, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, connectionstatus: BluetoothConnectionStatus, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, devicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bluetoothaddress: u64, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, classofdevice: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEAppearance(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBluetoothLEAppearance {
    type Vtable = IBluetoothLEAppearance_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d2079f2_66a8_4258_985e_02b4d9509f18);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAppearance_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEAppearanceCategoriesStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBluetoothLEAppearanceCategoriesStatics {
    type Vtable = IBluetoothLEAppearanceCategoriesStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d4d54fe_046a_4185_aab6_824cf0610861);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAppearanceCategoriesStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEAppearanceStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBluetoothLEAppearanceStatics {
    type Vtable = IBluetoothLEAppearanceStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa193c0c7_4504_4f4a_9ba5_cd1054e5e065);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAppearanceStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, rawvalue: u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, appearancecategory: u16, appearancesubcategory: u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEAppearanceSubcategoriesStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBluetoothLEAppearanceSubcategoriesStatics {
    type Vtable = IBluetoothLEAppearanceSubcategoriesStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe57ba606_2144_415a_8312_71ccf291f8d1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAppearanceSubcategoriesStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEConnectionParameters(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBluetoothLEConnectionParameters {
    type Vtable = IBluetoothLEConnectionParameters_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33cb0771_8da9_508f_a366_1ca388c929ab);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEConnectionParameters_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEConnectionPhy(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBluetoothLEConnectionPhy {
    type Vtable = IBluetoothLEConnectionPhy_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x781e5e48_621e_5a7e_8be6_1b9561ff63c9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEConnectionPhy_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEConnectionPhyInfo(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBluetoothLEConnectionPhyInfo {
    type Vtable = IBluetoothLEConnectionPhyInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a100bdd_602e_5c27_a1ae_b230015a6394);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEConnectionPhyInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEDevice(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBluetoothLEDevice {
    type Vtable = IBluetoothLEDevice_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5ee2f7b_4ad8_4642_ac48_80a0b500e887);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEDevice_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation_Collections")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut BluetoothConnectionStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, serviceuuid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_GenericAttributeProfile"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEDevice2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBluetoothLEDevice2 {
    type Vtable = IBluetoothLEDevice2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x26f062b3_7aee_4d31_baba_b1b9775f5916);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEDevice2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Enumeration")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut BluetoothAddressType) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEDevice3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBluetoothLEDevice3 {
    type Vtable = IBluetoothLEDevice3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaee9e493_44ac_40dc_af33_b2c13c01ca46);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEDevice3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Enumeration")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))] usize,
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Enumeration", feature = "Foundation")))] usize,
    #[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation")))] usize,
    #[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cachemode: BluetoothCacheMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation")))] usize,
    #[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, serviceuuid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation")))] usize,
    #[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, serviceuuid: ::windows::core::GUID, cachemode: BluetoothCacheMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEDevice4(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBluetoothLEDevice4 {
    type Vtable = IBluetoothLEDevice4_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2b605031_2248_4b2f_acf0_7cee36fc5870);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEDevice4_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEDevice5(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBluetoothLEDevice5 {
    type Vtable = IBluetoothLEDevice5_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9d6a1260_5287_458e_95ba_17c8b7bb326e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEDevice5_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEDevice6(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBluetoothLEDevice6 {
    type Vtable = IBluetoothLEDevice6_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xca7190ef_0cae_573c_a1ca_e1fc5bfc39e2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEDevice6_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, preferredconnectionparameters: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEDeviceStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBluetoothLEDeviceStatics {
    type Vtable = IBluetoothLEDeviceStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc8cf1a19_f0b6_4bf0_8689_41303de2d9f4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEDeviceStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bluetoothaddress: u64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEDeviceStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBluetoothLEDeviceStatics2 {
    type Vtable = IBluetoothLEDeviceStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5f12c06b_3bac_43e8_ad16_563271bd41c2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEDeviceStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pairingstate: bool, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, connectionstatus: BluetoothConnectionStatus, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, devicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bluetoothaddress: u64, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bluetoothaddress: u64, bluetoothaddresstype: BluetoothAddressType, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, appearance: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bluetoothaddress: u64, bluetoothaddresstype: BluetoothAddressType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEPreferredConnectionParameters(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBluetoothLEPreferredConnectionParameters {
    type Vtable = IBluetoothLEPreferredConnectionParameters_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2f44344_7372_5f7b_9b34_29c944f5a715);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEPreferredConnectionParameters_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEPreferredConnectionParametersRequest(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBluetoothLEPreferredConnectionParametersRequest {
    type Vtable = IBluetoothLEPreferredConnectionParametersRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8a375276_a528_5266_b661_cce6a5ff9739);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEPreferredConnectionParametersRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut BluetoothLEPreferredConnectionParametersRequestStatus) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEPreferredConnectionParametersStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBluetoothLEPreferredConnectionParametersStatics {
    type Vtable = IBluetoothLEPreferredConnectionParametersStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0e3e8edc_2751_55aa_a838_8faeee818d72);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEPreferredConnectionParametersStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothSignalStrengthFilter(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBluetoothSignalStrengthFilter {
    type Vtable = IBluetoothSignalStrengthFilter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdf7b7391_6bb5_4cfe_90b1_5d7324edcf7f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothSignalStrengthFilter_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothUuidHelperStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBluetoothUuidHelperStatics {
    type Vtable = IBluetoothUuidHelperStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x17df0cd8_cf74_4b21_afe6_f57a11bcdea0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothUuidHelperStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, shortid: u32, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uuid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
