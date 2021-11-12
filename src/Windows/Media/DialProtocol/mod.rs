#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DialApp(pub ::windows::core::IInspectable);
impl DialApp {
    pub fn AppName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestLaunchAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, appargument: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DialAppLaunchResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), appargument.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<DialAppLaunchResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn StopAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DialAppStopResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<DialAppStopResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetAppStateAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DialAppStateDetails>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<DialAppStateDetails>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for DialApp {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.DialProtocol.DialApp;{555ffbd3-45b7-49f3-bbd7-302db6084646})");
}
unsafe impl ::windows::core::Interface for DialApp {
    type Vtable = IDialApp_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x555ffbd3_45b7_49f3_bbd7_302db6084646);
}
impl ::windows::core::RuntimeName for DialApp {
    const NAME: &'static str = "Windows.Media.DialProtocol.DialApp";
}
impl ::core::convert::From<DialApp> for ::windows::core::IUnknown {
    fn from(value: DialApp) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DialApp> for ::windows::core::IUnknown {
    fn from(value: &DialApp) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DialApp {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DialApp {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DialApp> for ::windows::core::IInspectable {
    fn from(value: DialApp) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DialApp> for ::windows::core::IInspectable {
    fn from(value: &DialApp) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DialApp {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DialApp {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DialApp {}
unsafe impl ::core::marker::Sync for DialApp {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DialAppLaunchResult(pub i32);
impl DialAppLaunchResult {
    pub const Launched: DialAppLaunchResult = DialAppLaunchResult(0i32);
    pub const FailedToLaunch: DialAppLaunchResult = DialAppLaunchResult(1i32);
    pub const NotFound: DialAppLaunchResult = DialAppLaunchResult(2i32);
    pub const NetworkFailure: DialAppLaunchResult = DialAppLaunchResult(3i32);
}
impl ::core::convert::From<i32> for DialAppLaunchResult {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DialAppLaunchResult {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DialAppLaunchResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.DialProtocol.DialAppLaunchResult;i4)");
}
impl ::windows::core::DefaultType for DialAppLaunchResult {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DialAppState(pub i32);
impl DialAppState {
    pub const Unknown: DialAppState = DialAppState(0i32);
    pub const Stopped: DialAppState = DialAppState(1i32);
    pub const Running: DialAppState = DialAppState(2i32);
    pub const NetworkFailure: DialAppState = DialAppState(3i32);
}
impl ::core::convert::From<i32> for DialAppState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DialAppState {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DialAppState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.DialProtocol.DialAppState;i4)");
}
impl ::windows::core::DefaultType for DialAppState {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DialAppStateDetails(pub ::windows::core::IInspectable);
impl DialAppStateDetails {
    pub fn State(&self) -> ::windows::core::Result<DialAppState> {
        let this = self;
        unsafe {
            let mut result__: DialAppState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DialAppState>(result__)
        }
    }
    pub fn FullXml(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for DialAppStateDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.DialProtocol.DialAppStateDetails;{ddc4a4a1-f5de-400d-bea4-8c8466bb2961})");
}
unsafe impl ::windows::core::Interface for DialAppStateDetails {
    type Vtable = IDialAppStateDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xddc4a4a1_f5de_400d_bea4_8c8466bb2961);
}
impl ::windows::core::RuntimeName for DialAppStateDetails {
    const NAME: &'static str = "Windows.Media.DialProtocol.DialAppStateDetails";
}
impl ::core::convert::From<DialAppStateDetails> for ::windows::core::IUnknown {
    fn from(value: DialAppStateDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DialAppStateDetails> for ::windows::core::IUnknown {
    fn from(value: &DialAppStateDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DialAppStateDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DialAppStateDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DialAppStateDetails> for ::windows::core::IInspectable {
    fn from(value: DialAppStateDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DialAppStateDetails> for ::windows::core::IInspectable {
    fn from(value: &DialAppStateDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DialAppStateDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DialAppStateDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DialAppStateDetails {}
unsafe impl ::core::marker::Sync for DialAppStateDetails {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DialAppStopResult(pub i32);
impl DialAppStopResult {
    pub const Stopped: DialAppStopResult = DialAppStopResult(0i32);
    pub const StopFailed: DialAppStopResult = DialAppStopResult(1i32);
    pub const OperationNotSupported: DialAppStopResult = DialAppStopResult(2i32);
    pub const NetworkFailure: DialAppStopResult = DialAppStopResult(3i32);
}
impl ::core::convert::From<i32> for DialAppStopResult {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DialAppStopResult {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DialAppStopResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.DialProtocol.DialAppStopResult;i4)");
}
impl ::windows::core::DefaultType for DialAppStopResult {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DialDevice(pub ::windows::core::IInspectable);
impl DialDevice {
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn GetDialApp<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, appname: Param0) -> ::windows::core::Result<DialApp> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), appname.into_param().abi(), &mut result__).from_abi::<DialApp>(result__)
        }
    }
    pub fn GetDeviceSelector<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(appname: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IDialDeviceStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), appname.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(value: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DialDevice>> {
        Self::IDialDeviceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<DialDevice>>(result__)
        })
    }
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))]
    pub fn DeviceInfoSupportsDialAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Devices::Enumeration::DeviceInformation>>(device: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IDialDeviceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), device.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    pub fn FriendlyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IDialDevice2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Thumbnail(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = &::windows::core::Interface::cast::<IDialDevice2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IRandomAccessStreamReference>(result__)
        }
    }
    pub fn IDialDeviceStatics<R, F: FnOnce(&IDialDeviceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DialDevice, IDialDeviceStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for DialDevice {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.DialProtocol.DialDevice;{fff0edaf-759f-41d2-a20a-7f29ce0b3784})");
}
unsafe impl ::windows::core::Interface for DialDevice {
    type Vtable = IDialDevice_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfff0edaf_759f_41d2_a20a_7f29ce0b3784);
}
impl ::windows::core::RuntimeName for DialDevice {
    const NAME: &'static str = "Windows.Media.DialProtocol.DialDevice";
}
impl ::core::convert::From<DialDevice> for ::windows::core::IUnknown {
    fn from(value: DialDevice) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DialDevice> for ::windows::core::IUnknown {
    fn from(value: &DialDevice) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DialDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DialDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DialDevice> for ::windows::core::IInspectable {
    fn from(value: DialDevice) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DialDevice> for ::windows::core::IInspectable {
    fn from(value: &DialDevice) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DialDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DialDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DialDevice {}
unsafe impl ::core::marker::Sync for DialDevice {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DialDeviceDisplayStatus(pub i32);
impl DialDeviceDisplayStatus {
    pub const None: DialDeviceDisplayStatus = DialDeviceDisplayStatus(0i32);
    pub const Connecting: DialDeviceDisplayStatus = DialDeviceDisplayStatus(1i32);
    pub const Connected: DialDeviceDisplayStatus = DialDeviceDisplayStatus(2i32);
    pub const Disconnecting: DialDeviceDisplayStatus = DialDeviceDisplayStatus(3i32);
    pub const Disconnected: DialDeviceDisplayStatus = DialDeviceDisplayStatus(4i32);
    pub const Error: DialDeviceDisplayStatus = DialDeviceDisplayStatus(5i32);
}
impl ::core::convert::From<i32> for DialDeviceDisplayStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DialDeviceDisplayStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DialDeviceDisplayStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.DialProtocol.DialDeviceDisplayStatus;i4)");
}
impl ::windows::core::DefaultType for DialDeviceDisplayStatus {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DialDevicePicker(pub ::windows::core::IInspectable);
impl DialDevicePicker {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DialDevicePicker, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Filter(&self) -> ::windows::core::Result<DialDevicePickerFilter> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DialDevicePickerFilter>(result__)
        }
    }
    #[cfg(feature = "Devices_Enumeration")]
    pub fn Appearance(&self) -> ::windows::core::Result<super::super::Devices::Enumeration::DevicePickerAppearance> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Enumeration::DevicePickerAppearance>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn DialDeviceSelected<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<DialDevicePicker, DialDeviceSelectedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveDialDeviceSelected<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn DisconnectButtonClicked<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<DialDevicePicker, DialDisconnectButtonClickedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveDisconnectButtonClicked<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn DialDevicePickerDismissed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<DialDevicePicker, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveDialDevicePickerDismissed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Show<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>>(&self, selection: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), selection.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub fn ShowWithPlacement<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>>(&self, selection: Param0, preferredplacement: super::super::UI::Popups::Placement) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), selection.into_param().abi(), preferredplacement).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PickSingleDialDeviceAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>>(&self, selection: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DialDevice>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), selection.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<DialDevice>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub fn PickSingleDialDeviceAsyncWithPlacement<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>>(&self, selection: Param0, preferredplacement: super::super::UI::Popups::Placement) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DialDevice>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), selection.into_param().abi(), preferredplacement, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<DialDevice>>(result__)
        }
    }
    pub fn Hide(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn SetDisplayStatus<'a, Param0: ::windows::core::IntoParam<'a, DialDevice>>(&self, device: Param0, status: DialDeviceDisplayStatus) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), device.into_param().abi(), status).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for DialDevicePicker {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.DialProtocol.DialDevicePicker;{ba7e520a-ff59-4f4b-bdac-d89f495ad6e1})");
}
unsafe impl ::windows::core::Interface for DialDevicePicker {
    type Vtable = IDialDevicePicker_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xba7e520a_ff59_4f4b_bdac_d89f495ad6e1);
}
impl ::windows::core::RuntimeName for DialDevicePicker {
    const NAME: &'static str = "Windows.Media.DialProtocol.DialDevicePicker";
}
impl ::core::convert::From<DialDevicePicker> for ::windows::core::IUnknown {
    fn from(value: DialDevicePicker) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DialDevicePicker> for ::windows::core::IUnknown {
    fn from(value: &DialDevicePicker) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DialDevicePicker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DialDevicePicker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DialDevicePicker> for ::windows::core::IInspectable {
    fn from(value: DialDevicePicker) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DialDevicePicker> for ::windows::core::IInspectable {
    fn from(value: &DialDevicePicker) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DialDevicePicker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DialDevicePicker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DialDevicePicker {}
unsafe impl ::core::marker::Sync for DialDevicePicker {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DialDevicePickerFilter(pub ::windows::core::IInspectable);
impl DialDevicePickerFilter {
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedAppNames(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for DialDevicePickerFilter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.DialProtocol.DialDevicePickerFilter;{c17c93ba-86c0-485d-b8d6-0f9a8f641590})");
}
unsafe impl ::windows::core::Interface for DialDevicePickerFilter {
    type Vtable = IDialDevicePickerFilter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc17c93ba_86c0_485d_b8d6_0f9a8f641590);
}
impl ::windows::core::RuntimeName for DialDevicePickerFilter {
    const NAME: &'static str = "Windows.Media.DialProtocol.DialDevicePickerFilter";
}
impl ::core::convert::From<DialDevicePickerFilter> for ::windows::core::IUnknown {
    fn from(value: DialDevicePickerFilter) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DialDevicePickerFilter> for ::windows::core::IUnknown {
    fn from(value: &DialDevicePickerFilter) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DialDevicePickerFilter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DialDevicePickerFilter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DialDevicePickerFilter> for ::windows::core::IInspectable {
    fn from(value: DialDevicePickerFilter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DialDevicePickerFilter> for ::windows::core::IInspectable {
    fn from(value: &DialDevicePickerFilter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DialDevicePickerFilter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DialDevicePickerFilter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DialDevicePickerFilter {}
unsafe impl ::core::marker::Sync for DialDevicePickerFilter {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DialDeviceSelectedEventArgs(pub ::windows::core::IInspectable);
impl DialDeviceSelectedEventArgs {
    pub fn SelectedDialDevice(&self) -> ::windows::core::Result<DialDevice> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DialDevice>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for DialDeviceSelectedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.DialProtocol.DialDeviceSelectedEventArgs;{480b92ad-ac76-47eb-9c06-a19304da0247})");
}
unsafe impl ::windows::core::Interface for DialDeviceSelectedEventArgs {
    type Vtable = IDialDeviceSelectedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x480b92ad_ac76_47eb_9c06_a19304da0247);
}
impl ::windows::core::RuntimeName for DialDeviceSelectedEventArgs {
    const NAME: &'static str = "Windows.Media.DialProtocol.DialDeviceSelectedEventArgs";
}
impl ::core::convert::From<DialDeviceSelectedEventArgs> for ::windows::core::IUnknown {
    fn from(value: DialDeviceSelectedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DialDeviceSelectedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &DialDeviceSelectedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DialDeviceSelectedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DialDeviceSelectedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DialDeviceSelectedEventArgs> for ::windows::core::IInspectable {
    fn from(value: DialDeviceSelectedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DialDeviceSelectedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &DialDeviceSelectedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DialDeviceSelectedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DialDeviceSelectedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DialDeviceSelectedEventArgs {}
unsafe impl ::core::marker::Sync for DialDeviceSelectedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DialDisconnectButtonClickedEventArgs(pub ::windows::core::IInspectable);
impl DialDisconnectButtonClickedEventArgs {
    pub fn Device(&self) -> ::windows::core::Result<DialDevice> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DialDevice>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for DialDisconnectButtonClickedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.DialProtocol.DialDisconnectButtonClickedEventArgs;{52765152-9c81-4e55-adc2-0ebe99cde3b6})");
}
unsafe impl ::windows::core::Interface for DialDisconnectButtonClickedEventArgs {
    type Vtable = IDialDisconnectButtonClickedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x52765152_9c81_4e55_adc2_0ebe99cde3b6);
}
impl ::windows::core::RuntimeName for DialDisconnectButtonClickedEventArgs {
    const NAME: &'static str = "Windows.Media.DialProtocol.DialDisconnectButtonClickedEventArgs";
}
impl ::core::convert::From<DialDisconnectButtonClickedEventArgs> for ::windows::core::IUnknown {
    fn from(value: DialDisconnectButtonClickedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DialDisconnectButtonClickedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &DialDisconnectButtonClickedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DialDisconnectButtonClickedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DialDisconnectButtonClickedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DialDisconnectButtonClickedEventArgs> for ::windows::core::IInspectable {
    fn from(value: DialDisconnectButtonClickedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DialDisconnectButtonClickedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &DialDisconnectButtonClickedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DialDisconnectButtonClickedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DialDisconnectButtonClickedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DialDisconnectButtonClickedEventArgs {}
unsafe impl ::core::marker::Sync for DialDisconnectButtonClickedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DialReceiverApp(pub ::windows::core::IInspectable);
impl DialReceiverApp {
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetAdditionalDataAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn SetAdditionalDataAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>>>(&self, additionaldata: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), additionaldata.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn Current() -> ::windows::core::Result<DialReceiverApp> {
        Self::IDialReceiverAppStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DialReceiverApp>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn GetUniqueDeviceNameAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        let this = &::windows::core::Interface::cast::<IDialReceiverApp2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        }
    }
    pub fn IDialReceiverAppStatics<R, F: FnOnce(&IDialReceiverAppStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DialReceiverApp, IDialReceiverAppStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for DialReceiverApp {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.DialProtocol.DialReceiverApp;{fd3e7c57-5045-470e-b304-4dd9b13e7d11})");
}
unsafe impl ::windows::core::Interface for DialReceiverApp {
    type Vtable = IDialReceiverApp_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfd3e7c57_5045_470e_b304_4dd9b13e7d11);
}
impl ::windows::core::RuntimeName for DialReceiverApp {
    const NAME: &'static str = "Windows.Media.DialProtocol.DialReceiverApp";
}
impl ::core::convert::From<DialReceiverApp> for ::windows::core::IUnknown {
    fn from(value: DialReceiverApp) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DialReceiverApp> for ::windows::core::IUnknown {
    fn from(value: &DialReceiverApp) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DialReceiverApp {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DialReceiverApp {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DialReceiverApp> for ::windows::core::IInspectable {
    fn from(value: DialReceiverApp) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DialReceiverApp> for ::windows::core::IInspectable {
    fn from(value: &DialReceiverApp) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DialReceiverApp {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DialReceiverApp {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DialReceiverApp {}
unsafe impl ::core::marker::Sync for DialReceiverApp {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IDialApp(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDialApp {
    type Vtable = IDialApp_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x555ffbd3_45b7_49f3_bbd7_302db6084646);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDialApp_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, appargument: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDialAppStateDetails(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDialAppStateDetails {
    type Vtable = IDialAppStateDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xddc4a4a1_f5de_400d_bea4_8c8466bb2961);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDialAppStateDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut DialAppState) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDialDevice(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDialDevice {
    type Vtable = IDialDevice_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfff0edaf_759f_41d2_a20a_7f29ce0b3784);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDialDevice_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, appname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDialDevice2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDialDevice2 {
    type Vtable = IDialDevice2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbab7f3d5_5bfb_4eba_8b32_b57c5c5ee5c9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDialDevice2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDialDevicePicker(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDialDevicePicker {
    type Vtable = IDialDevicePicker_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xba7e520a_ff59_4f4b_bdac_d89f495ad6e1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDialDevicePicker_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Enumeration")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))] usize,
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
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, selection: super::super::Foundation::Rect, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, device: ::windows::core::RawPtr, status: DialDeviceDisplayStatus) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDialDevicePickerFilter(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDialDevicePickerFilter {
    type Vtable = IDialDevicePickerFilter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc17c93ba_86c0_485d_b8d6_0f9a8f641590);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDialDevicePickerFilter_abi(
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
pub struct IDialDeviceSelectedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDialDeviceSelectedEventArgs {
    type Vtable = IDialDeviceSelectedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x480b92ad_ac76_47eb_9c06_a19304da0247);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDialDeviceSelectedEventArgs_abi(
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
pub struct IDialDeviceStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDialDeviceStatics {
    type Vtable = IDialDeviceStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa69cc95_01f8_4758_8461_2bbd1cdc3cf3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDialDeviceStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, appname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, device: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Enumeration", feature = "Foundation")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDialDisconnectButtonClickedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDialDisconnectButtonClickedEventArgs {
    type Vtable = IDialDisconnectButtonClickedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x52765152_9c81_4e55_adc2_0ebe99cde3b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDialDisconnectButtonClickedEventArgs_abi(
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
pub struct IDialReceiverApp(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDialReceiverApp {
    type Vtable = IDialReceiverApp_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfd3e7c57_5045_470e_b304_4dd9b13e7d11);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDialReceiverApp_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, additionaldata: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDialReceiverApp2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDialReceiverApp2 {
    type Vtable = IDialReceiverApp2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x530c5805_9130_42ac_a504_1977dcb2ea8a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDialReceiverApp2_abi(
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
pub struct IDialReceiverAppStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDialReceiverAppStatics {
    type Vtable = IDialReceiverAppStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x53183a3c_4c36_4d02_b28a_f2a9da38ec52);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDialReceiverAppStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
