#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Devices_Enumeration_Pnp")]
pub mod Pnp;
#[doc = "*Required features: `Devices_Enumeration`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DeviceAccessChangedEventArgs(pub ::windows::runtime::IInspectable);
impl DeviceAccessChangedEventArgs {
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<DeviceAccessStatus> {
        let this = self;
        unsafe {
            let mut result__: DeviceAccessStatus = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DeviceAccessStatus>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IDeviceAccessChangedEventArgs2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DeviceAccessChangedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DeviceAccessChangedEventArgs;{deda0bcc-4f9d-4f58-9dba-a9bc800408d5})");
}
unsafe impl ::windows::runtime::Interface for DeviceAccessChangedEventArgs {
    type Vtable = IDeviceAccessChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xdeda0bcc_4f9d_4f58_9dba_a9bc800408d5);
}
impl ::windows::runtime::RuntimeName for DeviceAccessChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceAccessChangedEventArgs";
}
impl ::core::convert::From<DeviceAccessChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: DeviceAccessChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DeviceAccessChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &DeviceAccessChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DeviceAccessChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a DeviceAccessChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DeviceAccessChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: DeviceAccessChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DeviceAccessChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &DeviceAccessChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DeviceAccessChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DeviceAccessChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DeviceAccessChangedEventArgs {}
unsafe impl ::core::marker::Sync for DeviceAccessChangedEventArgs {}
#[doc = "*Required features: `Devices_Enumeration`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DeviceAccessInformation(pub ::windows::runtime::IInspectable);
impl DeviceAccessInformation {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`*"]
    pub fn AccessChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<DeviceAccessInformation, DeviceAccessChangedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`*"]
    pub fn RemoveAccessChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn CurrentStatus(&self) -> ::windows::runtime::Result<DeviceAccessStatus> {
        let this = self;
        unsafe {
            let mut result__: DeviceAccessStatus = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DeviceAccessStatus>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn CreateFromId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(deviceid: Param0) -> ::windows::runtime::Result<DeviceAccessInformation> {
        Self::IDeviceAccessInformationStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<DeviceAccessInformation>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn CreateFromDeviceClassId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(deviceclassid: Param0) -> ::windows::runtime::Result<DeviceAccessInformation> {
        Self::IDeviceAccessInformationStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), deviceclassid.into_param().abi(), &mut result__).from_abi::<DeviceAccessInformation>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn CreateFromDeviceClass(deviceclass: DeviceClass) -> ::windows::runtime::Result<DeviceAccessInformation> {
        Self::IDeviceAccessInformationStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), deviceclass, &mut result__).from_abi::<DeviceAccessInformation>(result__)
        })
    }
    pub fn IDeviceAccessInformationStatics<R, F: FnOnce(&IDeviceAccessInformationStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<DeviceAccessInformation, IDeviceAccessInformationStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DeviceAccessInformation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DeviceAccessInformation;{0baa9a73-6de5-4915-8ddd-9a0554a6f545})");
}
unsafe impl ::windows::runtime::Interface for DeviceAccessInformation {
    type Vtable = IDeviceAccessInformation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x0baa9a73_6de5_4915_8ddd_9a0554a6f545);
}
impl ::windows::runtime::RuntimeName for DeviceAccessInformation {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceAccessInformation";
}
impl ::core::convert::From<DeviceAccessInformation> for ::windows::runtime::IUnknown {
    fn from(value: DeviceAccessInformation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DeviceAccessInformation> for ::windows::runtime::IUnknown {
    fn from(value: &DeviceAccessInformation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DeviceAccessInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a DeviceAccessInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DeviceAccessInformation> for ::windows::runtime::IInspectable {
    fn from(value: DeviceAccessInformation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DeviceAccessInformation> for ::windows::runtime::IInspectable {
    fn from(value: &DeviceAccessInformation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DeviceAccessInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DeviceAccessInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DeviceAccessInformation {}
unsafe impl ::core::marker::Sync for DeviceAccessInformation {}
#[doc = "*Required features: `Devices_Enumeration`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DeviceAccessStatus(pub i32);
impl DeviceAccessStatus {
    pub const Unspecified: DeviceAccessStatus = DeviceAccessStatus(0i32);
    pub const Allowed: DeviceAccessStatus = DeviceAccessStatus(1i32);
    pub const DeniedByUser: DeviceAccessStatus = DeviceAccessStatus(2i32);
    pub const DeniedBySystem: DeviceAccessStatus = DeviceAccessStatus(3i32);
}
impl ::core::convert::From<i32> for DeviceAccessStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DeviceAccessStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DeviceAccessStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.Enumeration.DeviceAccessStatus;i4)");
}
impl ::windows::runtime::DefaultType for DeviceAccessStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Enumeration`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DeviceClass(pub i32);
impl DeviceClass {
    pub const All: DeviceClass = DeviceClass(0i32);
    pub const AudioCapture: DeviceClass = DeviceClass(1i32);
    pub const AudioRender: DeviceClass = DeviceClass(2i32);
    pub const PortableStorageDevice: DeviceClass = DeviceClass(3i32);
    pub const VideoCapture: DeviceClass = DeviceClass(4i32);
    pub const ImageScanner: DeviceClass = DeviceClass(5i32);
    pub const Location: DeviceClass = DeviceClass(6i32);
}
impl ::core::convert::From<i32> for DeviceClass {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DeviceClass {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DeviceClass {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.Enumeration.DeviceClass;i4)");
}
impl ::windows::runtime::DefaultType for DeviceClass {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Enumeration`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DeviceConnectionChangeTriggerDetails(pub ::windows::runtime::IInspectable);
impl DeviceConnectionChangeTriggerDetails {
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn DeviceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DeviceConnectionChangeTriggerDetails {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DeviceConnectionChangeTriggerDetails;{b8578c0c-bbc1-484b-bffa-7b31dcc200b2})");
}
unsafe impl ::windows::runtime::Interface for DeviceConnectionChangeTriggerDetails {
    type Vtable = IDeviceConnectionChangeTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xb8578c0c_bbc1_484b_bffa_7b31dcc200b2);
}
impl ::windows::runtime::RuntimeName for DeviceConnectionChangeTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceConnectionChangeTriggerDetails";
}
impl ::core::convert::From<DeviceConnectionChangeTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: DeviceConnectionChangeTriggerDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DeviceConnectionChangeTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: &DeviceConnectionChangeTriggerDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DeviceConnectionChangeTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a DeviceConnectionChangeTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DeviceConnectionChangeTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: DeviceConnectionChangeTriggerDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DeviceConnectionChangeTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: &DeviceConnectionChangeTriggerDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DeviceConnectionChangeTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DeviceConnectionChangeTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DeviceConnectionChangeTriggerDetails {}
unsafe impl ::core::marker::Sync for DeviceConnectionChangeTriggerDetails {}
#[doc = "*Required features: `Devices_Enumeration`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DeviceDisconnectButtonClickedEventArgs(pub ::windows::runtime::IInspectable);
impl DeviceDisconnectButtonClickedEventArgs {
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn Device(&self) -> ::windows::runtime::Result<DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DeviceInformation>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DeviceDisconnectButtonClickedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DeviceDisconnectButtonClickedEventArgs;{8e44b56d-f902-4a00-b536-f37992e6a2a7})");
}
unsafe impl ::windows::runtime::Interface for DeviceDisconnectButtonClickedEventArgs {
    type Vtable = IDeviceDisconnectButtonClickedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8e44b56d_f902_4a00_b536_f37992e6a2a7);
}
impl ::windows::runtime::RuntimeName for DeviceDisconnectButtonClickedEventArgs {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceDisconnectButtonClickedEventArgs";
}
impl ::core::convert::From<DeviceDisconnectButtonClickedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: DeviceDisconnectButtonClickedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DeviceDisconnectButtonClickedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &DeviceDisconnectButtonClickedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DeviceDisconnectButtonClickedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a DeviceDisconnectButtonClickedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DeviceDisconnectButtonClickedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: DeviceDisconnectButtonClickedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DeviceDisconnectButtonClickedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &DeviceDisconnectButtonClickedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DeviceDisconnectButtonClickedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DeviceDisconnectButtonClickedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DeviceDisconnectButtonClickedEventArgs {}
unsafe impl ::core::marker::Sync for DeviceDisconnectButtonClickedEventArgs {}
#[doc = "*Required features: `Devices_Enumeration`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DeviceInformation(pub ::windows::runtime::IInspectable);
impl DeviceInformation {
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn IsEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn IsDefault(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn EnclosureLocation(&self) -> ::windows::runtime::Result<EnclosureLocation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<EnclosureLocation>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation_Collections`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn Update<'a, Param0: ::windows::runtime::IntoParam<'a, DeviceInformationUpdate>>(&self, updateinfo: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), updateinfo.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`, `Storage_Streams`*"]
    pub fn GetThumbnailAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<DeviceThumbnail>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<DeviceThumbnail>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`, `Storage_Streams`*"]
    pub fn GetGlyphThumbnailAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<DeviceThumbnail>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<DeviceThumbnail>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<DeviceInformationKind> {
        let this = &::windows::runtime::Interface::cast::<IDeviceInformation2>(self)?;
        unsafe {
            let mut result__: DeviceInformationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DeviceInformationKind>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn Pairing(&self) -> ::windows::runtime::Result<DeviceInformationPairing> {
        let this = &::windows::runtime::Interface::cast::<IDeviceInformation2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DeviceInformationPairing>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`*"]
    pub fn CreateFromIdAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(deviceid: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<DeviceInformation>> {
        Self::IDeviceInformationStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<DeviceInformation>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`, `Foundation_Collections`*"]
    pub fn CreateFromIdAsyncAdditionalProperties<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>>(deviceid: Param0, additionalproperties: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<DeviceInformation>> {
        Self::IDeviceInformationStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), additionalproperties.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<DeviceInformation>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`, `Foundation_Collections`*"]
    pub fn FindAllAsync() -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<DeviceInformationCollection>> {
        Self::IDeviceInformationStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<DeviceInformationCollection>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`, `Foundation_Collections`*"]
    pub fn FindAllAsyncDeviceClass(deviceclass: DeviceClass) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<DeviceInformationCollection>> {
        Self::IDeviceInformationStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), deviceclass, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<DeviceInformationCollection>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`, `Foundation_Collections`*"]
    pub fn FindAllAsyncAqsFilter<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(aqsfilter: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<DeviceInformationCollection>> {
        Self::IDeviceInformationStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), aqsfilter.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<DeviceInformationCollection>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`, `Foundation_Collections`*"]
    pub fn FindAllAsyncAqsFilterAndAdditionalProperties<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>>(aqsfilter: Param0, additionalproperties: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<DeviceInformationCollection>> {
        Self::IDeviceInformationStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), aqsfilter.into_param().abi(), additionalproperties.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<DeviceInformationCollection>>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn CreateWatcher() -> ::windows::runtime::Result<DeviceWatcher> {
        Self::IDeviceInformationStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DeviceWatcher>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn CreateWatcherDeviceClass(deviceclass: DeviceClass) -> ::windows::runtime::Result<DeviceWatcher> {
        Self::IDeviceInformationStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), deviceclass, &mut result__).from_abi::<DeviceWatcher>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn CreateWatcherAqsFilter<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(aqsfilter: Param0) -> ::windows::runtime::Result<DeviceWatcher> {
        Self::IDeviceInformationStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), aqsfilter.into_param().abi(), &mut result__).from_abi::<DeviceWatcher>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation_Collections`*"]
    pub fn CreateWatcherAqsFilterAndAdditionalProperties<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>>(aqsfilter: Param0, additionalproperties: Param1) -> ::windows::runtime::Result<DeviceWatcher> {
        Self::IDeviceInformationStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), aqsfilter.into_param().abi(), additionalproperties.into_param().abi(), &mut result__).from_abi::<DeviceWatcher>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn GetAqsFilterFromDeviceClass(deviceclass: DeviceClass) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IDeviceInformationStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), deviceclass, &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`, `Foundation_Collections`*"]
    pub fn CreateFromIdAsyncWithKindAndAdditionalProperties<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>>(deviceid: Param0, additionalproperties: Param1, kind: DeviceInformationKind) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<DeviceInformation>> {
        Self::IDeviceInformationStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), additionalproperties.into_param().abi(), kind, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<DeviceInformation>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`, `Foundation_Collections`*"]
    pub fn FindAllAsyncWithKindAqsFilterAndAdditionalProperties<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>>(aqsfilter: Param0, additionalproperties: Param1, kind: DeviceInformationKind) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<DeviceInformationCollection>> {
        Self::IDeviceInformationStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), aqsfilter.into_param().abi(), additionalproperties.into_param().abi(), kind, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<DeviceInformationCollection>>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation_Collections`*"]
    pub fn CreateWatcherWithKindAqsFilterAndAdditionalProperties<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>>(aqsfilter: Param0, additionalproperties: Param1, kind: DeviceInformationKind) -> ::windows::runtime::Result<DeviceWatcher> {
        Self::IDeviceInformationStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), aqsfilter.into_param().abi(), additionalproperties.into_param().abi(), kind, &mut result__).from_abi::<DeviceWatcher>(result__)
        })
    }
    pub fn IDeviceInformationStatics<R, F: FnOnce(&IDeviceInformationStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<DeviceInformation, IDeviceInformationStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IDeviceInformationStatics2<R, F: FnOnce(&IDeviceInformationStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<DeviceInformation, IDeviceInformationStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DeviceInformation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DeviceInformation;{aba0fb95-4398-489d-8e44-e6130927011f})");
}
unsafe impl ::windows::runtime::Interface for DeviceInformation {
    type Vtable = IDeviceInformation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xaba0fb95_4398_489d_8e44_e6130927011f);
}
impl ::windows::runtime::RuntimeName for DeviceInformation {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceInformation";
}
impl ::core::convert::From<DeviceInformation> for ::windows::runtime::IUnknown {
    fn from(value: DeviceInformation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DeviceInformation> for ::windows::runtime::IUnknown {
    fn from(value: &DeviceInformation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DeviceInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a DeviceInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DeviceInformation> for ::windows::runtime::IInspectable {
    fn from(value: DeviceInformation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DeviceInformation> for ::windows::runtime::IInspectable {
    fn from(value: &DeviceInformation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DeviceInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DeviceInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DeviceInformation {}
unsafe impl ::core::marker::Sync for DeviceInformation {}
#[cfg(feature = "Foundation_Collections")]
#[doc = "*Required features: `Devices_Enumeration`, `Foundation_Collections`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DeviceInformationCollection(pub ::windows::runtime::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl DeviceInformationCollection {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation_Collections`*"]
    pub fn GetAt(&self, index: u32) -> ::windows::runtime::Result<DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<DeviceInformation>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation_Collections`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation_Collections`*"]
    pub fn IndexOf<'a, Param0: ::windows::runtime::IntoParam<'a, DeviceInformation>>(&self, value: Param0, index: &mut u32) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation_Collections`*"]
    pub fn GetMany(&self, startindex: u32, items: &mut [<DeviceInformation as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation_Collections`*"]
    pub fn First(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IIterator<DeviceInformation>> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::Collections::IIterable<DeviceInformation>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IIterator<DeviceInformation>>(result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::runtime::RuntimeType for DeviceInformationCollection {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DeviceInformationCollection;pinterface({bbe1fa4c-b0e3-4583-baef-1f1b2e483e56};rc(Windows.Devices.Enumeration.DeviceInformation;{aba0fb95-4398-489d-8e44-e6130927011f})))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::runtime::Interface for DeviceInformationCollection {
    type Vtable = super::super::Foundation::Collections::IVectorView_abi<DeviceInformation>;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_signature(<super::super::Foundation::Collections::IVectorView<DeviceInformation> as ::windows::runtime::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::runtime::RuntimeName for DeviceInformationCollection {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceInformationCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<DeviceInformationCollection> for ::windows::runtime::IUnknown {
    fn from(value: DeviceInformationCollection) -> Self {
        value.0 .0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&DeviceInformationCollection> for ::windows::runtime::IUnknown {
    fn from(value: &DeviceInformationCollection) -> Self {
        value.0 .0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DeviceInformationCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a DeviceInformationCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<DeviceInformationCollection> for ::windows::runtime::IInspectable {
    fn from(value: DeviceInformationCollection) -> Self {
        value.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&DeviceInformationCollection> for ::windows::runtime::IInspectable {
    fn from(value: &DeviceInformationCollection) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DeviceInformationCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DeviceInformationCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<DeviceInformationCollection> for super::super::Foundation::Collections::IVectorView<DeviceInformation> {
    fn from(value: DeviceInformationCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&DeviceInformationCollection> for super::super::Foundation::Collections::IVectorView<DeviceInformation> {
    fn from(value: &DeviceInformationCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IVectorView<DeviceInformation>> for DeviceInformationCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::Collections::IVectorView<DeviceInformation>> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IVectorView<DeviceInformation>> for &DeviceInformationCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::Collections::IVectorView<DeviceInformation>> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<DeviceInformationCollection> for super::super::Foundation::Collections::IIterable<DeviceInformation> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: DeviceInformationCollection) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&DeviceInformationCollection> for super::super::Foundation::Collections::IIterable<DeviceInformation> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &DeviceInformationCollection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<DeviceInformation>> for DeviceInformationCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::Collections::IIterable<DeviceInformation>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<DeviceInformation>> for &DeviceInformationCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::Collections::IIterable<DeviceInformation>> {
        ::core::convert::TryInto::<super::super::Foundation::Collections::IIterable<DeviceInformation>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for DeviceInformationCollection {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for DeviceInformationCollection {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for DeviceInformationCollection {
    type Item = DeviceInformation;
    type IntoIter = super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for &DeviceInformationCollection {
    type Item = DeviceInformation;
    type IntoIter = super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::Foundation::Collections::VectorViewIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[doc = "*Required features: `Devices_Enumeration`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DeviceInformationCustomPairing(pub ::windows::runtime::IInspectable);
impl DeviceInformationCustomPairing {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`*"]
    pub fn PairAsync(&self, pairingkindssupported: DevicePairingKinds) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<DevicePairingResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), pairingkindssupported, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<DevicePairingResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`*"]
    pub fn PairWithProtectionLevelAsync(&self, pairingkindssupported: DevicePairingKinds, minprotectionlevel: DevicePairingProtectionLevel) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<DevicePairingResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), pairingkindssupported, minprotectionlevel, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<DevicePairingResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`*"]
    pub fn PairWithProtectionLevelAndSettingsAsync<'a, Param2: ::windows::runtime::IntoParam<'a, IDevicePairingSettings>>(&self, pairingkindssupported: DevicePairingKinds, minprotectionlevel: DevicePairingProtectionLevel, devicepairingsettings: Param2) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<DevicePairingResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), pairingkindssupported, minprotectionlevel, devicepairingsettings.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<DevicePairingResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`*"]
    pub fn PairingRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<DeviceInformationCustomPairing, DevicePairingRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`*"]
    pub fn RemovePairingRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DeviceInformationCustomPairing {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DeviceInformationCustomPairing;{85138c02-4ee6-4914-8370-107a39144c0e})");
}
unsafe impl ::windows::runtime::Interface for DeviceInformationCustomPairing {
    type Vtable = IDeviceInformationCustomPairing_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x85138c02_4ee6_4914_8370_107a39144c0e);
}
impl ::windows::runtime::RuntimeName for DeviceInformationCustomPairing {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceInformationCustomPairing";
}
impl ::core::convert::From<DeviceInformationCustomPairing> for ::windows::runtime::IUnknown {
    fn from(value: DeviceInformationCustomPairing) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DeviceInformationCustomPairing> for ::windows::runtime::IUnknown {
    fn from(value: &DeviceInformationCustomPairing) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DeviceInformationCustomPairing {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a DeviceInformationCustomPairing {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DeviceInformationCustomPairing> for ::windows::runtime::IInspectable {
    fn from(value: DeviceInformationCustomPairing) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DeviceInformationCustomPairing> for ::windows::runtime::IInspectable {
    fn from(value: &DeviceInformationCustomPairing) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DeviceInformationCustomPairing {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DeviceInformationCustomPairing {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DeviceInformationCustomPairing {}
unsafe impl ::core::marker::Sync for DeviceInformationCustomPairing {}
#[doc = "*Required features: `Devices_Enumeration`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DeviceInformationKind(pub i32);
impl DeviceInformationKind {
    pub const Unknown: DeviceInformationKind = DeviceInformationKind(0i32);
    pub const DeviceInterface: DeviceInformationKind = DeviceInformationKind(1i32);
    pub const DeviceContainer: DeviceInformationKind = DeviceInformationKind(2i32);
    pub const Device: DeviceInformationKind = DeviceInformationKind(3i32);
    pub const DeviceInterfaceClass: DeviceInformationKind = DeviceInformationKind(4i32);
    pub const AssociationEndpoint: DeviceInformationKind = DeviceInformationKind(5i32);
    pub const AssociationEndpointContainer: DeviceInformationKind = DeviceInformationKind(6i32);
    pub const AssociationEndpointService: DeviceInformationKind = DeviceInformationKind(7i32);
    pub const DevicePanel: DeviceInformationKind = DeviceInformationKind(8i32);
}
impl ::core::convert::From<i32> for DeviceInformationKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DeviceInformationKind {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DeviceInformationKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.Enumeration.DeviceInformationKind;i4)");
}
impl ::windows::runtime::DefaultType for DeviceInformationKind {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Enumeration`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DeviceInformationPairing(pub ::windows::runtime::IInspectable);
impl DeviceInformationPairing {
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn IsPaired(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn CanPair(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`*"]
    pub fn PairAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<DevicePairingResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<DevicePairingResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`*"]
    pub fn PairWithProtectionLevelAsync(&self, minprotectionlevel: DevicePairingProtectionLevel) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<DevicePairingResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), minprotectionlevel, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<DevicePairingResult>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn ProtectionLevel(&self) -> ::windows::runtime::Result<DevicePairingProtectionLevel> {
        let this = &::windows::runtime::Interface::cast::<IDeviceInformationPairing2>(self)?;
        unsafe {
            let mut result__: DevicePairingProtectionLevel = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DevicePairingProtectionLevel>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn Custom(&self) -> ::windows::runtime::Result<DeviceInformationCustomPairing> {
        let this = &::windows::runtime::Interface::cast::<IDeviceInformationPairing2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DeviceInformationCustomPairing>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`*"]
    pub fn PairWithProtectionLevelAndSettingsAsync<'a, Param1: ::windows::runtime::IntoParam<'a, IDevicePairingSettings>>(&self, minprotectionlevel: DevicePairingProtectionLevel, devicepairingsettings: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<DevicePairingResult>> {
        let this = &::windows::runtime::Interface::cast::<IDeviceInformationPairing2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), minprotectionlevel, devicepairingsettings.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<DevicePairingResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`*"]
    pub fn UnpairAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<DeviceUnpairingResult>> {
        let this = &::windows::runtime::Interface::cast::<IDeviceInformationPairing2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<DeviceUnpairingResult>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn TryRegisterForAllInboundPairingRequests(pairingkindssupported: DevicePairingKinds) -> ::windows::runtime::Result<bool> {
        Self::IDeviceInformationPairingStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), pairingkindssupported, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn TryRegisterForAllInboundPairingRequestsWithProtectionLevel(pairingkindssupported: DevicePairingKinds, minprotectionlevel: DevicePairingProtectionLevel) -> ::windows::runtime::Result<bool> {
        Self::IDeviceInformationPairingStatics2(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), pairingkindssupported, minprotectionlevel, &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IDeviceInformationPairingStatics<R, F: FnOnce(&IDeviceInformationPairingStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<DeviceInformationPairing, IDeviceInformationPairingStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IDeviceInformationPairingStatics2<R, F: FnOnce(&IDeviceInformationPairingStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<DeviceInformationPairing, IDeviceInformationPairingStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DeviceInformationPairing {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DeviceInformationPairing;{2c4769f5-f684-40d5-8469-e8dbaab70485})");
}
unsafe impl ::windows::runtime::Interface for DeviceInformationPairing {
    type Vtable = IDeviceInformationPairing_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x2c4769f5_f684_40d5_8469_e8dbaab70485);
}
impl ::windows::runtime::RuntimeName for DeviceInformationPairing {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceInformationPairing";
}
impl ::core::convert::From<DeviceInformationPairing> for ::windows::runtime::IUnknown {
    fn from(value: DeviceInformationPairing) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DeviceInformationPairing> for ::windows::runtime::IUnknown {
    fn from(value: &DeviceInformationPairing) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DeviceInformationPairing {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a DeviceInformationPairing {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DeviceInformationPairing> for ::windows::runtime::IInspectable {
    fn from(value: DeviceInformationPairing) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DeviceInformationPairing> for ::windows::runtime::IInspectable {
    fn from(value: &DeviceInformationPairing) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DeviceInformationPairing {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DeviceInformationPairing {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DeviceInformationPairing {}
unsafe impl ::core::marker::Sync for DeviceInformationPairing {}
#[doc = "*Required features: `Devices_Enumeration`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DeviceInformationUpdate(pub ::windows::runtime::IInspectable);
impl DeviceInformationUpdate {
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation_Collections`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<DeviceInformationKind> {
        let this = &::windows::runtime::Interface::cast::<IDeviceInformationUpdate2>(self)?;
        unsafe {
            let mut result__: DeviceInformationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DeviceInformationKind>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DeviceInformationUpdate {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DeviceInformationUpdate;{8f315305-d972-44b7-a37e-9e822c78213b})");
}
unsafe impl ::windows::runtime::Interface for DeviceInformationUpdate {
    type Vtable = IDeviceInformationUpdate_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8f315305_d972_44b7_a37e_9e822c78213b);
}
impl ::windows::runtime::RuntimeName for DeviceInformationUpdate {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceInformationUpdate";
}
impl ::core::convert::From<DeviceInformationUpdate> for ::windows::runtime::IUnknown {
    fn from(value: DeviceInformationUpdate) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DeviceInformationUpdate> for ::windows::runtime::IUnknown {
    fn from(value: &DeviceInformationUpdate) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DeviceInformationUpdate {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a DeviceInformationUpdate {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DeviceInformationUpdate> for ::windows::runtime::IInspectable {
    fn from(value: DeviceInformationUpdate) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DeviceInformationUpdate> for ::windows::runtime::IInspectable {
    fn from(value: &DeviceInformationUpdate) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DeviceInformationUpdate {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DeviceInformationUpdate {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DeviceInformationUpdate {}
unsafe impl ::core::marker::Sync for DeviceInformationUpdate {}
#[doc = "*Required features: `Devices_Enumeration`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DevicePairingKinds(pub u32);
impl DevicePairingKinds {
    pub const None: DevicePairingKinds = DevicePairingKinds(0u32);
    pub const ConfirmOnly: DevicePairingKinds = DevicePairingKinds(1u32);
    pub const DisplayPin: DevicePairingKinds = DevicePairingKinds(2u32);
    pub const ProvidePin: DevicePairingKinds = DevicePairingKinds(4u32);
    pub const ConfirmPinMatch: DevicePairingKinds = DevicePairingKinds(8u32);
    pub const ProvidePasswordCredential: DevicePairingKinds = DevicePairingKinds(16u32);
}
impl ::core::convert::From<u32> for DevicePairingKinds {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DevicePairingKinds {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DevicePairingKinds {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.Enumeration.DevicePairingKinds;u4)");
}
impl ::windows::runtime::DefaultType for DevicePairingKinds {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for DevicePairingKinds {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for DevicePairingKinds {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for DevicePairingKinds {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for DevicePairingKinds {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for DevicePairingKinds {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Devices_Enumeration`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DevicePairingProtectionLevel(pub i32);
impl DevicePairingProtectionLevel {
    pub const Default: DevicePairingProtectionLevel = DevicePairingProtectionLevel(0i32);
    pub const None: DevicePairingProtectionLevel = DevicePairingProtectionLevel(1i32);
    pub const Encryption: DevicePairingProtectionLevel = DevicePairingProtectionLevel(2i32);
    pub const EncryptionAndAuthentication: DevicePairingProtectionLevel = DevicePairingProtectionLevel(3i32);
}
impl ::core::convert::From<i32> for DevicePairingProtectionLevel {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DevicePairingProtectionLevel {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DevicePairingProtectionLevel {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.Enumeration.DevicePairingProtectionLevel;i4)");
}
impl ::windows::runtime::DefaultType for DevicePairingProtectionLevel {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Enumeration`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DevicePairingRequestedEventArgs(pub ::windows::runtime::IInspectable);
impl DevicePairingRequestedEventArgs {
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn DeviceInformation(&self) -> ::windows::runtime::Result<DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DeviceInformation>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn PairingKind(&self) -> ::windows::runtime::Result<DevicePairingKinds> {
        let this = self;
        unsafe {
            let mut result__: DevicePairingKinds = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DevicePairingKinds>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn Pin(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn Accept(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn AcceptWithPin<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, pin: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), pin.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Deferral>(result__)
        }
    }
    #[cfg(feature = "Security_Credentials")]
    #[doc = "*Required features: `Devices_Enumeration`, `Security_Credentials`*"]
    pub fn AcceptWithPasswordCredential<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Security::Credentials::PasswordCredential>>(&self, passwordcredential: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IDevicePairingRequestedEventArgs2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), passwordcredential.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DevicePairingRequestedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DevicePairingRequestedEventArgs;{f717fc56-de6b-487f-8376-0180aca69963})");
}
unsafe impl ::windows::runtime::Interface for DevicePairingRequestedEventArgs {
    type Vtable = IDevicePairingRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf717fc56_de6b_487f_8376_0180aca69963);
}
impl ::windows::runtime::RuntimeName for DevicePairingRequestedEventArgs {
    const NAME: &'static str = "Windows.Devices.Enumeration.DevicePairingRequestedEventArgs";
}
impl ::core::convert::From<DevicePairingRequestedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: DevicePairingRequestedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DevicePairingRequestedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &DevicePairingRequestedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DevicePairingRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a DevicePairingRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DevicePairingRequestedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: DevicePairingRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DevicePairingRequestedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &DevicePairingRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DevicePairingRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DevicePairingRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DevicePairingRequestedEventArgs {}
unsafe impl ::core::marker::Sync for DevicePairingRequestedEventArgs {}
#[doc = "*Required features: `Devices_Enumeration`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DevicePairingResult(pub ::windows::runtime::IInspectable);
impl DevicePairingResult {
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<DevicePairingResultStatus> {
        let this = self;
        unsafe {
            let mut result__: DevicePairingResultStatus = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DevicePairingResultStatus>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn ProtectionLevelUsed(&self) -> ::windows::runtime::Result<DevicePairingProtectionLevel> {
        let this = self;
        unsafe {
            let mut result__: DevicePairingProtectionLevel = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DevicePairingProtectionLevel>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DevicePairingResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DevicePairingResult;{072b02bf-dd95-4025-9b37-de51adba37b7})");
}
unsafe impl ::windows::runtime::Interface for DevicePairingResult {
    type Vtable = IDevicePairingResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x072b02bf_dd95_4025_9b37_de51adba37b7);
}
impl ::windows::runtime::RuntimeName for DevicePairingResult {
    const NAME: &'static str = "Windows.Devices.Enumeration.DevicePairingResult";
}
impl ::core::convert::From<DevicePairingResult> for ::windows::runtime::IUnknown {
    fn from(value: DevicePairingResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DevicePairingResult> for ::windows::runtime::IUnknown {
    fn from(value: &DevicePairingResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DevicePairingResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a DevicePairingResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DevicePairingResult> for ::windows::runtime::IInspectable {
    fn from(value: DevicePairingResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DevicePairingResult> for ::windows::runtime::IInspectable {
    fn from(value: &DevicePairingResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DevicePairingResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DevicePairingResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DevicePairingResult {}
unsafe impl ::core::marker::Sync for DevicePairingResult {}
#[doc = "*Required features: `Devices_Enumeration`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DevicePairingResultStatus(pub i32);
impl DevicePairingResultStatus {
    pub const Paired: DevicePairingResultStatus = DevicePairingResultStatus(0i32);
    pub const NotReadyToPair: DevicePairingResultStatus = DevicePairingResultStatus(1i32);
    pub const NotPaired: DevicePairingResultStatus = DevicePairingResultStatus(2i32);
    pub const AlreadyPaired: DevicePairingResultStatus = DevicePairingResultStatus(3i32);
    pub const ConnectionRejected: DevicePairingResultStatus = DevicePairingResultStatus(4i32);
    pub const TooManyConnections: DevicePairingResultStatus = DevicePairingResultStatus(5i32);
    pub const HardwareFailure: DevicePairingResultStatus = DevicePairingResultStatus(6i32);
    pub const AuthenticationTimeout: DevicePairingResultStatus = DevicePairingResultStatus(7i32);
    pub const AuthenticationNotAllowed: DevicePairingResultStatus = DevicePairingResultStatus(8i32);
    pub const AuthenticationFailure: DevicePairingResultStatus = DevicePairingResultStatus(9i32);
    pub const NoSupportedProfiles: DevicePairingResultStatus = DevicePairingResultStatus(10i32);
    pub const ProtectionLevelCouldNotBeMet: DevicePairingResultStatus = DevicePairingResultStatus(11i32);
    pub const AccessDenied: DevicePairingResultStatus = DevicePairingResultStatus(12i32);
    pub const InvalidCeremonyData: DevicePairingResultStatus = DevicePairingResultStatus(13i32);
    pub const PairingCanceled: DevicePairingResultStatus = DevicePairingResultStatus(14i32);
    pub const OperationAlreadyInProgress: DevicePairingResultStatus = DevicePairingResultStatus(15i32);
    pub const RequiredHandlerNotRegistered: DevicePairingResultStatus = DevicePairingResultStatus(16i32);
    pub const RejectedByHandler: DevicePairingResultStatus = DevicePairingResultStatus(17i32);
    pub const RemoteDeviceHasAssociation: DevicePairingResultStatus = DevicePairingResultStatus(18i32);
    pub const Failed: DevicePairingResultStatus = DevicePairingResultStatus(19i32);
}
impl ::core::convert::From<i32> for DevicePairingResultStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DevicePairingResultStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DevicePairingResultStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.Enumeration.DevicePairingResultStatus;i4)");
}
impl ::windows::runtime::DefaultType for DevicePairingResultStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Enumeration`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DevicePicker(pub ::windows::runtime::IInspectable);
impl DevicePicker {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<DevicePicker, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn Filter(&self) -> ::windows::runtime::Result<DevicePickerFilter> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DevicePickerFilter>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn Appearance(&self) -> ::windows::runtime::Result<DevicePickerAppearance> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DevicePickerAppearance>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation_Collections`*"]
    pub fn RequestedProperties(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`*"]
    pub fn DeviceSelected<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<DevicePicker, DeviceSelectedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`*"]
    pub fn RemoveDeviceSelected<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`*"]
    pub fn DisconnectButtonClicked<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<DevicePicker, DeviceDisconnectButtonClickedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`*"]
    pub fn RemoveDisconnectButtonClicked<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`*"]
    pub fn DevicePickerDismissed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<DevicePicker, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`*"]
    pub fn RemoveDevicePickerDismissed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`*"]
    pub fn Show<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Rect>>(&self, selection: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), selection.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`, `UI_Popups`*"]
    pub fn ShowWithPlacement<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Rect>>(&self, selection: Param0, placement: super::super::UI::Popups::Placement) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), selection.into_param().abi(), placement).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`*"]
    pub fn PickSingleDeviceAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Rect>>(&self, selection: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<DeviceInformation>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), selection.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<DeviceInformation>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`, `UI_Popups`*"]
    pub fn PickSingleDeviceAsyncWithPlacement<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Rect>>(&self, selection: Param0, placement: super::super::UI::Popups::Placement) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<DeviceInformation>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::core::mem::transmute_copy(this), selection.into_param().abi(), placement, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<DeviceInformation>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn Hide(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).19)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn SetDisplayStatus<'a, Param0: ::windows::runtime::IntoParam<'a, DeviceInformation>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, device: Param0, status: Param1, options: DevicePickerDisplayStatusOptions) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).20)(::core::mem::transmute_copy(this), device.into_param().abi(), status.into_param().abi(), options).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DevicePicker {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DevicePicker;{84997aa2-034a-4440-8813-7d0bd479bf5a})");
}
unsafe impl ::windows::runtime::Interface for DevicePicker {
    type Vtable = IDevicePicker_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x84997aa2_034a_4440_8813_7d0bd479bf5a);
}
impl ::windows::runtime::RuntimeName for DevicePicker {
    const NAME: &'static str = "Windows.Devices.Enumeration.DevicePicker";
}
impl ::core::convert::From<DevicePicker> for ::windows::runtime::IUnknown {
    fn from(value: DevicePicker) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DevicePicker> for ::windows::runtime::IUnknown {
    fn from(value: &DevicePicker) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DevicePicker {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a DevicePicker {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DevicePicker> for ::windows::runtime::IInspectable {
    fn from(value: DevicePicker) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DevicePicker> for ::windows::runtime::IInspectable {
    fn from(value: &DevicePicker) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DevicePicker {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DevicePicker {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DevicePicker {}
unsafe impl ::core::marker::Sync for DevicePicker {}
#[doc = "*Required features: `Devices_Enumeration`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DevicePickerAppearance(pub ::windows::runtime::IInspectable);
impl DevicePickerAppearance {
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn Title(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn SetTitle<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI")]
    #[doc = "*Required features: `Devices_Enumeration`, `UI`*"]
    pub fn ForegroundColor(&self) -> ::windows::runtime::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::UI::Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Color>(result__)
        }
    }
    #[cfg(feature = "UI")]
    #[doc = "*Required features: `Devices_Enumeration`, `UI`*"]
    pub fn SetForegroundColor<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::UI::Color>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI")]
    #[doc = "*Required features: `Devices_Enumeration`, `UI`*"]
    pub fn BackgroundColor(&self) -> ::windows::runtime::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::UI::Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Color>(result__)
        }
    }
    #[cfg(feature = "UI")]
    #[doc = "*Required features: `Devices_Enumeration`, `UI`*"]
    pub fn SetBackgroundColor<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::UI::Color>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI")]
    #[doc = "*Required features: `Devices_Enumeration`, `UI`*"]
    pub fn AccentColor(&self) -> ::windows::runtime::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::UI::Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Color>(result__)
        }
    }
    #[cfg(feature = "UI")]
    #[doc = "*Required features: `Devices_Enumeration`, `UI`*"]
    pub fn SetAccentColor<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::UI::Color>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI")]
    #[doc = "*Required features: `Devices_Enumeration`, `UI`*"]
    pub fn SelectedForegroundColor(&self) -> ::windows::runtime::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::UI::Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Color>(result__)
        }
    }
    #[cfg(feature = "UI")]
    #[doc = "*Required features: `Devices_Enumeration`, `UI`*"]
    pub fn SetSelectedForegroundColor<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::UI::Color>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI")]
    #[doc = "*Required features: `Devices_Enumeration`, `UI`*"]
    pub fn SelectedBackgroundColor(&self) -> ::windows::runtime::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::UI::Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Color>(result__)
        }
    }
    #[cfg(feature = "UI")]
    #[doc = "*Required features: `Devices_Enumeration`, `UI`*"]
    pub fn SetSelectedBackgroundColor<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::UI::Color>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI")]
    #[doc = "*Required features: `Devices_Enumeration`, `UI`*"]
    pub fn SelectedAccentColor(&self) -> ::windows::runtime::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::UI::Color = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Color>(result__)
        }
    }
    #[cfg(feature = "UI")]
    #[doc = "*Required features: `Devices_Enumeration`, `UI`*"]
    pub fn SetSelectedAccentColor<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::UI::Color>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).19)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DevicePickerAppearance {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DevicePickerAppearance;{e69a12c6-e627-4ed8-9b6c-460af445e56d})");
}
unsafe impl ::windows::runtime::Interface for DevicePickerAppearance {
    type Vtable = IDevicePickerAppearance_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xe69a12c6_e627_4ed8_9b6c_460af445e56d);
}
impl ::windows::runtime::RuntimeName for DevicePickerAppearance {
    const NAME: &'static str = "Windows.Devices.Enumeration.DevicePickerAppearance";
}
impl ::core::convert::From<DevicePickerAppearance> for ::windows::runtime::IUnknown {
    fn from(value: DevicePickerAppearance) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DevicePickerAppearance> for ::windows::runtime::IUnknown {
    fn from(value: &DevicePickerAppearance) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DevicePickerAppearance {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a DevicePickerAppearance {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DevicePickerAppearance> for ::windows::runtime::IInspectable {
    fn from(value: DevicePickerAppearance) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DevicePickerAppearance> for ::windows::runtime::IInspectable {
    fn from(value: &DevicePickerAppearance) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DevicePickerAppearance {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DevicePickerAppearance {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DevicePickerAppearance {}
unsafe impl ::core::marker::Sync for DevicePickerAppearance {}
#[doc = "*Required features: `Devices_Enumeration`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DevicePickerDisplayStatusOptions(pub u32);
impl DevicePickerDisplayStatusOptions {
    pub const None: DevicePickerDisplayStatusOptions = DevicePickerDisplayStatusOptions(0u32);
    pub const ShowProgress: DevicePickerDisplayStatusOptions = DevicePickerDisplayStatusOptions(1u32);
    pub const ShowDisconnectButton: DevicePickerDisplayStatusOptions = DevicePickerDisplayStatusOptions(2u32);
    pub const ShowRetryButton: DevicePickerDisplayStatusOptions = DevicePickerDisplayStatusOptions(4u32);
}
impl ::core::convert::From<u32> for DevicePickerDisplayStatusOptions {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DevicePickerDisplayStatusOptions {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DevicePickerDisplayStatusOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.Enumeration.DevicePickerDisplayStatusOptions;u4)");
}
impl ::windows::runtime::DefaultType for DevicePickerDisplayStatusOptions {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for DevicePickerDisplayStatusOptions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for DevicePickerDisplayStatusOptions {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for DevicePickerDisplayStatusOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for DevicePickerDisplayStatusOptions {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for DevicePickerDisplayStatusOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Devices_Enumeration`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DevicePickerFilter(pub ::windows::runtime::IInspectable);
impl DevicePickerFilter {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation_Collections`*"]
    pub fn SupportedDeviceClasses(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<DeviceClass>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<DeviceClass>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation_Collections`*"]
    pub fn SupportedDeviceSelectors(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DevicePickerFilter {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DevicePickerFilter;{91db92a2-57cb-48f1-9b59-a59b7a1f02a2})");
}
unsafe impl ::windows::runtime::Interface for DevicePickerFilter {
    type Vtable = IDevicePickerFilter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x91db92a2_57cb_48f1_9b59_a59b7a1f02a2);
}
impl ::windows::runtime::RuntimeName for DevicePickerFilter {
    const NAME: &'static str = "Windows.Devices.Enumeration.DevicePickerFilter";
}
impl ::core::convert::From<DevicePickerFilter> for ::windows::runtime::IUnknown {
    fn from(value: DevicePickerFilter) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DevicePickerFilter> for ::windows::runtime::IUnknown {
    fn from(value: &DevicePickerFilter) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DevicePickerFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a DevicePickerFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DevicePickerFilter> for ::windows::runtime::IInspectable {
    fn from(value: DevicePickerFilter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DevicePickerFilter> for ::windows::runtime::IInspectable {
    fn from(value: &DevicePickerFilter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DevicePickerFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DevicePickerFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DevicePickerFilter {}
unsafe impl ::core::marker::Sync for DevicePickerFilter {}
#[doc = "*Required features: `Devices_Enumeration`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DeviceSelectedEventArgs(pub ::windows::runtime::IInspectable);
impl DeviceSelectedEventArgs {
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn SelectedDevice(&self) -> ::windows::runtime::Result<DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DeviceInformation>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DeviceSelectedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DeviceSelectedEventArgs;{269edade-1d2f-4940-8402-4156b81d3c77})");
}
unsafe impl ::windows::runtime::Interface for DeviceSelectedEventArgs {
    type Vtable = IDeviceSelectedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x269edade_1d2f_4940_8402_4156b81d3c77);
}
impl ::windows::runtime::RuntimeName for DeviceSelectedEventArgs {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceSelectedEventArgs";
}
impl ::core::convert::From<DeviceSelectedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: DeviceSelectedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DeviceSelectedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &DeviceSelectedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DeviceSelectedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a DeviceSelectedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DeviceSelectedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: DeviceSelectedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DeviceSelectedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &DeviceSelectedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DeviceSelectedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DeviceSelectedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DeviceSelectedEventArgs {}
unsafe impl ::core::marker::Sync for DeviceSelectedEventArgs {}
#[cfg(feature = "Storage_Streams")]
#[doc = "*Required features: `Devices_Enumeration`, `Storage_Streams`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DeviceThumbnail(pub ::windows::runtime::IInspectable);
#[cfg(feature = "Storage_Streams")]
impl DeviceThumbnail {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Enumeration`, `Storage_Streams`*"]
    pub fn ContentType(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::super::Storage::Streams::IContentTypeProvider>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`, `Storage_Streams`*"]
    pub fn ReadAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, buffer: Param0, count: u32, options: super::super::Storage::Streams::InputStreamOptions) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IBuffer, u32>> {
        let this = &::windows::runtime::Interface::cast::<super::super::Storage::Streams::IInputStream>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), buffer.into_param().abi(), count, options, &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IBuffer, u32>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`, `Storage_Streams`*"]
    pub fn WriteAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, buffer: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>> {
        let this = &::windows::runtime::Interface::cast::<super::super::Storage::Streams::IOutputStream>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), buffer.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`, `Storage_Streams`*"]
    pub fn FlushAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::runtime::Interface::cast::<super::super::Storage::Streams::IOutputStream>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Enumeration`, `Storage_Streams`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<u64> {
        let this = &::windows::runtime::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Enumeration`, `Storage_Streams`*"]
    pub fn SetSize(&self, value: u64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Enumeration`, `Storage_Streams`*"]
    pub fn GetInputStreamAt(&self, position: u64) -> ::windows::runtime::Result<super::super::Storage::Streams::IInputStream> {
        let this = &::windows::runtime::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), position, &mut result__).from_abi::<super::super::Storage::Streams::IInputStream>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Enumeration`, `Storage_Streams`*"]
    pub fn GetOutputStreamAt(&self, position: u64) -> ::windows::runtime::Result<super::super::Storage::Streams::IOutputStream> {
        let this = &::windows::runtime::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), position, &mut result__).from_abi::<super::super::Storage::Streams::IOutputStream>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Enumeration`, `Storage_Streams`*"]
    pub fn Position(&self) -> ::windows::runtime::Result<u64> {
        let this = &::windows::runtime::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Enumeration`, `Storage_Streams`*"]
    pub fn Seek(&self, position: u64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), position).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Enumeration`, `Storage_Streams`*"]
    pub fn CloneStream(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IRandomAccessStream> {
        let this = &::windows::runtime::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IRandomAccessStream>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Enumeration`, `Storage_Streams`*"]
    pub fn CanRead(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Enumeration`, `Storage_Streams`*"]
    pub fn CanWrite(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
#[cfg(feature = "Storage_Streams")]
unsafe impl ::windows::runtime::RuntimeType for DeviceThumbnail {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DeviceThumbnail;{cc254827-4b3d-438f-9232-10c76bc7e038})");
}
#[cfg(feature = "Storage_Streams")]
unsafe impl ::windows::runtime::Interface for DeviceThumbnail {
    type Vtable = super::super::Storage::Streams::IRandomAccessStreamWithContentType_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xcc254827_4b3d_438f_9232_10c76bc7e038);
}
#[cfg(feature = "Storage_Streams")]
impl ::windows::runtime::RuntimeName for DeviceThumbnail {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceThumbnail";
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::From<DeviceThumbnail> for ::windows::runtime::IUnknown {
    fn from(value: DeviceThumbnail) -> Self {
        value.0 .0
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::From<&DeviceThumbnail> for ::windows::runtime::IUnknown {
    fn from(value: &DeviceThumbnail) -> Self {
        value.0 .0.clone()
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DeviceThumbnail {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a DeviceThumbnail {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::From<DeviceThumbnail> for ::windows::runtime::IInspectable {
    fn from(value: DeviceThumbnail) -> Self {
        value.0
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::From<&DeviceThumbnail> for ::windows::runtime::IInspectable {
    fn from(value: &DeviceThumbnail) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DeviceThumbnail {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DeviceThumbnail {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::From<DeviceThumbnail> for super::super::Storage::Streams::IRandomAccessStreamWithContentType {
    fn from(value: DeviceThumbnail) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::From<&DeviceThumbnail> for super::super::Storage::Streams::IRandomAccessStreamWithContentType {
    fn from(value: &DeviceThumbnail) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStreamWithContentType> for DeviceThumbnail {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Storage::Streams::IRandomAccessStreamWithContentType> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStreamWithContentType> for &DeviceThumbnail {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Storage::Streams::IRandomAccessStreamWithContentType> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl ::core::convert::TryFrom<DeviceThumbnail> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: DeviceThumbnail) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl ::core::convert::TryFrom<&DeviceThumbnail> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &DeviceThumbnail) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for DeviceThumbnail {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &DeviceThumbnail {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<DeviceThumbnail> for super::super::Storage::Streams::IContentTypeProvider {
    type Error = ::windows::runtime::Error;
    fn try_from(value: DeviceThumbnail) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&DeviceThumbnail> for super::super::Storage::Streams::IContentTypeProvider {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &DeviceThumbnail) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IContentTypeProvider> for DeviceThumbnail {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Storage::Streams::IContentTypeProvider> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IContentTypeProvider> for &DeviceThumbnail {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Storage::Streams::IContentTypeProvider> {
        ::core::convert::TryInto::<super::super::Storage::Streams::IContentTypeProvider>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<DeviceThumbnail> for super::super::Storage::Streams::IInputStream {
    type Error = ::windows::runtime::Error;
    fn try_from(value: DeviceThumbnail) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&DeviceThumbnail> for super::super::Storage::Streams::IInputStream {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &DeviceThumbnail) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IInputStream> for DeviceThumbnail {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Storage::Streams::IInputStream> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IInputStream> for &DeviceThumbnail {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Storage::Streams::IInputStream> {
        ::core::convert::TryInto::<super::super::Storage::Streams::IInputStream>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<DeviceThumbnail> for super::super::Storage::Streams::IOutputStream {
    type Error = ::windows::runtime::Error;
    fn try_from(value: DeviceThumbnail) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&DeviceThumbnail> for super::super::Storage::Streams::IOutputStream {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &DeviceThumbnail) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IOutputStream> for DeviceThumbnail {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Storage::Streams::IOutputStream> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IOutputStream> for &DeviceThumbnail {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Storage::Streams::IOutputStream> {
        ::core::convert::TryInto::<super::super::Storage::Streams::IOutputStream>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<DeviceThumbnail> for super::super::Storage::Streams::IRandomAccessStream {
    type Error = ::windows::runtime::Error;
    fn try_from(value: DeviceThumbnail) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&DeviceThumbnail> for super::super::Storage::Streams::IRandomAccessStream {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &DeviceThumbnail) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream> for DeviceThumbnail {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Storage::Streams::IRandomAccessStream> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream> for &DeviceThumbnail {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Storage::Streams::IRandomAccessStream> {
        ::core::convert::TryInto::<super::super::Storage::Streams::IRandomAccessStream>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Storage_Streams")]
unsafe impl ::core::marker::Send for DeviceThumbnail {}
#[cfg(feature = "Storage_Streams")]
unsafe impl ::core::marker::Sync for DeviceThumbnail {}
#[doc = "*Required features: `Devices_Enumeration`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DeviceUnpairingResult(pub ::windows::runtime::IInspectable);
impl DeviceUnpairingResult {
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<DeviceUnpairingResultStatus> {
        let this = self;
        unsafe {
            let mut result__: DeviceUnpairingResultStatus = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DeviceUnpairingResultStatus>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DeviceUnpairingResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DeviceUnpairingResult;{66f44ad3-79d9-444b-92cf-a92ef72571c7})");
}
unsafe impl ::windows::runtime::Interface for DeviceUnpairingResult {
    type Vtable = IDeviceUnpairingResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x66f44ad3_79d9_444b_92cf_a92ef72571c7);
}
impl ::windows::runtime::RuntimeName for DeviceUnpairingResult {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceUnpairingResult";
}
impl ::core::convert::From<DeviceUnpairingResult> for ::windows::runtime::IUnknown {
    fn from(value: DeviceUnpairingResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DeviceUnpairingResult> for ::windows::runtime::IUnknown {
    fn from(value: &DeviceUnpairingResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DeviceUnpairingResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a DeviceUnpairingResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DeviceUnpairingResult> for ::windows::runtime::IInspectable {
    fn from(value: DeviceUnpairingResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DeviceUnpairingResult> for ::windows::runtime::IInspectable {
    fn from(value: &DeviceUnpairingResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DeviceUnpairingResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DeviceUnpairingResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DeviceUnpairingResult {}
unsafe impl ::core::marker::Sync for DeviceUnpairingResult {}
#[doc = "*Required features: `Devices_Enumeration`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DeviceUnpairingResultStatus(pub i32);
impl DeviceUnpairingResultStatus {
    pub const Unpaired: DeviceUnpairingResultStatus = DeviceUnpairingResultStatus(0i32);
    pub const AlreadyUnpaired: DeviceUnpairingResultStatus = DeviceUnpairingResultStatus(1i32);
    pub const OperationAlreadyInProgress: DeviceUnpairingResultStatus = DeviceUnpairingResultStatus(2i32);
    pub const AccessDenied: DeviceUnpairingResultStatus = DeviceUnpairingResultStatus(3i32);
    pub const Failed: DeviceUnpairingResultStatus = DeviceUnpairingResultStatus(4i32);
}
impl ::core::convert::From<i32> for DeviceUnpairingResultStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DeviceUnpairingResultStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DeviceUnpairingResultStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.Enumeration.DeviceUnpairingResultStatus;i4)");
}
impl ::windows::runtime::DefaultType for DeviceUnpairingResultStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Enumeration`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DeviceWatcher(pub ::windows::runtime::IInspectable);
impl DeviceWatcher {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`*"]
    pub fn Added<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<DeviceWatcher, DeviceInformation>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`*"]
    pub fn RemoveAdded<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`*"]
    pub fn Updated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<DeviceWatcher, DeviceInformationUpdate>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`*"]
    pub fn RemoveUpdated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`*"]
    pub fn Removed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<DeviceWatcher, DeviceInformationUpdate>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`*"]
    pub fn RemoveRemoved<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`*"]
    pub fn EnumerationCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<DeviceWatcher, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`*"]
    pub fn RemoveEnumerationCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`*"]
    pub fn Stopped<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<DeviceWatcher, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`*"]
    pub fn RemoveStopped<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<DeviceWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__: DeviceWatcherStatus = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DeviceWatcherStatus>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn Start(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn Stop(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).18)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(all(feature = "ApplicationModel_Background", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_Enumeration`, `ApplicationModel_Background`, `Foundation_Collections`*"]
    pub fn GetBackgroundTrigger<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<DeviceWatcherEventKind>>>(&self, requestedeventkinds: Param0) -> ::windows::runtime::Result<super::super::ApplicationModel::Background::DeviceWatcherTrigger> {
        let this = &::windows::runtime::Interface::cast::<IDeviceWatcher2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), requestedeventkinds.into_param().abi(), &mut result__).from_abi::<super::super::ApplicationModel::Background::DeviceWatcherTrigger>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DeviceWatcher {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DeviceWatcher;{c9eab97d-8f6b-4f96-a9f4-abc814e22271})");
}
unsafe impl ::windows::runtime::Interface for DeviceWatcher {
    type Vtable = IDeviceWatcher_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc9eab97d_8f6b_4f96_a9f4_abc814e22271);
}
impl ::windows::runtime::RuntimeName for DeviceWatcher {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceWatcher";
}
impl ::core::convert::From<DeviceWatcher> for ::windows::runtime::IUnknown {
    fn from(value: DeviceWatcher) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DeviceWatcher> for ::windows::runtime::IUnknown {
    fn from(value: &DeviceWatcher) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DeviceWatcher {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a DeviceWatcher {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DeviceWatcher> for ::windows::runtime::IInspectable {
    fn from(value: DeviceWatcher) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DeviceWatcher> for ::windows::runtime::IInspectable {
    fn from(value: &DeviceWatcher) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DeviceWatcher {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DeviceWatcher {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DeviceWatcher {}
unsafe impl ::core::marker::Sync for DeviceWatcher {}
#[doc = "*Required features: `Devices_Enumeration`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DeviceWatcherEvent(pub ::windows::runtime::IInspectable);
impl DeviceWatcherEvent {
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<DeviceWatcherEventKind> {
        let this = self;
        unsafe {
            let mut result__: DeviceWatcherEventKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DeviceWatcherEventKind>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn DeviceInformation(&self) -> ::windows::runtime::Result<DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DeviceInformation>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn DeviceInformationUpdate(&self) -> ::windows::runtime::Result<DeviceInformationUpdate> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DeviceInformationUpdate>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DeviceWatcherEvent {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DeviceWatcherEvent;{74aa9c0b-1dbd-47fd-b635-3cc556d0ff8b})");
}
unsafe impl ::windows::runtime::Interface for DeviceWatcherEvent {
    type Vtable = IDeviceWatcherEvent_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x74aa9c0b_1dbd_47fd_b635_3cc556d0ff8b);
}
impl ::windows::runtime::RuntimeName for DeviceWatcherEvent {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceWatcherEvent";
}
impl ::core::convert::From<DeviceWatcherEvent> for ::windows::runtime::IUnknown {
    fn from(value: DeviceWatcherEvent) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DeviceWatcherEvent> for ::windows::runtime::IUnknown {
    fn from(value: &DeviceWatcherEvent) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DeviceWatcherEvent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a DeviceWatcherEvent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DeviceWatcherEvent> for ::windows::runtime::IInspectable {
    fn from(value: DeviceWatcherEvent) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DeviceWatcherEvent> for ::windows::runtime::IInspectable {
    fn from(value: &DeviceWatcherEvent) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DeviceWatcherEvent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DeviceWatcherEvent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DeviceWatcherEvent {}
unsafe impl ::core::marker::Sync for DeviceWatcherEvent {}
#[doc = "*Required features: `Devices_Enumeration`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DeviceWatcherEventKind(pub i32);
impl DeviceWatcherEventKind {
    pub const Add: DeviceWatcherEventKind = DeviceWatcherEventKind(0i32);
    pub const Update: DeviceWatcherEventKind = DeviceWatcherEventKind(1i32);
    pub const Remove: DeviceWatcherEventKind = DeviceWatcherEventKind(2i32);
}
impl ::core::convert::From<i32> for DeviceWatcherEventKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DeviceWatcherEventKind {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DeviceWatcherEventKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.Enumeration.DeviceWatcherEventKind;i4)");
}
impl ::windows::runtime::DefaultType for DeviceWatcherEventKind {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Enumeration`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DeviceWatcherStatus(pub i32);
impl DeviceWatcherStatus {
    pub const Created: DeviceWatcherStatus = DeviceWatcherStatus(0i32);
    pub const Started: DeviceWatcherStatus = DeviceWatcherStatus(1i32);
    pub const EnumerationCompleted: DeviceWatcherStatus = DeviceWatcherStatus(2i32);
    pub const Stopping: DeviceWatcherStatus = DeviceWatcherStatus(3i32);
    pub const Stopped: DeviceWatcherStatus = DeviceWatcherStatus(4i32);
    pub const Aborted: DeviceWatcherStatus = DeviceWatcherStatus(5i32);
}
impl ::core::convert::From<i32> for DeviceWatcherStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DeviceWatcherStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DeviceWatcherStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.Enumeration.DeviceWatcherStatus;i4)");
}
impl ::windows::runtime::DefaultType for DeviceWatcherStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Enumeration`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DeviceWatcherTriggerDetails(pub ::windows::runtime::IInspectable);
impl DeviceWatcherTriggerDetails {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation_Collections`*"]
    pub fn DeviceWatcherEvents(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<DeviceWatcherEvent>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<DeviceWatcherEvent>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DeviceWatcherTriggerDetails {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DeviceWatcherTriggerDetails;{38808119-4cb7-4e57-a56d-776d07cbfef9})");
}
unsafe impl ::windows::runtime::Interface for DeviceWatcherTriggerDetails {
    type Vtable = IDeviceWatcherTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x38808119_4cb7_4e57_a56d_776d07cbfef9);
}
impl ::windows::runtime::RuntimeName for DeviceWatcherTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceWatcherTriggerDetails";
}
impl ::core::convert::From<DeviceWatcherTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: DeviceWatcherTriggerDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DeviceWatcherTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: &DeviceWatcherTriggerDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DeviceWatcherTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a DeviceWatcherTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DeviceWatcherTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: DeviceWatcherTriggerDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DeviceWatcherTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: &DeviceWatcherTriggerDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DeviceWatcherTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DeviceWatcherTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DeviceWatcherTriggerDetails {}
unsafe impl ::core::marker::Sync for DeviceWatcherTriggerDetails {}
#[doc = "*Required features: `Devices_Enumeration`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct EnclosureLocation(pub ::windows::runtime::IInspectable);
impl EnclosureLocation {
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn InDock(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn InLid(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn Panel(&self) -> ::windows::runtime::Result<Panel> {
        let this = self;
        unsafe {
            let mut result__: Panel = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Panel>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn RotationAngleInDegreesClockwise(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IEnclosureLocation2>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for EnclosureLocation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.EnclosureLocation;{42340a27-5810-459c-aabb-c65e1f813ecf})");
}
unsafe impl ::windows::runtime::Interface for EnclosureLocation {
    type Vtable = IEnclosureLocation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x42340a27_5810_459c_aabb_c65e1f813ecf);
}
impl ::windows::runtime::RuntimeName for EnclosureLocation {
    const NAME: &'static str = "Windows.Devices.Enumeration.EnclosureLocation";
}
impl ::core::convert::From<EnclosureLocation> for ::windows::runtime::IUnknown {
    fn from(value: EnclosureLocation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&EnclosureLocation> for ::windows::runtime::IUnknown {
    fn from(value: &EnclosureLocation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for EnclosureLocation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a EnclosureLocation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<EnclosureLocation> for ::windows::runtime::IInspectable {
    fn from(value: EnclosureLocation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&EnclosureLocation> for ::windows::runtime::IInspectable {
    fn from(value: &EnclosureLocation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for EnclosureLocation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a EnclosureLocation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for EnclosureLocation {}
unsafe impl ::core::marker::Sync for EnclosureLocation {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IDeviceAccessChangedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDeviceAccessChangedEventArgs {
    type Vtable = IDeviceAccessChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xdeda0bcc_4f9d_4f58_9dba_a9bc800408d5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceAccessChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut DeviceAccessStatus) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDeviceAccessChangedEventArgs2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDeviceAccessChangedEventArgs2 {
    type Vtable = IDeviceAccessChangedEventArgs2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x82523262_934b_4b30_a178_adc39f2f2be3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceAccessChangedEventArgs2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDeviceAccessInformation(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDeviceAccessInformation {
    type Vtable = IDeviceAccessInformation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x0baa9a73_6de5_4915_8ddd_9a0554a6f545);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceAccessInformation_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut DeviceAccessStatus) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDeviceAccessInformationStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDeviceAccessInformationStatics {
    type Vtable = IDeviceAccessInformationStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x574bd3d3_5f30_45cd_8a94_724fe5973084);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceAccessInformationStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deviceid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deviceclassid: ::windows::runtime::GUID, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deviceclass: DeviceClass, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDeviceConnectionChangeTriggerDetails(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDeviceConnectionChangeTriggerDetails {
    type Vtable = IDeviceConnectionChangeTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xb8578c0c_bbc1_484b_bffa_7b31dcc200b2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceConnectionChangeTriggerDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDeviceDisconnectButtonClickedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDeviceDisconnectButtonClickedEventArgs {
    type Vtable = IDeviceDisconnectButtonClickedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8e44b56d_f902_4a00_b536_f37992e6a2a7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceDisconnectButtonClickedEventArgs_abi(
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
pub struct IDeviceInformation(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDeviceInformation {
    type Vtable = IDeviceInformation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xaba0fb95_4398_489d_8e44_e6130927011f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceInformation_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, updateinfo: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDeviceInformation2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDeviceInformation2 {
    type Vtable = IDeviceInformation2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf156a638_7997_48d9_a10c_269d46533f48);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceInformation2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut DeviceInformationKind) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDeviceInformationCustomPairing(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDeviceInformationCustomPairing {
    type Vtable = IDeviceInformationCustomPairing_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x85138c02_4ee6_4914_8370_107a39144c0e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceInformationCustomPairing_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pairingkindssupported: DevicePairingKinds, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pairingkindssupported: DevicePairingKinds, minprotectionlevel: DevicePairingProtectionLevel, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pairingkindssupported: DevicePairingKinds, minprotectionlevel: DevicePairingProtectionLevel, devicepairingsettings: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDeviceInformationPairing(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDeviceInformationPairing {
    type Vtable = IDeviceInformationPairing_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x2c4769f5_f684_40d5_8469_e8dbaab70485);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceInformationPairing_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, minprotectionlevel: DevicePairingProtectionLevel, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDeviceInformationPairing2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDeviceInformationPairing2 {
    type Vtable = IDeviceInformationPairing2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf68612fd_0aee_4328_85cc_1c742bb1790d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceInformationPairing2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut DevicePairingProtectionLevel) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, minprotectionlevel: DevicePairingProtectionLevel, devicepairingsettings: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDeviceInformationPairingStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDeviceInformationPairingStatics {
    type Vtable = IDeviceInformationPairingStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xe915c408_36d4_49a1_bf13_514173799b6b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceInformationPairingStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pairingkindssupported: DevicePairingKinds, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDeviceInformationPairingStatics2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDeviceInformationPairingStatics2 {
    type Vtable = IDeviceInformationPairingStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x04de5372_b7b7_476b_a74f_c5836a704d98);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceInformationPairingStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pairingkindssupported: DevicePairingKinds, minprotectionlevel: DevicePairingProtectionLevel, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDeviceInformationStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDeviceInformationStatics {
    type Vtable = IDeviceInformationStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc17f100e_3a46_4a78_8013_769dc9b97390);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceInformationStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deviceid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deviceid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, additionalproperties: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deviceclass: DeviceClass, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, aqsfilter: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, aqsfilter: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, additionalproperties: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deviceclass: DeviceClass, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, aqsfilter: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, aqsfilter: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, additionalproperties: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDeviceInformationStatics2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDeviceInformationStatics2 {
    type Vtable = IDeviceInformationStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x493b4f34_a84f_45fd_9167_15d1cb1bd1f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceInformationStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deviceclass: DeviceClass, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deviceid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, additionalproperties: ::windows::runtime::RawPtr, kind: DeviceInformationKind, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, aqsfilter: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, additionalproperties: ::windows::runtime::RawPtr, kind: DeviceInformationKind, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, aqsfilter: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, additionalproperties: ::windows::runtime::RawPtr, kind: DeviceInformationKind, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDeviceInformationUpdate(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDeviceInformationUpdate {
    type Vtable = IDeviceInformationUpdate_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8f315305_d972_44b7_a37e_9e822c78213b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceInformationUpdate_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDeviceInformationUpdate2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDeviceInformationUpdate2 {
    type Vtable = IDeviceInformationUpdate2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x5d9d148c_a873_485e_baa6_aa620788e3cc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceInformationUpdate2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut DeviceInformationKind) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDevicePairingRequestedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDevicePairingRequestedEventArgs {
    type Vtable = IDevicePairingRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf717fc56_de6b_487f_8376_0180aca69963);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDevicePairingRequestedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut DevicePairingKinds) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pin: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDevicePairingRequestedEventArgs2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDevicePairingRequestedEventArgs2 {
    type Vtable = IDevicePairingRequestedEventArgs2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc83752d9_e4d3_4db0_a360_a105e437dbdc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDevicePairingRequestedEventArgs2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Security_Credentials")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, passwordcredential: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDevicePairingResult(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDevicePairingResult {
    type Vtable = IDevicePairingResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x072b02bf_dd95_4025_9b37_de51adba37b7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDevicePairingResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut DevicePairingResultStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut DevicePairingProtectionLevel) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Devices_Enumeration`*"]
pub struct IDevicePairingSettings(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDevicePairingSettings {
    type Vtable = IDevicePairingSettings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x482cb27c_83bb_420e_be51_6602b222de54);
}
impl IDevicePairingSettings {}
unsafe impl ::windows::runtime::RuntimeType for IDevicePairingSettings {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{482cb27c-83bb-420e-be51-6602b222de54}");
}
impl ::core::convert::From<IDevicePairingSettings> for ::windows::runtime::IUnknown {
    fn from(value: IDevicePairingSettings) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IDevicePairingSettings> for ::windows::runtime::IUnknown {
    fn from(value: &IDevicePairingSettings) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDevicePairingSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDevicePairingSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IDevicePairingSettings> for ::windows::runtime::IInspectable {
    fn from(value: IDevicePairingSettings) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDevicePairingSettings> for ::windows::runtime::IInspectable {
    fn from(value: &IDevicePairingSettings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IDevicePairingSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IDevicePairingSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDevicePairingSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDevicePicker(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDevicePicker {
    type Vtable = IDevicePicker_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x84997aa2_034a_4440_8813_7d0bd479bf5a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDevicePicker_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
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
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, selection: super::super::Foundation::Rect) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, selection: super::super::Foundation::Rect, placement: super::super::UI::Popups::Placement) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, selection: super::super::Foundation::Rect, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, selection: super::super::Foundation::Rect, placement: super::super::UI::Popups::Placement, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, device: ::windows::runtime::RawPtr, status: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, options: DevicePickerDisplayStatusOptions) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDevicePickerAppearance(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDevicePickerAppearance {
    type Vtable = IDevicePickerAppearance_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xe69a12c6_e627_4ed8_9b6c_460af445e56d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDevicePickerAppearance_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::UI::Color) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::UI::Color) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::UI::Color) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::UI::Color) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::UI::Color) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::UI::Color) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::UI::Color) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::UI::Color) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::UI::Color) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::UI::Color) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::UI::Color) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::UI::Color) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDevicePickerFilter(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDevicePickerFilter {
    type Vtable = IDevicePickerFilter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x91db92a2_57cb_48f1_9b59_a59b7a1f02a2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDevicePickerFilter_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDeviceSelectedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDeviceSelectedEventArgs {
    type Vtable = IDeviceSelectedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x269edade_1d2f_4940_8402_4156b81d3c77);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceSelectedEventArgs_abi(
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
pub struct IDeviceUnpairingResult(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDeviceUnpairingResult {
    type Vtable = IDeviceUnpairingResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x66f44ad3_79d9_444b_92cf_a92ef72571c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceUnpairingResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut DeviceUnpairingResultStatus) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDeviceWatcher(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDeviceWatcher {
    type Vtable = IDeviceWatcher_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc9eab97d_8f6b_4f96_a9f4_abc814e22271);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceWatcher_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
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
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut DeviceWatcherStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDeviceWatcher2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDeviceWatcher2 {
    type Vtable = IDeviceWatcher2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xff08456e_ed14_49e9_9a69_8117c54ae971);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceWatcher2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "ApplicationModel_Background", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, requestedeventkinds: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Background", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDeviceWatcherEvent(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDeviceWatcherEvent {
    type Vtable = IDeviceWatcherEvent_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x74aa9c0b_1dbd_47fd_b635_3cc556d0ff8b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceWatcherEvent_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut DeviceWatcherEventKind) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDeviceWatcherTriggerDetails(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDeviceWatcherTriggerDetails {
    type Vtable = IDeviceWatcherTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x38808119_4cb7_4e57_a56d_776d07cbfef9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceWatcherTriggerDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEnclosureLocation(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEnclosureLocation {
    type Vtable = IEnclosureLocation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x42340a27_5810_459c_aabb_c65e1f813ecf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnclosureLocation_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Panel) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEnclosureLocation2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEnclosureLocation2 {
    type Vtable = IEnclosureLocation2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x2885995b_e07d_485d_8a9e_bdf29aef4f66);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnclosureLocation2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Devices_Enumeration`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct Panel(pub i32);
impl Panel {
    pub const Unknown: Panel = Panel(0i32);
    pub const Front: Panel = Panel(1i32);
    pub const Back: Panel = Panel(2i32);
    pub const Top: Panel = Panel(3i32);
    pub const Bottom: Panel = Panel(4i32);
    pub const Left: Panel = Panel(5i32);
    pub const Right: Panel = Panel(6i32);
}
impl ::core::convert::From<i32> for Panel {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for Panel {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for Panel {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.Enumeration.Panel;i4)");
}
impl ::windows::runtime::DefaultType for Panel {
    type DefaultType = Self;
}
