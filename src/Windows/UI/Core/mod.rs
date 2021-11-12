#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "UI_Core_AnimationMetrics")]
pub mod AnimationMetrics;
#[cfg(feature = "UI_Core_Preview")]
pub mod Preview;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AcceleratorKeyEventArgs(pub ::windows::core::IInspectable);
impl AcceleratorKeyEventArgs {
    pub fn EventType(&self) -> ::windows::core::Result<CoreAcceleratorKeyEventType> {
        let this = self;
        unsafe {
            let mut result__: CoreAcceleratorKeyEventType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreAcceleratorKeyEventType>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn VirtualKey(&self) -> ::windows::core::Result<super::super::System::VirtualKey> {
        let this = self;
        unsafe {
            let mut result__: super::super::System::VirtualKey = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::VirtualKey>(result__)
        }
    }
    pub fn KeyStatus(&self) -> ::windows::core::Result<CorePhysicalKeyStatus> {
        let this = self;
        unsafe {
            let mut result__: CorePhysicalKeyStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CorePhysicalKeyStatus>(result__)
        }
    }
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICoreWindowEventArgs>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWindowEventArgs>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAcceleratorKeyEventArgs2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AcceleratorKeyEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Core.AcceleratorKeyEventArgs;{ff1c4c4a-9287-470b-836e-9086e3126ade})");
}
unsafe impl ::windows::core::Interface for AcceleratorKeyEventArgs {
    type Vtable = IAcceleratorKeyEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xff1c4c4a_9287_470b_836e_9086e3126ade);
}
impl ::windows::core::RuntimeName for AcceleratorKeyEventArgs {
    const NAME: &'static str = "Windows.UI.Core.AcceleratorKeyEventArgs";
}
impl ::core::convert::From<AcceleratorKeyEventArgs> for ::windows::core::IUnknown {
    fn from(value: AcceleratorKeyEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AcceleratorKeyEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AcceleratorKeyEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AcceleratorKeyEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AcceleratorKeyEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AcceleratorKeyEventArgs> for ::windows::core::IInspectable {
    fn from(value: AcceleratorKeyEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AcceleratorKeyEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AcceleratorKeyEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AcceleratorKeyEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AcceleratorKeyEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<AcceleratorKeyEventArgs> for ICoreWindowEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: AcceleratorKeyEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AcceleratorKeyEventArgs> for ICoreWindowEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &AcceleratorKeyEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICoreWindowEventArgs> for AcceleratorKeyEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ICoreWindowEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICoreWindowEventArgs> for &AcceleratorKeyEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ICoreWindowEventArgs> {
        ::core::convert::TryInto::<ICoreWindowEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AcceleratorKeyEventArgs {}
unsafe impl ::core::marker::Sync for AcceleratorKeyEventArgs {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppViewBackButtonVisibility(pub i32);
impl AppViewBackButtonVisibility {
    pub const Visible: AppViewBackButtonVisibility = AppViewBackButtonVisibility(0i32);
    pub const Collapsed: AppViewBackButtonVisibility = AppViewBackButtonVisibility(1i32);
    pub const Disabled: AppViewBackButtonVisibility = AppViewBackButtonVisibility(2i32);
}
impl ::core::convert::From<i32> for AppViewBackButtonVisibility {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AppViewBackButtonVisibility {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AppViewBackButtonVisibility {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Core.AppViewBackButtonVisibility;i4)");
}
impl ::windows::core::DefaultType for AppViewBackButtonVisibility {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AutomationProviderRequestedEventArgs(pub ::windows::core::IInspectable);
impl AutomationProviderRequestedEventArgs {
    pub fn AutomationProvider(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn SetAutomationProvider<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICoreWindowEventArgs>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWindowEventArgs>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for AutomationProviderRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Core.AutomationProviderRequestedEventArgs;{961ff258-21bf-4b42-a298-fa479d4c52e2})");
}
unsafe impl ::windows::core::Interface for AutomationProviderRequestedEventArgs {
    type Vtable = IAutomationProviderRequestedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x961ff258_21bf_4b42_a298_fa479d4c52e2);
}
impl ::windows::core::RuntimeName for AutomationProviderRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Core.AutomationProviderRequestedEventArgs";
}
impl ::core::convert::From<AutomationProviderRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AutomationProviderRequestedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AutomationProviderRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AutomationProviderRequestedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AutomationProviderRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AutomationProviderRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AutomationProviderRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AutomationProviderRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AutomationProviderRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AutomationProviderRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AutomationProviderRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AutomationProviderRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<AutomationProviderRequestedEventArgs> for ICoreWindowEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: AutomationProviderRequestedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AutomationProviderRequestedEventArgs> for ICoreWindowEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &AutomationProviderRequestedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICoreWindowEventArgs> for AutomationProviderRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ICoreWindowEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICoreWindowEventArgs> for &AutomationProviderRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ICoreWindowEventArgs> {
        ::core::convert::TryInto::<ICoreWindowEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BackRequestedEventArgs(pub ::windows::core::IInspectable);
impl BackRequestedEventArgs {
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
}
unsafe impl ::windows::core::RuntimeType for BackRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Core.BackRequestedEventArgs;{d603d28a-e411-4a4e-ba41-6a327a8675bc})");
}
unsafe impl ::windows::core::Interface for BackRequestedEventArgs {
    type Vtable = IBackRequestedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd603d28a_e411_4a4e_ba41_6a327a8675bc);
}
impl ::windows::core::RuntimeName for BackRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Core.BackRequestedEventArgs";
}
impl ::core::convert::From<BackRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: BackRequestedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BackRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &BackRequestedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BackRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BackRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BackRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: BackRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BackRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &BackRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BackRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BackRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for BackRequestedEventArgs {}
unsafe impl ::core::marker::Sync for BackRequestedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CharacterReceivedEventArgs(pub ::windows::core::IInspectable);
impl CharacterReceivedEventArgs {
    pub fn KeyCode(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn KeyStatus(&self) -> ::windows::core::Result<CorePhysicalKeyStatus> {
        let this = self;
        unsafe {
            let mut result__: CorePhysicalKeyStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CorePhysicalKeyStatus>(result__)
        }
    }
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICoreWindowEventArgs>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWindowEventArgs>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for CharacterReceivedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Core.CharacterReceivedEventArgs;{c584659f-99b2-4bcc-bd33-04e63f42902e})");
}
unsafe impl ::windows::core::Interface for CharacterReceivedEventArgs {
    type Vtable = ICharacterReceivedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc584659f_99b2_4bcc_bd33_04e63f42902e);
}
impl ::windows::core::RuntimeName for CharacterReceivedEventArgs {
    const NAME: &'static str = "Windows.UI.Core.CharacterReceivedEventArgs";
}
impl ::core::convert::From<CharacterReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: CharacterReceivedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CharacterReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CharacterReceivedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CharacterReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CharacterReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CharacterReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: CharacterReceivedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CharacterReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CharacterReceivedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CharacterReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CharacterReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<CharacterReceivedEventArgs> for ICoreWindowEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: CharacterReceivedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CharacterReceivedEventArgs> for ICoreWindowEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &CharacterReceivedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICoreWindowEventArgs> for CharacterReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ICoreWindowEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICoreWindowEventArgs> for &CharacterReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ICoreWindowEventArgs> {
        ::core::convert::TryInto::<ICoreWindowEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ClosestInteractiveBoundsRequestedEventArgs(pub ::windows::core::IInspectable);
impl ClosestInteractiveBoundsRequestedEventArgs {
    #[cfg(feature = "Foundation")]
    pub fn PointerPosition(&self) -> ::windows::core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Point>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SearchBounds(&self) -> ::windows::core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ClosestInteractiveBounds(&self) -> ::windows::core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetClosestInteractiveBounds<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for ClosestInteractiveBoundsRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Core.ClosestInteractiveBoundsRequestedEventArgs;{347c11d7-f6f8-40e3-b29f-ae50d3e86486})");
}
unsafe impl ::windows::core::Interface for ClosestInteractiveBoundsRequestedEventArgs {
    type Vtable = IClosestInteractiveBoundsRequestedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x347c11d7_f6f8_40e3_b29f_ae50d3e86486);
}
impl ::windows::core::RuntimeName for ClosestInteractiveBoundsRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Core.ClosestInteractiveBoundsRequestedEventArgs";
}
impl ::core::convert::From<ClosestInteractiveBoundsRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ClosestInteractiveBoundsRequestedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ClosestInteractiveBoundsRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ClosestInteractiveBoundsRequestedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ClosestInteractiveBoundsRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ClosestInteractiveBoundsRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ClosestInteractiveBoundsRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ClosestInteractiveBoundsRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ClosestInteractiveBoundsRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ClosestInteractiveBoundsRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ClosestInteractiveBoundsRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ClosestInteractiveBoundsRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CoreAcceleratorKeyEventType(pub i32);
impl CoreAcceleratorKeyEventType {
    pub const Character: CoreAcceleratorKeyEventType = CoreAcceleratorKeyEventType(2i32);
    pub const DeadCharacter: CoreAcceleratorKeyEventType = CoreAcceleratorKeyEventType(3i32);
    pub const KeyDown: CoreAcceleratorKeyEventType = CoreAcceleratorKeyEventType(0i32);
    pub const KeyUp: CoreAcceleratorKeyEventType = CoreAcceleratorKeyEventType(1i32);
    pub const SystemCharacter: CoreAcceleratorKeyEventType = CoreAcceleratorKeyEventType(6i32);
    pub const SystemDeadCharacter: CoreAcceleratorKeyEventType = CoreAcceleratorKeyEventType(7i32);
    pub const SystemKeyDown: CoreAcceleratorKeyEventType = CoreAcceleratorKeyEventType(4i32);
    pub const SystemKeyUp: CoreAcceleratorKeyEventType = CoreAcceleratorKeyEventType(5i32);
    pub const UnicodeCharacter: CoreAcceleratorKeyEventType = CoreAcceleratorKeyEventType(8i32);
}
impl ::core::convert::From<i32> for CoreAcceleratorKeyEventType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CoreAcceleratorKeyEventType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CoreAcceleratorKeyEventType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Core.CoreAcceleratorKeyEventType;i4)");
}
impl ::windows::core::DefaultType for CoreAcceleratorKeyEventType {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CoreAcceleratorKeys(pub ::windows::core::IInspectable);
impl CoreAcceleratorKeys {
    #[cfg(feature = "Foundation")]
    pub fn AcceleratorKeyActivated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<CoreDispatcher, AcceleratorKeyEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveAcceleratorKeyActivated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreAcceleratorKeys {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Core.CoreAcceleratorKeys;{9ffdf7f5-b8c9-4ef0-b7d2-1de626561fc8})");
}
unsafe impl ::windows::core::Interface for CoreAcceleratorKeys {
    type Vtable = ICoreAcceleratorKeys_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9ffdf7f5_b8c9_4ef0_b7d2_1de626561fc8);
}
impl ::windows::core::RuntimeName for CoreAcceleratorKeys {
    const NAME: &'static str = "Windows.UI.Core.CoreAcceleratorKeys";
}
impl ::core::convert::From<CoreAcceleratorKeys> for ::windows::core::IUnknown {
    fn from(value: CoreAcceleratorKeys) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreAcceleratorKeys> for ::windows::core::IUnknown {
    fn from(value: &CoreAcceleratorKeys) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreAcceleratorKeys {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreAcceleratorKeys {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreAcceleratorKeys> for ::windows::core::IInspectable {
    fn from(value: CoreAcceleratorKeys) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreAcceleratorKeys> for ::windows::core::IInspectable {
    fn from(value: &CoreAcceleratorKeys) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreAcceleratorKeys {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreAcceleratorKeys {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<CoreAcceleratorKeys> for ICoreAcceleratorKeys {
    fn from(value: CoreAcceleratorKeys) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreAcceleratorKeys> for ICoreAcceleratorKeys {
    fn from(value: &CoreAcceleratorKeys) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICoreAcceleratorKeys> for CoreAcceleratorKeys {
    fn into_param(self) -> ::windows::core::Param<'a, ICoreAcceleratorKeys> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICoreAcceleratorKeys> for &CoreAcceleratorKeys {
    fn into_param(self) -> ::windows::core::Param<'a, ICoreAcceleratorKeys> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CoreAcceleratorKeys {}
unsafe impl ::core::marker::Sync for CoreAcceleratorKeys {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CoreComponentInputSource(pub ::windows::core::IInspectable);
impl CoreComponentInputSource {
    pub fn Dispatcher(&self) -> ::windows::core::Result<CoreDispatcher> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreDispatcher>(result__)
        }
    }
    pub fn IsInputEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsInputEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn InputEnabled<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, InputEnabledEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveInputEnabled<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    pub fn HasFocus(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICoreComponentFocusable>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GotFocus<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, CoreWindowEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICoreComponentFocusable>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveGotFocus<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreComponentFocusable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn LostFocus<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, CoreWindowEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICoreComponentFocusable>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveLostFocus<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreComponentFocusable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "System")]
    pub fn GetCurrentKeyState(&self, virtualkey: super::super::System::VirtualKey) -> ::windows::core::Result<CoreVirtualKeyStates> {
        let this = &::windows::core::Interface::cast::<ICoreKeyboardInputSource>(self)?;
        unsafe {
            let mut result__: CoreVirtualKeyStates = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), virtualkey, &mut result__).from_abi::<CoreVirtualKeyStates>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn CharacterReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, CharacterReceivedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICoreKeyboardInputSource>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveCharacterReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreKeyboardInputSource>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn KeyDown<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, KeyEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICoreKeyboardInputSource>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveKeyDown<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreKeyboardInputSource>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn KeyUp<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, KeyEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICoreKeyboardInputSource>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveKeyUp<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreKeyboardInputSource>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    pub fn ReleasePointerCapture(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn SetPointerCapture(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn HasCapture(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn PointerPosition(&self) -> ::windows::core::Result<super::super::Foundation::Point> {
        let this = &::windows::core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Point>(result__)
        }
    }
    pub fn PointerCursor(&self) -> ::windows::core::Result<CoreCursor> {
        let this = &::windows::core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreCursor>(result__)
        }
    }
    pub fn SetPointerCursor<'a, Param0: ::windows::core::IntoParam<'a, CoreCursor>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PointerCaptureLost<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerCaptureLost<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PointerEntered<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerEntered<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PointerExited<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerExited<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PointerMoved<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerMoved<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PointerPressed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerPressed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PointerReleased<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerReleased<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PointerWheelChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerWheelChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn TouchHitTesting<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, TouchHitTestingEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICoreTouchHitTesting>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveTouchHitTesting<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreTouchHitTesting>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn ClosestInteractiveBoundsRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<CoreComponentInputSource, ClosestInteractiveBoundsRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICoreClosestInteractiveBoundsRequested>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveClosestInteractiveBoundsRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreClosestInteractiveBoundsRequested>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    pub fn GetCurrentKeyEventDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ICoreKeyboardInputSource2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows::core::Result<super::super::System::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<ICorePointerInputSource2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::DispatcherQueue>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreComponentInputSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Core.CoreComponentInputSource;{9f488807-4580-4be8-be68-92a9311713bb})");
}
unsafe impl ::windows::core::Interface for CoreComponentInputSource {
    type Vtable = ICoreInputSourceBase_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f488807_4580_4be8_be68_92a9311713bb);
}
impl ::windows::core::RuntimeName for CoreComponentInputSource {
    const NAME: &'static str = "Windows.UI.Core.CoreComponentInputSource";
}
impl ::core::convert::From<CoreComponentInputSource> for ::windows::core::IUnknown {
    fn from(value: CoreComponentInputSource) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreComponentInputSource> for ::windows::core::IUnknown {
    fn from(value: &CoreComponentInputSource) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreComponentInputSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreComponentInputSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreComponentInputSource> for ::windows::core::IInspectable {
    fn from(value: CoreComponentInputSource) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreComponentInputSource> for ::windows::core::IInspectable {
    fn from(value: &CoreComponentInputSource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreComponentInputSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreComponentInputSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<CoreComponentInputSource> for ICoreInputSourceBase {
    fn from(value: CoreComponentInputSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreComponentInputSource> for ICoreInputSourceBase {
    fn from(value: &CoreComponentInputSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICoreInputSourceBase> for CoreComponentInputSource {
    fn into_param(self) -> ::windows::core::Param<'a, ICoreInputSourceBase> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICoreInputSourceBase> for &CoreComponentInputSource {
    fn into_param(self) -> ::windows::core::Param<'a, ICoreInputSourceBase> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<CoreComponentInputSource> for ICorePointerInputSource {
    type Error = ::windows::core::Error;
    fn try_from(value: CoreComponentInputSource) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CoreComponentInputSource> for ICorePointerInputSource {
    type Error = ::windows::core::Error;
    fn try_from(value: &CoreComponentInputSource) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICorePointerInputSource> for CoreComponentInputSource {
    fn into_param(self) -> ::windows::core::Param<'a, ICorePointerInputSource> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICorePointerInputSource> for &CoreComponentInputSource {
    fn into_param(self) -> ::windows::core::Param<'a, ICorePointerInputSource> {
        ::core::convert::TryInto::<ICorePointerInputSource>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<CoreComponentInputSource> for ICorePointerInputSource2 {
    type Error = ::windows::core::Error;
    fn try_from(value: CoreComponentInputSource) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CoreComponentInputSource> for ICorePointerInputSource2 {
    type Error = ::windows::core::Error;
    fn try_from(value: &CoreComponentInputSource) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICorePointerInputSource2> for CoreComponentInputSource {
    fn into_param(self) -> ::windows::core::Param<'a, ICorePointerInputSource2> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICorePointerInputSource2> for &CoreComponentInputSource {
    fn into_param(self) -> ::windows::core::Param<'a, ICorePointerInputSource2> {
        ::core::convert::TryInto::<ICorePointerInputSource2>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for CoreComponentInputSource {}
unsafe impl ::core::marker::Sync for CoreComponentInputSource {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CoreCursor(pub ::windows::core::IInspectable);
impl CoreCursor {
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn Type(&self) -> ::windows::core::Result<CoreCursorType> {
        let this = self;
        unsafe {
            let mut result__: CoreCursorType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreCursorType>(result__)
        }
    }
    pub fn CreateCursor(r#type: CoreCursorType, id: u32) -> ::windows::core::Result<CoreCursor> {
        Self::ICoreCursorFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), r#type, id, &mut result__).from_abi::<CoreCursor>(result__)
        })
    }
    pub fn ICoreCursorFactory<R, F: FnOnce(&ICoreCursorFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CoreCursor, ICoreCursorFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreCursor {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Core.CoreCursor;{96893acf-111d-442c-8a77-b87992f8e2d6})");
}
unsafe impl ::windows::core::Interface for CoreCursor {
    type Vtable = ICoreCursor_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96893acf_111d_442c_8a77_b87992f8e2d6);
}
impl ::windows::core::RuntimeName for CoreCursor {
    const NAME: &'static str = "Windows.UI.Core.CoreCursor";
}
impl ::core::convert::From<CoreCursor> for ::windows::core::IUnknown {
    fn from(value: CoreCursor) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreCursor> for ::windows::core::IUnknown {
    fn from(value: &CoreCursor) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreCursor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreCursor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreCursor> for ::windows::core::IInspectable {
    fn from(value: CoreCursor) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreCursor> for ::windows::core::IInspectable {
    fn from(value: &CoreCursor) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreCursor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreCursor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreCursor {}
unsafe impl ::core::marker::Sync for CoreCursor {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CoreCursorType(pub i32);
impl CoreCursorType {
    pub const Arrow: CoreCursorType = CoreCursorType(0i32);
    pub const Cross: CoreCursorType = CoreCursorType(1i32);
    pub const Custom: CoreCursorType = CoreCursorType(2i32);
    pub const Hand: CoreCursorType = CoreCursorType(3i32);
    pub const Help: CoreCursorType = CoreCursorType(4i32);
    pub const IBeam: CoreCursorType = CoreCursorType(5i32);
    pub const SizeAll: CoreCursorType = CoreCursorType(6i32);
    pub const SizeNortheastSouthwest: CoreCursorType = CoreCursorType(7i32);
    pub const SizeNorthSouth: CoreCursorType = CoreCursorType(8i32);
    pub const SizeNorthwestSoutheast: CoreCursorType = CoreCursorType(9i32);
    pub const SizeWestEast: CoreCursorType = CoreCursorType(10i32);
    pub const UniversalNo: CoreCursorType = CoreCursorType(11i32);
    pub const UpArrow: CoreCursorType = CoreCursorType(12i32);
    pub const Wait: CoreCursorType = CoreCursorType(13i32);
    pub const Pin: CoreCursorType = CoreCursorType(14i32);
    pub const Person: CoreCursorType = CoreCursorType(15i32);
}
impl ::core::convert::From<i32> for CoreCursorType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CoreCursorType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CoreCursorType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Core.CoreCursorType;i4)");
}
impl ::windows::core::DefaultType for CoreCursorType {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CoreDispatcher(pub ::windows::core::IInspectable);
impl CoreDispatcher {
    pub fn HasThreadAccess(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn ProcessEvents(&self, options: CoreProcessEventsOption) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), options).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn RunAsync<'a, Param1: ::windows::core::IntoParam<'a, DispatchedHandler>>(&self, priority: CoreDispatcherPriority, agilecallback: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), priority, agilecallback.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RunIdleAsync<'a, Param0: ::windows::core::IntoParam<'a, IdleDispatchedHandler>>(&self, agilecallback: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), agilecallback.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn AcceleratorKeyActivated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<CoreDispatcher, AcceleratorKeyEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICoreAcceleratorKeys>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveAcceleratorKeyActivated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreAcceleratorKeys>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn TryRunAsync<'a, Param1: ::windows::core::IntoParam<'a, DispatchedHandler>>(&self, priority: CoreDispatcherPriority, agilecallback: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<ICoreDispatcher2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), priority, agilecallback.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn TryRunIdleAsync<'a, Param0: ::windows::core::IntoParam<'a, IdleDispatchedHandler>>(&self, agilecallback: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<ICoreDispatcher2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), agilecallback.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn CurrentPriority(&self) -> ::windows::core::Result<CoreDispatcherPriority> {
        let this = &::windows::core::Interface::cast::<ICoreDispatcherWithTaskPriority>(self)?;
        unsafe {
            let mut result__: CoreDispatcherPriority = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreDispatcherPriority>(result__)
        }
    }
    pub fn SetCurrentPriority(&self, value: CoreDispatcherPriority) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreDispatcherWithTaskPriority>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ShouldYield(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICoreDispatcherWithTaskPriority>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn ShouldYieldToPriority(&self, priority: CoreDispatcherPriority) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICoreDispatcherWithTaskPriority>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), priority, &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn StopProcessEvents(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreDispatcherWithTaskPriority>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreDispatcher {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Core.CoreDispatcher;{60db2fa8-b705-4fde-a7d6-ebbb1891d39e})");
}
unsafe impl ::windows::core::Interface for CoreDispatcher {
    type Vtable = ICoreDispatcher_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x60db2fa8_b705_4fde_a7d6_ebbb1891d39e);
}
impl ::windows::core::RuntimeName for CoreDispatcher {
    const NAME: &'static str = "Windows.UI.Core.CoreDispatcher";
}
impl ::core::convert::From<CoreDispatcher> for ::windows::core::IUnknown {
    fn from(value: CoreDispatcher) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreDispatcher> for ::windows::core::IUnknown {
    fn from(value: &CoreDispatcher) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreDispatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreDispatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreDispatcher> for ::windows::core::IInspectable {
    fn from(value: CoreDispatcher) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreDispatcher> for ::windows::core::IInspectable {
    fn from(value: &CoreDispatcher) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreDispatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreDispatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<CoreDispatcher> for ICoreAcceleratorKeys {
    type Error = ::windows::core::Error;
    fn try_from(value: CoreDispatcher) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CoreDispatcher> for ICoreAcceleratorKeys {
    type Error = ::windows::core::Error;
    fn try_from(value: &CoreDispatcher) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICoreAcceleratorKeys> for CoreDispatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ICoreAcceleratorKeys> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICoreAcceleratorKeys> for &CoreDispatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ICoreAcceleratorKeys> {
        ::core::convert::TryInto::<ICoreAcceleratorKeys>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for CoreDispatcher {}
unsafe impl ::core::marker::Sync for CoreDispatcher {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CoreDispatcherPriority(pub i32);
impl CoreDispatcherPriority {
    pub const Idle: CoreDispatcherPriority = CoreDispatcherPriority(-2i32);
    pub const Low: CoreDispatcherPriority = CoreDispatcherPriority(-1i32);
    pub const Normal: CoreDispatcherPriority = CoreDispatcherPriority(0i32);
    pub const High: CoreDispatcherPriority = CoreDispatcherPriority(1i32);
}
impl ::core::convert::From<i32> for CoreDispatcherPriority {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CoreDispatcherPriority {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CoreDispatcherPriority {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Core.CoreDispatcherPriority;i4)");
}
impl ::windows::core::DefaultType for CoreDispatcherPriority {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CoreIndependentInputFilters(pub u32);
impl CoreIndependentInputFilters {
    pub const None: CoreIndependentInputFilters = CoreIndependentInputFilters(0u32);
    pub const MouseButton: CoreIndependentInputFilters = CoreIndependentInputFilters(1u32);
    pub const MouseWheel: CoreIndependentInputFilters = CoreIndependentInputFilters(2u32);
    pub const MouseHover: CoreIndependentInputFilters = CoreIndependentInputFilters(4u32);
    pub const PenWithBarrel: CoreIndependentInputFilters = CoreIndependentInputFilters(8u32);
    pub const PenInverted: CoreIndependentInputFilters = CoreIndependentInputFilters(16u32);
}
impl ::core::convert::From<u32> for CoreIndependentInputFilters {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CoreIndependentInputFilters {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CoreIndependentInputFilters {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Core.CoreIndependentInputFilters;u4)");
}
impl ::windows::core::DefaultType for CoreIndependentInputFilters {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for CoreIndependentInputFilters {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for CoreIndependentInputFilters {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for CoreIndependentInputFilters {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for CoreIndependentInputFilters {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for CoreIndependentInputFilters {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CoreIndependentInputSource(pub ::windows::core::IInspectable);
impl CoreIndependentInputSource {
    pub fn Dispatcher(&self) -> ::windows::core::Result<CoreDispatcher> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreDispatcher>(result__)
        }
    }
    pub fn IsInputEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsInputEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn InputEnabled<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, InputEnabledEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveInputEnabled<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    pub fn ReleasePointerCapture(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn SetPointerCapture(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn HasCapture(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn PointerPosition(&self) -> ::windows::core::Result<super::super::Foundation::Point> {
        let this = &::windows::core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Point>(result__)
        }
    }
    pub fn PointerCursor(&self) -> ::windows::core::Result<CoreCursor> {
        let this = &::windows::core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreCursor>(result__)
        }
    }
    pub fn SetPointerCursor<'a, Param0: ::windows::core::IntoParam<'a, CoreCursor>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PointerCaptureLost<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerCaptureLost<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PointerEntered<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerEntered<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PointerExited<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerExited<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PointerMoved<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerMoved<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PointerPressed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerPressed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PointerReleased<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerReleased<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PointerWheelChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerWheelChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PointerRoutedAway<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ICorePointerRedirector, PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICorePointerRedirector>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerRoutedAway<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICorePointerRedirector>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PointerRoutedTo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ICorePointerRedirector, PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICorePointerRedirector>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerRoutedTo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICorePointerRedirector>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PointerRoutedReleased<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ICorePointerRedirector, PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICorePointerRedirector>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerRoutedReleased<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICorePointerRedirector>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows::core::Result<super::super::System::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<ICorePointerInputSource2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::DispatcherQueue>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreIndependentInputSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Core.CoreIndependentInputSource;{9f488807-4580-4be8-be68-92a9311713bb})");
}
unsafe impl ::windows::core::Interface for CoreIndependentInputSource {
    type Vtable = ICoreInputSourceBase_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f488807_4580_4be8_be68_92a9311713bb);
}
impl ::windows::core::RuntimeName for CoreIndependentInputSource {
    const NAME: &'static str = "Windows.UI.Core.CoreIndependentInputSource";
}
impl ::core::convert::From<CoreIndependentInputSource> for ::windows::core::IUnknown {
    fn from(value: CoreIndependentInputSource) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreIndependentInputSource> for ::windows::core::IUnknown {
    fn from(value: &CoreIndependentInputSource) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreIndependentInputSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreIndependentInputSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreIndependentInputSource> for ::windows::core::IInspectable {
    fn from(value: CoreIndependentInputSource) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreIndependentInputSource> for ::windows::core::IInspectable {
    fn from(value: &CoreIndependentInputSource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreIndependentInputSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreIndependentInputSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<CoreIndependentInputSource> for ICoreInputSourceBase {
    fn from(value: CoreIndependentInputSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreIndependentInputSource> for ICoreInputSourceBase {
    fn from(value: &CoreIndependentInputSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICoreInputSourceBase> for CoreIndependentInputSource {
    fn into_param(self) -> ::windows::core::Param<'a, ICoreInputSourceBase> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICoreInputSourceBase> for &CoreIndependentInputSource {
    fn into_param(self) -> ::windows::core::Param<'a, ICoreInputSourceBase> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<CoreIndependentInputSource> for ICorePointerInputSource {
    type Error = ::windows::core::Error;
    fn try_from(value: CoreIndependentInputSource) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CoreIndependentInputSource> for ICorePointerInputSource {
    type Error = ::windows::core::Error;
    fn try_from(value: &CoreIndependentInputSource) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICorePointerInputSource> for CoreIndependentInputSource {
    fn into_param(self) -> ::windows::core::Param<'a, ICorePointerInputSource> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICorePointerInputSource> for &CoreIndependentInputSource {
    fn into_param(self) -> ::windows::core::Param<'a, ICorePointerInputSource> {
        ::core::convert::TryInto::<ICorePointerInputSource>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<CoreIndependentInputSource> for ICorePointerRedirector {
    type Error = ::windows::core::Error;
    fn try_from(value: CoreIndependentInputSource) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CoreIndependentInputSource> for ICorePointerRedirector {
    type Error = ::windows::core::Error;
    fn try_from(value: &CoreIndependentInputSource) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICorePointerRedirector> for CoreIndependentInputSource {
    fn into_param(self) -> ::windows::core::Param<'a, ICorePointerRedirector> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICorePointerRedirector> for &CoreIndependentInputSource {
    fn into_param(self) -> ::windows::core::Param<'a, ICorePointerRedirector> {
        ::core::convert::TryInto::<ICorePointerRedirector>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<CoreIndependentInputSource> for ICorePointerInputSource2 {
    type Error = ::windows::core::Error;
    fn try_from(value: CoreIndependentInputSource) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CoreIndependentInputSource> for ICorePointerInputSource2 {
    type Error = ::windows::core::Error;
    fn try_from(value: &CoreIndependentInputSource) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICorePointerInputSource2> for CoreIndependentInputSource {
    fn into_param(self) -> ::windows::core::Param<'a, ICorePointerInputSource2> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICorePointerInputSource2> for &CoreIndependentInputSource {
    fn into_param(self) -> ::windows::core::Param<'a, ICorePointerInputSource2> {
        ::core::convert::TryInto::<ICorePointerInputSource2>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for CoreIndependentInputSource {}
unsafe impl ::core::marker::Sync for CoreIndependentInputSource {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CoreIndependentInputSourceController(pub ::windows::core::IInspectable);
impl CoreIndependentInputSourceController {
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn IsTransparentForUncontrolledInput(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsTransparentForUncontrolledInput(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IsPalmRejectionEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsPalmRejectionEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Source(&self) -> ::windows::core::Result<CoreIndependentInputSource> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreIndependentInputSource>(result__)
        }
    }
    pub fn SetControlledInput(&self, inputtypes: CoreInputDeviceTypes) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), inputtypes).ok() }
    }
    pub fn SetControlledInputWithFilters(&self, inputtypes: CoreInputDeviceTypes, required: CoreIndependentInputFilters, excluded: CoreIndependentInputFilters) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), inputtypes, required, excluded).ok() }
    }
    #[cfg(feature = "UI_Composition")]
    pub fn CreateForVisual<'a, Param0: ::windows::core::IntoParam<'a, super::Composition::Visual>>(visual: Param0) -> ::windows::core::Result<CoreIndependentInputSourceController> {
        Self::ICoreIndependentInputSourceControllerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), visual.into_param().abi(), &mut result__).from_abi::<CoreIndependentInputSourceController>(result__)
        })
    }
    #[cfg(feature = "UI_Composition")]
    pub fn CreateForIVisualElement<'a, Param0: ::windows::core::IntoParam<'a, super::Composition::IVisualElement>>(visualelement: Param0) -> ::windows::core::Result<CoreIndependentInputSourceController> {
        Self::ICoreIndependentInputSourceControllerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), visualelement.into_param().abi(), &mut result__).from_abi::<CoreIndependentInputSourceController>(result__)
        })
    }
    pub fn ICoreIndependentInputSourceControllerStatics<R, F: FnOnce(&ICoreIndependentInputSourceControllerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CoreIndependentInputSourceController, ICoreIndependentInputSourceControllerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreIndependentInputSourceController {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Core.CoreIndependentInputSourceController;{0963261c-84fe-578a-83ca-6425309ccde4})");
}
unsafe impl ::windows::core::Interface for CoreIndependentInputSourceController {
    type Vtable = ICoreIndependentInputSourceController_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0963261c_84fe_578a_83ca_6425309ccde4);
}
impl ::windows::core::RuntimeName for CoreIndependentInputSourceController {
    const NAME: &'static str = "Windows.UI.Core.CoreIndependentInputSourceController";
}
impl ::core::convert::From<CoreIndependentInputSourceController> for ::windows::core::IUnknown {
    fn from(value: CoreIndependentInputSourceController) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreIndependentInputSourceController> for ::windows::core::IUnknown {
    fn from(value: &CoreIndependentInputSourceController) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreIndependentInputSourceController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreIndependentInputSourceController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreIndependentInputSourceController> for ::windows::core::IInspectable {
    fn from(value: CoreIndependentInputSourceController) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreIndependentInputSourceController> for ::windows::core::IInspectable {
    fn from(value: &CoreIndependentInputSourceController) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreIndependentInputSourceController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreIndependentInputSourceController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<CoreIndependentInputSourceController> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: CoreIndependentInputSourceController) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CoreIndependentInputSourceController> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &CoreIndependentInputSourceController) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for CoreIndependentInputSourceController {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &CoreIndependentInputSourceController {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for CoreIndependentInputSourceController {}
unsafe impl ::core::marker::Sync for CoreIndependentInputSourceController {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CoreInputDeviceTypes(pub u32);
impl CoreInputDeviceTypes {
    pub const None: CoreInputDeviceTypes = CoreInputDeviceTypes(0u32);
    pub const Touch: CoreInputDeviceTypes = CoreInputDeviceTypes(1u32);
    pub const Pen: CoreInputDeviceTypes = CoreInputDeviceTypes(2u32);
    pub const Mouse: CoreInputDeviceTypes = CoreInputDeviceTypes(4u32);
}
impl ::core::convert::From<u32> for CoreInputDeviceTypes {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CoreInputDeviceTypes {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CoreInputDeviceTypes {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Core.CoreInputDeviceTypes;u4)");
}
impl ::windows::core::DefaultType for CoreInputDeviceTypes {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for CoreInputDeviceTypes {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for CoreInputDeviceTypes {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for CoreInputDeviceTypes {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for CoreInputDeviceTypes {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for CoreInputDeviceTypes {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct CorePhysicalKeyStatus {
    pub RepeatCount: u32,
    pub ScanCode: u32,
    pub IsExtendedKey: bool,
    pub IsMenuKeyDown: bool,
    pub WasKeyDown: bool,
    pub IsKeyReleased: bool,
}
impl CorePhysicalKeyStatus {}
impl ::core::default::Default for CorePhysicalKeyStatus {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for CorePhysicalKeyStatus {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("CorePhysicalKeyStatus").field("RepeatCount", &self.RepeatCount).field("ScanCode", &self.ScanCode).field("IsExtendedKey", &self.IsExtendedKey).field("IsMenuKeyDown", &self.IsMenuKeyDown).field("WasKeyDown", &self.WasKeyDown).field("IsKeyReleased", &self.IsKeyReleased).finish()
    }
}
impl ::core::cmp::PartialEq for CorePhysicalKeyStatus {
    fn eq(&self, other: &Self) -> bool {
        self.RepeatCount == other.RepeatCount && self.ScanCode == other.ScanCode && self.IsExtendedKey == other.IsExtendedKey && self.IsMenuKeyDown == other.IsMenuKeyDown && self.WasKeyDown == other.WasKeyDown && self.IsKeyReleased == other.IsKeyReleased
    }
}
impl ::core::cmp::Eq for CorePhysicalKeyStatus {}
unsafe impl ::windows::core::Abi for CorePhysicalKeyStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CorePhysicalKeyStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.UI.Core.CorePhysicalKeyStatus;u4;u4;b1;b1;b1;b1)");
}
impl ::windows::core::DefaultType for CorePhysicalKeyStatus {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CoreProcessEventsOption(pub i32);
impl CoreProcessEventsOption {
    pub const ProcessOneAndAllPending: CoreProcessEventsOption = CoreProcessEventsOption(0i32);
    pub const ProcessOneIfPresent: CoreProcessEventsOption = CoreProcessEventsOption(1i32);
    pub const ProcessUntilQuit: CoreProcessEventsOption = CoreProcessEventsOption(2i32);
    pub const ProcessAllIfPresent: CoreProcessEventsOption = CoreProcessEventsOption(3i32);
}
impl ::core::convert::From<i32> for CoreProcessEventsOption {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CoreProcessEventsOption {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CoreProcessEventsOption {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Core.CoreProcessEventsOption;i4)");
}
impl ::windows::core::DefaultType for CoreProcessEventsOption {
    type DefaultType = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Foundation")]
pub struct CoreProximityEvaluation {
    pub Score: i32,
    pub AdjustedPoint: super::super::Foundation::Point,
}
#[cfg(feature = "Foundation")]
impl CoreProximityEvaluation {}
#[cfg(feature = "Foundation")]
impl ::core::default::Default for CoreProximityEvaluation {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Foundation")]
impl ::core::fmt::Debug for CoreProximityEvaluation {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("CoreProximityEvaluation").field("Score", &self.Score).field("AdjustedPoint", &self.AdjustedPoint).finish()
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::PartialEq for CoreProximityEvaluation {
    fn eq(&self, other: &Self) -> bool {
        self.Score == other.Score && self.AdjustedPoint == other.AdjustedPoint
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::Eq for CoreProximityEvaluation {}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::Abi for CoreProximityEvaluation {
    type Abi = Self;
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::RuntimeType for CoreProximityEvaluation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.UI.Core.CoreProximityEvaluation;i4;struct(Windows.Foundation.Point;f4;f4))");
}
#[cfg(feature = "Foundation")]
impl ::windows::core::DefaultType for CoreProximityEvaluation {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CoreProximityEvaluationScore(pub i32);
impl CoreProximityEvaluationScore {
    pub const Closest: CoreProximityEvaluationScore = CoreProximityEvaluationScore(0i32);
    pub const Farthest: CoreProximityEvaluationScore = CoreProximityEvaluationScore(2147483647i32);
}
impl ::core::convert::From<i32> for CoreProximityEvaluationScore {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CoreProximityEvaluationScore {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CoreProximityEvaluationScore {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Core.CoreProximityEvaluationScore;i4)");
}
impl ::windows::core::DefaultType for CoreProximityEvaluationScore {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CoreVirtualKeyStates(pub u32);
impl CoreVirtualKeyStates {
    pub const None: CoreVirtualKeyStates = CoreVirtualKeyStates(0u32);
    pub const Down: CoreVirtualKeyStates = CoreVirtualKeyStates(1u32);
    pub const Locked: CoreVirtualKeyStates = CoreVirtualKeyStates(2u32);
}
impl ::core::convert::From<u32> for CoreVirtualKeyStates {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CoreVirtualKeyStates {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CoreVirtualKeyStates {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Core.CoreVirtualKeyStates;u4)");
}
impl ::windows::core::DefaultType for CoreVirtualKeyStates {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for CoreVirtualKeyStates {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for CoreVirtualKeyStates {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for CoreVirtualKeyStates {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for CoreVirtualKeyStates {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for CoreVirtualKeyStates {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CoreWindow(pub ::windows::core::IInspectable);
impl CoreWindow {
    pub fn AutomationHostProvider(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Bounds(&self) -> ::windows::core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CustomProperties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IPropertySet>(result__)
        }
    }
    pub fn Dispatcher(&self) -> ::windows::core::Result<CoreDispatcher> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreDispatcher>(result__)
        }
    }
    pub fn FlowDirection(&self) -> ::windows::core::Result<CoreWindowFlowDirection> {
        let this = self;
        unsafe {
            let mut result__: CoreWindowFlowDirection = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreWindowFlowDirection>(result__)
        }
    }
    pub fn SetFlowDirection(&self, value: CoreWindowFlowDirection) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IsInputEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsInputEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn PointerCursor(&self) -> ::windows::core::Result<CoreCursor> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreCursor>(result__)
        }
    }
    pub fn SetPointerCursor<'a, Param0: ::windows::core::IntoParam<'a, CoreCursor>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PointerPosition(&self) -> ::windows::core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Point>(result__)
        }
    }
    pub fn Visible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn Activate(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "System")]
    pub fn GetAsyncKeyState(&self, virtualkey: super::super::System::VirtualKey) -> ::windows::core::Result<CoreVirtualKeyStates> {
        let this = self;
        unsafe {
            let mut result__: CoreVirtualKeyStates = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), virtualkey, &mut result__).from_abi::<CoreVirtualKeyStates>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn GetKeyState(&self, virtualkey: super::super::System::VirtualKey) -> ::windows::core::Result<CoreVirtualKeyStates> {
        let this = self;
        unsafe {
            let mut result__: CoreVirtualKeyStates = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), virtualkey, &mut result__).from_abi::<CoreVirtualKeyStates>(result__)
        }
    }
    pub fn ReleasePointerCapture(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn SetPointerCapture(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Activated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<CoreWindow, WindowActivatedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveActivated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn AutomationProviderRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<CoreWindow, AutomationProviderRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveAutomationProviderRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn CharacterReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<CoreWindow, CharacterReceivedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveCharacterReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Closed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<CoreWindow, CoreWindowEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).30)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveClosed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).31)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn InputEnabled<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<CoreWindow, InputEnabledEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).32)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveInputEnabled<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).33)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn KeyDown<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<CoreWindow, KeyEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).34)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveKeyDown<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).35)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn KeyUp<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<CoreWindow, KeyEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).36)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveKeyUp<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).37)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PointerCaptureLost<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).38)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerCaptureLost<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).39)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PointerEntered<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).40)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerEntered<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).41)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PointerExited<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).42)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerExited<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).43)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PointerMoved<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).44)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerMoved<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).45)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PointerPressed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).46)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerPressed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).47)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PointerReleased<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).48)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerReleased<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).49)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn TouchHitTesting<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<CoreWindow, TouchHitTestingEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).50)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveTouchHitTesting<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).51)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PointerWheelChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).52)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerWheelChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).53)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn SizeChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<CoreWindow, WindowSizeChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).54)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveSizeChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).55)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn VisibilityChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<CoreWindow, VisibilityChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).56)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveVisibilityChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).57)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetPointerPosition<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Point>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWindow2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn GetForCurrentThread() -> ::windows::core::Result<CoreWindow> {
        Self::ICoreWindowStatic(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreWindow>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn PointerRoutedAway<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ICorePointerRedirector, PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICorePointerRedirector>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerRoutedAway<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICorePointerRedirector>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PointerRoutedTo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ICorePointerRedirector, PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICorePointerRedirector>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerRoutedTo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICorePointerRedirector>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PointerRoutedReleased<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ICorePointerRedirector, PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICorePointerRedirector>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerRoutedReleased<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICorePointerRedirector>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn ClosestInteractiveBoundsRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<CoreWindow, ClosestInteractiveBoundsRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICoreWindow3>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveClosestInteractiveBoundsRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWindow3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    pub fn GetCurrentKeyEventDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ICoreWindow3>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ResizeStarted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<CoreWindow, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICoreWindow4>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveResizeStarted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWindow4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn ResizeCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<CoreWindow, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICoreWindow4>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveResizeCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWindow4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows::core::Result<super::super::System::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<ICoreWindow5>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::DispatcherQueue>(result__)
        }
    }
    pub fn ActivationMode(&self) -> ::windows::core::Result<CoreWindowActivationMode> {
        let this = &::windows::core::Interface::cast::<ICoreWindow5>(self)?;
        unsafe {
            let mut result__: CoreWindowActivationMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreWindowActivationMode>(result__)
        }
    }
    pub fn UIContext(&self) -> ::windows::core::Result<super::UIContext> {
        let this = &::windows::core::Interface::cast::<ICoreWindowWithContext>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::UIContext>(result__)
        }
    }
    pub fn ICoreWindowStatic<R, F: FnOnce(&ICoreWindowStatic) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CoreWindow, ICoreWindowStatic> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWindow {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Core.CoreWindow;{79b9d5f2-879e-4b89-b798-79e47598030c})");
}
unsafe impl ::windows::core::Interface for CoreWindow {
    type Vtable = ICoreWindow_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79b9d5f2_879e_4b89_b798_79e47598030c);
}
impl ::windows::core::RuntimeName for CoreWindow {
    const NAME: &'static str = "Windows.UI.Core.CoreWindow";
}
impl ::core::convert::From<CoreWindow> for ::windows::core::IUnknown {
    fn from(value: CoreWindow) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreWindow> for ::windows::core::IUnknown {
    fn from(value: &CoreWindow) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreWindow {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreWindow {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreWindow> for ::windows::core::IInspectable {
    fn from(value: CoreWindow) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreWindow> for ::windows::core::IInspectable {
    fn from(value: &CoreWindow) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreWindow {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreWindow {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<CoreWindow> for ICoreWindow {
    fn from(value: CoreWindow) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWindow> for ICoreWindow {
    fn from(value: &CoreWindow) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICoreWindow> for CoreWindow {
    fn into_param(self) -> ::windows::core::Param<'a, ICoreWindow> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICoreWindow> for &CoreWindow {
    fn into_param(self) -> ::windows::core::Param<'a, ICoreWindow> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<CoreWindow> for ICorePointerRedirector {
    type Error = ::windows::core::Error;
    fn try_from(value: CoreWindow) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CoreWindow> for ICorePointerRedirector {
    type Error = ::windows::core::Error;
    fn try_from(value: &CoreWindow) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICorePointerRedirector> for CoreWindow {
    fn into_param(self) -> ::windows::core::Param<'a, ICorePointerRedirector> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICorePointerRedirector> for &CoreWindow {
    fn into_param(self) -> ::windows::core::Param<'a, ICorePointerRedirector> {
        ::core::convert::TryInto::<ICorePointerRedirector>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CoreWindowActivationMode(pub i32);
impl CoreWindowActivationMode {
    pub const None: CoreWindowActivationMode = CoreWindowActivationMode(0i32);
    pub const Deactivated: CoreWindowActivationMode = CoreWindowActivationMode(1i32);
    pub const ActivatedNotForeground: CoreWindowActivationMode = CoreWindowActivationMode(2i32);
    pub const ActivatedInForeground: CoreWindowActivationMode = CoreWindowActivationMode(3i32);
}
impl ::core::convert::From<i32> for CoreWindowActivationMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CoreWindowActivationMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CoreWindowActivationMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Core.CoreWindowActivationMode;i4)");
}
impl ::windows::core::DefaultType for CoreWindowActivationMode {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CoreWindowActivationState(pub i32);
impl CoreWindowActivationState {
    pub const CodeActivated: CoreWindowActivationState = CoreWindowActivationState(0i32);
    pub const Deactivated: CoreWindowActivationState = CoreWindowActivationState(1i32);
    pub const PointerActivated: CoreWindowActivationState = CoreWindowActivationState(2i32);
}
impl ::core::convert::From<i32> for CoreWindowActivationState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CoreWindowActivationState {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CoreWindowActivationState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Core.CoreWindowActivationState;i4)");
}
impl ::windows::core::DefaultType for CoreWindowActivationState {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CoreWindowDialog(pub ::windows::core::IInspectable);
impl CoreWindowDialog {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CoreWindowDialog, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    pub fn Showing<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<CoreWindow, CoreWindowPopupShowingEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveShowing<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn MaxSize(&self) -> ::windows::core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Size = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Size>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn MinSize(&self) -> ::windows::core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Size = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Size>(result__)
        }
    }
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetTitle<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn IsInteractionDelayed(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn SetIsInteractionDelayed(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Popups"))]
    pub fn Commands(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Popups::IUICommand>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::Popups::IUICommand>>(result__)
        }
    }
    pub fn DefaultCommandIndex(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn SetDefaultCommandIndex(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn CancelCommandIndex(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn SetCancelCommandIndex(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "UI_Popups")]
    pub fn BackButtonCommand(&self) -> ::windows::core::Result<super::Popups::UICommandInvokedHandler> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Popups::UICommandInvokedHandler>(result__)
        }
    }
    #[cfg(feature = "UI_Popups")]
    pub fn SetBackButtonCommand<'a, Param0: ::windows::core::IntoParam<'a, super::Popups::UICommandInvokedHandler>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub fn ShowAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::Popups::IUICommand>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::Popups::IUICommand>>(result__)
        }
    }
    pub fn CreateWithTitle<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(title: Param0) -> ::windows::core::Result<CoreWindowDialog> {
        Self::ICoreWindowDialogFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), title.into_param().abi(), &mut result__).from_abi::<CoreWindowDialog>(result__)
        })
    }
    pub fn ICoreWindowDialogFactory<R, F: FnOnce(&ICoreWindowDialogFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CoreWindowDialog, ICoreWindowDialogFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWindowDialog {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Core.CoreWindowDialog;{e7392ce0-c78d-427e-8b2c-01ff420c69d5})");
}
unsafe impl ::windows::core::Interface for CoreWindowDialog {
    type Vtable = ICoreWindowDialog_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe7392ce0_c78d_427e_8b2c_01ff420c69d5);
}
impl ::windows::core::RuntimeName for CoreWindowDialog {
    const NAME: &'static str = "Windows.UI.Core.CoreWindowDialog";
}
impl ::core::convert::From<CoreWindowDialog> for ::windows::core::IUnknown {
    fn from(value: CoreWindowDialog) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreWindowDialog> for ::windows::core::IUnknown {
    fn from(value: &CoreWindowDialog) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreWindowDialog {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreWindowDialog {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreWindowDialog> for ::windows::core::IInspectable {
    fn from(value: CoreWindowDialog) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreWindowDialog> for ::windows::core::IInspectable {
    fn from(value: &CoreWindowDialog) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreWindowDialog {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreWindowDialog {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CoreWindowEventArgs(pub ::windows::core::IInspectable);
impl CoreWindowEventArgs {
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
}
unsafe impl ::windows::core::RuntimeType for CoreWindowEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Core.CoreWindowEventArgs;{272b1ef3-c633-4da5-a26c-c6d0f56b29da})");
}
unsafe impl ::windows::core::Interface for CoreWindowEventArgs {
    type Vtable = ICoreWindowEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x272b1ef3_c633_4da5_a26c_c6d0f56b29da);
}
impl ::windows::core::RuntimeName for CoreWindowEventArgs {
    const NAME: &'static str = "Windows.UI.Core.CoreWindowEventArgs";
}
impl ::core::convert::From<CoreWindowEventArgs> for ::windows::core::IUnknown {
    fn from(value: CoreWindowEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreWindowEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CoreWindowEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreWindowEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreWindowEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreWindowEventArgs> for ::windows::core::IInspectable {
    fn from(value: CoreWindowEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreWindowEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CoreWindowEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreWindowEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreWindowEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<CoreWindowEventArgs> for ICoreWindowEventArgs {
    fn from(value: CoreWindowEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWindowEventArgs> for ICoreWindowEventArgs {
    fn from(value: &CoreWindowEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICoreWindowEventArgs> for CoreWindowEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ICoreWindowEventArgs> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICoreWindowEventArgs> for &CoreWindowEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ICoreWindowEventArgs> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CoreWindowFlowDirection(pub i32);
impl CoreWindowFlowDirection {
    pub const LeftToRight: CoreWindowFlowDirection = CoreWindowFlowDirection(0i32);
    pub const RightToLeft: CoreWindowFlowDirection = CoreWindowFlowDirection(1i32);
}
impl ::core::convert::From<i32> for CoreWindowFlowDirection {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CoreWindowFlowDirection {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CoreWindowFlowDirection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Core.CoreWindowFlowDirection;i4)");
}
impl ::windows::core::DefaultType for CoreWindowFlowDirection {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CoreWindowFlyout(pub ::windows::core::IInspectable);
impl CoreWindowFlyout {
    #[cfg(feature = "Foundation")]
    pub fn Showing<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<CoreWindow, CoreWindowPopupShowingEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveShowing<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn MaxSize(&self) -> ::windows::core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Size = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Size>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn MinSize(&self) -> ::windows::core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Size = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Size>(result__)
        }
    }
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetTitle<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn IsInteractionDelayed(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn SetIsInteractionDelayed(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Popups"))]
    pub fn Commands(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Popups::IUICommand>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::Popups::IUICommand>>(result__)
        }
    }
    pub fn DefaultCommandIndex(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn SetDefaultCommandIndex(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "UI_Popups")]
    pub fn BackButtonCommand(&self) -> ::windows::core::Result<super::Popups::UICommandInvokedHandler> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Popups::UICommandInvokedHandler>(result__)
        }
    }
    #[cfg(feature = "UI_Popups")]
    pub fn SetBackButtonCommand<'a, Param0: ::windows::core::IntoParam<'a, super::Popups::UICommandInvokedHandler>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub fn ShowAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::Popups::IUICommand>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::Popups::IUICommand>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Point>>(position: Param0) -> ::windows::core::Result<CoreWindowFlyout> {
        Self::ICoreWindowFlyoutFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), position.into_param().abi(), &mut result__).from_abi::<CoreWindowFlyout>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn CreateWithTitle<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Point>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(position: Param0, title: Param1) -> ::windows::core::Result<CoreWindowFlyout> {
        Self::ICoreWindowFlyoutFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), position.into_param().abi(), title.into_param().abi(), &mut result__).from_abi::<CoreWindowFlyout>(result__)
        })
    }
    pub fn ICoreWindowFlyoutFactory<R, F: FnOnce(&ICoreWindowFlyoutFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CoreWindowFlyout, ICoreWindowFlyoutFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWindowFlyout {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Core.CoreWindowFlyout;{e89d854d-2050-40bb-b344-f6f355eeb314})");
}
unsafe impl ::windows::core::Interface for CoreWindowFlyout {
    type Vtable = ICoreWindowFlyout_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe89d854d_2050_40bb_b344_f6f355eeb314);
}
impl ::windows::core::RuntimeName for CoreWindowFlyout {
    const NAME: &'static str = "Windows.UI.Core.CoreWindowFlyout";
}
impl ::core::convert::From<CoreWindowFlyout> for ::windows::core::IUnknown {
    fn from(value: CoreWindowFlyout) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreWindowFlyout> for ::windows::core::IUnknown {
    fn from(value: &CoreWindowFlyout) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreWindowFlyout {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreWindowFlyout {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreWindowFlyout> for ::windows::core::IInspectable {
    fn from(value: CoreWindowFlyout) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreWindowFlyout> for ::windows::core::IInspectable {
    fn from(value: &CoreWindowFlyout) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreWindowFlyout {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreWindowFlyout {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CoreWindowPopupShowingEventArgs(pub ::windows::core::IInspectable);
impl CoreWindowPopupShowingEventArgs {
    #[cfg(feature = "Foundation")]
    pub fn SetDesiredSize<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Size>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWindowPopupShowingEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Core.CoreWindowPopupShowingEventArgs;{26155fa2-5ba5-4ea4-a3b4-2dc7d63c8e26})");
}
unsafe impl ::windows::core::Interface for CoreWindowPopupShowingEventArgs {
    type Vtable = ICoreWindowPopupShowingEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x26155fa2_5ba5_4ea4_a3b4_2dc7d63c8e26);
}
impl ::windows::core::RuntimeName for CoreWindowPopupShowingEventArgs {
    const NAME: &'static str = "Windows.UI.Core.CoreWindowPopupShowingEventArgs";
}
impl ::core::convert::From<CoreWindowPopupShowingEventArgs> for ::windows::core::IUnknown {
    fn from(value: CoreWindowPopupShowingEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreWindowPopupShowingEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CoreWindowPopupShowingEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreWindowPopupShowingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreWindowPopupShowingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreWindowPopupShowingEventArgs> for ::windows::core::IInspectable {
    fn from(value: CoreWindowPopupShowingEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreWindowPopupShowingEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CoreWindowPopupShowingEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreWindowPopupShowingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreWindowPopupShowingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CoreWindowResizeManager(pub ::windows::core::IInspectable);
impl CoreWindowResizeManager {
    pub fn NotifyLayoutCompleted(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn SetShouldWaitForLayoutCompletion(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWindowResizeManagerLayoutCapability>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ShouldWaitForLayoutCompletion(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICoreWindowResizeManagerLayoutCapability>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn GetForCurrentView() -> ::windows::core::Result<CoreWindowResizeManager> {
        Self::ICoreWindowResizeManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreWindowResizeManager>(result__)
        })
    }
    pub fn ICoreWindowResizeManagerStatics<R, F: FnOnce(&ICoreWindowResizeManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CoreWindowResizeManager, ICoreWindowResizeManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWindowResizeManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Core.CoreWindowResizeManager;{b8f0b925-b350-48b3-a198-5c1a84700243})");
}
unsafe impl ::windows::core::Interface for CoreWindowResizeManager {
    type Vtable = ICoreWindowResizeManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb8f0b925_b350_48b3_a198_5c1a84700243);
}
impl ::windows::core::RuntimeName for CoreWindowResizeManager {
    const NAME: &'static str = "Windows.UI.Core.CoreWindowResizeManager";
}
impl ::core::convert::From<CoreWindowResizeManager> for ::windows::core::IUnknown {
    fn from(value: CoreWindowResizeManager) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreWindowResizeManager> for ::windows::core::IUnknown {
    fn from(value: &CoreWindowResizeManager) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreWindowResizeManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreWindowResizeManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreWindowResizeManager> for ::windows::core::IInspectable {
    fn from(value: CoreWindowResizeManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreWindowResizeManager> for ::windows::core::IInspectable {
    fn from(value: &CoreWindowResizeManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreWindowResizeManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreWindowResizeManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CoreWindowResizeManager {}
unsafe impl ::core::marker::Sync for CoreWindowResizeManager {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DispatchedHandler(::windows::core::IUnknown);
impl DispatchedHandler {
    pub fn new<F: FnMut() -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = DispatchedHandler_box::<F> {
            vtable: &DispatchedHandler_box::<F>::VTABLE,
            count: ::windows::core::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for DispatchedHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"delegate({d1f276c4-98d8-4636-bf49-eb79507548e9})");
}
unsafe impl ::windows::core::Interface for DispatchedHandler {
    type Vtable = DispatchedHandler_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd1f276c4_98d8_4636_bf49_eb79507548e9);
}
#[repr(C)]
#[doc(hidden)]
pub struct DispatchedHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(C)]
struct DispatchedHandler_box<F: FnMut() -> ::windows::core::Result<()> + 'static> {
    vtable: *const DispatchedHandler_abi,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut() -> ::windows::core::Result<()> + 'static> DispatchedHandler_box<F> {
    const VTABLE: DispatchedHandler_abi = DispatchedHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<DispatchedHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::core::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)().into()
    }
}
#[repr(transparent)]
#[doc(hidden)]
pub struct IAcceleratorKeyEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAcceleratorKeyEventArgs {
    type Vtable = IAcceleratorKeyEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xff1c4c4a_9287_470b_836e_9086e3126ade);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAcceleratorKeyEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut CoreAcceleratorKeyEventType) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::System::VirtualKey) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut CorePhysicalKeyStatus) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAcceleratorKeyEventArgs2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAcceleratorKeyEventArgs2 {
    type Vtable = IAcceleratorKeyEventArgs2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd300a9f6_2f7e_4873_a555_166e596ee1c5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAcceleratorKeyEventArgs2_abi(
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
pub struct IAutomationProviderRequestedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAutomationProviderRequestedEventArgs {
    type Vtable = IAutomationProviderRequestedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x961ff258_21bf_4b42_a298_fa479d4c52e2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationProviderRequestedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBackRequestedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBackRequestedEventArgs {
    type Vtable = IBackRequestedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd603d28a_e411_4a4e_ba41_6a327a8675bc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackRequestedEventArgs_abi(
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
pub struct ICharacterReceivedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICharacterReceivedEventArgs {
    type Vtable = ICharacterReceivedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc584659f_99b2_4bcc_bd33_04e63f42902e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICharacterReceivedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut CorePhysicalKeyStatus) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IClosestInteractiveBoundsRequestedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IClosestInteractiveBoundsRequestedEventArgs {
    type Vtable = IClosestInteractiveBoundsRequestedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x347c11d7_f6f8_40e3_b29f_ae50d3e86486);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClosestInteractiveBoundsRequestedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ICoreAcceleratorKeys(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreAcceleratorKeys {
    type Vtable = ICoreAcceleratorKeys_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9ffdf7f5_b8c9_4ef0_b7d2_1de626561fc8);
}
impl ICoreAcceleratorKeys {
    #[cfg(feature = "Foundation")]
    pub fn AcceleratorKeyActivated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<CoreDispatcher, AcceleratorKeyEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveAcceleratorKeyActivated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for ICoreAcceleratorKeys {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{9ffdf7f5-b8c9-4ef0-b7d2-1de626561fc8}");
}
impl ::core::convert::From<ICoreAcceleratorKeys> for ::windows::core::IUnknown {
    fn from(value: ICoreAcceleratorKeys) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ICoreAcceleratorKeys> for ::windows::core::IUnknown {
    fn from(value: &ICoreAcceleratorKeys) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICoreAcceleratorKeys {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ICoreAcceleratorKeys {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ICoreAcceleratorKeys> for ::windows::core::IInspectable {
    fn from(value: ICoreAcceleratorKeys) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICoreAcceleratorKeys> for ::windows::core::IInspectable {
    fn from(value: &ICoreAcceleratorKeys) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ICoreAcceleratorKeys {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ICoreAcceleratorKeys {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreAcceleratorKeys_abi(
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreClosestInteractiveBoundsRequested(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreClosestInteractiveBoundsRequested {
    type Vtable = ICoreClosestInteractiveBoundsRequested_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf303043a_e8bf_4e8e_ae69_c9dadd57a114);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreClosestInteractiveBoundsRequested_abi(
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreComponentFocusable(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreComponentFocusable {
    type Vtable = ICoreComponentFocusable_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x52f96fa3_8742_4411_ae69_79a85f29ac8b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreComponentFocusable_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreCursor(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreCursor {
    type Vtable = ICoreCursor_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96893acf_111d_442c_8a77_b87992f8e2d6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreCursor_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut CoreCursorType) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreCursorFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreCursorFactory {
    type Vtable = ICoreCursorFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf6359621_a79d_4ed3_8c32_a9ef9d6b76a4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreCursorFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, r#type: CoreCursorType, id: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreDispatcher(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreDispatcher {
    type Vtable = ICoreDispatcher_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x60db2fa8_b705_4fde_a7d6_ebbb1891d39e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreDispatcher_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, options: CoreProcessEventsOption) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, priority: CoreDispatcherPriority, agilecallback: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, agilecallback: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreDispatcher2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreDispatcher2 {
    type Vtable = ICoreDispatcher2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f5e63c7_e3aa_4eae_b0e0_dcf321ca4b2f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreDispatcher2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, priority: CoreDispatcherPriority, agilecallback: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, agilecallback: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreDispatcherWithTaskPriority(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreDispatcherWithTaskPriority {
    type Vtable = ICoreDispatcherWithTaskPriority_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbafaecad_484d_41be_ba80_1d58c65263ea);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreDispatcherWithTaskPriority_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut CoreDispatcherPriority) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: CoreDispatcherPriority) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, priority: CoreDispatcherPriority, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreIndependentInputSourceController(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreIndependentInputSourceController {
    type Vtable = ICoreIndependentInputSourceController_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0963261c_84fe_578a_83ca_6425309ccde4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreIndependentInputSourceController_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, inputtypes: CoreInputDeviceTypes) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, inputtypes: CoreInputDeviceTypes, required: CoreIndependentInputFilters, excluded: CoreIndependentInputFilters) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreIndependentInputSourceControllerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreIndependentInputSourceControllerStatics {
    type Vtable = ICoreIndependentInputSourceControllerStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3edc4e20_9a8a_5691_8586_fca4cb57526d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreIndependentInputSourceControllerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Composition")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, visual: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))] usize,
    #[cfg(feature = "UI_Composition")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, visualelement: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ICoreInputSourceBase(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreInputSourceBase {
    type Vtable = ICoreInputSourceBase_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f488807_4580_4be8_be68_92a9311713bb);
}
impl ICoreInputSourceBase {
    pub fn Dispatcher(&self) -> ::windows::core::Result<CoreDispatcher> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreDispatcher>(result__)
        }
    }
    pub fn IsInputEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsInputEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn InputEnabled<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, InputEnabledEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveInputEnabled<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for ICoreInputSourceBase {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{9f488807-4580-4be8-be68-92a9311713bb}");
}
impl ::core::convert::From<ICoreInputSourceBase> for ::windows::core::IUnknown {
    fn from(value: ICoreInputSourceBase) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ICoreInputSourceBase> for ::windows::core::IUnknown {
    fn from(value: &ICoreInputSourceBase) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICoreInputSourceBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ICoreInputSourceBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ICoreInputSourceBase> for ::windows::core::IInspectable {
    fn from(value: ICoreInputSourceBase) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICoreInputSourceBase> for ::windows::core::IInspectable {
    fn from(value: &ICoreInputSourceBase) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ICoreInputSourceBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ICoreInputSourceBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputSourceBase_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreKeyboardInputSource(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreKeyboardInputSource {
    type Vtable = ICoreKeyboardInputSource_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x231c9088_e469_4df1_b208_6e490d71cb90);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreKeyboardInputSource_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, virtualkey: super::super::System::VirtualKey, result__: *mut CoreVirtualKeyStates) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreKeyboardInputSource2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreKeyboardInputSource2 {
    type Vtable = ICoreKeyboardInputSource2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfa24cb94_f963_47a5_8778_207c482b0afd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreKeyboardInputSource2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ICorePointerInputSource(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICorePointerInputSource {
    type Vtable = ICorePointerInputSource_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbbf1bb18_e47a_48eb_8807_f8f8d3ea4551);
}
impl ICorePointerInputSource {
    pub fn ReleasePointerCapture(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn SetPointerCapture(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn HasCapture(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn PointerPosition(&self) -> ::windows::core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Point>(result__)
        }
    }
    pub fn PointerCursor(&self) -> ::windows::core::Result<CoreCursor> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreCursor>(result__)
        }
    }
    pub fn SetPointerCursor<'a, Param0: ::windows::core::IntoParam<'a, CoreCursor>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PointerCaptureLost<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerCaptureLost<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PointerEntered<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerEntered<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PointerExited<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerExited<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PointerMoved<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerMoved<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PointerPressed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerPressed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PointerReleased<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerReleased<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PointerWheelChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerWheelChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for ICorePointerInputSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{bbf1bb18-e47a-48eb-8807-f8f8d3ea4551}");
}
impl ::core::convert::From<ICorePointerInputSource> for ::windows::core::IUnknown {
    fn from(value: ICorePointerInputSource) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ICorePointerInputSource> for ::windows::core::IUnknown {
    fn from(value: &ICorePointerInputSource) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICorePointerInputSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ICorePointerInputSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ICorePointerInputSource> for ::windows::core::IInspectable {
    fn from(value: ICorePointerInputSource) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICorePointerInputSource> for ::windows::core::IInspectable {
    fn from(value: &ICorePointerInputSource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ICorePointerInputSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ICorePointerInputSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorePointerInputSource_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ICorePointerInputSource2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICorePointerInputSource2 {
    type Vtable = ICorePointerInputSource2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd703708a_4516_4786_b1e5_2751d563f997);
}
impl ICorePointerInputSource2 {
    pub fn ReleasePointerCapture(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn SetPointerCapture(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn HasCapture(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn PointerPosition(&self) -> ::windows::core::Result<super::super::Foundation::Point> {
        let this = &::windows::core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Point>(result__)
        }
    }
    pub fn PointerCursor(&self) -> ::windows::core::Result<CoreCursor> {
        let this = &::windows::core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreCursor>(result__)
        }
    }
    pub fn SetPointerCursor<'a, Param0: ::windows::core::IntoParam<'a, CoreCursor>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PointerCaptureLost<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerCaptureLost<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PointerEntered<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerEntered<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PointerExited<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerExited<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PointerMoved<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerMoved<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PointerPressed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerPressed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PointerReleased<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerReleased<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PointerWheelChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerWheelChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICorePointerInputSource>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows::core::Result<super::super::System::DispatcherQueue> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::DispatcherQueue>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ICorePointerInputSource2 {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{d703708a-4516-4786-b1e5-2751d563f997}");
}
impl ::core::convert::From<ICorePointerInputSource2> for ::windows::core::IUnknown {
    fn from(value: ICorePointerInputSource2) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ICorePointerInputSource2> for ::windows::core::IUnknown {
    fn from(value: &ICorePointerInputSource2) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICorePointerInputSource2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ICorePointerInputSource2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ICorePointerInputSource2> for ::windows::core::IInspectable {
    fn from(value: ICorePointerInputSource2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICorePointerInputSource2> for ::windows::core::IInspectable {
    fn from(value: &ICorePointerInputSource2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ICorePointerInputSource2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ICorePointerInputSource2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<ICorePointerInputSource2> for ICorePointerInputSource {
    type Error = ::windows::core::Error;
    fn try_from(value: ICorePointerInputSource2) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ICorePointerInputSource2> for ICorePointerInputSource {
    type Error = ::windows::core::Error;
    fn try_from(value: &ICorePointerInputSource2) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICorePointerInputSource> for ICorePointerInputSource2 {
    fn into_param(self) -> ::windows::core::Param<'a, ICorePointerInputSource> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICorePointerInputSource> for &ICorePointerInputSource2 {
    fn into_param(self) -> ::windows::core::Param<'a, ICorePointerInputSource> {
        ::core::convert::TryInto::<ICorePointerInputSource>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorePointerInputSource2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ICorePointerRedirector(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICorePointerRedirector {
    type Vtable = ICorePointerRedirector_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f9d0c94_5688_4b0c_a9f1_f931f7fa3dc3);
}
impl ICorePointerRedirector {
    #[cfg(feature = "Foundation")]
    pub fn PointerRoutedAway<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ICorePointerRedirector, PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerRoutedAway<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PointerRoutedTo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ICorePointerRedirector, PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerRoutedTo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PointerRoutedReleased<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ICorePointerRedirector, PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerRoutedReleased<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for ICorePointerRedirector {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{8f9d0c94-5688-4b0c-a9f1-f931f7fa3dc3}");
}
impl ::core::convert::From<ICorePointerRedirector> for ::windows::core::IUnknown {
    fn from(value: ICorePointerRedirector) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ICorePointerRedirector> for ::windows::core::IUnknown {
    fn from(value: &ICorePointerRedirector) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICorePointerRedirector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ICorePointerRedirector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ICorePointerRedirector> for ::windows::core::IInspectable {
    fn from(value: ICorePointerRedirector) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICorePointerRedirector> for ::windows::core::IInspectable {
    fn from(value: &ICorePointerRedirector) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ICorePointerRedirector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ICorePointerRedirector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorePointerRedirector_abi(
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
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreTouchHitTesting(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreTouchHitTesting {
    type Vtable = ICoreTouchHitTesting_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb1d8a289_3acf_4124_9fa3_ea8aba353c21);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTouchHitTesting_abi(
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
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ICoreWindow(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWindow {
    type Vtable = ICoreWindow_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79b9d5f2_879e_4b89_b798_79e47598030c);
}
impl ICoreWindow {
    pub fn AutomationHostProvider(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Bounds(&self) -> ::windows::core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CustomProperties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IPropertySet>(result__)
        }
    }
    pub fn Dispatcher(&self) -> ::windows::core::Result<CoreDispatcher> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreDispatcher>(result__)
        }
    }
    pub fn FlowDirection(&self) -> ::windows::core::Result<CoreWindowFlowDirection> {
        let this = self;
        unsafe {
            let mut result__: CoreWindowFlowDirection = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreWindowFlowDirection>(result__)
        }
    }
    pub fn SetFlowDirection(&self, value: CoreWindowFlowDirection) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IsInputEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsInputEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn PointerCursor(&self) -> ::windows::core::Result<CoreCursor> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreCursor>(result__)
        }
    }
    pub fn SetPointerCursor<'a, Param0: ::windows::core::IntoParam<'a, CoreCursor>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PointerPosition(&self) -> ::windows::core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Point>(result__)
        }
    }
    pub fn Visible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn Activate(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "System")]
    pub fn GetAsyncKeyState(&self, virtualkey: super::super::System::VirtualKey) -> ::windows::core::Result<CoreVirtualKeyStates> {
        let this = self;
        unsafe {
            let mut result__: CoreVirtualKeyStates = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), virtualkey, &mut result__).from_abi::<CoreVirtualKeyStates>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn GetKeyState(&self, virtualkey: super::super::System::VirtualKey) -> ::windows::core::Result<CoreVirtualKeyStates> {
        let this = self;
        unsafe {
            let mut result__: CoreVirtualKeyStates = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), virtualkey, &mut result__).from_abi::<CoreVirtualKeyStates>(result__)
        }
    }
    pub fn ReleasePointerCapture(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn SetPointerCapture(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Activated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<CoreWindow, WindowActivatedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveActivated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn AutomationProviderRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<CoreWindow, AutomationProviderRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveAutomationProviderRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn CharacterReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<CoreWindow, CharacterReceivedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveCharacterReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Closed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<CoreWindow, CoreWindowEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).30)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveClosed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).31)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn InputEnabled<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<CoreWindow, InputEnabledEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).32)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveInputEnabled<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).33)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn KeyDown<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<CoreWindow, KeyEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).34)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveKeyDown<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).35)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn KeyUp<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<CoreWindow, KeyEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).36)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveKeyUp<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).37)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PointerCaptureLost<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).38)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerCaptureLost<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).39)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PointerEntered<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).40)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerEntered<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).41)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PointerExited<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).42)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerExited<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).43)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PointerMoved<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).44)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerMoved<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).45)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PointerPressed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).46)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerPressed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).47)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PointerReleased<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).48)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerReleased<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).49)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn TouchHitTesting<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<CoreWindow, TouchHitTestingEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).50)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveTouchHitTesting<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).51)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PointerWheelChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).52)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerWheelChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).53)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn SizeChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<CoreWindow, WindowSizeChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).54)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveSizeChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).55)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn VisibilityChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<CoreWindow, VisibilityChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).56)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveVisibilityChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).57)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for ICoreWindow {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{79b9d5f2-879e-4b89-b798-79e47598030c}");
}
impl ::core::convert::From<ICoreWindow> for ::windows::core::IUnknown {
    fn from(value: ICoreWindow) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ICoreWindow> for ::windows::core::IUnknown {
    fn from(value: &ICoreWindow) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICoreWindow {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ICoreWindow {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ICoreWindow> for ::windows::core::IInspectable {
    fn from(value: ICoreWindow) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICoreWindow> for ::windows::core::IInspectable {
    fn from(value: &ICoreWindow) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ICoreWindow {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ICoreWindow {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWindow_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut CoreWindowFlowDirection) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: CoreWindowFlowDirection) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, virtualkey: super::super::System::VirtualKey, result__: *mut CoreVirtualKeyStates) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, virtualkey: super::super::System::VirtualKey, result__: *mut CoreVirtualKeyStates) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreWindow2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWindow2 {
    type Vtable = ICoreWindow2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7c2b1b85_6917_4361_9c02_0d9e3a420b95);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWindow2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreWindow3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWindow3 {
    type Vtable = ICoreWindow3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x32c20dd8_faef_4375_a2ab_32640e4815c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWindow3_abi(
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
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreWindow4(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWindow4 {
    type Vtable = ICoreWindow4_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x35caf0d0_47f0_436c_af97_0dd88f6f5f02);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWindow4_abi(
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
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreWindow5(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWindow5 {
    type Vtable = ICoreWindow5_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4b4ae1e1_2e6d_4eaa_bda1_1c5cc1bee141);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWindow5_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut CoreWindowActivationMode) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreWindowDialog(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWindowDialog {
    type Vtable = ICoreWindowDialog_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe7392ce0_c78d_427e_8b2c_01ff420c69d5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWindowDialog_abi(
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
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Popups"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Popups")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Popups")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Popups"))] usize,
    #[cfg(feature = "UI_Popups")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Popups"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreWindowDialogFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWindowDialogFactory {
    type Vtable = ICoreWindowDialogFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcfb2a855_1c59_4b13_b1e5_16e29805f7c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWindowDialogFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, title: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ICoreWindowEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWindowEventArgs {
    type Vtable = ICoreWindowEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x272b1ef3_c633_4da5_a26c_c6d0f56b29da);
}
impl ICoreWindowEventArgs {
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
}
unsafe impl ::windows::core::RuntimeType for ICoreWindowEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{272b1ef3-c633-4da5-a26c-c6d0f56b29da}");
}
impl ::core::convert::From<ICoreWindowEventArgs> for ::windows::core::IUnknown {
    fn from(value: ICoreWindowEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ICoreWindowEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ICoreWindowEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICoreWindowEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ICoreWindowEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ICoreWindowEventArgs> for ::windows::core::IInspectable {
    fn from(value: ICoreWindowEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICoreWindowEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ICoreWindowEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ICoreWindowEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ICoreWindowEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWindowEventArgs_abi(
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
pub struct ICoreWindowFlyout(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWindowFlyout {
    type Vtable = ICoreWindowFlyout_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe89d854d_2050_40bb_b344_f6f355eeb314);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWindowFlyout_abi(
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
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Popups"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Popups")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Popups")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Popups"))] usize,
    #[cfg(feature = "UI_Popups")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Popups"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreWindowFlyoutFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWindowFlyoutFactory {
    type Vtable = ICoreWindowFlyoutFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdec4c6c4_93e8_4f7c_be27_cefaa1af68a7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWindowFlyoutFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, position: super::super::Foundation::Point, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, position: super::super::Foundation::Point, title: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreWindowPopupShowingEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWindowPopupShowingEventArgs {
    type Vtable = ICoreWindowPopupShowingEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x26155fa2_5ba5_4ea4_a3b4_2dc7d63c8e26);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWindowPopupShowingEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreWindowResizeManager(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWindowResizeManager {
    type Vtable = ICoreWindowResizeManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb8f0b925_b350_48b3_a198_5c1a84700243);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWindowResizeManager_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreWindowResizeManagerLayoutCapability(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWindowResizeManagerLayoutCapability {
    type Vtable = ICoreWindowResizeManagerLayoutCapability_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb74f27b_a544_4301_80e6_0ae033ef4536);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWindowResizeManagerLayoutCapability_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreWindowResizeManagerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWindowResizeManagerStatics {
    type Vtable = ICoreWindowResizeManagerStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae4a9045_6d70_49db_8e68_46ffbd17d38d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWindowResizeManagerStatics_abi(
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
pub struct ICoreWindowStatic(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWindowStatic {
    type Vtable = ICoreWindowStatic_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d239005_3c2a_41b1_9022_536bb9cf93b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWindowStatic_abi(
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
pub struct ICoreWindowWithContext(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreWindowWithContext {
    type Vtable = ICoreWindowWithContext_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9ac40241_3575_4c3b_af66_e8c529d4d06c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWindowWithContext_abi(
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
pub struct IIdleDispatchedHandlerArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IIdleDispatchedHandlerArgs {
    type Vtable = IIdleDispatchedHandlerArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x98bb6a24_dc1c_43cb_b4ed_d1c0eb2391f3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIdleDispatchedHandlerArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IInitializeWithCoreWindow(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInitializeWithCoreWindow {
    type Vtable = IInitializeWithCoreWindow_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x188f20d6_9873_464a_ace5_57e010f465e6);
}
impl IInitializeWithCoreWindow {
    pub fn Initialize<'a, Param0: ::windows::core::IntoParam<'a, CoreWindow>>(&self, window: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), window.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for IInitializeWithCoreWindow {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{188f20d6-9873-464a-ace5-57e010f465e6}");
}
impl ::core::convert::From<IInitializeWithCoreWindow> for ::windows::core::IUnknown {
    fn from(value: IInitializeWithCoreWindow) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IInitializeWithCoreWindow> for ::windows::core::IUnknown {
    fn from(value: &IInitializeWithCoreWindow) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IInitializeWithCoreWindow {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IInitializeWithCoreWindow {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IInitializeWithCoreWindow> for ::windows::core::IInspectable {
    fn from(value: IInitializeWithCoreWindow) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IInitializeWithCoreWindow> for ::windows::core::IInspectable {
    fn from(value: &IInitializeWithCoreWindow) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IInitializeWithCoreWindow {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IInitializeWithCoreWindow {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInitializeWithCoreWindow_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, window: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInputEnabledEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInputEnabledEventArgs {
    type Vtable = IInputEnabledEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x80371d4f_2fd8_4c24_aa86_3163a87b4e5a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputEnabledEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IKeyEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IKeyEventArgs {
    type Vtable = IKeyEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5ff5e930_2544_4a17_bd78_1f2fdebb106b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::System::VirtualKey) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut CorePhysicalKeyStatus) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IKeyEventArgs2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IKeyEventArgs2 {
    type Vtable = IKeyEventArgs2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x583add98_0790_4571_9b12_645ef9d79e42);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyEventArgs2_abi(
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
pub struct IPointerEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPointerEventArgs {
    type Vtable = IPointerEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x920d9cb1_a5fc_4a21_8c09_49dfe6ffe25f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Input")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Input"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Input"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Input")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISystemNavigationManager(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISystemNavigationManager {
    type Vtable = ISystemNavigationManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x93023118_cf50_42a6_9706_69107fa122e1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemNavigationManager_abi(
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISystemNavigationManager2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISystemNavigationManager2 {
    type Vtable = ISystemNavigationManager2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8c510401_67be_49ae_9509_671c1e54a389);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemNavigationManager2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AppViewBackButtonVisibility) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: AppViewBackButtonVisibility) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISystemNavigationManagerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISystemNavigationManagerStatics {
    type Vtable = ISystemNavigationManagerStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdc52b5ce_bee0_4305_8c54_68228ed683b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemNavigationManagerStatics_abi(
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
pub struct ITouchHitTestingEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITouchHitTestingEventArgs {
    type Vtable = ITouchHitTestingEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x22f3b823_0b7c_424e_9df7_33d4f962931b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITouchHitTestingEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut CoreProximityEvaluation) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: CoreProximityEvaluation) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, controlboundingbox: super::super::Foundation::Rect, result__: *mut CoreProximityEvaluation) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, controlVertices_array_size: u32, controlvertices: *const super::super::Foundation::Point, result__: *mut CoreProximityEvaluation) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVisibilityChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVisibilityChangedEventArgs {
    type Vtable = IVisibilityChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbf9918ea_d801_4564_a495_b1e84f8ad085);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisibilityChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWindowActivatedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IWindowActivatedEventArgs {
    type Vtable = IWindowActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x179d65e7_4658_4cb6_aa13_41d094ea255e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut CoreWindowActivationState) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWindowSizeChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IWindowSizeChangedEventArgs {
    type Vtable = IWindowSizeChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5a200ec7_0426_47dc_b86c_6f475915e451);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowSizeChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IdleDispatchedHandler(::windows::core::IUnknown);
impl IdleDispatchedHandler {
    pub fn new<F: FnMut(&::core::option::Option<IdleDispatchedHandlerArgs>) -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = IdleDispatchedHandler_box::<F> {
            vtable: &IdleDispatchedHandler_box::<F>::VTABLE,
            count: ::windows::core::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, IdleDispatchedHandlerArgs>>(&self, e: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), e.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for IdleDispatchedHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"delegate({a42b0c24-7f21-4abc-99c1-8f01007f0880})");
}
unsafe impl ::windows::core::Interface for IdleDispatchedHandler {
    type Vtable = IdleDispatchedHandler_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa42b0c24_7f21_4abc_99c1_8f01007f0880);
}
#[repr(C)]
#[doc(hidden)]
pub struct IdleDispatchedHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(C)]
struct IdleDispatchedHandler_box<F: FnMut(&::core::option::Option<IdleDispatchedHandlerArgs>) -> ::windows::core::Result<()> + 'static> {
    vtable: *const IdleDispatchedHandler_abi,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<IdleDispatchedHandlerArgs>) -> ::windows::core::Result<()> + 'static> IdleDispatchedHandler_box<F> {
    const VTABLE: IdleDispatchedHandler_abi = IdleDispatchedHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<IdleDispatchedHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::core::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::core::RawPtr, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(&*(&e as *const <IdleDispatchedHandlerArgs as ::windows::core::Abi>::Abi as *const <IdleDispatchedHandlerArgs as ::windows::core::DefaultType>::DefaultType)).into()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IdleDispatchedHandlerArgs(pub ::windows::core::IInspectable);
impl IdleDispatchedHandlerArgs {
    pub fn IsDispatcherIdle(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IdleDispatchedHandlerArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Core.IdleDispatchedHandlerArgs;{98bb6a24-dc1c-43cb-b4ed-d1c0eb2391f3})");
}
unsafe impl ::windows::core::Interface for IdleDispatchedHandlerArgs {
    type Vtable = IIdleDispatchedHandlerArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x98bb6a24_dc1c_43cb_b4ed_d1c0eb2391f3);
}
impl ::windows::core::RuntimeName for IdleDispatchedHandlerArgs {
    const NAME: &'static str = "Windows.UI.Core.IdleDispatchedHandlerArgs";
}
impl ::core::convert::From<IdleDispatchedHandlerArgs> for ::windows::core::IUnknown {
    fn from(value: IdleDispatchedHandlerArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IdleDispatchedHandlerArgs> for ::windows::core::IUnknown {
    fn from(value: &IdleDispatchedHandlerArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IdleDispatchedHandlerArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IdleDispatchedHandlerArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IdleDispatchedHandlerArgs> for ::windows::core::IInspectable {
    fn from(value: IdleDispatchedHandlerArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IdleDispatchedHandlerArgs> for ::windows::core::IInspectable {
    fn from(value: &IdleDispatchedHandlerArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IdleDispatchedHandlerArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IdleDispatchedHandlerArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct InputEnabledEventArgs(pub ::windows::core::IInspectable);
impl InputEnabledEventArgs {
    pub fn InputEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICoreWindowEventArgs>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWindowEventArgs>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for InputEnabledEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Core.InputEnabledEventArgs;{80371d4f-2fd8-4c24-aa86-3163a87b4e5a})");
}
unsafe impl ::windows::core::Interface for InputEnabledEventArgs {
    type Vtable = IInputEnabledEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x80371d4f_2fd8_4c24_aa86_3163a87b4e5a);
}
impl ::windows::core::RuntimeName for InputEnabledEventArgs {
    const NAME: &'static str = "Windows.UI.Core.InputEnabledEventArgs";
}
impl ::core::convert::From<InputEnabledEventArgs> for ::windows::core::IUnknown {
    fn from(value: InputEnabledEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InputEnabledEventArgs> for ::windows::core::IUnknown {
    fn from(value: &InputEnabledEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InputEnabledEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InputEnabledEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InputEnabledEventArgs> for ::windows::core::IInspectable {
    fn from(value: InputEnabledEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InputEnabledEventArgs> for ::windows::core::IInspectable {
    fn from(value: &InputEnabledEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InputEnabledEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InputEnabledEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<InputEnabledEventArgs> for ICoreWindowEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: InputEnabledEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InputEnabledEventArgs> for ICoreWindowEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &InputEnabledEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICoreWindowEventArgs> for InputEnabledEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ICoreWindowEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICoreWindowEventArgs> for &InputEnabledEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ICoreWindowEventArgs> {
        ::core::convert::TryInto::<ICoreWindowEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct KeyEventArgs(pub ::windows::core::IInspectable);
impl KeyEventArgs {
    #[cfg(feature = "System")]
    pub fn VirtualKey(&self) -> ::windows::core::Result<super::super::System::VirtualKey> {
        let this = self;
        unsafe {
            let mut result__: super::super::System::VirtualKey = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::VirtualKey>(result__)
        }
    }
    pub fn KeyStatus(&self) -> ::windows::core::Result<CorePhysicalKeyStatus> {
        let this = self;
        unsafe {
            let mut result__: CorePhysicalKeyStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CorePhysicalKeyStatus>(result__)
        }
    }
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICoreWindowEventArgs>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWindowEventArgs>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IKeyEventArgs2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for KeyEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Core.KeyEventArgs;{5ff5e930-2544-4a17-bd78-1f2fdebb106b})");
}
unsafe impl ::windows::core::Interface for KeyEventArgs {
    type Vtable = IKeyEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5ff5e930_2544_4a17_bd78_1f2fdebb106b);
}
impl ::windows::core::RuntimeName for KeyEventArgs {
    const NAME: &'static str = "Windows.UI.Core.KeyEventArgs";
}
impl ::core::convert::From<KeyEventArgs> for ::windows::core::IUnknown {
    fn from(value: KeyEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&KeyEventArgs> for ::windows::core::IUnknown {
    fn from(value: &KeyEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for KeyEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a KeyEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<KeyEventArgs> for ::windows::core::IInspectable {
    fn from(value: KeyEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&KeyEventArgs> for ::windows::core::IInspectable {
    fn from(value: &KeyEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for KeyEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a KeyEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<KeyEventArgs> for ICoreWindowEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: KeyEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&KeyEventArgs> for ICoreWindowEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &KeyEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICoreWindowEventArgs> for KeyEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ICoreWindowEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICoreWindowEventArgs> for &KeyEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ICoreWindowEventArgs> {
        ::core::convert::TryInto::<ICoreWindowEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PointerEventArgs(pub ::windows::core::IInspectable);
impl PointerEventArgs {
    #[cfg(feature = "UI_Input")]
    pub fn CurrentPoint(&self) -> ::windows::core::Result<super::Input::PointerPoint> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Input::PointerPoint>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn KeyModifiers(&self) -> ::windows::core::Result<super::super::System::VirtualKeyModifiers> {
        let this = self;
        unsafe {
            let mut result__: super::super::System::VirtualKeyModifiers = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::VirtualKeyModifiers>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Input"))]
    pub fn GetIntermediatePoints(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Input::PointerPoint>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::Input::PointerPoint>>(result__)
        }
    }
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICoreWindowEventArgs>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWindowEventArgs>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for PointerEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Core.PointerEventArgs;{920d9cb1-a5fc-4a21-8c09-49dfe6ffe25f})");
}
unsafe impl ::windows::core::Interface for PointerEventArgs {
    type Vtable = IPointerEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x920d9cb1_a5fc_4a21_8c09_49dfe6ffe25f);
}
impl ::windows::core::RuntimeName for PointerEventArgs {
    const NAME: &'static str = "Windows.UI.Core.PointerEventArgs";
}
impl ::core::convert::From<PointerEventArgs> for ::windows::core::IUnknown {
    fn from(value: PointerEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PointerEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PointerEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PointerEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PointerEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PointerEventArgs> for ::windows::core::IInspectable {
    fn from(value: PointerEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PointerEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PointerEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PointerEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PointerEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<PointerEventArgs> for ICoreWindowEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: PointerEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PointerEventArgs> for ICoreWindowEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &PointerEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICoreWindowEventArgs> for PointerEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ICoreWindowEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICoreWindowEventArgs> for &PointerEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ICoreWindowEventArgs> {
        ::core::convert::TryInto::<ICoreWindowEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SystemNavigationManager(pub ::windows::core::IInspectable);
impl SystemNavigationManager {
    #[cfg(feature = "Foundation")]
    pub fn BackRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<BackRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveBackRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn AppViewBackButtonVisibility(&self) -> ::windows::core::Result<AppViewBackButtonVisibility> {
        let this = &::windows::core::Interface::cast::<ISystemNavigationManager2>(self)?;
        unsafe {
            let mut result__: AppViewBackButtonVisibility = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppViewBackButtonVisibility>(result__)
        }
    }
    pub fn SetAppViewBackButtonVisibility(&self, value: AppViewBackButtonVisibility) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISystemNavigationManager2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn GetForCurrentView() -> ::windows::core::Result<SystemNavigationManager> {
        Self::ISystemNavigationManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SystemNavigationManager>(result__)
        })
    }
    pub fn ISystemNavigationManagerStatics<R, F: FnOnce(&ISystemNavigationManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SystemNavigationManager, ISystemNavigationManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for SystemNavigationManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Core.SystemNavigationManager;{93023118-cf50-42a6-9706-69107fa122e1})");
}
unsafe impl ::windows::core::Interface for SystemNavigationManager {
    type Vtable = ISystemNavigationManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x93023118_cf50_42a6_9706_69107fa122e1);
}
impl ::windows::core::RuntimeName for SystemNavigationManager {
    const NAME: &'static str = "Windows.UI.Core.SystemNavigationManager";
}
impl ::core::convert::From<SystemNavigationManager> for ::windows::core::IUnknown {
    fn from(value: SystemNavigationManager) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SystemNavigationManager> for ::windows::core::IUnknown {
    fn from(value: &SystemNavigationManager) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SystemNavigationManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SystemNavigationManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SystemNavigationManager> for ::windows::core::IInspectable {
    fn from(value: SystemNavigationManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SystemNavigationManager> for ::windows::core::IInspectable {
    fn from(value: &SystemNavigationManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SystemNavigationManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SystemNavigationManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SystemNavigationManager {}
unsafe impl ::core::marker::Sync for SystemNavigationManager {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct TouchHitTestingEventArgs(pub ::windows::core::IInspectable);
impl TouchHitTestingEventArgs {
    #[cfg(feature = "Foundation")]
    pub fn ProximityEvaluation(&self) -> ::windows::core::Result<CoreProximityEvaluation> {
        let this = self;
        unsafe {
            let mut result__: CoreProximityEvaluation = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreProximityEvaluation>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetProximityEvaluation<'a, Param0: ::windows::core::IntoParam<'a, CoreProximityEvaluation>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Point(&self) -> ::windows::core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Point>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn BoundingBox(&self) -> ::windows::core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn EvaluateProximityToRect<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>>(&self, controlboundingbox: Param0) -> ::windows::core::Result<CoreProximityEvaluation> {
        let this = self;
        unsafe {
            let mut result__: CoreProximityEvaluation = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), controlboundingbox.into_param().abi(), &mut result__).from_abi::<CoreProximityEvaluation>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn EvaluateProximityToPolygon(&self, controlvertices: &[<super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<CoreProximityEvaluation> {
        let this = self;
        unsafe {
            let mut result__: CoreProximityEvaluation = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), controlvertices.len() as u32, ::core::mem::transmute(controlvertices.as_ptr()), &mut result__).from_abi::<CoreProximityEvaluation>(result__)
        }
    }
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICoreWindowEventArgs>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWindowEventArgs>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for TouchHitTestingEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Core.TouchHitTestingEventArgs;{22f3b823-0b7c-424e-9df7-33d4f962931b})");
}
unsafe impl ::windows::core::Interface for TouchHitTestingEventArgs {
    type Vtable = ITouchHitTestingEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x22f3b823_0b7c_424e_9df7_33d4f962931b);
}
impl ::windows::core::RuntimeName for TouchHitTestingEventArgs {
    const NAME: &'static str = "Windows.UI.Core.TouchHitTestingEventArgs";
}
impl ::core::convert::From<TouchHitTestingEventArgs> for ::windows::core::IUnknown {
    fn from(value: TouchHitTestingEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&TouchHitTestingEventArgs> for ::windows::core::IUnknown {
    fn from(value: &TouchHitTestingEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TouchHitTestingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TouchHitTestingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<TouchHitTestingEventArgs> for ::windows::core::IInspectable {
    fn from(value: TouchHitTestingEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&TouchHitTestingEventArgs> for ::windows::core::IInspectable {
    fn from(value: &TouchHitTestingEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TouchHitTestingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TouchHitTestingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<TouchHitTestingEventArgs> for ICoreWindowEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: TouchHitTestingEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&TouchHitTestingEventArgs> for ICoreWindowEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &TouchHitTestingEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICoreWindowEventArgs> for TouchHitTestingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ICoreWindowEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICoreWindowEventArgs> for &TouchHitTestingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ICoreWindowEventArgs> {
        ::core::convert::TryInto::<ICoreWindowEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct VisibilityChangedEventArgs(pub ::windows::core::IInspectable);
impl VisibilityChangedEventArgs {
    pub fn Visible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICoreWindowEventArgs>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWindowEventArgs>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for VisibilityChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Core.VisibilityChangedEventArgs;{bf9918ea-d801-4564-a495-b1e84f8ad085})");
}
unsafe impl ::windows::core::Interface for VisibilityChangedEventArgs {
    type Vtable = IVisibilityChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbf9918ea_d801_4564_a495_b1e84f8ad085);
}
impl ::windows::core::RuntimeName for VisibilityChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Core.VisibilityChangedEventArgs";
}
impl ::core::convert::From<VisibilityChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: VisibilityChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&VisibilityChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &VisibilityChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VisibilityChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a VisibilityChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<VisibilityChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: VisibilityChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&VisibilityChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &VisibilityChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VisibilityChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a VisibilityChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<VisibilityChangedEventArgs> for ICoreWindowEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: VisibilityChangedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&VisibilityChangedEventArgs> for ICoreWindowEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &VisibilityChangedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICoreWindowEventArgs> for VisibilityChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ICoreWindowEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICoreWindowEventArgs> for &VisibilityChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ICoreWindowEventArgs> {
        ::core::convert::TryInto::<ICoreWindowEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct WindowActivatedEventArgs(pub ::windows::core::IInspectable);
impl WindowActivatedEventArgs {
    pub fn WindowActivationState(&self) -> ::windows::core::Result<CoreWindowActivationState> {
        let this = self;
        unsafe {
            let mut result__: CoreWindowActivationState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreWindowActivationState>(result__)
        }
    }
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICoreWindowEventArgs>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWindowEventArgs>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for WindowActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Core.WindowActivatedEventArgs;{179d65e7-4658-4cb6-aa13-41d094ea255e})");
}
unsafe impl ::windows::core::Interface for WindowActivatedEventArgs {
    type Vtable = IWindowActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x179d65e7_4658_4cb6_aa13_41d094ea255e);
}
impl ::windows::core::RuntimeName for WindowActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.Core.WindowActivatedEventArgs";
}
impl ::core::convert::From<WindowActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WindowActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&WindowActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WindowActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WindowActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WindowActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<WindowActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WindowActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&WindowActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WindowActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WindowActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WindowActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<WindowActivatedEventArgs> for ICoreWindowEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WindowActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WindowActivatedEventArgs> for ICoreWindowEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WindowActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICoreWindowEventArgs> for WindowActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ICoreWindowEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICoreWindowEventArgs> for &WindowActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ICoreWindowEventArgs> {
        ::core::convert::TryInto::<ICoreWindowEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct WindowSizeChangedEventArgs(pub ::windows::core::IInspectable);
impl WindowSizeChangedEventArgs {
    #[cfg(feature = "Foundation")]
    pub fn Size(&self) -> ::windows::core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Size = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Size>(result__)
        }
    }
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICoreWindowEventArgs>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreWindowEventArgs>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for WindowSizeChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Core.WindowSizeChangedEventArgs;{5a200ec7-0426-47dc-b86c-6f475915e451})");
}
unsafe impl ::windows::core::Interface for WindowSizeChangedEventArgs {
    type Vtable = IWindowSizeChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5a200ec7_0426_47dc_b86c_6f475915e451);
}
impl ::windows::core::RuntimeName for WindowSizeChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Core.WindowSizeChangedEventArgs";
}
impl ::core::convert::From<WindowSizeChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WindowSizeChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&WindowSizeChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WindowSizeChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WindowSizeChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WindowSizeChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<WindowSizeChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WindowSizeChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&WindowSizeChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WindowSizeChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WindowSizeChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WindowSizeChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<WindowSizeChangedEventArgs> for ICoreWindowEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WindowSizeChangedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WindowSizeChangedEventArgs> for ICoreWindowEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WindowSizeChangedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICoreWindowEventArgs> for WindowSizeChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ICoreWindowEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICoreWindowEventArgs> for &WindowSizeChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ICoreWindowEventArgs> {
        ::core::convert::TryInto::<ICoreWindowEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
