#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "Devices_Bluetooth_Advertisement")]
pub mod Advertisement;
#[cfg(feature = "Devices_Bluetooth_Background")]
pub mod Background;
#[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
pub mod GenericAttributeProfile;
#[cfg(feature = "Devices_Bluetooth_Rfcomm")]
pub mod Rfcomm;
#[repr(transparent)]
pub struct BluetoothAdapter(::windows::core::IUnknown);
impl BluetoothAdapter {
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn BluetoothAddress(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn IsClassicSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsLowEnergySupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsPeripheralRoleSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsCentralRoleSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsAdvertisementOffloadSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(all(feature = "Devices_Radios", feature = "Foundation"))]
    pub fn GetRadioAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::Radios::Radio>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::Radios::Radio>>(result__)
        }
    }
    pub fn AreClassicSecureConnectionsSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IBluetoothAdapter2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn AreLowEnergySecureConnectionsSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IBluetoothAdapter2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsExtendedAdvertisingSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IBluetoothAdapter3>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn MaxAdvertisementDataLength(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IBluetoothAdapter3>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IBluetoothAdapterStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BluetoothAdapter>> {
        Self::IBluetoothAdapterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<BluetoothAdapter>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn GetDefaultAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BluetoothAdapter>> {
        Self::IBluetoothAdapterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<BluetoothAdapter>>(result__)
        })
    }
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
unsafe impl ::windows::core::RuntimeType for BluetoothAdapter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.BluetoothAdapter;{7974f04c-5f7a-4a34-9225-a855f84b1a8b})");
}
unsafe impl ::windows::core::Interface for BluetoothAdapter {
    type Vtable = IBluetoothAdapterVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7974f04c_5f7a_4a34_9225_a855f84b1a8b);
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &BluetoothAdapter {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &BluetoothAdapter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BluetoothAdapter {}
unsafe impl ::core::marker::Sync for BluetoothAdapter {}
#[repr(transparent)]
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
unsafe impl ::windows::core::Abi for BluetoothAddressType {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BluetoothAddressType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BluetoothAddressType {}
unsafe impl ::windows::core::RuntimeType for BluetoothAddressType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.BluetoothAddressType;i4)");
}
impl ::windows::core::DefaultType for BluetoothAddressType {
    type DefaultType = Self;
}
#[repr(transparent)]
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
unsafe impl ::windows::core::Abi for BluetoothCacheMode {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BluetoothCacheMode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BluetoothCacheMode {}
unsafe impl ::windows::core::RuntimeType for BluetoothCacheMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.BluetoothCacheMode;i4)");
}
impl ::windows::core::DefaultType for BluetoothCacheMode {
    type DefaultType = Self;
}
#[repr(transparent)]
pub struct BluetoothClassOfDevice(::windows::core::IUnknown);
impl BluetoothClassOfDevice {
    pub fn RawValue(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn MajorClass(&self) -> ::windows::core::Result<BluetoothMajorClass> {
        let this = self;
        unsafe {
            let mut result__: BluetoothMajorClass = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothMajorClass>(result__)
        }
    }
    pub fn MinorClass(&self) -> ::windows::core::Result<BluetoothMinorClass> {
        let this = self;
        unsafe {
            let mut result__: BluetoothMinorClass = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothMinorClass>(result__)
        }
    }
    pub fn ServiceCapabilities(&self) -> ::windows::core::Result<BluetoothServiceCapabilities> {
        let this = self;
        unsafe {
            let mut result__: BluetoothServiceCapabilities = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothServiceCapabilities>(result__)
        }
    }
    pub fn FromRawValue(rawvalue: u32) -> ::windows::core::Result<BluetoothClassOfDevice> {
        Self::IBluetoothClassOfDeviceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), rawvalue, &mut result__).from_abi::<BluetoothClassOfDevice>(result__)
        })
    }
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
unsafe impl ::windows::core::RuntimeType for BluetoothClassOfDevice {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.BluetoothClassOfDevice;{d640227e-d7d7-4661-9454-65039ca17a2b})");
}
unsafe impl ::windows::core::Interface for BluetoothClassOfDevice {
    type Vtable = IBluetoothClassOfDeviceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd640227e_d7d7_4661_9454_65039ca17a2b);
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &BluetoothClassOfDevice {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &BluetoothClassOfDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BluetoothClassOfDevice {}
unsafe impl ::core::marker::Sync for BluetoothClassOfDevice {}
#[repr(transparent)]
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
unsafe impl ::windows::core::Abi for BluetoothConnectionStatus {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BluetoothConnectionStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BluetoothConnectionStatus {}
unsafe impl ::windows::core::RuntimeType for BluetoothConnectionStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.BluetoothConnectionStatus;i4)");
}
impl ::windows::core::DefaultType for BluetoothConnectionStatus {
    type DefaultType = Self;
}
#[repr(transparent)]
pub struct BluetoothDevice(::windows::core::IUnknown);
impl BluetoothDevice {
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Networking")]
    pub fn HostName(&self) -> ::windows::core::Result<super::super::Networking::HostName> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Networking::HostName>(result__)
        }
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ClassOfDevice(&self) -> ::windows::core::Result<BluetoothClassOfDevice> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothClassOfDevice>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn SdpRecords(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Storage::Streams::IBuffer>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Storage::Streams::IBuffer>>(result__)
        }
    }
    #[cfg(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn RfcommServices(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<Rfcomm::RfcommDeviceService>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<Rfcomm::RfcommDeviceService>>(result__)
        }
    }
    pub fn ConnectionStatus(&self) -> ::windows::core::Result<BluetoothConnectionStatus> {
        let this = self;
        unsafe {
            let mut result__: BluetoothConnectionStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothConnectionStatus>(result__)
        }
    }
    pub fn BluetoothAddress(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn NameChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<BluetoothDevice, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveNameChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn SdpRecordsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<BluetoothDevice, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveSdpRecordsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn ConnectionStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<BluetoothDevice, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveConnectionStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Devices_Enumeration")]
    pub fn DeviceInformation(&self) -> ::windows::core::Result<super::Enumeration::DeviceInformation> {
        let this = &::windows::core::Interface::cast::<IBluetoothDevice2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Enumeration::DeviceInformation>(result__)
        }
    }
    #[cfg(feature = "Devices_Enumeration")]
    pub fn DeviceAccessInformation(&self) -> ::windows::core::Result<super::Enumeration::DeviceAccessInformation> {
        let this = &::windows::core::Interface::cast::<IBluetoothDevice3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Enumeration::DeviceAccessInformation>(result__)
        }
    }
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))]
    pub fn RequestAccessAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::Enumeration::DeviceAccessStatus>> {
        let this = &::windows::core::Interface::cast::<IBluetoothDevice3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::Enumeration::DeviceAccessStatus>>(result__)
        }
    }
    #[cfg(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Foundation"))]
    pub fn GetRfcommServicesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Rfcomm::RfcommDeviceServicesResult>> {
        let this = &::windows::core::Interface::cast::<IBluetoothDevice3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<Rfcomm::RfcommDeviceServicesResult>>(result__)
        }
    }
    #[cfg(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Foundation"))]
    pub fn GetRfcommServicesWithCacheModeAsync(&self, cachemode: BluetoothCacheMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Rfcomm::RfcommDeviceServicesResult>> {
        let this = &::windows::core::Interface::cast::<IBluetoothDevice3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), cachemode, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<Rfcomm::RfcommDeviceServicesResult>>(result__)
        }
    }
    #[cfg(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Foundation"))]
    pub fn GetRfcommServicesForIdAsync<'a, Param0: ::windows::core::IntoParam<'a, Rfcomm::RfcommServiceId>>(&self, serviceid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Rfcomm::RfcommDeviceServicesResult>> {
        let this = &::windows::core::Interface::cast::<IBluetoothDevice3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), serviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<Rfcomm::RfcommDeviceServicesResult>>(result__)
        }
    }
    #[cfg(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Foundation"))]
    pub fn GetRfcommServicesForIdWithCacheModeAsync<'a, Param0: ::windows::core::IntoParam<'a, Rfcomm::RfcommServiceId>>(&self, serviceid: Param0, cachemode: BluetoothCacheMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Rfcomm::RfcommDeviceServicesResult>> {
        let this = &::windows::core::Interface::cast::<IBluetoothDevice3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), serviceid.into_param().abi(), cachemode, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<Rfcomm::RfcommDeviceServicesResult>>(result__)
        }
    }
    pub fn BluetoothDeviceId(&self) -> ::windows::core::Result<BluetoothDeviceId> {
        let this = &::windows::core::Interface::cast::<IBluetoothDevice4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothDeviceId>(result__)
        }
    }
    pub fn WasSecureConnectionUsedForPairing(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IBluetoothDevice5>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BluetoothDevice>> {
        Self::IBluetoothDeviceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<BluetoothDevice>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Networking"))]
    pub fn FromHostNameAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Networking::HostName>>(hostname: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BluetoothDevice>> {
        Self::IBluetoothDeviceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), hostname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<BluetoothDevice>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn FromBluetoothAddressAsync(address: u64) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BluetoothDevice>> {
        Self::IBluetoothDeviceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), address, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<BluetoothDevice>>(result__)
        })
    }
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IBluetoothDeviceStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn GetDeviceSelectorFromPairingState(pairingstate: bool) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IBluetoothDeviceStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), pairingstate, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn GetDeviceSelectorFromConnectionStatus(connectionstatus: BluetoothConnectionStatus) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IBluetoothDeviceStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), connectionstatus, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn GetDeviceSelectorFromDeviceName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(devicename: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IBluetoothDeviceStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), devicename.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn GetDeviceSelectorFromBluetoothAddress(bluetoothaddress: u64) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IBluetoothDeviceStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), bluetoothaddress, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn GetDeviceSelectorFromClassOfDevice<'a, Param0: ::windows::core::IntoParam<'a, BluetoothClassOfDevice>>(classofdevice: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IBluetoothDeviceStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), classofdevice.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
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
unsafe impl ::windows::core::RuntimeType for BluetoothDevice {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.BluetoothDevice;{2335b156-90d2-4a04-aef5-0e20b9e6b707})");
}
unsafe impl ::windows::core::Interface for BluetoothDevice {
    type Vtable = IBluetoothDeviceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2335b156_90d2_4a04_aef5_0e20b9e6b707);
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &BluetoothDevice {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &BluetoothDevice {
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
#[repr(transparent)]
pub struct BluetoothDeviceId(::windows::core::IUnknown);
impl BluetoothDeviceId {
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn IsClassicDevice(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsLowEnergyDevice(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
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
unsafe impl ::windows::core::RuntimeType for BluetoothDeviceId {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.BluetoothDeviceId;{c17949af-57c1-4642-bcce-e6c06b20ae76})");
}
unsafe impl ::windows::core::Interface for BluetoothDeviceId {
    type Vtable = IBluetoothDeviceIdVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc17949af_57c1_4642_bcce_e6c06b20ae76);
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &BluetoothDeviceId {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &BluetoothDeviceId {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BluetoothDeviceId {}
unsafe impl ::core::marker::Sync for BluetoothDeviceId {}
#[repr(transparent)]
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
unsafe impl ::windows::core::Abi for BluetoothError {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BluetoothError {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BluetoothError {}
unsafe impl ::windows::core::RuntimeType for BluetoothError {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.BluetoothError;i4)");
}
impl ::windows::core::DefaultType for BluetoothError {
    type DefaultType = Self;
}
#[repr(transparent)]
pub struct BluetoothLEAppearance(::windows::core::IUnknown);
impl BluetoothLEAppearance {
    pub fn RawValue(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    pub fn Category(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    pub fn SubCategory(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    pub fn FromRawValue(rawvalue: u16) -> ::windows::core::Result<BluetoothLEAppearance> {
        Self::IBluetoothLEAppearanceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), rawvalue, &mut result__).from_abi::<BluetoothLEAppearance>(result__)
        })
    }
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
unsafe impl ::windows::core::RuntimeType for BluetoothLEAppearance {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.BluetoothLEAppearance;{5d2079f2-66a8-4258-985e-02b4d9509f18})");
}
unsafe impl ::windows::core::Interface for BluetoothLEAppearance {
    type Vtable = IBluetoothLEAppearanceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d2079f2_66a8_4258_985e_02b4d9509f18);
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &BluetoothLEAppearance {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &BluetoothLEAppearance {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BluetoothLEAppearance {}
unsafe impl ::core::marker::Sync for BluetoothLEAppearance {}
pub struct BluetoothLEAppearanceCategories {}
impl BluetoothLEAppearanceCategories {
    pub fn Uncategorized() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    pub fn Phone() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    pub fn Computer() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    pub fn Watch() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    pub fn Clock() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    pub fn Display() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    pub fn RemoteControl() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    pub fn EyeGlasses() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    pub fn Tag() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    pub fn Keyring() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    pub fn MediaPlayer() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    pub fn BarcodeScanner() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    pub fn Thermometer() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    pub fn HeartRate() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    pub fn BloodPressure() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    pub fn HumanInterfaceDevice() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    pub fn GlucoseMeter() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    pub fn RunningWalking() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    pub fn Cycling() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    pub fn PulseOximeter() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    pub fn WeightScale() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceCategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
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
pub struct BluetoothLEAppearanceSubcategories {}
impl BluetoothLEAppearanceSubcategories {
    pub fn Generic() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    pub fn SportsWatch() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    pub fn ThermometerEar() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    pub fn HeartRateBelt() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    pub fn BloodPressureArm() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    pub fn BloodPressureWrist() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    pub fn Keyboard() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    pub fn Mouse() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    pub fn Joystick() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    pub fn Gamepad() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    pub fn DigitizerTablet() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    pub fn CardReader() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    pub fn DigitalPen() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    pub fn BarcodeScanner() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    pub fn RunningWalkingInShoe() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    pub fn RunningWalkingOnShoe() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    pub fn RunningWalkingOnHip() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    pub fn CyclingComputer() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    pub fn CyclingSpeedSensor() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    pub fn CyclingCadenceSensor() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    pub fn CyclingPowerSensor() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    pub fn CyclingSpeedCadenceSensor() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    pub fn OximeterFingertip() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    pub fn OximeterWristWorn() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    pub fn LocationDisplay() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).30)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    pub fn LocationNavigationDisplay() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).31)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    pub fn LocationPod() -> ::windows::core::Result<u16> {
        Self::IBluetoothLEAppearanceSubcategoriesStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).32)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
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
#[repr(transparent)]
pub struct BluetoothLEConnectionParameters(::windows::core::IUnknown);
impl BluetoothLEConnectionParameters {
    pub fn LinkTimeout(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    pub fn ConnectionLatency(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    pub fn ConnectionInterval(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
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
unsafe impl ::windows::core::RuntimeType for BluetoothLEConnectionParameters {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.BluetoothLEConnectionParameters;{33cb0771-8da9-508f-a366-1ca388c929ab})");
}
unsafe impl ::windows::core::Interface for BluetoothLEConnectionParameters {
    type Vtable = IBluetoothLEConnectionParametersVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33cb0771_8da9_508f_a366_1ca388c929ab);
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &BluetoothLEConnectionParameters {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &BluetoothLEConnectionParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BluetoothLEConnectionParameters {}
unsafe impl ::core::marker::Sync for BluetoothLEConnectionParameters {}
#[repr(transparent)]
pub struct BluetoothLEConnectionPhy(::windows::core::IUnknown);
impl BluetoothLEConnectionPhy {
    pub fn TransmitInfo(&self) -> ::windows::core::Result<BluetoothLEConnectionPhyInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothLEConnectionPhyInfo>(result__)
        }
    }
    pub fn ReceiveInfo(&self) -> ::windows::core::Result<BluetoothLEConnectionPhyInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothLEConnectionPhyInfo>(result__)
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
unsafe impl ::windows::core::RuntimeType for BluetoothLEConnectionPhy {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.BluetoothLEConnectionPhy;{781e5e48-621e-5a7e-8be6-1b9561ff63c9})");
}
unsafe impl ::windows::core::Interface for BluetoothLEConnectionPhy {
    type Vtable = IBluetoothLEConnectionPhyVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x781e5e48_621e_5a7e_8be6_1b9561ff63c9);
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &BluetoothLEConnectionPhy {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &BluetoothLEConnectionPhy {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BluetoothLEConnectionPhy {}
unsafe impl ::core::marker::Sync for BluetoothLEConnectionPhy {}
#[repr(transparent)]
pub struct BluetoothLEConnectionPhyInfo(::windows::core::IUnknown);
impl BluetoothLEConnectionPhyInfo {
    pub fn IsUncoded1MPhy(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsUncoded2MPhy(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsCodedPhy(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
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
unsafe impl ::windows::core::RuntimeType for BluetoothLEConnectionPhyInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.BluetoothLEConnectionPhyInfo;{9a100bdd-602e-5c27-a1ae-b230015a6394})");
}
unsafe impl ::windows::core::Interface for BluetoothLEConnectionPhyInfo {
    type Vtable = IBluetoothLEConnectionPhyInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a100bdd_602e_5c27_a1ae_b230015a6394);
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &BluetoothLEConnectionPhyInfo {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &BluetoothLEConnectionPhyInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BluetoothLEConnectionPhyInfo {}
unsafe impl ::core::marker::Sync for BluetoothLEConnectionPhyInfo {}
#[repr(transparent)]
pub struct BluetoothLEDevice(::windows::core::IUnknown);
impl BluetoothLEDevice {
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn GattServices(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<GenericAttributeProfile::GattDeviceService>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<GenericAttributeProfile::GattDeviceService>>(result__)
        }
    }
    pub fn ConnectionStatus(&self) -> ::windows::core::Result<BluetoothConnectionStatus> {
        let this = self;
        unsafe {
            let mut result__: BluetoothConnectionStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothConnectionStatus>(result__)
        }
    }
    pub fn BluetoothAddress(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "deprecated"))]
    pub fn GetGattService<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, serviceuuid: Param0) -> ::windows::core::Result<GenericAttributeProfile::GattDeviceService> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), serviceuuid.into_param().abi(), &mut result__).from_abi::<GenericAttributeProfile::GattDeviceService>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn NameChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<BluetoothLEDevice, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveNameChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn GattServicesChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<BluetoothLEDevice, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveGattServicesChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn ConnectionStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<BluetoothLEDevice, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveConnectionStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Devices_Enumeration")]
    pub fn DeviceInformation(&self) -> ::windows::core::Result<super::Enumeration::DeviceInformation> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEDevice2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Enumeration::DeviceInformation>(result__)
        }
    }
    pub fn Appearance(&self) -> ::windows::core::Result<BluetoothLEAppearance> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEDevice2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothLEAppearance>(result__)
        }
    }
    pub fn BluetoothAddressType(&self) -> ::windows::core::Result<BluetoothAddressType> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEDevice2>(self)?;
        unsafe {
            let mut result__: BluetoothAddressType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothAddressType>(result__)
        }
    }
    #[cfg(feature = "Devices_Enumeration")]
    pub fn DeviceAccessInformation(&self) -> ::windows::core::Result<super::Enumeration::DeviceAccessInformation> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEDevice3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Enumeration::DeviceAccessInformation>(result__)
        }
    }
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))]
    pub fn RequestAccessAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::Enumeration::DeviceAccessStatus>> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEDevice3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::Enumeration::DeviceAccessStatus>>(result__)
        }
    }
    #[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation"))]
    pub fn GetGattServicesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<GenericAttributeProfile::GattDeviceServicesResult>> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEDevice3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<GenericAttributeProfile::GattDeviceServicesResult>>(result__)
        }
    }
    #[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation"))]
    pub fn GetGattServicesWithCacheModeAsync(&self, cachemode: BluetoothCacheMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<GenericAttributeProfile::GattDeviceServicesResult>> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEDevice3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), cachemode, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<GenericAttributeProfile::GattDeviceServicesResult>>(result__)
        }
    }
    #[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation"))]
    pub fn GetGattServicesForUuidAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, serviceuuid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<GenericAttributeProfile::GattDeviceServicesResult>> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEDevice3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), serviceuuid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<GenericAttributeProfile::GattDeviceServicesResult>>(result__)
        }
    }
    #[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation"))]
    pub fn GetGattServicesForUuidWithCacheModeAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, serviceuuid: Param0, cachemode: BluetoothCacheMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<GenericAttributeProfile::GattDeviceServicesResult>> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEDevice3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), serviceuuid.into_param().abi(), cachemode, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<GenericAttributeProfile::GattDeviceServicesResult>>(result__)
        }
    }
    pub fn BluetoothDeviceId(&self) -> ::windows::core::Result<BluetoothDeviceId> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEDevice4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothDeviceId>(result__)
        }
    }
    pub fn WasSecureConnectionUsedForPairing(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEDevice5>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn GetConnectionParameters(&self) -> ::windows::core::Result<BluetoothLEConnectionParameters> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEDevice6>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothLEConnectionParameters>(result__)
        }
    }
    pub fn GetConnectionPhy(&self) -> ::windows::core::Result<BluetoothLEConnectionPhy> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEDevice6>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothLEConnectionPhy>(result__)
        }
    }
    pub fn RequestPreferredConnectionParameters<'a, Param0: ::windows::core::IntoParam<'a, BluetoothLEPreferredConnectionParameters>>(&self, preferredconnectionparameters: Param0) -> ::windows::core::Result<BluetoothLEPreferredConnectionParametersRequest> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEDevice6>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), preferredconnectionparameters.into_param().abi(), &mut result__).from_abi::<BluetoothLEPreferredConnectionParametersRequest>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ConnectionParametersChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<BluetoothLEDevice, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEDevice6>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveConnectionParametersChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEDevice6>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn ConnectionPhyChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<BluetoothLEDevice, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEDevice6>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveConnectionPhyChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEDevice6>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BluetoothLEDevice>> {
        Self::IBluetoothLEDeviceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<BluetoothLEDevice>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn FromBluetoothAddressAsync(bluetoothaddress: u64) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BluetoothLEDevice>> {
        Self::IBluetoothLEDeviceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), bluetoothaddress, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<BluetoothLEDevice>>(result__)
        })
    }
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IBluetoothLEDeviceStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn GetDeviceSelectorFromPairingState(pairingstate: bool) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IBluetoothLEDeviceStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), pairingstate, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn GetDeviceSelectorFromConnectionStatus(connectionstatus: BluetoothConnectionStatus) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IBluetoothLEDeviceStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), connectionstatus, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn GetDeviceSelectorFromDeviceName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(devicename: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IBluetoothLEDeviceStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), devicename.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn GetDeviceSelectorFromBluetoothAddress(bluetoothaddress: u64) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IBluetoothLEDeviceStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), bluetoothaddress, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn GetDeviceSelectorFromBluetoothAddressWithBluetoothAddressType(bluetoothaddress: u64, bluetoothaddresstype: BluetoothAddressType) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IBluetoothLEDeviceStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), bluetoothaddress, bluetoothaddresstype, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn GetDeviceSelectorFromAppearance<'a, Param0: ::windows::core::IntoParam<'a, BluetoothLEAppearance>>(appearance: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IBluetoothLEDeviceStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), appearance.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn FromBluetoothAddressWithBluetoothAddressTypeAsync(bluetoothaddress: u64, bluetoothaddresstype: BluetoothAddressType) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BluetoothLEDevice>> {
        Self::IBluetoothLEDeviceStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), bluetoothaddress, bluetoothaddresstype, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<BluetoothLEDevice>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
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
unsafe impl ::windows::core::RuntimeType for BluetoothLEDevice {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.BluetoothLEDevice;{b5ee2f7b-4ad8-4642-ac48-80a0b500e887})");
}
unsafe impl ::windows::core::Interface for BluetoothLEDevice {
    type Vtable = IBluetoothLEDeviceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5ee2f7b_4ad8_4642_ac48_80a0b500e887);
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &BluetoothLEDevice {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &BluetoothLEDevice {
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
#[repr(transparent)]
pub struct BluetoothLEPreferredConnectionParameters(::windows::core::IUnknown);
impl BluetoothLEPreferredConnectionParameters {
    pub fn LinkTimeout(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    pub fn ConnectionLatency(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    pub fn MinConnectionInterval(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    pub fn MaxConnectionInterval(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    pub fn Balanced() -> ::windows::core::Result<BluetoothLEPreferredConnectionParameters> {
        Self::IBluetoothLEPreferredConnectionParametersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothLEPreferredConnectionParameters>(result__)
        })
    }
    pub fn ThroughputOptimized() -> ::windows::core::Result<BluetoothLEPreferredConnectionParameters> {
        Self::IBluetoothLEPreferredConnectionParametersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothLEPreferredConnectionParameters>(result__)
        })
    }
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
unsafe impl ::windows::core::RuntimeType for BluetoothLEPreferredConnectionParameters {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.BluetoothLEPreferredConnectionParameters;{f2f44344-7372-5f7b-9b34-29c944f5a715})");
}
unsafe impl ::windows::core::Interface for BluetoothLEPreferredConnectionParameters {
    type Vtable = IBluetoothLEPreferredConnectionParametersVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2f44344_7372_5f7b_9b34_29c944f5a715);
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &BluetoothLEPreferredConnectionParameters {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &BluetoothLEPreferredConnectionParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BluetoothLEPreferredConnectionParameters {}
unsafe impl ::core::marker::Sync for BluetoothLEPreferredConnectionParameters {}
#[repr(transparent)]
pub struct BluetoothLEPreferredConnectionParametersRequest(::windows::core::IUnknown);
impl BluetoothLEPreferredConnectionParametersRequest {
    pub fn Status(&self) -> ::windows::core::Result<BluetoothLEPreferredConnectionParametersRequestStatus> {
        let this = self;
        unsafe {
            let mut result__: BluetoothLEPreferredConnectionParametersRequestStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothLEPreferredConnectionParametersRequestStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
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
unsafe impl ::windows::core::RuntimeType for BluetoothLEPreferredConnectionParametersRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.BluetoothLEPreferredConnectionParametersRequest;{8a375276-a528-5266-b661-cce6a5ff9739})");
}
unsafe impl ::windows::core::Interface for BluetoothLEPreferredConnectionParametersRequest {
    type Vtable = IBluetoothLEPreferredConnectionParametersRequestVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8a375276_a528_5266_b661_cce6a5ff9739);
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &BluetoothLEPreferredConnectionParametersRequest {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &BluetoothLEPreferredConnectionParametersRequest {
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
#[repr(transparent)]
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
unsafe impl ::windows::core::Abi for BluetoothLEPreferredConnectionParametersRequestStatus {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BluetoothLEPreferredConnectionParametersRequestStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BluetoothLEPreferredConnectionParametersRequestStatus {}
unsafe impl ::windows::core::RuntimeType for BluetoothLEPreferredConnectionParametersRequestStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.BluetoothLEPreferredConnectionParametersRequestStatus;i4)");
}
impl ::windows::core::DefaultType for BluetoothLEPreferredConnectionParametersRequestStatus {
    type DefaultType = Self;
}
#[repr(transparent)]
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
unsafe impl ::windows::core::Abi for BluetoothMajorClass {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BluetoothMajorClass {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BluetoothMajorClass {}
unsafe impl ::windows::core::RuntimeType for BluetoothMajorClass {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.BluetoothMajorClass;i4)");
}
impl ::windows::core::DefaultType for BluetoothMajorClass {
    type DefaultType = Self;
}
#[repr(transparent)]
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
unsafe impl ::windows::core::Abi for BluetoothMinorClass {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BluetoothMinorClass {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BluetoothMinorClass {}
unsafe impl ::windows::core::RuntimeType for BluetoothMinorClass {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.BluetoothMinorClass;i4)");
}
impl ::windows::core::DefaultType for BluetoothMinorClass {
    type DefaultType = Self;
}
#[repr(transparent)]
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
unsafe impl ::windows::core::Abi for BluetoothServiceCapabilities {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BluetoothServiceCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BluetoothServiceCapabilities {}
unsafe impl ::windows::core::RuntimeType for BluetoothServiceCapabilities {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.BluetoothServiceCapabilities;u4)");
}
impl ::windows::core::DefaultType for BluetoothServiceCapabilities {
    type DefaultType = Self;
}
#[repr(transparent)]
pub struct BluetoothSignalStrengthFilter(::windows::core::IUnknown);
impl BluetoothSignalStrengthFilter {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BluetoothSignalStrengthFilter, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    pub fn InRangeThresholdInDBm(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i16>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i16>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetInRangeThresholdInDBm<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<i16>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn OutOfRangeThresholdInDBm(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i16>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i16>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetOutOfRangeThresholdInDBm<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<i16>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn OutOfRangeTimeout(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetOutOfRangeTimeout<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn SamplingInterval(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetSamplingInterval<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
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
unsafe impl ::windows::core::RuntimeType for BluetoothSignalStrengthFilter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.BluetoothSignalStrengthFilter;{df7b7391-6bb5-4cfe-90b1-5d7324edcf7f})");
}
unsafe impl ::windows::core::Interface for BluetoothSignalStrengthFilter {
    type Vtable = IBluetoothSignalStrengthFilterVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdf7b7391_6bb5_4cfe_90b1_5d7324edcf7f);
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &BluetoothSignalStrengthFilter {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &BluetoothSignalStrengthFilter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BluetoothSignalStrengthFilter {}
unsafe impl ::core::marker::Sync for BluetoothSignalStrengthFilter {}
pub struct BluetoothUuidHelper {}
impl BluetoothUuidHelper {
    pub fn FromShortId(shortid: u32) -> ::windows::core::Result<::windows::core::GUID> {
        Self::IBluetoothUuidHelperStatics(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), shortid, &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
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
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothAdapter(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothAdapter {
    type Vtable = IBluetoothAdapterVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7974f04c_5f7a_4a34_9225_a855f84b1a8b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothAdapterVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Devices_Radios", feature = "Foundation"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(any(feature = "Devices_Radios", feature = "Foundation")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothAdapter2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothAdapter2 {
    type Vtable = IBluetoothAdapter2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xac94cecc_24d5_41b3_916d_1097c50b102b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothAdapter2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothAdapter3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothAdapter3 {
    type Vtable = IBluetoothAdapter3Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f8624e0_cba9_5211_9f89_3aac62b4c6b8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothAdapter3Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothAdapterStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothAdapterStatics {
    type Vtable = IBluetoothAdapterStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8b02fb6a_ac4c_4741_8661_8eab7d17ea9f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothAdapterStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothClassOfDevice(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothClassOfDevice {
    type Vtable = IBluetoothClassOfDeviceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd640227e_d7d7_4661_9454_65039ca17a2b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothClassOfDeviceVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BluetoothMajorClass) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BluetoothMinorClass) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BluetoothServiceCapabilities) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothClassOfDeviceStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothClassOfDeviceStatics {
    type Vtable = IBluetoothClassOfDeviceStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe46135bd_0fa2_416c_91b4_c1e48ca061c1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothClassOfDeviceStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rawvalue: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, majorclass: BluetoothMajorClass, minorclass: BluetoothMinorClass, servicecapabilities: BluetoothServiceCapabilities, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothDevice(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothDevice {
    type Vtable = IBluetoothDeviceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2335b156_90d2_4a04_aef5_0e20b9e6b707);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothDeviceVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Networking")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Networking"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(any(feature = "Foundation_Collections", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Foundation_Collections", feature = "deprecated"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(any(feature = "Devices_Bluetooth_Rfcomm", feature = "Foundation_Collections", feature = "deprecated")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BluetoothConnectionStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothDevice2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothDevice2 {
    type Vtable = IBluetoothDevice2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0133f954_b156_4dd0_b1f5_c11bc31a5163);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothDevice2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Enumeration")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothDevice3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothDevice3 {
    type Vtable = IBluetoothDevice3Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x57fff78b_651a_4454_b90f_eb21ef0b0d71);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothDevice3Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Enumeration")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))] usize,
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(any(feature = "Devices_Enumeration", feature = "Foundation")))] usize,
    #[cfg(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Foundation"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(any(feature = "Devices_Bluetooth_Rfcomm", feature = "Foundation")))] usize,
    #[cfg(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Foundation"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cachemode: BluetoothCacheMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(any(feature = "Devices_Bluetooth_Rfcomm", feature = "Foundation")))] usize,
    #[cfg(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Foundation"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, serviceid: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(any(feature = "Devices_Bluetooth_Rfcomm", feature = "Foundation")))] usize,
    #[cfg(all(feature = "Devices_Bluetooth_Rfcomm", feature = "Foundation"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, serviceid: ::windows::core::RawPtr, cachemode: BluetoothCacheMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(any(feature = "Devices_Bluetooth_Rfcomm", feature = "Foundation")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothDevice4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothDevice4 {
    type Vtable = IBluetoothDevice4Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x817c34ad_0e9c_42b2_a8dc_3e8094940d12);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothDevice4Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothDevice5(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothDevice5 {
    type Vtable = IBluetoothDevice5Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5e0b385_5e85_4559_a10d_1c7281379f96);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothDevice5Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothDeviceId(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothDeviceId {
    type Vtable = IBluetoothDeviceIdVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc17949af_57c1_4642_bcce_e6c06b20ae76);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothDeviceIdVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothDeviceIdStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothDeviceIdStatics {
    type Vtable = IBluetoothDeviceIdStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa7884e67_3efb_4f31_bbc2_810e09977404);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothDeviceIdStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothDeviceStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothDeviceStatics {
    type Vtable = IBluetoothDeviceStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0991df51_57db_4725_bbd7_84f64327ec2c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothDeviceStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Networking"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hostname: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(any(feature = "Foundation", feature = "Networking")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, address: u64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothDeviceStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothDeviceStatics2 {
    type Vtable = IBluetoothDeviceStatics2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc29e8e2f_4e14_4477_aa1b_b8b47e5b7ece);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothDeviceStatics2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pairingstate: bool, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connectionstatus: BluetoothConnectionStatus, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, devicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bluetoothaddress: u64, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, classofdevice: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEAppearance(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothLEAppearance {
    type Vtable = IBluetoothLEAppearanceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d2079f2_66a8_4258_985e_02b4d9509f18);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAppearanceVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEAppearanceCategoriesStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothLEAppearanceCategoriesStatics {
    type Vtable = IBluetoothLEAppearanceCategoriesStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d4d54fe_046a_4185_aab6_824cf0610861);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAppearanceCategoriesStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEAppearanceStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothLEAppearanceStatics {
    type Vtable = IBluetoothLEAppearanceStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa193c0c7_4504_4f4a_9ba5_cd1054e5e065);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAppearanceStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rawvalue: u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appearancecategory: u16, appearancesubcategory: u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEAppearanceSubcategoriesStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothLEAppearanceSubcategoriesStatics {
    type Vtable = IBluetoothLEAppearanceSubcategoriesStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe57ba606_2144_415a_8312_71ccf291f8d1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAppearanceSubcategoriesStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEConnectionParameters(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothLEConnectionParameters {
    type Vtable = IBluetoothLEConnectionParametersVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33cb0771_8da9_508f_a366_1ca388c929ab);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEConnectionParametersVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEConnectionPhy(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothLEConnectionPhy {
    type Vtable = IBluetoothLEConnectionPhyVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x781e5e48_621e_5a7e_8be6_1b9561ff63c9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEConnectionPhyVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEConnectionPhyInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothLEConnectionPhyInfo {
    type Vtable = IBluetoothLEConnectionPhyInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a100bdd_602e_5c27_a1ae_b230015a6394);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEConnectionPhyInfoVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEDevice(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothLEDevice {
    type Vtable = IBluetoothLEDeviceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5ee2f7b_4ad8_4642_ac48_80a0b500e887);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEDeviceVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation_Collections", feature = "deprecated"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(any(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation_Collections", feature = "deprecated")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BluetoothConnectionStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "deprecated"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, serviceuuid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(any(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "deprecated")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEDevice2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothLEDevice2 {
    type Vtable = IBluetoothLEDevice2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x26f062b3_7aee_4d31_baba_b1b9775f5916);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEDevice2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Enumeration")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BluetoothAddressType) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEDevice3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothLEDevice3 {
    type Vtable = IBluetoothLEDevice3Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaee9e493_44ac_40dc_af33_b2c13c01ca46);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEDevice3Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Enumeration")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))] usize,
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(any(feature = "Devices_Enumeration", feature = "Foundation")))] usize,
    #[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(any(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation")))] usize,
    #[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cachemode: BluetoothCacheMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(any(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation")))] usize,
    #[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, serviceuuid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(any(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation")))] usize,
    #[cfg(all(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, serviceuuid: ::windows::core::GUID, cachemode: BluetoothCacheMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(any(feature = "Devices_Bluetooth_GenericAttributeProfile", feature = "Foundation")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEDevice4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothLEDevice4 {
    type Vtable = IBluetoothLEDevice4Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2b605031_2248_4b2f_acf0_7cee36fc5870);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEDevice4Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEDevice5(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothLEDevice5 {
    type Vtable = IBluetoothLEDevice5Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9d6a1260_5287_458e_95ba_17c8b7bb326e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEDevice5Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEDevice6(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothLEDevice6 {
    type Vtable = IBluetoothLEDevice6Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xca7190ef_0cae_573c_a1ca_e1fc5bfc39e2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEDevice6Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, preferredconnectionparameters: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEDeviceStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothLEDeviceStatics {
    type Vtable = IBluetoothLEDeviceStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc8cf1a19_f0b6_4bf0_8689_41303de2d9f4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEDeviceStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bluetoothaddress: u64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEDeviceStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothLEDeviceStatics2 {
    type Vtable = IBluetoothLEDeviceStatics2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5f12c06b_3bac_43e8_ad16_563271bd41c2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEDeviceStatics2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pairingstate: bool, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connectionstatus: BluetoothConnectionStatus, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, devicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bluetoothaddress: u64, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bluetoothaddress: u64, bluetoothaddresstype: BluetoothAddressType, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appearance: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bluetoothaddress: u64, bluetoothaddresstype: BluetoothAddressType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEPreferredConnectionParameters(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothLEPreferredConnectionParameters {
    type Vtable = IBluetoothLEPreferredConnectionParametersVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2f44344_7372_5f7b_9b34_29c944f5a715);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEPreferredConnectionParametersVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEPreferredConnectionParametersRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothLEPreferredConnectionParametersRequest {
    type Vtable = IBluetoothLEPreferredConnectionParametersRequestVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8a375276_a528_5266_b661_cce6a5ff9739);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEPreferredConnectionParametersRequestVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BluetoothLEPreferredConnectionParametersRequestStatus) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEPreferredConnectionParametersStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothLEPreferredConnectionParametersStatics {
    type Vtable = IBluetoothLEPreferredConnectionParametersStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0e3e8edc_2751_55aa_a838_8faeee818d72);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEPreferredConnectionParametersStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothSignalStrengthFilter(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothSignalStrengthFilter {
    type Vtable = IBluetoothSignalStrengthFilterVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdf7b7391_6bb5_4cfe_90b1_5d7324edcf7f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothSignalStrengthFilterVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothUuidHelperStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothUuidHelperStatics {
    type Vtable = IBluetoothUuidHelperStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x17df0cd8_cf74_4b21_afe6_f57a11bcdea0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothUuidHelperStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shortid: u32, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uuid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
