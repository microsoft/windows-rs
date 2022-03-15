#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "Devices_Bluetooth_Advertisement")]
pub mod Advertisement;
#[cfg(feature = "Devices_Bluetooth_Background")]
pub mod Background;
#[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
pub mod GenericAttributeProfile;
#[cfg(feature = "Devices_Bluetooth_Rfcomm")]
pub mod Rfcomm;
#[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
#[repr(transparent)]
pub struct BluetoothAdapter(::windows::core::IUnknown);
impl BluetoothAdapter {
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn BluetoothAddress(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BluetoothAddress)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn IsClassicSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsClassicSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn IsLowEnergySupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsLowEnergySupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn IsPeripheralRoleSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsPeripheralRoleSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn IsCentralRoleSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsCentralRoleSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn IsAdvertisementOffloadSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsAdvertisementOffloadSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`, `\"Devices_Radios\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "Devices_Radios", feature = "Foundation"))]
    pub fn GetRadioAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::Radios::Radio>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetRadioAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::Radios::Radio>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn AreClassicSecureConnectionsSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IBluetoothAdapter2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AreClassicSecureConnectionsSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn AreLowEnergySecureConnectionsSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IBluetoothAdapter2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AreLowEnergySecureConnectionsSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn IsExtendedAdvertisingSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IBluetoothAdapter3>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsExtendedAdvertisingSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn MaxAdvertisementDataLength(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IBluetoothAdapter3>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MaxAdvertisementDataLength)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IBluetoothAdapterStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeviceSelector)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BluetoothAdapter>> {
        Self::IBluetoothAdapterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FromIdAsync)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<BluetoothAdapter>>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDefaultAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BluetoothAdapter>> {
        Self::IBluetoothAdapterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDefaultAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<BluetoothAdapter>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IBluetoothAdapterStatics<R, F: FnOnce(&IBluetoothAdapterStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BluetoothAdapter, IBluetoothAdapterStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for BluetoothAdapter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BluetoothAdapter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BluetoothAdapter {}
impl ::core::fmt::Debug for BluetoothAdapter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BluetoothAdapter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BluetoothAdapter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.BluetoothAdapter;{7974f04c-5f7a-4a34-9225-a855f84b1a8b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for BluetoothAdapter {
    type Vtable = IBluetoothAdapter_Vtbl;
    const IID: ::windows::core::GUID = <IBluetoothAdapter as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BluetoothAdapter {
    const NAME: &'static str = "Windows.Devices.Bluetooth.BluetoothAdapter";
}
impl ::core::convert::From<BluetoothAdapter> for ::windows::core::IUnknown {
    fn from(value: BluetoothAdapter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BluetoothAdapter> for ::windows::core::IUnknown {
    fn from(value: &BluetoothAdapter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BluetoothAdapter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BluetoothAdapter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BluetoothAdapter> for ::windows::core::IInspectable {
    fn from(value: BluetoothAdapter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BluetoothAdapter> for ::windows::core::IInspectable {
    fn from(value: &BluetoothAdapter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BluetoothAdapter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BluetoothAdapter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BluetoothAdapter {}
unsafe impl ::core::marker::Sync for BluetoothAdapter {}
#[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct BluetoothAddressType(pub i32);
impl BluetoothAddressType {
    pub const Public: Self = Self(0i32);
    pub const Random: Self = Self(1i32);
    pub const Unspecified: Self = Self(2i32);
}
impl ::core::marker::Copy for BluetoothAddressType {}
impl ::core::clone::Clone for BluetoothAddressType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BluetoothAddressType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for BluetoothAddressType {
    type Abi = Self;
}
impl ::core::fmt::Debug for BluetoothAddressType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BluetoothAddressType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BluetoothAddressType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.BluetoothAddressType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct BluetoothCacheMode(pub i32);
impl BluetoothCacheMode {
    pub const Cached: Self = Self(0i32);
    pub const Uncached: Self = Self(1i32);
}
impl ::core::marker::Copy for BluetoothCacheMode {}
impl ::core::clone::Clone for BluetoothCacheMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BluetoothCacheMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for BluetoothCacheMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for BluetoothCacheMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BluetoothCacheMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BluetoothCacheMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.BluetoothCacheMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
#[repr(transparent)]
pub struct BluetoothClassOfDevice(::windows::core::IUnknown);
impl BluetoothClassOfDevice {
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn RawValue(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RawValue)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn MajorClass(&self) -> ::windows::core::Result<BluetoothMajorClass> {
        let this = self;
        unsafe {
            let mut result__: BluetoothMajorClass = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MajorClass)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothMajorClass>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn MinorClass(&self) -> ::windows::core::Result<BluetoothMinorClass> {
        let this = self;
        unsafe {
            let mut result__: BluetoothMinorClass = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MinorClass)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothMinorClass>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn ServiceCapabilities(&self) -> ::windows::core::Result<BluetoothServiceCapabilities> {
        let this = self;
        unsafe {
            let mut result__: BluetoothServiceCapabilities = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ServiceCapabilities)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothServiceCapabilities>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn FromRawValue(rawvalue: u32) -> ::windows::core::Result<BluetoothClassOfDevice> {
        Self::IBluetoothClassOfDeviceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FromRawValue)(::core::mem::transmute_copy(this), rawvalue, &mut result__).from_abi::<BluetoothClassOfDevice>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn FromParts(majorclass: BluetoothMajorClass, minorclass: BluetoothMinorClass, servicecapabilities: BluetoothServiceCapabilities) -> ::windows::core::Result<BluetoothClassOfDevice> {
        Self::IBluetoothClassOfDeviceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FromParts)(::core::mem::transmute_copy(this), majorclass, minorclass, servicecapabilities, &mut result__).from_abi::<BluetoothClassOfDevice>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IBluetoothClassOfDeviceStatics<R, F: FnOnce(&IBluetoothClassOfDeviceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BluetoothClassOfDevice, IBluetoothClassOfDeviceStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for BluetoothClassOfDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BluetoothClassOfDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BluetoothClassOfDevice {}
impl ::core::fmt::Debug for BluetoothClassOfDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BluetoothClassOfDevice").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BluetoothClassOfDevice {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.BluetoothClassOfDevice;{d640227e-d7d7-4661-9454-65039ca17a2b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for BluetoothClassOfDevice {
    type Vtable = IBluetoothClassOfDevice_Vtbl;
    const IID: ::windows::core::GUID = <IBluetoothClassOfDevice as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BluetoothClassOfDevice {
    const NAME: &'static str = "Windows.Devices.Bluetooth.BluetoothClassOfDevice";
}
impl ::core::convert::From<BluetoothClassOfDevice> for ::windows::core::IUnknown {
    fn from(value: BluetoothClassOfDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BluetoothClassOfDevice> for ::windows::core::IUnknown {
    fn from(value: &BluetoothClassOfDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BluetoothClassOfDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BluetoothClassOfDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BluetoothClassOfDevice> for ::windows::core::IInspectable {
    fn from(value: BluetoothClassOfDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BluetoothClassOfDevice> for ::windows::core::IInspectable {
    fn from(value: &BluetoothClassOfDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BluetoothClassOfDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BluetoothClassOfDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BluetoothClassOfDevice {}
unsafe impl ::core::marker::Sync for BluetoothClassOfDevice {}
#[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct BluetoothConnectionStatus(pub i32);
impl BluetoothConnectionStatus {
    pub const Disconnected: Self = Self(0i32);
    pub const Connected: Self = Self(1i32);
}
impl ::core::marker::Copy for BluetoothConnectionStatus {}
impl ::core::clone::Clone for BluetoothConnectionStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BluetoothConnectionStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for BluetoothConnectionStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for BluetoothConnectionStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BluetoothConnectionStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BluetoothConnectionStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.BluetoothConnectionStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
#[repr(transparent)]
pub struct BluetoothDevice(::windows::core::IUnknown);
impl BluetoothDevice {
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`, `\"Networking\"`*"]
    #[cfg(feature = "Networking")]
    pub fn HostName(&self) -> ::windows::core::Result<super::super::Networking::HostName> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HostName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Networking::HostName>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Name)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn ClassOfDevice(&self) -> ::windows::core::Result<BluetoothClassOfDevice> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ClassOfDevice)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothClassOfDevice>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`, `\"Foundation_Collections\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn SdpRecords(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Storage::Streams::IBuffer>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SdpRecords)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Storage::Streams::IBuffer>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`, `\"Devices_Bluetooth_Rfcomm\"`, `\"Foundation_Collections\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn RfcommServices(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<Rfcomm::RfcommDeviceService>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RfcommServices)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<Rfcomm::RfcommDeviceService>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn ConnectionStatus(&self) -> ::windows::core::Result<BluetoothConnectionStatus> {
        let this = self;
        unsafe {
            let mut result__: BluetoothConnectionStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ConnectionStatus)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothConnectionStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn BluetoothAddress(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BluetoothAddress)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn NameChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<BluetoothDevice, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NameChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveNameChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveNameChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SdpRecordsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<BluetoothDevice, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SdpRecordsChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSdpRecordsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveSdpRecordsChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ConnectionStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<BluetoothDevice, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ConnectionStatusChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveConnectionStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveConnectionStatusChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`, `\"Devices_Enumeration\"`*"]
    #[cfg(feature = "Devices_Enumeration")]
    pub fn DeviceInformation(&self) -> ::windows::core::Result<super::Enumeration::DeviceInformation> {
        let this = &::windows::core::Interface::cast::<IBluetoothDevice2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceInformation)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Enumeration::DeviceInformation>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`, `\"Devices_Enumeration\"`*"]
    #[cfg(feature = "Devices_Enumeration")]
    pub fn DeviceAccessInformation(&self) -> ::windows::core::Result<super::Enumeration::DeviceAccessInformation> {
        let this = &::windows::core::Interface::cast::<IBluetoothDevice3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceAccessInformation)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Enumeration::DeviceAccessInformation>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`, `\"Devices_Enumeration\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))]
    pub fn RequestAccessAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::Enumeration::DeviceAccessStatus>> {
        let this = &::windows::core::Interface::cast::<IBluetoothDevice3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestAccessAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::Enumeration::DeviceAccessStatus>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`, `\"Devices_Bluetooth_Rfcomm\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Foundation"))]
    pub fn GetRfcommServicesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Rfcomm::RfcommDeviceServicesResult>> {
        let this = &::windows::core::Interface::cast::<IBluetoothDevice3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetRfcommServicesAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<Rfcomm::RfcommDeviceServicesResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`, `\"Devices_Bluetooth_Rfcomm\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Foundation"))]
    pub fn GetRfcommServicesWithCacheModeAsync(&self, cachemode: BluetoothCacheMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Rfcomm::RfcommDeviceServicesResult>> {
        let this = &::windows::core::Interface::cast::<IBluetoothDevice3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetRfcommServicesWithCacheModeAsync)(::core::mem::transmute_copy(this), cachemode, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<Rfcomm::RfcommDeviceServicesResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`, `\"Devices_Bluetooth_Rfcomm\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Foundation"))]
    pub fn GetRfcommServicesForIdAsync<'a, Param0: ::windows::core::IntoParam<'a, Rfcomm::RfcommServiceId>>(&self, serviceid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Rfcomm::RfcommDeviceServicesResult>> {
        let this = &::windows::core::Interface::cast::<IBluetoothDevice3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetRfcommServicesForIdAsync)(::core::mem::transmute_copy(this), serviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<Rfcomm::RfcommDeviceServicesResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`, `\"Devices_Bluetooth_Rfcomm\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Foundation"))]
    pub fn GetRfcommServicesForIdWithCacheModeAsync<'a, Param0: ::windows::core::IntoParam<'a, Rfcomm::RfcommServiceId>>(&self, serviceid: Param0, cachemode: BluetoothCacheMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Rfcomm::RfcommDeviceServicesResult>> {
        let this = &::windows::core::Interface::cast::<IBluetoothDevice3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetRfcommServicesForIdWithCacheModeAsync)(::core::mem::transmute_copy(this), serviceid.into_param().abi(), cachemode, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<Rfcomm::RfcommDeviceServicesResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn BluetoothDeviceId(&self) -> ::windows::core::Result<BluetoothDeviceId> {
        let this = &::windows::core::Interface::cast::<IBluetoothDevice4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BluetoothDeviceId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothDeviceId>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn WasSecureConnectionUsedForPairing(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IBluetoothDevice5>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WasSecureConnectionUsedForPairing)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BluetoothDevice>> {
        Self::IBluetoothDeviceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FromIdAsync)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<BluetoothDevice>>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`, `\"Foundation\"`, `\"Networking\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Networking"))]
    pub fn FromHostNameAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Networking::HostName>>(hostname: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BluetoothDevice>> {
        Self::IBluetoothDeviceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FromHostNameAsync)(::core::mem::transmute_copy(this), hostname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<BluetoothDevice>>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromBluetoothAddressAsync(address: u64) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BluetoothDevice>> {
        Self::IBluetoothDeviceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FromBluetoothAddressAsync)(::core::mem::transmute_copy(this), address, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<BluetoothDevice>>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IBluetoothDeviceStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeviceSelector)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn GetDeviceSelectorFromPairingState(pairingstate: bool) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IBluetoothDeviceStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeviceSelectorFromPairingState)(::core::mem::transmute_copy(this), pairingstate, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn GetDeviceSelectorFromConnectionStatus(connectionstatus: BluetoothConnectionStatus) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IBluetoothDeviceStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeviceSelectorFromConnectionStatus)(::core::mem::transmute_copy(this), connectionstatus, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn GetDeviceSelectorFromDeviceName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(devicename: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IBluetoothDeviceStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeviceSelectorFromDeviceName)(::core::mem::transmute_copy(this), devicename.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn GetDeviceSelectorFromBluetoothAddress(bluetoothaddress: u64) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IBluetoothDeviceStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeviceSelectorFromBluetoothAddress)(::core::mem::transmute_copy(this), bluetoothaddress, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn GetDeviceSelectorFromClassOfDevice<'a, Param0: ::windows::core::IntoParam<'a, BluetoothClassOfDevice>>(classofdevice: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IBluetoothDeviceStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeviceSelectorFromClassOfDevice)(::core::mem::transmute_copy(this), classofdevice.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc(hidden)]
    pub fn IBluetoothDeviceStatics<R, F: FnOnce(&IBluetoothDeviceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BluetoothDevice, IBluetoothDeviceStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IBluetoothDeviceStatics2<R, F: FnOnce(&IBluetoothDeviceStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BluetoothDevice, IBluetoothDeviceStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for BluetoothDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BluetoothDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BluetoothDevice {}
impl ::core::fmt::Debug for BluetoothDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BluetoothDevice").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BluetoothDevice {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.BluetoothDevice;{2335b156-90d2-4a04-aef5-0e20b9e6b707})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for BluetoothDevice {
    type Vtable = IBluetoothDevice_Vtbl;
    const IID: ::windows::core::GUID = <IBluetoothDevice as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BluetoothDevice {
    const NAME: &'static str = "Windows.Devices.Bluetooth.BluetoothDevice";
}
impl ::core::convert::From<BluetoothDevice> for ::windows::core::IUnknown {
    fn from(value: BluetoothDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BluetoothDevice> for ::windows::core::IUnknown {
    fn from(value: &BluetoothDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BluetoothDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BluetoothDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BluetoothDevice> for ::windows::core::IInspectable {
    fn from(value: BluetoothDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BluetoothDevice> for ::windows::core::IInspectable {
    fn from(value: &BluetoothDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BluetoothDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BluetoothDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
#[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
#[repr(transparent)]
pub struct BluetoothDeviceId(::windows::core::IUnknown);
impl BluetoothDeviceId {
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Id)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn IsClassicDevice(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsClassicDevice)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn IsLowEnergyDevice(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsLowEnergyDevice)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn FromId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0) -> ::windows::core::Result<BluetoothDeviceId> {
        Self::IBluetoothDeviceIdStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FromId)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<BluetoothDeviceId>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IBluetoothDeviceIdStatics<R, F: FnOnce(&IBluetoothDeviceIdStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BluetoothDeviceId, IBluetoothDeviceIdStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for BluetoothDeviceId {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BluetoothDeviceId {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BluetoothDeviceId {}
impl ::core::fmt::Debug for BluetoothDeviceId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BluetoothDeviceId").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BluetoothDeviceId {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.BluetoothDeviceId;{c17949af-57c1-4642-bcce-e6c06b20ae76})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for BluetoothDeviceId {
    type Vtable = IBluetoothDeviceId_Vtbl;
    const IID: ::windows::core::GUID = <IBluetoothDeviceId as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BluetoothDeviceId {
    const NAME: &'static str = "Windows.Devices.Bluetooth.BluetoothDeviceId";
}
impl ::core::convert::From<BluetoothDeviceId> for ::windows::core::IUnknown {
    fn from(value: BluetoothDeviceId) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BluetoothDeviceId> for ::windows::core::IUnknown {
    fn from(value: &BluetoothDeviceId) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BluetoothDeviceId {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BluetoothDeviceId {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BluetoothDeviceId> for ::windows::core::IInspectable {
    fn from(value: BluetoothDeviceId) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BluetoothDeviceId> for ::windows::core::IInspectable {
    fn from(value: &BluetoothDeviceId) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BluetoothDeviceId {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BluetoothDeviceId {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BluetoothDeviceId {}
unsafe impl ::core::marker::Sync for BluetoothDeviceId {}
#[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct BluetoothError(pub i32);
impl BluetoothError {
    pub const Success: Self = Self(0i32);
    pub const RadioNotAvailable: Self = Self(1i32);
    pub const ResourceInUse: Self = Self(2i32);
    pub const DeviceNotConnected: Self = Self(3i32);
    pub const OtherError: Self = Self(4i32);
    pub const DisabledByPolicy: Self = Self(5i32);
    pub const NotSupported: Self = Self(6i32);
    pub const DisabledByUser: Self = Self(7i32);
    pub const ConsentRequired: Self = Self(8i32);
    pub const TransportNotSupported: Self = Self(9i32);
}
impl ::core::marker::Copy for BluetoothError {}
impl ::core::clone::Clone for BluetoothError {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BluetoothError {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for BluetoothError {
    type Abi = Self;
}
impl ::core::fmt::Debug for BluetoothError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BluetoothError").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BluetoothError {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.BluetoothError;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
#[repr(transparent)]
pub struct BluetoothLEAppearance(::windows::core::IUnknown);
impl BluetoothLEAppearance {
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn RawValue(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RawValue)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn Category(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Category)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn SubCategory(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SubCategory)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn FromRawValue(rawvalue: u16) -> ::windows::core::Result<BluetoothLEAppearance> {
        Self::IBluetoothLEAppearanceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FromRawValue)(::core::mem::transmute_copy(this), rawvalue, &mut result__).from_abi::<BluetoothLEAppearance>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn FromParts(appearancecategory: u16, appearancesubcategory: u16) -> ::windows::core::Result<BluetoothLEAppearance> {
        Self::IBluetoothLEAppearanceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FromParts)(::core::mem::transmute_copy(this), appearancecategory, appearancesubcategory, &mut result__).from_abi::<BluetoothLEAppearance>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IBluetoothLEAppearanceStatics<R, F: FnOnce(&IBluetoothLEAppearanceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BluetoothLEAppearance, IBluetoothLEAppearanceStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for BluetoothLEAppearance {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BluetoothLEAppearance {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BluetoothLEAppearance {}
impl ::core::fmt::Debug for BluetoothLEAppearance {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BluetoothLEAppearance").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BluetoothLEAppearance {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.BluetoothLEAppearance;{5d2079f2-66a8-4258-985e-02b4d9509f18})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for BluetoothLEAppearance {
    type Vtable = IBluetoothLEAppearance_Vtbl;
    const IID: ::windows::core::GUID = <IBluetoothLEAppearance as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BluetoothLEAppearance {
    const NAME: &'static str = "Windows.Devices.Bluetooth.BluetoothLEAppearance";
}
impl ::core::convert::From<BluetoothLEAppearance> for ::windows::core::IUnknown {
    fn from(value: BluetoothLEAppearance) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BluetoothLEAppearance> for ::windows::core::IUnknown {
    fn from(value: &BluetoothLEAppearance) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BluetoothLEAppearance {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BluetoothLEAppearance {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BluetoothLEAppearance> for ::windows::core::IInspectable {
    fn from(value: BluetoothLEAppearance) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BluetoothLEAppearance> for ::windows::core::IInspectable {
    fn from(value: &BluetoothLEAppearance) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BluetoothLEAppearance {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BluetoothLEAppearance {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BluetoothLEAppearance {}
unsafe impl ::core::marker::Sync for BluetoothLEAppearance {}
#[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
pub struct BluetoothLEAppearanceCategories {}
impl BluetoothLEAppearanceCategories {
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn Uncategorized() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Uncategorized)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn Phone() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Phone)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn Computer() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Computer)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn Watch() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Watch)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn Clock() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Clock)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn Display() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Display)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn RemoteControl() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RemoteControl)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn EyeGlasses() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EyeGlasses)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn Tag() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Tag)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn Keyring() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Keyring)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn MediaPlayer() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MediaPlayer)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn BarcodeScanner() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BarcodeScanner)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn Thermometer() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Thermometer)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn HeartRate() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HeartRate)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn BloodPressure() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BloodPressure)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn HumanInterfaceDevice() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HumanInterfaceDevice)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn GlucoseMeter() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GlucoseMeter)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn RunningWalking() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RunningWalking)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn Cycling() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Cycling)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn PulseOximeter() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PulseOximeter)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn WeightScale() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WeightScale)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn OutdoorSportActivity() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OutdoorSportActivity)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IBluetoothLEAppearanceCategoriesStatics<R, F: FnOnce(&IBluetoothLEAppearanceCategoriesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BluetoothLEAppearanceCategories, IBluetoothLEAppearanceCategoriesStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for BluetoothLEAppearanceCategories {
    const NAME: &'static str = "Windows.Devices.Bluetooth.BluetoothLEAppearanceCategories";
}
#[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
pub struct BluetoothLEAppearanceSubcategories {}
impl BluetoothLEAppearanceSubcategories {
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn Generic() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Generic)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn SportsWatch() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SportsWatch)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn ThermometerEar() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ThermometerEar)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn HeartRateBelt() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HeartRateBelt)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn BloodPressureArm() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BloodPressureArm)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn BloodPressureWrist() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BloodPressureWrist)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn Keyboard() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Keyboard)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn Mouse() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Mouse)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn Joystick() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Joystick)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn Gamepad() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Gamepad)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn DigitizerTablet() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DigitizerTablet)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn CardReader() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CardReader)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn DigitalPen() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DigitalPen)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn BarcodeScanner() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BarcodeScanner)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn RunningWalkingInShoe() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RunningWalkingInShoe)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn RunningWalkingOnShoe() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RunningWalkingOnShoe)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn RunningWalkingOnHip() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RunningWalkingOnHip)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn CyclingComputer() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CyclingComputer)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn CyclingSpeedSensor() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CyclingSpeedSensor)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn CyclingCadenceSensor() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CyclingCadenceSensor)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn CyclingPowerSensor() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CyclingPowerSensor)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn CyclingSpeedCadenceSensor() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CyclingSpeedCadenceSensor)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn OximeterFingertip() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OximeterFingertip)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn OximeterWristWorn() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OximeterWristWorn)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn LocationDisplay() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LocationDisplay)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn LocationNavigationDisplay() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LocationNavigationDisplay)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn LocationPod() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LocationPod)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn LocationNavigationPod() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LocationNavigationPod)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IBluetoothLEAppearanceSubcategoriesStatics<R, F: FnOnce(&IBluetoothLEAppearanceSubcategoriesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BluetoothLEAppearanceSubcategories, IBluetoothLEAppearanceSubcategoriesStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for BluetoothLEAppearanceSubcategories {
    const NAME: &'static str = "Windows.Devices.Bluetooth.BluetoothLEAppearanceSubcategories";
}
#[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
#[repr(transparent)]
pub struct BluetoothLEConnectionParameters(::windows::core::IUnknown);
impl BluetoothLEConnectionParameters {
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn LinkTimeout(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LinkTimeout)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn ConnectionLatency(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ConnectionLatency)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn ConnectionInterval(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ConnectionInterval)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
}
impl ::core::clone::Clone for BluetoothLEConnectionParameters {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BluetoothLEConnectionParameters {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BluetoothLEConnectionParameters {}
impl ::core::fmt::Debug for BluetoothLEConnectionParameters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BluetoothLEConnectionParameters").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BluetoothLEConnectionParameters {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.BluetoothLEConnectionParameters;{33cb0771-8da9-508f-a366-1ca388c929ab})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for BluetoothLEConnectionParameters {
    type Vtable = IBluetoothLEConnectionParameters_Vtbl;
    const IID: ::windows::core::GUID = <IBluetoothLEConnectionParameters as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BluetoothLEConnectionParameters {
    const NAME: &'static str = "Windows.Devices.Bluetooth.BluetoothLEConnectionParameters";
}
impl ::core::convert::From<BluetoothLEConnectionParameters> for ::windows::core::IUnknown {
    fn from(value: BluetoothLEConnectionParameters) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BluetoothLEConnectionParameters> for ::windows::core::IUnknown {
    fn from(value: &BluetoothLEConnectionParameters) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BluetoothLEConnectionParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BluetoothLEConnectionParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BluetoothLEConnectionParameters> for ::windows::core::IInspectable {
    fn from(value: BluetoothLEConnectionParameters) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BluetoothLEConnectionParameters> for ::windows::core::IInspectable {
    fn from(value: &BluetoothLEConnectionParameters) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BluetoothLEConnectionParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BluetoothLEConnectionParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BluetoothLEConnectionParameters {}
unsafe impl ::core::marker::Sync for BluetoothLEConnectionParameters {}
#[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
#[repr(transparent)]
pub struct BluetoothLEConnectionPhy(::windows::core::IUnknown);
impl BluetoothLEConnectionPhy {
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn TransmitInfo(&self) -> ::windows::core::Result<BluetoothLEConnectionPhyInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TransmitInfo)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothLEConnectionPhyInfo>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn ReceiveInfo(&self) -> ::windows::core::Result<BluetoothLEConnectionPhyInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReceiveInfo)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothLEConnectionPhyInfo>(result__)
        }
    }
}
impl ::core::clone::Clone for BluetoothLEConnectionPhy {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BluetoothLEConnectionPhy {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BluetoothLEConnectionPhy {}
impl ::core::fmt::Debug for BluetoothLEConnectionPhy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BluetoothLEConnectionPhy").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BluetoothLEConnectionPhy {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.BluetoothLEConnectionPhy;{781e5e48-621e-5a7e-8be6-1b9561ff63c9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for BluetoothLEConnectionPhy {
    type Vtable = IBluetoothLEConnectionPhy_Vtbl;
    const IID: ::windows::core::GUID = <IBluetoothLEConnectionPhy as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BluetoothLEConnectionPhy {
    const NAME: &'static str = "Windows.Devices.Bluetooth.BluetoothLEConnectionPhy";
}
impl ::core::convert::From<BluetoothLEConnectionPhy> for ::windows::core::IUnknown {
    fn from(value: BluetoothLEConnectionPhy) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BluetoothLEConnectionPhy> for ::windows::core::IUnknown {
    fn from(value: &BluetoothLEConnectionPhy) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BluetoothLEConnectionPhy {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BluetoothLEConnectionPhy {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BluetoothLEConnectionPhy> for ::windows::core::IInspectable {
    fn from(value: BluetoothLEConnectionPhy) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BluetoothLEConnectionPhy> for ::windows::core::IInspectable {
    fn from(value: &BluetoothLEConnectionPhy) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BluetoothLEConnectionPhy {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BluetoothLEConnectionPhy {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BluetoothLEConnectionPhy {}
unsafe impl ::core::marker::Sync for BluetoothLEConnectionPhy {}
#[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
#[repr(transparent)]
pub struct BluetoothLEConnectionPhyInfo(::windows::core::IUnknown);
impl BluetoothLEConnectionPhyInfo {
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn IsUncoded1MPhy(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsUncoded1MPhy)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn IsUncoded2MPhy(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsUncoded2MPhy)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn IsCodedPhy(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsCodedPhy)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for BluetoothLEConnectionPhyInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BluetoothLEConnectionPhyInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BluetoothLEConnectionPhyInfo {}
impl ::core::fmt::Debug for BluetoothLEConnectionPhyInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BluetoothLEConnectionPhyInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BluetoothLEConnectionPhyInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.BluetoothLEConnectionPhyInfo;{9a100bdd-602e-5c27-a1ae-b230015a6394})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for BluetoothLEConnectionPhyInfo {
    type Vtable = IBluetoothLEConnectionPhyInfo_Vtbl;
    const IID: ::windows::core::GUID = <IBluetoothLEConnectionPhyInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BluetoothLEConnectionPhyInfo {
    const NAME: &'static str = "Windows.Devices.Bluetooth.BluetoothLEConnectionPhyInfo";
}
impl ::core::convert::From<BluetoothLEConnectionPhyInfo> for ::windows::core::IUnknown {
    fn from(value: BluetoothLEConnectionPhyInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BluetoothLEConnectionPhyInfo> for ::windows::core::IUnknown {
    fn from(value: &BluetoothLEConnectionPhyInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BluetoothLEConnectionPhyInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BluetoothLEConnectionPhyInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BluetoothLEConnectionPhyInfo> for ::windows::core::IInspectable {
    fn from(value: BluetoothLEConnectionPhyInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BluetoothLEConnectionPhyInfo> for ::windows::core::IInspectable {
    fn from(value: &BluetoothLEConnectionPhyInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BluetoothLEConnectionPhyInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BluetoothLEConnectionPhyInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BluetoothLEConnectionPhyInfo {}
unsafe impl ::core::marker::Sync for BluetoothLEConnectionPhyInfo {}
#[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
#[repr(transparent)]
pub struct BluetoothLEDevice(::windows::core::IUnknown);
impl BluetoothLEDevice {
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Name)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`, `\"Devices_Bluetooth_GenericAttributeProfile\"`, `\"Foundation_Collections\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn GattServices(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<GenericAttributeProfile::GattDeviceService>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GattServices)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<GenericAttributeProfile::GattDeviceService>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn ConnectionStatus(&self) -> ::windows::core::Result<BluetoothConnectionStatus> {
        let this = self;
        unsafe {
            let mut result__: BluetoothConnectionStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ConnectionStatus)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothConnectionStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn BluetoothAddress(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BluetoothAddress)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`, `\"Devices_Bluetooth_GenericAttributeProfile\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "deprecated"))]
    pub fn GetGattService<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, serviceuuid: Param0) -> ::windows::core::Result<GenericAttributeProfile::GattDeviceService> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetGattService)(::core::mem::transmute_copy(this), serviceuuid.into_param().abi(), &mut result__).from_abi::<GenericAttributeProfile::GattDeviceService>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn NameChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<BluetoothLEDevice, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NameChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveNameChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveNameChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GattServicesChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<BluetoothLEDevice, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GattServicesChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveGattServicesChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveGattServicesChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ConnectionStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<BluetoothLEDevice, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ConnectionStatusChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveConnectionStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveConnectionStatusChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`, `\"Devices_Enumeration\"`*"]
    #[cfg(feature = "Devices_Enumeration")]
    pub fn DeviceInformation(&self) -> ::windows::core::Result<super::Enumeration::DeviceInformation> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEDevice2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceInformation)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Enumeration::DeviceInformation>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn Appearance(&self) -> ::windows::core::Result<BluetoothLEAppearance> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEDevice2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Appearance)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothLEAppearance>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn BluetoothAddressType(&self) -> ::windows::core::Result<BluetoothAddressType> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEDevice2>(self)?;
        unsafe {
            let mut result__: BluetoothAddressType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BluetoothAddressType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothAddressType>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`, `\"Devices_Enumeration\"`*"]
    #[cfg(feature = "Devices_Enumeration")]
    pub fn DeviceAccessInformation(&self) -> ::windows::core::Result<super::Enumeration::DeviceAccessInformation> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEDevice3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceAccessInformation)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Enumeration::DeviceAccessInformation>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`, `\"Devices_Enumeration\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))]
    pub fn RequestAccessAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::Enumeration::DeviceAccessStatus>> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEDevice3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestAccessAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::Enumeration::DeviceAccessStatus>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`, `\"Devices_Bluetooth_GenericAttributeProfile\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation"))]
    pub fn GetGattServicesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<GenericAttributeProfile::GattDeviceServicesResult>> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEDevice3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetGattServicesAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<GenericAttributeProfile::GattDeviceServicesResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`, `\"Devices_Bluetooth_GenericAttributeProfile\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation"))]
    pub fn GetGattServicesWithCacheModeAsync(&self, cachemode: BluetoothCacheMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<GenericAttributeProfile::GattDeviceServicesResult>> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEDevice3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetGattServicesWithCacheModeAsync)(::core::mem::transmute_copy(this), cachemode, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<GenericAttributeProfile::GattDeviceServicesResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`, `\"Devices_Bluetooth_GenericAttributeProfile\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation"))]
    pub fn GetGattServicesForUuidAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, serviceuuid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<GenericAttributeProfile::GattDeviceServicesResult>> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEDevice3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetGattServicesForUuidAsync)(::core::mem::transmute_copy(this), serviceuuid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<GenericAttributeProfile::GattDeviceServicesResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`, `\"Devices_Bluetooth_GenericAttributeProfile\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation"))]
    pub fn GetGattServicesForUuidWithCacheModeAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, serviceuuid: Param0, cachemode: BluetoothCacheMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<GenericAttributeProfile::GattDeviceServicesResult>> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEDevice3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetGattServicesForUuidWithCacheModeAsync)(::core::mem::transmute_copy(this), serviceuuid.into_param().abi(), cachemode, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<GenericAttributeProfile::GattDeviceServicesResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn BluetoothDeviceId(&self) -> ::windows::core::Result<BluetoothDeviceId> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEDevice4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BluetoothDeviceId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothDeviceId>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn WasSecureConnectionUsedForPairing(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEDevice5>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WasSecureConnectionUsedForPairing)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn GetConnectionParameters(&self) -> ::windows::core::Result<BluetoothLEConnectionParameters> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEDevice6>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetConnectionParameters)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothLEConnectionParameters>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn GetConnectionPhy(&self) -> ::windows::core::Result<BluetoothLEConnectionPhy> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEDevice6>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetConnectionPhy)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothLEConnectionPhy>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn RequestPreferredConnectionParameters<'a, Param0: ::windows::core::IntoParam<'a, BluetoothLEPreferredConnectionParameters>>(&self, preferredconnectionparameters: Param0) -> ::windows::core::Result<BluetoothLEPreferredConnectionParametersRequest> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEDevice6>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestPreferredConnectionParameters)(::core::mem::transmute_copy(this), preferredconnectionparameters.into_param().abi(), &mut result__).from_abi::<BluetoothLEPreferredConnectionParametersRequest>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ConnectionParametersChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<BluetoothLEDevice, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEDevice6>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ConnectionParametersChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveConnectionParametersChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEDevice6>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveConnectionParametersChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ConnectionPhyChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<BluetoothLEDevice, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEDevice6>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ConnectionPhyChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveConnectionPhyChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEDevice6>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveConnectionPhyChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BluetoothLEDevice>> {
        Self::IBluetoothLEDeviceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FromIdAsync)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<BluetoothLEDevice>>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromBluetoothAddressAsync(bluetoothaddress: u64) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BluetoothLEDevice>> {
        Self::IBluetoothLEDeviceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FromBluetoothAddressAsync)(::core::mem::transmute_copy(this), bluetoothaddress, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<BluetoothLEDevice>>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IBluetoothLEDeviceStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeviceSelector)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn GetDeviceSelectorFromPairingState(pairingstate: bool) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IBluetoothLEDeviceStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeviceSelectorFromPairingState)(::core::mem::transmute_copy(this), pairingstate, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn GetDeviceSelectorFromConnectionStatus(connectionstatus: BluetoothConnectionStatus) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IBluetoothLEDeviceStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeviceSelectorFromConnectionStatus)(::core::mem::transmute_copy(this), connectionstatus, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn GetDeviceSelectorFromDeviceName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(devicename: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IBluetoothLEDeviceStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeviceSelectorFromDeviceName)(::core::mem::transmute_copy(this), devicename.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn GetDeviceSelectorFromBluetoothAddress(bluetoothaddress: u64) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IBluetoothLEDeviceStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeviceSelectorFromBluetoothAddress)(::core::mem::transmute_copy(this), bluetoothaddress, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn GetDeviceSelectorFromBluetoothAddressWithBluetoothAddressType(bluetoothaddress: u64, bluetoothaddresstype: BluetoothAddressType) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IBluetoothLEDeviceStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeviceSelectorFromBluetoothAddressWithBluetoothAddressType)(::core::mem::transmute_copy(this), bluetoothaddress, bluetoothaddresstype, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn GetDeviceSelectorFromAppearance<'a, Param0: ::windows::core::IntoParam<'a, BluetoothLEAppearance>>(appearance: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IBluetoothLEDeviceStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeviceSelectorFromAppearance)(::core::mem::transmute_copy(this), appearance.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromBluetoothAddressWithBluetoothAddressTypeAsync(bluetoothaddress: u64, bluetoothaddresstype: BluetoothAddressType) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BluetoothLEDevice>> {
        Self::IBluetoothLEDeviceStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FromBluetoothAddressWithBluetoothAddressTypeAsync)(::core::mem::transmute_copy(this), bluetoothaddress, bluetoothaddresstype, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<BluetoothLEDevice>>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc(hidden)]
    pub fn IBluetoothLEDeviceStatics<R, F: FnOnce(&IBluetoothLEDeviceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BluetoothLEDevice, IBluetoothLEDeviceStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IBluetoothLEDeviceStatics2<R, F: FnOnce(&IBluetoothLEDeviceStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BluetoothLEDevice, IBluetoothLEDeviceStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for BluetoothLEDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BluetoothLEDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BluetoothLEDevice {}
impl ::core::fmt::Debug for BluetoothLEDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BluetoothLEDevice").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BluetoothLEDevice {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.BluetoothLEDevice;{b5ee2f7b-4ad8-4642-ac48-80a0b500e887})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for BluetoothLEDevice {
    type Vtable = IBluetoothLEDevice_Vtbl;
    const IID: ::windows::core::GUID = <IBluetoothLEDevice as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BluetoothLEDevice {
    const NAME: &'static str = "Windows.Devices.Bluetooth.BluetoothLEDevice";
}
impl ::core::convert::From<BluetoothLEDevice> for ::windows::core::IUnknown {
    fn from(value: BluetoothLEDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BluetoothLEDevice> for ::windows::core::IUnknown {
    fn from(value: &BluetoothLEDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BluetoothLEDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BluetoothLEDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BluetoothLEDevice> for ::windows::core::IInspectable {
    fn from(value: BluetoothLEDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BluetoothLEDevice> for ::windows::core::IInspectable {
    fn from(value: &BluetoothLEDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BluetoothLEDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BluetoothLEDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
#[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
#[repr(transparent)]
pub struct BluetoothLEPreferredConnectionParameters(::windows::core::IUnknown);
impl BluetoothLEPreferredConnectionParameters {
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn LinkTimeout(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LinkTimeout)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn ConnectionLatency(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ConnectionLatency)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn MinConnectionInterval(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MinConnectionInterval)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn MaxConnectionInterval(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MaxConnectionInterval)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn Balanced() -> ::windows::core::Result<BluetoothLEPreferredConnectionParameters> {
        Self::IBluetoothLEPreferredConnectionParametersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Balanced)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothLEPreferredConnectionParameters>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn ThroughputOptimized() -> ::windows::core::Result<BluetoothLEPreferredConnectionParameters> {
        Self::IBluetoothLEPreferredConnectionParametersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ThroughputOptimized)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothLEPreferredConnectionParameters>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn PowerOptimized() -> ::windows::core::Result<BluetoothLEPreferredConnectionParameters> {
        Self::IBluetoothLEPreferredConnectionParametersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PowerOptimized)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothLEPreferredConnectionParameters>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IBluetoothLEPreferredConnectionParametersStatics<R, F: FnOnce(&IBluetoothLEPreferredConnectionParametersStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BluetoothLEPreferredConnectionParameters, IBluetoothLEPreferredConnectionParametersStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for BluetoothLEPreferredConnectionParameters {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BluetoothLEPreferredConnectionParameters {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BluetoothLEPreferredConnectionParameters {}
impl ::core::fmt::Debug for BluetoothLEPreferredConnectionParameters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BluetoothLEPreferredConnectionParameters").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BluetoothLEPreferredConnectionParameters {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.BluetoothLEPreferredConnectionParameters;{f2f44344-7372-5f7b-9b34-29c944f5a715})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for BluetoothLEPreferredConnectionParameters {
    type Vtable = IBluetoothLEPreferredConnectionParameters_Vtbl;
    const IID: ::windows::core::GUID = <IBluetoothLEPreferredConnectionParameters as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BluetoothLEPreferredConnectionParameters {
    const NAME: &'static str = "Windows.Devices.Bluetooth.BluetoothLEPreferredConnectionParameters";
}
impl ::core::convert::From<BluetoothLEPreferredConnectionParameters> for ::windows::core::IUnknown {
    fn from(value: BluetoothLEPreferredConnectionParameters) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BluetoothLEPreferredConnectionParameters> for ::windows::core::IUnknown {
    fn from(value: &BluetoothLEPreferredConnectionParameters) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BluetoothLEPreferredConnectionParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BluetoothLEPreferredConnectionParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BluetoothLEPreferredConnectionParameters> for ::windows::core::IInspectable {
    fn from(value: BluetoothLEPreferredConnectionParameters) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BluetoothLEPreferredConnectionParameters> for ::windows::core::IInspectable {
    fn from(value: &BluetoothLEPreferredConnectionParameters) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BluetoothLEPreferredConnectionParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BluetoothLEPreferredConnectionParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BluetoothLEPreferredConnectionParameters {}
unsafe impl ::core::marker::Sync for BluetoothLEPreferredConnectionParameters {}
#[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
#[repr(transparent)]
pub struct BluetoothLEPreferredConnectionParametersRequest(::windows::core::IUnknown);
impl BluetoothLEPreferredConnectionParametersRequest {
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<BluetoothLEPreferredConnectionParametersRequestStatus> {
        let this = self;
        unsafe {
            let mut result__: BluetoothLEPreferredConnectionParametersRequestStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothLEPreferredConnectionParametersRequestStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for BluetoothLEPreferredConnectionParametersRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BluetoothLEPreferredConnectionParametersRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BluetoothLEPreferredConnectionParametersRequest {}
impl ::core::fmt::Debug for BluetoothLEPreferredConnectionParametersRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BluetoothLEPreferredConnectionParametersRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BluetoothLEPreferredConnectionParametersRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.BluetoothLEPreferredConnectionParametersRequest;{8a375276-a528-5266-b661-cce6a5ff9739})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for BluetoothLEPreferredConnectionParametersRequest {
    type Vtable = IBluetoothLEPreferredConnectionParametersRequest_Vtbl;
    const IID: ::windows::core::GUID = <IBluetoothLEPreferredConnectionParametersRequest as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BluetoothLEPreferredConnectionParametersRequest {
    const NAME: &'static str = "Windows.Devices.Bluetooth.BluetoothLEPreferredConnectionParametersRequest";
}
impl ::core::convert::From<BluetoothLEPreferredConnectionParametersRequest> for ::windows::core::IUnknown {
    fn from(value: BluetoothLEPreferredConnectionParametersRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BluetoothLEPreferredConnectionParametersRequest> for ::windows::core::IUnknown {
    fn from(value: &BluetoothLEPreferredConnectionParametersRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BluetoothLEPreferredConnectionParametersRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BluetoothLEPreferredConnectionParametersRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BluetoothLEPreferredConnectionParametersRequest> for ::windows::core::IInspectable {
    fn from(value: BluetoothLEPreferredConnectionParametersRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BluetoothLEPreferredConnectionParametersRequest> for ::windows::core::IInspectable {
    fn from(value: &BluetoothLEPreferredConnectionParametersRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BluetoothLEPreferredConnectionParametersRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BluetoothLEPreferredConnectionParametersRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
#[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct BluetoothLEPreferredConnectionParametersRequestStatus(pub i32);
impl BluetoothLEPreferredConnectionParametersRequestStatus {
    pub const Unspecified: Self = Self(0i32);
    pub const Success: Self = Self(1i32);
    pub const DeviceNotAvailable: Self = Self(2i32);
    pub const AccessDenied: Self = Self(3i32);
}
impl ::core::marker::Copy for BluetoothLEPreferredConnectionParametersRequestStatus {}
impl ::core::clone::Clone for BluetoothLEPreferredConnectionParametersRequestStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BluetoothLEPreferredConnectionParametersRequestStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for BluetoothLEPreferredConnectionParametersRequestStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for BluetoothLEPreferredConnectionParametersRequestStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BluetoothLEPreferredConnectionParametersRequestStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BluetoothLEPreferredConnectionParametersRequestStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.BluetoothLEPreferredConnectionParametersRequestStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct BluetoothMajorClass(pub i32);
impl BluetoothMajorClass {
    pub const Miscellaneous: Self = Self(0i32);
    pub const Computer: Self = Self(1i32);
    pub const Phone: Self = Self(2i32);
    pub const NetworkAccessPoint: Self = Self(3i32);
    pub const AudioVideo: Self = Self(4i32);
    pub const Peripheral: Self = Self(5i32);
    pub const Imaging: Self = Self(6i32);
    pub const Wearable: Self = Self(7i32);
    pub const Toy: Self = Self(8i32);
    pub const Health: Self = Self(9i32);
}
impl ::core::marker::Copy for BluetoothMajorClass {}
impl ::core::clone::Clone for BluetoothMajorClass {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BluetoothMajorClass {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for BluetoothMajorClass {
    type Abi = Self;
}
impl ::core::fmt::Debug for BluetoothMajorClass {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BluetoothMajorClass").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BluetoothMajorClass {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.BluetoothMajorClass;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct BluetoothMinorClass(pub i32);
impl BluetoothMinorClass {
    pub const Uncategorized: Self = Self(0i32);
    pub const ComputerDesktop: Self = Self(1i32);
    pub const ComputerServer: Self = Self(2i32);
    pub const ComputerLaptop: Self = Self(3i32);
    pub const ComputerHandheld: Self = Self(4i32);
    pub const ComputerPalmSize: Self = Self(5i32);
    pub const ComputerWearable: Self = Self(6i32);
    pub const ComputerTablet: Self = Self(7i32);
    pub const PhoneCellular: Self = Self(1i32);
    pub const PhoneCordless: Self = Self(2i32);
    pub const PhoneSmartPhone: Self = Self(3i32);
    pub const PhoneWired: Self = Self(4i32);
    pub const PhoneIsdn: Self = Self(5i32);
    pub const NetworkFullyAvailable: Self = Self(0i32);
    pub const NetworkUsed01To17Percent: Self = Self(8i32);
    pub const NetworkUsed17To33Percent: Self = Self(16i32);
    pub const NetworkUsed33To50Percent: Self = Self(24i32);
    pub const NetworkUsed50To67Percent: Self = Self(32i32);
    pub const NetworkUsed67To83Percent: Self = Self(40i32);
    pub const NetworkUsed83To99Percent: Self = Self(48i32);
    pub const NetworkNoServiceAvailable: Self = Self(56i32);
    pub const AudioVideoWearableHeadset: Self = Self(1i32);
    pub const AudioVideoHandsFree: Self = Self(2i32);
    pub const AudioVideoMicrophone: Self = Self(4i32);
    pub const AudioVideoLoudspeaker: Self = Self(5i32);
    pub const AudioVideoHeadphones: Self = Self(6i32);
    pub const AudioVideoPortableAudio: Self = Self(7i32);
    pub const AudioVideoCarAudio: Self = Self(8i32);
    pub const AudioVideoSetTopBox: Self = Self(9i32);
    pub const AudioVideoHifiAudioDevice: Self = Self(10i32);
    pub const AudioVideoVcr: Self = Self(11i32);
    pub const AudioVideoVideoCamera: Self = Self(12i32);
    pub const AudioVideoCamcorder: Self = Self(13i32);
    pub const AudioVideoVideoMonitor: Self = Self(14i32);
    pub const AudioVideoVideoDisplayAndLoudspeaker: Self = Self(15i32);
    pub const AudioVideoVideoConferencing: Self = Self(16i32);
    pub const AudioVideoGamingOrToy: Self = Self(18i32);
    pub const PeripheralJoystick: Self = Self(1i32);
    pub const PeripheralGamepad: Self = Self(2i32);
    pub const PeripheralRemoteControl: Self = Self(3i32);
    pub const PeripheralSensing: Self = Self(4i32);
    pub const PeripheralDigitizerTablet: Self = Self(5i32);
    pub const PeripheralCardReader: Self = Self(6i32);
    pub const PeripheralDigitalPen: Self = Self(7i32);
    pub const PeripheralHandheldScanner: Self = Self(8i32);
    pub const PeripheralHandheldGesture: Self = Self(9i32);
    pub const WearableWristwatch: Self = Self(1i32);
    pub const WearablePager: Self = Self(2i32);
    pub const WearableJacket: Self = Self(3i32);
    pub const WearableHelmet: Self = Self(4i32);
    pub const WearableGlasses: Self = Self(5i32);
    pub const ToyRobot: Self = Self(1i32);
    pub const ToyVehicle: Self = Self(2i32);
    pub const ToyDoll: Self = Self(3i32);
    pub const ToyController: Self = Self(4i32);
    pub const ToyGame: Self = Self(5i32);
    pub const HealthBloodPressureMonitor: Self = Self(1i32);
    pub const HealthThermometer: Self = Self(2i32);
    pub const HealthWeighingScale: Self = Self(3i32);
    pub const HealthGlucoseMeter: Self = Self(4i32);
    pub const HealthPulseOximeter: Self = Self(5i32);
    pub const HealthHeartRateMonitor: Self = Self(6i32);
    pub const HealthHealthDataDisplay: Self = Self(7i32);
    pub const HealthStepCounter: Self = Self(8i32);
    pub const HealthBodyCompositionAnalyzer: Self = Self(9i32);
    pub const HealthPeakFlowMonitor: Self = Self(10i32);
    pub const HealthMedicationMonitor: Self = Self(11i32);
    pub const HealthKneeProsthesis: Self = Self(12i32);
    pub const HealthAnkleProsthesis: Self = Self(13i32);
    pub const HealthGenericHealthManager: Self = Self(14i32);
    pub const HealthPersonalMobilityDevice: Self = Self(15i32);
}
impl ::core::marker::Copy for BluetoothMinorClass {}
impl ::core::clone::Clone for BluetoothMinorClass {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BluetoothMinorClass {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for BluetoothMinorClass {
    type Abi = Self;
}
impl ::core::fmt::Debug for BluetoothMinorClass {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BluetoothMinorClass").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BluetoothMinorClass {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.BluetoothMinorClass;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct BluetoothServiceCapabilities(pub u32);
impl BluetoothServiceCapabilities {
    pub const None: Self = Self(0u32);
    pub const LimitedDiscoverableMode: Self = Self(1u32);
    pub const PositioningService: Self = Self(8u32);
    pub const NetworkingService: Self = Self(16u32);
    pub const RenderingService: Self = Self(32u32);
    pub const CapturingService: Self = Self(64u32);
    pub const ObjectTransferService: Self = Self(128u32);
    pub const AudioService: Self = Self(256u32);
    pub const TelephoneService: Self = Self(512u32);
    pub const InformationService: Self = Self(1024u32);
}
impl ::core::marker::Copy for BluetoothServiceCapabilities {}
impl ::core::clone::Clone for BluetoothServiceCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BluetoothServiceCapabilities {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for BluetoothServiceCapabilities {
    type Abi = Self;
}
impl ::core::fmt::Debug for BluetoothServiceCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BluetoothServiceCapabilities").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for BluetoothServiceCapabilities {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for BluetoothServiceCapabilities {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for BluetoothServiceCapabilities {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for BluetoothServiceCapabilities {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for BluetoothServiceCapabilities {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for BluetoothServiceCapabilities {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.BluetoothServiceCapabilities;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
#[repr(transparent)]
pub struct BluetoothSignalStrengthFilter(::windows::core::IUnknown);
impl BluetoothSignalStrengthFilter {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BluetoothSignalStrengthFilter, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn InRangeThresholdInDBm(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i16>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InRangeThresholdInDBm)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i16>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetInRangeThresholdInDBm<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<i16>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetInRangeThresholdInDBm)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OutOfRangeThresholdInDBm(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i16>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OutOfRangeThresholdInDBm)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i16>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetOutOfRangeThresholdInDBm<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<i16>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetOutOfRangeThresholdInDBm)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OutOfRangeTimeout(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OutOfRangeTimeout)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetOutOfRangeTimeout<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetOutOfRangeTimeout)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SamplingInterval(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SamplingInterval)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetSamplingInterval<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSamplingInterval)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for BluetoothSignalStrengthFilter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BluetoothSignalStrengthFilter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BluetoothSignalStrengthFilter {}
impl ::core::fmt::Debug for BluetoothSignalStrengthFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BluetoothSignalStrengthFilter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BluetoothSignalStrengthFilter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.BluetoothSignalStrengthFilter;{df7b7391-6bb5-4cfe-90b1-5d7324edcf7f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for BluetoothSignalStrengthFilter {
    type Vtable = IBluetoothSignalStrengthFilter_Vtbl;
    const IID: ::windows::core::GUID = <IBluetoothSignalStrengthFilter as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BluetoothSignalStrengthFilter {
    const NAME: &'static str = "Windows.Devices.Bluetooth.BluetoothSignalStrengthFilter";
}
impl ::core::convert::From<BluetoothSignalStrengthFilter> for ::windows::core::IUnknown {
    fn from(value: BluetoothSignalStrengthFilter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BluetoothSignalStrengthFilter> for ::windows::core::IUnknown {
    fn from(value: &BluetoothSignalStrengthFilter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BluetoothSignalStrengthFilter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BluetoothSignalStrengthFilter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BluetoothSignalStrengthFilter> for ::windows::core::IInspectable {
    fn from(value: BluetoothSignalStrengthFilter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BluetoothSignalStrengthFilter> for ::windows::core::IInspectable {
    fn from(value: &BluetoothSignalStrengthFilter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BluetoothSignalStrengthFilter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BluetoothSignalStrengthFilter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BluetoothSignalStrengthFilter {}
unsafe impl ::core::marker::Sync for BluetoothSignalStrengthFilter {}
#[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
pub struct BluetoothUuidHelper {}
impl BluetoothUuidHelper {
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    pub fn FromShortId(shortid: u32) -> ::windows::core::Result<::windows::core::GUID> {
        Self::IBluetoothUuidHelperStatics(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FromShortId)(::core::mem::transmute_copy(this), shortid, &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryGetShortId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(uuid: Param0) -> ::windows::core::Result<super::super::Foundation::IReference<u32>> {
        Self::IBluetoothUuidHelperStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryGetShortId)(::core::mem::transmute_copy(this), uuid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IReference<u32>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IBluetoothUuidHelperStatics<R, F: FnOnce(&IBluetoothUuidHelperStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BluetoothUuidHelper, IBluetoothUuidHelperStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for BluetoothUuidHelper {
    const NAME: &'static str = "Windows.Devices.Bluetooth.BluetoothUuidHelper";
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothAdapter(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothAdapter {
    type Vtable = IBluetoothAdapter_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7974f04c_5f7a_4a34_9225_a855f84b1a8b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothAdapter_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub BluetoothAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub IsClassicSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsLowEnergySupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsPeripheralRoleSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsCentralRoleSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsAdvertisementOffloadSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Devices_Radios", feature = "Foundation"))]
    pub GetRadioAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Radios", feature = "Foundation")))]
    GetRadioAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothAdapter2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothAdapter2 {
    type Vtable = IBluetoothAdapter2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xac94cecc_24d5_41b3_916d_1097c50b102b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothAdapter2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub AreClassicSecureConnectionsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub AreLowEnergySecureConnectionsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothAdapter3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothAdapter3 {
    type Vtable = IBluetoothAdapter3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f8624e0_cba9_5211_9f89_3aac62b4c6b8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothAdapter3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsExtendedAdvertisingSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub MaxAdvertisementDataLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothAdapterStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothAdapterStatics {
    type Vtable = IBluetoothAdapterStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8b02fb6a_ac4c_4741_8661_8eab7d17ea9f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothAdapterStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetDefaultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDefaultAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothClassOfDevice(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothClassOfDevice {
    type Vtable = IBluetoothClassOfDevice_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd640227e_d7d7_4661_9454_65039ca17a2b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothClassOfDevice_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub RawValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub MajorClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BluetoothMajorClass) -> ::windows::core::HRESULT,
    pub MinorClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BluetoothMinorClass) -> ::windows::core::HRESULT,
    pub ServiceCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BluetoothServiceCapabilities) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothClassOfDeviceStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothClassOfDeviceStatics {
    type Vtable = IBluetoothClassOfDeviceStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe46135bd_0fa2_416c_91b4_c1e48ca061c1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothClassOfDeviceStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub FromRawValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rawvalue: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub FromParts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, majorclass: BluetoothMajorClass, minorclass: BluetoothMinorClass, servicecapabilities: BluetoothServiceCapabilities, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothDevice(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothDevice {
    type Vtable = IBluetoothDevice_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2335b156_90d2_4a04_aef5_0e20b9e6b707);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothDevice_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Networking")]
    pub HostName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Networking"))]
    HostName: usize,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ClassOfDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub SdpRecords: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    SdpRecords: usize,
    #[cfg(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Foundation_Collections", feature = "deprecated"))]
    pub RfcommServices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Foundation_Collections", feature = "deprecated")))]
    RfcommServices: usize,
    pub ConnectionStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BluetoothConnectionStatus) -> ::windows::core::HRESULT,
    pub BluetoothAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub NameChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NameChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveNameChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveNameChanged: usize,
    #[cfg(feature = "Foundation")]
    pub SdpRecordsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SdpRecordsChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSdpRecordsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSdpRecordsChanged: usize,
    #[cfg(feature = "Foundation")]
    pub ConnectionStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConnectionStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveConnectionStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveConnectionStatusChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothDevice2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothDevice2 {
    type Vtable = IBluetoothDevice2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0133f954_b156_4dd0_b1f5_c11bc31a5163);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothDevice2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Devices_Enumeration")]
    pub DeviceInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    DeviceInformation: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothDevice3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothDevice3 {
    type Vtable = IBluetoothDevice3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x57fff78b_651a_4454_b90f_eb21ef0b0d71);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothDevice3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Devices_Enumeration")]
    pub DeviceAccessInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    DeviceAccessInformation: usize,
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))]
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Enumeration", feature = "Foundation")))]
    RequestAccessAsync: usize,
    #[cfg(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Foundation"))]
    pub GetRfcommServicesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Foundation")))]
    GetRfcommServicesAsync: usize,
    #[cfg(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Foundation"))]
    pub GetRfcommServicesWithCacheModeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cachemode: BluetoothCacheMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Foundation")))]
    GetRfcommServicesWithCacheModeAsync: usize,
    #[cfg(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Foundation"))]
    pub GetRfcommServicesForIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, serviceid: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Foundation")))]
    GetRfcommServicesForIdAsync: usize,
    #[cfg(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Foundation"))]
    pub GetRfcommServicesForIdWithCacheModeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, serviceid: ::windows::core::RawPtr, cachemode: BluetoothCacheMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Foundation")))]
    GetRfcommServicesForIdWithCacheModeAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothDevice4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothDevice4 {
    type Vtable = IBluetoothDevice4_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x817c34ad_0e9c_42b2_a8dc_3e8094940d12);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothDevice4_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub BluetoothDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothDevice5(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothDevice5 {
    type Vtable = IBluetoothDevice5_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5e0b385_5e85_4559_a10d_1c7281379f96);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothDevice5_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub WasSecureConnectionUsedForPairing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothDeviceId(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothDeviceId {
    type Vtable = IBluetoothDeviceId_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc17949af_57c1_4642_bcce_e6c06b20ae76);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothDeviceId_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IsClassicDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsLowEnergyDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothDeviceIdStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothDeviceIdStatics {
    type Vtable = IBluetoothDeviceIdStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa7884e67_3efb_4f31_bbc2_810e09977404);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothDeviceIdStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub FromId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothDeviceStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothDeviceStatics {
    type Vtable = IBluetoothDeviceStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0991df51_57db_4725_bbd7_84f64327ec2c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothDeviceStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Networking"))]
    pub FromHostNameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hostname: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Networking")))]
    FromHostNameAsync: usize,
    #[cfg(feature = "Foundation")]
    pub FromBluetoothAddressAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, address: u64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromBluetoothAddressAsync: usize,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothDeviceStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothDeviceStatics2 {
    type Vtable = IBluetoothDeviceStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc29e8e2f_4e14_4477_aa1b_b8b47e5b7ece);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothDeviceStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetDeviceSelectorFromPairingState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pairingstate: bool, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetDeviceSelectorFromConnectionStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connectionstatus: BluetoothConnectionStatus, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetDeviceSelectorFromDeviceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, devicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetDeviceSelectorFromBluetoothAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bluetoothaddress: u64, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetDeviceSelectorFromClassOfDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, classofdevice: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEAppearance(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothLEAppearance {
    type Vtable = IBluetoothLEAppearance_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d2079f2_66a8_4258_985e_02b4d9509f18);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAppearance_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub RawValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub Category: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub SubCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEAppearanceCategoriesStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothLEAppearanceCategoriesStatics {
    type Vtable = IBluetoothLEAppearanceCategoriesStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d4d54fe_046a_4185_aab6_824cf0610861);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAppearanceCategoriesStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Uncategorized: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub Phone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub Computer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub Watch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub Clock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub Display: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub RemoteControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub EyeGlasses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub Tag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub Keyring: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub MediaPlayer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub BarcodeScanner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub Thermometer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub HeartRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub BloodPressure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub HumanInterfaceDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub GlucoseMeter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub RunningWalking: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub Cycling: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub PulseOximeter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub WeightScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub OutdoorSportActivity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEAppearanceStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothLEAppearanceStatics {
    type Vtable = IBluetoothLEAppearanceStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa193c0c7_4504_4f4a_9ba5_cd1054e5e065);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAppearanceStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub FromRawValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rawvalue: u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub FromParts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appearancecategory: u16, appearancesubcategory: u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEAppearanceSubcategoriesStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothLEAppearanceSubcategoriesStatics {
    type Vtable = IBluetoothLEAppearanceSubcategoriesStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe57ba606_2144_415a_8312_71ccf291f8d1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAppearanceSubcategoriesStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Generic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub SportsWatch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub ThermometerEar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub HeartRateBelt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub BloodPressureArm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub BloodPressureWrist: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub Keyboard: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub Mouse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub Joystick: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub Gamepad: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub DigitizerTablet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub CardReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub DigitalPen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub BarcodeScanner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub RunningWalkingInShoe: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub RunningWalkingOnShoe: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub RunningWalkingOnHip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub CyclingComputer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub CyclingSpeedSensor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub CyclingCadenceSensor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub CyclingPowerSensor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub CyclingSpeedCadenceSensor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub OximeterFingertip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub OximeterWristWorn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub LocationDisplay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub LocationNavigationDisplay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub LocationPod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub LocationNavigationPod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEConnectionParameters(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothLEConnectionParameters {
    type Vtable = IBluetoothLEConnectionParameters_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33cb0771_8da9_508f_a366_1ca388c929ab);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEConnectionParameters_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub LinkTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub ConnectionLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub ConnectionInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEConnectionPhy(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothLEConnectionPhy {
    type Vtable = IBluetoothLEConnectionPhy_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x781e5e48_621e_5a7e_8be6_1b9561ff63c9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEConnectionPhy_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub TransmitInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub ReceiveInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEConnectionPhyInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothLEConnectionPhyInfo {
    type Vtable = IBluetoothLEConnectionPhyInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a100bdd_602e_5c27_a1ae_b230015a6394);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEConnectionPhyInfo_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsUncoded1MPhy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsUncoded2MPhy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsCodedPhy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEDevice(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothLEDevice {
    type Vtable = IBluetoothLEDevice_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5ee2f7b_4ad8_4642_ac48_80a0b500e887);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEDevice_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation_Collections", feature = "deprecated"))]
    pub GattServices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation_Collections", feature = "deprecated")))]
    GattServices: usize,
    pub ConnectionStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BluetoothConnectionStatus) -> ::windows::core::HRESULT,
    pub BluetoothAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "deprecated"))]
    pub GetGattService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, serviceuuid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "deprecated")))]
    GetGattService: usize,
    #[cfg(feature = "Foundation")]
    pub NameChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NameChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveNameChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveNameChanged: usize,
    #[cfg(feature = "Foundation")]
    pub GattServicesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GattServicesChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveGattServicesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveGattServicesChanged: usize,
    #[cfg(feature = "Foundation")]
    pub ConnectionStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConnectionStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveConnectionStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveConnectionStatusChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEDevice2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothLEDevice2 {
    type Vtable = IBluetoothLEDevice2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x26f062b3_7aee_4d31_baba_b1b9775f5916);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEDevice2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Devices_Enumeration")]
    pub DeviceInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    DeviceInformation: usize,
    pub Appearance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub BluetoothAddressType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BluetoothAddressType) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEDevice3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothLEDevice3 {
    type Vtable = IBluetoothLEDevice3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaee9e493_44ac_40dc_af33_b2c13c01ca46);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEDevice3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Devices_Enumeration")]
    pub DeviceAccessInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    DeviceAccessInformation: usize,
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))]
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Enumeration", feature = "Foundation")))]
    RequestAccessAsync: usize,
    #[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation"))]
    pub GetGattServicesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation")))]
    GetGattServicesAsync: usize,
    #[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation"))]
    pub GetGattServicesWithCacheModeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cachemode: BluetoothCacheMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation")))]
    GetGattServicesWithCacheModeAsync: usize,
    #[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation"))]
    pub GetGattServicesForUuidAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, serviceuuid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation")))]
    GetGattServicesForUuidAsync: usize,
    #[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation"))]
    pub GetGattServicesForUuidWithCacheModeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, serviceuuid: ::windows::core::GUID, cachemode: BluetoothCacheMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation")))]
    GetGattServicesForUuidWithCacheModeAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEDevice4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothLEDevice4 {
    type Vtable = IBluetoothLEDevice4_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2b605031_2248_4b2f_acf0_7cee36fc5870);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEDevice4_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub BluetoothDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEDevice5(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothLEDevice5 {
    type Vtable = IBluetoothLEDevice5_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9d6a1260_5287_458e_95ba_17c8b7bb326e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEDevice5_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub WasSecureConnectionUsedForPairing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEDevice6(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothLEDevice6 {
    type Vtable = IBluetoothLEDevice6_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xca7190ef_0cae_573c_a1ca_e1fc5bfc39e2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEDevice6_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetConnectionParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetConnectionPhy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub RequestPreferredConnectionParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, preferredconnectionparameters: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ConnectionParametersChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConnectionParametersChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveConnectionParametersChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveConnectionParametersChanged: usize,
    #[cfg(feature = "Foundation")]
    pub ConnectionPhyChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConnectionPhyChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveConnectionPhyChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveConnectionPhyChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEDeviceStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothLEDeviceStatics {
    type Vtable = IBluetoothLEDeviceStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc8cf1a19_f0b6_4bf0_8689_41303de2d9f4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEDeviceStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    #[cfg(feature = "Foundation")]
    pub FromBluetoothAddressAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bluetoothaddress: u64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromBluetoothAddressAsync: usize,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEDeviceStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothLEDeviceStatics2 {
    type Vtable = IBluetoothLEDeviceStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5f12c06b_3bac_43e8_ad16_563271bd41c2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEDeviceStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetDeviceSelectorFromPairingState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pairingstate: bool, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetDeviceSelectorFromConnectionStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connectionstatus: BluetoothConnectionStatus, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetDeviceSelectorFromDeviceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, devicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetDeviceSelectorFromBluetoothAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bluetoothaddress: u64, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetDeviceSelectorFromBluetoothAddressWithBluetoothAddressType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bluetoothaddress: u64, bluetoothaddresstype: BluetoothAddressType, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetDeviceSelectorFromAppearance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appearance: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromBluetoothAddressWithBluetoothAddressTypeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bluetoothaddress: u64, bluetoothaddresstype: BluetoothAddressType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromBluetoothAddressWithBluetoothAddressTypeAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEPreferredConnectionParameters(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothLEPreferredConnectionParameters {
    type Vtable = IBluetoothLEPreferredConnectionParameters_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2f44344_7372_5f7b_9b34_29c944f5a715);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEPreferredConnectionParameters_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub LinkTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub ConnectionLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub MinConnectionInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub MaxConnectionInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEPreferredConnectionParametersRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothLEPreferredConnectionParametersRequest {
    type Vtable = IBluetoothLEPreferredConnectionParametersRequest_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8a375276_a528_5266_b661_cce6a5ff9739);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEPreferredConnectionParametersRequest_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BluetoothLEPreferredConnectionParametersRequestStatus) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEPreferredConnectionParametersStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothLEPreferredConnectionParametersStatics {
    type Vtable = IBluetoothLEPreferredConnectionParametersStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0e3e8edc_2751_55aa_a838_8faeee818d72);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEPreferredConnectionParametersStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Balanced: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub ThroughputOptimized: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub PowerOptimized: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothSignalStrengthFilter(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothSignalStrengthFilter {
    type Vtable = IBluetoothSignalStrengthFilter_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdf7b7391_6bb5_4cfe_90b1_5d7324edcf7f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothSignalStrengthFilter_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub InRangeThresholdInDBm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InRangeThresholdInDBm: usize,
    #[cfg(feature = "Foundation")]
    pub SetInRangeThresholdInDBm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetInRangeThresholdInDBm: usize,
    #[cfg(feature = "Foundation")]
    pub OutOfRangeThresholdInDBm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OutOfRangeThresholdInDBm: usize,
    #[cfg(feature = "Foundation")]
    pub SetOutOfRangeThresholdInDBm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetOutOfRangeThresholdInDBm: usize,
    #[cfg(feature = "Foundation")]
    pub OutOfRangeTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OutOfRangeTimeout: usize,
    #[cfg(feature = "Foundation")]
    pub SetOutOfRangeTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetOutOfRangeTimeout: usize,
    #[cfg(feature = "Foundation")]
    pub SamplingInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SamplingInterval: usize,
    #[cfg(feature = "Foundation")]
    pub SetSamplingInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSamplingInterval: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothUuidHelperStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothUuidHelperStatics {
    type Vtable = IBluetoothUuidHelperStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x17df0cd8_cf74_4b21_afe6_f57a11bcdea0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothUuidHelperStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub FromShortId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shortid: u32, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TryGetShortId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uuid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryGetShortId: usize,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
