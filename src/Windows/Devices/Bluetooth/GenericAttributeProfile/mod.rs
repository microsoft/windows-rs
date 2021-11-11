#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GattCharacteristic(pub ::windows::core::IInspectable);
impl GattCharacteristic {
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation_Collections`*"]
    pub fn GetDescriptors<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, descriptoruuid: Param0) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattDescriptor>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), descriptoruuid.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<GattDescriptor>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn CharacteristicProperties(&self) -> ::windows::core::Result<GattCharacteristicProperties> {
        let this = self;
        unsafe {
            let mut result__: GattCharacteristicProperties = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GattCharacteristicProperties>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn ProtectionLevel(&self) -> ::windows::core::Result<GattProtectionLevel> {
        let this = self;
        unsafe {
            let mut result__: GattProtectionLevel = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GattProtectionLevel>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn SetProtectionLevel(&self, value: GattProtectionLevel) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn UserDescription(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn Uuid(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn AttributeHandle(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation_Collections`*"]
    pub fn PresentationFormats(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattPresentationFormat>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<GattPresentationFormat>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn ReadValueAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattReadResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<GattReadResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn ReadValueWithCacheModeAsync(&self, cachemode: super::BluetoothCacheMode) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattReadResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), cachemode, &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<GattReadResult>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`, `Storage_Streams`*"]
    pub fn WriteValueAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(&self, value: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattCommunicationStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<GattCommunicationStatus>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`, `Storage_Streams`*"]
    pub fn WriteValueWithOptionAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(&self, value: Param0, writeoption: GattWriteOption) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattCommunicationStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value.into_param().abi(), writeoption, &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<GattCommunicationStatus>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn ReadClientCharacteristicConfigurationDescriptorAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattReadClientCharacteristicConfigurationDescriptorResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<GattReadClientCharacteristicConfigurationDescriptorResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn WriteClientCharacteristicConfigurationDescriptorAsync(&self, clientcharacteristicconfigurationdescriptorvalue: GattClientCharacteristicConfigurationDescriptorValue) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattCommunicationStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), clientcharacteristicconfigurationdescriptorvalue, &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<GattCommunicationStatus>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn ValueChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<GattCharacteristic, GattValueChangedEventArgs>>>(&self, valuechangedhandler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), valuechangedhandler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn RemoveValueChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, valuechangedeventcookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), valuechangedeventcookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn Service(&self) -> ::windows::core::Result<GattDeviceService> {
        let this = &::windows::core::Interface::cast::<IGattCharacteristic2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GattDeviceService>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation_Collections`*"]
    pub fn GetAllDescriptors(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattDescriptor>> {
        let this = &::windows::core::Interface::cast::<IGattCharacteristic2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<GattDescriptor>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn ConvertShortIdToUuid(shortid: u16) -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicStatics(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), shortid, &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn GetDescriptorsAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattDescriptorsResult>> {
        let this = &::windows::core::Interface::cast::<IGattCharacteristic3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<GattDescriptorsResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn GetDescriptorsWithCacheModeAsync(&self, cachemode: super::BluetoothCacheMode) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattDescriptorsResult>> {
        let this = &::windows::core::Interface::cast::<IGattCharacteristic3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), cachemode, &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<GattDescriptorsResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn GetDescriptorsForUuidAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, descriptoruuid: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattDescriptorsResult>> {
        let this = &::windows::core::Interface::cast::<IGattCharacteristic3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), descriptoruuid.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<GattDescriptorsResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn GetDescriptorsForUuidWithCacheModeAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, descriptoruuid: Param0, cachemode: super::BluetoothCacheMode) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattDescriptorsResult>> {
        let this = &::windows::core::Interface::cast::<IGattCharacteristic3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), descriptoruuid.into_param().abi(), cachemode, &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<GattDescriptorsResult>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`, `Storage_Streams`*"]
    pub fn WriteValueWithResultAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(&self, value: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattWriteResult>> {
        let this = &::windows::core::Interface::cast::<IGattCharacteristic3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<GattWriteResult>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`, `Storage_Streams`*"]
    pub fn WriteValueWithResultAndOptionAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(&self, value: Param0, writeoption: GattWriteOption) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattWriteResult>> {
        let this = &::windows::core::Interface::cast::<IGattCharacteristic3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi(), writeoption, &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<GattWriteResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn WriteClientCharacteristicConfigurationDescriptorWithResultAsync(&self, clientcharacteristicconfigurationdescriptorvalue: GattClientCharacteristicConfigurationDescriptorValue) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattWriteResult>> {
        let this = &::windows::core::Interface::cast::<IGattCharacteristic3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), clientcharacteristicconfigurationdescriptorvalue, &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<GattWriteResult>>(result__)
        }
    }
    pub fn IGattCharacteristicStatics<R, F: FnOnce(&IGattCharacteristicStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GattCharacteristic, IGattCharacteristicStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for GattCharacteristic {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.GenericAttributeProfile.GattCharacteristic;{59cb50c1-5934-4f68-a198-eb864fa44e6b})");
}
unsafe impl ::windows::core::Interface for GattCharacteristic {
    type Vtable = IGattCharacteristic_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x59cb50c1_5934_4f68_a198_eb864fa44e6b);
}
impl ::windows::core::RuntimeName for GattCharacteristic {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattCharacteristic";
}
impl ::core::convert::From<GattCharacteristic> for ::windows::core::IUnknown {
    fn from(value: GattCharacteristic) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GattCharacteristic> for ::windows::core::IUnknown {
    fn from(value: &GattCharacteristic) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GattCharacteristic {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GattCharacteristic {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GattCharacteristic> for ::windows::core::IInspectable {
    fn from(value: GattCharacteristic) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GattCharacteristic> for ::windows::core::IInspectable {
    fn from(value: &GattCharacteristic) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GattCharacteristic {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GattCharacteristic {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GattCharacteristic {}
unsafe impl ::core::marker::Sync for GattCharacteristic {}
#[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GattCharacteristicProperties(pub u32);
impl GattCharacteristicProperties {
    pub const None: GattCharacteristicProperties = GattCharacteristicProperties(0u32);
    pub const Broadcast: GattCharacteristicProperties = GattCharacteristicProperties(1u32);
    pub const Read: GattCharacteristicProperties = GattCharacteristicProperties(2u32);
    pub const WriteWithoutResponse: GattCharacteristicProperties = GattCharacteristicProperties(4u32);
    pub const Write: GattCharacteristicProperties = GattCharacteristicProperties(8u32);
    pub const Notify: GattCharacteristicProperties = GattCharacteristicProperties(16u32);
    pub const Indicate: GattCharacteristicProperties = GattCharacteristicProperties(32u32);
    pub const AuthenticatedSignedWrites: GattCharacteristicProperties = GattCharacteristicProperties(64u32);
    pub const ExtendedProperties: GattCharacteristicProperties = GattCharacteristicProperties(128u32);
    pub const ReliableWrites: GattCharacteristicProperties = GattCharacteristicProperties(256u32);
    pub const WritableAuxiliaries: GattCharacteristicProperties = GattCharacteristicProperties(512u32);
}
impl ::core::convert::From<u32> for GattCharacteristicProperties {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for GattCharacteristicProperties {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for GattCharacteristicProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.GenericAttributeProfile.GattCharacteristicProperties;u4)");
}
impl ::windows::core::DefaultType for GattCharacteristicProperties {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for GattCharacteristicProperties {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for GattCharacteristicProperties {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for GattCharacteristicProperties {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for GattCharacteristicProperties {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for GattCharacteristicProperties {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
pub struct GattCharacteristicUuids {}
impl GattCharacteristicUuids {
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn BatteryLevel() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn BloodPressureFeature() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn BloodPressureMeasurement() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn BodySensorLocation() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn CscFeature() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn CscMeasurement() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn GlucoseFeature() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn GlucoseMeasurement() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn GlucoseMeasurementContext() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn HeartRateControlPoint() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn HeartRateMeasurement() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn IntermediateCuffPressure() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn IntermediateTemperature() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn MeasurementInterval() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn RecordAccessControlPoint() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn RscFeature() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn RscMeasurement() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn SCControlPoint() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn SensorLocation() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn TemperatureMeasurement() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn TemperatureType() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn AlertCategoryId() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn AlertCategoryIdBitMask() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn AlertLevel() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn AlertNotificationControlPoint() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn AlertStatus() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn GapAppearance() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn BootKeyboardInputReport() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn BootKeyboardOutputReport() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn BootMouseInputReport() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn CurrentTime() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn CyclingPowerControlPoint() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn CyclingPowerFeature() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn CyclingPowerMeasurement() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn CyclingPowerVector() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn DateTime() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn DayDateTime() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn DayOfWeek() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn GapDeviceName() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn DstOffset() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn ExactTime256() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn FirmwareRevisionString() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn HardwareRevisionString() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn HidControlPoint() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn HidInformation() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn Ieee1107320601RegulatoryCertificationDataList() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).30)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn LnControlPoint() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).31)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn LnFeature() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).32)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn LocalTimeInformation() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).33)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn LocationAndSpeed() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).34)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn ManufacturerNameString() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).35)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn ModelNumberString() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).36)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn Navigation() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).37)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn NewAlert() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).38)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn GapPeripheralPreferredConnectionParameters() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).39)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn GapPeripheralPrivacyFlag() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).40)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn PnpId() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).41)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn PositionQuality() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).42)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn ProtocolMode() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).43)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn GapReconnectionAddress() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).44)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn ReferenceTimeInformation() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).45)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn Report() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).46)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn ReportMap() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).47)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn RingerControlPoint() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).48)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn RingerSetting() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).49)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn ScanIntervalWindow() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).50)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn ScanRefresh() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).51)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn SerialNumberString() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).52)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn GattServiceChanged() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).53)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn SoftwareRevisionString() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).54)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn SupportedNewAlertCategory() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).55)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn SupportUnreadAlertCategory() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).56)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn SystemId() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).57)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn TimeAccuracy() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).58)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn TimeSource() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).59)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn TimeUpdateControlPoint() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).60)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn TimeUpdateState() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).61)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn TimeWithDst() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).62)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn TimeZone() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).63)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn TxPowerLevel() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).64)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn UnreadAlertStatus() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattCharacteristicUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).65)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    pub fn IGattCharacteristicUuidsStatics<R, F: FnOnce(&IGattCharacteristicUuidsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GattCharacteristicUuids, IGattCharacteristicUuidsStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IGattCharacteristicUuidsStatics2<R, F: FnOnce(&IGattCharacteristicUuidsStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GattCharacteristicUuids, IGattCharacteristicUuidsStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for GattCharacteristicUuids {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattCharacteristicUuids";
}
#[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GattCharacteristicsResult(pub ::windows::core::IInspectable);
impl GattCharacteristicsResult {
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn Status(&self) -> ::windows::core::Result<GattCommunicationStatus> {
        let this = self;
        unsafe {
            let mut result__: GattCommunicationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GattCommunicationStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn ProtocolError(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u8>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<u8>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation_Collections`*"]
    pub fn Characteristics(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattCharacteristic>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<GattCharacteristic>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for GattCharacteristicsResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.GenericAttributeProfile.GattCharacteristicsResult;{1194945c-b257-4f3e-9db7-f68bc9a9aef2})");
}
unsafe impl ::windows::core::Interface for GattCharacteristicsResult {
    type Vtable = IGattCharacteristicsResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1194945c_b257_4f3e_9db7_f68bc9a9aef2);
}
impl ::windows::core::RuntimeName for GattCharacteristicsResult {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattCharacteristicsResult";
}
impl ::core::convert::From<GattCharacteristicsResult> for ::windows::core::IUnknown {
    fn from(value: GattCharacteristicsResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GattCharacteristicsResult> for ::windows::core::IUnknown {
    fn from(value: &GattCharacteristicsResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GattCharacteristicsResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GattCharacteristicsResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GattCharacteristicsResult> for ::windows::core::IInspectable {
    fn from(value: GattCharacteristicsResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GattCharacteristicsResult> for ::windows::core::IInspectable {
    fn from(value: &GattCharacteristicsResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GattCharacteristicsResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GattCharacteristicsResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GattCharacteristicsResult {}
unsafe impl ::core::marker::Sync for GattCharacteristicsResult {}
#[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GattClientCharacteristicConfigurationDescriptorValue(pub i32);
impl GattClientCharacteristicConfigurationDescriptorValue {
    pub const None: GattClientCharacteristicConfigurationDescriptorValue = GattClientCharacteristicConfigurationDescriptorValue(0i32);
    pub const Notify: GattClientCharacteristicConfigurationDescriptorValue = GattClientCharacteristicConfigurationDescriptorValue(1i32);
    pub const Indicate: GattClientCharacteristicConfigurationDescriptorValue = GattClientCharacteristicConfigurationDescriptorValue(2i32);
}
impl ::core::convert::From<i32> for GattClientCharacteristicConfigurationDescriptorValue {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for GattClientCharacteristicConfigurationDescriptorValue {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for GattClientCharacteristicConfigurationDescriptorValue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.GenericAttributeProfile.GattClientCharacteristicConfigurationDescriptorValue;i4)");
}
impl ::windows::core::DefaultType for GattClientCharacteristicConfigurationDescriptorValue {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GattClientNotificationResult(pub ::windows::core::IInspectable);
impl GattClientNotificationResult {
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn SubscribedClient(&self) -> ::windows::core::Result<GattSubscribedClient> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GattSubscribedClient>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn Status(&self) -> ::windows::core::Result<GattCommunicationStatus> {
        let this = self;
        unsafe {
            let mut result__: GattCommunicationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GattCommunicationStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn ProtocolError(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u8>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<u8>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn BytesSent(&self) -> ::windows::core::Result<u16> {
        let this = &::windows::core::Interface::cast::<IGattClientNotificationResult2>(self)?;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for GattClientNotificationResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.GenericAttributeProfile.GattClientNotificationResult;{506d5599-0112-419a-8e3b-ae21afabd2c2})");
}
unsafe impl ::windows::core::Interface for GattClientNotificationResult {
    type Vtable = IGattClientNotificationResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x506d5599_0112_419a_8e3b_ae21afabd2c2);
}
impl ::windows::core::RuntimeName for GattClientNotificationResult {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattClientNotificationResult";
}
impl ::core::convert::From<GattClientNotificationResult> for ::windows::core::IUnknown {
    fn from(value: GattClientNotificationResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GattClientNotificationResult> for ::windows::core::IUnknown {
    fn from(value: &GattClientNotificationResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GattClientNotificationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GattClientNotificationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GattClientNotificationResult> for ::windows::core::IInspectable {
    fn from(value: GattClientNotificationResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GattClientNotificationResult> for ::windows::core::IInspectable {
    fn from(value: &GattClientNotificationResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GattClientNotificationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GattClientNotificationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GattClientNotificationResult {}
unsafe impl ::core::marker::Sync for GattClientNotificationResult {}
#[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GattCommunicationStatus(pub i32);
impl GattCommunicationStatus {
    pub const Success: GattCommunicationStatus = GattCommunicationStatus(0i32);
    pub const Unreachable: GattCommunicationStatus = GattCommunicationStatus(1i32);
    pub const ProtocolError: GattCommunicationStatus = GattCommunicationStatus(2i32);
    pub const AccessDenied: GattCommunicationStatus = GattCommunicationStatus(3i32);
}
impl ::core::convert::From<i32> for GattCommunicationStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for GattCommunicationStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for GattCommunicationStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.GenericAttributeProfile.GattCommunicationStatus;i4)");
}
impl ::windows::core::DefaultType for GattCommunicationStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GattDescriptor(pub ::windows::core::IInspectable);
impl GattDescriptor {
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn ProtectionLevel(&self) -> ::windows::core::Result<GattProtectionLevel> {
        let this = self;
        unsafe {
            let mut result__: GattProtectionLevel = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GattProtectionLevel>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn SetProtectionLevel(&self, value: GattProtectionLevel) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn Uuid(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn AttributeHandle(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn ReadValueAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattReadResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<GattReadResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn ReadValueWithCacheModeAsync(&self, cachemode: super::BluetoothCacheMode) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattReadResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), cachemode, &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<GattReadResult>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`, `Storage_Streams`*"]
    pub fn WriteValueAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(&self, value: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattCommunicationStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<GattCommunicationStatus>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn ConvertShortIdToUuid(shortid: u16) -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattDescriptorStatics(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), shortid, &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`, `Storage_Streams`*"]
    pub fn WriteValueWithResultAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(&self, value: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattWriteResult>> {
        let this = &::windows::core::Interface::cast::<IGattDescriptor2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<GattWriteResult>>(result__)
        }
    }
    pub fn IGattDescriptorStatics<R, F: FnOnce(&IGattDescriptorStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GattDescriptor, IGattDescriptorStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for GattDescriptor {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.GenericAttributeProfile.GattDescriptor;{92055f2b-8084-4344-b4c2-284de19a8506})");
}
unsafe impl ::windows::core::Interface for GattDescriptor {
    type Vtable = IGattDescriptor_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x92055f2b_8084_4344_b4c2_284de19a8506);
}
impl ::windows::core::RuntimeName for GattDescriptor {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattDescriptor";
}
impl ::core::convert::From<GattDescriptor> for ::windows::core::IUnknown {
    fn from(value: GattDescriptor) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GattDescriptor> for ::windows::core::IUnknown {
    fn from(value: &GattDescriptor) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GattDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GattDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GattDescriptor> for ::windows::core::IInspectable {
    fn from(value: GattDescriptor) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GattDescriptor> for ::windows::core::IInspectable {
    fn from(value: &GattDescriptor) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GattDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GattDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GattDescriptor {}
unsafe impl ::core::marker::Sync for GattDescriptor {}
#[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
pub struct GattDescriptorUuids {}
impl GattDescriptorUuids {
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn CharacteristicAggregateFormat() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattDescriptorUuidsStatics(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn CharacteristicExtendedProperties() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattDescriptorUuidsStatics(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn CharacteristicPresentationFormat() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattDescriptorUuidsStatics(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn CharacteristicUserDescription() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattDescriptorUuidsStatics(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn ClientCharacteristicConfiguration() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattDescriptorUuidsStatics(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn ServerCharacteristicConfiguration() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattDescriptorUuidsStatics(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    pub fn IGattDescriptorUuidsStatics<R, F: FnOnce(&IGattDescriptorUuidsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GattDescriptorUuids, IGattDescriptorUuidsStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for GattDescriptorUuids {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattDescriptorUuids";
}
#[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GattDescriptorsResult(pub ::windows::core::IInspectable);
impl GattDescriptorsResult {
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn Status(&self) -> ::windows::core::Result<GattCommunicationStatus> {
        let this = self;
        unsafe {
            let mut result__: GattCommunicationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GattCommunicationStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn ProtocolError(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u8>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<u8>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation_Collections`*"]
    pub fn Descriptors(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattDescriptor>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<GattDescriptor>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for GattDescriptorsResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.GenericAttributeProfile.GattDescriptorsResult;{9bc091f3-95e7-4489-8d25-ff81955a57b9})");
}
unsafe impl ::windows::core::Interface for GattDescriptorsResult {
    type Vtable = IGattDescriptorsResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9bc091f3_95e7_4489_8d25_ff81955a57b9);
}
impl ::windows::core::RuntimeName for GattDescriptorsResult {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattDescriptorsResult";
}
impl ::core::convert::From<GattDescriptorsResult> for ::windows::core::IUnknown {
    fn from(value: GattDescriptorsResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GattDescriptorsResult> for ::windows::core::IUnknown {
    fn from(value: &GattDescriptorsResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GattDescriptorsResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GattDescriptorsResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GattDescriptorsResult> for ::windows::core::IInspectable {
    fn from(value: GattDescriptorsResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GattDescriptorsResult> for ::windows::core::IInspectable {
    fn from(value: &GattDescriptorsResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GattDescriptorsResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GattDescriptorsResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GattDescriptorsResult {}
unsafe impl ::core::marker::Sync for GattDescriptorsResult {}
#[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GattDeviceService(pub ::windows::core::IInspectable);
impl GattDeviceService {
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation_Collections`*"]
    pub fn GetCharacteristics<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, characteristicuuid: Param0) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattCharacteristic>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), characteristicuuid.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<GattCharacteristic>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation_Collections`*"]
    pub fn GetIncludedServices<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, serviceuuid: Param0) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattDeviceService>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), serviceuuid.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<GattDeviceService>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn Uuid(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn AttributeHandle(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn Device(&self) -> ::windows::core::Result<super::BluetoothLEDevice> {
        let this = &::windows::core::Interface::cast::<IGattDeviceService2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::BluetoothLEDevice>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation_Collections`*"]
    pub fn ParentServices(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattDeviceService>> {
        let this = &::windows::core::Interface::cast::<IGattDeviceService2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<GattDeviceService>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation_Collections`*"]
    pub fn GetAllCharacteristics(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattCharacteristic>> {
        let this = &::windows::core::Interface::cast::<IGattDeviceService2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<GattCharacteristic>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation_Collections`*"]
    pub fn GetAllIncludedServices(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattDeviceService>> {
        let this = &::windows::core::Interface::cast::<IGattDeviceService2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<GattDeviceService>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn FromIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattDeviceService>> {
        Self::IGattDeviceServiceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<GattDeviceService>>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn GetDeviceSelectorFromUuid<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(serviceuuid: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IGattDeviceServiceStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), serviceuuid.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn GetDeviceSelectorFromShortId(serviceshortid: u16) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IGattDeviceServiceStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), serviceshortid, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn ConvertShortIdToUuid(shortid: u16) -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattDeviceServiceStatics(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), shortid, &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[cfg(feature = "Devices_Enumeration")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Devices_Enumeration`*"]
    pub fn DeviceAccessInformation(&self) -> ::windows::core::Result<super::super::Enumeration::DeviceAccessInformation> {
        let this = &::windows::core::Interface::cast::<IGattDeviceService3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Enumeration::DeviceAccessInformation>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn Session(&self) -> ::windows::core::Result<GattSession> {
        let this = &::windows::core::Interface::cast::<IGattDeviceService3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GattSession>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn SharingMode(&self) -> ::windows::core::Result<GattSharingMode> {
        let this = &::windows::core::Interface::cast::<IGattDeviceService3>(self)?;
        unsafe {
            let mut result__: GattSharingMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GattSharingMode>(result__)
        }
    }
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Devices_Enumeration`, `Foundation`*"]
    pub fn RequestAccessAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::Enumeration::DeviceAccessStatus>> {
        let this = &::windows::core::Interface::cast::<IGattDeviceService3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::Enumeration::DeviceAccessStatus>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn OpenAsync(&self, sharingmode: GattSharingMode) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattOpenStatus>> {
        let this = &::windows::core::Interface::cast::<IGattDeviceService3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), sharingmode, &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<GattOpenStatus>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn GetCharacteristicsAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattCharacteristicsResult>> {
        let this = &::windows::core::Interface::cast::<IGattDeviceService3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<GattCharacteristicsResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn GetCharacteristicsWithCacheModeAsync(&self, cachemode: super::BluetoothCacheMode) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattCharacteristicsResult>> {
        let this = &::windows::core::Interface::cast::<IGattDeviceService3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), cachemode, &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<GattCharacteristicsResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn GetCharacteristicsForUuidAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, characteristicuuid: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattCharacteristicsResult>> {
        let this = &::windows::core::Interface::cast::<IGattDeviceService3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), characteristicuuid.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<GattCharacteristicsResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn GetCharacteristicsForUuidWithCacheModeAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, characteristicuuid: Param0, cachemode: super::BluetoothCacheMode) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattCharacteristicsResult>> {
        let this = &::windows::core::Interface::cast::<IGattDeviceService3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), characteristicuuid.into_param().abi(), cachemode, &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<GattCharacteristicsResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn GetIncludedServicesAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattDeviceServicesResult>> {
        let this = &::windows::core::Interface::cast::<IGattDeviceService3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<GattDeviceServicesResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn GetIncludedServicesWithCacheModeAsync(&self, cachemode: super::BluetoothCacheMode) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattDeviceServicesResult>> {
        let this = &::windows::core::Interface::cast::<IGattDeviceService3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), cachemode, &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<GattDeviceServicesResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn GetIncludedServicesForUuidAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, serviceuuid: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattDeviceServicesResult>> {
        let this = &::windows::core::Interface::cast::<IGattDeviceService3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), serviceuuid.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<GattDeviceServicesResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn GetIncludedServicesForUuidWithCacheModeAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, serviceuuid: Param0, cachemode: super::BluetoothCacheMode) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattDeviceServicesResult>> {
        let this = &::windows::core::Interface::cast::<IGattDeviceService3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), serviceuuid.into_param().abi(), cachemode, &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<GattDeviceServicesResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn FromIdWithSharingModeAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0, sharingmode: GattSharingMode) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattDeviceService>> {
        Self::IGattDeviceServiceStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), sharingmode, &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<GattDeviceService>>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn GetDeviceSelectorForBluetoothDeviceId<'a, Param0: ::windows::core::IntoParam<'a, super::BluetoothDeviceId>>(bluetoothdeviceid: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IGattDeviceServiceStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), bluetoothdeviceid.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn GetDeviceSelectorForBluetoothDeviceIdWithCacheMode<'a, Param0: ::windows::core::IntoParam<'a, super::BluetoothDeviceId>>(bluetoothdeviceid: Param0, cachemode: super::BluetoothCacheMode) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IGattDeviceServiceStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), bluetoothdeviceid.into_param().abi(), cachemode, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn GetDeviceSelectorForBluetoothDeviceIdAndUuid<'a, Param0: ::windows::core::IntoParam<'a, super::BluetoothDeviceId>, Param1: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(bluetoothdeviceid: Param0, serviceuuid: Param1) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IGattDeviceServiceStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), bluetoothdeviceid.into_param().abi(), serviceuuid.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn GetDeviceSelectorForBluetoothDeviceIdAndUuidWithCacheMode<'a, Param0: ::windows::core::IntoParam<'a, super::BluetoothDeviceId>, Param1: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(bluetoothdeviceid: Param0, serviceuuid: Param1, cachemode: super::BluetoothCacheMode) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IGattDeviceServiceStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), bluetoothdeviceid.into_param().abi(), serviceuuid.into_param().abi(), cachemode, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn IGattDeviceServiceStatics<R, F: FnOnce(&IGattDeviceServiceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GattDeviceService, IGattDeviceServiceStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IGattDeviceServiceStatics2<R, F: FnOnce(&IGattDeviceServiceStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GattDeviceService, IGattDeviceServiceStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for GattDeviceService {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.GenericAttributeProfile.GattDeviceService;{ac7b7c05-b33c-47cf-990f-6b8f5577df71})");
}
unsafe impl ::windows::core::Interface for GattDeviceService {
    type Vtable = IGattDeviceService_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xac7b7c05_b33c_47cf_990f_6b8f5577df71);
}
impl ::windows::core::RuntimeName for GattDeviceService {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattDeviceService";
}
impl ::core::convert::From<GattDeviceService> for ::windows::core::IUnknown {
    fn from(value: GattDeviceService) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GattDeviceService> for ::windows::core::IUnknown {
    fn from(value: &GattDeviceService) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GattDeviceService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GattDeviceService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GattDeviceService> for ::windows::core::IInspectable {
    fn from(value: GattDeviceService) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GattDeviceService> for ::windows::core::IInspectable {
    fn from(value: &GattDeviceService) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GattDeviceService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GattDeviceService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<GattDeviceService> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: GattDeviceService) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&GattDeviceService> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &GattDeviceService) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for GattDeviceService {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for &GattDeviceService {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for GattDeviceService {}
unsafe impl ::core::marker::Sync for GattDeviceService {}
#[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GattDeviceServicesResult(pub ::windows::core::IInspectable);
impl GattDeviceServicesResult {
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn Status(&self) -> ::windows::core::Result<GattCommunicationStatus> {
        let this = self;
        unsafe {
            let mut result__: GattCommunicationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GattCommunicationStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn ProtocolError(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u8>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<u8>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation_Collections`*"]
    pub fn Services(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattDeviceService>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<GattDeviceService>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for GattDeviceServicesResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.GenericAttributeProfile.GattDeviceServicesResult;{171dd3ee-016d-419d-838a-576cf475a3d8})");
}
unsafe impl ::windows::core::Interface for GattDeviceServicesResult {
    type Vtable = IGattDeviceServicesResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x171dd3ee_016d_419d_838a_576cf475a3d8);
}
impl ::windows::core::RuntimeName for GattDeviceServicesResult {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattDeviceServicesResult";
}
impl ::core::convert::From<GattDeviceServicesResult> for ::windows::core::IUnknown {
    fn from(value: GattDeviceServicesResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GattDeviceServicesResult> for ::windows::core::IUnknown {
    fn from(value: &GattDeviceServicesResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GattDeviceServicesResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GattDeviceServicesResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GattDeviceServicesResult> for ::windows::core::IInspectable {
    fn from(value: GattDeviceServicesResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GattDeviceServicesResult> for ::windows::core::IInspectable {
    fn from(value: &GattDeviceServicesResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GattDeviceServicesResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GattDeviceServicesResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GattDeviceServicesResult {}
unsafe impl ::core::marker::Sync for GattDeviceServicesResult {}
#[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GattLocalCharacteristic(pub ::windows::core::IInspectable);
impl GattLocalCharacteristic {
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn Uuid(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Storage_Streams`*"]
    pub fn StaticValue(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn CharacteristicProperties(&self) -> ::windows::core::Result<GattCharacteristicProperties> {
        let this = self;
        unsafe {
            let mut result__: GattCharacteristicProperties = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GattCharacteristicProperties>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn ReadProtectionLevel(&self) -> ::windows::core::Result<GattProtectionLevel> {
        let this = self;
        unsafe {
            let mut result__: GattProtectionLevel = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GattProtectionLevel>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn WriteProtectionLevel(&self) -> ::windows::core::Result<GattProtectionLevel> {
        let this = self;
        unsafe {
            let mut result__: GattProtectionLevel = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GattProtectionLevel>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn CreateDescriptorAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param1: ::windows::core::IntoParam<'a, GattLocalDescriptorParameters>>(&self, descriptoruuid: Param0, parameters: Param1) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattLocalDescriptorResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), descriptoruuid.into_param().abi(), parameters.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<GattLocalDescriptorResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation_Collections`*"]
    pub fn Descriptors(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattLocalDescriptor>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<GattLocalDescriptor>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn UserDescription(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation_Collections`*"]
    pub fn PresentationFormats(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattPresentationFormat>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<GattPresentationFormat>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation_Collections`*"]
    pub fn SubscribedClients(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattSubscribedClient>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<GattSubscribedClient>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn SubscribedClientsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<GattLocalCharacteristic, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn RemoveSubscribedClientsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn ReadRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<GattLocalCharacteristic, GattReadRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn RemoveReadRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn WriteRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<GattLocalCharacteristic, GattWriteRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn RemoveWriteRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`, `Foundation_Collections`, `Storage_Streams`*"]
    pub fn NotifyValueAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(&self, value: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<GattClientNotificationResult>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<GattClientNotificationResult>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`, `Storage_Streams`*"]
    pub fn NotifyValueForSubscribedClientAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>, Param1: ::windows::core::IntoParam<'a, GattSubscribedClient>>(&self, value: Param0, subscribedclient: Param1) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattClientNotificationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), value.into_param().abi(), subscribedclient.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<GattClientNotificationResult>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for GattLocalCharacteristic {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.GenericAttributeProfile.GattLocalCharacteristic;{aede376d-5412-4d74-92a8-8deb8526829c})");
}
unsafe impl ::windows::core::Interface for GattLocalCharacteristic {
    type Vtable = IGattLocalCharacteristic_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaede376d_5412_4d74_92a8_8deb8526829c);
}
impl ::windows::core::RuntimeName for GattLocalCharacteristic {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattLocalCharacteristic";
}
impl ::core::convert::From<GattLocalCharacteristic> for ::windows::core::IUnknown {
    fn from(value: GattLocalCharacteristic) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GattLocalCharacteristic> for ::windows::core::IUnknown {
    fn from(value: &GattLocalCharacteristic) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GattLocalCharacteristic {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GattLocalCharacteristic {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GattLocalCharacteristic> for ::windows::core::IInspectable {
    fn from(value: GattLocalCharacteristic) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GattLocalCharacteristic> for ::windows::core::IInspectable {
    fn from(value: &GattLocalCharacteristic) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GattLocalCharacteristic {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GattLocalCharacteristic {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GattLocalCharacteristic {}
unsafe impl ::core::marker::Sync for GattLocalCharacteristic {}
#[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GattLocalCharacteristicParameters(pub ::windows::core::IInspectable);
impl GattLocalCharacteristicParameters {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GattLocalCharacteristicParameters, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Storage_Streams`*"]
    pub fn SetStaticValue<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Storage_Streams`*"]
    pub fn StaticValue(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn SetCharacteristicProperties(&self, value: GattCharacteristicProperties) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn CharacteristicProperties(&self) -> ::windows::core::Result<GattCharacteristicProperties> {
        let this = self;
        unsafe {
            let mut result__: GattCharacteristicProperties = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GattCharacteristicProperties>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn SetReadProtectionLevel(&self, value: GattProtectionLevel) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn ReadProtectionLevel(&self) -> ::windows::core::Result<GattProtectionLevel> {
        let this = self;
        unsafe {
            let mut result__: GattProtectionLevel = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GattProtectionLevel>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn SetWriteProtectionLevel(&self, value: GattProtectionLevel) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn WriteProtectionLevel(&self) -> ::windows::core::Result<GattProtectionLevel> {
        let this = self;
        unsafe {
            let mut result__: GattProtectionLevel = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GattProtectionLevel>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn SetUserDescription<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn UserDescription(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation_Collections`*"]
    pub fn PresentationFormats(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<GattPresentationFormat>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<GattPresentationFormat>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for GattLocalCharacteristicParameters {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.GenericAttributeProfile.GattLocalCharacteristicParameters;{faf73db4-4cff-44c7-8445-040e6ead0063})");
}
unsafe impl ::windows::core::Interface for GattLocalCharacteristicParameters {
    type Vtable = IGattLocalCharacteristicParameters_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfaf73db4_4cff_44c7_8445_040e6ead0063);
}
impl ::windows::core::RuntimeName for GattLocalCharacteristicParameters {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattLocalCharacteristicParameters";
}
impl ::core::convert::From<GattLocalCharacteristicParameters> for ::windows::core::IUnknown {
    fn from(value: GattLocalCharacteristicParameters) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GattLocalCharacteristicParameters> for ::windows::core::IUnknown {
    fn from(value: &GattLocalCharacteristicParameters) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GattLocalCharacteristicParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GattLocalCharacteristicParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GattLocalCharacteristicParameters> for ::windows::core::IInspectable {
    fn from(value: GattLocalCharacteristicParameters) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GattLocalCharacteristicParameters> for ::windows::core::IInspectable {
    fn from(value: &GattLocalCharacteristicParameters) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GattLocalCharacteristicParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GattLocalCharacteristicParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GattLocalCharacteristicParameters {}
unsafe impl ::core::marker::Sync for GattLocalCharacteristicParameters {}
#[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GattLocalCharacteristicResult(pub ::windows::core::IInspectable);
impl GattLocalCharacteristicResult {
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn Characteristic(&self) -> ::windows::core::Result<GattLocalCharacteristic> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GattLocalCharacteristic>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn Error(&self) -> ::windows::core::Result<super::BluetoothError> {
        let this = self;
        unsafe {
            let mut result__: super::BluetoothError = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::BluetoothError>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for GattLocalCharacteristicResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.GenericAttributeProfile.GattLocalCharacteristicResult;{7975de9b-0170-4397-9666-92f863f12ee6})");
}
unsafe impl ::windows::core::Interface for GattLocalCharacteristicResult {
    type Vtable = IGattLocalCharacteristicResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7975de9b_0170_4397_9666_92f863f12ee6);
}
impl ::windows::core::RuntimeName for GattLocalCharacteristicResult {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattLocalCharacteristicResult";
}
impl ::core::convert::From<GattLocalCharacteristicResult> for ::windows::core::IUnknown {
    fn from(value: GattLocalCharacteristicResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GattLocalCharacteristicResult> for ::windows::core::IUnknown {
    fn from(value: &GattLocalCharacteristicResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GattLocalCharacteristicResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GattLocalCharacteristicResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GattLocalCharacteristicResult> for ::windows::core::IInspectable {
    fn from(value: GattLocalCharacteristicResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GattLocalCharacteristicResult> for ::windows::core::IInspectable {
    fn from(value: &GattLocalCharacteristicResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GattLocalCharacteristicResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GattLocalCharacteristicResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GattLocalCharacteristicResult {}
unsafe impl ::core::marker::Sync for GattLocalCharacteristicResult {}
#[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GattLocalDescriptor(pub ::windows::core::IInspectable);
impl GattLocalDescriptor {
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn Uuid(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Storage_Streams`*"]
    pub fn StaticValue(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn ReadProtectionLevel(&self) -> ::windows::core::Result<GattProtectionLevel> {
        let this = self;
        unsafe {
            let mut result__: GattProtectionLevel = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GattProtectionLevel>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn WriteProtectionLevel(&self) -> ::windows::core::Result<GattProtectionLevel> {
        let this = self;
        unsafe {
            let mut result__: GattProtectionLevel = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GattProtectionLevel>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn ReadRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<GattLocalDescriptor, GattReadRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn RemoveReadRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn WriteRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<GattLocalDescriptor, GattWriteRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn RemoveWriteRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for GattLocalDescriptor {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.GenericAttributeProfile.GattLocalDescriptor;{f48ebe06-789d-4a4b-8652-bd017b5d2fc6})");
}
unsafe impl ::windows::core::Interface for GattLocalDescriptor {
    type Vtable = IGattLocalDescriptor_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf48ebe06_789d_4a4b_8652_bd017b5d2fc6);
}
impl ::windows::core::RuntimeName for GattLocalDescriptor {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattLocalDescriptor";
}
impl ::core::convert::From<GattLocalDescriptor> for ::windows::core::IUnknown {
    fn from(value: GattLocalDescriptor) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GattLocalDescriptor> for ::windows::core::IUnknown {
    fn from(value: &GattLocalDescriptor) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GattLocalDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GattLocalDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GattLocalDescriptor> for ::windows::core::IInspectable {
    fn from(value: GattLocalDescriptor) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GattLocalDescriptor> for ::windows::core::IInspectable {
    fn from(value: &GattLocalDescriptor) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GattLocalDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GattLocalDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GattLocalDescriptor {}
unsafe impl ::core::marker::Sync for GattLocalDescriptor {}
#[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GattLocalDescriptorParameters(pub ::windows::core::IInspectable);
impl GattLocalDescriptorParameters {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GattLocalDescriptorParameters, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Storage_Streams`*"]
    pub fn SetStaticValue<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Storage_Streams`*"]
    pub fn StaticValue(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn SetReadProtectionLevel(&self, value: GattProtectionLevel) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn ReadProtectionLevel(&self) -> ::windows::core::Result<GattProtectionLevel> {
        let this = self;
        unsafe {
            let mut result__: GattProtectionLevel = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GattProtectionLevel>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn SetWriteProtectionLevel(&self, value: GattProtectionLevel) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn WriteProtectionLevel(&self) -> ::windows::core::Result<GattProtectionLevel> {
        let this = self;
        unsafe {
            let mut result__: GattProtectionLevel = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GattProtectionLevel>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for GattLocalDescriptorParameters {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.GenericAttributeProfile.GattLocalDescriptorParameters;{5fdede6a-f3c1-4b66-8c4b-e3d2293b40e9})");
}
unsafe impl ::windows::core::Interface for GattLocalDescriptorParameters {
    type Vtable = IGattLocalDescriptorParameters_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5fdede6a_f3c1_4b66_8c4b_e3d2293b40e9);
}
impl ::windows::core::RuntimeName for GattLocalDescriptorParameters {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattLocalDescriptorParameters";
}
impl ::core::convert::From<GattLocalDescriptorParameters> for ::windows::core::IUnknown {
    fn from(value: GattLocalDescriptorParameters) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GattLocalDescriptorParameters> for ::windows::core::IUnknown {
    fn from(value: &GattLocalDescriptorParameters) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GattLocalDescriptorParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GattLocalDescriptorParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GattLocalDescriptorParameters> for ::windows::core::IInspectable {
    fn from(value: GattLocalDescriptorParameters) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GattLocalDescriptorParameters> for ::windows::core::IInspectable {
    fn from(value: &GattLocalDescriptorParameters) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GattLocalDescriptorParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GattLocalDescriptorParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GattLocalDescriptorParameters {}
unsafe impl ::core::marker::Sync for GattLocalDescriptorParameters {}
#[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GattLocalDescriptorResult(pub ::windows::core::IInspectable);
impl GattLocalDescriptorResult {
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn Descriptor(&self) -> ::windows::core::Result<GattLocalDescriptor> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GattLocalDescriptor>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn Error(&self) -> ::windows::core::Result<super::BluetoothError> {
        let this = self;
        unsafe {
            let mut result__: super::BluetoothError = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::BluetoothError>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for GattLocalDescriptorResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.GenericAttributeProfile.GattLocalDescriptorResult;{375791be-321f-4366-bfc1-3bc6b82c79f8})");
}
unsafe impl ::windows::core::Interface for GattLocalDescriptorResult {
    type Vtable = IGattLocalDescriptorResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x375791be_321f_4366_bfc1_3bc6b82c79f8);
}
impl ::windows::core::RuntimeName for GattLocalDescriptorResult {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattLocalDescriptorResult";
}
impl ::core::convert::From<GattLocalDescriptorResult> for ::windows::core::IUnknown {
    fn from(value: GattLocalDescriptorResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GattLocalDescriptorResult> for ::windows::core::IUnknown {
    fn from(value: &GattLocalDescriptorResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GattLocalDescriptorResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GattLocalDescriptorResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GattLocalDescriptorResult> for ::windows::core::IInspectable {
    fn from(value: GattLocalDescriptorResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GattLocalDescriptorResult> for ::windows::core::IInspectable {
    fn from(value: &GattLocalDescriptorResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GattLocalDescriptorResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GattLocalDescriptorResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GattLocalDescriptorResult {}
unsafe impl ::core::marker::Sync for GattLocalDescriptorResult {}
#[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GattLocalService(pub ::windows::core::IInspectable);
impl GattLocalService {
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn Uuid(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn CreateCharacteristicAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param1: ::windows::core::IntoParam<'a, GattLocalCharacteristicParameters>>(&self, characteristicuuid: Param0, parameters: Param1) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattLocalCharacteristicResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), characteristicuuid.into_param().abi(), parameters.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<GattLocalCharacteristicResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation_Collections`*"]
    pub fn Characteristics(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattLocalCharacteristic>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<GattLocalCharacteristic>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for GattLocalService {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.GenericAttributeProfile.GattLocalService;{f513e258-f7f7-4902-b803-57fcc7d6fe83})");
}
unsafe impl ::windows::core::Interface for GattLocalService {
    type Vtable = IGattLocalService_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf513e258_f7f7_4902_b803_57fcc7d6fe83);
}
impl ::windows::core::RuntimeName for GattLocalService {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattLocalService";
}
impl ::core::convert::From<GattLocalService> for ::windows::core::IUnknown {
    fn from(value: GattLocalService) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GattLocalService> for ::windows::core::IUnknown {
    fn from(value: &GattLocalService) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GattLocalService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GattLocalService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GattLocalService> for ::windows::core::IInspectable {
    fn from(value: GattLocalService) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GattLocalService> for ::windows::core::IInspectable {
    fn from(value: &GattLocalService) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GattLocalService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GattLocalService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GattLocalService {}
unsafe impl ::core::marker::Sync for GattLocalService {}
#[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GattOpenStatus(pub i32);
impl GattOpenStatus {
    pub const Unspecified: GattOpenStatus = GattOpenStatus(0i32);
    pub const Success: GattOpenStatus = GattOpenStatus(1i32);
    pub const AlreadyOpened: GattOpenStatus = GattOpenStatus(2i32);
    pub const NotFound: GattOpenStatus = GattOpenStatus(3i32);
    pub const SharingViolation: GattOpenStatus = GattOpenStatus(4i32);
    pub const AccessDenied: GattOpenStatus = GattOpenStatus(5i32);
}
impl ::core::convert::From<i32> for GattOpenStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for GattOpenStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for GattOpenStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.GenericAttributeProfile.GattOpenStatus;i4)");
}
impl ::windows::core::DefaultType for GattOpenStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GattPresentationFormat(pub ::windows::core::IInspectable);
impl GattPresentationFormat {
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn FormatType(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn Exponent(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn Unit(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn Namespace(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn Description(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn BluetoothSigAssignedNumbers() -> ::windows::core::Result<u8> {
        Self::IGattPresentationFormatStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn FromParts(formattype: u8, exponent: i32, unit: u16, namespaceid: u8, description: u16) -> ::windows::core::Result<GattPresentationFormat> {
        Self::IGattPresentationFormatStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), formattype, exponent, unit, namespaceid, description, &mut result__).from_abi::<GattPresentationFormat>(result__)
        })
    }
    pub fn IGattPresentationFormatStatics<R, F: FnOnce(&IGattPresentationFormatStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GattPresentationFormat, IGattPresentationFormatStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IGattPresentationFormatStatics2<R, F: FnOnce(&IGattPresentationFormatStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GattPresentationFormat, IGattPresentationFormatStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for GattPresentationFormat {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.GenericAttributeProfile.GattPresentationFormat;{196d0021-faad-45dc-ae5b-2ac3184e84db})");
}
unsafe impl ::windows::core::Interface for GattPresentationFormat {
    type Vtable = IGattPresentationFormat_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x196d0021_faad_45dc_ae5b_2ac3184e84db);
}
impl ::windows::core::RuntimeName for GattPresentationFormat {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattPresentationFormat";
}
impl ::core::convert::From<GattPresentationFormat> for ::windows::core::IUnknown {
    fn from(value: GattPresentationFormat) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GattPresentationFormat> for ::windows::core::IUnknown {
    fn from(value: &GattPresentationFormat) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GattPresentationFormat {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GattPresentationFormat {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GattPresentationFormat> for ::windows::core::IInspectable {
    fn from(value: GattPresentationFormat) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GattPresentationFormat> for ::windows::core::IInspectable {
    fn from(value: &GattPresentationFormat) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GattPresentationFormat {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GattPresentationFormat {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GattPresentationFormat {}
unsafe impl ::core::marker::Sync for GattPresentationFormat {}
#[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
pub struct GattPresentationFormatTypes {}
impl GattPresentationFormatTypes {
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn Boolean() -> ::windows::core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn Bit2() -> ::windows::core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn Nibble() -> ::windows::core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn UInt8() -> ::windows::core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn UInt12() -> ::windows::core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn UInt16() -> ::windows::core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn UInt24() -> ::windows::core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn UInt32() -> ::windows::core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn UInt48() -> ::windows::core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn UInt64() -> ::windows::core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn UInt128() -> ::windows::core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn SInt8() -> ::windows::core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn SInt12() -> ::windows::core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn SInt16() -> ::windows::core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn SInt24() -> ::windows::core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn SInt32() -> ::windows::core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn SInt48() -> ::windows::core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn SInt64() -> ::windows::core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn SInt128() -> ::windows::core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn Float32() -> ::windows::core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn Float64() -> ::windows::core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn SFloat() -> ::windows::core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn Float() -> ::windows::core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn DUInt16() -> ::windows::core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn Utf8() -> ::windows::core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).30)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn Utf16() -> ::windows::core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).31)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn Struct() -> ::windows::core::Result<u8> {
        Self::IGattPresentationFormatTypesStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).32)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    pub fn IGattPresentationFormatTypesStatics<R, F: FnOnce(&IGattPresentationFormatTypesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GattPresentationFormatTypes, IGattPresentationFormatTypesStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for GattPresentationFormatTypes {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattPresentationFormatTypes";
}
#[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GattProtectionLevel(pub i32);
impl GattProtectionLevel {
    pub const Plain: GattProtectionLevel = GattProtectionLevel(0i32);
    pub const AuthenticationRequired: GattProtectionLevel = GattProtectionLevel(1i32);
    pub const EncryptionRequired: GattProtectionLevel = GattProtectionLevel(2i32);
    pub const EncryptionAndAuthenticationRequired: GattProtectionLevel = GattProtectionLevel(3i32);
}
impl ::core::convert::From<i32> for GattProtectionLevel {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for GattProtectionLevel {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for GattProtectionLevel {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.GenericAttributeProfile.GattProtectionLevel;i4)");
}
impl ::windows::core::DefaultType for GattProtectionLevel {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
pub struct GattProtocolError {}
impl GattProtocolError {
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn InvalidHandle() -> ::windows::core::Result<u8> {
        Self::IGattProtocolErrorStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn ReadNotPermitted() -> ::windows::core::Result<u8> {
        Self::IGattProtocolErrorStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn WriteNotPermitted() -> ::windows::core::Result<u8> {
        Self::IGattProtocolErrorStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn InvalidPdu() -> ::windows::core::Result<u8> {
        Self::IGattProtocolErrorStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn InsufficientAuthentication() -> ::windows::core::Result<u8> {
        Self::IGattProtocolErrorStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn RequestNotSupported() -> ::windows::core::Result<u8> {
        Self::IGattProtocolErrorStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn InvalidOffset() -> ::windows::core::Result<u8> {
        Self::IGattProtocolErrorStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn InsufficientAuthorization() -> ::windows::core::Result<u8> {
        Self::IGattProtocolErrorStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn PrepareQueueFull() -> ::windows::core::Result<u8> {
        Self::IGattProtocolErrorStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn AttributeNotFound() -> ::windows::core::Result<u8> {
        Self::IGattProtocolErrorStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn AttributeNotLong() -> ::windows::core::Result<u8> {
        Self::IGattProtocolErrorStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn InsufficientEncryptionKeySize() -> ::windows::core::Result<u8> {
        Self::IGattProtocolErrorStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn InvalidAttributeValueLength() -> ::windows::core::Result<u8> {
        Self::IGattProtocolErrorStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn UnlikelyError() -> ::windows::core::Result<u8> {
        Self::IGattProtocolErrorStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn InsufficientEncryption() -> ::windows::core::Result<u8> {
        Self::IGattProtocolErrorStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn UnsupportedGroupType() -> ::windows::core::Result<u8> {
        Self::IGattProtocolErrorStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn InsufficientResources() -> ::windows::core::Result<u8> {
        Self::IGattProtocolErrorStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    pub fn IGattProtocolErrorStatics<R, F: FnOnce(&IGattProtocolErrorStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GattProtocolError, IGattProtocolErrorStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for GattProtocolError {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattProtocolError";
}
#[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GattReadClientCharacteristicConfigurationDescriptorResult(pub ::windows::core::IInspectable);
impl GattReadClientCharacteristicConfigurationDescriptorResult {
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn Status(&self) -> ::windows::core::Result<GattCommunicationStatus> {
        let this = self;
        unsafe {
            let mut result__: GattCommunicationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GattCommunicationStatus>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn ClientCharacteristicConfigurationDescriptor(&self) -> ::windows::core::Result<GattClientCharacteristicConfigurationDescriptorValue> {
        let this = self;
        unsafe {
            let mut result__: GattClientCharacteristicConfigurationDescriptorValue = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GattClientCharacteristicConfigurationDescriptorValue>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn ProtocolError(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u8>> {
        let this = &::windows::core::Interface::cast::<IGattReadClientCharacteristicConfigurationDescriptorResult2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<u8>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for GattReadClientCharacteristicConfigurationDescriptorResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.GenericAttributeProfile.GattReadClientCharacteristicConfigurationDescriptorResult;{63a66f09-1aea-4c4c-a50f-97bae474b348})");
}
unsafe impl ::windows::core::Interface for GattReadClientCharacteristicConfigurationDescriptorResult {
    type Vtable = IGattReadClientCharacteristicConfigurationDescriptorResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x63a66f09_1aea_4c4c_a50f_97bae474b348);
}
impl ::windows::core::RuntimeName for GattReadClientCharacteristicConfigurationDescriptorResult {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattReadClientCharacteristicConfigurationDescriptorResult";
}
impl ::core::convert::From<GattReadClientCharacteristicConfigurationDescriptorResult> for ::windows::core::IUnknown {
    fn from(value: GattReadClientCharacteristicConfigurationDescriptorResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GattReadClientCharacteristicConfigurationDescriptorResult> for ::windows::core::IUnknown {
    fn from(value: &GattReadClientCharacteristicConfigurationDescriptorResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GattReadClientCharacteristicConfigurationDescriptorResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GattReadClientCharacteristicConfigurationDescriptorResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GattReadClientCharacteristicConfigurationDescriptorResult> for ::windows::core::IInspectable {
    fn from(value: GattReadClientCharacteristicConfigurationDescriptorResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GattReadClientCharacteristicConfigurationDescriptorResult> for ::windows::core::IInspectable {
    fn from(value: &GattReadClientCharacteristicConfigurationDescriptorResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GattReadClientCharacteristicConfigurationDescriptorResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GattReadClientCharacteristicConfigurationDescriptorResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GattReadClientCharacteristicConfigurationDescriptorResult {}
unsafe impl ::core::marker::Sync for GattReadClientCharacteristicConfigurationDescriptorResult {}
#[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GattReadRequest(pub ::windows::core::IInspectable);
impl GattReadRequest {
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn Offset(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn Length(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn State(&self) -> ::windows::core::Result<GattRequestState> {
        let this = self;
        unsafe {
            let mut result__: GattRequestState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GattRequestState>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn StateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<GattReadRequest, GattRequestStateChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn RemoveStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Storage_Streams`*"]
    pub fn RespondWithValue<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn RespondWithProtocolError(&self, protocolerror: u8) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), protocolerror).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for GattReadRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.GenericAttributeProfile.GattReadRequest;{f1dd6535-6acd-42a6-a4bb-d789dae0043e})");
}
unsafe impl ::windows::core::Interface for GattReadRequest {
    type Vtable = IGattReadRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf1dd6535_6acd_42a6_a4bb_d789dae0043e);
}
impl ::windows::core::RuntimeName for GattReadRequest {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattReadRequest";
}
impl ::core::convert::From<GattReadRequest> for ::windows::core::IUnknown {
    fn from(value: GattReadRequest) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GattReadRequest> for ::windows::core::IUnknown {
    fn from(value: &GattReadRequest) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GattReadRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GattReadRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GattReadRequest> for ::windows::core::IInspectable {
    fn from(value: GattReadRequest) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GattReadRequest> for ::windows::core::IInspectable {
    fn from(value: &GattReadRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GattReadRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GattReadRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GattReadRequest {}
unsafe impl ::core::marker::Sync for GattReadRequest {}
#[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GattReadRequestedEventArgs(pub ::windows::core::IInspectable);
impl GattReadRequestedEventArgs {
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn Session(&self) -> ::windows::core::Result<GattSession> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GattSession>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn GetRequestAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattReadRequest>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<GattReadRequest>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for GattReadRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.GenericAttributeProfile.GattReadRequestedEventArgs;{93497243-f39c-484b-8ab6-996ba486cfa3})");
}
unsafe impl ::windows::core::Interface for GattReadRequestedEventArgs {
    type Vtable = IGattReadRequestedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x93497243_f39c_484b_8ab6_996ba486cfa3);
}
impl ::windows::core::RuntimeName for GattReadRequestedEventArgs {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattReadRequestedEventArgs";
}
impl ::core::convert::From<GattReadRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: GattReadRequestedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GattReadRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &GattReadRequestedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GattReadRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GattReadRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GattReadRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: GattReadRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GattReadRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &GattReadRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GattReadRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GattReadRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GattReadRequestedEventArgs {}
unsafe impl ::core::marker::Sync for GattReadRequestedEventArgs {}
#[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GattReadResult(pub ::windows::core::IInspectable);
impl GattReadResult {
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn Status(&self) -> ::windows::core::Result<GattCommunicationStatus> {
        let this = self;
        unsafe {
            let mut result__: GattCommunicationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GattCommunicationStatus>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Storage_Streams`*"]
    pub fn Value(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn ProtocolError(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u8>> {
        let this = &::windows::core::Interface::cast::<IGattReadResult2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<u8>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for GattReadResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.GenericAttributeProfile.GattReadResult;{63a66f08-1aea-4c4c-a50f-97bae474b348})");
}
unsafe impl ::windows::core::Interface for GattReadResult {
    type Vtable = IGattReadResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x63a66f08_1aea_4c4c_a50f_97bae474b348);
}
impl ::windows::core::RuntimeName for GattReadResult {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattReadResult";
}
impl ::core::convert::From<GattReadResult> for ::windows::core::IUnknown {
    fn from(value: GattReadResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GattReadResult> for ::windows::core::IUnknown {
    fn from(value: &GattReadResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GattReadResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GattReadResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GattReadResult> for ::windows::core::IInspectable {
    fn from(value: GattReadResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GattReadResult> for ::windows::core::IInspectable {
    fn from(value: &GattReadResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GattReadResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GattReadResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GattReadResult {}
unsafe impl ::core::marker::Sync for GattReadResult {}
#[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GattReliableWriteTransaction(pub ::windows::core::IInspectable);
impl GattReliableWriteTransaction {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GattReliableWriteTransaction, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Storage_Streams`*"]
    pub fn WriteValue<'a, Param0: ::windows::core::IntoParam<'a, GattCharacteristic>, Param1: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(&self, characteristic: Param0, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), characteristic.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn CommitAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattCommunicationStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<GattCommunicationStatus>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn CommitWithResultAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattWriteResult>> {
        let this = &::windows::core::Interface::cast::<IGattReliableWriteTransaction2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<GattWriteResult>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for GattReliableWriteTransaction {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.GenericAttributeProfile.GattReliableWriteTransaction;{63a66f07-1aea-4c4c-a50f-97bae474b348})");
}
unsafe impl ::windows::core::Interface for GattReliableWriteTransaction {
    type Vtable = IGattReliableWriteTransaction_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x63a66f07_1aea_4c4c_a50f_97bae474b348);
}
impl ::windows::core::RuntimeName for GattReliableWriteTransaction {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattReliableWriteTransaction";
}
impl ::core::convert::From<GattReliableWriteTransaction> for ::windows::core::IUnknown {
    fn from(value: GattReliableWriteTransaction) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GattReliableWriteTransaction> for ::windows::core::IUnknown {
    fn from(value: &GattReliableWriteTransaction) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GattReliableWriteTransaction {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GattReliableWriteTransaction {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GattReliableWriteTransaction> for ::windows::core::IInspectable {
    fn from(value: GattReliableWriteTransaction) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GattReliableWriteTransaction> for ::windows::core::IInspectable {
    fn from(value: &GattReliableWriteTransaction) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GattReliableWriteTransaction {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GattReliableWriteTransaction {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GattReliableWriteTransaction {}
unsafe impl ::core::marker::Sync for GattReliableWriteTransaction {}
#[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GattRequestState(pub i32);
impl GattRequestState {
    pub const Pending: GattRequestState = GattRequestState(0i32);
    pub const Completed: GattRequestState = GattRequestState(1i32);
    pub const Canceled: GattRequestState = GattRequestState(2i32);
}
impl ::core::convert::From<i32> for GattRequestState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for GattRequestState {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for GattRequestState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.GenericAttributeProfile.GattRequestState;i4)");
}
impl ::windows::core::DefaultType for GattRequestState {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GattRequestStateChangedEventArgs(pub ::windows::core::IInspectable);
impl GattRequestStateChangedEventArgs {
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn State(&self) -> ::windows::core::Result<GattRequestState> {
        let this = self;
        unsafe {
            let mut result__: GattRequestState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GattRequestState>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn Error(&self) -> ::windows::core::Result<super::BluetoothError> {
        let this = self;
        unsafe {
            let mut result__: super::BluetoothError = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::BluetoothError>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for GattRequestStateChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.GenericAttributeProfile.GattRequestStateChangedEventArgs;{e834d92c-27be-44b3-9d0d-4fc6e808dd3f})");
}
unsafe impl ::windows::core::Interface for GattRequestStateChangedEventArgs {
    type Vtable = IGattRequestStateChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe834d92c_27be_44b3_9d0d_4fc6e808dd3f);
}
impl ::windows::core::RuntimeName for GattRequestStateChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattRequestStateChangedEventArgs";
}
impl ::core::convert::From<GattRequestStateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: GattRequestStateChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GattRequestStateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &GattRequestStateChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GattRequestStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GattRequestStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GattRequestStateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: GattRequestStateChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GattRequestStateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &GattRequestStateChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GattRequestStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GattRequestStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GattRequestStateChangedEventArgs {}
unsafe impl ::core::marker::Sync for GattRequestStateChangedEventArgs {}
#[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GattServiceProvider(pub ::windows::core::IInspectable);
impl GattServiceProvider {
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn Service(&self) -> ::windows::core::Result<GattLocalService> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GattLocalService>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn AdvertisementStatus(&self) -> ::windows::core::Result<GattServiceProviderAdvertisementStatus> {
        let this = self;
        unsafe {
            let mut result__: GattServiceProviderAdvertisementStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GattServiceProviderAdvertisementStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn AdvertisementStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<GattServiceProvider, GattServiceProviderAdvertisementStatusChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn RemoveAdvertisementStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn StartAdvertising(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn StartAdvertisingWithParameters<'a, Param0: ::windows::core::IntoParam<'a, GattServiceProviderAdvertisingParameters>>(&self, parameters: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), parameters.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn StopAdvertising(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn CreateAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(serviceuuid: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattServiceProviderResult>> {
        Self::IGattServiceProviderStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), serviceuuid.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<GattServiceProviderResult>>(result__)
        })
    }
    pub fn IGattServiceProviderStatics<R, F: FnOnce(&IGattServiceProviderStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GattServiceProvider, IGattServiceProviderStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for GattServiceProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.GenericAttributeProfile.GattServiceProvider;{7822b3cd-2889-4f86-a051-3f0aed1c2760})");
}
unsafe impl ::windows::core::Interface for GattServiceProvider {
    type Vtable = IGattServiceProvider_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7822b3cd_2889_4f86_a051_3f0aed1c2760);
}
impl ::windows::core::RuntimeName for GattServiceProvider {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattServiceProvider";
}
impl ::core::convert::From<GattServiceProvider> for ::windows::core::IUnknown {
    fn from(value: GattServiceProvider) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GattServiceProvider> for ::windows::core::IUnknown {
    fn from(value: &GattServiceProvider) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GattServiceProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GattServiceProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GattServiceProvider> for ::windows::core::IInspectable {
    fn from(value: GattServiceProvider) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GattServiceProvider> for ::windows::core::IInspectable {
    fn from(value: &GattServiceProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GattServiceProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GattServiceProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GattServiceProvider {}
unsafe impl ::core::marker::Sync for GattServiceProvider {}
#[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GattServiceProviderAdvertisementStatus(pub i32);
impl GattServiceProviderAdvertisementStatus {
    pub const Created: GattServiceProviderAdvertisementStatus = GattServiceProviderAdvertisementStatus(0i32);
    pub const Stopped: GattServiceProviderAdvertisementStatus = GattServiceProviderAdvertisementStatus(1i32);
    pub const Started: GattServiceProviderAdvertisementStatus = GattServiceProviderAdvertisementStatus(2i32);
    pub const Aborted: GattServiceProviderAdvertisementStatus = GattServiceProviderAdvertisementStatus(3i32);
    pub const StartedWithoutAllAdvertisementData: GattServiceProviderAdvertisementStatus = GattServiceProviderAdvertisementStatus(4i32);
}
impl ::core::convert::From<i32> for GattServiceProviderAdvertisementStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for GattServiceProviderAdvertisementStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for GattServiceProviderAdvertisementStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.GenericAttributeProfile.GattServiceProviderAdvertisementStatus;i4)");
}
impl ::windows::core::DefaultType for GattServiceProviderAdvertisementStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GattServiceProviderAdvertisementStatusChangedEventArgs(pub ::windows::core::IInspectable);
impl GattServiceProviderAdvertisementStatusChangedEventArgs {
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn Error(&self) -> ::windows::core::Result<super::BluetoothError> {
        let this = self;
        unsafe {
            let mut result__: super::BluetoothError = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::BluetoothError>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn Status(&self) -> ::windows::core::Result<GattServiceProviderAdvertisementStatus> {
        let this = self;
        unsafe {
            let mut result__: GattServiceProviderAdvertisementStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GattServiceProviderAdvertisementStatus>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for GattServiceProviderAdvertisementStatusChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.GenericAttributeProfile.GattServiceProviderAdvertisementStatusChangedEventArgs;{59a5aa65-fa21-4ffc-b155-04d928012686})");
}
unsafe impl ::windows::core::Interface for GattServiceProviderAdvertisementStatusChangedEventArgs {
    type Vtable = IGattServiceProviderAdvertisementStatusChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x59a5aa65_fa21_4ffc_b155_04d928012686);
}
impl ::windows::core::RuntimeName for GattServiceProviderAdvertisementStatusChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattServiceProviderAdvertisementStatusChangedEventArgs";
}
impl ::core::convert::From<GattServiceProviderAdvertisementStatusChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: GattServiceProviderAdvertisementStatusChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GattServiceProviderAdvertisementStatusChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &GattServiceProviderAdvertisementStatusChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GattServiceProviderAdvertisementStatusChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GattServiceProviderAdvertisementStatusChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GattServiceProviderAdvertisementStatusChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: GattServiceProviderAdvertisementStatusChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GattServiceProviderAdvertisementStatusChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &GattServiceProviderAdvertisementStatusChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GattServiceProviderAdvertisementStatusChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GattServiceProviderAdvertisementStatusChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GattServiceProviderAdvertisementStatusChangedEventArgs {}
unsafe impl ::core::marker::Sync for GattServiceProviderAdvertisementStatusChangedEventArgs {}
#[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GattServiceProviderAdvertisingParameters(pub ::windows::core::IInspectable);
impl GattServiceProviderAdvertisingParameters {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GattServiceProviderAdvertisingParameters, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn SetIsConnectable(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn IsConnectable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn SetIsDiscoverable(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn IsDiscoverable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Storage_Streams`*"]
    pub fn SetServiceData<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGattServiceProviderAdvertisingParameters2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Storage_Streams`*"]
    pub fn ServiceData(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = &::windows::core::Interface::cast::<IGattServiceProviderAdvertisingParameters2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for GattServiceProviderAdvertisingParameters {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.GenericAttributeProfile.GattServiceProviderAdvertisingParameters;{e2ce31ab-6315-4c22-9bd7-781dbc3d8d82})");
}
unsafe impl ::windows::core::Interface for GattServiceProviderAdvertisingParameters {
    type Vtable = IGattServiceProviderAdvertisingParameters_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe2ce31ab_6315_4c22_9bd7_781dbc3d8d82);
}
impl ::windows::core::RuntimeName for GattServiceProviderAdvertisingParameters {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattServiceProviderAdvertisingParameters";
}
impl ::core::convert::From<GattServiceProviderAdvertisingParameters> for ::windows::core::IUnknown {
    fn from(value: GattServiceProviderAdvertisingParameters) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GattServiceProviderAdvertisingParameters> for ::windows::core::IUnknown {
    fn from(value: &GattServiceProviderAdvertisingParameters) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GattServiceProviderAdvertisingParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GattServiceProviderAdvertisingParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GattServiceProviderAdvertisingParameters> for ::windows::core::IInspectable {
    fn from(value: GattServiceProviderAdvertisingParameters) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GattServiceProviderAdvertisingParameters> for ::windows::core::IInspectable {
    fn from(value: &GattServiceProviderAdvertisingParameters) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GattServiceProviderAdvertisingParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GattServiceProviderAdvertisingParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GattServiceProviderAdvertisingParameters {}
unsafe impl ::core::marker::Sync for GattServiceProviderAdvertisingParameters {}
#[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GattServiceProviderResult(pub ::windows::core::IInspectable);
impl GattServiceProviderResult {
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn Error(&self) -> ::windows::core::Result<super::BluetoothError> {
        let this = self;
        unsafe {
            let mut result__: super::BluetoothError = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::BluetoothError>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn ServiceProvider(&self) -> ::windows::core::Result<GattServiceProvider> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GattServiceProvider>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for GattServiceProviderResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.GenericAttributeProfile.GattServiceProviderResult;{764696d8-c53e-428c-8a48-67afe02c3ae6})");
}
unsafe impl ::windows::core::Interface for GattServiceProviderResult {
    type Vtable = IGattServiceProviderResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x764696d8_c53e_428c_8a48_67afe02c3ae6);
}
impl ::windows::core::RuntimeName for GattServiceProviderResult {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattServiceProviderResult";
}
impl ::core::convert::From<GattServiceProviderResult> for ::windows::core::IUnknown {
    fn from(value: GattServiceProviderResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GattServiceProviderResult> for ::windows::core::IUnknown {
    fn from(value: &GattServiceProviderResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GattServiceProviderResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GattServiceProviderResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GattServiceProviderResult> for ::windows::core::IInspectable {
    fn from(value: GattServiceProviderResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GattServiceProviderResult> for ::windows::core::IInspectable {
    fn from(value: &GattServiceProviderResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GattServiceProviderResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GattServiceProviderResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GattServiceProviderResult {}
unsafe impl ::core::marker::Sync for GattServiceProviderResult {}
#[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
pub struct GattServiceUuids {}
impl GattServiceUuids {
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn Battery() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattServiceUuidsStatics(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn BloodPressure() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattServiceUuidsStatics(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn CyclingSpeedAndCadence() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattServiceUuidsStatics(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn GenericAccess() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattServiceUuidsStatics(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn GenericAttribute() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattServiceUuidsStatics(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn Glucose() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattServiceUuidsStatics(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn HealthThermometer() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattServiceUuidsStatics(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn HeartRate() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattServiceUuidsStatics(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn RunningSpeedAndCadence() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattServiceUuidsStatics(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn AlertNotification() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattServiceUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn CurrentTime() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattServiceUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn CyclingPower() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattServiceUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn DeviceInformation() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattServiceUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn HumanInterfaceDevice() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattServiceUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn ImmediateAlert() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattServiceUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn LinkLoss() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattServiceUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn LocationAndNavigation() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattServiceUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn NextDstChange() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattServiceUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn PhoneAlertStatus() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattServiceUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn ReferenceTimeUpdate() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattServiceUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn ScanParameters() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattServiceUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn TxPower() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGattServiceUuidsStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    pub fn IGattServiceUuidsStatics<R, F: FnOnce(&IGattServiceUuidsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GattServiceUuids, IGattServiceUuidsStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IGattServiceUuidsStatics2<R, F: FnOnce(&IGattServiceUuidsStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GattServiceUuids, IGattServiceUuidsStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for GattServiceUuids {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattServiceUuids";
}
#[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GattSession(pub ::windows::core::IInspectable);
impl GattSession {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn DeviceId(&self) -> ::windows::core::Result<super::BluetoothDeviceId> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::BluetoothDeviceId>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn CanMaintainConnection(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn SetMaintainConnection(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn MaintainConnection(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn MaxPduSize(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn SessionStatus(&self) -> ::windows::core::Result<GattSessionStatus> {
        let this = self;
        unsafe {
            let mut result__: GattSessionStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GattSessionStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn MaxPduSizeChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<GattSession, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn RemoveMaxPduSizeChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn SessionStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<GattSession, GattSessionStatusChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn RemoveSessionStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn FromDeviceIdAsync<'a, Param0: ::windows::core::IntoParam<'a, super::BluetoothDeviceId>>(deviceid: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattSession>> {
        Self::IGattSessionStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<GattSession>>(result__)
        })
    }
    pub fn IGattSessionStatics<R, F: FnOnce(&IGattSessionStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GattSession, IGattSessionStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for GattSession {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.GenericAttributeProfile.GattSession;{d23b5143-e04e-4c24-999c-9c256f9856b1})");
}
unsafe impl ::windows::core::Interface for GattSession {
    type Vtable = IGattSession_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd23b5143_e04e_4c24_999c_9c256f9856b1);
}
impl ::windows::core::RuntimeName for GattSession {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattSession";
}
impl ::core::convert::From<GattSession> for ::windows::core::IUnknown {
    fn from(value: GattSession) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GattSession> for ::windows::core::IUnknown {
    fn from(value: &GattSession) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GattSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GattSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GattSession> for ::windows::core::IInspectable {
    fn from(value: GattSession) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GattSession> for ::windows::core::IInspectable {
    fn from(value: &GattSession) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GattSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GattSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<GattSession> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: GattSession) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&GattSession> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &GattSession) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for GattSession {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for &GattSession {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for GattSession {}
unsafe impl ::core::marker::Sync for GattSession {}
#[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GattSessionStatus(pub i32);
impl GattSessionStatus {
    pub const Closed: GattSessionStatus = GattSessionStatus(0i32);
    pub const Active: GattSessionStatus = GattSessionStatus(1i32);
}
impl ::core::convert::From<i32> for GattSessionStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for GattSessionStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for GattSessionStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.GenericAttributeProfile.GattSessionStatus;i4)");
}
impl ::windows::core::DefaultType for GattSessionStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GattSessionStatusChangedEventArgs(pub ::windows::core::IInspectable);
impl GattSessionStatusChangedEventArgs {
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn Error(&self) -> ::windows::core::Result<super::BluetoothError> {
        let this = self;
        unsafe {
            let mut result__: super::BluetoothError = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::BluetoothError>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn Status(&self) -> ::windows::core::Result<GattSessionStatus> {
        let this = self;
        unsafe {
            let mut result__: GattSessionStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GattSessionStatus>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for GattSessionStatusChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.GenericAttributeProfile.GattSessionStatusChangedEventArgs;{7605b72e-837f-404c-ab34-3163f39ddf32})");
}
unsafe impl ::windows::core::Interface for GattSessionStatusChangedEventArgs {
    type Vtable = IGattSessionStatusChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7605b72e_837f_404c_ab34_3163f39ddf32);
}
impl ::windows::core::RuntimeName for GattSessionStatusChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattSessionStatusChangedEventArgs";
}
impl ::core::convert::From<GattSessionStatusChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: GattSessionStatusChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GattSessionStatusChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &GattSessionStatusChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GattSessionStatusChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GattSessionStatusChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GattSessionStatusChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: GattSessionStatusChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GattSessionStatusChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &GattSessionStatusChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GattSessionStatusChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GattSessionStatusChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GattSessionStatusChangedEventArgs {}
unsafe impl ::core::marker::Sync for GattSessionStatusChangedEventArgs {}
#[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GattSharingMode(pub i32);
impl GattSharingMode {
    pub const Unspecified: GattSharingMode = GattSharingMode(0i32);
    pub const Exclusive: GattSharingMode = GattSharingMode(1i32);
    pub const SharedReadOnly: GattSharingMode = GattSharingMode(2i32);
    pub const SharedReadAndWrite: GattSharingMode = GattSharingMode(3i32);
}
impl ::core::convert::From<i32> for GattSharingMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for GattSharingMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for GattSharingMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.GenericAttributeProfile.GattSharingMode;i4)");
}
impl ::windows::core::DefaultType for GattSharingMode {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GattSubscribedClient(pub ::windows::core::IInspectable);
impl GattSubscribedClient {
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn Session(&self) -> ::windows::core::Result<GattSession> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GattSession>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn MaxNotificationSize(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn MaxNotificationSizeChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<GattSubscribedClient, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn RemoveMaxNotificationSizeChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for GattSubscribedClient {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.GenericAttributeProfile.GattSubscribedClient;{736e9001-15a4-4ec2-9248-e3f20d463be9})");
}
unsafe impl ::windows::core::Interface for GattSubscribedClient {
    type Vtable = IGattSubscribedClient_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x736e9001_15a4_4ec2_9248_e3f20d463be9);
}
impl ::windows::core::RuntimeName for GattSubscribedClient {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattSubscribedClient";
}
impl ::core::convert::From<GattSubscribedClient> for ::windows::core::IUnknown {
    fn from(value: GattSubscribedClient) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GattSubscribedClient> for ::windows::core::IUnknown {
    fn from(value: &GattSubscribedClient) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GattSubscribedClient {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GattSubscribedClient {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GattSubscribedClient> for ::windows::core::IInspectable {
    fn from(value: GattSubscribedClient) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GattSubscribedClient> for ::windows::core::IInspectable {
    fn from(value: &GattSubscribedClient) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GattSubscribedClient {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GattSubscribedClient {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GattSubscribedClient {}
unsafe impl ::core::marker::Sync for GattSubscribedClient {}
#[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GattValueChangedEventArgs(pub ::windows::core::IInspectable);
impl GattValueChangedEventArgs {
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Storage_Streams`*"]
    pub fn CharacteristicValue(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for GattValueChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.GenericAttributeProfile.GattValueChangedEventArgs;{d21bdb54-06e3-4ed8-a263-acfac8ba7313})");
}
unsafe impl ::windows::core::Interface for GattValueChangedEventArgs {
    type Vtable = IGattValueChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd21bdb54_06e3_4ed8_a263_acfac8ba7313);
}
impl ::windows::core::RuntimeName for GattValueChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattValueChangedEventArgs";
}
impl ::core::convert::From<GattValueChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: GattValueChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GattValueChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &GattValueChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GattValueChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GattValueChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GattValueChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: GattValueChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GattValueChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &GattValueChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GattValueChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GattValueChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GattValueChangedEventArgs {}
unsafe impl ::core::marker::Sync for GattValueChangedEventArgs {}
#[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GattWriteOption(pub i32);
impl GattWriteOption {
    pub const WriteWithResponse: GattWriteOption = GattWriteOption(0i32);
    pub const WriteWithoutResponse: GattWriteOption = GattWriteOption(1i32);
}
impl ::core::convert::From<i32> for GattWriteOption {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for GattWriteOption {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for GattWriteOption {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.GenericAttributeProfile.GattWriteOption;i4)");
}
impl ::windows::core::DefaultType for GattWriteOption {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GattWriteRequest(pub ::windows::core::IInspectable);
impl GattWriteRequest {
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Storage_Streams`*"]
    pub fn Value(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn Offset(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn Option(&self) -> ::windows::core::Result<GattWriteOption> {
        let this = self;
        unsafe {
            let mut result__: GattWriteOption = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GattWriteOption>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn State(&self) -> ::windows::core::Result<GattRequestState> {
        let this = self;
        unsafe {
            let mut result__: GattRequestState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GattRequestState>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn StateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<GattWriteRequest, GattRequestStateChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn RemoveStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn Respond(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn RespondWithProtocolError(&self, protocolerror: u8) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), protocolerror).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for GattWriteRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.GenericAttributeProfile.GattWriteRequest;{aeb6a9ed-de2f-4fc2-a9a8-94ea7844f13d})");
}
unsafe impl ::windows::core::Interface for GattWriteRequest {
    type Vtable = IGattWriteRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaeb6a9ed_de2f_4fc2_a9a8_94ea7844f13d);
}
impl ::windows::core::RuntimeName for GattWriteRequest {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattWriteRequest";
}
impl ::core::convert::From<GattWriteRequest> for ::windows::core::IUnknown {
    fn from(value: GattWriteRequest) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GattWriteRequest> for ::windows::core::IUnknown {
    fn from(value: &GattWriteRequest) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GattWriteRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GattWriteRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GattWriteRequest> for ::windows::core::IInspectable {
    fn from(value: GattWriteRequest) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GattWriteRequest> for ::windows::core::IInspectable {
    fn from(value: &GattWriteRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GattWriteRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GattWriteRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GattWriteRequest {}
unsafe impl ::core::marker::Sync for GattWriteRequest {}
#[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GattWriteRequestedEventArgs(pub ::windows::core::IInspectable);
impl GattWriteRequestedEventArgs {
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn Session(&self) -> ::windows::core::Result<GattSession> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GattSession>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn GetRequestAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattWriteRequest>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<GattWriteRequest>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for GattWriteRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.GenericAttributeProfile.GattWriteRequestedEventArgs;{2dec8bbe-a73a-471a-94d5-037deadd0806})");
}
unsafe impl ::windows::core::Interface for GattWriteRequestedEventArgs {
    type Vtable = IGattWriteRequestedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2dec8bbe_a73a_471a_94d5_037deadd0806);
}
impl ::windows::core::RuntimeName for GattWriteRequestedEventArgs {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattWriteRequestedEventArgs";
}
impl ::core::convert::From<GattWriteRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: GattWriteRequestedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GattWriteRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &GattWriteRequestedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GattWriteRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GattWriteRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GattWriteRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: GattWriteRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GattWriteRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &GattWriteRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GattWriteRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GattWriteRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GattWriteRequestedEventArgs {}
unsafe impl ::core::marker::Sync for GattWriteRequestedEventArgs {}
#[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GattWriteResult(pub ::windows::core::IInspectable);
impl GattWriteResult {
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`*"]
    pub fn Status(&self) -> ::windows::core::Result<GattCommunicationStatus> {
        let this = self;
        unsafe {
            let mut result__: GattCommunicationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GattCommunicationStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_GenericAttributeProfile`, `Foundation`*"]
    pub fn ProtocolError(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u8>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<u8>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for GattWriteResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.GenericAttributeProfile.GattWriteResult;{4991ddb1-cb2b-44f7-99fc-d29a2871dc9b})");
}
unsafe impl ::windows::core::Interface for GattWriteResult {
    type Vtable = IGattWriteResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4991ddb1_cb2b_44f7_99fc_d29a2871dc9b);
}
impl ::windows::core::RuntimeName for GattWriteResult {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.GattWriteResult";
}
impl ::core::convert::From<GattWriteResult> for ::windows::core::IUnknown {
    fn from(value: GattWriteResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GattWriteResult> for ::windows::core::IUnknown {
    fn from(value: &GattWriteResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GattWriteResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GattWriteResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GattWriteResult> for ::windows::core::IInspectable {
    fn from(value: GattWriteResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GattWriteResult> for ::windows::core::IInspectable {
    fn from(value: &GattWriteResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GattWriteResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GattWriteResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GattWriteResult {}
unsafe impl ::core::marker::Sync for GattWriteResult {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IGattCharacteristic(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGattCharacteristic {
    type Vtable = IGattCharacteristic_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x59cb50c1_5934_4f68_a198_eb864fa44e6b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattCharacteristic_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, descriptoruuid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut GattCharacteristicProperties) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut GattProtectionLevel) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: GattProtectionLevel) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cachemode: super::BluetoothCacheMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, writeoption: GattWriteOption, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, clientcharacteristicconfigurationdescriptorvalue: GattClientCharacteristicConfigurationDescriptorValue, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, valuechangedhandler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, valuechangedeventcookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGattCharacteristic2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGattCharacteristic2 {
    type Vtable = IGattCharacteristic2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae1ab578_ec06_4764_b780_9835a1d35d6e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattCharacteristic2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGattCharacteristic3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGattCharacteristic3 {
    type Vtable = IGattCharacteristic3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3f3c663e_93d4_406b_b817_db81f8ed53b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattCharacteristic3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cachemode: super::BluetoothCacheMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, descriptoruuid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, descriptoruuid: ::windows::core::GUID, cachemode: super::BluetoothCacheMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, writeoption: GattWriteOption, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, clientcharacteristicconfigurationdescriptorvalue: GattClientCharacteristicConfigurationDescriptorValue, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGattCharacteristicStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGattCharacteristicStatics {
    type Vtable = IGattCharacteristicStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x59cb50c3_5934_4f68_a198_eb864fa44e6b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattCharacteristicStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, shortid: u16, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGattCharacteristicUuidsStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGattCharacteristicUuidsStatics {
    type Vtable = IGattCharacteristicUuidsStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x58fa4586_b1de_470c_b7de_0d11ff44f4b7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattCharacteristicUuidsStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGattCharacteristicUuidsStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGattCharacteristicUuidsStatics2 {
    type Vtable = IGattCharacteristicUuidsStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1855b425_d46e_4a2c_9c3f_ed6dea29e7be);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattCharacteristicUuidsStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGattCharacteristicsResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGattCharacteristicsResult {
    type Vtable = IGattCharacteristicsResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1194945c_b257_4f3e_9db7_f68bc9a9aef2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattCharacteristicsResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut GattCommunicationStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGattClientNotificationResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGattClientNotificationResult {
    type Vtable = IGattClientNotificationResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x506d5599_0112_419a_8e3b_ae21afabd2c2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattClientNotificationResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut GattCommunicationStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGattClientNotificationResult2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGattClientNotificationResult2 {
    type Vtable = IGattClientNotificationResult2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8faec497_45e0_497e_9582_29a1fe281ad5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattClientNotificationResult2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGattDescriptor(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGattDescriptor {
    type Vtable = IGattDescriptor_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x92055f2b_8084_4344_b4c2_284de19a8506);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattDescriptor_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut GattProtectionLevel) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: GattProtectionLevel) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cachemode: super::BluetoothCacheMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGattDescriptor2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGattDescriptor2 {
    type Vtable = IGattDescriptor2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f563d39_d630_406c_ba11_10cdd16b0e5e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattDescriptor2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGattDescriptorStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGattDescriptorStatics {
    type Vtable = IGattDescriptorStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x92055f2d_8084_4344_b4c2_284de19a8506);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattDescriptorStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, shortid: u16, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGattDescriptorUuidsStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGattDescriptorUuidsStatics {
    type Vtable = IGattDescriptorUuidsStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa6f862ce_9cfc_42f1_9185_ff37b75181d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattDescriptorUuidsStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGattDescriptorsResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGattDescriptorsResult {
    type Vtable = IGattDescriptorsResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9bc091f3_95e7_4489_8d25_ff81955a57b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattDescriptorsResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut GattCommunicationStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGattDeviceService(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGattDeviceService {
    type Vtable = IGattDeviceService_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xac7b7c05_b33c_47cf_990f_6b8f5577df71);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattDeviceService_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, characteristicuuid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, serviceuuid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGattDeviceService2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGattDeviceService2 {
    type Vtable = IGattDeviceService2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfc54520b_0b0d_4708_bae0_9ffd9489bc59);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattDeviceService2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGattDeviceService3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGattDeviceService3 {
    type Vtable = IGattDeviceService3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb293a950_0c53_437c_a9b3_5c3210c6e569);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattDeviceService3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Enumeration")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut GattSharingMode) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Enumeration", feature = "Foundation")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sharingmode: GattSharingMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cachemode: super::BluetoothCacheMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, characteristicuuid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, characteristicuuid: ::windows::core::GUID, cachemode: super::BluetoothCacheMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cachemode: super::BluetoothCacheMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, serviceuuid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, serviceuuid: ::windows::core::GUID, cachemode: super::BluetoothCacheMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGattDeviceServiceStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGattDeviceServiceStatics {
    type Vtable = IGattDeviceServiceStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x196d0022_faad_45dc_ae5b_2ac3184e84db);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattDeviceServiceStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, serviceuuid: ::windows::core::GUID, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, serviceshortid: u16, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, shortid: u16, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGattDeviceServiceStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGattDeviceServiceStatics2 {
    type Vtable = IGattDeviceServiceStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0604186e_24a6_4b0d_a2f2_30cc01545d25);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattDeviceServiceStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sharingmode: GattSharingMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bluetoothdeviceid: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bluetoothdeviceid: ::windows::core::RawPtr, cachemode: super::BluetoothCacheMode, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bluetoothdeviceid: ::windows::core::RawPtr, serviceuuid: ::windows::core::GUID, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bluetoothdeviceid: ::windows::core::RawPtr, serviceuuid: ::windows::core::GUID, cachemode: super::BluetoothCacheMode, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGattDeviceServicesResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGattDeviceServicesResult {
    type Vtable = IGattDeviceServicesResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x171dd3ee_016d_419d_838a_576cf475a3d8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattDeviceServicesResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut GattCommunicationStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGattLocalCharacteristic(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGattLocalCharacteristic {
    type Vtable = IGattLocalCharacteristic_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaede376d_5412_4d74_92a8_8deb8526829c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattLocalCharacteristic_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut GattCharacteristicProperties) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut GattProtectionLevel) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut GattProtectionLevel) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, descriptoruuid: ::windows::core::GUID, parameters: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, subscribedclient: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGattLocalCharacteristicParameters(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGattLocalCharacteristicParameters {
    type Vtable = IGattLocalCharacteristicParameters_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfaf73db4_4cff_44c7_8445_040e6ead0063);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattLocalCharacteristicParameters_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: GattCharacteristicProperties) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut GattCharacteristicProperties) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: GattProtectionLevel) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut GattProtectionLevel) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: GattProtectionLevel) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut GattProtectionLevel) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGattLocalCharacteristicResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGattLocalCharacteristicResult {
    type Vtable = IGattLocalCharacteristicResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7975de9b_0170_4397_9666_92f863f12ee6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattLocalCharacteristicResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::BluetoothError) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGattLocalDescriptor(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGattLocalDescriptor {
    type Vtable = IGattLocalDescriptor_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf48ebe06_789d_4a4b_8652_bd017b5d2fc6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattLocalDescriptor_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut GattProtectionLevel) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut GattProtectionLevel) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGattLocalDescriptorParameters(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGattLocalDescriptorParameters {
    type Vtable = IGattLocalDescriptorParameters_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5fdede6a_f3c1_4b66_8c4b_e3d2293b40e9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattLocalDescriptorParameters_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: GattProtectionLevel) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut GattProtectionLevel) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: GattProtectionLevel) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut GattProtectionLevel) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGattLocalDescriptorResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGattLocalDescriptorResult {
    type Vtable = IGattLocalDescriptorResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x375791be_321f_4366_bfc1_3bc6b82c79f8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattLocalDescriptorResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::BluetoothError) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGattLocalService(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGattLocalService {
    type Vtable = IGattLocalService_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf513e258_f7f7_4902_b803_57fcc7d6fe83);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattLocalService_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, characteristicuuid: ::windows::core::GUID, parameters: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGattPresentationFormat(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGattPresentationFormat {
    type Vtable = IGattPresentationFormat_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x196d0021_faad_45dc_ae5b_2ac3184e84db);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattPresentationFormat_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGattPresentationFormatStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGattPresentationFormatStatics {
    type Vtable = IGattPresentationFormatStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x196d0020_faad_45dc_ae5b_2ac3184e84db);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattPresentationFormatStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u8) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGattPresentationFormatStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGattPresentationFormatStatics2 {
    type Vtable = IGattPresentationFormatStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9c21713_b82f_435e_b634_21fd85a43c07);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattPresentationFormatStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, formattype: u8, exponent: i32, unit: u16, namespaceid: u8, description: u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGattPresentationFormatTypesStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGattPresentationFormatTypesStatics {
    type Vtable = IGattPresentationFormatTypesStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfaf1ba0a_30ba_409c_bef7_cffb6d03b8fb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattPresentationFormatTypesStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u8) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGattProtocolErrorStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGattProtocolErrorStatics {
    type Vtable = IGattProtocolErrorStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xca46c5c5_0ecc_4809_bea3_cf79bc991e37);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattProtocolErrorStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u8) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGattReadClientCharacteristicConfigurationDescriptorResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGattReadClientCharacteristicConfigurationDescriptorResult {
    type Vtable = IGattReadClientCharacteristicConfigurationDescriptorResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x63a66f09_1aea_4c4c_a50f_97bae474b348);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattReadClientCharacteristicConfigurationDescriptorResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut GattCommunicationStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut GattClientCharacteristicConfigurationDescriptorValue) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGattReadClientCharacteristicConfigurationDescriptorResult2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGattReadClientCharacteristicConfigurationDescriptorResult2 {
    type Vtable = IGattReadClientCharacteristicConfigurationDescriptorResult2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1bf1a59d_ba4d_4622_8651_f4ee150d0a5d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattReadClientCharacteristicConfigurationDescriptorResult2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGattReadRequest(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGattReadRequest {
    type Vtable = IGattReadRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf1dd6535_6acd_42a6_a4bb_d789dae0043e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattReadRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut GattRequestState) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, protocolerror: u8) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGattReadRequestedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGattReadRequestedEventArgs {
    type Vtable = IGattReadRequestedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x93497243_f39c_484b_8ab6_996ba486cfa3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattReadRequestedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGattReadResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGattReadResult {
    type Vtable = IGattReadResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x63a66f08_1aea_4c4c_a50f_97bae474b348);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattReadResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut GattCommunicationStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGattReadResult2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGattReadResult2 {
    type Vtable = IGattReadResult2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa10f50a0_fb43_48af_baaa_638a5c6329fe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattReadResult2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGattReliableWriteTransaction(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGattReliableWriteTransaction {
    type Vtable = IGattReliableWriteTransaction_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x63a66f07_1aea_4c4c_a50f_97bae474b348);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattReliableWriteTransaction_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, characteristic: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGattReliableWriteTransaction2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGattReliableWriteTransaction2 {
    type Vtable = IGattReliableWriteTransaction2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51113987_ef12_462f_9fb2_a1a43a679416);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattReliableWriteTransaction2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGattRequestStateChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGattRequestStateChangedEventArgs {
    type Vtable = IGattRequestStateChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe834d92c_27be_44b3_9d0d_4fc6e808dd3f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattRequestStateChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut GattRequestState) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::BluetoothError) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGattServiceProvider(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGattServiceProvider {
    type Vtable = IGattServiceProvider_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7822b3cd_2889_4f86_a051_3f0aed1c2760);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattServiceProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut GattServiceProviderAdvertisementStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, parameters: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGattServiceProviderAdvertisementStatusChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGattServiceProviderAdvertisementStatusChangedEventArgs {
    type Vtable = IGattServiceProviderAdvertisementStatusChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x59a5aa65_fa21_4ffc_b155_04d928012686);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattServiceProviderAdvertisementStatusChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::BluetoothError) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut GattServiceProviderAdvertisementStatus) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGattServiceProviderAdvertisingParameters(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGattServiceProviderAdvertisingParameters {
    type Vtable = IGattServiceProviderAdvertisingParameters_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe2ce31ab_6315_4c22_9bd7_781dbc3d8d82);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattServiceProviderAdvertisingParameters_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGattServiceProviderAdvertisingParameters2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGattServiceProviderAdvertisingParameters2 {
    type Vtable = IGattServiceProviderAdvertisingParameters2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xff68468d_ca92_4434_9743_0e90988ad879);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattServiceProviderAdvertisingParameters2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGattServiceProviderResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGattServiceProviderResult {
    type Vtable = IGattServiceProviderResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x764696d8_c53e_428c_8a48_67afe02c3ae6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattServiceProviderResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::BluetoothError) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGattServiceProviderStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGattServiceProviderStatics {
    type Vtable = IGattServiceProviderStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x31794063_5256_4054_a4f4_7bbe7755a57e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattServiceProviderStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, serviceuuid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGattServiceUuidsStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGattServiceUuidsStatics {
    type Vtable = IGattServiceUuidsStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6dc57058_9aba_4417_b8f2_dce016d34ee2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattServiceUuidsStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGattServiceUuidsStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGattServiceUuidsStatics2 {
    type Vtable = IGattServiceUuidsStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd2ae94f5_3d15_4f79_9c0c_eaafa675155c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattServiceUuidsStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGattSession(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGattSession {
    type Vtable = IGattSession_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd23b5143_e04e_4c24_999c_9c256f9856b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattSession_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut GattSessionStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGattSessionStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGattSessionStatics {
    type Vtable = IGattSessionStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2e65b95c_539f_4db7_82a8_73bdbbf73ebf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattSessionStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceid: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGattSessionStatusChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGattSessionStatusChangedEventArgs {
    type Vtable = IGattSessionStatusChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7605b72e_837f_404c_ab34_3163f39ddf32);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattSessionStatusChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::BluetoothError) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut GattSessionStatus) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGattSubscribedClient(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGattSubscribedClient {
    type Vtable = IGattSubscribedClient_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x736e9001_15a4_4ec2_9248_e3f20d463be9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattSubscribedClient_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGattValueChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGattValueChangedEventArgs {
    type Vtable = IGattValueChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd21bdb54_06e3_4ed8_a263_acfac8ba7313);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattValueChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGattWriteRequest(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGattWriteRequest {
    type Vtable = IGattWriteRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaeb6a9ed_de2f_4fc2_a9a8_94ea7844f13d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattWriteRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut GattWriteOption) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut GattRequestState) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, protocolerror: u8) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGattWriteRequestedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGattWriteRequestedEventArgs {
    type Vtable = IGattWriteRequestedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2dec8bbe_a73a_471a_94d5_037deadd0806);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattWriteRequestedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGattWriteResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGattWriteResult {
    type Vtable = IGattWriteResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4991ddb1_cb2b_44f7_99fc_d29a2871dc9b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattWriteResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut GattCommunicationStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
