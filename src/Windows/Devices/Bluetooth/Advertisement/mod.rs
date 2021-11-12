#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BluetoothLEAdvertisement(pub ::windows::core::IInspectable);
impl BluetoothLEAdvertisement {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BluetoothLEAdvertisement, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    pub fn Flags(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<BluetoothLEAdvertisementFlags>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<BluetoothLEAdvertisementFlags>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetFlags<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::IReference<BluetoothLEAdvertisementFlags>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn LocalName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetLocalName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ServiceUuids(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::GUID>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<::windows::core::GUID>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ManufacturerData(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<BluetoothLEManufacturerData>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<BluetoothLEManufacturerData>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn DataSections(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<BluetoothLEAdvertisementDataSection>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<BluetoothLEAdvertisementDataSection>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetManufacturerDataByCompanyId(&self, companyid: u16) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<BluetoothLEManufacturerData>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), companyid, &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<BluetoothLEManufacturerData>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSectionsByType(&self, r#type: u8) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<BluetoothLEAdvertisementDataSection>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), r#type, &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<BluetoothLEAdvertisementDataSection>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for BluetoothLEAdvertisement {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisement;{066fb2b7-33d1-4e7d-8367-cf81d0f79653})");
}
unsafe impl ::windows::core::Interface for BluetoothLEAdvertisement {
    type Vtable = IBluetoothLEAdvertisement_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x066fb2b7_33d1_4e7d_8367_cf81d0f79653);
}
impl ::windows::core::RuntimeName for BluetoothLEAdvertisement {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisement";
}
impl ::core::convert::From<BluetoothLEAdvertisement> for ::windows::core::IUnknown {
    fn from(value: BluetoothLEAdvertisement) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BluetoothLEAdvertisement> for ::windows::core::IUnknown {
    fn from(value: &BluetoothLEAdvertisement) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BluetoothLEAdvertisement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BluetoothLEAdvertisement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BluetoothLEAdvertisement> for ::windows::core::IInspectable {
    fn from(value: BluetoothLEAdvertisement) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BluetoothLEAdvertisement> for ::windows::core::IInspectable {
    fn from(value: &BluetoothLEAdvertisement) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BluetoothLEAdvertisement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BluetoothLEAdvertisement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for BluetoothLEAdvertisement {}
unsafe impl ::core::marker::Sync for BluetoothLEAdvertisement {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BluetoothLEAdvertisementBytePattern(pub ::windows::core::IInspectable);
impl BluetoothLEAdvertisementBytePattern {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BluetoothLEAdvertisementBytePattern, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn DataType(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    pub fn SetDataType(&self, value: u8) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Offset(&self) -> ::windows::core::Result<i16> {
        let this = self;
        unsafe {
            let mut result__: i16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i16>(result__)
        }
    }
    pub fn SetOffset(&self, value: i16) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Data(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetData<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Create<'a, Param2: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(datatype: u8, offset: i16, data: Param2) -> ::windows::core::Result<BluetoothLEAdvertisementBytePattern> {
        Self::IBluetoothLEAdvertisementBytePatternFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), datatype, offset, data.into_param().abi(), &mut result__).from_abi::<BluetoothLEAdvertisementBytePattern>(result__)
        })
    }
    pub fn IBluetoothLEAdvertisementBytePatternFactory<R, F: FnOnce(&IBluetoothLEAdvertisementBytePatternFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BluetoothLEAdvertisementBytePattern, IBluetoothLEAdvertisementBytePatternFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for BluetoothLEAdvertisementBytePattern {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementBytePattern;{fbfad7f2-b9c5-4a08-bc51-502f8ef68a79})");
}
unsafe impl ::windows::core::Interface for BluetoothLEAdvertisementBytePattern {
    type Vtable = IBluetoothLEAdvertisementBytePattern_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfbfad7f2_b9c5_4a08_bc51_502f8ef68a79);
}
impl ::windows::core::RuntimeName for BluetoothLEAdvertisementBytePattern {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementBytePattern";
}
impl ::core::convert::From<BluetoothLEAdvertisementBytePattern> for ::windows::core::IUnknown {
    fn from(value: BluetoothLEAdvertisementBytePattern) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BluetoothLEAdvertisementBytePattern> for ::windows::core::IUnknown {
    fn from(value: &BluetoothLEAdvertisementBytePattern) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BluetoothLEAdvertisementBytePattern {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BluetoothLEAdvertisementBytePattern {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BluetoothLEAdvertisementBytePattern> for ::windows::core::IInspectable {
    fn from(value: BluetoothLEAdvertisementBytePattern) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BluetoothLEAdvertisementBytePattern> for ::windows::core::IInspectable {
    fn from(value: &BluetoothLEAdvertisementBytePattern) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BluetoothLEAdvertisementBytePattern {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BluetoothLEAdvertisementBytePattern {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for BluetoothLEAdvertisementBytePattern {}
unsafe impl ::core::marker::Sync for BluetoothLEAdvertisementBytePattern {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BluetoothLEAdvertisementDataSection(pub ::windows::core::IInspectable);
impl BluetoothLEAdvertisementDataSection {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BluetoothLEAdvertisementDataSection, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn DataType(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    pub fn SetDataType(&self, value: u8) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Data(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetData<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Create<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(datatype: u8, data: Param1) -> ::windows::core::Result<BluetoothLEAdvertisementDataSection> {
        Self::IBluetoothLEAdvertisementDataSectionFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), datatype, data.into_param().abi(), &mut result__).from_abi::<BluetoothLEAdvertisementDataSection>(result__)
        })
    }
    pub fn IBluetoothLEAdvertisementDataSectionFactory<R, F: FnOnce(&IBluetoothLEAdvertisementDataSectionFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BluetoothLEAdvertisementDataSection, IBluetoothLEAdvertisementDataSectionFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for BluetoothLEAdvertisementDataSection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementDataSection;{d7213314-3a43-40f9-b6f0-92bfefc34ae3})");
}
unsafe impl ::windows::core::Interface for BluetoothLEAdvertisementDataSection {
    type Vtable = IBluetoothLEAdvertisementDataSection_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7213314_3a43_40f9_b6f0_92bfefc34ae3);
}
impl ::windows::core::RuntimeName for BluetoothLEAdvertisementDataSection {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementDataSection";
}
impl ::core::convert::From<BluetoothLEAdvertisementDataSection> for ::windows::core::IUnknown {
    fn from(value: BluetoothLEAdvertisementDataSection) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BluetoothLEAdvertisementDataSection> for ::windows::core::IUnknown {
    fn from(value: &BluetoothLEAdvertisementDataSection) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BluetoothLEAdvertisementDataSection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BluetoothLEAdvertisementDataSection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BluetoothLEAdvertisementDataSection> for ::windows::core::IInspectable {
    fn from(value: BluetoothLEAdvertisementDataSection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BluetoothLEAdvertisementDataSection> for ::windows::core::IInspectable {
    fn from(value: &BluetoothLEAdvertisementDataSection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BluetoothLEAdvertisementDataSection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BluetoothLEAdvertisementDataSection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for BluetoothLEAdvertisementDataSection {}
unsafe impl ::core::marker::Sync for BluetoothLEAdvertisementDataSection {}
pub struct BluetoothLEAdvertisementDataTypes {}
impl BluetoothLEAdvertisementDataTypes {
    pub fn Flags() -> ::windows::core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    pub fn IncompleteService16BitUuids() -> ::windows::core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    pub fn CompleteService16BitUuids() -> ::windows::core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    pub fn IncompleteService32BitUuids() -> ::windows::core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    pub fn CompleteService32BitUuids() -> ::windows::core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    pub fn IncompleteService128BitUuids() -> ::windows::core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    pub fn CompleteService128BitUuids() -> ::windows::core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    pub fn ShortenedLocalName() -> ::windows::core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    pub fn CompleteLocalName() -> ::windows::core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    pub fn TxPowerLevel() -> ::windows::core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    pub fn SlaveConnectionIntervalRange() -> ::windows::core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    pub fn ServiceSolicitation16BitUuids() -> ::windows::core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    pub fn ServiceSolicitation32BitUuids() -> ::windows::core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    pub fn ServiceSolicitation128BitUuids() -> ::windows::core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    pub fn ServiceData16BitUuids() -> ::windows::core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    pub fn ServiceData32BitUuids() -> ::windows::core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    pub fn ServiceData128BitUuids() -> ::windows::core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    pub fn PublicTargetAddress() -> ::windows::core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    pub fn RandomTargetAddress() -> ::windows::core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    pub fn Appearance() -> ::windows::core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    pub fn AdvertisingInterval() -> ::windows::core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    pub fn ManufacturerSpecificData() -> ::windows::core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        })
    }
    pub fn IBluetoothLEAdvertisementDataTypesStatics<R, F: FnOnce(&IBluetoothLEAdvertisementDataTypesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BluetoothLEAdvertisementDataTypes, IBluetoothLEAdvertisementDataTypesStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for BluetoothLEAdvertisementDataTypes {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementDataTypes";
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BluetoothLEAdvertisementFilter(pub ::windows::core::IInspectable);
impl BluetoothLEAdvertisementFilter {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BluetoothLEAdvertisementFilter, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Advertisement(&self) -> ::windows::core::Result<BluetoothLEAdvertisement> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothLEAdvertisement>(result__)
        }
    }
    pub fn SetAdvertisement<'a, Param0: ::windows::core::IntoParam<'a, BluetoothLEAdvertisement>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn BytePatterns(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<BluetoothLEAdvertisementBytePattern>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<BluetoothLEAdvertisementBytePattern>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for BluetoothLEAdvertisementFilter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementFilter;{131eb0d3-d04e-47b1-837e-49405bf6f80f})");
}
unsafe impl ::windows::core::Interface for BluetoothLEAdvertisementFilter {
    type Vtable = IBluetoothLEAdvertisementFilter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x131eb0d3_d04e_47b1_837e_49405bf6f80f);
}
impl ::windows::core::RuntimeName for BluetoothLEAdvertisementFilter {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementFilter";
}
impl ::core::convert::From<BluetoothLEAdvertisementFilter> for ::windows::core::IUnknown {
    fn from(value: BluetoothLEAdvertisementFilter) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BluetoothLEAdvertisementFilter> for ::windows::core::IUnknown {
    fn from(value: &BluetoothLEAdvertisementFilter) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BluetoothLEAdvertisementFilter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BluetoothLEAdvertisementFilter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BluetoothLEAdvertisementFilter> for ::windows::core::IInspectable {
    fn from(value: BluetoothLEAdvertisementFilter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BluetoothLEAdvertisementFilter> for ::windows::core::IInspectable {
    fn from(value: &BluetoothLEAdvertisementFilter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BluetoothLEAdvertisementFilter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BluetoothLEAdvertisementFilter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for BluetoothLEAdvertisementFilter {}
unsafe impl ::core::marker::Sync for BluetoothLEAdvertisementFilter {}
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
unsafe impl ::windows::core::Abi for BluetoothLEAdvertisementFlags {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for BluetoothLEAdvertisementFlags {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementFlags;u4)");
}
impl ::windows::core::DefaultType for BluetoothLEAdvertisementFlags {
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
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BluetoothLEAdvertisementPublisher(pub ::windows::core::IInspectable);
impl BluetoothLEAdvertisementPublisher {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BluetoothLEAdvertisementPublisher, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Status(&self) -> ::windows::core::Result<BluetoothLEAdvertisementPublisherStatus> {
        let this = self;
        unsafe {
            let mut result__: BluetoothLEAdvertisementPublisherStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothLEAdvertisementPublisherStatus>(result__)
        }
    }
    pub fn Advertisement(&self) -> ::windows::core::Result<BluetoothLEAdvertisement> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothLEAdvertisement>(result__)
        }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn StatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<BluetoothLEAdvertisementPublisher, BluetoothLEAdvertisementPublisherStatusChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, BluetoothLEAdvertisement>>(advertisement: Param0) -> ::windows::core::Result<BluetoothLEAdvertisementPublisher> {
        Self::IBluetoothLEAdvertisementPublisherFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), advertisement.into_param().abi(), &mut result__).from_abi::<BluetoothLEAdvertisementPublisher>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn PreferredTransmitPowerLevelInDBm(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<i16>> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEAdvertisementPublisher2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<i16>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetPreferredTransmitPowerLevelInDBm<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::IReference<i16>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEAdvertisementPublisher2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn UseExtendedAdvertisement(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEAdvertisementPublisher2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetUseExtendedAdvertisement(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEAdvertisementPublisher2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IsAnonymous(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEAdvertisementPublisher2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsAnonymous(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEAdvertisementPublisher2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IncludeTransmitPowerLevel(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEAdvertisementPublisher2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIncludeTransmitPowerLevel(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEAdvertisementPublisher2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IBluetoothLEAdvertisementPublisherFactory<R, F: FnOnce(&IBluetoothLEAdvertisementPublisherFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BluetoothLEAdvertisementPublisher, IBluetoothLEAdvertisementPublisherFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for BluetoothLEAdvertisementPublisher {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementPublisher;{cde820f9-d9fa-43d6-a264-ddd8b7da8b78})");
}
unsafe impl ::windows::core::Interface for BluetoothLEAdvertisementPublisher {
    type Vtable = IBluetoothLEAdvertisementPublisher_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcde820f9_d9fa_43d6_a264_ddd8b7da8b78);
}
impl ::windows::core::RuntimeName for BluetoothLEAdvertisementPublisher {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementPublisher";
}
impl ::core::convert::From<BluetoothLEAdvertisementPublisher> for ::windows::core::IUnknown {
    fn from(value: BluetoothLEAdvertisementPublisher) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BluetoothLEAdvertisementPublisher> for ::windows::core::IUnknown {
    fn from(value: &BluetoothLEAdvertisementPublisher) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BluetoothLEAdvertisementPublisher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BluetoothLEAdvertisementPublisher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BluetoothLEAdvertisementPublisher> for ::windows::core::IInspectable {
    fn from(value: BluetoothLEAdvertisementPublisher) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BluetoothLEAdvertisementPublisher> for ::windows::core::IInspectable {
    fn from(value: &BluetoothLEAdvertisementPublisher) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BluetoothLEAdvertisementPublisher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BluetoothLEAdvertisementPublisher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for BluetoothLEAdvertisementPublisher {}
unsafe impl ::core::marker::Sync for BluetoothLEAdvertisementPublisher {}
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
unsafe impl ::windows::core::Abi for BluetoothLEAdvertisementPublisherStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for BluetoothLEAdvertisementPublisherStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementPublisherStatus;i4)");
}
impl ::windows::core::DefaultType for BluetoothLEAdvertisementPublisherStatus {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BluetoothLEAdvertisementPublisherStatusChangedEventArgs(pub ::windows::core::IInspectable);
impl BluetoothLEAdvertisementPublisherStatusChangedEventArgs {
    pub fn Status(&self) -> ::windows::core::Result<BluetoothLEAdvertisementPublisherStatus> {
        let this = self;
        unsafe {
            let mut result__: BluetoothLEAdvertisementPublisherStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothLEAdvertisementPublisherStatus>(result__)
        }
    }
    pub fn Error(&self) -> ::windows::core::Result<super::BluetoothError> {
        let this = self;
        unsafe {
            let mut result__: super::BluetoothError = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::BluetoothError>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SelectedTransmitPowerLevelInDBm(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<i16>> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEAdvertisementPublisherStatusChangedEventArgs2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<i16>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for BluetoothLEAdvertisementPublisherStatusChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementPublisherStatusChangedEventArgs;{09c2bd9f-2dff-4b23-86ee-0d14fb94aeae})");
}
unsafe impl ::windows::core::Interface for BluetoothLEAdvertisementPublisherStatusChangedEventArgs {
    type Vtable = IBluetoothLEAdvertisementPublisherStatusChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x09c2bd9f_2dff_4b23_86ee_0d14fb94aeae);
}
impl ::windows::core::RuntimeName for BluetoothLEAdvertisementPublisherStatusChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementPublisherStatusChangedEventArgs";
}
impl ::core::convert::From<BluetoothLEAdvertisementPublisherStatusChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: BluetoothLEAdvertisementPublisherStatusChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BluetoothLEAdvertisementPublisherStatusChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &BluetoothLEAdvertisementPublisherStatusChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BluetoothLEAdvertisementPublisherStatusChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BluetoothLEAdvertisementPublisherStatusChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BluetoothLEAdvertisementPublisherStatusChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: BluetoothLEAdvertisementPublisherStatusChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BluetoothLEAdvertisementPublisherStatusChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &BluetoothLEAdvertisementPublisherStatusChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BluetoothLEAdvertisementPublisherStatusChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BluetoothLEAdvertisementPublisherStatusChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for BluetoothLEAdvertisementPublisherStatusChangedEventArgs {}
unsafe impl ::core::marker::Sync for BluetoothLEAdvertisementPublisherStatusChangedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BluetoothLEAdvertisementReceivedEventArgs(pub ::windows::core::IInspectable);
impl BluetoothLEAdvertisementReceivedEventArgs {
    pub fn RawSignalStrengthInDBm(&self) -> ::windows::core::Result<i16> {
        let this = self;
        unsafe {
            let mut result__: i16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i16>(result__)
        }
    }
    pub fn BluetoothAddress(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn AdvertisementType(&self) -> ::windows::core::Result<BluetoothLEAdvertisementType> {
        let this = self;
        unsafe {
            let mut result__: BluetoothLEAdvertisementType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothLEAdvertisementType>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    pub fn Advertisement(&self) -> ::windows::core::Result<BluetoothLEAdvertisement> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothLEAdvertisement>(result__)
        }
    }
    pub fn BluetoothAddressType(&self) -> ::windows::core::Result<super::BluetoothAddressType> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEAdvertisementReceivedEventArgs2>(self)?;
        unsafe {
            let mut result__: super::BluetoothAddressType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::BluetoothAddressType>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn TransmitPowerLevelInDBm(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<i16>> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEAdvertisementReceivedEventArgs2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<i16>>(result__)
        }
    }
    pub fn IsAnonymous(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEAdvertisementReceivedEventArgs2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsConnectable(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEAdvertisementReceivedEventArgs2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsScannable(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEAdvertisementReceivedEventArgs2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsDirected(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEAdvertisementReceivedEventArgs2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsScanResponse(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEAdvertisementReceivedEventArgs2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for BluetoothLEAdvertisementReceivedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementReceivedEventArgs;{27987ddf-e596-41be-8d43-9e6731d4a913})");
}
unsafe impl ::windows::core::Interface for BluetoothLEAdvertisementReceivedEventArgs {
    type Vtable = IBluetoothLEAdvertisementReceivedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27987ddf_e596_41be_8d43_9e6731d4a913);
}
impl ::windows::core::RuntimeName for BluetoothLEAdvertisementReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementReceivedEventArgs";
}
impl ::core::convert::From<BluetoothLEAdvertisementReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: BluetoothLEAdvertisementReceivedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BluetoothLEAdvertisementReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &BluetoothLEAdvertisementReceivedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BluetoothLEAdvertisementReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BluetoothLEAdvertisementReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BluetoothLEAdvertisementReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: BluetoothLEAdvertisementReceivedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BluetoothLEAdvertisementReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &BluetoothLEAdvertisementReceivedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BluetoothLEAdvertisementReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BluetoothLEAdvertisementReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for BluetoothLEAdvertisementReceivedEventArgs {}
unsafe impl ::core::marker::Sync for BluetoothLEAdvertisementReceivedEventArgs {}
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
unsafe impl ::windows::core::Abi for BluetoothLEAdvertisementType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for BluetoothLEAdvertisementType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementType;i4)");
}
impl ::windows::core::DefaultType for BluetoothLEAdvertisementType {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BluetoothLEAdvertisementWatcher(pub ::windows::core::IInspectable);
impl BluetoothLEAdvertisementWatcher {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BluetoothLEAdvertisementWatcher, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    pub fn MinSamplingInterval(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn MaxSamplingInterval(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn MinOutOfRangeTimeout(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn MaxOutOfRangeTimeout(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    pub fn Status(&self) -> ::windows::core::Result<BluetoothLEAdvertisementWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__: BluetoothLEAdvertisementWatcherStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothLEAdvertisementWatcherStatus>(result__)
        }
    }
    pub fn ScanningMode(&self) -> ::windows::core::Result<BluetoothLEScanningMode> {
        let this = self;
        unsafe {
            let mut result__: BluetoothLEScanningMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothLEScanningMode>(result__)
        }
    }
    pub fn SetScanningMode(&self, value: BluetoothLEScanningMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn SignalStrengthFilter(&self) -> ::windows::core::Result<super::BluetoothSignalStrengthFilter> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::BluetoothSignalStrengthFilter>(result__)
        }
    }
    pub fn SetSignalStrengthFilter<'a, Param0: ::windows::core::IntoParam<'a, super::BluetoothSignalStrengthFilter>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn AdvertisementFilter(&self) -> ::windows::core::Result<BluetoothLEAdvertisementFilter> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BluetoothLEAdvertisementFilter>(result__)
        }
    }
    pub fn SetAdvertisementFilter<'a, Param0: ::windows::core::IntoParam<'a, BluetoothLEAdvertisementFilter>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Received<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<BluetoothLEAdvertisementWatcher, BluetoothLEAdvertisementReceivedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Stopped<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<BluetoothLEAdvertisementWatcher, BluetoothLEAdvertisementWatcherStoppedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveStopped<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, BluetoothLEAdvertisementFilter>>(advertisementfilter: Param0) -> ::windows::core::Result<BluetoothLEAdvertisementWatcher> {
        Self::IBluetoothLEAdvertisementWatcherFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), advertisementfilter.into_param().abi(), &mut result__).from_abi::<BluetoothLEAdvertisementWatcher>(result__)
        })
    }
    pub fn AllowExtendedAdvertisements(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEAdvertisementWatcher2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowExtendedAdvertisements(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEAdvertisementWatcher2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IBluetoothLEAdvertisementWatcherFactory<R, F: FnOnce(&IBluetoothLEAdvertisementWatcherFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BluetoothLEAdvertisementWatcher, IBluetoothLEAdvertisementWatcherFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for BluetoothLEAdvertisementWatcher {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementWatcher;{a6ac336f-f3d3-4297-8d6c-c81ea6623f40})");
}
unsafe impl ::windows::core::Interface for BluetoothLEAdvertisementWatcher {
    type Vtable = IBluetoothLEAdvertisementWatcher_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa6ac336f_f3d3_4297_8d6c_c81ea6623f40);
}
impl ::windows::core::RuntimeName for BluetoothLEAdvertisementWatcher {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementWatcher";
}
impl ::core::convert::From<BluetoothLEAdvertisementWatcher> for ::windows::core::IUnknown {
    fn from(value: BluetoothLEAdvertisementWatcher) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BluetoothLEAdvertisementWatcher> for ::windows::core::IUnknown {
    fn from(value: &BluetoothLEAdvertisementWatcher) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BluetoothLEAdvertisementWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BluetoothLEAdvertisementWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BluetoothLEAdvertisementWatcher> for ::windows::core::IInspectable {
    fn from(value: BluetoothLEAdvertisementWatcher) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BluetoothLEAdvertisementWatcher> for ::windows::core::IInspectable {
    fn from(value: &BluetoothLEAdvertisementWatcher) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BluetoothLEAdvertisementWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BluetoothLEAdvertisementWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for BluetoothLEAdvertisementWatcher {}
unsafe impl ::core::marker::Sync for BluetoothLEAdvertisementWatcher {}
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
unsafe impl ::windows::core::Abi for BluetoothLEAdvertisementWatcherStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for BluetoothLEAdvertisementWatcherStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementWatcherStatus;i4)");
}
impl ::windows::core::DefaultType for BluetoothLEAdvertisementWatcherStatus {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BluetoothLEAdvertisementWatcherStoppedEventArgs(pub ::windows::core::IInspectable);
impl BluetoothLEAdvertisementWatcherStoppedEventArgs {
    pub fn Error(&self) -> ::windows::core::Result<super::BluetoothError> {
        let this = self;
        unsafe {
            let mut result__: super::BluetoothError = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::BluetoothError>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for BluetoothLEAdvertisementWatcherStoppedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementWatcherStoppedEventArgs;{dd40f84d-e7b9-43e3-9c04-0685d085fd8c})");
}
unsafe impl ::windows::core::Interface for BluetoothLEAdvertisementWatcherStoppedEventArgs {
    type Vtable = IBluetoothLEAdvertisementWatcherStoppedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdd40f84d_e7b9_43e3_9c04_0685d085fd8c);
}
impl ::windows::core::RuntimeName for BluetoothLEAdvertisementWatcherStoppedEventArgs {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementWatcherStoppedEventArgs";
}
impl ::core::convert::From<BluetoothLEAdvertisementWatcherStoppedEventArgs> for ::windows::core::IUnknown {
    fn from(value: BluetoothLEAdvertisementWatcherStoppedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BluetoothLEAdvertisementWatcherStoppedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &BluetoothLEAdvertisementWatcherStoppedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BluetoothLEAdvertisementWatcherStoppedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BluetoothLEAdvertisementWatcherStoppedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BluetoothLEAdvertisementWatcherStoppedEventArgs> for ::windows::core::IInspectable {
    fn from(value: BluetoothLEAdvertisementWatcherStoppedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BluetoothLEAdvertisementWatcherStoppedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &BluetoothLEAdvertisementWatcherStoppedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BluetoothLEAdvertisementWatcherStoppedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BluetoothLEAdvertisementWatcherStoppedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for BluetoothLEAdvertisementWatcherStoppedEventArgs {}
unsafe impl ::core::marker::Sync for BluetoothLEAdvertisementWatcherStoppedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BluetoothLEManufacturerData(pub ::windows::core::IInspectable);
impl BluetoothLEManufacturerData {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BluetoothLEManufacturerData, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn CompanyId(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    pub fn SetCompanyId(&self, value: u16) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Data(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetData<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Create<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(companyid: u16, data: Param1) -> ::windows::core::Result<BluetoothLEManufacturerData> {
        Self::IBluetoothLEManufacturerDataFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), companyid, data.into_param().abi(), &mut result__).from_abi::<BluetoothLEManufacturerData>(result__)
        })
    }
    pub fn IBluetoothLEManufacturerDataFactory<R, F: FnOnce(&IBluetoothLEManufacturerDataFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BluetoothLEManufacturerData, IBluetoothLEManufacturerDataFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for BluetoothLEManufacturerData {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Advertisement.BluetoothLEManufacturerData;{912dba18-6963-4533-b061-4694dafb34e5})");
}
unsafe impl ::windows::core::Interface for BluetoothLEManufacturerData {
    type Vtable = IBluetoothLEManufacturerData_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x912dba18_6963_4533_b061_4694dafb34e5);
}
impl ::windows::core::RuntimeName for BluetoothLEManufacturerData {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.BluetoothLEManufacturerData";
}
impl ::core::convert::From<BluetoothLEManufacturerData> for ::windows::core::IUnknown {
    fn from(value: BluetoothLEManufacturerData) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BluetoothLEManufacturerData> for ::windows::core::IUnknown {
    fn from(value: &BluetoothLEManufacturerData) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BluetoothLEManufacturerData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BluetoothLEManufacturerData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BluetoothLEManufacturerData> for ::windows::core::IInspectable {
    fn from(value: BluetoothLEManufacturerData) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BluetoothLEManufacturerData> for ::windows::core::IInspectable {
    fn from(value: &BluetoothLEManufacturerData) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BluetoothLEManufacturerData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BluetoothLEManufacturerData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for BluetoothLEManufacturerData {}
unsafe impl ::core::marker::Sync for BluetoothLEManufacturerData {}
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
unsafe impl ::windows::core::Abi for BluetoothLEScanningMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for BluetoothLEScanningMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.Advertisement.BluetoothLEScanningMode;i4)");
}
impl ::windows::core::DefaultType for BluetoothLEScanningMode {
    type DefaultType = Self;
}
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisement(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBluetoothLEAdvertisement {
    type Vtable = IBluetoothLEAdvertisement_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x066fb2b7_33d1_4e7d_8367_cf81d0f79653);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisement_abi(
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
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, companyid: u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, r#type: u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementBytePattern(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBluetoothLEAdvertisementBytePattern {
    type Vtable = IBluetoothLEAdvertisementBytePattern_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfbfad7f2_b9c5_4a08_bc51_502f8ef68a79);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementBytePattern_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementBytePatternFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBluetoothLEAdvertisementBytePatternFactory {
    type Vtable = IBluetoothLEAdvertisementBytePatternFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc2e24d73_fd5c_4ec3_be2a_9ca6fa11b7bd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementBytePatternFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, datatype: u8, offset: i16, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementDataSection(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBluetoothLEAdvertisementDataSection {
    type Vtable = IBluetoothLEAdvertisementDataSection_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7213314_3a43_40f9_b6f0_92bfefc34ae3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementDataSection_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u8) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementDataSectionFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBluetoothLEAdvertisementDataSectionFactory {
    type Vtable = IBluetoothLEAdvertisementDataSectionFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe7a40942_a845_4045_bf7e_3e9971db8a6b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementDataSectionFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, datatype: u8, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementDataTypesStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBluetoothLEAdvertisementDataTypesStatics {
    type Vtable = IBluetoothLEAdvertisementDataTypesStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3bb6472f_0606_434b_a76e_74159f0684d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementDataTypesStatics_abi(
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementFilter(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBluetoothLEAdvertisementFilter {
    type Vtable = IBluetoothLEAdvertisementFilter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x131eb0d3_d04e_47b1_837e_49405bf6f80f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementFilter_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementPublisher(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBluetoothLEAdvertisementPublisher {
    type Vtable = IBluetoothLEAdvertisementPublisher_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcde820f9_d9fa_43d6_a264_ddd8b7da8b78);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementPublisher_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut BluetoothLEAdvertisementPublisherStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementPublisher2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBluetoothLEAdvertisementPublisher2 {
    type Vtable = IBluetoothLEAdvertisementPublisher2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfbdb545e_56f1_510f_a434_217fbd9e7bd2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementPublisher2_abi(
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
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementPublisherFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBluetoothLEAdvertisementPublisherFactory {
    type Vtable = IBluetoothLEAdvertisementPublisherFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c5f065e_b863_4981_a1af_1c544d8b0c0d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementPublisherFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, advertisement: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementPublisherStatusChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBluetoothLEAdvertisementPublisherStatusChangedEventArgs {
    type Vtable = IBluetoothLEAdvertisementPublisherStatusChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x09c2bd9f_2dff_4b23_86ee_0d14fb94aeae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementPublisherStatusChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut BluetoothLEAdvertisementPublisherStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::BluetoothError) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementPublisherStatusChangedEventArgs2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBluetoothLEAdvertisementPublisherStatusChangedEventArgs2 {
    type Vtable = IBluetoothLEAdvertisementPublisherStatusChangedEventArgs2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f62790e_dc88_5c8b_b34e_10b321850f88);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementPublisherStatusChangedEventArgs2_abi(
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
pub struct IBluetoothLEAdvertisementReceivedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBluetoothLEAdvertisementReceivedEventArgs {
    type Vtable = IBluetoothLEAdvertisementReceivedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27987ddf_e596_41be_8d43_9e6731d4a913);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementReceivedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut BluetoothLEAdvertisementType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementReceivedEventArgs2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBluetoothLEAdvertisementReceivedEventArgs2 {
    type Vtable = IBluetoothLEAdvertisementReceivedEventArgs2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x12d9c87b_0399_5f0e_a348_53b02b6b162e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementReceivedEventArgs2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::BluetoothAddressType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementWatcher(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBluetoothLEAdvertisementWatcher {
    type Vtable = IBluetoothLEAdvertisementWatcher_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa6ac336f_f3d3_4297_8d6c_c81ea6623f40);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementWatcher_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut BluetoothLEAdvertisementWatcherStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut BluetoothLEScanningMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: BluetoothLEScanningMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
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
pub struct IBluetoothLEAdvertisementWatcher2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBluetoothLEAdvertisementWatcher2 {
    type Vtable = IBluetoothLEAdvertisementWatcher2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x01bf26bc_b164_5805_90a3_e8a7997ff225);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementWatcher2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementWatcherFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBluetoothLEAdvertisementWatcherFactory {
    type Vtable = IBluetoothLEAdvertisementWatcherFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9aaf2d56_39ac_453e_b32a_85c657e017f1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementWatcherFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, advertisementfilter: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementWatcherStoppedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBluetoothLEAdvertisementWatcherStoppedEventArgs {
    type Vtable = IBluetoothLEAdvertisementWatcherStoppedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdd40f84d_e7b9_43e3_9c04_0685d085fd8c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementWatcherStoppedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::BluetoothError) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEManufacturerData(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBluetoothLEManufacturerData {
    type Vtable = IBluetoothLEManufacturerData_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x912dba18_6963_4533_b061_4694dafb34e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEManufacturerData_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBluetoothLEManufacturerDataFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBluetoothLEManufacturerDataFactory {
    type Vtable = IBluetoothLEManufacturerDataFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc09b39f8_319a_441e_8de5_66a81e877a6c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEManufacturerDataFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, companyid: u16, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
