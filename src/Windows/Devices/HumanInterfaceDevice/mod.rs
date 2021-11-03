#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct HidBooleanControl(::windows::runtime::IInspectable);
impl HidBooleanControl {
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
    pub fn UsagePage(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
    pub fn UsageId(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
    pub fn IsActive(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
    pub fn SetIsActive(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
    pub fn ControlDescription(&self) -> ::windows::runtime::Result<HidBooleanControlDescription> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<HidBooleanControlDescription>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HidBooleanControl {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.HumanInterfaceDevice.HidBooleanControl;{524df48a-3695-408c-bba2-e2eb5abfbc20})");
}
unsafe impl ::windows::runtime::Interface for HidBooleanControl {
    type Vtable = IHidBooleanControl_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1380840586, 13973, 16524, [187, 162, 226, 235, 90, 191, 188, 32]);
}
impl ::windows::runtime::RuntimeName for HidBooleanControl {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.HidBooleanControl";
}
unsafe impl ::std::marker::Send for HidBooleanControl {}
unsafe impl ::std::marker::Sync for HidBooleanControl {}
#[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct HidBooleanControlDescription(::windows::runtime::IInspectable);
impl HidBooleanControlDescription {
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
    pub fn ReportId(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
    pub fn ReportType(&self) -> ::windows::runtime::Result<HidReportType> {
        let this = self;
        unsafe {
            let mut result__: HidReportType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<HidReportType>(result__)
        }
    }
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
    pub fn UsagePage(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
    pub fn UsageId(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`, `Foundation_Collections`*"]
    pub fn ParentCollections(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<HidCollection>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<HidCollection>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
    pub fn IsAbsolute(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IHidBooleanControlDescription2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HidBooleanControlDescription {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.HumanInterfaceDevice.HidBooleanControlDescription;{6196e543-29d8-4a2a-8683-849e207bbe31})");
}
unsafe impl ::windows::runtime::Interface for HidBooleanControlDescription {
    type Vtable = IHidBooleanControlDescription_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1637279043, 10712, 18986, [134, 131, 132, 158, 32, 123, 190, 49]);
}
impl ::windows::runtime::RuntimeName for HidBooleanControlDescription {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.HidBooleanControlDescription";
}
unsafe impl ::std::marker::Send for HidBooleanControlDescription {}
unsafe impl ::std::marker::Sync for HidBooleanControlDescription {}
#[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct HidCollection(::windows::runtime::IInspectable);
impl HidCollection {
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<HidCollectionType> {
        let this = self;
        unsafe {
            let mut result__: HidCollectionType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<HidCollectionType>(result__)
        }
    }
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
    pub fn UsagePage(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
    pub fn UsageId(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HidCollection {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.HumanInterfaceDevice.HidCollection;{7189f5a3-32f1-46e3-befd-44d2663b7e6a})");
}
unsafe impl ::windows::runtime::Interface for HidCollection {
    type Vtable = IHidCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1904866723, 13041, 18147, [190, 253, 68, 210, 102, 59, 126, 106]);
}
impl ::windows::runtime::RuntimeName for HidCollection {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.HidCollection";
}
unsafe impl ::std::marker::Send for HidCollection {}
unsafe impl ::std::marker::Sync for HidCollection {}
#[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct HidCollectionType(pub i32);
impl HidCollectionType {
    pub const Physical: HidCollectionType = HidCollectionType(0i32);
    pub const Application: HidCollectionType = HidCollectionType(1i32);
    pub const Logical: HidCollectionType = HidCollectionType(2i32);
    pub const Report: HidCollectionType = HidCollectionType(3i32);
    pub const NamedArray: HidCollectionType = HidCollectionType(4i32);
    pub const UsageSwitch: HidCollectionType = HidCollectionType(5i32);
    pub const UsageModifier: HidCollectionType = HidCollectionType(6i32);
    pub const Other: HidCollectionType = HidCollectionType(7i32);
}
impl ::std::convert::From<i32> for HidCollectionType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HidCollectionType {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for HidCollectionType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.HumanInterfaceDevice.HidCollectionType;i4)");
}
impl ::windows::runtime::DefaultType for HidCollectionType {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct HidDevice(::windows::runtime::IInspectable);
impl HidDevice {
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
    pub fn VendorId(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
    pub fn ProductId(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
    pub fn Version(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
    pub fn UsagePage(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
    pub fn UsageId(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`, `Foundation`*"]
    pub fn GetInputReportAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<HidInputReport>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<HidInputReport>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`, `Foundation`*"]
    pub fn GetInputReportByIdAsync(&self, reportid: u16) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<HidInputReport>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), reportid, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<HidInputReport>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`, `Foundation`*"]
    pub fn GetFeatureReportAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<HidFeatureReport>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<HidFeatureReport>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`, `Foundation`*"]
    pub fn GetFeatureReportByIdAsync(&self, reportid: u16) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<HidFeatureReport>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), reportid, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<HidFeatureReport>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
    pub fn CreateOutputReport(&self) -> ::windows::runtime::Result<HidOutputReport> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<HidOutputReport>(result__)
        }
    }
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
    pub fn CreateOutputReportById(&self, reportid: u16) -> ::windows::runtime::Result<HidOutputReport> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), reportid, &mut result__).from_abi::<HidOutputReport>(result__)
        }
    }
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
    pub fn CreateFeatureReport(&self) -> ::windows::runtime::Result<HidFeatureReport> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<HidFeatureReport>(result__)
        }
    }
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
    pub fn CreateFeatureReportById(&self, reportid: u16) -> ::windows::runtime::Result<HidFeatureReport> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), reportid, &mut result__).from_abi::<HidFeatureReport>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`, `Foundation`*"]
    pub fn SendOutputReportAsync<'a, Param0: ::windows::runtime::IntoParam<'a, HidOutputReport>>(&self, outputreport: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), outputreport.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`, `Foundation`*"]
    pub fn SendFeatureReportAsync<'a, Param0: ::windows::runtime::IntoParam<'a, HidFeatureReport>>(&self, featurereport: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), featurereport.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`, `Foundation_Collections`*"]
    pub fn GetBooleanControlDescriptions(&self, reporttype: HidReportType, usagepage: u16, usageid: u16) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<HidBooleanControlDescription>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), reporttype, usagepage, usageid, &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<HidBooleanControlDescription>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`, `Foundation_Collections`*"]
    pub fn GetNumericControlDescriptions(&self, reporttype: HidReportType, usagepage: u16, usageid: u16) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<HidNumericControlDescription>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), reporttype, usagepage, usageid, &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<HidNumericControlDescription>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`, `Foundation`*"]
    pub fn InputReportReceived<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<HidDevice, HidInputReportReceivedEventArgs>>>(&self, reporthandler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), reporthandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`, `Foundation`*"]
    pub fn RemoveInputReportReceived<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
    pub fn GetDeviceSelector(usagepage: u16, usageid: u16) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IHidDeviceStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), usagepage, usageid, &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
    pub fn GetDeviceSelectorVidPid(usagepage: u16, usageid: u16, vendorid: u16, productid: u16) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IHidDeviceStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), usagepage, usageid, vendorid, productid, &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`, `Foundation`, `Storage`*"]
    pub fn FromIdAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(deviceid: Param0, accessmode: super::super::Storage::FileAccessMode) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<HidDevice>> {
        Self::IHidDeviceStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), deviceid.into_param().abi(), accessmode, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<HidDevice>>(result__)
        })
    }
    pub fn IHidDeviceStatics<R, F: FnOnce(&IHidDeviceStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<HidDevice, IHidDeviceStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HidDevice {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.HumanInterfaceDevice.HidDevice;{5f8a14e7-2200-432e-95da-d09b87d574a8})");
}
unsafe impl ::windows::runtime::Interface for HidDevice {
    type Vtable = IHidDevice_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1602884839, 8704, 17198, [149, 218, 208, 155, 135, 213, 116, 168]);
}
impl ::windows::runtime::RuntimeName for HidDevice {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.HidDevice";
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<HidDevice> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HidDevice) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&HidDevice> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HidDevice) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for HidDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &HidDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for HidDevice {}
unsafe impl ::std::marker::Sync for HidDevice {}
#[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct HidFeatureReport(::windows::runtime::IInspectable);
impl HidFeatureReport {
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`, `Storage_Streams`*"]
    pub fn Data(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`, `Storage_Streams`*"]
    pub fn SetData<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
    pub fn GetBooleanControl(&self, usagepage: u16, usageid: u16) -> ::windows::runtime::Result<HidBooleanControl> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), usagepage, usageid, &mut result__).from_abi::<HidBooleanControl>(result__)
        }
    }
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
    pub fn GetBooleanControlByDescription<'a, Param0: ::windows::runtime::IntoParam<'a, HidBooleanControlDescription>>(&self, controldescription: Param0) -> ::windows::runtime::Result<HidBooleanControl> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), controldescription.into_param().abi(), &mut result__).from_abi::<HidBooleanControl>(result__)
        }
    }
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
    pub fn GetNumericControl(&self, usagepage: u16, usageid: u16) -> ::windows::runtime::Result<HidNumericControl> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), usagepage, usageid, &mut result__).from_abi::<HidNumericControl>(result__)
        }
    }
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
    pub fn GetNumericControlByDescription<'a, Param0: ::windows::runtime::IntoParam<'a, HidNumericControlDescription>>(&self, controldescription: Param0) -> ::windows::runtime::Result<HidNumericControl> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), controldescription.into_param().abi(), &mut result__).from_abi::<HidNumericControl>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HidFeatureReport {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.HumanInterfaceDevice.HidFeatureReport;{841d9b79-5ae5-46e3-82ef-1fec5c8942f4})");
}
unsafe impl ::windows::runtime::Interface for HidFeatureReport {
    type Vtable = IHidFeatureReport_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2216532857, 23269, 18147, [130, 239, 31, 236, 92, 137, 66, 244]);
}
impl ::windows::runtime::RuntimeName for HidFeatureReport {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.HidFeatureReport";
}
unsafe impl ::std::marker::Send for HidFeatureReport {}
unsafe impl ::std::marker::Sync for HidFeatureReport {}
#[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct HidInputReport(::windows::runtime::IInspectable);
impl HidInputReport {
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`, `Storage_Streams`*"]
    pub fn Data(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`, `Foundation_Collections`*"]
    pub fn ActivatedBooleanControls(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<HidBooleanControl>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<HidBooleanControl>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`, `Foundation_Collections`*"]
    pub fn TransitionedBooleanControls(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<HidBooleanControl>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<HidBooleanControl>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
    pub fn GetBooleanControl(&self, usagepage: u16, usageid: u16) -> ::windows::runtime::Result<HidBooleanControl> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), usagepage, usageid, &mut result__).from_abi::<HidBooleanControl>(result__)
        }
    }
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
    pub fn GetBooleanControlByDescription<'a, Param0: ::windows::runtime::IntoParam<'a, HidBooleanControlDescription>>(&self, controldescription: Param0) -> ::windows::runtime::Result<HidBooleanControl> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), controldescription.into_param().abi(), &mut result__).from_abi::<HidBooleanControl>(result__)
        }
    }
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
    pub fn GetNumericControl(&self, usagepage: u16, usageid: u16) -> ::windows::runtime::Result<HidNumericControl> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), usagepage, usageid, &mut result__).from_abi::<HidNumericControl>(result__)
        }
    }
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
    pub fn GetNumericControlByDescription<'a, Param0: ::windows::runtime::IntoParam<'a, HidNumericControlDescription>>(&self, controldescription: Param0) -> ::windows::runtime::Result<HidNumericControl> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), controldescription.into_param().abi(), &mut result__).from_abi::<HidNumericControl>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HidInputReport {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.HumanInterfaceDevice.HidInputReport;{c35d0e50-f7e7-4e8d-b23e-cabbe56b90e9})");
}
unsafe impl ::windows::runtime::Interface for HidInputReport {
    type Vtable = IHidInputReport_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3277655632, 63463, 20109, [178, 62, 202, 187, 229, 107, 144, 233]);
}
impl ::windows::runtime::RuntimeName for HidInputReport {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.HidInputReport";
}
unsafe impl ::std::marker::Send for HidInputReport {}
unsafe impl ::std::marker::Sync for HidInputReport {}
#[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct HidInputReportReceivedEventArgs(::windows::runtime::IInspectable);
impl HidInputReportReceivedEventArgs {
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
    pub fn Report(&self) -> ::windows::runtime::Result<HidInputReport> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<HidInputReport>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HidInputReportReceivedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.HumanInterfaceDevice.HidInputReportReceivedEventArgs;{7059c5cb-59b2-4dc2-985c-0adc6136fa2d})");
}
unsafe impl ::windows::runtime::Interface for HidInputReportReceivedEventArgs {
    type Vtable = IHidInputReportReceivedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1884931531, 22962, 19906, [152, 92, 10, 220, 97, 54, 250, 45]);
}
impl ::windows::runtime::RuntimeName for HidInputReportReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.HidInputReportReceivedEventArgs";
}
unsafe impl ::std::marker::Send for HidInputReportReceivedEventArgs {}
unsafe impl ::std::marker::Sync for HidInputReportReceivedEventArgs {}
#[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct HidNumericControl(::windows::runtime::IInspectable);
impl HidNumericControl {
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
    pub fn IsGrouped(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
    pub fn UsagePage(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
    pub fn UsageId(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
    pub fn Value(&self) -> ::windows::runtime::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
    pub fn SetValue(&self, value: i64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
    pub fn ScaledValue(&self) -> ::windows::runtime::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
    pub fn SetScaledValue(&self, value: i64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
    pub fn ControlDescription(&self) -> ::windows::runtime::Result<HidNumericControlDescription> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<HidNumericControlDescription>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HidNumericControl {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.HumanInterfaceDevice.HidNumericControl;{e38a12a5-35a7-4b75-89c8-fb1f28b10823})");
}
unsafe impl ::windows::runtime::Interface for HidNumericControl {
    type Vtable = IHidNumericControl_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3817476773, 13735, 19317, [137, 200, 251, 31, 40, 177, 8, 35]);
}
impl ::windows::runtime::RuntimeName for HidNumericControl {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.HidNumericControl";
}
unsafe impl ::std::marker::Send for HidNumericControl {}
unsafe impl ::std::marker::Sync for HidNumericControl {}
#[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct HidNumericControlDescription(::windows::runtime::IInspectable);
impl HidNumericControlDescription {
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
    pub fn ReportId(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
    pub fn ReportType(&self) -> ::windows::runtime::Result<HidReportType> {
        let this = self;
        unsafe {
            let mut result__: HidReportType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<HidReportType>(result__)
        }
    }
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
    pub fn ReportSize(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
    pub fn ReportCount(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
    pub fn UsagePage(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
    pub fn UsageId(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
    pub fn LogicalMinimum(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
    pub fn LogicalMaximum(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
    pub fn PhysicalMinimum(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
    pub fn PhysicalMaximum(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
    pub fn UnitExponent(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
    pub fn Unit(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
    pub fn IsAbsolute(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
    pub fn HasNull(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`, `Foundation_Collections`*"]
    pub fn ParentCollections(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<HidCollection>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<HidCollection>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HidNumericControlDescription {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.HumanInterfaceDevice.HidNumericControlDescription;{638d5e86-1d97-4c75-927f-5ff58ba05e32})");
}
unsafe impl ::windows::runtime::Interface for HidNumericControlDescription {
    type Vtable = IHidNumericControlDescription_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1670209158, 7575, 19573, [146, 127, 95, 245, 139, 160, 94, 50]);
}
impl ::windows::runtime::RuntimeName for HidNumericControlDescription {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.HidNumericControlDescription";
}
unsafe impl ::std::marker::Send for HidNumericControlDescription {}
unsafe impl ::std::marker::Sync for HidNumericControlDescription {}
#[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct HidOutputReport(::windows::runtime::IInspectable);
impl HidOutputReport {
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`, `Storage_Streams`*"]
    pub fn Data(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`, `Storage_Streams`*"]
    pub fn SetData<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
    pub fn GetBooleanControl(&self, usagepage: u16, usageid: u16) -> ::windows::runtime::Result<HidBooleanControl> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), usagepage, usageid, &mut result__).from_abi::<HidBooleanControl>(result__)
        }
    }
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
    pub fn GetBooleanControlByDescription<'a, Param0: ::windows::runtime::IntoParam<'a, HidBooleanControlDescription>>(&self, controldescription: Param0) -> ::windows::runtime::Result<HidBooleanControl> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), controldescription.into_param().abi(), &mut result__).from_abi::<HidBooleanControl>(result__)
        }
    }
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
    pub fn GetNumericControl(&self, usagepage: u16, usageid: u16) -> ::windows::runtime::Result<HidNumericControl> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), usagepage, usageid, &mut result__).from_abi::<HidNumericControl>(result__)
        }
    }
    #[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
    pub fn GetNumericControlByDescription<'a, Param0: ::windows::runtime::IntoParam<'a, HidNumericControlDescription>>(&self, controldescription: Param0) -> ::windows::runtime::Result<HidNumericControl> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), controldescription.into_param().abi(), &mut result__).from_abi::<HidNumericControl>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HidOutputReport {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.HumanInterfaceDevice.HidOutputReport;{62cb2544-c896-4463-93c1-df9db053c450})");
}
unsafe impl ::windows::runtime::Interface for HidOutputReport {
    type Vtable = IHidOutputReport_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1657480516, 51350, 17507, [147, 193, 223, 157, 176, 83, 196, 80]);
}
impl ::windows::runtime::RuntimeName for HidOutputReport {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.HidOutputReport";
}
unsafe impl ::std::marker::Send for HidOutputReport {}
unsafe impl ::std::marker::Sync for HidOutputReport {}
#[doc = "*Required features: `Devices_HumanInterfaceDevice`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct HidReportType(pub i32);
impl HidReportType {
    pub const Input: HidReportType = HidReportType(0i32);
    pub const Output: HidReportType = HidReportType(1i32);
    pub const Feature: HidReportType = HidReportType(2i32);
}
impl ::std::convert::From<i32> for HidReportType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HidReportType {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for HidReportType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.HumanInterfaceDevice.HidReportType;i4)");
}
impl ::windows::runtime::DefaultType for HidReportType {
    type DefaultType = Self;
}
#[repr(transparent)]
#[doc(hidden)]
pub struct IHidBooleanControl(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHidBooleanControl {
    type Vtable = IHidBooleanControl_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1380840586, 13973, 16524, [187, 162, 226, 235, 90, 191, 188, 32]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidBooleanControl_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHidBooleanControlDescription(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHidBooleanControlDescription {
    type Vtable = IHidBooleanControlDescription_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1637279043, 10712, 18986, [134, 131, 132, 158, 32, 123, 190, 49]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidBooleanControlDescription_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut HidReportType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHidBooleanControlDescription2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHidBooleanControlDescription2 {
    type Vtable = IHidBooleanControlDescription2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3371094762, 35447, 19510, [170, 0, 95, 240, 68, 157, 62, 115]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidBooleanControlDescription2_abi(
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
pub struct IHidCollection(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHidCollection {
    type Vtable = IHidCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1904866723, 13041, 18147, [190, 253, 68, 210, 102, 59, 126, 106]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut HidCollectionType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHidDevice(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHidDevice {
    type Vtable = IHidDevice_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1602884839, 8704, 17198, [149, 218, 208, 155, 135, 213, 116, 168]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidDevice_abi(
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
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reportid: u16, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reportid: u16, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reportid: u16, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reportid: u16, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, outputreport: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, featurereport: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reporttype: HidReportType, usagepage: u16, usageid: u16, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reporttype: HidReportType, usagepage: u16, usageid: u16, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reporthandler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHidDeviceStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHidDeviceStatics {
    type Vtable = IHidDeviceStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2656666084, 38998, 16780, [159, 115, 119, 222, 12, 216, 87, 84]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidDeviceStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, usagepage: u16, usageid: u16, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, usagepage: u16, usageid: u16, vendorid: u16, productid: u16, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deviceid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, accessmode: super::super::Storage::FileAccessMode, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHidFeatureReport(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHidFeatureReport {
    type Vtable = IHidFeatureReport_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2216532857, 23269, 18147, [130, 239, 31, 236, 92, 137, 66, 244]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidFeatureReport_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, usagepage: u16, usageid: u16, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, controldescription: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, usagepage: u16, usageid: u16, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, controldescription: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHidInputReport(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHidInputReport {
    type Vtable = IHidInputReport_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3277655632, 63463, 20109, [178, 62, 202, 187, 229, 107, 144, 233]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidInputReport_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, usagepage: u16, usageid: u16, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, controldescription: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, usagepage: u16, usageid: u16, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, controldescription: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHidInputReportReceivedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHidInputReportReceivedEventArgs {
    type Vtable = IHidInputReportReceivedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1884931531, 22962, 19906, [152, 92, 10, 220, 97, 54, 250, 45]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidInputReportReceivedEventArgs_abi(
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
pub struct IHidNumericControl(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHidNumericControl {
    type Vtable = IHidNumericControl_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3817476773, 13735, 19317, [137, 200, 251, 31, 40, 177, 8, 35]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidNumericControl_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHidNumericControlDescription(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHidNumericControlDescription {
    type Vtable = IHidNumericControlDescription_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1670209158, 7575, 19573, [146, 127, 95, 245, 139, 160, 94, 50]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidNumericControlDescription_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut HidReportType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHidOutputReport(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHidOutputReport {
    type Vtable = IHidOutputReport_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1657480516, 51350, 17507, [147, 193, 223, 157, 176, 83, 196, 80]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidOutputReport_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, usagepage: u16, usageid: u16, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, controldescription: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, usagepage: u16, usageid: u16, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, controldescription: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
