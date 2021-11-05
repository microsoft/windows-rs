#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Media_Casting`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct CastingConnection(pub ::windows::runtime::IInspectable);
impl CastingConnection {
    #[doc = "*Required features: `Media_Casting`*"]
    pub fn State(&self) -> ::windows::runtime::Result<CastingConnectionState> {
        let this = self;
        unsafe {
            let mut result__: CastingConnectionState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CastingConnectionState>(result__)
        }
    }
    #[doc = "*Required features: `Media_Casting`*"]
    pub fn Device(&self) -> ::windows::runtime::Result<CastingDevice> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CastingDevice>(result__)
        }
    }
    #[doc = "*Required features: `Media_Casting`*"]
    pub fn Source(&self) -> ::windows::runtime::Result<CastingSource> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CastingSource>(result__)
        }
    }
    #[doc = "*Required features: `Media_Casting`*"]
    pub fn SetSource<'a, Param0: ::windows::runtime::IntoParam<'a, CastingSource>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Casting`, `Foundation`*"]
    pub fn StateChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<CastingConnection, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Casting`, `Foundation`*"]
    pub fn RemoveStateChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Casting`, `Foundation`*"]
    pub fn ErrorOccurred<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<CastingConnection, CastingConnectionErrorOccurredEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Casting`, `Foundation`*"]
    pub fn RemoveErrorOccurred<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Casting`, `Foundation`*"]
    pub fn RequestStartCastingAsync<'a, Param0: ::windows::runtime::IntoParam<'a, CastingSource>>(&self, value: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<CastingConnectionErrorStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<CastingConnectionErrorStatus>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Casting`, `Foundation`*"]
    pub fn DisconnectAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<CastingConnectionErrorStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<CastingConnectionErrorStatus>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Casting`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CastingConnection {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Casting.CastingConnection;{cd951653-c2f1-4498-8b78-5fb4cd3640dd})");
}
unsafe impl ::windows::runtime::Interface for CastingConnection {
    type Vtable = ICastingConnection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3449099859, 49905, 17560, [139, 120, 95, 180, 205, 54, 64, 221]);
}
impl ::windows::runtime::RuntimeName for CastingConnection {
    const NAME: &'static str = "Windows.Media.Casting.CastingConnection";
}
impl ::std::convert::From<CastingConnection> for ::windows::runtime::IUnknown {
    fn from(value: CastingConnection) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&CastingConnection> for ::windows::runtime::IUnknown {
    fn from(value: &CastingConnection) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CastingConnection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a CastingConnection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<CastingConnection> for ::windows::runtime::IInspectable {
    fn from(value: CastingConnection) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CastingConnection> for ::windows::runtime::IInspectable {
    fn from(value: &CastingConnection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CastingConnection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CastingConnection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<CastingConnection> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: CastingConnection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&CastingConnection> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &CastingConnection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for CastingConnection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &CastingConnection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for CastingConnection {}
unsafe impl ::std::marker::Sync for CastingConnection {}
#[doc = "*Required features: `Media_Casting`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct CastingConnectionErrorOccurredEventArgs(pub ::windows::runtime::IInspectable);
impl CastingConnectionErrorOccurredEventArgs {
    #[doc = "*Required features: `Media_Casting`*"]
    pub fn ErrorStatus(&self) -> ::windows::runtime::Result<CastingConnectionErrorStatus> {
        let this = self;
        unsafe {
            let mut result__: CastingConnectionErrorStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CastingConnectionErrorStatus>(result__)
        }
    }
    #[doc = "*Required features: `Media_Casting`*"]
    pub fn Message(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CastingConnectionErrorOccurredEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Casting.CastingConnectionErrorOccurredEventArgs;{a7fb3c69-8719-4f00-81fb-961863c79a32})");
}
unsafe impl ::windows::runtime::Interface for CastingConnectionErrorOccurredEventArgs {
    type Vtable = ICastingConnectionErrorOccurredEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2818260073, 34585, 20224, [129, 251, 150, 24, 99, 199, 154, 50]);
}
impl ::windows::runtime::RuntimeName for CastingConnectionErrorOccurredEventArgs {
    const NAME: &'static str = "Windows.Media.Casting.CastingConnectionErrorOccurredEventArgs";
}
impl ::std::convert::From<CastingConnectionErrorOccurredEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: CastingConnectionErrorOccurredEventArgs) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&CastingConnectionErrorOccurredEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &CastingConnectionErrorOccurredEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CastingConnectionErrorOccurredEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a CastingConnectionErrorOccurredEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<CastingConnectionErrorOccurredEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: CastingConnectionErrorOccurredEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CastingConnectionErrorOccurredEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &CastingConnectionErrorOccurredEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CastingConnectionErrorOccurredEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CastingConnectionErrorOccurredEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CastingConnectionErrorOccurredEventArgs {}
unsafe impl ::std::marker::Sync for CastingConnectionErrorOccurredEventArgs {}
#[doc = "*Required features: `Media_Casting`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CastingConnectionErrorStatus(pub i32);
impl CastingConnectionErrorStatus {
    pub const Succeeded: CastingConnectionErrorStatus = CastingConnectionErrorStatus(0i32);
    pub const DeviceDidNotRespond: CastingConnectionErrorStatus = CastingConnectionErrorStatus(1i32);
    pub const DeviceError: CastingConnectionErrorStatus = CastingConnectionErrorStatus(2i32);
    pub const DeviceLocked: CastingConnectionErrorStatus = CastingConnectionErrorStatus(3i32);
    pub const ProtectedPlaybackFailed: CastingConnectionErrorStatus = CastingConnectionErrorStatus(4i32);
    pub const InvalidCastingSource: CastingConnectionErrorStatus = CastingConnectionErrorStatus(5i32);
    pub const Unknown: CastingConnectionErrorStatus = CastingConnectionErrorStatus(6i32);
}
impl ::std::convert::From<i32> for CastingConnectionErrorStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CastingConnectionErrorStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for CastingConnectionErrorStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Casting.CastingConnectionErrorStatus;i4)");
}
impl ::windows::runtime::DefaultType for CastingConnectionErrorStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media_Casting`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CastingConnectionState(pub i32);
impl CastingConnectionState {
    pub const Disconnected: CastingConnectionState = CastingConnectionState(0i32);
    pub const Connected: CastingConnectionState = CastingConnectionState(1i32);
    pub const Rendering: CastingConnectionState = CastingConnectionState(2i32);
    pub const Disconnecting: CastingConnectionState = CastingConnectionState(3i32);
    pub const Connecting: CastingConnectionState = CastingConnectionState(4i32);
}
impl ::std::convert::From<i32> for CastingConnectionState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CastingConnectionState {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for CastingConnectionState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Casting.CastingConnectionState;i4)");
}
impl ::windows::runtime::DefaultType for CastingConnectionState {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media_Casting`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct CastingDevice(pub ::windows::runtime::IInspectable);
impl CastingDevice {
    #[doc = "*Required features: `Media_Casting`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_Casting`*"]
    pub fn FriendlyName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Media_Casting`, `Storage_Streams`*"]
    pub fn Icon(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IRandomAccessStreamWithContentType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IRandomAccessStreamWithContentType>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Casting`, `Foundation`*"]
    pub fn GetSupportedCastingPlaybackTypesAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<CastingPlaybackTypes>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<CastingPlaybackTypes>>(result__)
        }
    }
    #[doc = "*Required features: `Media_Casting`*"]
    pub fn CreateCastingConnection(&self) -> ::windows::runtime::Result<CastingConnection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CastingConnection>(result__)
        }
    }
    #[doc = "*Required features: `Media_Casting`*"]
    pub fn GetDeviceSelector(r#type: CastingPlaybackTypes) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::ICastingDeviceStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), r#type, &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Casting`, `Foundation`*"]
    pub fn GetDeviceSelectorFromCastingSourceAsync<'a, Param0: ::windows::runtime::IntoParam<'a, CastingSource>>(castingsource: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<::windows::runtime::HSTRING>> {
        Self::ICastingDeviceStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), castingsource.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::runtime::HSTRING>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Casting`, `Foundation`*"]
    pub fn FromIdAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(value: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<CastingDevice>> {
        Self::ICastingDeviceStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<CastingDevice>>(result__)
        })
    }
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))]
    #[doc = "*Required features: `Media_Casting`, `Devices_Enumeration`, `Foundation`*"]
    pub fn DeviceInfoSupportsCastingAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Devices::Enumeration::DeviceInformation>>(device: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::ICastingDeviceStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), device.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    pub fn ICastingDeviceStatics<R, F: FnOnce(&ICastingDeviceStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<CastingDevice, ICastingDeviceStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CastingDevice {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Casting.CastingDevice;{de721c83-4a43-4ad1-a6d2-2492a796c3f2})");
}
unsafe impl ::windows::runtime::Interface for CastingDevice {
    type Vtable = ICastingDevice_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3732020355, 19011, 19153, [166, 210, 36, 146, 167, 150, 195, 242]);
}
impl ::windows::runtime::RuntimeName for CastingDevice {
    const NAME: &'static str = "Windows.Media.Casting.CastingDevice";
}
impl ::std::convert::From<CastingDevice> for ::windows::runtime::IUnknown {
    fn from(value: CastingDevice) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&CastingDevice> for ::windows::runtime::IUnknown {
    fn from(value: &CastingDevice) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CastingDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a CastingDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<CastingDevice> for ::windows::runtime::IInspectable {
    fn from(value: CastingDevice) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CastingDevice> for ::windows::runtime::IInspectable {
    fn from(value: &CastingDevice) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CastingDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CastingDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CastingDevice {}
unsafe impl ::std::marker::Sync for CastingDevice {}
#[doc = "*Required features: `Media_Casting`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct CastingDevicePicker(pub ::windows::runtime::IInspectable);
impl CastingDevicePicker {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<CastingDevicePicker, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Media_Casting`*"]
    pub fn Filter(&self) -> ::windows::runtime::Result<CastingDevicePickerFilter> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CastingDevicePickerFilter>(result__)
        }
    }
    #[cfg(feature = "Devices_Enumeration")]
    #[doc = "*Required features: `Media_Casting`, `Devices_Enumeration`*"]
    pub fn Appearance(&self) -> ::windows::runtime::Result<super::super::Devices::Enumeration::DevicePickerAppearance> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Enumeration::DevicePickerAppearance>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Casting`, `Foundation`*"]
    pub fn CastingDeviceSelected<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<CastingDevicePicker, CastingDeviceSelectedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Casting`, `Foundation`*"]
    pub fn RemoveCastingDeviceSelected<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Casting`, `Foundation`*"]
    pub fn CastingDevicePickerDismissed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<CastingDevicePicker, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Casting`, `Foundation`*"]
    pub fn RemoveCastingDevicePickerDismissed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Casting`, `Foundation`*"]
    pub fn Show<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Rect>>(&self, selection: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), selection.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    #[doc = "*Required features: `Media_Casting`, `Foundation`, `UI_Popups`*"]
    pub fn ShowWithPlacement<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Rect>>(&self, selection: Param0, preferredplacement: super::super::UI::Popups::Placement) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), selection.into_param().abi(), preferredplacement).ok() }
    }
    #[doc = "*Required features: `Media_Casting`*"]
    pub fn Hide(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CastingDevicePicker {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Casting.CastingDevicePicker;{dcd39924-0591-49be-aacb-4b82ee756a95})");
}
unsafe impl ::windows::runtime::Interface for CastingDevicePicker {
    type Vtable = ICastingDevicePicker_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3704854820, 1425, 18878, [170, 203, 75, 130, 238, 117, 106, 149]);
}
impl ::windows::runtime::RuntimeName for CastingDevicePicker {
    const NAME: &'static str = "Windows.Media.Casting.CastingDevicePicker";
}
impl ::std::convert::From<CastingDevicePicker> for ::windows::runtime::IUnknown {
    fn from(value: CastingDevicePicker) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&CastingDevicePicker> for ::windows::runtime::IUnknown {
    fn from(value: &CastingDevicePicker) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CastingDevicePicker {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a CastingDevicePicker {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<CastingDevicePicker> for ::windows::runtime::IInspectable {
    fn from(value: CastingDevicePicker) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CastingDevicePicker> for ::windows::runtime::IInspectable {
    fn from(value: &CastingDevicePicker) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CastingDevicePicker {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CastingDevicePicker {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CastingDevicePicker {}
unsafe impl ::std::marker::Sync for CastingDevicePicker {}
#[doc = "*Required features: `Media_Casting`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct CastingDevicePickerFilter(pub ::windows::runtime::IInspectable);
impl CastingDevicePickerFilter {
    #[doc = "*Required features: `Media_Casting`*"]
    pub fn SupportsAudio(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_Casting`*"]
    pub fn SetSupportsAudio(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Casting`*"]
    pub fn SupportsVideo(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_Casting`*"]
    pub fn SetSupportsVideo(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Casting`*"]
    pub fn SupportsPictures(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_Casting`*"]
    pub fn SetSupportsPictures(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_Casting`, `Foundation_Collections`*"]
    pub fn SupportedCastingSources(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<CastingSource>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<CastingSource>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CastingDevicePickerFilter {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Casting.CastingDevicePickerFilter;{be8c619c-b563-4354-ae33-9fdaad8c6291})");
}
unsafe impl ::windows::runtime::Interface for CastingDevicePickerFilter {
    type Vtable = ICastingDevicePickerFilter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3196871068, 46435, 17236, [174, 51, 159, 218, 173, 140, 98, 145]);
}
impl ::windows::runtime::RuntimeName for CastingDevicePickerFilter {
    const NAME: &'static str = "Windows.Media.Casting.CastingDevicePickerFilter";
}
impl ::std::convert::From<CastingDevicePickerFilter> for ::windows::runtime::IUnknown {
    fn from(value: CastingDevicePickerFilter) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&CastingDevicePickerFilter> for ::windows::runtime::IUnknown {
    fn from(value: &CastingDevicePickerFilter) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CastingDevicePickerFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a CastingDevicePickerFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<CastingDevicePickerFilter> for ::windows::runtime::IInspectable {
    fn from(value: CastingDevicePickerFilter) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CastingDevicePickerFilter> for ::windows::runtime::IInspectable {
    fn from(value: &CastingDevicePickerFilter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CastingDevicePickerFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CastingDevicePickerFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CastingDevicePickerFilter {}
unsafe impl ::std::marker::Sync for CastingDevicePickerFilter {}
#[doc = "*Required features: `Media_Casting`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct CastingDeviceSelectedEventArgs(pub ::windows::runtime::IInspectable);
impl CastingDeviceSelectedEventArgs {
    #[doc = "*Required features: `Media_Casting`*"]
    pub fn SelectedCastingDevice(&self) -> ::windows::runtime::Result<CastingDevice> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CastingDevice>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CastingDeviceSelectedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Casting.CastingDeviceSelectedEventArgs;{dc439e86-dd57-4d0d-9400-af45e4fb3663})");
}
unsafe impl ::windows::runtime::Interface for CastingDeviceSelectedEventArgs {
    type Vtable = ICastingDeviceSelectedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3695419014, 56663, 19725, [148, 0, 175, 69, 228, 251, 54, 99]);
}
impl ::windows::runtime::RuntimeName for CastingDeviceSelectedEventArgs {
    const NAME: &'static str = "Windows.Media.Casting.CastingDeviceSelectedEventArgs";
}
impl ::std::convert::From<CastingDeviceSelectedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: CastingDeviceSelectedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&CastingDeviceSelectedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &CastingDeviceSelectedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CastingDeviceSelectedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a CastingDeviceSelectedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<CastingDeviceSelectedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: CastingDeviceSelectedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CastingDeviceSelectedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &CastingDeviceSelectedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CastingDeviceSelectedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CastingDeviceSelectedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CastingDeviceSelectedEventArgs {}
unsafe impl ::std::marker::Sync for CastingDeviceSelectedEventArgs {}
#[doc = "*Required features: `Media_Casting`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CastingPlaybackTypes(pub u32);
impl CastingPlaybackTypes {
    pub const None: CastingPlaybackTypes = CastingPlaybackTypes(0u32);
    pub const Audio: CastingPlaybackTypes = CastingPlaybackTypes(1u32);
    pub const Video: CastingPlaybackTypes = CastingPlaybackTypes(2u32);
    pub const Picture: CastingPlaybackTypes = CastingPlaybackTypes(4u32);
}
impl ::std::convert::From<u32> for CastingPlaybackTypes {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CastingPlaybackTypes {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for CastingPlaybackTypes {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Casting.CastingPlaybackTypes;u4)");
}
impl ::windows::runtime::DefaultType for CastingPlaybackTypes {
    type DefaultType = Self;
}
impl ::std::ops::BitOr for CastingPlaybackTypes {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for CastingPlaybackTypes {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for CastingPlaybackTypes {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for CastingPlaybackTypes {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for CastingPlaybackTypes {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Media_Casting`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct CastingSource(pub ::windows::runtime::IInspectable);
impl CastingSource {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Casting`, `Foundation`*"]
    pub fn PreferredSourceUri(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Casting`, `Foundation`*"]
    pub fn SetPreferredSourceUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CastingSource {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Casting.CastingSource;{f429ea72-3467-47e6-a027-522923e9d727})");
}
unsafe impl ::windows::runtime::Interface for CastingSource {
    type Vtable = ICastingSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4096387698, 13415, 18406, [160, 39, 82, 41, 35, 233, 215, 39]);
}
impl ::windows::runtime::RuntimeName for CastingSource {
    const NAME: &'static str = "Windows.Media.Casting.CastingSource";
}
impl ::std::convert::From<CastingSource> for ::windows::runtime::IUnknown {
    fn from(value: CastingSource) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&CastingSource> for ::windows::runtime::IUnknown {
    fn from(value: &CastingSource) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CastingSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a CastingSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<CastingSource> for ::windows::runtime::IInspectable {
    fn from(value: CastingSource) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CastingSource> for ::windows::runtime::IInspectable {
    fn from(value: &CastingSource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CastingSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CastingSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CastingSource {}
unsafe impl ::std::marker::Sync for CastingSource {}
#[repr(transparent)]
#[doc(hidden)]
pub struct ICastingConnection(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICastingConnection {
    type Vtable = ICastingConnection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3449099859, 49905, 17560, [139, 120, 95, 180, 205, 54, 64, 221]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICastingConnection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut CastingConnectionState) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICastingConnectionErrorOccurredEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICastingConnectionErrorOccurredEventArgs {
    type Vtable = ICastingConnectionErrorOccurredEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2818260073, 34585, 20224, [129, 251, 150, 24, 99, 199, 154, 50]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICastingConnectionErrorOccurredEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut CastingConnectionErrorStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICastingDevice(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICastingDevice {
    type Vtable = ICastingDevice_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3732020355, 19011, 19153, [166, 210, 36, 146, 167, 150, 195, 242]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICastingDevice_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICastingDevicePicker(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICastingDevicePicker {
    type Vtable = ICastingDevicePicker_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3704854820, 1425, 18878, [170, 203, 75, 130, 238, 117, 106, 149]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICastingDevicePicker_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Enumeration")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))] usize,
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
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICastingDevicePickerFilter(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICastingDevicePickerFilter {
    type Vtable = ICastingDevicePickerFilter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3196871068, 46435, 17236, [174, 51, 159, 218, 173, 140, 98, 145]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICastingDevicePickerFilter_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICastingDeviceSelectedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICastingDeviceSelectedEventArgs {
    type Vtable = ICastingDeviceSelectedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3695419014, 56663, 19725, [148, 0, 175, 69, 228, 251, 54, 99]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICastingDeviceSelectedEventArgs_abi(
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
pub struct ICastingDeviceStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICastingDeviceStatics {
    type Vtable = ICastingDeviceStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3889780951, 19731, 16951, [163, 101, 76, 79, 106, 76, 253, 47]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICastingDeviceStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: CastingPlaybackTypes, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, castingsource: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, device: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Devices_Enumeration", feature = "Foundation")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICastingSource(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICastingSource {
    type Vtable = ICastingSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4096387698, 13415, 18406, [160, 39, 82, 41, 35, 233, 215, 39]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICastingSource_abi(
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
);
