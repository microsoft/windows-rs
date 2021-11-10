#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BluetoothLEAdvertisement(pub ::windows::runtime::IInspectable);
impl BluetoothLEAdvertisement {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<BluetoothLEAdvertisement, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`, `Foundation`*"]
    pub fn Flags(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<BluetoothLEAdvertisementFlags>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<BluetoothLEAdvertisementFlags>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`, `Foundation`*"]
    pub fn SetFlags<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IReference<BluetoothLEAdvertisementFlags>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
    pub fn LocalName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
    pub fn SetLocalName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`, `Foundation_Collections`*"]
    pub fn ServiceUuids(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVector<::windows::runtime::GUID>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<::windows::runtime::GUID>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`, `Foundation_Collections`*"]
    pub fn ManufacturerData(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVector<BluetoothLEManufacturerData>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<BluetoothLEManufacturerData>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`, `Foundation_Collections`*"]
    pub fn DataSections(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVector<BluetoothLEAdvertisementDataSection>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<BluetoothLEAdvertisementDataSection>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`, `Foundation_Collections`*"]
    pub fn GetManufacturerDataByCompanyId(&self, companyid: u16) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<BluetoothLEManufacturerData>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), companyid, &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<BluetoothLEManufacturerData>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`, `Foundation_Collections`*"]
    pub fn GetSectionsByType(&self, r#type: u8) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<BluetoothLEAdvertisementDataSection>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), r#type, &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<BluetoothLEAdvertisementDataSection>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BluetoothLEAdvertisement {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisement;{066fb2b7-33d1-4e7d-8367-cf81d0f79653})");
}
unsafe impl ::windows::runtime::Interface for BluetoothLEAdvertisement {
    type Vtable = IBluetoothLEAdvertisement_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x066fb2b7_33d1_4e7d_8367_cf81d0f79653);
}
impl ::windows::runtime::RuntimeName for BluetoothLEAdvertisement {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisement";
}
impl ::core::convert::From<BluetoothLEAdvertisement> for ::windows::runtime::IUnknown {
    fn from(value: BluetoothLEAdvertisement) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BluetoothLEAdvertisement> for ::windows::runtime::IUnknown {
    fn from(value: &BluetoothLEAdvertisement) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for BluetoothLEAdvertisement {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a BluetoothLEAdvertisement {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BluetoothLEAdvertisement> for ::windows::runtime::IInspectable {
    fn from(value: BluetoothLEAdvertisement) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BluetoothLEAdvertisement> for ::windows::runtime::IInspectable {
    fn from(value: &BluetoothLEAdvertisement) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for BluetoothLEAdvertisement {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a BluetoothLEAdvertisement {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for BluetoothLEAdvertisement {}
unsafe impl ::core::marker::Sync for BluetoothLEAdvertisement {}
#[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BluetoothLEAdvertisementBytePattern(pub ::windows::runtime::IInspectable);
impl BluetoothLEAdvertisementBytePattern {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<BluetoothLEAdvertisementBytePattern, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
    pub fn DataType(&self) -> ::windows::runtime::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
    pub fn SetDataType(&self, value: u8) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
    pub fn Offset(&self) -> ::windows::runtime::Result<i16> {
        let this = self;
        unsafe {
            let mut result__: i16 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i16>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
    pub fn SetOffset(&self, value: i16) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`, `Storage_Streams`*"]
    pub fn Data(&self) -> ::windows::runtime::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`, `Storage_Streams`*"]
    pub fn SetData<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`, `Storage_Streams`*"]
    pub fn Create<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(datatype: u8, offset: i16, data: Param2) -> ::windows::runtime::Result<BluetoothLEAdvertisementBytePattern> {
        Self::IBluetoothLEAdvertisementBytePatternFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), datatype, offset, data.into_param().abi(), &mut result__).from_abi::<BluetoothLEAdvertisementBytePattern>(result__)
        })
    }
    pub fn IBluetoothLEAdvertisementBytePatternFactory<R, F: FnOnce(&IBluetoothLEAdvertisementBytePatternFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<BluetoothLEAdvertisementBytePattern, IBluetoothLEAdvertisementBytePatternFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BluetoothLEAdvertisementBytePattern {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementBytePattern;{fbfad7f2-b9c5-4a08-bc51-502f8ef68a79})");
}
unsafe impl ::windows::runtime::Interface for BluetoothLEAdvertisementBytePattern {
    type Vtable = IBluetoothLEAdvertisementBytePattern_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xfbfad7f2_b9c5_4a08_bc51_502f8ef68a79);
}
impl ::windows::runtime::RuntimeName for BluetoothLEAdvertisementBytePattern {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementBytePattern";
}
impl ::core::convert::From<BluetoothLEAdvertisementBytePattern> for ::windows::runtime::IUnknown {
    fn from(value: BluetoothLEAdvertisementBytePattern) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BluetoothLEAdvertisementBytePattern> for ::windows::runtime::IUnknown {
    fn from(value: &BluetoothLEAdvertisementBytePattern) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for BluetoothLEAdvertisementBytePattern {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a BluetoothLEAdvertisementBytePattern {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BluetoothLEAdvertisementBytePattern> for ::windows::runtime::IInspectable {
    fn from(value: BluetoothLEAdvertisementBytePattern) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BluetoothLEAdvertisementBytePattern> for ::windows::runtime::IInspectable {
    fn from(value: &BluetoothLEAdvertisementBytePattern) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for BluetoothLEAdvertisementBytePattern {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a BluetoothLEAdvertisementBytePattern {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for BluetoothLEAdvertisementBytePattern {}
unsafe impl ::core::marker::Sync for BluetoothLEAdvertisementBytePattern {}
#[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BluetoothLEAdvertisementDataSection(pub ::windows::runtime::IInspectable);
impl BluetoothLEAdvertisementDataSection {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<BluetoothLEAdvertisementDataSection, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
    pub fn DataType(&self) -> ::windows::runtime::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
    pub fn SetDataType(&self, value: u8) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`, `Storage_Streams`*"]
    pub fn Data(&self) -> ::windows::runtime::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`, `Storage_Streams`*"]
    pub fn SetData<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`, `Storage_Streams`*"]
    pub fn Create<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(datatype: u8, data: Param1) -> ::windows::runtime::Result<BluetoothLEAdvertisementDataSection> {
        Self::IBluetoothLEAdvertisementDataSectionFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), datatype, data.into_param().abi(), &mut result__).from_abi::<BluetoothLEAdvertisementDataSection>(result__)
        })
    }
    pub fn IBluetoothLEAdvertisementDataSectionFactory<R, F: FnOnce(&IBluetoothLEAdvertisementDataSectionFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<BluetoothLEAdvertisementDataSection, IBluetoothLEAdvertisementDataSectionFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BluetoothLEAdvertisementDataSection {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementDataSection;{d7213314-3a43-40f9-b6f0-92bfefc34ae3})");
}
unsafe impl ::windows::runtime::Interface for BluetoothLEAdvertisementDataSection {
    type Vtable = IBluetoothLEAdvertisementDataSection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd7213314_3a43_40f9_b6f0_92bfefc34ae3);
}
impl ::windows::runtime::RuntimeName for BluetoothLEAdvertisementDataSection {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementDataSection";
}
impl ::core::convert::From<BluetoothLEAdvertisementDataSection> for ::windows::runtime::IUnknown {
    fn from(value: BluetoothLEAdvertisementDataSection) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BluetoothLEAdvertisementDataSection> for ::windows::runtime::IUnknown {
    fn from(value: &BluetoothLEAdvertisementDataSection) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for BluetoothLEAdvertisementDataSection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a BluetoothLEAdvertisementDataSection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BluetoothLEAdvertisementDataSection> for ::windows::runtime::IInspectable {
    fn from(value: BluetoothLEAdvertisementDataSection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BluetoothLEAdvertisementDataSection> for ::windows::runtime::IInspectable {
    fn from(value: &BluetoothLEAdvertisementDataSection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for BluetoothLEAdvertisementDataSection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a BluetoothLEAdvertisementDataSection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for BluetoothLEAdvertisementDataSection {}
unsafe impl ::core::marker::Sync for BluetoothLEAdvertisementDataSection {}
#[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
pub struct BluetoothLEAdvertisementDataTypes {}
impl BluetoothLEAdvertisementDataTypes {
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
    pub fn Flags() -> ::windows::runtime::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
    pub fn IncompleteService16BitUuids() -> ::windows::runtime::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
    pub fn CompleteService16BitUuids() -> ::windows::runtime::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
    pub fn IncompleteService32BitUuids() -> ::windows::runtime::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
    pub fn CompleteService32BitUuids() -> ::windows::runtime::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
    pub fn IncompleteService128BitUuids() -> ::windows::runtime::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
    pub fn CompleteService128BitUuids() -> ::windows::runtime::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
    pub fn ShortenedLocalName() -> ::windows::runtime::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
    pub fn CompleteLocalName() -> ::windows::runtime::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
    pub fn TxPowerLevel() -> ::windows::runtime::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
    pub fn SlaveConnectionIntervalRange() -> ::windows::runtime::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
    pub fn ServiceSolicitation16BitUuids() -> ::windows::runtime::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
    pub fn ServiceSolicitation32BitUuids() -> ::windows::runtime::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
    pub fn ServiceSolicitation128BitUuids() -> ::windows::runtime::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
    pub fn ServiceData16BitUuids() -> ::windows::runtime::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
    pub fn ServiceData32BitUuids() -> ::windows::runtime::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
    pub fn ServiceData128BitUuids() -> ::windows::runtime::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
    pub fn PublicTargetAddress() -> ::windows::runtime::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
    pub fn RandomTargetAddress() -> ::windows::runtime::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
    pub fn Appearance() -> ::windows::runtime::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
    pub fn AdvertisingInterval() -> ::windows::runtime::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
    pub fn ManufacturerSpecificData() -> ::windows::runtime::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    pub fn IBluetoothLEAdvertisementDataTypesStatics<R, F: FnOnce(&IBluetoothLEAdvertisementDataTypesStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<BluetoothLEAdvertisementDataTypes, IBluetoothLEAdvertisementDataTypesStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for BluetoothLEAdvertisementDataTypes {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementDataTypes";
}
#[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BluetoothLEAdvertisementFilter(pub ::windows::runtime::IInspectable);
impl BluetoothLEAdvertisementFilter {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<BluetoothLEAdvertisementFilter, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
    pub fn Advertisement(&self) -> ::windows::runtime::Result<BluetoothLEAdvertisement> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothLEAdvertisement>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
    pub fn SetAdvertisement<'a, Param0: ::windows::runtime::IntoParam<'a, BluetoothLEAdvertisement>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`, `Foundation_Collections`*"]
    pub fn BytePatterns(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVector<BluetoothLEAdvertisementBytePattern>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<BluetoothLEAdvertisementBytePattern>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BluetoothLEAdvertisementFilter {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementFilter;{131eb0d3-d04e-47b1-837e-49405bf6f80f})");
}
unsafe impl ::windows::runtime::Interface for BluetoothLEAdvertisementFilter {
    type Vtable = IBluetoothLEAdvertisementFilter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x131eb0d3_d04e_47b1_837e_49405bf6f80f);
}
impl ::windows::runtime::RuntimeName for BluetoothLEAdvertisementFilter {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementFilter";
}
impl ::core::convert::From<BluetoothLEAdvertisementFilter> for ::windows::runtime::IUnknown {
    fn from(value: BluetoothLEAdvertisementFilter) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BluetoothLEAdvertisementFilter> for ::windows::runtime::IUnknown {
    fn from(value: &BluetoothLEAdvertisementFilter) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for BluetoothLEAdvertisementFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a BluetoothLEAdvertisementFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BluetoothLEAdvertisementFilter> for ::windows::runtime::IInspectable {
    fn from(value: BluetoothLEAdvertisementFilter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BluetoothLEAdvertisementFilter> for ::windows::runtime::IInspectable {
    fn from(value: &BluetoothLEAdvertisementFilter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for BluetoothLEAdvertisementFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a BluetoothLEAdvertisementFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for BluetoothLEAdvertisementFilter {}
unsafe impl ::core::marker::Sync for BluetoothLEAdvertisementFilter {}
#[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct BluetoothLEAdvertisementFlags(pub u32);
impl BluetoothLEAdvertisementFlags {
    pub const None: BluetoothLEAdvertisementFlags = BluetoothLEAdvertisementFlags(0u32);
    pub const LimitedDiscoverableMode: BluetoothLEAdvertisementFlags = BluetoothLEAdvertisementFlags(1u32);
    pub const GeneralDiscoverableMode: BluetoothLEAdvertisementFlags = BluetoothLEAdvertisementFlags(2u32);
    pub const ClassicNotSupported: BluetoothLEAdvertisementFlags = BluetoothLEAdvertisementFlags(4u32);
    pub const DualModeControllerCapable: BluetoothLEAdvertisementFlags = BluetoothLEAdvertisementFlags(8u32);
    pub const DualModeHostCapable: BluetoothLEAdvertisementFlags = BluetoothLEAdvertisementFlags(16u32);
}
impl ::core::convert::From<u32> for BluetoothLEAdvertisementFlags {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for BluetoothLEAdvertisementFlags {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for BluetoothLEAdvertisementFlags {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementFlags;u4)");
}
impl ::windows::runtime::DefaultType for BluetoothLEAdvertisementFlags {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for BluetoothLEAdvertisementFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for BluetoothLEAdvertisementFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for BluetoothLEAdvertisementFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for BluetoothLEAdvertisementFlags {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for BluetoothLEAdvertisementFlags {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BluetoothLEAdvertisementPublisher(pub ::windows::runtime::IInspectable);
impl BluetoothLEAdvertisementPublisher {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<BluetoothLEAdvertisementPublisher, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<BluetoothLEAdvertisementPublisherStatus> {
        let this = self;
        unsafe {
            let mut result__: BluetoothLEAdvertisementPublisherStatus = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothLEAdvertisementPublisherStatus>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
    pub fn Advertisement(&self) -> ::windows::runtime::Result<BluetoothLEAdvertisement> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothLEAdvertisement>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
    pub fn Start(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
    pub fn Stop(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`, `Foundation`*"]
    pub fn StatusChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<BluetoothLEAdvertisementPublisher, BluetoothLEAdvertisementPublisherStatusChangedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`, `Foundation`*"]
    pub fn RemoveStatusChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, BluetoothLEAdvertisement>>(advertisement: Param0) -> ::windows::runtime::Result<BluetoothLEAdvertisementPublisher> {
        Self::IBluetoothLEAdvertisementPublisherFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), advertisement.into_param().abi(), &mut result__).from_abi::<BluetoothLEAdvertisementPublisher>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`, `Foundation`*"]
    pub fn PreferredTransmitPowerLevelInDBm(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<i16>> {
        let this = &::windows::runtime::Interface::cast::<IBluetoothLEAdvertisementPublisher2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<i16>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`, `Foundation`*"]
    pub fn SetPreferredTransmitPowerLevelInDBm<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IReference<i16>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBluetoothLEAdvertisementPublisher2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
    pub fn UseExtendedAdvertisement(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IBluetoothLEAdvertisementPublisher2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
    pub fn SetUseExtendedAdvertisement(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBluetoothLEAdvertisementPublisher2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
    pub fn IsAnonymous(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IBluetoothLEAdvertisementPublisher2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
    pub fn SetIsAnonymous(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBluetoothLEAdvertisementPublisher2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
    pub fn IncludeTransmitPowerLevel(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IBluetoothLEAdvertisementPublisher2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
    pub fn SetIncludeTransmitPowerLevel(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBluetoothLEAdvertisementPublisher2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IBluetoothLEAdvertisementPublisherFactory<R, F: FnOnce(&IBluetoothLEAdvertisementPublisherFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<BluetoothLEAdvertisementPublisher, IBluetoothLEAdvertisementPublisherFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BluetoothLEAdvertisementPublisher {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementPublisher;{cde820f9-d9fa-43d6-a264-ddd8b7da8b78})");
}
unsafe impl ::windows::runtime::Interface for BluetoothLEAdvertisementPublisher {
    type Vtable = IBluetoothLEAdvertisementPublisher_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xcde820f9_d9fa_43d6_a264_ddd8b7da8b78);
}
impl ::windows::runtime::RuntimeName for BluetoothLEAdvertisementPublisher {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementPublisher";
}
impl ::core::convert::From<BluetoothLEAdvertisementPublisher> for ::windows::runtime::IUnknown {
    fn from(value: BluetoothLEAdvertisementPublisher) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BluetoothLEAdvertisementPublisher> for ::windows::runtime::IUnknown {
    fn from(value: &BluetoothLEAdvertisementPublisher) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for BluetoothLEAdvertisementPublisher {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a BluetoothLEAdvertisementPublisher {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BluetoothLEAdvertisementPublisher> for ::windows::runtime::IInspectable {
    fn from(value: BluetoothLEAdvertisementPublisher) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BluetoothLEAdvertisementPublisher> for ::windows::runtime::IInspectable {
    fn from(value: &BluetoothLEAdvertisementPublisher) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for BluetoothLEAdvertisementPublisher {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a BluetoothLEAdvertisementPublisher {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for BluetoothLEAdvertisementPublisher {}
unsafe impl ::core::marker::Sync for BluetoothLEAdvertisementPublisher {}
#[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct BluetoothLEAdvertisementPublisherStatus(pub i32);
impl BluetoothLEAdvertisementPublisherStatus {
    pub const Created: BluetoothLEAdvertisementPublisherStatus = BluetoothLEAdvertisementPublisherStatus(0i32);
    pub const Waiting: BluetoothLEAdvertisementPublisherStatus = BluetoothLEAdvertisementPublisherStatus(1i32);
    pub const Started: BluetoothLEAdvertisementPublisherStatus = BluetoothLEAdvertisementPublisherStatus(2i32);
    pub const Stopping: BluetoothLEAdvertisementPublisherStatus = BluetoothLEAdvertisementPublisherStatus(3i32);
    pub const Stopped: BluetoothLEAdvertisementPublisherStatus = BluetoothLEAdvertisementPublisherStatus(4i32);
    pub const Aborted: BluetoothLEAdvertisementPublisherStatus = BluetoothLEAdvertisementPublisherStatus(5i32);
}
impl ::core::convert::From<i32> for BluetoothLEAdvertisementPublisherStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for BluetoothLEAdvertisementPublisherStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for BluetoothLEAdvertisementPublisherStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementPublisherStatus;i4)");
}
impl ::windows::runtime::DefaultType for BluetoothLEAdvertisementPublisherStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BluetoothLEAdvertisementPublisherStatusChangedEventArgs(pub ::windows::runtime::IInspectable);
impl BluetoothLEAdvertisementPublisherStatusChangedEventArgs {
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<BluetoothLEAdvertisementPublisherStatus> {
        let this = self;
        unsafe {
            let mut result__: BluetoothLEAdvertisementPublisherStatus = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothLEAdvertisementPublisherStatus>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
    pub fn Error(&self) -> ::windows::runtime::Result<super::BluetoothError> {
        let this = self;
        unsafe {
            let mut result__: super::BluetoothError = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::BluetoothError>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`, `Foundation`*"]
    pub fn SelectedTransmitPowerLevelInDBm(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<i16>> {
        let this = &::windows::runtime::Interface::cast::<IBluetoothLEAdvertisementPublisherStatusChangedEventArgs2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<i16>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BluetoothLEAdvertisementPublisherStatusChangedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementPublisherStatusChangedEventArgs;{09c2bd9f-2dff-4b23-86ee-0d14fb94aeae})");
}
unsafe impl ::windows::runtime::Interface for BluetoothLEAdvertisementPublisherStatusChangedEventArgs {
    type Vtable = IBluetoothLEAdvertisementPublisherStatusChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x09c2bd9f_2dff_4b23_86ee_0d14fb94aeae);
}
impl ::windows::runtime::RuntimeName for BluetoothLEAdvertisementPublisherStatusChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementPublisherStatusChangedEventArgs";
}
impl ::core::convert::From<BluetoothLEAdvertisementPublisherStatusChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: BluetoothLEAdvertisementPublisherStatusChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BluetoothLEAdvertisementPublisherStatusChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &BluetoothLEAdvertisementPublisherStatusChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for BluetoothLEAdvertisementPublisherStatusChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a BluetoothLEAdvertisementPublisherStatusChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BluetoothLEAdvertisementPublisherStatusChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: BluetoothLEAdvertisementPublisherStatusChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BluetoothLEAdvertisementPublisherStatusChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &BluetoothLEAdvertisementPublisherStatusChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for BluetoothLEAdvertisementPublisherStatusChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a BluetoothLEAdvertisementPublisherStatusChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for BluetoothLEAdvertisementPublisherStatusChangedEventArgs {}
unsafe impl ::core::marker::Sync for BluetoothLEAdvertisementPublisherStatusChangedEventArgs {}
#[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BluetoothLEAdvertisementReceivedEventArgs(pub ::windows::runtime::IInspectable);
impl BluetoothLEAdvertisementReceivedEventArgs {
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
    pub fn RawSignalStrengthInDBm(&self) -> ::windows::runtime::Result<i16> {
        let this = self;
        unsafe {
            let mut result__: i16 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i16>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
    pub fn BluetoothAddress(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
    pub fn AdvertisementType(&self) -> ::windows::runtime::Result<BluetoothLEAdvertisementType> {
        let this = self;
        unsafe {
            let mut result__: BluetoothLEAdvertisementType = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothLEAdvertisementType>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`, `Foundation`*"]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
    pub fn Advertisement(&self) -> ::windows::runtime::Result<BluetoothLEAdvertisement> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothLEAdvertisement>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
    pub fn BluetoothAddressType(&self) -> ::windows::runtime::Result<super::BluetoothAddressType> {
        let this = &::windows::runtime::Interface::cast::<IBluetoothLEAdvertisementReceivedEventArgs2>(self)?;
        unsafe {
            let mut result__: super::BluetoothAddressType = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::BluetoothAddressType>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`, `Foundation`*"]
    pub fn TransmitPowerLevelInDBm(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<i16>> {
        let this = &::windows::runtime::Interface::cast::<IBluetoothLEAdvertisementReceivedEventArgs2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<i16>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
    pub fn IsAnonymous(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IBluetoothLEAdvertisementReceivedEventArgs2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
    pub fn IsConnectable(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IBluetoothLEAdvertisementReceivedEventArgs2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
    pub fn IsScannable(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IBluetoothLEAdvertisementReceivedEventArgs2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
    pub fn IsDirected(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IBluetoothLEAdvertisementReceivedEventArgs2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
    pub fn IsScanResponse(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IBluetoothLEAdvertisementReceivedEventArgs2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BluetoothLEAdvertisementReceivedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementReceivedEventArgs;{27987ddf-e596-41be-8d43-9e6731d4a913})");
}
unsafe impl ::windows::runtime::Interface for BluetoothLEAdvertisementReceivedEventArgs {
    type Vtable = IBluetoothLEAdvertisementReceivedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x27987ddf_e596_41be_8d43_9e6731d4a913);
}
impl ::windows::runtime::RuntimeName for BluetoothLEAdvertisementReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementReceivedEventArgs";
}
impl ::core::convert::From<BluetoothLEAdvertisementReceivedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: BluetoothLEAdvertisementReceivedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BluetoothLEAdvertisementReceivedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &BluetoothLEAdvertisementReceivedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for BluetoothLEAdvertisementReceivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a BluetoothLEAdvertisementReceivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BluetoothLEAdvertisementReceivedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: BluetoothLEAdvertisementReceivedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BluetoothLEAdvertisementReceivedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &BluetoothLEAdvertisementReceivedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for BluetoothLEAdvertisementReceivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a BluetoothLEAdvertisementReceivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for BluetoothLEAdvertisementReceivedEventArgs {}
unsafe impl ::core::marker::Sync for BluetoothLEAdvertisementReceivedEventArgs {}
#[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct BluetoothLEAdvertisementType(pub i32);
impl BluetoothLEAdvertisementType {
    pub const ConnectableUndirected: BluetoothLEAdvertisementType = BluetoothLEAdvertisementType(0i32);
    pub const ConnectableDirected: BluetoothLEAdvertisementType = BluetoothLEAdvertisementType(1i32);
    pub const ScannableUndirected: BluetoothLEAdvertisementType = BluetoothLEAdvertisementType(2i32);
    pub const NonConnectableUndirected: BluetoothLEAdvertisementType = BluetoothLEAdvertisementType(3i32);
    pub const ScanResponse: BluetoothLEAdvertisementType = BluetoothLEAdvertisementType(4i32);
    pub const Extended: BluetoothLEAdvertisementType = BluetoothLEAdvertisementType(5i32);
}
impl ::core::convert::From<i32> for BluetoothLEAdvertisementType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for BluetoothLEAdvertisementType {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for BluetoothLEAdvertisementType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementType;i4)");
}
impl ::windows::runtime::DefaultType for BluetoothLEAdvertisementType {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BluetoothLEAdvertisementWatcher(pub ::windows::runtime::IInspectable);
impl BluetoothLEAdvertisementWatcher {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<BluetoothLEAdvertisementWatcher, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`, `Foundation`*"]
    pub fn MinSamplingInterval(&self) -> ::windows::runtime::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`, `Foundation`*"]
    pub fn MaxSamplingInterval(&self) -> ::windows::runtime::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`, `Foundation`*"]
    pub fn MinOutOfRangeTimeout(&self) -> ::windows::runtime::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`, `Foundation`*"]
    pub fn MaxOutOfRangeTimeout(&self) -> ::windows::runtime::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<BluetoothLEAdvertisementWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__: BluetoothLEAdvertisementWatcherStatus = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothLEAdvertisementWatcherStatus>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
    pub fn ScanningMode(&self) -> ::windows::runtime::Result<BluetoothLEScanningMode> {
        let this = self;
        unsafe {
            let mut result__: BluetoothLEScanningMode = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothLEScanningMode>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
    pub fn SetScanningMode(&self, value: BluetoothLEScanningMode) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
    pub fn SignalStrengthFilter(&self) -> ::windows::runtime::Result<super::BluetoothSignalStrengthFilter> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::BluetoothSignalStrengthFilter>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
    pub fn SetSignalStrengthFilter<'a, Param0: ::windows::runtime::IntoParam<'a, super::BluetoothSignalStrengthFilter>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
    pub fn AdvertisementFilter(&self) -> ::windows::runtime::Result<BluetoothLEAdvertisementFilter> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothLEAdvertisementFilter>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
    pub fn SetAdvertisementFilter<'a, Param0: ::windows::runtime::IntoParam<'a, BluetoothLEAdvertisementFilter>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
    pub fn Start(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
    pub fn Stop(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).18)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`, `Foundation`*"]
    pub fn Received<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<BluetoothLEAdvertisementWatcher, BluetoothLEAdvertisementReceivedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`, `Foundation`*"]
    pub fn RemoveReceived<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).20)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`, `Foundation`*"]
    pub fn Stopped<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<BluetoothLEAdvertisementWatcher, BluetoothLEAdvertisementWatcherStoppedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`, `Foundation`*"]
    pub fn RemoveStopped<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).22)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, BluetoothLEAdvertisementFilter>>(advertisementfilter: Param0) -> ::windows::runtime::Result<BluetoothLEAdvertisementWatcher> {
        Self::IBluetoothLEAdvertisementWatcherFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), advertisementfilter.into_param().abi(), &mut result__).from_abi::<BluetoothLEAdvertisementWatcher>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
    pub fn AllowExtendedAdvertisements(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IBluetoothLEAdvertisementWatcher2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
    pub fn SetAllowExtendedAdvertisements(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBluetoothLEAdvertisementWatcher2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IBluetoothLEAdvertisementWatcherFactory<R, F: FnOnce(&IBluetoothLEAdvertisementWatcherFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<BluetoothLEAdvertisementWatcher, IBluetoothLEAdvertisementWatcherFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BluetoothLEAdvertisementWatcher {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementWatcher;{a6ac336f-f3d3-4297-8d6c-c81ea6623f40})");
}
unsafe impl ::windows::runtime::Interface for BluetoothLEAdvertisementWatcher {
    type Vtable = IBluetoothLEAdvertisementWatcher_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa6ac336f_f3d3_4297_8d6c_c81ea6623f40);
}
impl ::windows::runtime::RuntimeName for BluetoothLEAdvertisementWatcher {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementWatcher";
}
impl ::core::convert::From<BluetoothLEAdvertisementWatcher> for ::windows::runtime::IUnknown {
    fn from(value: BluetoothLEAdvertisementWatcher) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BluetoothLEAdvertisementWatcher> for ::windows::runtime::IUnknown {
    fn from(value: &BluetoothLEAdvertisementWatcher) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for BluetoothLEAdvertisementWatcher {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a BluetoothLEAdvertisementWatcher {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BluetoothLEAdvertisementWatcher> for ::windows::runtime::IInspectable {
    fn from(value: BluetoothLEAdvertisementWatcher) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BluetoothLEAdvertisementWatcher> for ::windows::runtime::IInspectable {
    fn from(value: &BluetoothLEAdvertisementWatcher) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for BluetoothLEAdvertisementWatcher {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a BluetoothLEAdvertisementWatcher {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for BluetoothLEAdvertisementWatcher {}
unsafe impl ::core::marker::Sync for BluetoothLEAdvertisementWatcher {}
#[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct BluetoothLEAdvertisementWatcherStatus(pub i32);
impl BluetoothLEAdvertisementWatcherStatus {
    pub const Created: BluetoothLEAdvertisementWatcherStatus = BluetoothLEAdvertisementWatcherStatus(0i32);
    pub const Started: BluetoothLEAdvertisementWatcherStatus = BluetoothLEAdvertisementWatcherStatus(1i32);
    pub const Stopping: BluetoothLEAdvertisementWatcherStatus = BluetoothLEAdvertisementWatcherStatus(2i32);
    pub const Stopped: BluetoothLEAdvertisementWatcherStatus = BluetoothLEAdvertisementWatcherStatus(3i32);
    pub const Aborted: BluetoothLEAdvertisementWatcherStatus = BluetoothLEAdvertisementWatcherStatus(4i32);
}
impl ::core::convert::From<i32> for BluetoothLEAdvertisementWatcherStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for BluetoothLEAdvertisementWatcherStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for BluetoothLEAdvertisementWatcherStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementWatcherStatus;i4)");
}
impl ::windows::runtime::DefaultType for BluetoothLEAdvertisementWatcherStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BluetoothLEAdvertisementWatcherStoppedEventArgs(pub ::windows::runtime::IInspectable);
impl BluetoothLEAdvertisementWatcherStoppedEventArgs {
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
    pub fn Error(&self) -> ::windows::runtime::Result<super::BluetoothError> {
        let this = self;
        unsafe {
            let mut result__: super::BluetoothError = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::BluetoothError>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BluetoothLEAdvertisementWatcherStoppedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementWatcherStoppedEventArgs;{dd40f84d-e7b9-43e3-9c04-0685d085fd8c})");
}
unsafe impl ::windows::runtime::Interface for BluetoothLEAdvertisementWatcherStoppedEventArgs {
    type Vtable = IBluetoothLEAdvertisementWatcherStoppedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xdd40f84d_e7b9_43e3_9c04_0685d085fd8c);
}
impl ::windows::runtime::RuntimeName for BluetoothLEAdvertisementWatcherStoppedEventArgs {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementWatcherStoppedEventArgs";
}
impl ::core::convert::From<BluetoothLEAdvertisementWatcherStoppedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: BluetoothLEAdvertisementWatcherStoppedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BluetoothLEAdvertisementWatcherStoppedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &BluetoothLEAdvertisementWatcherStoppedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for BluetoothLEAdvertisementWatcherStoppedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a BluetoothLEAdvertisementWatcherStoppedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BluetoothLEAdvertisementWatcherStoppedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: BluetoothLEAdvertisementWatcherStoppedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BluetoothLEAdvertisementWatcherStoppedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &BluetoothLEAdvertisementWatcherStoppedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for BluetoothLEAdvertisementWatcherStoppedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a BluetoothLEAdvertisementWatcherStoppedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for BluetoothLEAdvertisementWatcherStoppedEventArgs {}
unsafe impl ::core::marker::Sync for BluetoothLEAdvertisementWatcherStoppedEventArgs {}
#[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BluetoothLEManufacturerData(pub ::windows::runtime::IInspectable);
impl BluetoothLEManufacturerData {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<BluetoothLEManufacturerData, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
    pub fn CompanyId(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
    pub fn SetCompanyId(&self, value: u16) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`, `Storage_Streams`*"]
    pub fn Data(&self) -> ::windows::runtime::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`, `Storage_Streams`*"]
    pub fn SetData<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Bluetooth_Advertisement`, `Storage_Streams`*"]
    pub fn Create<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(companyid: u16, data: Param1) -> ::windows::runtime::Result<BluetoothLEManufacturerData> {
        Self::IBluetoothLEManufacturerDataFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), companyid, data.into_param().abi(), &mut result__).from_abi::<BluetoothLEManufacturerData>(result__)
        })
    }
    pub fn IBluetoothLEManufacturerDataFactory<R, F: FnOnce(&IBluetoothLEManufacturerDataFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<BluetoothLEManufacturerData, IBluetoothLEManufacturerDataFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BluetoothLEManufacturerData {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Advertisement.BluetoothLEManufacturerData;{912dba18-6963-4533-b061-4694dafb34e5})");
}
unsafe impl ::windows::runtime::Interface for BluetoothLEManufacturerData {
    type Vtable = IBluetoothLEManufacturerData_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x912dba18_6963_4533_b061_4694dafb34e5);
}
impl ::windows::runtime::RuntimeName for BluetoothLEManufacturerData {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.BluetoothLEManufacturerData";
}
impl ::core::convert::From<BluetoothLEManufacturerData> for ::windows::runtime::IUnknown {
    fn from(value: BluetoothLEManufacturerData) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BluetoothLEManufacturerData> for ::windows::runtime::IUnknown {
    fn from(value: &BluetoothLEManufacturerData) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for BluetoothLEManufacturerData {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a BluetoothLEManufacturerData {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BluetoothLEManufacturerData> for ::windows::runtime::IInspectable {
    fn from(value: BluetoothLEManufacturerData) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BluetoothLEManufacturerData> for ::windows::runtime::IInspectable {
    fn from(value: &BluetoothLEManufacturerData) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for BluetoothLEManufacturerData {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a BluetoothLEManufacturerData {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for BluetoothLEManufacturerData {}
unsafe impl ::core::marker::Sync for BluetoothLEManufacturerData {}
#[doc = "*Required features: `Devices_Bluetooth_Advertisement`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct BluetoothLEScanningMode(pub i32);
impl BluetoothLEScanningMode {
    pub const Passive: BluetoothLEScanningMode = BluetoothLEScanningMode(0i32);
    pub const Active: BluetoothLEScanningMode = BluetoothLEScanningMode(1i32);
    pub const None: BluetoothLEScanningMode = BluetoothLEScanningMode(2i32);
}
impl ::core::convert::From<i32> for BluetoothLEScanningMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for BluetoothLEScanningMode {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for BluetoothLEScanningMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.Advertisement.BluetoothLEScanningMode;i4)");
}
impl ::windows::runtime::DefaultType for BluetoothLEScanningMode {
    type DefaultType = Self;
}
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisement(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBluetoothLEAdvertisement {
    type Vtable = IBluetoothLEAdvertisement_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x066fb2b7_33d1_4e7d_8367_cf81d0f79653);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisement_abi(
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, companyid: u16, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: u8, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementBytePattern(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBluetoothLEAdvertisementBytePattern {
    type Vtable = IBluetoothLEAdvertisementBytePattern_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xfbfad7f2_b9c5_4a08_bc51_502f8ef68a79);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementBytePattern_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementBytePatternFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBluetoothLEAdvertisementBytePatternFactory {
    type Vtable = IBluetoothLEAdvertisementBytePatternFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc2e24d73_fd5c_4ec3_be2a_9ca6fa11b7bd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementBytePatternFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, datatype: u8, offset: i16, data: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementDataSection(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBluetoothLEAdvertisementDataSection {
    type Vtable = IBluetoothLEAdvertisementDataSection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd7213314_3a43_40f9_b6f0_92bfefc34ae3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementDataSection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u8) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementDataSectionFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBluetoothLEAdvertisementDataSectionFactory {
    type Vtable = IBluetoothLEAdvertisementDataSectionFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xe7a40942_a845_4045_bf7e_3e9971db8a6b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementDataSectionFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, datatype: u8, data: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementDataTypesStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBluetoothLEAdvertisementDataTypesStatics {
    type Vtable = IBluetoothLEAdvertisementDataTypesStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x3bb6472f_0606_434b_a76e_74159f0684d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementDataTypesStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementFilter(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBluetoothLEAdvertisementFilter {
    type Vtable = IBluetoothLEAdvertisementFilter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x131eb0d3_d04e_47b1_837e_49405bf6f80f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementFilter_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementPublisher(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBluetoothLEAdvertisementPublisher {
    type Vtable = IBluetoothLEAdvertisementPublisher_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xcde820f9_d9fa_43d6_a264_ddd8b7da8b78);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementPublisher_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut BluetoothLEAdvertisementPublisherStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementPublisher2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBluetoothLEAdvertisementPublisher2 {
    type Vtable = IBluetoothLEAdvertisementPublisher2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xfbdb545e_56f1_510f_a434_217fbd9e7bd2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementPublisher2_abi(
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementPublisherFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBluetoothLEAdvertisementPublisherFactory {
    type Vtable = IBluetoothLEAdvertisementPublisherFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x5c5f065e_b863_4981_a1af_1c544d8b0c0d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementPublisherFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, advertisement: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementPublisherStatusChangedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBluetoothLEAdvertisementPublisherStatusChangedEventArgs {
    type Vtable = IBluetoothLEAdvertisementPublisherStatusChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x09c2bd9f_2dff_4b23_86ee_0d14fb94aeae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementPublisherStatusChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut BluetoothLEAdvertisementPublisherStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::BluetoothError) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementPublisherStatusChangedEventArgs2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBluetoothLEAdvertisementPublisherStatusChangedEventArgs2 {
    type Vtable = IBluetoothLEAdvertisementPublisherStatusChangedEventArgs2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8f62790e_dc88_5c8b_b34e_10b321850f88);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementPublisherStatusChangedEventArgs2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementReceivedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBluetoothLEAdvertisementReceivedEventArgs {
    type Vtable = IBluetoothLEAdvertisementReceivedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x27987ddf_e596_41be_8d43_9e6731d4a913);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementReceivedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut BluetoothLEAdvertisementType) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementReceivedEventArgs2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBluetoothLEAdvertisementReceivedEventArgs2 {
    type Vtable = IBluetoothLEAdvertisementReceivedEventArgs2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x12d9c87b_0399_5f0e_a348_53b02b6b162e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementReceivedEventArgs2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::BluetoothAddressType) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementWatcher(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBluetoothLEAdvertisementWatcher {
    type Vtable = IBluetoothLEAdvertisementWatcher_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa6ac336f_f3d3_4297_8d6c_c81ea6623f40);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementWatcher_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut BluetoothLEAdvertisementWatcherStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut BluetoothLEScanningMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: BluetoothLEScanningMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementWatcher2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBluetoothLEAdvertisementWatcher2 {
    type Vtable = IBluetoothLEAdvertisementWatcher2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x01bf26bc_b164_5805_90a3_e8a7997ff225);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementWatcher2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementWatcherFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBluetoothLEAdvertisementWatcherFactory {
    type Vtable = IBluetoothLEAdvertisementWatcherFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x9aaf2d56_39ac_453e_b32a_85c657e017f1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementWatcherFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, advertisementfilter: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementWatcherStoppedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBluetoothLEAdvertisementWatcherStoppedEventArgs {
    type Vtable = IBluetoothLEAdvertisementWatcherStoppedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xdd40f84d_e7b9_43e3_9c04_0685d085fd8c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementWatcherStoppedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::BluetoothError) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEManufacturerData(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBluetoothLEManufacturerData {
    type Vtable = IBluetoothLEManufacturerData_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x912dba18_6963_4533_b061_4694dafb34e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEManufacturerData_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEManufacturerDataFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBluetoothLEManufacturerDataFactory {
    type Vtable = IBluetoothLEManufacturerDataFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc09b39f8_319a_441e_8de5_66a81e877a6c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEManufacturerDataFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, companyid: u16, data: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
