#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GazeDeviceConfigurationStatePreview(pub i32);
impl GazeDeviceConfigurationStatePreview {
    pub const Unknown: GazeDeviceConfigurationStatePreview = GazeDeviceConfigurationStatePreview(0i32);
    pub const Ready: GazeDeviceConfigurationStatePreview = GazeDeviceConfigurationStatePreview(1i32);
    pub const Configuring: GazeDeviceConfigurationStatePreview = GazeDeviceConfigurationStatePreview(2i32);
    pub const ScreenSetupNeeded: GazeDeviceConfigurationStatePreview = GazeDeviceConfigurationStatePreview(3i32);
    pub const UserCalibrationNeeded: GazeDeviceConfigurationStatePreview = GazeDeviceConfigurationStatePreview(4i32);
}
impl ::core::convert::From<i32> for GazeDeviceConfigurationStatePreview {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for GazeDeviceConfigurationStatePreview {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for GazeDeviceConfigurationStatePreview {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Input.Preview.GazeDeviceConfigurationStatePreview;i4)");
}
impl ::windows::core::DefaultType for GazeDeviceConfigurationStatePreview {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GazeDevicePreview(pub ::windows::core::IInspectable);
impl GazeDevicePreview {
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn CanTrackEyes(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn CanTrackHead(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn ConfigurationState(&self) -> ::windows::core::Result<GazeDeviceConfigurationStatePreview> {
        let this = self;
        unsafe {
            let mut result__: GazeDeviceConfigurationStatePreview = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GazeDeviceConfigurationStatePreview>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestCalibrationAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(all(feature = "Devices_HumanInterfaceDevice", feature = "Foundation_Collections"))]
    pub fn GetNumericControlDescriptions(&self, usagepage: u16, usageid: u16) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::super::HumanInterfaceDevice::HidNumericControlDescription>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), usagepage, usageid, &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<super::super::HumanInterfaceDevice::HidNumericControlDescription>>(result__)
        }
    }
    #[cfg(all(feature = "Devices_HumanInterfaceDevice", feature = "Foundation_Collections"))]
    pub fn GetBooleanControlDescriptions(&self, usagepage: u16, usageid: u16) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::super::HumanInterfaceDevice::HidBooleanControlDescription>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), usagepage, usageid, &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<super::super::HumanInterfaceDevice::HidBooleanControlDescription>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for GazeDevicePreview {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Input.Preview.GazeDevicePreview;{e79e7ee9-b389-11e7-b201-c8d3ffb75721})");
}
unsafe impl ::windows::core::Interface for GazeDevicePreview {
    type Vtable = IGazeDevicePreview_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe79e7ee9_b389_11e7_b201_c8d3ffb75721);
}
impl ::windows::core::RuntimeName for GazeDevicePreview {
    const NAME: &'static str = "Windows.Devices.Input.Preview.GazeDevicePreview";
}
impl ::core::convert::From<GazeDevicePreview> for ::windows::core::IUnknown {
    fn from(value: GazeDevicePreview) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GazeDevicePreview> for ::windows::core::IUnknown {
    fn from(value: &GazeDevicePreview) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GazeDevicePreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GazeDevicePreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GazeDevicePreview> for ::windows::core::IInspectable {
    fn from(value: GazeDevicePreview) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GazeDevicePreview> for ::windows::core::IInspectable {
    fn from(value: &GazeDevicePreview) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GazeDevicePreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GazeDevicePreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GazeDevicePreview {}
unsafe impl ::core::marker::Sync for GazeDevicePreview {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GazeDeviceWatcherAddedPreviewEventArgs(pub ::windows::core::IInspectable);
impl GazeDeviceWatcherAddedPreviewEventArgs {
    pub fn Device(&self) -> ::windows::core::Result<GazeDevicePreview> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GazeDevicePreview>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for GazeDeviceWatcherAddedPreviewEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Input.Preview.GazeDeviceWatcherAddedPreviewEventArgs;{e79e7eed-b389-11e7-b201-c8d3ffb75721})");
}
unsafe impl ::windows::core::Interface for GazeDeviceWatcherAddedPreviewEventArgs {
    type Vtable = IGazeDeviceWatcherAddedPreviewEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe79e7eed_b389_11e7_b201_c8d3ffb75721);
}
impl ::windows::core::RuntimeName for GazeDeviceWatcherAddedPreviewEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.Preview.GazeDeviceWatcherAddedPreviewEventArgs";
}
impl ::core::convert::From<GazeDeviceWatcherAddedPreviewEventArgs> for ::windows::core::IUnknown {
    fn from(value: GazeDeviceWatcherAddedPreviewEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GazeDeviceWatcherAddedPreviewEventArgs> for ::windows::core::IUnknown {
    fn from(value: &GazeDeviceWatcherAddedPreviewEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GazeDeviceWatcherAddedPreviewEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GazeDeviceWatcherAddedPreviewEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GazeDeviceWatcherAddedPreviewEventArgs> for ::windows::core::IInspectable {
    fn from(value: GazeDeviceWatcherAddedPreviewEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GazeDeviceWatcherAddedPreviewEventArgs> for ::windows::core::IInspectable {
    fn from(value: &GazeDeviceWatcherAddedPreviewEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GazeDeviceWatcherAddedPreviewEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GazeDeviceWatcherAddedPreviewEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GazeDeviceWatcherAddedPreviewEventArgs {}
unsafe impl ::core::marker::Sync for GazeDeviceWatcherAddedPreviewEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GazeDeviceWatcherPreview(pub ::windows::core::IInspectable);
impl GazeDeviceWatcherPreview {
    #[cfg(feature = "Foundation")]
    pub fn Added<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<GazeDeviceWatcherPreview, GazeDeviceWatcherAddedPreviewEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveAdded<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Removed<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<GazeDeviceWatcherPreview, GazeDeviceWatcherRemovedPreviewEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveRemoved<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Updated<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<GazeDeviceWatcherPreview, GazeDeviceWatcherUpdatedPreviewEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveUpdated<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn EnumerationCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<GazeDeviceWatcherPreview, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveEnumerationCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for GazeDeviceWatcherPreview {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Input.Preview.GazeDeviceWatcherPreview;{e79e7ee7-b389-11e7-b201-c8d3ffb75721})");
}
unsafe impl ::windows::core::Interface for GazeDeviceWatcherPreview {
    type Vtable = IGazeDeviceWatcherPreview_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe79e7ee7_b389_11e7_b201_c8d3ffb75721);
}
impl ::windows::core::RuntimeName for GazeDeviceWatcherPreview {
    const NAME: &'static str = "Windows.Devices.Input.Preview.GazeDeviceWatcherPreview";
}
impl ::core::convert::From<GazeDeviceWatcherPreview> for ::windows::core::IUnknown {
    fn from(value: GazeDeviceWatcherPreview) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GazeDeviceWatcherPreview> for ::windows::core::IUnknown {
    fn from(value: &GazeDeviceWatcherPreview) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GazeDeviceWatcherPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GazeDeviceWatcherPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GazeDeviceWatcherPreview> for ::windows::core::IInspectable {
    fn from(value: GazeDeviceWatcherPreview) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GazeDeviceWatcherPreview> for ::windows::core::IInspectable {
    fn from(value: &GazeDeviceWatcherPreview) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GazeDeviceWatcherPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GazeDeviceWatcherPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GazeDeviceWatcherPreview {}
unsafe impl ::core::marker::Sync for GazeDeviceWatcherPreview {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GazeDeviceWatcherRemovedPreviewEventArgs(pub ::windows::core::IInspectable);
impl GazeDeviceWatcherRemovedPreviewEventArgs {
    pub fn Device(&self) -> ::windows::core::Result<GazeDevicePreview> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GazeDevicePreview>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for GazeDeviceWatcherRemovedPreviewEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Input.Preview.GazeDeviceWatcherRemovedPreviewEventArgs;{f2631f08-0e3f-431f-a606-50b35af94a1c})");
}
unsafe impl ::windows::core::Interface for GazeDeviceWatcherRemovedPreviewEventArgs {
    type Vtable = IGazeDeviceWatcherRemovedPreviewEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2631f08_0e3f_431f_a606_50b35af94a1c);
}
impl ::windows::core::RuntimeName for GazeDeviceWatcherRemovedPreviewEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.Preview.GazeDeviceWatcherRemovedPreviewEventArgs";
}
impl ::core::convert::From<GazeDeviceWatcherRemovedPreviewEventArgs> for ::windows::core::IUnknown {
    fn from(value: GazeDeviceWatcherRemovedPreviewEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GazeDeviceWatcherRemovedPreviewEventArgs> for ::windows::core::IUnknown {
    fn from(value: &GazeDeviceWatcherRemovedPreviewEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GazeDeviceWatcherRemovedPreviewEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GazeDeviceWatcherRemovedPreviewEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GazeDeviceWatcherRemovedPreviewEventArgs> for ::windows::core::IInspectable {
    fn from(value: GazeDeviceWatcherRemovedPreviewEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GazeDeviceWatcherRemovedPreviewEventArgs> for ::windows::core::IInspectable {
    fn from(value: &GazeDeviceWatcherRemovedPreviewEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GazeDeviceWatcherRemovedPreviewEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GazeDeviceWatcherRemovedPreviewEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GazeDeviceWatcherRemovedPreviewEventArgs {}
unsafe impl ::core::marker::Sync for GazeDeviceWatcherRemovedPreviewEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GazeDeviceWatcherUpdatedPreviewEventArgs(pub ::windows::core::IInspectable);
impl GazeDeviceWatcherUpdatedPreviewEventArgs {
    pub fn Device(&self) -> ::windows::core::Result<GazeDevicePreview> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GazeDevicePreview>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for GazeDeviceWatcherUpdatedPreviewEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Input.Preview.GazeDeviceWatcherUpdatedPreviewEventArgs;{7fe830ef-7f08-4737-88e1-4a83ae4e4885})");
}
unsafe impl ::windows::core::Interface for GazeDeviceWatcherUpdatedPreviewEventArgs {
    type Vtable = IGazeDeviceWatcherUpdatedPreviewEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7fe830ef_7f08_4737_88e1_4a83ae4e4885);
}
impl ::windows::core::RuntimeName for GazeDeviceWatcherUpdatedPreviewEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.Preview.GazeDeviceWatcherUpdatedPreviewEventArgs";
}
impl ::core::convert::From<GazeDeviceWatcherUpdatedPreviewEventArgs> for ::windows::core::IUnknown {
    fn from(value: GazeDeviceWatcherUpdatedPreviewEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GazeDeviceWatcherUpdatedPreviewEventArgs> for ::windows::core::IUnknown {
    fn from(value: &GazeDeviceWatcherUpdatedPreviewEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GazeDeviceWatcherUpdatedPreviewEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GazeDeviceWatcherUpdatedPreviewEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GazeDeviceWatcherUpdatedPreviewEventArgs> for ::windows::core::IInspectable {
    fn from(value: GazeDeviceWatcherUpdatedPreviewEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GazeDeviceWatcherUpdatedPreviewEventArgs> for ::windows::core::IInspectable {
    fn from(value: &GazeDeviceWatcherUpdatedPreviewEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GazeDeviceWatcherUpdatedPreviewEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GazeDeviceWatcherUpdatedPreviewEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GazeDeviceWatcherUpdatedPreviewEventArgs {}
unsafe impl ::core::marker::Sync for GazeDeviceWatcherUpdatedPreviewEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GazeEnteredPreviewEventArgs(pub ::windows::core::IInspectable);
impl GazeEnteredPreviewEventArgs {
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn CurrentPoint(&self) -> ::windows::core::Result<GazePointPreview> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GazePointPreview>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for GazeEnteredPreviewEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Input.Preview.GazeEnteredPreviewEventArgs;{2567bf43-1225-489f-9dd1-daa7c50fbf4b})");
}
unsafe impl ::windows::core::Interface for GazeEnteredPreviewEventArgs {
    type Vtable = IGazeEnteredPreviewEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2567bf43_1225_489f_9dd1_daa7c50fbf4b);
}
impl ::windows::core::RuntimeName for GazeEnteredPreviewEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.Preview.GazeEnteredPreviewEventArgs";
}
impl ::core::convert::From<GazeEnteredPreviewEventArgs> for ::windows::core::IUnknown {
    fn from(value: GazeEnteredPreviewEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GazeEnteredPreviewEventArgs> for ::windows::core::IUnknown {
    fn from(value: &GazeEnteredPreviewEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GazeEnteredPreviewEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GazeEnteredPreviewEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GazeEnteredPreviewEventArgs> for ::windows::core::IInspectable {
    fn from(value: GazeEnteredPreviewEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GazeEnteredPreviewEventArgs> for ::windows::core::IInspectable {
    fn from(value: &GazeEnteredPreviewEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GazeEnteredPreviewEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GazeEnteredPreviewEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GazeEnteredPreviewEventArgs {}
unsafe impl ::core::marker::Sync for GazeEnteredPreviewEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GazeExitedPreviewEventArgs(pub ::windows::core::IInspectable);
impl GazeExitedPreviewEventArgs {
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn CurrentPoint(&self) -> ::windows::core::Result<GazePointPreview> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GazePointPreview>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for GazeExitedPreviewEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Input.Preview.GazeExitedPreviewEventArgs;{5d0af07e-7d83-40ef-9f0a-fbc1bbdcc5ac})");
}
unsafe impl ::windows::core::Interface for GazeExitedPreviewEventArgs {
    type Vtable = IGazeExitedPreviewEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d0af07e_7d83_40ef_9f0a_fbc1bbdcc5ac);
}
impl ::windows::core::RuntimeName for GazeExitedPreviewEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.Preview.GazeExitedPreviewEventArgs";
}
impl ::core::convert::From<GazeExitedPreviewEventArgs> for ::windows::core::IUnknown {
    fn from(value: GazeExitedPreviewEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GazeExitedPreviewEventArgs> for ::windows::core::IUnknown {
    fn from(value: &GazeExitedPreviewEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GazeExitedPreviewEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GazeExitedPreviewEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GazeExitedPreviewEventArgs> for ::windows::core::IInspectable {
    fn from(value: GazeExitedPreviewEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GazeExitedPreviewEventArgs> for ::windows::core::IInspectable {
    fn from(value: &GazeExitedPreviewEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GazeExitedPreviewEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GazeExitedPreviewEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GazeExitedPreviewEventArgs {}
unsafe impl ::core::marker::Sync for GazeExitedPreviewEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GazeInputSourcePreview(pub ::windows::core::IInspectable);
impl GazeInputSourcePreview {
    #[cfg(feature = "Foundation")]
    pub fn GazeMoved<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<GazeInputSourcePreview, GazeMovedPreviewEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveGazeMoved<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn GazeEntered<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<GazeInputSourcePreview, GazeEnteredPreviewEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveGazeEntered<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn GazeExited<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<GazeInputSourcePreview, GazeExitedPreviewEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveGazeExited<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn GetForCurrentView() -> ::windows::core::Result<GazeInputSourcePreview> {
        Self::IGazeInputSourcePreviewStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GazeInputSourcePreview>(result__)
        })
    }
    pub fn CreateWatcher() -> ::windows::core::Result<GazeDeviceWatcherPreview> {
        Self::IGazeInputSourcePreviewStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GazeDeviceWatcherPreview>(result__)
        })
    }
    pub fn IGazeInputSourcePreviewStatics<R, F: FnOnce(&IGazeInputSourcePreviewStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GazeInputSourcePreview, IGazeInputSourcePreviewStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for GazeInputSourcePreview {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Input.Preview.GazeInputSourcePreview;{e79e7ee8-b389-11e7-b201-c8d3ffb75721})");
}
unsafe impl ::windows::core::Interface for GazeInputSourcePreview {
    type Vtable = IGazeInputSourcePreview_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe79e7ee8_b389_11e7_b201_c8d3ffb75721);
}
impl ::windows::core::RuntimeName for GazeInputSourcePreview {
    const NAME: &'static str = "Windows.Devices.Input.Preview.GazeInputSourcePreview";
}
impl ::core::convert::From<GazeInputSourcePreview> for ::windows::core::IUnknown {
    fn from(value: GazeInputSourcePreview) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GazeInputSourcePreview> for ::windows::core::IUnknown {
    fn from(value: &GazeInputSourcePreview) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GazeInputSourcePreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GazeInputSourcePreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GazeInputSourcePreview> for ::windows::core::IInspectable {
    fn from(value: GazeInputSourcePreview) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GazeInputSourcePreview> for ::windows::core::IInspectable {
    fn from(value: &GazeInputSourcePreview) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GazeInputSourcePreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GazeInputSourcePreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GazeInputSourcePreview {}
unsafe impl ::core::marker::Sync for GazeInputSourcePreview {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GazeMovedPreviewEventArgs(pub ::windows::core::IInspectable);
impl GazeMovedPreviewEventArgs {
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn CurrentPoint(&self) -> ::windows::core::Result<GazePointPreview> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GazePointPreview>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetIntermediatePoints(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<GazePointPreview>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<GazePointPreview>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for GazeMovedPreviewEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Input.Preview.GazeMovedPreviewEventArgs;{e79e7eeb-b389-11e7-b201-c8d3ffb75721})");
}
unsafe impl ::windows::core::Interface for GazeMovedPreviewEventArgs {
    type Vtable = IGazeMovedPreviewEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe79e7eeb_b389_11e7_b201_c8d3ffb75721);
}
impl ::windows::core::RuntimeName for GazeMovedPreviewEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.Preview.GazeMovedPreviewEventArgs";
}
impl ::core::convert::From<GazeMovedPreviewEventArgs> for ::windows::core::IUnknown {
    fn from(value: GazeMovedPreviewEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GazeMovedPreviewEventArgs> for ::windows::core::IUnknown {
    fn from(value: &GazeMovedPreviewEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GazeMovedPreviewEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GazeMovedPreviewEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GazeMovedPreviewEventArgs> for ::windows::core::IInspectable {
    fn from(value: GazeMovedPreviewEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GazeMovedPreviewEventArgs> for ::windows::core::IInspectable {
    fn from(value: &GazeMovedPreviewEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GazeMovedPreviewEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GazeMovedPreviewEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GazeMovedPreviewEventArgs {}
unsafe impl ::core::marker::Sync for GazeMovedPreviewEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GazePointPreview(pub ::windows::core::IInspectable);
impl GazePointPreview {
    pub fn SourceDevice(&self) -> ::windows::core::Result<GazeDevicePreview> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GazeDevicePreview>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn EyeGazePosition(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::Point>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::Point>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn HeadGazePosition(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::Point>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::Point>>(result__)
        }
    }
    pub fn Timestamp(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[cfg(feature = "Devices_HumanInterfaceDevice")]
    pub fn HidInputReport(&self) -> ::windows::core::Result<super::super::HumanInterfaceDevice::HidInputReport> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::HumanInterfaceDevice::HidInputReport>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for GazePointPreview {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Input.Preview.GazePointPreview;{e79e7eea-b389-11e7-b201-c8d3ffb75721})");
}
unsafe impl ::windows::core::Interface for GazePointPreview {
    type Vtable = IGazePointPreview_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe79e7eea_b389_11e7_b201_c8d3ffb75721);
}
impl ::windows::core::RuntimeName for GazePointPreview {
    const NAME: &'static str = "Windows.Devices.Input.Preview.GazePointPreview";
}
impl ::core::convert::From<GazePointPreview> for ::windows::core::IUnknown {
    fn from(value: GazePointPreview) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GazePointPreview> for ::windows::core::IUnknown {
    fn from(value: &GazePointPreview) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GazePointPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GazePointPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GazePointPreview> for ::windows::core::IInspectable {
    fn from(value: GazePointPreview) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GazePointPreview> for ::windows::core::IInspectable {
    fn from(value: &GazePointPreview) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GazePointPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GazePointPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GazePointPreview {}
unsafe impl ::core::marker::Sync for GazePointPreview {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IGazeDevicePreview(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGazeDevicePreview {
    type Vtable = IGazeDevicePreview_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe79e7ee9_b389_11e7_b201_c8d3ffb75721);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGazeDevicePreview_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut GazeDeviceConfigurationStatePreview) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Devices_HumanInterfaceDevice", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, usagepage: u16, usageid: u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_HumanInterfaceDevice", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Devices_HumanInterfaceDevice", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, usagepage: u16, usageid: u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_HumanInterfaceDevice", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGazeDeviceWatcherAddedPreviewEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGazeDeviceWatcherAddedPreviewEventArgs {
    type Vtable = IGazeDeviceWatcherAddedPreviewEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe79e7eed_b389_11e7_b201_c8d3ffb75721);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGazeDeviceWatcherAddedPreviewEventArgs_abi(
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
pub struct IGazeDeviceWatcherPreview(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGazeDeviceWatcherPreview {
    type Vtable = IGazeDeviceWatcherPreview_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe79e7ee7_b389_11e7_b201_c8d3ffb75721);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGazeDeviceWatcherPreview_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
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
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGazeDeviceWatcherRemovedPreviewEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGazeDeviceWatcherRemovedPreviewEventArgs {
    type Vtable = IGazeDeviceWatcherRemovedPreviewEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2631f08_0e3f_431f_a606_50b35af94a1c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGazeDeviceWatcherRemovedPreviewEventArgs_abi(
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
pub struct IGazeDeviceWatcherUpdatedPreviewEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGazeDeviceWatcherUpdatedPreviewEventArgs {
    type Vtable = IGazeDeviceWatcherUpdatedPreviewEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7fe830ef_7f08_4737_88e1_4a83ae4e4885);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGazeDeviceWatcherUpdatedPreviewEventArgs_abi(
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
pub struct IGazeEnteredPreviewEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGazeEnteredPreviewEventArgs {
    type Vtable = IGazeEnteredPreviewEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2567bf43_1225_489f_9dd1_daa7c50fbf4b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGazeEnteredPreviewEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGazeExitedPreviewEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGazeExitedPreviewEventArgs {
    type Vtable = IGazeExitedPreviewEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d0af07e_7d83_40ef_9f0a_fbc1bbdcc5ac);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGazeExitedPreviewEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGazeInputSourcePreview(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGazeInputSourcePreview {
    type Vtable = IGazeInputSourcePreview_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe79e7ee8_b389_11e7_b201_c8d3ffb75721);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGazeInputSourcePreview_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGazeInputSourcePreviewStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGazeInputSourcePreviewStatics {
    type Vtable = IGazeInputSourcePreviewStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe79e7ee6_b389_11e7_b201_c8d3ffb75721);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGazeInputSourcePreviewStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGazeMovedPreviewEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGazeMovedPreviewEventArgs {
    type Vtable = IGazeMovedPreviewEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe79e7eeb_b389_11e7_b201_c8d3ffb75721);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGazeMovedPreviewEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGazePointPreview(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGazePointPreview {
    type Vtable = IGazePointPreview_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe79e7eea_b389_11e7_b201_c8d3ffb75721);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGazePointPreview_abi(
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
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_HumanInterfaceDevice")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_HumanInterfaceDevice"))] usize,
);
