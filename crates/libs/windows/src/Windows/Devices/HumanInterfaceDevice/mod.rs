#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
#[repr(transparent)]
pub struct HidBooleanControl(::windows::core::IUnknown);
impl HidBooleanControl {
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Id)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn UsagePage(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UsagePage)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn UsageId(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UsageId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn IsActive(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsActive)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn SetIsActive(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsActive)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn ControlDescription(&self) -> ::windows::core::Result<HidBooleanControlDescription> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ControlDescription)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HidBooleanControlDescription>(result__)
        }
    }
}
impl ::core::clone::Clone for HidBooleanControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HidBooleanControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HidBooleanControl {}
impl ::core::fmt::Debug for HidBooleanControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HidBooleanControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HidBooleanControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.HumanInterfaceDevice.HidBooleanControl;{524df48a-3695-408c-bba2-e2eb5abfbc20})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for HidBooleanControl {
    type Vtable = IHidBooleanControl_Vtbl;
    const IID: ::windows::core::GUID = <IHidBooleanControl as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HidBooleanControl {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.HidBooleanControl";
}
impl ::core::convert::From<HidBooleanControl> for ::windows::core::IUnknown {
    fn from(value: HidBooleanControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HidBooleanControl> for ::windows::core::IUnknown {
    fn from(value: &HidBooleanControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HidBooleanControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a HidBooleanControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HidBooleanControl> for ::windows::core::IInspectable {
    fn from(value: HidBooleanControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HidBooleanControl> for ::windows::core::IInspectable {
    fn from(value: &HidBooleanControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HidBooleanControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a HidBooleanControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for HidBooleanControl {}
unsafe impl ::core::marker::Sync for HidBooleanControl {}
#[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
#[repr(transparent)]
pub struct HidBooleanControlDescription(::windows::core::IUnknown);
impl HidBooleanControlDescription {
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Id)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn ReportId(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReportId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn ReportType(&self) -> ::windows::core::Result<HidReportType> {
        let this = self;
        unsafe {
            let mut result__: HidReportType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReportType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HidReportType>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn UsagePage(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UsagePage)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn UsageId(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UsageId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ParentCollections(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<HidCollection>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ParentCollections)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<HidCollection>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn IsAbsolute(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IHidBooleanControlDescription2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsAbsolute)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for HidBooleanControlDescription {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HidBooleanControlDescription {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HidBooleanControlDescription {}
impl ::core::fmt::Debug for HidBooleanControlDescription {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HidBooleanControlDescription").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HidBooleanControlDescription {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.HumanInterfaceDevice.HidBooleanControlDescription;{6196e543-29d8-4a2a-8683-849e207bbe31})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for HidBooleanControlDescription {
    type Vtable = IHidBooleanControlDescription_Vtbl;
    const IID: ::windows::core::GUID = <IHidBooleanControlDescription as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HidBooleanControlDescription {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.HidBooleanControlDescription";
}
impl ::core::convert::From<HidBooleanControlDescription> for ::windows::core::IUnknown {
    fn from(value: HidBooleanControlDescription) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HidBooleanControlDescription> for ::windows::core::IUnknown {
    fn from(value: &HidBooleanControlDescription) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HidBooleanControlDescription {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a HidBooleanControlDescription {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HidBooleanControlDescription> for ::windows::core::IInspectable {
    fn from(value: HidBooleanControlDescription) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HidBooleanControlDescription> for ::windows::core::IInspectable {
    fn from(value: &HidBooleanControlDescription) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HidBooleanControlDescription {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a HidBooleanControlDescription {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for HidBooleanControlDescription {}
unsafe impl ::core::marker::Sync for HidBooleanControlDescription {}
#[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
#[repr(transparent)]
pub struct HidCollection(::windows::core::IUnknown);
impl HidCollection {
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Id)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn Type(&self) -> ::windows::core::Result<HidCollectionType> {
        let this = self;
        unsafe {
            let mut result__: HidCollectionType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Type)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HidCollectionType>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn UsagePage(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UsagePage)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn UsageId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UsageId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for HidCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HidCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HidCollection {}
impl ::core::fmt::Debug for HidCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HidCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HidCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.HumanInterfaceDevice.HidCollection;{7189f5a3-32f1-46e3-befd-44d2663b7e6a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for HidCollection {
    type Vtable = IHidCollection_Vtbl;
    const IID: ::windows::core::GUID = <IHidCollection as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HidCollection {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.HidCollection";
}
impl ::core::convert::From<HidCollection> for ::windows::core::IUnknown {
    fn from(value: HidCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HidCollection> for ::windows::core::IUnknown {
    fn from(value: &HidCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HidCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a HidCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HidCollection> for ::windows::core::IInspectable {
    fn from(value: HidCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HidCollection> for ::windows::core::IInspectable {
    fn from(value: &HidCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HidCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a HidCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for HidCollection {}
unsafe impl ::core::marker::Sync for HidCollection {}
#[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HidCollectionType(pub i32);
impl HidCollectionType {
    pub const Physical: Self = Self(0i32);
    pub const Application: Self = Self(1i32);
    pub const Logical: Self = Self(2i32);
    pub const Report: Self = Self(3i32);
    pub const NamedArray: Self = Self(4i32);
    pub const UsageSwitch: Self = Self(5i32);
    pub const UsageModifier: Self = Self(6i32);
    pub const Other: Self = Self(7i32);
}
impl ::core::marker::Copy for HidCollectionType {}
impl ::core::clone::Clone for HidCollectionType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HidCollectionType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HidCollectionType {
    type Abi = Self;
}
impl ::core::fmt::Debug for HidCollectionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HidCollectionType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HidCollectionType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.HumanInterfaceDevice.HidCollectionType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
#[repr(transparent)]
pub struct HidDevice(::windows::core::IUnknown);
impl HidDevice {
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn VendorId(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).VendorId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn ProductId(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProductId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn Version(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Version)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn UsagePage(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UsagePage)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn UsageId(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UsageId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetInputReportAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<HidInputReport>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetInputReportAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<HidInputReport>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetInputReportByIdAsync(&self, reportid: u16) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<HidInputReport>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetInputReportByIdAsync)(::core::mem::transmute_copy(this), reportid, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<HidInputReport>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetFeatureReportAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<HidFeatureReport>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetFeatureReportAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<HidFeatureReport>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetFeatureReportByIdAsync(&self, reportid: u16) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<HidFeatureReport>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetFeatureReportByIdAsync)(::core::mem::transmute_copy(this), reportid, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<HidFeatureReport>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn CreateOutputReport(&self) -> ::windows::core::Result<HidOutputReport> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateOutputReport)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HidOutputReport>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn CreateOutputReportById(&self, reportid: u16) -> ::windows::core::Result<HidOutputReport> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateOutputReportById)(::core::mem::transmute_copy(this), reportid, &mut result__).from_abi::<HidOutputReport>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn CreateFeatureReport(&self) -> ::windows::core::Result<HidFeatureReport> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFeatureReport)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HidFeatureReport>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn CreateFeatureReportById(&self, reportid: u16) -> ::windows::core::Result<HidFeatureReport> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFeatureReportById)(::core::mem::transmute_copy(this), reportid, &mut result__).from_abi::<HidFeatureReport>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SendOutputReportAsync<'a, Param0: ::windows::core::IntoParam<'a, HidOutputReport>>(&self, outputreport: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SendOutputReportAsync)(::core::mem::transmute_copy(this), outputreport.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SendFeatureReportAsync<'a, Param0: ::windows::core::IntoParam<'a, HidFeatureReport>>(&self, featurereport: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SendFeatureReportAsync)(::core::mem::transmute_copy(this), featurereport.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetBooleanControlDescriptions(&self, reporttype: HidReportType, usagepage: u16, usageid: u16) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<HidBooleanControlDescription>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetBooleanControlDescriptions)(::core::mem::transmute_copy(this), reporttype, usagepage, usageid, &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<HidBooleanControlDescription>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetNumericControlDescriptions(&self, reporttype: HidReportType, usagepage: u16, usageid: u16) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<HidNumericControlDescription>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetNumericControlDescriptions)(::core::mem::transmute_copy(this), reporttype, usagepage, usageid, &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<HidNumericControlDescription>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn InputReportReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<HidDevice, HidInputReportReceivedEventArgs>>>(&self, reporthandler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InputReportReceived)(::core::mem::transmute_copy(this), reporthandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveInputReportReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveInputReportReceived)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn GetDeviceSelector(usagepage: u16, usageid: u16) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IHidDeviceStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeviceSelector)(::core::mem::transmute_copy(this), usagepage, usageid, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn GetDeviceSelectorVidPid(usagepage: u16, usageid: u16, vendorid: u16, productid: u16) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IHidDeviceStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeviceSelectorVidPid)(::core::mem::transmute_copy(this), usagepage, usageid, vendorid, productid, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`, `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn FromIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0, accessmode: super::super::Storage::FileAccessMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<HidDevice>> {
        Self::IHidDeviceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FromIdAsync)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), accessmode, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<HidDevice>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IHidDeviceStatics<R, F: FnOnce(&IHidDeviceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HidDevice, IHidDeviceStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HidDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HidDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HidDevice {}
impl ::core::fmt::Debug for HidDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HidDevice").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HidDevice {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.HumanInterfaceDevice.HidDevice;{5f8a14e7-2200-432e-95da-d09b87d574a8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for HidDevice {
    type Vtable = IHidDevice_Vtbl;
    const IID: ::windows::core::GUID = <IHidDevice as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HidDevice {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.HidDevice";
}
impl ::core::convert::From<HidDevice> for ::windows::core::IUnknown {
    fn from(value: HidDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HidDevice> for ::windows::core::IUnknown {
    fn from(value: &HidDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HidDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a HidDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HidDevice> for ::windows::core::IInspectable {
    fn from(value: HidDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HidDevice> for ::windows::core::IInspectable {
    fn from(value: &HidDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HidDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a HidDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HidDevice> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: HidDevice) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HidDevice> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HidDevice) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for HidDevice {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &HidDevice {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HidDevice {}
unsafe impl ::core::marker::Sync for HidDevice {}
#[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
#[repr(transparent)]
pub struct HidFeatureReport(::windows::core::IUnknown);
impl HidFeatureReport {
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn Id(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Id)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Data(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Data)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetData<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetData)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn GetBooleanControl(&self, usagepage: u16, usageid: u16) -> ::windows::core::Result<HidBooleanControl> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetBooleanControl)(::core::mem::transmute_copy(this), usagepage, usageid, &mut result__).from_abi::<HidBooleanControl>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn GetBooleanControlByDescription<'a, Param0: ::windows::core::IntoParam<'a, HidBooleanControlDescription>>(&self, controldescription: Param0) -> ::windows::core::Result<HidBooleanControl> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetBooleanControlByDescription)(::core::mem::transmute_copy(this), controldescription.into_param().abi(), &mut result__).from_abi::<HidBooleanControl>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn GetNumericControl(&self, usagepage: u16, usageid: u16) -> ::windows::core::Result<HidNumericControl> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetNumericControl)(::core::mem::transmute_copy(this), usagepage, usageid, &mut result__).from_abi::<HidNumericControl>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn GetNumericControlByDescription<'a, Param0: ::windows::core::IntoParam<'a, HidNumericControlDescription>>(&self, controldescription: Param0) -> ::windows::core::Result<HidNumericControl> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetNumericControlByDescription)(::core::mem::transmute_copy(this), controldescription.into_param().abi(), &mut result__).from_abi::<HidNumericControl>(result__)
        }
    }
}
impl ::core::clone::Clone for HidFeatureReport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HidFeatureReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HidFeatureReport {}
impl ::core::fmt::Debug for HidFeatureReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HidFeatureReport").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HidFeatureReport {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.HumanInterfaceDevice.HidFeatureReport;{841d9b79-5ae5-46e3-82ef-1fec5c8942f4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for HidFeatureReport {
    type Vtable = IHidFeatureReport_Vtbl;
    const IID: ::windows::core::GUID = <IHidFeatureReport as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HidFeatureReport {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.HidFeatureReport";
}
impl ::core::convert::From<HidFeatureReport> for ::windows::core::IUnknown {
    fn from(value: HidFeatureReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HidFeatureReport> for ::windows::core::IUnknown {
    fn from(value: &HidFeatureReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HidFeatureReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a HidFeatureReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HidFeatureReport> for ::windows::core::IInspectable {
    fn from(value: HidFeatureReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HidFeatureReport> for ::windows::core::IInspectable {
    fn from(value: &HidFeatureReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HidFeatureReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a HidFeatureReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for HidFeatureReport {}
unsafe impl ::core::marker::Sync for HidFeatureReport {}
#[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
#[repr(transparent)]
pub struct HidInputReport(::windows::core::IUnknown);
impl HidInputReport {
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn Id(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Id)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Data(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Data)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ActivatedBooleanControls(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<HidBooleanControl>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ActivatedBooleanControls)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<HidBooleanControl>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn TransitionedBooleanControls(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<HidBooleanControl>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TransitionedBooleanControls)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<HidBooleanControl>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn GetBooleanControl(&self, usagepage: u16, usageid: u16) -> ::windows::core::Result<HidBooleanControl> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetBooleanControl)(::core::mem::transmute_copy(this), usagepage, usageid, &mut result__).from_abi::<HidBooleanControl>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn GetBooleanControlByDescription<'a, Param0: ::windows::core::IntoParam<'a, HidBooleanControlDescription>>(&self, controldescription: Param0) -> ::windows::core::Result<HidBooleanControl> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetBooleanControlByDescription)(::core::mem::transmute_copy(this), controldescription.into_param().abi(), &mut result__).from_abi::<HidBooleanControl>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn GetNumericControl(&self, usagepage: u16, usageid: u16) -> ::windows::core::Result<HidNumericControl> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetNumericControl)(::core::mem::transmute_copy(this), usagepage, usageid, &mut result__).from_abi::<HidNumericControl>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn GetNumericControlByDescription<'a, Param0: ::windows::core::IntoParam<'a, HidNumericControlDescription>>(&self, controldescription: Param0) -> ::windows::core::Result<HidNumericControl> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetNumericControlByDescription)(::core::mem::transmute_copy(this), controldescription.into_param().abi(), &mut result__).from_abi::<HidNumericControl>(result__)
        }
    }
}
impl ::core::clone::Clone for HidInputReport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HidInputReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HidInputReport {}
impl ::core::fmt::Debug for HidInputReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HidInputReport").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HidInputReport {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.HumanInterfaceDevice.HidInputReport;{c35d0e50-f7e7-4e8d-b23e-cabbe56b90e9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for HidInputReport {
    type Vtable = IHidInputReport_Vtbl;
    const IID: ::windows::core::GUID = <IHidInputReport as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HidInputReport {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.HidInputReport";
}
impl ::core::convert::From<HidInputReport> for ::windows::core::IUnknown {
    fn from(value: HidInputReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HidInputReport> for ::windows::core::IUnknown {
    fn from(value: &HidInputReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HidInputReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a HidInputReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HidInputReport> for ::windows::core::IInspectable {
    fn from(value: HidInputReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HidInputReport> for ::windows::core::IInspectable {
    fn from(value: &HidInputReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HidInputReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a HidInputReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for HidInputReport {}
unsafe impl ::core::marker::Sync for HidInputReport {}
#[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
#[repr(transparent)]
pub struct HidInputReportReceivedEventArgs(::windows::core::IUnknown);
impl HidInputReportReceivedEventArgs {
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn Report(&self) -> ::windows::core::Result<HidInputReport> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Report)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HidInputReport>(result__)
        }
    }
}
impl ::core::clone::Clone for HidInputReportReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HidInputReportReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HidInputReportReceivedEventArgs {}
impl ::core::fmt::Debug for HidInputReportReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HidInputReportReceivedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HidInputReportReceivedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.HumanInterfaceDevice.HidInputReportReceivedEventArgs;{7059c5cb-59b2-4dc2-985c-0adc6136fa2d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for HidInputReportReceivedEventArgs {
    type Vtable = IHidInputReportReceivedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IHidInputReportReceivedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HidInputReportReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.HidInputReportReceivedEventArgs";
}
impl ::core::convert::From<HidInputReportReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: HidInputReportReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HidInputReportReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &HidInputReportReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HidInputReportReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a HidInputReportReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HidInputReportReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: HidInputReportReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HidInputReportReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &HidInputReportReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HidInputReportReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a HidInputReportReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for HidInputReportReceivedEventArgs {}
unsafe impl ::core::marker::Sync for HidInputReportReceivedEventArgs {}
#[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
#[repr(transparent)]
pub struct HidNumericControl(::windows::core::IUnknown);
impl HidNumericControl {
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Id)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn IsGrouped(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsGrouped)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn UsagePage(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UsagePage)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn UsageId(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UsageId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn Value(&self) -> ::windows::core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Value)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn SetValue(&self, value: i64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetValue)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn ScaledValue(&self) -> ::windows::core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ScaledValue)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn SetScaledValue(&self, value: i64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetScaledValue)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn ControlDescription(&self) -> ::windows::core::Result<HidNumericControlDescription> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ControlDescription)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HidNumericControlDescription>(result__)
        }
    }
}
impl ::core::clone::Clone for HidNumericControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HidNumericControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HidNumericControl {}
impl ::core::fmt::Debug for HidNumericControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HidNumericControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HidNumericControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.HumanInterfaceDevice.HidNumericControl;{e38a12a5-35a7-4b75-89c8-fb1f28b10823})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for HidNumericControl {
    type Vtable = IHidNumericControl_Vtbl;
    const IID: ::windows::core::GUID = <IHidNumericControl as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HidNumericControl {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.HidNumericControl";
}
impl ::core::convert::From<HidNumericControl> for ::windows::core::IUnknown {
    fn from(value: HidNumericControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HidNumericControl> for ::windows::core::IUnknown {
    fn from(value: &HidNumericControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HidNumericControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a HidNumericControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HidNumericControl> for ::windows::core::IInspectable {
    fn from(value: HidNumericControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HidNumericControl> for ::windows::core::IInspectable {
    fn from(value: &HidNumericControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HidNumericControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a HidNumericControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for HidNumericControl {}
unsafe impl ::core::marker::Sync for HidNumericControl {}
#[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
#[repr(transparent)]
pub struct HidNumericControlDescription(::windows::core::IUnknown);
impl HidNumericControlDescription {
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Id)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn ReportId(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReportId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn ReportType(&self) -> ::windows::core::Result<HidReportType> {
        let this = self;
        unsafe {
            let mut result__: HidReportType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReportType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HidReportType>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn ReportSize(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReportSize)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn ReportCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReportCount)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn UsagePage(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UsagePage)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn UsageId(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UsageId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn LogicalMinimum(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LogicalMinimum)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn LogicalMaximum(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LogicalMaximum)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn PhysicalMinimum(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PhysicalMinimum)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn PhysicalMaximum(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PhysicalMaximum)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn UnitExponent(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UnitExponent)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn Unit(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Unit)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn IsAbsolute(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsAbsolute)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn HasNull(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HasNull)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ParentCollections(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<HidCollection>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ParentCollections)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<HidCollection>>(result__)
        }
    }
}
impl ::core::clone::Clone for HidNumericControlDescription {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HidNumericControlDescription {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HidNumericControlDescription {}
impl ::core::fmt::Debug for HidNumericControlDescription {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HidNumericControlDescription").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HidNumericControlDescription {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.HumanInterfaceDevice.HidNumericControlDescription;{638d5e86-1d97-4c75-927f-5ff58ba05e32})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for HidNumericControlDescription {
    type Vtable = IHidNumericControlDescription_Vtbl;
    const IID: ::windows::core::GUID = <IHidNumericControlDescription as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HidNumericControlDescription {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.HidNumericControlDescription";
}
impl ::core::convert::From<HidNumericControlDescription> for ::windows::core::IUnknown {
    fn from(value: HidNumericControlDescription) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HidNumericControlDescription> for ::windows::core::IUnknown {
    fn from(value: &HidNumericControlDescription) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HidNumericControlDescription {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a HidNumericControlDescription {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HidNumericControlDescription> for ::windows::core::IInspectable {
    fn from(value: HidNumericControlDescription) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HidNumericControlDescription> for ::windows::core::IInspectable {
    fn from(value: &HidNumericControlDescription) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HidNumericControlDescription {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a HidNumericControlDescription {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for HidNumericControlDescription {}
unsafe impl ::core::marker::Sync for HidNumericControlDescription {}
#[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
#[repr(transparent)]
pub struct HidOutputReport(::windows::core::IUnknown);
impl HidOutputReport {
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn Id(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Id)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Data(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Data)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetData<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetData)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn GetBooleanControl(&self, usagepage: u16, usageid: u16) -> ::windows::core::Result<HidBooleanControl> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetBooleanControl)(::core::mem::transmute_copy(this), usagepage, usageid, &mut result__).from_abi::<HidBooleanControl>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn GetBooleanControlByDescription<'a, Param0: ::windows::core::IntoParam<'a, HidBooleanControlDescription>>(&self, controldescription: Param0) -> ::windows::core::Result<HidBooleanControl> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetBooleanControlByDescription)(::core::mem::transmute_copy(this), controldescription.into_param().abi(), &mut result__).from_abi::<HidBooleanControl>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn GetNumericControl(&self, usagepage: u16, usageid: u16) -> ::windows::core::Result<HidNumericControl> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetNumericControl)(::core::mem::transmute_copy(this), usagepage, usageid, &mut result__).from_abi::<HidNumericControl>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    pub fn GetNumericControlByDescription<'a, Param0: ::windows::core::IntoParam<'a, HidNumericControlDescription>>(&self, controldescription: Param0) -> ::windows::core::Result<HidNumericControl> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetNumericControlByDescription)(::core::mem::transmute_copy(this), controldescription.into_param().abi(), &mut result__).from_abi::<HidNumericControl>(result__)
        }
    }
}
impl ::core::clone::Clone for HidOutputReport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HidOutputReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HidOutputReport {}
impl ::core::fmt::Debug for HidOutputReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HidOutputReport").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HidOutputReport {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.HumanInterfaceDevice.HidOutputReport;{62cb2544-c896-4463-93c1-df9db053c450})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for HidOutputReport {
    type Vtable = IHidOutputReport_Vtbl;
    const IID: ::windows::core::GUID = <IHidOutputReport as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HidOutputReport {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.HidOutputReport";
}
impl ::core::convert::From<HidOutputReport> for ::windows::core::IUnknown {
    fn from(value: HidOutputReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HidOutputReport> for ::windows::core::IUnknown {
    fn from(value: &HidOutputReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HidOutputReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a HidOutputReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HidOutputReport> for ::windows::core::IInspectable {
    fn from(value: HidOutputReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HidOutputReport> for ::windows::core::IInspectable {
    fn from(value: &HidOutputReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HidOutputReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a HidOutputReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for HidOutputReport {}
unsafe impl ::core::marker::Sync for HidOutputReport {}
#[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HidReportType(pub i32);
impl HidReportType {
    pub const Input: Self = Self(0i32);
    pub const Output: Self = Self(1i32);
    pub const Feature: Self = Self(2i32);
}
impl ::core::marker::Copy for HidReportType {}
impl ::core::clone::Clone for HidReportType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HidReportType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HidReportType {
    type Abi = Self;
}
impl ::core::fmt::Debug for HidReportType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HidReportType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HidReportType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.HumanInterfaceDevice.HidReportType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHidBooleanControl(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHidBooleanControl {
    type Vtable = IHidBooleanControl_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x524df48a_3695_408c_bba2_e2eb5abfbc20);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidBooleanControl_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub UsagePage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub UsageId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub IsActive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsActive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub ControlDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHidBooleanControlDescription(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHidBooleanControlDescription {
    type Vtable = IHidBooleanControlDescription_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6196e543_29d8_4a2a_8683_849e207bbe31);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidBooleanControlDescription_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub ReportId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub ReportType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HidReportType) -> ::windows::core::HRESULT,
    pub UsagePage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub UsageId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ParentCollections: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ParentCollections: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHidBooleanControlDescription2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHidBooleanControlDescription2 {
    type Vtable = IHidBooleanControlDescription2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc8eed2ea_8a77_4c36_aa00_5ff0449d3e73);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidBooleanControlDescription2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsAbsolute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHidCollection(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHidCollection {
    type Vtable = IHidCollection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7189f5a3_32f1_46e3_befd_44d2663b7e6a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidCollection_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HidCollectionType) -> ::windows::core::HRESULT,
    pub UsagePage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub UsageId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHidDevice(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHidDevice {
    type Vtable = IHidDevice_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5f8a14e7_2200_432e_95da_d09b87d574a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidDevice_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub VendorId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub ProductId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub Version: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub UsagePage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub UsageId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetInputReportAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetInputReportAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetInputReportByIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reportid: u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetInputReportByIdAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetFeatureReportAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetFeatureReportAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetFeatureReportByIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reportid: u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetFeatureReportByIdAsync: usize,
    pub CreateOutputReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateOutputReportById: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reportid: u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateFeatureReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateFeatureReportById: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reportid: u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SendOutputReportAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, outputreport: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SendOutputReportAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SendFeatureReportAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, featurereport: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SendFeatureReportAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetBooleanControlDescriptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reporttype: HidReportType, usagepage: u16, usageid: u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetBooleanControlDescriptions: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetNumericControlDescriptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reporttype: HidReportType, usagepage: u16, usageid: u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetNumericControlDescriptions: usize,
    #[cfg(feature = "Foundation")]
    pub InputReportReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reporthandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InputReportReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveInputReportReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveInputReportReceived: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHidDeviceStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHidDeviceStatics {
    type Vtable = IHidDeviceStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9e5981e4_9856_418c_9f73_77de0cd85754);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidDeviceStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usagepage: u16, usageid: u16, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetDeviceSelectorVidPid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usagepage: u16, usageid: u16, vendorid: u16, productid: u16, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, accessmode: super::super::Storage::FileAccessMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    FromIdAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHidFeatureReport(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHidFeatureReport {
    type Vtable = IHidFeatureReport_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x841d9b79_5ae5_46e3_82ef_1fec5c8942f4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidFeatureReport_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Data: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetData: usize,
    pub GetBooleanControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usagepage: u16, usageid: u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetBooleanControlByDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, controldescription: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetNumericControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usagepage: u16, usageid: u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetNumericControlByDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, controldescription: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHidInputReport(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHidInputReport {
    type Vtable = IHidInputReport_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc35d0e50_f7e7_4e8d_b23e_cabbe56b90e9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidInputReport_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Data: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ActivatedBooleanControls: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ActivatedBooleanControls: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub TransitionedBooleanControls: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TransitionedBooleanControls: usize,
    pub GetBooleanControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usagepage: u16, usageid: u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetBooleanControlByDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, controldescription: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetNumericControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usagepage: u16, usageid: u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetNumericControlByDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, controldescription: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHidInputReportReceivedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHidInputReportReceivedEventArgs {
    type Vtable = IHidInputReportReceivedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7059c5cb_59b2_4dc2_985c_0adc6136fa2d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidInputReportReceivedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Report: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHidNumericControl(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHidNumericControl {
    type Vtable = IHidNumericControl_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe38a12a5_35a7_4b75_89c8_fb1f28b10823);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidNumericControl_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub IsGrouped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub UsagePage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub UsageId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i64) -> ::windows::core::HRESULT,
    pub ScaledValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows::core::HRESULT,
    pub SetScaledValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i64) -> ::windows::core::HRESULT,
    pub ControlDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHidNumericControlDescription(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHidNumericControlDescription {
    type Vtable = IHidNumericControlDescription_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x638d5e86_1d97_4c75_927f_5ff58ba05e32);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidNumericControlDescription_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub ReportId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub ReportType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HidReportType) -> ::windows::core::HRESULT,
    pub ReportSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub ReportCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub UsagePage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub UsageId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub LogicalMinimum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub LogicalMaximum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub PhysicalMinimum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub PhysicalMaximum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub UnitExponent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Unit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub IsAbsolute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub HasNull: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ParentCollections: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ParentCollections: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHidOutputReport(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHidOutputReport {
    type Vtable = IHidOutputReport_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x62cb2544_c896_4463_93c1_df9db053c450);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidOutputReport_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Data: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetData: usize,
    pub GetBooleanControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usagepage: u16, usageid: u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetBooleanControlByDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, controldescription: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetNumericControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usagepage: u16, usageid: u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetNumericControlByDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, controldescription: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
