#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Devices_Enumeration_Pnp")]
pub mod Pnp;
#[doc = "*Required features: `Devices_Enumeration`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DeviceAccessChangedEventArgs(pub ::windows::core::IInspectable);
impl DeviceAccessChangedEventArgs {
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn Status(&self) -> ::windows::core::Result<DeviceAccessStatus> {
        let this = self;
        unsafe {
            let mut result__: DeviceAccessStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DeviceAccessStatus>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IDeviceAccessChangedEventArgs2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for DeviceAccessChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DeviceAccessChangedEventArgs;{deda0bcc-4f9d-4f58-9dba-a9bc800408d5})");
}
unsafe impl ::windows::core::Interface for DeviceAccessChangedEventArgs {
    type Vtable = IDeviceAccessChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdeda0bcc_4f9d_4f58_9dba_a9bc800408d5);
}
impl ::windows::core::RuntimeName for DeviceAccessChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceAccessChangedEventArgs";
}
impl ::core::convert::From<DeviceAccessChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: DeviceAccessChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DeviceAccessChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &DeviceAccessChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DeviceAccessChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DeviceAccessChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DeviceAccessChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: DeviceAccessChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DeviceAccessChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &DeviceAccessChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DeviceAccessChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DeviceAccessChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DeviceAccessChangedEventArgs {}
unsafe impl ::core::marker::Sync for DeviceAccessChangedEventArgs {}
#[doc = "*Required features: `Devices_Enumeration`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DeviceAccessInformation(pub ::windows::core::IInspectable);
impl DeviceAccessInformation {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`*"]
    pub fn AccessChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<DeviceAccessInformation, DeviceAccessChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`*"]
    pub fn RemoveAccessChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn CurrentStatus(&self) -> ::windows::core::Result<DeviceAccessStatus> {
        let this = self;
        unsafe {
            let mut result__: DeviceAccessStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DeviceAccessStatus>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn CreateFromId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0) -> ::windows::core::Result<DeviceAccessInformation> {
        Self::IDeviceAccessInformationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<DeviceAccessInformation>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn CreateFromDeviceClassId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(deviceclassid: Param0) -> ::windows::core::Result<DeviceAccessInformation> {
        Self::IDeviceAccessInformationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), deviceclassid.into_param().abi(), &mut result__).from_abi::<DeviceAccessInformation>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn CreateFromDeviceClass(deviceclass: DeviceClass) -> ::windows::core::Result<DeviceAccessInformation> {
        Self::IDeviceAccessInformationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), deviceclass, &mut result__).from_abi::<DeviceAccessInformation>(result__)
        })
    }
    pub fn IDeviceAccessInformationStatics<R, F: FnOnce(&IDeviceAccessInformationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DeviceAccessInformation, IDeviceAccessInformationStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for DeviceAccessInformation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DeviceAccessInformation;{0baa9a73-6de5-4915-8ddd-9a0554a6f545})");
}
unsafe impl ::windows::core::Interface for DeviceAccessInformation {
    type Vtable = IDeviceAccessInformation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0baa9a73_6de5_4915_8ddd_9a0554a6f545);
}
impl ::windows::core::RuntimeName for DeviceAccessInformation {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceAccessInformation";
}
impl ::core::convert::From<DeviceAccessInformation> for ::windows::core::IUnknown {
    fn from(value: DeviceAccessInformation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DeviceAccessInformation> for ::windows::core::IUnknown {
    fn from(value: &DeviceAccessInformation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DeviceAccessInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DeviceAccessInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DeviceAccessInformation> for ::windows::core::IInspectable {
    fn from(value: DeviceAccessInformation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DeviceAccessInformation> for ::windows::core::IInspectable {
    fn from(value: &DeviceAccessInformation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DeviceAccessInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DeviceAccessInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
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
unsafe impl ::windows::core::Abi for DeviceAccessStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DeviceAccessStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Enumeration.DeviceAccessStatus;i4)");
}
impl ::windows::core::DefaultType for DeviceAccessStatus {
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
unsafe impl ::windows::core::Abi for DeviceClass {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DeviceClass {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Enumeration.DeviceClass;i4)");
}
impl ::windows::core::DefaultType for DeviceClass {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Enumeration`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DeviceConnectionChangeTriggerDetails(pub ::windows::core::IInspectable);
impl DeviceConnectionChangeTriggerDetails {
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for DeviceConnectionChangeTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DeviceConnectionChangeTriggerDetails;{b8578c0c-bbc1-484b-bffa-7b31dcc200b2})");
}
unsafe impl ::windows::core::Interface for DeviceConnectionChangeTriggerDetails {
    type Vtable = IDeviceConnectionChangeTriggerDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb8578c0c_bbc1_484b_bffa_7b31dcc200b2);
}
impl ::windows::core::RuntimeName for DeviceConnectionChangeTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceConnectionChangeTriggerDetails";
}
impl ::core::convert::From<DeviceConnectionChangeTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: DeviceConnectionChangeTriggerDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DeviceConnectionChangeTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: &DeviceConnectionChangeTriggerDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DeviceConnectionChangeTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DeviceConnectionChangeTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DeviceConnectionChangeTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: DeviceConnectionChangeTriggerDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DeviceConnectionChangeTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: &DeviceConnectionChangeTriggerDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DeviceConnectionChangeTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DeviceConnectionChangeTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DeviceConnectionChangeTriggerDetails {}
unsafe impl ::core::marker::Sync for DeviceConnectionChangeTriggerDetails {}
#[doc = "*Required features: `Devices_Enumeration`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DeviceDisconnectButtonClickedEventArgs(pub ::windows::core::IInspectable);
impl DeviceDisconnectButtonClickedEventArgs {
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn Device(&self) -> ::windows::core::Result<DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DeviceInformation>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for DeviceDisconnectButtonClickedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DeviceDisconnectButtonClickedEventArgs;{8e44b56d-f902-4a00-b536-f37992e6a2a7})");
}
unsafe impl ::windows::core::Interface for DeviceDisconnectButtonClickedEventArgs {
    type Vtable = IDeviceDisconnectButtonClickedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8e44b56d_f902_4a00_b536_f37992e6a2a7);
}
impl ::windows::core::RuntimeName for DeviceDisconnectButtonClickedEventArgs {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceDisconnectButtonClickedEventArgs";
}
impl ::core::convert::From<DeviceDisconnectButtonClickedEventArgs> for ::windows::core::IUnknown {
    fn from(value: DeviceDisconnectButtonClickedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DeviceDisconnectButtonClickedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &DeviceDisconnectButtonClickedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DeviceDisconnectButtonClickedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DeviceDisconnectButtonClickedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DeviceDisconnectButtonClickedEventArgs> for ::windows::core::IInspectable {
    fn from(value: DeviceDisconnectButtonClickedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DeviceDisconnectButtonClickedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &DeviceDisconnectButtonClickedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DeviceDisconnectButtonClickedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DeviceDisconnectButtonClickedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DeviceDisconnectButtonClickedEventArgs {}
unsafe impl ::core::marker::Sync for DeviceDisconnectButtonClickedEventArgs {}
#[doc = "*Required features: `Devices_Enumeration`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DeviceInformation(pub ::windows::core::IInspectable);
impl DeviceInformation {
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn IsDefault(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn EnclosureLocation(&self) -> ::windows::core::Result<EnclosureLocation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<EnclosureLocation>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation_Collections`*"]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn Update<'a, Param0: ::windows::core::IntoParam<'a, DeviceInformationUpdate>>(&self, updateinfo: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), updateinfo.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`, `Storage_Streams`*"]
    pub fn GetThumbnailAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceThumbnail>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<DeviceThumbnail>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`, `Storage_Streams`*"]
    pub fn GetGlyphThumbnailAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceThumbnail>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<DeviceThumbnail>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn Kind(&self) -> ::windows::core::Result<DeviceInformationKind> {
        let this = &::windows::core::Interface::cast::<IDeviceInformation2>(self)?;
        unsafe {
            let mut result__: DeviceInformationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DeviceInformationKind>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn Pairing(&self) -> ::windows::core::Result<DeviceInformationPairing> {
        let this = &::windows::core::Interface::cast::<IDeviceInformation2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DeviceInformationPairing>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`*"]
    pub fn CreateFromIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceInformation>> {
        Self::IDeviceInformationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<DeviceInformation>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`, `Foundation_Collections`*"]
    pub fn CreateFromIdAsyncAdditionalProperties<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(deviceid: Param0, additionalproperties: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceInformation>> {
        Self::IDeviceInformationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), additionalproperties.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<DeviceInformation>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`, `Foundation_Collections`*"]
    pub fn FindAllAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceInformationCollection>> {
        Self::IDeviceInformationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<DeviceInformationCollection>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`, `Foundation_Collections`*"]
    pub fn FindAllAsyncDeviceClass(deviceclass: DeviceClass) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceInformationCollection>> {
        Self::IDeviceInformationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), deviceclass, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<DeviceInformationCollection>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`, `Foundation_Collections`*"]
    pub fn FindAllAsyncAqsFilter<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(aqsfilter: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceInformationCollection>> {
        Self::IDeviceInformationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), aqsfilter.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<DeviceInformationCollection>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`, `Foundation_Collections`*"]
    pub fn FindAllAsyncAqsFilterAndAdditionalProperties<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(aqsfilter: Param0, additionalproperties: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceInformationCollection>> {
        Self::IDeviceInformationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), aqsfilter.into_param().abi(), additionalproperties.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<DeviceInformationCollection>>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn CreateWatcher() -> ::windows::core::Result<DeviceWatcher> {
        Self::IDeviceInformationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DeviceWatcher>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn CreateWatcherDeviceClass(deviceclass: DeviceClass) -> ::windows::core::Result<DeviceWatcher> {
        Self::IDeviceInformationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), deviceclass, &mut result__).from_abi::<DeviceWatcher>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn CreateWatcherAqsFilter<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(aqsfilter: Param0) -> ::windows::core::Result<DeviceWatcher> {
        Self::IDeviceInformationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), aqsfilter.into_param().abi(), &mut result__).from_abi::<DeviceWatcher>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation_Collections`*"]
    pub fn CreateWatcherAqsFilterAndAdditionalProperties<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(aqsfilter: Param0, additionalproperties: Param1) -> ::windows::core::Result<DeviceWatcher> {
        Self::IDeviceInformationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), aqsfilter.into_param().abi(), additionalproperties.into_param().abi(), &mut result__).from_abi::<DeviceWatcher>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn GetAqsFilterFromDeviceClass(deviceclass: DeviceClass) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IDeviceInformationStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), deviceclass, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`, `Foundation_Collections`*"]
    pub fn CreateFromIdAsyncWithKindAndAdditionalProperties<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(deviceid: Param0, additionalproperties: Param1, kind: DeviceInformationKind) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceInformation>> {
        Self::IDeviceInformationStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), additionalproperties.into_param().abi(), kind, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<DeviceInformation>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`, `Foundation_Collections`*"]
    pub fn FindAllAsyncWithKindAqsFilterAndAdditionalProperties<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(aqsfilter: Param0, additionalproperties: Param1, kind: DeviceInformationKind) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceInformationCollection>> {
        Self::IDeviceInformationStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), aqsfilter.into_param().abi(), additionalproperties.into_param().abi(), kind, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<DeviceInformationCollection>>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation_Collections`*"]
    pub fn CreateWatcherWithKindAqsFilterAndAdditionalProperties<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(aqsfilter: Param0, additionalproperties: Param1, kind: DeviceInformationKind) -> ::windows::core::Result<DeviceWatcher> {
        Self::IDeviceInformationStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), aqsfilter.into_param().abi(), additionalproperties.into_param().abi(), kind, &mut result__).from_abi::<DeviceWatcher>(result__)
        })
    }
    pub fn IDeviceInformationStatics<R, F: FnOnce(&IDeviceInformationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DeviceInformation, IDeviceInformationStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IDeviceInformationStatics2<R, F: FnOnce(&IDeviceInformationStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DeviceInformation, IDeviceInformationStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for DeviceInformation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DeviceInformation;{aba0fb95-4398-489d-8e44-e6130927011f})");
}
unsafe impl ::windows::core::Interface for DeviceInformation {
    type Vtable = IDeviceInformation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaba0fb95_4398_489d_8e44_e6130927011f);
}
impl ::windows::core::RuntimeName for DeviceInformation {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceInformation";
}
impl ::core::convert::From<DeviceInformation> for ::windows::core::IUnknown {
    fn from(value: DeviceInformation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DeviceInformation> for ::windows::core::IUnknown {
    fn from(value: &DeviceInformation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DeviceInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DeviceInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DeviceInformation> for ::windows::core::IInspectable {
    fn from(value: DeviceInformation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DeviceInformation> for ::windows::core::IInspectable {
    fn from(value: &DeviceInformation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DeviceInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DeviceInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DeviceInformation {}
unsafe impl ::core::marker::Sync for DeviceInformation {}
#[cfg(feature = "Foundation_Collections")]
#[doc = "*Required features: `Devices_Enumeration`, `Foundation_Collections`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DeviceInformationCollection(pub ::windows::core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl DeviceInformationCollection {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation_Collections`*"]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<DeviceInformation>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation_Collections`*"]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation_Collections`*"]
    pub fn IndexOf<'a, Param0: ::windows::core::IntoParam<'a, DeviceInformation>>(&self, value: Param0, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation_Collections`*"]
    pub fn GetMany(&self, startindex: u32, items: &mut [<DeviceInformation as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation_Collections`*"]
    pub fn First(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IIterator<DeviceInformation>> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IIterable<DeviceInformation>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IIterator<DeviceInformation>>(result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for DeviceInformationCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DeviceInformationCollection;pinterface({bbe1fa4c-b0e3-4583-baef-1f1b2e483e56};rc(Windows.Devices.Enumeration.DeviceInformation;{aba0fb95-4398-489d-8e44-e6130927011f})))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for DeviceInformationCollection {
    type Vtable = super::super::Foundation::Collections::IVectorView_abi<DeviceInformation>;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<super::super::Foundation::Collections::IVectorView<DeviceInformation> as ::windows::core::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for DeviceInformationCollection {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceInformationCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<DeviceInformationCollection> for ::windows::core::IUnknown {
    fn from(value: DeviceInformationCollection) -> Self {
        value.0 .0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&DeviceInformationCollection> for ::windows::core::IUnknown {
    fn from(value: &DeviceInformationCollection) -> Self {
        value.0 .0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DeviceInformationCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DeviceInformationCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<DeviceInformationCollection> for ::windows::core::IInspectable {
    fn from(value: DeviceInformationCollection) -> Self {
        value.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&DeviceInformationCollection> for ::windows::core::IInspectable {
    fn from(value: &DeviceInformationCollection) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DeviceInformationCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DeviceInformationCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IVectorView<DeviceInformation>> for DeviceInformationCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IVectorView<DeviceInformation>> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IVectorView<DeviceInformation>> for &DeviceInformationCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IVectorView<DeviceInformation>> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<DeviceInformationCollection> for super::super::Foundation::Collections::IIterable<DeviceInformation> {
    type Error = ::windows::core::Error;
    fn try_from(value: DeviceInformationCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&DeviceInformationCollection> for super::super::Foundation::Collections::IIterable<DeviceInformation> {
    type Error = ::windows::core::Error;
    fn try_from(value: &DeviceInformationCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<DeviceInformation>> for DeviceInformationCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IIterable<DeviceInformation>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<DeviceInformation>> for &DeviceInformationCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IIterable<DeviceInformation>> {
        ::core::convert::TryInto::<super::super::Foundation::Collections::IIterable<DeviceInformation>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
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
pub struct DeviceInformationCustomPairing(pub ::windows::core::IInspectable);
impl DeviceInformationCustomPairing {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`*"]
    pub fn PairAsync(&self, pairingkindssupported: DevicePairingKinds) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DevicePairingResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), pairingkindssupported, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<DevicePairingResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`*"]
    pub fn PairWithProtectionLevelAsync(&self, pairingkindssupported: DevicePairingKinds, minprotectionlevel: DevicePairingProtectionLevel) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DevicePairingResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), pairingkindssupported, minprotectionlevel, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<DevicePairingResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`*"]
    pub fn PairWithProtectionLevelAndSettingsAsync<'a, Param2: ::windows::core::IntoParam<'a, IDevicePairingSettings>>(&self, pairingkindssupported: DevicePairingKinds, minprotectionlevel: DevicePairingProtectionLevel, devicepairingsettings: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DevicePairingResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), pairingkindssupported, minprotectionlevel, devicepairingsettings.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<DevicePairingResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`*"]
    pub fn PairingRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<DeviceInformationCustomPairing, DevicePairingRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`*"]
    pub fn RemovePairingRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for DeviceInformationCustomPairing {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DeviceInformationCustomPairing;{85138c02-4ee6-4914-8370-107a39144c0e})");
}
unsafe impl ::windows::core::Interface for DeviceInformationCustomPairing {
    type Vtable = IDeviceInformationCustomPairing_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x85138c02_4ee6_4914_8370_107a39144c0e);
}
impl ::windows::core::RuntimeName for DeviceInformationCustomPairing {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceInformationCustomPairing";
}
impl ::core::convert::From<DeviceInformationCustomPairing> for ::windows::core::IUnknown {
    fn from(value: DeviceInformationCustomPairing) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DeviceInformationCustomPairing> for ::windows::core::IUnknown {
    fn from(value: &DeviceInformationCustomPairing) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DeviceInformationCustomPairing {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DeviceInformationCustomPairing {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DeviceInformationCustomPairing> for ::windows::core::IInspectable {
    fn from(value: DeviceInformationCustomPairing) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DeviceInformationCustomPairing> for ::windows::core::IInspectable {
    fn from(value: &DeviceInformationCustomPairing) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DeviceInformationCustomPairing {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DeviceInformationCustomPairing {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
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
unsafe impl ::windows::core::Abi for DeviceInformationKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DeviceInformationKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Enumeration.DeviceInformationKind;i4)");
}
impl ::windows::core::DefaultType for DeviceInformationKind {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Enumeration`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DeviceInformationPairing(pub ::windows::core::IInspectable);
impl DeviceInformationPairing {
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn IsPaired(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn CanPair(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`*"]
    pub fn PairAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DevicePairingResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<DevicePairingResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`*"]
    pub fn PairWithProtectionLevelAsync(&self, minprotectionlevel: DevicePairingProtectionLevel) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DevicePairingResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), minprotectionlevel, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<DevicePairingResult>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn ProtectionLevel(&self) -> ::windows::core::Result<DevicePairingProtectionLevel> {
        let this = &::windows::core::Interface::cast::<IDeviceInformationPairing2>(self)?;
        unsafe {
            let mut result__: DevicePairingProtectionLevel = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DevicePairingProtectionLevel>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn Custom(&self) -> ::windows::core::Result<DeviceInformationCustomPairing> {
        let this = &::windows::core::Interface::cast::<IDeviceInformationPairing2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DeviceInformationCustomPairing>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`*"]
    pub fn PairWithProtectionLevelAndSettingsAsync<'a, Param1: ::windows::core::IntoParam<'a, IDevicePairingSettings>>(&self, minprotectionlevel: DevicePairingProtectionLevel, devicepairingsettings: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DevicePairingResult>> {
        let this = &::windows::core::Interface::cast::<IDeviceInformationPairing2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), minprotectionlevel, devicepairingsettings.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<DevicePairingResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`*"]
    pub fn UnpairAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceUnpairingResult>> {
        let this = &::windows::core::Interface::cast::<IDeviceInformationPairing2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<DeviceUnpairingResult>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn TryRegisterForAllInboundPairingRequests(pairingkindssupported: DevicePairingKinds) -> ::windows::core::Result<bool> {
        Self::IDeviceInformationPairingStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), pairingkindssupported, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn TryRegisterForAllInboundPairingRequestsWithProtectionLevel(pairingkindssupported: DevicePairingKinds, minprotectionlevel: DevicePairingProtectionLevel) -> ::windows::core::Result<bool> {
        Self::IDeviceInformationPairingStatics2(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), pairingkindssupported, minprotectionlevel, &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IDeviceInformationPairingStatics<R, F: FnOnce(&IDeviceInformationPairingStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DeviceInformationPairing, IDeviceInformationPairingStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IDeviceInformationPairingStatics2<R, F: FnOnce(&IDeviceInformationPairingStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DeviceInformationPairing, IDeviceInformationPairingStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for DeviceInformationPairing {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DeviceInformationPairing;{2c4769f5-f684-40d5-8469-e8dbaab70485})");
}
unsafe impl ::windows::core::Interface for DeviceInformationPairing {
    type Vtable = IDeviceInformationPairing_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c4769f5_f684_40d5_8469_e8dbaab70485);
}
impl ::windows::core::RuntimeName for DeviceInformationPairing {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceInformationPairing";
}
impl ::core::convert::From<DeviceInformationPairing> for ::windows::core::IUnknown {
    fn from(value: DeviceInformationPairing) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DeviceInformationPairing> for ::windows::core::IUnknown {
    fn from(value: &DeviceInformationPairing) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DeviceInformationPairing {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DeviceInformationPairing {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DeviceInformationPairing> for ::windows::core::IInspectable {
    fn from(value: DeviceInformationPairing) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DeviceInformationPairing> for ::windows::core::IInspectable {
    fn from(value: &DeviceInformationPairing) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DeviceInformationPairing {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DeviceInformationPairing {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DeviceInformationPairing {}
unsafe impl ::core::marker::Sync for DeviceInformationPairing {}
#[doc = "*Required features: `Devices_Enumeration`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DeviceInformationUpdate(pub ::windows::core::IInspectable);
impl DeviceInformationUpdate {
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation_Collections`*"]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn Kind(&self) -> ::windows::core::Result<DeviceInformationKind> {
        let this = &::windows::core::Interface::cast::<IDeviceInformationUpdate2>(self)?;
        unsafe {
            let mut result__: DeviceInformationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DeviceInformationKind>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for DeviceInformationUpdate {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DeviceInformationUpdate;{8f315305-d972-44b7-a37e-9e822c78213b})");
}
unsafe impl ::windows::core::Interface for DeviceInformationUpdate {
    type Vtable = IDeviceInformationUpdate_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f315305_d972_44b7_a37e_9e822c78213b);
}
impl ::windows::core::RuntimeName for DeviceInformationUpdate {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceInformationUpdate";
}
impl ::core::convert::From<DeviceInformationUpdate> for ::windows::core::IUnknown {
    fn from(value: DeviceInformationUpdate) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DeviceInformationUpdate> for ::windows::core::IUnknown {
    fn from(value: &DeviceInformationUpdate) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DeviceInformationUpdate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DeviceInformationUpdate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DeviceInformationUpdate> for ::windows::core::IInspectable {
    fn from(value: DeviceInformationUpdate) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DeviceInformationUpdate> for ::windows::core::IInspectable {
    fn from(value: &DeviceInformationUpdate) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DeviceInformationUpdate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DeviceInformationUpdate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
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
unsafe impl ::windows::core::Abi for DevicePairingKinds {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DevicePairingKinds {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Enumeration.DevicePairingKinds;u4)");
}
impl ::windows::core::DefaultType for DevicePairingKinds {
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
unsafe impl ::windows::core::Abi for DevicePairingProtectionLevel {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DevicePairingProtectionLevel {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Enumeration.DevicePairingProtectionLevel;i4)");
}
impl ::windows::core::DefaultType for DevicePairingProtectionLevel {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Enumeration`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DevicePairingRequestedEventArgs(pub ::windows::core::IInspectable);
impl DevicePairingRequestedEventArgs {
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn DeviceInformation(&self) -> ::windows::core::Result<DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DeviceInformation>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn PairingKind(&self) -> ::windows::core::Result<DevicePairingKinds> {
        let this = self;
        unsafe {
            let mut result__: DevicePairingKinds = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DevicePairingKinds>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn Pin(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn Accept(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn AcceptWithPin<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, pin: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), pin.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Deferral>(result__)
        }
    }
    #[cfg(feature = "Security_Credentials")]
    #[doc = "*Required features: `Devices_Enumeration`, `Security_Credentials`*"]
    pub fn AcceptWithPasswordCredential<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::Credentials::PasswordCredential>>(&self, passwordcredential: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IDevicePairingRequestedEventArgs2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), passwordcredential.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for DevicePairingRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DevicePairingRequestedEventArgs;{f717fc56-de6b-487f-8376-0180aca69963})");
}
unsafe impl ::windows::core::Interface for DevicePairingRequestedEventArgs {
    type Vtable = IDevicePairingRequestedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf717fc56_de6b_487f_8376_0180aca69963);
}
impl ::windows::core::RuntimeName for DevicePairingRequestedEventArgs {
    const NAME: &'static str = "Windows.Devices.Enumeration.DevicePairingRequestedEventArgs";
}
impl ::core::convert::From<DevicePairingRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: DevicePairingRequestedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DevicePairingRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &DevicePairingRequestedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DevicePairingRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DevicePairingRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DevicePairingRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: DevicePairingRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DevicePairingRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &DevicePairingRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DevicePairingRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DevicePairingRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DevicePairingRequestedEventArgs {}
unsafe impl ::core::marker::Sync for DevicePairingRequestedEventArgs {}
#[doc = "*Required features: `Devices_Enumeration`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DevicePairingResult(pub ::windows::core::IInspectable);
impl DevicePairingResult {
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn Status(&self) -> ::windows::core::Result<DevicePairingResultStatus> {
        let this = self;
        unsafe {
            let mut result__: DevicePairingResultStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DevicePairingResultStatus>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn ProtectionLevelUsed(&self) -> ::windows::core::Result<DevicePairingProtectionLevel> {
        let this = self;
        unsafe {
            let mut result__: DevicePairingProtectionLevel = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DevicePairingProtectionLevel>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for DevicePairingResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DevicePairingResult;{072b02bf-dd95-4025-9b37-de51adba37b7})");
}
unsafe impl ::windows::core::Interface for DevicePairingResult {
    type Vtable = IDevicePairingResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x072b02bf_dd95_4025_9b37_de51adba37b7);
}
impl ::windows::core::RuntimeName for DevicePairingResult {
    const NAME: &'static str = "Windows.Devices.Enumeration.DevicePairingResult";
}
impl ::core::convert::From<DevicePairingResult> for ::windows::core::IUnknown {
    fn from(value: DevicePairingResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DevicePairingResult> for ::windows::core::IUnknown {
    fn from(value: &DevicePairingResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DevicePairingResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DevicePairingResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DevicePairingResult> for ::windows::core::IInspectable {
    fn from(value: DevicePairingResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DevicePairingResult> for ::windows::core::IInspectable {
    fn from(value: &DevicePairingResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DevicePairingResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DevicePairingResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
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
unsafe impl ::windows::core::Abi for DevicePairingResultStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DevicePairingResultStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Enumeration.DevicePairingResultStatus;i4)");
}
impl ::windows::core::DefaultType for DevicePairingResultStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Enumeration`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DevicePicker(pub ::windows::core::IInspectable);
impl DevicePicker {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DevicePicker, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn Filter(&self) -> ::windows::core::Result<DevicePickerFilter> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DevicePickerFilter>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn Appearance(&self) -> ::windows::core::Result<DevicePickerAppearance> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DevicePickerAppearance>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation_Collections`*"]
    pub fn RequestedProperties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`*"]
    pub fn DeviceSelected<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<DevicePicker, DeviceSelectedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`*"]
    pub fn RemoveDeviceSelected<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`*"]
    pub fn DisconnectButtonClicked<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<DevicePicker, DeviceDisconnectButtonClickedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`*"]
    pub fn RemoveDisconnectButtonClicked<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`*"]
    pub fn DevicePickerDismissed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<DevicePicker, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`*"]
    pub fn RemoveDevicePickerDismissed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`*"]
    pub fn Show<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>>(&self, selection: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), selection.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`, `UI_Popups`*"]
    pub fn ShowWithPlacement<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>>(&self, selection: Param0, placement: super::super::UI::Popups::Placement) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), selection.into_param().abi(), placement).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`*"]
    pub fn PickSingleDeviceAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>>(&self, selection: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceInformation>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), selection.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<DeviceInformation>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`, `UI_Popups`*"]
    pub fn PickSingleDeviceAsyncWithPlacement<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>>(&self, selection: Param0, placement: super::super::UI::Popups::Placement) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceInformation>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), selection.into_param().abi(), placement, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<DeviceInformation>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn Hide(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn SetDisplayStatus<'a, Param0: ::windows::core::IntoParam<'a, DeviceInformation>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, device: Param0, status: Param1, options: DevicePickerDisplayStatusOptions) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), device.into_param().abi(), status.into_param().abi(), options).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for DevicePicker {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DevicePicker;{84997aa2-034a-4440-8813-7d0bd479bf5a})");
}
unsafe impl ::windows::core::Interface for DevicePicker {
    type Vtable = IDevicePicker_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x84997aa2_034a_4440_8813_7d0bd479bf5a);
}
impl ::windows::core::RuntimeName for DevicePicker {
    const NAME: &'static str = "Windows.Devices.Enumeration.DevicePicker";
}
impl ::core::convert::From<DevicePicker> for ::windows::core::IUnknown {
    fn from(value: DevicePicker) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DevicePicker> for ::windows::core::IUnknown {
    fn from(value: &DevicePicker) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DevicePicker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DevicePicker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DevicePicker> for ::windows::core::IInspectable {
    fn from(value: DevicePicker) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DevicePicker> for ::windows::core::IInspectable {
    fn from(value: &DevicePicker) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DevicePicker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DevicePicker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DevicePicker {}
unsafe impl ::core::marker::Sync for DevicePicker {}
#[doc = "*Required features: `Devices_Enumeration`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DevicePickerAppearance(pub ::windows::core::IInspectable);
impl DevicePickerAppearance {
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn SetTitle<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI")]
    #[doc = "*Required features: `Devices_Enumeration`, `UI`*"]
    pub fn ForegroundColor(&self) -> ::windows::core::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::UI::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Color>(result__)
        }
    }
    #[cfg(feature = "UI")]
    #[doc = "*Required features: `Devices_Enumeration`, `UI`*"]
    pub fn SetForegroundColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::UI::Color>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI")]
    #[doc = "*Required features: `Devices_Enumeration`, `UI`*"]
    pub fn BackgroundColor(&self) -> ::windows::core::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::UI::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Color>(result__)
        }
    }
    #[cfg(feature = "UI")]
    #[doc = "*Required features: `Devices_Enumeration`, `UI`*"]
    pub fn SetBackgroundColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::UI::Color>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI")]
    #[doc = "*Required features: `Devices_Enumeration`, `UI`*"]
    pub fn AccentColor(&self) -> ::windows::core::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::UI::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Color>(result__)
        }
    }
    #[cfg(feature = "UI")]
    #[doc = "*Required features: `Devices_Enumeration`, `UI`*"]
    pub fn SetAccentColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::UI::Color>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI")]
    #[doc = "*Required features: `Devices_Enumeration`, `UI`*"]
    pub fn SelectedForegroundColor(&self) -> ::windows::core::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::UI::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Color>(result__)
        }
    }
    #[cfg(feature = "UI")]
    #[doc = "*Required features: `Devices_Enumeration`, `UI`*"]
    pub fn SetSelectedForegroundColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::UI::Color>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI")]
    #[doc = "*Required features: `Devices_Enumeration`, `UI`*"]
    pub fn SelectedBackgroundColor(&self) -> ::windows::core::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::UI::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Color>(result__)
        }
    }
    #[cfg(feature = "UI")]
    #[doc = "*Required features: `Devices_Enumeration`, `UI`*"]
    pub fn SetSelectedBackgroundColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::UI::Color>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI")]
    #[doc = "*Required features: `Devices_Enumeration`, `UI`*"]
    pub fn SelectedAccentColor(&self) -> ::windows::core::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::UI::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Color>(result__)
        }
    }
    #[cfg(feature = "UI")]
    #[doc = "*Required features: `Devices_Enumeration`, `UI`*"]
    pub fn SetSelectedAccentColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::UI::Color>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for DevicePickerAppearance {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DevicePickerAppearance;{e69a12c6-e627-4ed8-9b6c-460af445e56d})");
}
unsafe impl ::windows::core::Interface for DevicePickerAppearance {
    type Vtable = IDevicePickerAppearance_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe69a12c6_e627_4ed8_9b6c_460af445e56d);
}
impl ::windows::core::RuntimeName for DevicePickerAppearance {
    const NAME: &'static str = "Windows.Devices.Enumeration.DevicePickerAppearance";
}
impl ::core::convert::From<DevicePickerAppearance> for ::windows::core::IUnknown {
    fn from(value: DevicePickerAppearance) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DevicePickerAppearance> for ::windows::core::IUnknown {
    fn from(value: &DevicePickerAppearance) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DevicePickerAppearance {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DevicePickerAppearance {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DevicePickerAppearance> for ::windows::core::IInspectable {
    fn from(value: DevicePickerAppearance) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DevicePickerAppearance> for ::windows::core::IInspectable {
    fn from(value: &DevicePickerAppearance) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DevicePickerAppearance {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DevicePickerAppearance {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
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
unsafe impl ::windows::core::Abi for DevicePickerDisplayStatusOptions {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DevicePickerDisplayStatusOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Enumeration.DevicePickerDisplayStatusOptions;u4)");
}
impl ::windows::core::DefaultType for DevicePickerDisplayStatusOptions {
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
pub struct DevicePickerFilter(pub ::windows::core::IInspectable);
impl DevicePickerFilter {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation_Collections`*"]
    pub fn SupportedDeviceClasses(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<DeviceClass>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<DeviceClass>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation_Collections`*"]
    pub fn SupportedDeviceSelectors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for DevicePickerFilter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DevicePickerFilter;{91db92a2-57cb-48f1-9b59-a59b7a1f02a2})");
}
unsafe impl ::windows::core::Interface for DevicePickerFilter {
    type Vtable = IDevicePickerFilter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x91db92a2_57cb_48f1_9b59_a59b7a1f02a2);
}
impl ::windows::core::RuntimeName for DevicePickerFilter {
    const NAME: &'static str = "Windows.Devices.Enumeration.DevicePickerFilter";
}
impl ::core::convert::From<DevicePickerFilter> for ::windows::core::IUnknown {
    fn from(value: DevicePickerFilter) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DevicePickerFilter> for ::windows::core::IUnknown {
    fn from(value: &DevicePickerFilter) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DevicePickerFilter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DevicePickerFilter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DevicePickerFilter> for ::windows::core::IInspectable {
    fn from(value: DevicePickerFilter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DevicePickerFilter> for ::windows::core::IInspectable {
    fn from(value: &DevicePickerFilter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DevicePickerFilter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DevicePickerFilter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DevicePickerFilter {}
unsafe impl ::core::marker::Sync for DevicePickerFilter {}
#[doc = "*Required features: `Devices_Enumeration`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DeviceSelectedEventArgs(pub ::windows::core::IInspectable);
impl DeviceSelectedEventArgs {
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn SelectedDevice(&self) -> ::windows::core::Result<DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DeviceInformation>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for DeviceSelectedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DeviceSelectedEventArgs;{269edade-1d2f-4940-8402-4156b81d3c77})");
}
unsafe impl ::windows::core::Interface for DeviceSelectedEventArgs {
    type Vtable = IDeviceSelectedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x269edade_1d2f_4940_8402_4156b81d3c77);
}
impl ::windows::core::RuntimeName for DeviceSelectedEventArgs {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceSelectedEventArgs";
}
impl ::core::convert::From<DeviceSelectedEventArgs> for ::windows::core::IUnknown {
    fn from(value: DeviceSelectedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DeviceSelectedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &DeviceSelectedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DeviceSelectedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DeviceSelectedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DeviceSelectedEventArgs> for ::windows::core::IInspectable {
    fn from(value: DeviceSelectedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DeviceSelectedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &DeviceSelectedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DeviceSelectedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DeviceSelectedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DeviceSelectedEventArgs {}
unsafe impl ::core::marker::Sync for DeviceSelectedEventArgs {}
#[cfg(feature = "Storage_Streams")]
#[doc = "*Required features: `Devices_Enumeration`, `Storage_Streams`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DeviceThumbnail(pub ::windows::core::IInspectable);
#[cfg(feature = "Storage_Streams")]
impl DeviceThumbnail {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Enumeration`, `Storage_Streams`*"]
    pub fn ContentType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IContentTypeProvider>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`, `Storage_Streams`*"]
    pub fn ReadAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, buffer: Param0, count: u32, options: super::super::Storage::Streams::InputStreamOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IBuffer, u32>> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IInputStream>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), buffer.into_param().abi(), count, options, &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IBuffer, u32>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`, `Storage_Streams`*"]
    pub fn WriteAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, buffer: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IOutputStream>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), buffer.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`, `Storage_Streams`*"]
    pub fn FlushAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IOutputStream>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Enumeration`, `Storage_Streams`*"]
    pub fn Size(&self) -> ::windows::core::Result<u64> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Enumeration`, `Storage_Streams`*"]
    pub fn SetSize(&self, value: u64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Enumeration`, `Storage_Streams`*"]
    pub fn GetInputStreamAt(&self, position: u64) -> ::windows::core::Result<super::super::Storage::Streams::IInputStream> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), position, &mut result__).from_abi::<super::super::Storage::Streams::IInputStream>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Enumeration`, `Storage_Streams`*"]
    pub fn GetOutputStreamAt(&self, position: u64) -> ::windows::core::Result<super::super::Storage::Streams::IOutputStream> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), position, &mut result__).from_abi::<super::super::Storage::Streams::IOutputStream>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Enumeration`, `Storage_Streams`*"]
    pub fn Position(&self) -> ::windows::core::Result<u64> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Enumeration`, `Storage_Streams`*"]
    pub fn Seek(&self, position: u64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), position).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Enumeration`, `Storage_Streams`*"]
    pub fn CloneStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStream> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IRandomAccessStream>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Enumeration`, `Storage_Streams`*"]
    pub fn CanRead(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Enumeration`, `Storage_Streams`*"]
    pub fn CanWrite(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
#[cfg(feature = "Storage_Streams")]
unsafe impl ::windows::core::RuntimeType for DeviceThumbnail {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DeviceThumbnail;{cc254827-4b3d-438f-9232-10c76bc7e038})");
}
#[cfg(feature = "Storage_Streams")]
unsafe impl ::windows::core::Interface for DeviceThumbnail {
    type Vtable = super::super::Storage::Streams::IRandomAccessStreamWithContentType_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcc254827_4b3d_438f_9232_10c76bc7e038);
}
#[cfg(feature = "Storage_Streams")]
impl ::windows::core::RuntimeName for DeviceThumbnail {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceThumbnail";
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::From<DeviceThumbnail> for ::windows::core::IUnknown {
    fn from(value: DeviceThumbnail) -> Self {
        value.0 .0
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::From<&DeviceThumbnail> for ::windows::core::IUnknown {
    fn from(value: &DeviceThumbnail) -> Self {
        value.0 .0.clone()
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DeviceThumbnail {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DeviceThumbnail {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::From<DeviceThumbnail> for ::windows::core::IInspectable {
    fn from(value: DeviceThumbnail) -> Self {
        value.0
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::From<&DeviceThumbnail> for ::windows::core::IInspectable {
    fn from(value: &DeviceThumbnail) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DeviceThumbnail {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DeviceThumbnail {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStreamWithContentType> for DeviceThumbnail {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Storage::Streams::IRandomAccessStreamWithContentType> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStreamWithContentType> for &DeviceThumbnail {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Storage::Streams::IRandomAccessStreamWithContentType> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl ::core::convert::TryFrom<DeviceThumbnail> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: DeviceThumbnail) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl ::core::convert::TryFrom<&DeviceThumbnail> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &DeviceThumbnail) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for DeviceThumbnail {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &DeviceThumbnail {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<DeviceThumbnail> for super::super::Storage::Streams::IContentTypeProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: DeviceThumbnail) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&DeviceThumbnail> for super::super::Storage::Streams::IContentTypeProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: &DeviceThumbnail) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Storage::Streams::IContentTypeProvider> for DeviceThumbnail {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Storage::Streams::IContentTypeProvider> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Storage::Streams::IContentTypeProvider> for &DeviceThumbnail {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Storage::Streams::IContentTypeProvider> {
        ::core::convert::TryInto::<super::super::Storage::Streams::IContentTypeProvider>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<DeviceThumbnail> for super::super::Storage::Streams::IInputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: DeviceThumbnail) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&DeviceThumbnail> for super::super::Storage::Streams::IInputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: &DeviceThumbnail) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Storage::Streams::IInputStream> for DeviceThumbnail {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Storage::Streams::IInputStream> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Storage::Streams::IInputStream> for &DeviceThumbnail {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Storage::Streams::IInputStream> {
        ::core::convert::TryInto::<super::super::Storage::Streams::IInputStream>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<DeviceThumbnail> for super::super::Storage::Streams::IOutputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: DeviceThumbnail) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&DeviceThumbnail> for super::super::Storage::Streams::IOutputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: &DeviceThumbnail) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Storage::Streams::IOutputStream> for DeviceThumbnail {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Storage::Streams::IOutputStream> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Storage::Streams::IOutputStream> for &DeviceThumbnail {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Storage::Streams::IOutputStream> {
        ::core::convert::TryInto::<super::super::Storage::Streams::IOutputStream>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<DeviceThumbnail> for super::super::Storage::Streams::IRandomAccessStream {
    type Error = ::windows::core::Error;
    fn try_from(value: DeviceThumbnail) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&DeviceThumbnail> for super::super::Storage::Streams::IRandomAccessStream {
    type Error = ::windows::core::Error;
    fn try_from(value: &DeviceThumbnail) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream> for DeviceThumbnail {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Storage::Streams::IRandomAccessStream> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream> for &DeviceThumbnail {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Storage::Streams::IRandomAccessStream> {
        ::core::convert::TryInto::<super::super::Storage::Streams::IRandomAccessStream>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Storage_Streams")]
unsafe impl ::core::marker::Send for DeviceThumbnail {}
#[cfg(feature = "Storage_Streams")]
unsafe impl ::core::marker::Sync for DeviceThumbnail {}
#[doc = "*Required features: `Devices_Enumeration`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DeviceUnpairingResult(pub ::windows::core::IInspectable);
impl DeviceUnpairingResult {
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn Status(&self) -> ::windows::core::Result<DeviceUnpairingResultStatus> {
        let this = self;
        unsafe {
            let mut result__: DeviceUnpairingResultStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DeviceUnpairingResultStatus>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for DeviceUnpairingResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DeviceUnpairingResult;{66f44ad3-79d9-444b-92cf-a92ef72571c7})");
}
unsafe impl ::windows::core::Interface for DeviceUnpairingResult {
    type Vtable = IDeviceUnpairingResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x66f44ad3_79d9_444b_92cf_a92ef72571c7);
}
impl ::windows::core::RuntimeName for DeviceUnpairingResult {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceUnpairingResult";
}
impl ::core::convert::From<DeviceUnpairingResult> for ::windows::core::IUnknown {
    fn from(value: DeviceUnpairingResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DeviceUnpairingResult> for ::windows::core::IUnknown {
    fn from(value: &DeviceUnpairingResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DeviceUnpairingResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DeviceUnpairingResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DeviceUnpairingResult> for ::windows::core::IInspectable {
    fn from(value: DeviceUnpairingResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DeviceUnpairingResult> for ::windows::core::IInspectable {
    fn from(value: &DeviceUnpairingResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DeviceUnpairingResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DeviceUnpairingResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
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
unsafe impl ::windows::core::Abi for DeviceUnpairingResultStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DeviceUnpairingResultStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Enumeration.DeviceUnpairingResultStatus;i4)");
}
impl ::windows::core::DefaultType for DeviceUnpairingResultStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Enumeration`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DeviceWatcher(pub ::windows::core::IInspectable);
impl DeviceWatcher {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`*"]
    pub fn Added<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<DeviceWatcher, DeviceInformation>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`*"]
    pub fn RemoveAdded<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`*"]
    pub fn Updated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<DeviceWatcher, DeviceInformationUpdate>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`*"]
    pub fn RemoveUpdated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`*"]
    pub fn Removed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<DeviceWatcher, DeviceInformationUpdate>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`*"]
    pub fn RemoveRemoved<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`*"]
    pub fn EnumerationCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<DeviceWatcher, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`*"]
    pub fn RemoveEnumerationCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`*"]
    pub fn Stopped<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<DeviceWatcher, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation`*"]
    pub fn RemoveStopped<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn Status(&self) -> ::windows::core::Result<DeviceWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__: DeviceWatcherStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DeviceWatcherStatus>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(all(feature = "ApplicationModel_Background", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_Enumeration`, `ApplicationModel_Background`, `Foundation_Collections`*"]
    pub fn GetBackgroundTrigger<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<DeviceWatcherEventKind>>>(&self, requestedeventkinds: Param0) -> ::windows::core::Result<super::super::ApplicationModel::Background::DeviceWatcherTrigger> {
        let this = &::windows::core::Interface::cast::<IDeviceWatcher2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), requestedeventkinds.into_param().abi(), &mut result__).from_abi::<super::super::ApplicationModel::Background::DeviceWatcherTrigger>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for DeviceWatcher {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DeviceWatcher;{c9eab97d-8f6b-4f96-a9f4-abc814e22271})");
}
unsafe impl ::windows::core::Interface for DeviceWatcher {
    type Vtable = IDeviceWatcher_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc9eab97d_8f6b_4f96_a9f4_abc814e22271);
}
impl ::windows::core::RuntimeName for DeviceWatcher {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceWatcher";
}
impl ::core::convert::From<DeviceWatcher> for ::windows::core::IUnknown {
    fn from(value: DeviceWatcher) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DeviceWatcher> for ::windows::core::IUnknown {
    fn from(value: &DeviceWatcher) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DeviceWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DeviceWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DeviceWatcher> for ::windows::core::IInspectable {
    fn from(value: DeviceWatcher) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DeviceWatcher> for ::windows::core::IInspectable {
    fn from(value: &DeviceWatcher) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DeviceWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DeviceWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DeviceWatcher {}
unsafe impl ::core::marker::Sync for DeviceWatcher {}
#[doc = "*Required features: `Devices_Enumeration`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DeviceWatcherEvent(pub ::windows::core::IInspectable);
impl DeviceWatcherEvent {
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn Kind(&self) -> ::windows::core::Result<DeviceWatcherEventKind> {
        let this = self;
        unsafe {
            let mut result__: DeviceWatcherEventKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DeviceWatcherEventKind>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn DeviceInformation(&self) -> ::windows::core::Result<DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DeviceInformation>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn DeviceInformationUpdate(&self) -> ::windows::core::Result<DeviceInformationUpdate> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DeviceInformationUpdate>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for DeviceWatcherEvent {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DeviceWatcherEvent;{74aa9c0b-1dbd-47fd-b635-3cc556d0ff8b})");
}
unsafe impl ::windows::core::Interface for DeviceWatcherEvent {
    type Vtable = IDeviceWatcherEvent_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x74aa9c0b_1dbd_47fd_b635_3cc556d0ff8b);
}
impl ::windows::core::RuntimeName for DeviceWatcherEvent {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceWatcherEvent";
}
impl ::core::convert::From<DeviceWatcherEvent> for ::windows::core::IUnknown {
    fn from(value: DeviceWatcherEvent) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DeviceWatcherEvent> for ::windows::core::IUnknown {
    fn from(value: &DeviceWatcherEvent) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DeviceWatcherEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DeviceWatcherEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DeviceWatcherEvent> for ::windows::core::IInspectable {
    fn from(value: DeviceWatcherEvent) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DeviceWatcherEvent> for ::windows::core::IInspectable {
    fn from(value: &DeviceWatcherEvent) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DeviceWatcherEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DeviceWatcherEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
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
unsafe impl ::windows::core::Abi for DeviceWatcherEventKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DeviceWatcherEventKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Enumeration.DeviceWatcherEventKind;i4)");
}
impl ::windows::core::DefaultType for DeviceWatcherEventKind {
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
unsafe impl ::windows::core::Abi for DeviceWatcherStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DeviceWatcherStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Enumeration.DeviceWatcherStatus;i4)");
}
impl ::windows::core::DefaultType for DeviceWatcherStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Enumeration`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DeviceWatcherTriggerDetails(pub ::windows::core::IInspectable);
impl DeviceWatcherTriggerDetails {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Enumeration`, `Foundation_Collections`*"]
    pub fn DeviceWatcherEvents(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<DeviceWatcherEvent>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<DeviceWatcherEvent>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for DeviceWatcherTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DeviceWatcherTriggerDetails;{38808119-4cb7-4e57-a56d-776d07cbfef9})");
}
unsafe impl ::windows::core::Interface for DeviceWatcherTriggerDetails {
    type Vtable = IDeviceWatcherTriggerDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38808119_4cb7_4e57_a56d_776d07cbfef9);
}
impl ::windows::core::RuntimeName for DeviceWatcherTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceWatcherTriggerDetails";
}
impl ::core::convert::From<DeviceWatcherTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: DeviceWatcherTriggerDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DeviceWatcherTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: &DeviceWatcherTriggerDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DeviceWatcherTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DeviceWatcherTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DeviceWatcherTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: DeviceWatcherTriggerDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DeviceWatcherTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: &DeviceWatcherTriggerDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DeviceWatcherTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DeviceWatcherTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DeviceWatcherTriggerDetails {}
unsafe impl ::core::marker::Sync for DeviceWatcherTriggerDetails {}
#[doc = "*Required features: `Devices_Enumeration`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct EnclosureLocation(pub ::windows::core::IInspectable);
impl EnclosureLocation {
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn InDock(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn InLid(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn Panel(&self) -> ::windows::core::Result<Panel> {
        let this = self;
        unsafe {
            let mut result__: Panel = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Panel>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Enumeration`*"]
    pub fn RotationAngleInDegreesClockwise(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IEnclosureLocation2>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for EnclosureLocation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.EnclosureLocation;{42340a27-5810-459c-aabb-c65e1f813ecf})");
}
unsafe impl ::windows::core::Interface for EnclosureLocation {
    type Vtable = IEnclosureLocation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x42340a27_5810_459c_aabb_c65e1f813ecf);
}
impl ::windows::core::RuntimeName for EnclosureLocation {
    const NAME: &'static str = "Windows.Devices.Enumeration.EnclosureLocation";
}
impl ::core::convert::From<EnclosureLocation> for ::windows::core::IUnknown {
    fn from(value: EnclosureLocation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&EnclosureLocation> for ::windows::core::IUnknown {
    fn from(value: &EnclosureLocation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EnclosureLocation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a EnclosureLocation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<EnclosureLocation> for ::windows::core::IInspectable {
    fn from(value: EnclosureLocation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&EnclosureLocation> for ::windows::core::IInspectable {
    fn from(value: &EnclosureLocation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EnclosureLocation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a EnclosureLocation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for EnclosureLocation {}
unsafe impl ::core::marker::Sync for EnclosureLocation {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IDeviceAccessChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDeviceAccessChangedEventArgs {
    type Vtable = IDeviceAccessChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdeda0bcc_4f9d_4f58_9dba_a9bc800408d5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceAccessChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut DeviceAccessStatus) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDeviceAccessChangedEventArgs2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDeviceAccessChangedEventArgs2 {
    type Vtable = IDeviceAccessChangedEventArgs2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x82523262_934b_4b30_a178_adc39f2f2be3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceAccessChangedEventArgs2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDeviceAccessInformation(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDeviceAccessInformation {
    type Vtable = IDeviceAccessInformation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0baa9a73_6de5_4915_8ddd_9a0554a6f545);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceAccessInformation_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut DeviceAccessStatus) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDeviceAccessInformationStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDeviceAccessInformationStatics {
    type Vtable = IDeviceAccessInformationStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x574bd3d3_5f30_45cd_8a94_724fe5973084);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceAccessInformationStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceclassid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceclass: DeviceClass, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDeviceConnectionChangeTriggerDetails(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDeviceConnectionChangeTriggerDetails {
    type Vtable = IDeviceConnectionChangeTriggerDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb8578c0c_bbc1_484b_bffa_7b31dcc200b2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceConnectionChangeTriggerDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDeviceDisconnectButtonClickedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDeviceDisconnectButtonClickedEventArgs {
    type Vtable = IDeviceDisconnectButtonClickedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8e44b56d_f902_4a00_b536_f37992e6a2a7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceDisconnectButtonClickedEventArgs_abi(
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
pub struct IDeviceInformation(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDeviceInformation {
    type Vtable = IDeviceInformation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaba0fb95_4398_489d_8e44_e6130927011f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceInformation_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, updateinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDeviceInformation2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDeviceInformation2 {
    type Vtable = IDeviceInformation2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf156a638_7997_48d9_a10c_269d46533f48);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceInformation2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut DeviceInformationKind) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDeviceInformationCustomPairing(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDeviceInformationCustomPairing {
    type Vtable = IDeviceInformationCustomPairing_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x85138c02_4ee6_4914_8370_107a39144c0e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceInformationCustomPairing_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pairingkindssupported: DevicePairingKinds, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pairingkindssupported: DevicePairingKinds, minprotectionlevel: DevicePairingProtectionLevel, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pairingkindssupported: DevicePairingKinds, minprotectionlevel: DevicePairingProtectionLevel, devicepairingsettings: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDeviceInformationPairing(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDeviceInformationPairing {
    type Vtable = IDeviceInformationPairing_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c4769f5_f684_40d5_8469_e8dbaab70485);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceInformationPairing_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, minprotectionlevel: DevicePairingProtectionLevel, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDeviceInformationPairing2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDeviceInformationPairing2 {
    type Vtable = IDeviceInformationPairing2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf68612fd_0aee_4328_85cc_1c742bb1790d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceInformationPairing2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut DevicePairingProtectionLevel) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, minprotectionlevel: DevicePairingProtectionLevel, devicepairingsettings: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDeviceInformationPairingStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDeviceInformationPairingStatics {
    type Vtable = IDeviceInformationPairingStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe915c408_36d4_49a1_bf13_514173799b6b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceInformationPairingStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pairingkindssupported: DevicePairingKinds, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDeviceInformationPairingStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDeviceInformationPairingStatics2 {
    type Vtable = IDeviceInformationPairingStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x04de5372_b7b7_476b_a74f_c5836a704d98);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceInformationPairingStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pairingkindssupported: DevicePairingKinds, minprotectionlevel: DevicePairingProtectionLevel, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDeviceInformationStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDeviceInformationStatics {
    type Vtable = IDeviceInformationStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc17f100e_3a46_4a78_8013_769dc9b97390);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceInformationStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, additionalproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceclass: DeviceClass, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, aqsfilter: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, aqsfilter: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, additionalproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceclass: DeviceClass, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, aqsfilter: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, aqsfilter: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, additionalproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDeviceInformationStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDeviceInformationStatics2 {
    type Vtable = IDeviceInformationStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x493b4f34_a84f_45fd_9167_15d1cb1bd1f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceInformationStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceclass: DeviceClass, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, additionalproperties: ::windows::core::RawPtr, kind: DeviceInformationKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, aqsfilter: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, additionalproperties: ::windows::core::RawPtr, kind: DeviceInformationKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, aqsfilter: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, additionalproperties: ::windows::core::RawPtr, kind: DeviceInformationKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDeviceInformationUpdate(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDeviceInformationUpdate {
    type Vtable = IDeviceInformationUpdate_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f315305_d972_44b7_a37e_9e822c78213b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceInformationUpdate_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDeviceInformationUpdate2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDeviceInformationUpdate2 {
    type Vtable = IDeviceInformationUpdate2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d9d148c_a873_485e_baa6_aa620788e3cc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceInformationUpdate2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut DeviceInformationKind) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDevicePairingRequestedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDevicePairingRequestedEventArgs {
    type Vtable = IDevicePairingRequestedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf717fc56_de6b_487f_8376_0180aca69963);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDevicePairingRequestedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut DevicePairingKinds) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pin: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDevicePairingRequestedEventArgs2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDevicePairingRequestedEventArgs2 {
    type Vtable = IDevicePairingRequestedEventArgs2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc83752d9_e4d3_4db0_a360_a105e437dbdc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDevicePairingRequestedEventArgs2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Security_Credentials")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, passwordcredential: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDevicePairingResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDevicePairingResult {
    type Vtable = IDevicePairingResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x072b02bf_dd95_4025_9b37_de51adba37b7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDevicePairingResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut DevicePairingResultStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut DevicePairingProtectionLevel) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Devices_Enumeration`*"]
pub struct IDevicePairingSettings(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDevicePairingSettings {
    type Vtable = IDevicePairingSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x482cb27c_83bb_420e_be51_6602b222de54);
}
impl IDevicePairingSettings {}
unsafe impl ::windows::core::RuntimeType for IDevicePairingSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{482cb27c-83bb-420e-be51-6602b222de54}");
}
impl ::core::convert::From<IDevicePairingSettings> for ::windows::core::IUnknown {
    fn from(value: IDevicePairingSettings) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IDevicePairingSettings> for ::windows::core::IUnknown {
    fn from(value: &IDevicePairingSettings) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDevicePairingSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDevicePairingSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IDevicePairingSettings> for ::windows::core::IInspectable {
    fn from(value: IDevicePairingSettings) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDevicePairingSettings> for ::windows::core::IInspectable {
    fn from(value: &IDevicePairingSettings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IDevicePairingSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IDevicePairingSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDevicePairingSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDevicePicker(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDevicePicker {
    type Vtable = IDevicePicker_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x84997aa2_034a_4440_8813_7d0bd479bf5a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDevicePicker_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
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
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, selection: super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, selection: super::super::Foundation::Rect, placement: super::super::UI::Popups::Placement) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, selection: super::super::Foundation::Rect, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, selection: super::super::Foundation::Rect, placement: super::super::UI::Popups::Placement, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, device: ::windows::core::RawPtr, status: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, options: DevicePickerDisplayStatusOptions) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDevicePickerAppearance(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDevicePickerAppearance {
    type Vtable = IDevicePickerAppearance_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe69a12c6_e627_4ed8_9b6c_460af445e56d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDevicePickerAppearance_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDevicePickerFilter(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDevicePickerFilter {
    type Vtable = IDevicePickerFilter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x91db92a2_57cb_48f1_9b59_a59b7a1f02a2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDevicePickerFilter_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDeviceSelectedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDeviceSelectedEventArgs {
    type Vtable = IDeviceSelectedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x269edade_1d2f_4940_8402_4156b81d3c77);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceSelectedEventArgs_abi(
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
pub struct IDeviceUnpairingResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDeviceUnpairingResult {
    type Vtable = IDeviceUnpairingResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x66f44ad3_79d9_444b_92cf_a92ef72571c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceUnpairingResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut DeviceUnpairingResultStatus) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDeviceWatcher(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDeviceWatcher {
    type Vtable = IDeviceWatcher_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc9eab97d_8f6b_4f96_a9f4_abc814e22271);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceWatcher_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
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
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut DeviceWatcherStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDeviceWatcher2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDeviceWatcher2 {
    type Vtable = IDeviceWatcher2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xff08456e_ed14_49e9_9a69_8117c54ae971);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceWatcher2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "ApplicationModel_Background", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, requestedeventkinds: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Background", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDeviceWatcherEvent(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDeviceWatcherEvent {
    type Vtable = IDeviceWatcherEvent_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x74aa9c0b_1dbd_47fd_b635_3cc556d0ff8b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceWatcherEvent_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut DeviceWatcherEventKind) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDeviceWatcherTriggerDetails(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDeviceWatcherTriggerDetails {
    type Vtable = IDeviceWatcherTriggerDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38808119_4cb7_4e57_a56d_776d07cbfef9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceWatcherTriggerDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEnclosureLocation(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IEnclosureLocation {
    type Vtable = IEnclosureLocation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x42340a27_5810_459c_aabb_c65e1f813ecf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnclosureLocation_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut Panel) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEnclosureLocation2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IEnclosureLocation2 {
    type Vtable = IEnclosureLocation2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2885995b_e07d_485d_8a9e_bdf29aef4f66);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnclosureLocation2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
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
unsafe impl ::windows::core::Abi for Panel {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for Panel {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Enumeration.Panel;i4)");
}
impl ::windows::core::DefaultType for Panel {
    type DefaultType = Self;
}
