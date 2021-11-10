#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `UI_Xaml_Input`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AccessKeyDisplayDismissedEventArgs(pub ::windows::runtime::IInspectable);
impl AccessKeyDisplayDismissedEventArgs {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<AccessKeyDisplayDismissedEventArgs, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AccessKeyDisplayDismissedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.AccessKeyDisplayDismissedEventArgs;{8a610dc6-d72d-4ca8-9f66-556f35b513da})");
}
unsafe impl ::windows::runtime::Interface for AccessKeyDisplayDismissedEventArgs {
    type Vtable = IAccessKeyDisplayDismissedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8a610dc6_d72d_4ca8_9f66_556f35b513da);
}
impl ::windows::runtime::RuntimeName for AccessKeyDisplayDismissedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.AccessKeyDisplayDismissedEventArgs";
}
impl ::core::convert::From<AccessKeyDisplayDismissedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: AccessKeyDisplayDismissedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AccessKeyDisplayDismissedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &AccessKeyDisplayDismissedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AccessKeyDisplayDismissedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AccessKeyDisplayDismissedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AccessKeyDisplayDismissedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: AccessKeyDisplayDismissedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AccessKeyDisplayDismissedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &AccessKeyDisplayDismissedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AccessKeyDisplayDismissedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AccessKeyDisplayDismissedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AccessKeyDisplayDismissedEventArgs {}
unsafe impl ::core::marker::Sync for AccessKeyDisplayDismissedEventArgs {}
#[doc = "*Required features: `UI_Xaml_Input`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AccessKeyDisplayRequestedEventArgs(pub ::windows::runtime::IInspectable);
impl AccessKeyDisplayRequestedEventArgs {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<AccessKeyDisplayRequestedEventArgs, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn PressedKeys(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AccessKeyDisplayRequestedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.AccessKeyDisplayRequestedEventArgs;{0c079e55-13fe-4d03-a61d-e12f06567286})");
}
unsafe impl ::windows::runtime::Interface for AccessKeyDisplayRequestedEventArgs {
    type Vtable = IAccessKeyDisplayRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x0c079e55_13fe_4d03_a61d_e12f06567286);
}
impl ::windows::runtime::RuntimeName for AccessKeyDisplayRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.AccessKeyDisplayRequestedEventArgs";
}
impl ::core::convert::From<AccessKeyDisplayRequestedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: AccessKeyDisplayRequestedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AccessKeyDisplayRequestedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &AccessKeyDisplayRequestedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AccessKeyDisplayRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AccessKeyDisplayRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AccessKeyDisplayRequestedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: AccessKeyDisplayRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AccessKeyDisplayRequestedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &AccessKeyDisplayRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AccessKeyDisplayRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AccessKeyDisplayRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AccessKeyDisplayRequestedEventArgs {}
unsafe impl ::core::marker::Sync for AccessKeyDisplayRequestedEventArgs {}
#[doc = "*Required features: `UI_Xaml_Input`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AccessKeyInvokedEventArgs(pub ::windows::runtime::IInspectable);
impl AccessKeyInvokedEventArgs {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<AccessKeyInvokedEventArgs, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn Handled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AccessKeyInvokedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.AccessKeyInvokedEventArgs;{cfe9cd97-c718-4091-b7dd-adf1c072b1e1})");
}
unsafe impl ::windows::runtime::Interface for AccessKeyInvokedEventArgs {
    type Vtable = IAccessKeyInvokedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xcfe9cd97_c718_4091_b7dd_adf1c072b1e1);
}
impl ::windows::runtime::RuntimeName for AccessKeyInvokedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.AccessKeyInvokedEventArgs";
}
impl ::core::convert::From<AccessKeyInvokedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: AccessKeyInvokedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AccessKeyInvokedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &AccessKeyInvokedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AccessKeyInvokedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AccessKeyInvokedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AccessKeyInvokedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: AccessKeyInvokedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AccessKeyInvokedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &AccessKeyInvokedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AccessKeyInvokedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AccessKeyInvokedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AccessKeyInvokedEventArgs {}
unsafe impl ::core::marker::Sync for AccessKeyInvokedEventArgs {}
#[doc = "*Required features: `UI_Xaml_Input`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AccessKeyManager(pub ::windows::runtime::IInspectable);
impl AccessKeyManager {
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn IsDisplayModeEnabled() -> ::windows::runtime::Result<bool> {
        Self::IAccessKeyManagerStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Input`, `Foundation`*"]
    pub fn IsDisplayModeEnabledChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<::windows::runtime::IInspectable, ::windows::runtime::IInspectable>>>(handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        Self::IAccessKeyManagerStatics(|this| unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Input`, `Foundation`*"]
    pub fn RemoveIsDisplayModeEnabledChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::runtime::Result<()> {
        Self::IAccessKeyManagerStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn ExitDisplayMode() -> ::windows::runtime::Result<()> {
        Self::IAccessKeyManagerStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this)).ok() })
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn AreKeyTipsEnabled() -> ::windows::runtime::Result<bool> {
        Self::IAccessKeyManagerStatics2(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn SetAreKeyTipsEnabled(value: bool) -> ::windows::runtime::Result<()> {
        Self::IAccessKeyManagerStatics2(|this| unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() })
    }
    pub fn IAccessKeyManagerStatics<R, F: FnOnce(&IAccessKeyManagerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<AccessKeyManager, IAccessKeyManagerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAccessKeyManagerStatics2<R, F: FnOnce(&IAccessKeyManagerStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<AccessKeyManager, IAccessKeyManagerStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AccessKeyManager {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.AccessKeyManager;{ecc973b0-2ee9-4b1c-98d7-6e0e816d334b})");
}
unsafe impl ::windows::runtime::Interface for AccessKeyManager {
    type Vtable = IAccessKeyManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xecc973b0_2ee9_4b1c_98d7_6e0e816d334b);
}
impl ::windows::runtime::RuntimeName for AccessKeyManager {
    const NAME: &'static str = "Windows.UI.Xaml.Input.AccessKeyManager";
}
impl ::core::convert::From<AccessKeyManager> for ::windows::runtime::IUnknown {
    fn from(value: AccessKeyManager) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AccessKeyManager> for ::windows::runtime::IUnknown {
    fn from(value: &AccessKeyManager) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AccessKeyManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AccessKeyManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AccessKeyManager> for ::windows::runtime::IInspectable {
    fn from(value: AccessKeyManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AccessKeyManager> for ::windows::runtime::IInspectable {
    fn from(value: &AccessKeyManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AccessKeyManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AccessKeyManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AccessKeyManager {}
unsafe impl ::core::marker::Sync for AccessKeyManager {}
#[doc = "*Required features: `UI_Xaml_Input`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CanExecuteRequestedEventArgs(pub ::windows::runtime::IInspectable);
impl CanExecuteRequestedEventArgs {
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn Parameter(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn CanExecute(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn SetCanExecute(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CanExecuteRequestedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.CanExecuteRequestedEventArgs;{c8e75256-1950-505d-993b-75907ef96830})");
}
unsafe impl ::windows::runtime::Interface for CanExecuteRequestedEventArgs {
    type Vtable = ICanExecuteRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc8e75256_1950_505d_993b_75907ef96830);
}
impl ::windows::runtime::RuntimeName for CanExecuteRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.CanExecuteRequestedEventArgs";
}
impl ::core::convert::From<CanExecuteRequestedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: CanExecuteRequestedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CanExecuteRequestedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &CanExecuteRequestedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CanExecuteRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a CanExecuteRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CanExecuteRequestedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: CanExecuteRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CanExecuteRequestedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &CanExecuteRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CanExecuteRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CanExecuteRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CanExecuteRequestedEventArgs {}
unsafe impl ::core::marker::Sync for CanExecuteRequestedEventArgs {}
#[doc = "*Required features: `UI_Xaml_Input`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CharacterReceivedRoutedEventArgs(pub ::windows::runtime::IInspectable);
impl CharacterReceivedRoutedEventArgs {
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn Character(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    #[doc = "*Required features: `UI_Xaml_Input`, `UI_Core`*"]
    pub fn KeyStatus(&self) -> ::windows::runtime::Result<super::super::Core::CorePhysicalKeyStatus> {
        let this = self;
        unsafe {
            let mut result__: super::super::Core::CorePhysicalKeyStatus = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Core::CorePhysicalKeyStatus>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn Handled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn OriginalSource(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IRoutedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CharacterReceivedRoutedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.CharacterReceivedRoutedEventArgs;{7849fd82-48e4-444d-9419-93ab8892c107})");
}
unsafe impl ::windows::runtime::Interface for CharacterReceivedRoutedEventArgs {
    type Vtable = ICharacterReceivedRoutedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x7849fd82_48e4_444d_9419_93ab8892c107);
}
impl ::windows::runtime::RuntimeName for CharacterReceivedRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.CharacterReceivedRoutedEventArgs";
}
impl ::core::convert::From<CharacterReceivedRoutedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: CharacterReceivedRoutedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CharacterReceivedRoutedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &CharacterReceivedRoutedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CharacterReceivedRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a CharacterReceivedRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CharacterReceivedRoutedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: CharacterReceivedRoutedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CharacterReceivedRoutedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &CharacterReceivedRoutedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CharacterReceivedRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CharacterReceivedRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<CharacterReceivedRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: CharacterReceivedRoutedEventArgs) -> Self {
        ::core::convert::Into::<super::RoutedEventArgs>::into(&value)
    }
}
impl ::core::convert::From<&CharacterReceivedRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: &CharacterReceivedRoutedEventArgs) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::RoutedEventArgs> for CharacterReceivedRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::RoutedEventArgs> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::RoutedEventArgs> for &CharacterReceivedRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::RoutedEventArgs> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for CharacterReceivedRoutedEventArgs {}
unsafe impl ::core::marker::Sync for CharacterReceivedRoutedEventArgs {}
#[doc = "*Required features: `UI_Xaml_Input`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ContextRequestedEventArgs(pub ::windows::runtime::IInspectable);
impl ContextRequestedEventArgs {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ContextRequestedEventArgs, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn Handled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Input`, `Foundation`*"]
    pub fn TryGetPosition<'a, Param0: ::windows::runtime::IntoParam<'a, super::UIElement>>(&self, relativeto: Param0, point: &mut super::super::super::Foundation::Point) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), relativeto.into_param().abi(), point, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn OriginalSource(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IRoutedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ContextRequestedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.ContextRequestedEventArgs;{42618e0a-1cb6-46fb-8374-0aec68aa5e51})");
}
unsafe impl ::windows::runtime::Interface for ContextRequestedEventArgs {
    type Vtable = IContextRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x42618e0a_1cb6_46fb_8374_0aec68aa5e51);
}
impl ::windows::runtime::RuntimeName for ContextRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.ContextRequestedEventArgs";
}
impl ::core::convert::From<ContextRequestedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: ContextRequestedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ContextRequestedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &ContextRequestedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ContextRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ContextRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ContextRequestedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: ContextRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ContextRequestedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &ContextRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ContextRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ContextRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ContextRequestedEventArgs> for super::RoutedEventArgs {
    fn from(value: ContextRequestedEventArgs) -> Self {
        ::core::convert::Into::<super::RoutedEventArgs>::into(&value)
    }
}
impl ::core::convert::From<&ContextRequestedEventArgs> for super::RoutedEventArgs {
    fn from(value: &ContextRequestedEventArgs) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::RoutedEventArgs> for ContextRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::RoutedEventArgs> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::RoutedEventArgs> for &ContextRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::RoutedEventArgs> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for ContextRequestedEventArgs {}
unsafe impl ::core::marker::Sync for ContextRequestedEventArgs {}
#[doc = "*Required features: `UI_Xaml_Input`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DoubleTappedEventHandler(::windows::runtime::IUnknown);
impl DoubleTappedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows::runtime::IInspectable>, &::core::option::Option<DoubleTappedRoutedEventArgs>) -> ::windows::runtime::Result<()> + 'static>(invoke: F) -> Self {
        let com = DoubleTappedEventHandler_box::<F> {
            vtable: &DoubleTappedEventHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::runtime::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn Invoke<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, DoubleTappedRoutedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).3)(::core::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DoubleTappedEventHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"delegate({3124d025-04a7-4d45-825e-8204a624dbf4})");
}
unsafe impl ::windows::runtime::Interface for DoubleTappedEventHandler {
    type Vtable = DoubleTappedEventHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x3124d025_04a7_4d45_825e_8204a624dbf4);
}
#[repr(C)]
#[doc(hidden)]
pub struct DoubleTappedEventHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr, e: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
struct DoubleTappedEventHandler_box<F: FnMut(&::core::option::Option<::windows::runtime::IInspectable>, &::core::option::Option<DoubleTappedRoutedEventArgs>) -> ::windows::runtime::Result<()> + 'static> {
    vtable: *const DoubleTappedEventHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows::runtime::IInspectable>, &::core::option::Option<DoubleTappedRoutedEventArgs>) -> ::windows::runtime::Result<()> + 'static> DoubleTappedEventHandler_box<F> {
    const VTABLE: DoubleTappedEventHandler_abi = DoubleTappedEventHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<DoubleTappedEventHandler as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::runtime::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr, e: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ((*this).invoke)(
            &*(&sender as *const <::windows::runtime::IInspectable as ::windows::runtime::Abi>::Abi as *const <::windows::runtime::IInspectable as ::windows::runtime::DefaultType>::DefaultType),
            &*(&e as *const <DoubleTappedRoutedEventArgs as ::windows::runtime::Abi>::Abi as *const <DoubleTappedRoutedEventArgs as ::windows::runtime::DefaultType>::DefaultType),
        )
        .into()
    }
}
#[doc = "*Required features: `UI_Xaml_Input`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DoubleTappedRoutedEventArgs(pub ::windows::runtime::IInspectable);
impl DoubleTappedRoutedEventArgs {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<DoubleTappedRoutedEventArgs, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Devices_Input")]
    #[doc = "*Required features: `UI_Xaml_Input`, `Devices_Input`*"]
    pub fn PointerDeviceType(&self) -> ::windows::runtime::Result<super::super::super::Devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Devices::Input::PointerDeviceType = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Devices::Input::PointerDeviceType>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn Handled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Input`, `Foundation`*"]
    pub fn GetPosition<'a, Param0: ::windows::runtime::IntoParam<'a, super::UIElement>>(&self, relativeto: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), relativeto.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn OriginalSource(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IRoutedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DoubleTappedRoutedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.DoubleTappedRoutedEventArgs;{af404424-26df-44f4-8714-9359249b62d3})");
}
unsafe impl ::windows::runtime::Interface for DoubleTappedRoutedEventArgs {
    type Vtable = IDoubleTappedRoutedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xaf404424_26df_44f4_8714_9359249b62d3);
}
impl ::windows::runtime::RuntimeName for DoubleTappedRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.DoubleTappedRoutedEventArgs";
}
impl ::core::convert::From<DoubleTappedRoutedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: DoubleTappedRoutedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DoubleTappedRoutedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &DoubleTappedRoutedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DoubleTappedRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a DoubleTappedRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DoubleTappedRoutedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: DoubleTappedRoutedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DoubleTappedRoutedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &DoubleTappedRoutedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DoubleTappedRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DoubleTappedRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<DoubleTappedRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: DoubleTappedRoutedEventArgs) -> Self {
        ::core::convert::Into::<super::RoutedEventArgs>::into(&value)
    }
}
impl ::core::convert::From<&DoubleTappedRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: &DoubleTappedRoutedEventArgs) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::RoutedEventArgs> for DoubleTappedRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::RoutedEventArgs> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::RoutedEventArgs> for &DoubleTappedRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::RoutedEventArgs> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for DoubleTappedRoutedEventArgs {}
unsafe impl ::core::marker::Sync for DoubleTappedRoutedEventArgs {}
#[doc = "*Required features: `UI_Xaml_Input`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ExecuteRequestedEventArgs(pub ::windows::runtime::IInspectable);
impl ExecuteRequestedEventArgs {
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn Parameter(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ExecuteRequestedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.ExecuteRequestedEventArgs;{e07fa734-a0b6-5755-9e87-24f54cca9372})");
}
unsafe impl ::windows::runtime::Interface for ExecuteRequestedEventArgs {
    type Vtable = IExecuteRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xe07fa734_a0b6_5755_9e87_24f54cca9372);
}
impl ::windows::runtime::RuntimeName for ExecuteRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.ExecuteRequestedEventArgs";
}
impl ::core::convert::From<ExecuteRequestedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: ExecuteRequestedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ExecuteRequestedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &ExecuteRequestedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ExecuteRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ExecuteRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ExecuteRequestedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: ExecuteRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ExecuteRequestedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &ExecuteRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ExecuteRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ExecuteRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ExecuteRequestedEventArgs {}
unsafe impl ::core::marker::Sync for ExecuteRequestedEventArgs {}
#[doc = "*Required features: `UI_Xaml_Input`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct FindNextElementOptions(pub ::windows::runtime::IInspectable);
impl FindNextElementOptions {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<FindNextElementOptions, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn SearchRoot(&self) -> ::windows::runtime::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn SetSearchRoot<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Input`, `Foundation`*"]
    pub fn ExclusionRect(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Input`, `Foundation`*"]
    pub fn SetExclusionRect<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Rect>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Input`, `Foundation`*"]
    pub fn HintRect(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Input`, `Foundation`*"]
    pub fn SetHintRect<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Rect>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn XYFocusNavigationStrategyOverride(&self) -> ::windows::runtime::Result<XYFocusNavigationStrategyOverride> {
        let this = self;
        unsafe {
            let mut result__: XYFocusNavigationStrategyOverride = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XYFocusNavigationStrategyOverride>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn SetXYFocusNavigationStrategyOverride(&self, value: XYFocusNavigationStrategyOverride) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for FindNextElementOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.FindNextElementOptions;{d88ae22b-46c2-41fc-897e-b5961977b89d})");
}
unsafe impl ::windows::runtime::Interface for FindNextElementOptions {
    type Vtable = IFindNextElementOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd88ae22b_46c2_41fc_897e_b5961977b89d);
}
impl ::windows::runtime::RuntimeName for FindNextElementOptions {
    const NAME: &'static str = "Windows.UI.Xaml.Input.FindNextElementOptions";
}
impl ::core::convert::From<FindNextElementOptions> for ::windows::runtime::IUnknown {
    fn from(value: FindNextElementOptions) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&FindNextElementOptions> for ::windows::runtime::IUnknown {
    fn from(value: &FindNextElementOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for FindNextElementOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a FindNextElementOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<FindNextElementOptions> for ::windows::runtime::IInspectable {
    fn from(value: FindNextElementOptions) -> Self {
        value.0
    }
}
impl ::core::convert::From<&FindNextElementOptions> for ::windows::runtime::IInspectable {
    fn from(value: &FindNextElementOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for FindNextElementOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a FindNextElementOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for FindNextElementOptions {}
unsafe impl ::core::marker::Sync for FindNextElementOptions {}
#[doc = "*Required features: `UI_Xaml_Input`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct FocusInputDeviceKind(pub i32);
impl FocusInputDeviceKind {
    pub const None: FocusInputDeviceKind = FocusInputDeviceKind(0i32);
    pub const Mouse: FocusInputDeviceKind = FocusInputDeviceKind(1i32);
    pub const Touch: FocusInputDeviceKind = FocusInputDeviceKind(2i32);
    pub const Pen: FocusInputDeviceKind = FocusInputDeviceKind(3i32);
    pub const Keyboard: FocusInputDeviceKind = FocusInputDeviceKind(4i32);
    pub const GameController: FocusInputDeviceKind = FocusInputDeviceKind(5i32);
}
impl ::core::convert::From<i32> for FocusInputDeviceKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FocusInputDeviceKind {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for FocusInputDeviceKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Input.FocusInputDeviceKind;i4)");
}
impl ::windows::runtime::DefaultType for FocusInputDeviceKind {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Xaml_Input`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct FocusManager(pub ::windows::runtime::IInspectable);
impl FocusManager {
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn GetFocusedElement() -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        Self::IFocusManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn TryMoveFocus(focusnavigationdirection: FocusNavigationDirection) -> ::windows::runtime::Result<bool> {
        Self::IFocusManagerStatics2(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), focusnavigationdirection, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn FindNextFocusableElement(focusnavigationdirection: FocusNavigationDirection) -> ::windows::runtime::Result<super::UIElement> {
        Self::IFocusManagerStatics3(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), focusnavigationdirection, &mut result__).from_abi::<super::UIElement>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Input`, `Foundation`*"]
    pub fn FindNextFocusableElementWithHint<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Rect>>(focusnavigationdirection: FocusNavigationDirection, hintrect: Param1) -> ::windows::runtime::Result<super::UIElement> {
        Self::IFocusManagerStatics3(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), focusnavigationdirection, hintrect.into_param().abi(), &mut result__).from_abi::<super::UIElement>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn TryMoveFocusWithOptions<'a, Param1: ::windows::runtime::IntoParam<'a, FindNextElementOptions>>(focusnavigationdirection: FocusNavigationDirection, focusnavigationoptions: Param1) -> ::windows::runtime::Result<bool> {
        Self::IFocusManagerStatics4(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), focusnavigationdirection, focusnavigationoptions.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn FindNextElement(focusnavigationdirection: FocusNavigationDirection) -> ::windows::runtime::Result<super::DependencyObject> {
        Self::IFocusManagerStatics4(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), focusnavigationdirection, &mut result__).from_abi::<super::DependencyObject>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn FindFirstFocusableElement<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>>(searchscope: Param0) -> ::windows::runtime::Result<super::DependencyObject> {
        Self::IFocusManagerStatics4(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), searchscope.into_param().abi(), &mut result__).from_abi::<super::DependencyObject>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn FindLastFocusableElement<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>>(searchscope: Param0) -> ::windows::runtime::Result<super::DependencyObject> {
        Self::IFocusManagerStatics4(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), searchscope.into_param().abi(), &mut result__).from_abi::<super::DependencyObject>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn FindNextElementWithOptions<'a, Param1: ::windows::runtime::IntoParam<'a, FindNextElementOptions>>(focusnavigationdirection: FocusNavigationDirection, focusnavigationoptions: Param1) -> ::windows::runtime::Result<super::DependencyObject> {
        Self::IFocusManagerStatics4(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), focusnavigationdirection, focusnavigationoptions.into_param().abi(), &mut result__).from_abi::<super::DependencyObject>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Input`, `Foundation`*"]
    pub fn TryFocusAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>>(element: Param0, value: super::FocusState) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<FocusMovementResult>> {
        Self::IFocusManagerStatics5(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), element.into_param().abi(), value, &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<FocusMovementResult>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Input`, `Foundation`*"]
    pub fn TryMoveFocusAsync(focusnavigationdirection: FocusNavigationDirection) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<FocusMovementResult>> {
        Self::IFocusManagerStatics5(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), focusnavigationdirection, &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<FocusMovementResult>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Input`, `Foundation`*"]
    pub fn TryMoveFocusWithOptionsAsync<'a, Param1: ::windows::runtime::IntoParam<'a, FindNextElementOptions>>(focusnavigationdirection: FocusNavigationDirection, focusnavigationoptions: Param1) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<FocusMovementResult>> {
        Self::IFocusManagerStatics5(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), focusnavigationdirection, focusnavigationoptions.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<FocusMovementResult>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Input`, `Foundation`*"]
    pub fn GotFocus<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventHandler<FocusManagerGotFocusEventArgs>>>(handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        Self::IFocusManagerStatics6(|this| unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Input`, `Foundation`*"]
    pub fn RemoveGotFocus<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::runtime::Result<()> {
        Self::IFocusManagerStatics6(|this| unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Input`, `Foundation`*"]
    pub fn LostFocus<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventHandler<FocusManagerLostFocusEventArgs>>>(handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        Self::IFocusManagerStatics6(|this| unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Input`, `Foundation`*"]
    pub fn RemoveLostFocus<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::runtime::Result<()> {
        Self::IFocusManagerStatics6(|this| unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Input`, `Foundation`*"]
    pub fn GettingFocus<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventHandler<GettingFocusEventArgs>>>(handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        Self::IFocusManagerStatics6(|this| unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Input`, `Foundation`*"]
    pub fn RemoveGettingFocus<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::runtime::Result<()> {
        Self::IFocusManagerStatics6(|this| unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Input`, `Foundation`*"]
    pub fn LosingFocus<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventHandler<LosingFocusEventArgs>>>(handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        Self::IFocusManagerStatics6(|this| unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Input`, `Foundation`*"]
    pub fn RemoveLosingFocus<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::runtime::Result<()> {
        Self::IFocusManagerStatics6(|this| unsafe { (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn GetFocusedElement2<'a, Param0: ::windows::runtime::IntoParam<'a, super::XamlRoot>>(xamlroot: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        Self::IFocusManagerStatics7(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), xamlroot.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        })
    }
    pub fn IFocusManagerStatics<R, F: FnOnce(&IFocusManagerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<FocusManager, IFocusManagerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IFocusManagerStatics2<R, F: FnOnce(&IFocusManagerStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<FocusManager, IFocusManagerStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IFocusManagerStatics3<R, F: FnOnce(&IFocusManagerStatics3) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<FocusManager, IFocusManagerStatics3> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IFocusManagerStatics4<R, F: FnOnce(&IFocusManagerStatics4) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<FocusManager, IFocusManagerStatics4> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IFocusManagerStatics5<R, F: FnOnce(&IFocusManagerStatics5) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<FocusManager, IFocusManagerStatics5> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IFocusManagerStatics6<R, F: FnOnce(&IFocusManagerStatics6) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<FocusManager, IFocusManagerStatics6> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IFocusManagerStatics7<R, F: FnOnce(&IFocusManagerStatics7) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<FocusManager, IFocusManagerStatics7> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for FocusManager {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.FocusManager;{c843f50b-3b83-4da1-9d6f-557c1169f341})");
}
unsafe impl ::windows::runtime::Interface for FocusManager {
    type Vtable = IFocusManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc843f50b_3b83_4da1_9d6f_557c1169f341);
}
impl ::windows::runtime::RuntimeName for FocusManager {
    const NAME: &'static str = "Windows.UI.Xaml.Input.FocusManager";
}
impl ::core::convert::From<FocusManager> for ::windows::runtime::IUnknown {
    fn from(value: FocusManager) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&FocusManager> for ::windows::runtime::IUnknown {
    fn from(value: &FocusManager) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for FocusManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a FocusManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<FocusManager> for ::windows::runtime::IInspectable {
    fn from(value: FocusManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&FocusManager> for ::windows::runtime::IInspectable {
    fn from(value: &FocusManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for FocusManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a FocusManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for FocusManager {}
unsafe impl ::core::marker::Sync for FocusManager {}
#[doc = "*Required features: `UI_Xaml_Input`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct FocusManagerGotFocusEventArgs(pub ::windows::runtime::IInspectable);
impl FocusManagerGotFocusEventArgs {
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn NewFocusedElement(&self) -> ::windows::runtime::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn CorrelationId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for FocusManagerGotFocusEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.FocusManagerGotFocusEventArgs;{97aa5d83-535b-507a-868e-62b706f06b61})");
}
unsafe impl ::windows::runtime::Interface for FocusManagerGotFocusEventArgs {
    type Vtable = IFocusManagerGotFocusEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x97aa5d83_535b_507a_868e_62b706f06b61);
}
impl ::windows::runtime::RuntimeName for FocusManagerGotFocusEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.FocusManagerGotFocusEventArgs";
}
impl ::core::convert::From<FocusManagerGotFocusEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: FocusManagerGotFocusEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&FocusManagerGotFocusEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &FocusManagerGotFocusEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for FocusManagerGotFocusEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a FocusManagerGotFocusEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<FocusManagerGotFocusEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: FocusManagerGotFocusEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&FocusManagerGotFocusEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &FocusManagerGotFocusEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for FocusManagerGotFocusEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a FocusManagerGotFocusEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for FocusManagerGotFocusEventArgs {}
unsafe impl ::core::marker::Sync for FocusManagerGotFocusEventArgs {}
#[doc = "*Required features: `UI_Xaml_Input`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct FocusManagerLostFocusEventArgs(pub ::windows::runtime::IInspectable);
impl FocusManagerLostFocusEventArgs {
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn OldFocusedElement(&self) -> ::windows::runtime::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn CorrelationId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for FocusManagerLostFocusEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.FocusManagerLostFocusEventArgs;{3e157e7a-9578-5cd3-aaa8-051b3d391978})");
}
unsafe impl ::windows::runtime::Interface for FocusManagerLostFocusEventArgs {
    type Vtable = IFocusManagerLostFocusEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x3e157e7a_9578_5cd3_aaa8_051b3d391978);
}
impl ::windows::runtime::RuntimeName for FocusManagerLostFocusEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.FocusManagerLostFocusEventArgs";
}
impl ::core::convert::From<FocusManagerLostFocusEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: FocusManagerLostFocusEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&FocusManagerLostFocusEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &FocusManagerLostFocusEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for FocusManagerLostFocusEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a FocusManagerLostFocusEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<FocusManagerLostFocusEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: FocusManagerLostFocusEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&FocusManagerLostFocusEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &FocusManagerLostFocusEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for FocusManagerLostFocusEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a FocusManagerLostFocusEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for FocusManagerLostFocusEventArgs {}
unsafe impl ::core::marker::Sync for FocusManagerLostFocusEventArgs {}
#[doc = "*Required features: `UI_Xaml_Input`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct FocusMovementResult(pub ::windows::runtime::IInspectable);
impl FocusMovementResult {
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn Succeeded(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for FocusMovementResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.FocusMovementResult;{06dfead3-c2ae-44bb-bfab-9c73de8407a4})");
}
unsafe impl ::windows::runtime::Interface for FocusMovementResult {
    type Vtable = IFocusMovementResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x06dfead3_c2ae_44bb_bfab_9c73de8407a4);
}
impl ::windows::runtime::RuntimeName for FocusMovementResult {
    const NAME: &'static str = "Windows.UI.Xaml.Input.FocusMovementResult";
}
impl ::core::convert::From<FocusMovementResult> for ::windows::runtime::IUnknown {
    fn from(value: FocusMovementResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&FocusMovementResult> for ::windows::runtime::IUnknown {
    fn from(value: &FocusMovementResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for FocusMovementResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a FocusMovementResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<FocusMovementResult> for ::windows::runtime::IInspectable {
    fn from(value: FocusMovementResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&FocusMovementResult> for ::windows::runtime::IInspectable {
    fn from(value: &FocusMovementResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for FocusMovementResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a FocusMovementResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for FocusMovementResult {}
unsafe impl ::core::marker::Sync for FocusMovementResult {}
#[doc = "*Required features: `UI_Xaml_Input`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct FocusNavigationDirection(pub i32);
impl FocusNavigationDirection {
    pub const Next: FocusNavigationDirection = FocusNavigationDirection(0i32);
    pub const Previous: FocusNavigationDirection = FocusNavigationDirection(1i32);
    pub const Up: FocusNavigationDirection = FocusNavigationDirection(2i32);
    pub const Down: FocusNavigationDirection = FocusNavigationDirection(3i32);
    pub const Left: FocusNavigationDirection = FocusNavigationDirection(4i32);
    pub const Right: FocusNavigationDirection = FocusNavigationDirection(5i32);
    pub const None: FocusNavigationDirection = FocusNavigationDirection(6i32);
}
impl ::core::convert::From<i32> for FocusNavigationDirection {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FocusNavigationDirection {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for FocusNavigationDirection {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Input.FocusNavigationDirection;i4)");
}
impl ::windows::runtime::DefaultType for FocusNavigationDirection {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Xaml_Input`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GettingFocusEventArgs(pub ::windows::runtime::IInspectable);
impl GettingFocusEventArgs {
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn OldFocusedElement(&self) -> ::windows::runtime::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn NewFocusedElement(&self) -> ::windows::runtime::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn SetNewFocusedElement<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn FocusState(&self) -> ::windows::runtime::Result<super::FocusState> {
        let this = self;
        unsafe {
            let mut result__: super::FocusState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::FocusState>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn Direction(&self) -> ::windows::runtime::Result<FocusNavigationDirection> {
        let this = self;
        unsafe {
            let mut result__: FocusNavigationDirection = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FocusNavigationDirection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn Handled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn InputDevice(&self) -> ::windows::runtime::Result<FocusInputDeviceKind> {
        let this = self;
        unsafe {
            let mut result__: FocusInputDeviceKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FocusInputDeviceKind>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn Cancel(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn SetCancel(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn TryCancel(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IGettingFocusEventArgs2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn TrySetNewFocusedElement<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>>(&self, element: Param0) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IGettingFocusEventArgs2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn CorrelationId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = &::windows::runtime::Interface::cast::<IGettingFocusEventArgs3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn OriginalSource(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IRoutedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for GettingFocusEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.GettingFocusEventArgs;{fa05b9ce-c67c-4be8-8fd4-c44d67877e0d})");
}
unsafe impl ::windows::runtime::Interface for GettingFocusEventArgs {
    type Vtable = IGettingFocusEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xfa05b9ce_c67c_4be8_8fd4_c44d67877e0d);
}
impl ::windows::runtime::RuntimeName for GettingFocusEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.GettingFocusEventArgs";
}
impl ::core::convert::From<GettingFocusEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: GettingFocusEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GettingFocusEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &GettingFocusEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for GettingFocusEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a GettingFocusEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GettingFocusEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: GettingFocusEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GettingFocusEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &GettingFocusEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for GettingFocusEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a GettingFocusEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<GettingFocusEventArgs> for super::RoutedEventArgs {
    fn from(value: GettingFocusEventArgs) -> Self {
        ::core::convert::Into::<super::RoutedEventArgs>::into(&value)
    }
}
impl ::core::convert::From<&GettingFocusEventArgs> for super::RoutedEventArgs {
    fn from(value: &GettingFocusEventArgs) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::RoutedEventArgs> for GettingFocusEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::RoutedEventArgs> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::RoutedEventArgs> for &GettingFocusEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::RoutedEventArgs> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for GettingFocusEventArgs {}
unsafe impl ::core::marker::Sync for GettingFocusEventArgs {}
#[doc = "*Required features: `UI_Xaml_Input`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct HoldingEventHandler(::windows::runtime::IUnknown);
impl HoldingEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows::runtime::IInspectable>, &::core::option::Option<HoldingRoutedEventArgs>) -> ::windows::runtime::Result<()> + 'static>(invoke: F) -> Self {
        let com = HoldingEventHandler_box::<F> {
            vtable: &HoldingEventHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::runtime::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn Invoke<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, HoldingRoutedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).3)(::core::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HoldingEventHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"delegate({ecae8ccd-8e5e-4fbe-9846-30a6370afcdf})");
}
unsafe impl ::windows::runtime::Interface for HoldingEventHandler {
    type Vtable = HoldingEventHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xecae8ccd_8e5e_4fbe_9846_30a6370afcdf);
}
#[repr(C)]
#[doc(hidden)]
pub struct HoldingEventHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr, e: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
struct HoldingEventHandler_box<F: FnMut(&::core::option::Option<::windows::runtime::IInspectable>, &::core::option::Option<HoldingRoutedEventArgs>) -> ::windows::runtime::Result<()> + 'static> {
    vtable: *const HoldingEventHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows::runtime::IInspectable>, &::core::option::Option<HoldingRoutedEventArgs>) -> ::windows::runtime::Result<()> + 'static> HoldingEventHandler_box<F> {
    const VTABLE: HoldingEventHandler_abi = HoldingEventHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<HoldingEventHandler as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::runtime::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr, e: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ((*this).invoke)(
            &*(&sender as *const <::windows::runtime::IInspectable as ::windows::runtime::Abi>::Abi as *const <::windows::runtime::IInspectable as ::windows::runtime::DefaultType>::DefaultType),
            &*(&e as *const <HoldingRoutedEventArgs as ::windows::runtime::Abi>::Abi as *const <HoldingRoutedEventArgs as ::windows::runtime::DefaultType>::DefaultType),
        )
        .into()
    }
}
#[doc = "*Required features: `UI_Xaml_Input`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct HoldingRoutedEventArgs(pub ::windows::runtime::IInspectable);
impl HoldingRoutedEventArgs {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<HoldingRoutedEventArgs, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Devices_Input")]
    #[doc = "*Required features: `UI_Xaml_Input`, `Devices_Input`*"]
    pub fn PointerDeviceType(&self) -> ::windows::runtime::Result<super::super::super::Devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Devices::Input::PointerDeviceType = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Devices::Input::PointerDeviceType>(result__)
        }
    }
    #[cfg(feature = "UI_Input")]
    #[doc = "*Required features: `UI_Xaml_Input`, `UI_Input`*"]
    pub fn HoldingState(&self) -> ::windows::runtime::Result<super::super::Input::HoldingState> {
        let this = self;
        unsafe {
            let mut result__: super::super::Input::HoldingState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Input::HoldingState>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn Handled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Input`, `Foundation`*"]
    pub fn GetPosition<'a, Param0: ::windows::runtime::IntoParam<'a, super::UIElement>>(&self, relativeto: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), relativeto.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn OriginalSource(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IRoutedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HoldingRoutedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.HoldingRoutedEventArgs;{c246ff23-d80d-44de-8db9-0d815e269ac0})");
}
unsafe impl ::windows::runtime::Interface for HoldingRoutedEventArgs {
    type Vtable = IHoldingRoutedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc246ff23_d80d_44de_8db9_0d815e269ac0);
}
impl ::windows::runtime::RuntimeName for HoldingRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.HoldingRoutedEventArgs";
}
impl ::core::convert::From<HoldingRoutedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: HoldingRoutedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&HoldingRoutedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &HoldingRoutedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HoldingRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a HoldingRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<HoldingRoutedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: HoldingRoutedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&HoldingRoutedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &HoldingRoutedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HoldingRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HoldingRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<HoldingRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: HoldingRoutedEventArgs) -> Self {
        ::core::convert::Into::<super::RoutedEventArgs>::into(&value)
    }
}
impl ::core::convert::From<&HoldingRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: &HoldingRoutedEventArgs) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::RoutedEventArgs> for HoldingRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::RoutedEventArgs> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::RoutedEventArgs> for &HoldingRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::RoutedEventArgs> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for HoldingRoutedEventArgs {}
unsafe impl ::core::marker::Sync for HoldingRoutedEventArgs {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IAccessKeyDisplayDismissedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAccessKeyDisplayDismissedEventArgs {
    type Vtable = IAccessKeyDisplayDismissedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8a610dc6_d72d_4ca8_9f66_556f35b513da);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessKeyDisplayDismissedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAccessKeyDisplayRequestedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAccessKeyDisplayRequestedEventArgs {
    type Vtable = IAccessKeyDisplayRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x0c079e55_13fe_4d03_a61d_e12f06567286);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessKeyDisplayRequestedEventArgs_abi(
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
pub struct IAccessKeyInvokedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAccessKeyInvokedEventArgs {
    type Vtable = IAccessKeyInvokedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xcfe9cd97_c718_4091_b7dd_adf1c072b1e1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessKeyInvokedEventArgs_abi(
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
pub struct IAccessKeyManager(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAccessKeyManager {
    type Vtable = IAccessKeyManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xecc973b0_2ee9_4b1c_98d7_6e0e816d334b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessKeyManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAccessKeyManagerStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAccessKeyManagerStatics {
    type Vtable = IAccessKeyManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x4ca0efe6_d9c8_4ebc_b4c7_30d1838a81f1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessKeyManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAccessKeyManagerStatics2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAccessKeyManagerStatics2 {
    type Vtable = IAccessKeyManagerStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x962bb594_2ab3_47c5_954b_7092f355f797);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessKeyManagerStatics2_abi(
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
pub struct ICanExecuteRequestedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICanExecuteRequestedEventArgs {
    type Vtable = ICanExecuteRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc8e75256_1950_505d_993b_75907ef96830);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanExecuteRequestedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICharacterReceivedRoutedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICharacterReceivedRoutedEventArgs {
    type Vtable = ICharacterReceivedRoutedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x7849fd82_48e4_444d_9419_93ab8892c107);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICharacterReceivedRoutedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI_Core")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Core::CorePhysicalKeyStatus) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Core"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `UI_Xaml_Input`*"]
pub struct ICommand(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICommand {
    type Vtable = ICommand_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xe5af3542_ca67_4081_995b_709dd13792df);
}
impl ICommand {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Input`, `Foundation`*"]
    pub fn CanExecuteChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventHandler<::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Input`, `Foundation`*"]
    pub fn RemoveCanExecuteChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn CanExecute<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, parameter: Param0) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), parameter.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn Execute<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, parameter: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), parameter.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ICommand {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{e5af3542-ca67-4081-995b-709dd13792df}");
}
impl ::core::convert::From<ICommand> for ::windows::runtime::IUnknown {
    fn from(value: ICommand) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ICommand> for ::windows::runtime::IUnknown {
    fn from(value: &ICommand) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ICommand {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ICommand {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ICommand> for ::windows::runtime::IInspectable {
    fn from(value: ICommand) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICommand> for ::windows::runtime::IInspectable {
    fn from(value: &ICommand) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ICommand {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ICommand {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommand_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parameter: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parameter: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContextRequestedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContextRequestedEventArgs {
    type Vtable = IContextRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x42618e0a_1cb6_46fb_8374_0aec68aa5e51);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContextRequestedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, relativeto: ::windows::runtime::RawPtr, point: *mut super::super::super::Foundation::Point, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDoubleTappedRoutedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDoubleTappedRoutedEventArgs {
    type Vtable = IDoubleTappedRoutedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xaf404424_26df_44f4_8714_9359249b62d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDoubleTappedRoutedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Input")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Devices::Input::PointerDeviceType) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Input"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, relativeto: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::Point) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IExecuteRequestedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IExecuteRequestedEventArgs {
    type Vtable = IExecuteRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xe07fa734_a0b6_5755_9e87_24f54cca9372);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExecuteRequestedEventArgs_abi(
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
pub struct IFindNextElementOptions(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFindNextElementOptions {
    type Vtable = IFindNextElementOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd88ae22b_46c2_41fc_897e_b5961977b89d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFindNextElementOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::Rect) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::super::Foundation::Rect) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::Rect) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::super::Foundation::Rect) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut XYFocusNavigationStrategyOverride) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: XYFocusNavigationStrategyOverride) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFocusManager(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFocusManager {
    type Vtable = IFocusManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc843f50b_3b83_4da1_9d6f_557c1169f341);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFocusManagerGotFocusEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFocusManagerGotFocusEventArgs {
    type Vtable = IFocusManagerGotFocusEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x97aa5d83_535b_507a_868e_62b706f06b61);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusManagerGotFocusEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFocusManagerLostFocusEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFocusManagerLostFocusEventArgs {
    type Vtable = IFocusManagerLostFocusEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x3e157e7a_9578_5cd3_aaa8_051b3d391978);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusManagerLostFocusEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFocusManagerStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFocusManagerStatics {
    type Vtable = IFocusManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x1eccd326_8182_4482_826a_0918e9ed9af7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusManagerStatics_abi(
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
pub struct IFocusManagerStatics2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFocusManagerStatics2 {
    type Vtable = IFocusManagerStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa920d761_dd87_4f31_beda_ef417fe7c04a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusManagerStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, focusnavigationdirection: FocusNavigationDirection, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFocusManagerStatics3(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFocusManagerStatics3 {
    type Vtable = IFocusManagerStatics3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x60805ebf_b149_417d_83f1_baeb560e2a47);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusManagerStatics3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, focusnavigationdirection: FocusNavigationDirection, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, focusnavigationdirection: FocusNavigationDirection, hintrect: super::super::super::Foundation::Rect, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFocusManagerStatics4(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFocusManagerStatics4 {
    type Vtable = IFocusManagerStatics4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x29276e9c_1c6c_414a_ba1c_96efd5962bcd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusManagerStatics4_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, focusnavigationdirection: FocusNavigationDirection, focusnavigationoptions: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, focusnavigationdirection: FocusNavigationDirection, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, searchscope: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, searchscope: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, focusnavigationdirection: FocusNavigationDirection, focusnavigationoptions: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFocusManagerStatics5(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFocusManagerStatics5 {
    type Vtable = IFocusManagerStatics5_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x280edc61_207a_4d7b_b98f_ce165e1b2015);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusManagerStatics5_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, element: ::windows::runtime::RawPtr, value: super::FocusState, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, focusnavigationdirection: FocusNavigationDirection, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, focusnavigationdirection: FocusNavigationDirection, focusnavigationoptions: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFocusManagerStatics6(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFocusManagerStatics6 {
    type Vtable = IFocusManagerStatics6_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x3546a1b6_20bf_5007_929d_e6d32e16afe4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusManagerStatics6_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
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
pub struct IFocusManagerStatics7(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFocusManagerStatics7 {
    type Vtable = IFocusManagerStatics7_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x95d6fa97_f0fc_5c32_b29d_07c04ec966b0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusManagerStatics7_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xamlroot: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFocusMovementResult(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFocusMovementResult {
    type Vtable = IFocusMovementResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x06dfead3_c2ae_44bb_bfab_9c73de8407a4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusMovementResult_abi(
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
pub struct IGettingFocusEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGettingFocusEventArgs {
    type Vtable = IGettingFocusEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xfa05b9ce_c67c_4be8_8fd4_c44d67877e0d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGettingFocusEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::FocusState) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut FocusNavigationDirection) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut FocusInputDeviceKind) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGettingFocusEventArgs2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGettingFocusEventArgs2 {
    type Vtable = IGettingFocusEventArgs2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x88754d7b_b4b9_4959_8bce_89bf212ed4eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGettingFocusEventArgs2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, element: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGettingFocusEventArgs3(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGettingFocusEventArgs3 {
    type Vtable = IGettingFocusEventArgs3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x4e024891_db3f_5e78_b75a_62bfc3510735);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGettingFocusEventArgs3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHoldingRoutedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHoldingRoutedEventArgs {
    type Vtable = IHoldingRoutedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc246ff23_d80d_44de_8db9_0d815e269ac0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHoldingRoutedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Input")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Devices::Input::PointerDeviceType) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Input"))] usize,
    #[cfg(feature = "UI_Input")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Input::HoldingState) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Input"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, relativeto: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::Point) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInertiaExpansionBehavior(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInertiaExpansionBehavior {
    type Vtable = IInertiaExpansionBehavior_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x751d87e5_8d42_44c5_965e_3cd30cc9d6f7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInertiaExpansionBehavior_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInertiaRotationBehavior(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInertiaRotationBehavior {
    type Vtable = IInertiaRotationBehavior_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x424cfb2e_bbfd_4625_ae78_20c65bf1efaf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInertiaRotationBehavior_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInertiaTranslationBehavior(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInertiaTranslationBehavior {
    type Vtable = IInertiaTranslationBehavior_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x45d3a512_3b32_4882_a4c2_ecfa2d4b6df0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInertiaTranslationBehavior_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInputScope(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInputScope {
    type Vtable = IInputScope_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x5c0f85f3_f9d8_4220_b666_045d074d9bfa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputScope_abi(
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
pub struct IInputScopeName(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInputScopeName {
    type Vtable = IInputScopeName_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xfd3e6997_08fb_4cba_a021_792d7589fd5a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputScopeName_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut InputScopeNameValue) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: InputScopeNameValue) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInputScopeNameFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInputScopeNameFactory {
    type Vtable = IInputScopeNameFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x4a40bb52_4bd7_4e54_8617_1cda8a1eda7f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputScopeNameFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, namevalue: InputScopeNameValue, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IKeyRoutedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IKeyRoutedEventArgs {
    type Vtable = IKeyRoutedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd4cd3dfe_4079_42e9_a39a_3095d3f049c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyRoutedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::System::VirtualKey) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "UI_Core")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Core::CorePhysicalKeyStatus) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Core"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IKeyRoutedEventArgs2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IKeyRoutedEventArgs2 {
    type Vtable = IKeyRoutedEventArgs2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x1b02d57a_9634_4f14_91b2_133e42fdb3cd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyRoutedEventArgs2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::System::VirtualKey) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IKeyRoutedEventArgs3(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IKeyRoutedEventArgs3 {
    type Vtable = IKeyRoutedEventArgs3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x2779f5b4_ca41_411b_a8ef_f4fc78e78057);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyRoutedEventArgs3_abi(
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
pub struct IKeyboardAccelerator(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IKeyboardAccelerator {
    type Vtable = IKeyboardAccelerator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x92e6181e_19ae_465a_9b3c_a71ee9ea7420);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyboardAccelerator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::System::VirtualKey) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::super::System::VirtualKey) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::System::VirtualKeyModifiers) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::super::System::VirtualKeyModifiers) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IKeyboardAcceleratorFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IKeyboardAcceleratorFactory {
    type Vtable = IKeyboardAcceleratorFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x44d88a99_4bfd_4a47_a893_515f388623f6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyboardAcceleratorFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, baseinterface: ::windows::runtime::RawPtr, innerinterface: *mut ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IKeyboardAcceleratorInvokedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IKeyboardAcceleratorInvokedEventArgs {
    type Vtable = IKeyboardAcceleratorInvokedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc00b03f2_04e7_4415_b17d_d76b9490de2b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyboardAcceleratorInvokedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IKeyboardAcceleratorInvokedEventArgs2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IKeyboardAcceleratorInvokedEventArgs2 {
    type Vtable = IKeyboardAcceleratorInvokedEventArgs2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xbefca4b8_5907_48ee_8e21_9c969078fa11);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyboardAcceleratorInvokedEventArgs2_abi(
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
pub struct IKeyboardAcceleratorStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IKeyboardAcceleratorStatics {
    type Vtable = IKeyboardAcceleratorStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x3bd43d51_9bb3_456d_bf15_804adfb86261);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyboardAcceleratorStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILosingFocusEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILosingFocusEventArgs {
    type Vtable = ILosingFocusEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf9f683c7_d789_472b_aa93_6d4105e6dabe);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILosingFocusEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::FocusState) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut FocusNavigationDirection) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut FocusInputDeviceKind) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILosingFocusEventArgs2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILosingFocusEventArgs2 {
    type Vtable = ILosingFocusEventArgs2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x0493fad9_c27f_469f_8e62_52b3a4f7cd54);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILosingFocusEventArgs2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, element: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILosingFocusEventArgs3(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILosingFocusEventArgs3 {
    type Vtable = ILosingFocusEventArgs3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc98900bd_0b79_566e_ad1f_436fa513ae22);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILosingFocusEventArgs3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IManipulationCompletedRoutedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IManipulationCompletedRoutedEventArgs {
    type Vtable = IManipulationCompletedRoutedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xb5ad9b23_2f41_498e_8319_015ee8a75346);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationCompletedRoutedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::Point) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "UI_Input"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Input::ManipulationDelta) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Input")))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Input"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Input::ManipulationVelocities) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Input")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Input")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Devices::Input::PointerDeviceType) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Input"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IManipulationDeltaRoutedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IManipulationDeltaRoutedEventArgs {
    type Vtable = IManipulationDeltaRoutedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x400d5794_4c6f_491d_82d6_3517109399c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationDeltaRoutedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::Point) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "UI_Input"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Input::ManipulationDelta) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Input")))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Input"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Input::ManipulationDelta) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Input")))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Input"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Input::ManipulationVelocities) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Input")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Input")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Devices::Input::PointerDeviceType) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Input"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IManipulationInertiaStartingRoutedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IManipulationInertiaStartingRoutedEventArgs {
    type Vtable = IManipulationInertiaStartingRoutedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x246a91a9_ca43_4c0b_acef_81e8b8147520);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationInertiaStartingRoutedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Input")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Devices::Input::PointerDeviceType) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Input"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Input"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Input::ManipulationDelta) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Input")))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Input"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Input::ManipulationDelta) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Input")))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Input"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Input::ManipulationVelocities) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Input")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IManipulationPivot(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IManipulationPivot {
    type Vtable = IManipulationPivot_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x2e3838a5_e6c2_4998_82ac_18748b141666);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationPivot_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::Point) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::super::Foundation::Point) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IManipulationPivotFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IManipulationPivotFactory {
    type Vtable = IManipulationPivotFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x6d05b039_3702_4396_ad9b_a825efa63a3b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationPivotFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, center: super::super::super::Foundation::Point, radius: f64, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IManipulationStartedRoutedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IManipulationStartedRoutedEventArgs {
    type Vtable = IManipulationStartedRoutedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x5db1aa05_9f80_48b6_ae6c_4f119de8ff13);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationStartedRoutedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::Point) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Input")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Devices::Input::PointerDeviceType) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Input"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Input"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Input::ManipulationDelta) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Input")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IManipulationStartedRoutedEventArgsFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IManipulationStartedRoutedEventArgsFactory {
    type Vtable = IManipulationStartedRoutedEventArgsFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x84c1daa7_7272_4463_b6c3_a40b9ba151fc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationStartedRoutedEventArgsFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, baseinterface: ::windows::runtime::RawPtr, innerinterface: *mut ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IManipulationStartingRoutedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IManipulationStartingRoutedEventArgs {
    type Vtable = IManipulationStartingRoutedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x18d636b7_53a4_4c15_a498_f3a9ca212a42);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationStartingRoutedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ManipulationModes) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ManipulationModes) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct INoFocusCandidateFoundEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INoFocusCandidateFoundEventArgs {
    type Vtable = INoFocusCandidateFoundEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xec3601a7_1007_48f9_b6b3_ed0bea53937d);
}
#[repr(C)]
#[doc(hidden)]
pub struct INoFocusCandidateFoundEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut FocusNavigationDirection) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut FocusInputDeviceKind) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPointer(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPointer {
    type Vtable = IPointer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x5ee8f39f_747d_4171_90e6_cd37a9dffb11);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Input")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Devices::Input::PointerDeviceType) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Input"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPointerRoutedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPointerRoutedEventArgs {
    type Vtable = IPointerRoutedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xda628f0a_9752_49e2_bde2_49eccab9194d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerRoutedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::System::VirtualKeyModifiers) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI_Input")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, relativeto: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Input"))] usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Input"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, relativeto: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Input")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPointerRoutedEventArgs2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPointerRoutedEventArgs2 {
    type Vtable = IPointerRoutedEventArgs2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x0821f294_1de6_4711_ba7c_8d4b8b0911d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerRoutedEventArgs2_abi(
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
pub struct IProcessKeyboardAcceleratorEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IProcessKeyboardAcceleratorEventArgs {
    type Vtable = IProcessKeyboardAcceleratorEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xfb79c216_972b_440c_9b83_2b4198dcf09d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessKeyboardAcceleratorEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::System::VirtualKey) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::System::VirtualKeyModifiers) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRightTappedRoutedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRightTappedRoutedEventArgs {
    type Vtable = IRightTappedRoutedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x6834869d_7bd5_4033_b237_172f79abe393);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRightTappedRoutedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Input")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Devices::Input::PointerDeviceType) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Input"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, relativeto: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::Point) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IStandardUICommand(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IStandardUICommand {
    type Vtable = IStandardUICommand_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd2bf7f43_0504_52d0_8aa6_0cb0f756eb27);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStandardUICommand_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut StandardUICommandKind) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IStandardUICommand2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IStandardUICommand2 {
    type Vtable = IStandardUICommand2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xe3666069_f9e4_51eb_885b_7a620a0782ea);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStandardUICommand2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: StandardUICommandKind) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IStandardUICommandFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IStandardUICommandFactory {
    type Vtable = IStandardUICommandFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8f1a7590_dce1_56e4_ab63_f5ce3ce4ebf6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStandardUICommandFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, baseinterface: ::windows::runtime::RawPtr, innerinterface: *mut ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, kind: StandardUICommandKind, baseinterface: ::windows::runtime::RawPtr, innerinterface: *mut ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IStandardUICommandStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IStandardUICommandStatics {
    type Vtable = IStandardUICommandStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x7ea87ed9_2978_5533_9b2e_6759ce88569f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStandardUICommandStatics_abi(
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
pub struct ITappedRoutedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITappedRoutedEventArgs {
    type Vtable = ITappedRoutedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa099e6be_e624_459a_bb1d_e05c73e2cc66);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITappedRoutedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Input")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Devices::Input::PointerDeviceType) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Input"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, relativeto: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::Point) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXamlUICommand(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXamlUICommand {
    type Vtable = IXamlUICommand_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8494f8d4_ead1_5f01_ad2e_a8cad4f9dc0e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlUICommand_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI_Xaml_Controls")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))] usize,
    #[cfg(feature = "UI_Xaml_Controls")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXamlUICommandFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXamlUICommandFactory {
    type Vtable = IXamlUICommandFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x1eec08c3_e061_5e10_9f2a_2baa840885c2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlUICommandFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, baseinterface: ::windows::runtime::RawPtr, innerinterface: *mut ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXamlUICommandStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXamlUICommandStatics {
    type Vtable = IXamlUICommandStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x66bc457c_1a0c_58ed_876e_71533f966db6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlUICommandStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `UI_Xaml_Input`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct InertiaExpansionBehavior(pub ::windows::runtime::IInspectable);
impl InertiaExpansionBehavior {
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn DesiredDeceleration(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn SetDesiredDeceleration(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn DesiredExpansion(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn SetDesiredExpansion(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InertiaExpansionBehavior {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.InertiaExpansionBehavior;{751d87e5-8d42-44c5-965e-3cd30cc9d6f7})");
}
unsafe impl ::windows::runtime::Interface for InertiaExpansionBehavior {
    type Vtable = IInertiaExpansionBehavior_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x751d87e5_8d42_44c5_965e_3cd30cc9d6f7);
}
impl ::windows::runtime::RuntimeName for InertiaExpansionBehavior {
    const NAME: &'static str = "Windows.UI.Xaml.Input.InertiaExpansionBehavior";
}
impl ::core::convert::From<InertiaExpansionBehavior> for ::windows::runtime::IUnknown {
    fn from(value: InertiaExpansionBehavior) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InertiaExpansionBehavior> for ::windows::runtime::IUnknown {
    fn from(value: &InertiaExpansionBehavior) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InertiaExpansionBehavior {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a InertiaExpansionBehavior {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InertiaExpansionBehavior> for ::windows::runtime::IInspectable {
    fn from(value: InertiaExpansionBehavior) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InertiaExpansionBehavior> for ::windows::runtime::IInspectable {
    fn from(value: &InertiaExpansionBehavior) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for InertiaExpansionBehavior {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a InertiaExpansionBehavior {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for InertiaExpansionBehavior {}
unsafe impl ::core::marker::Sync for InertiaExpansionBehavior {}
#[doc = "*Required features: `UI_Xaml_Input`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct InertiaRotationBehavior(pub ::windows::runtime::IInspectable);
impl InertiaRotationBehavior {
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn DesiredDeceleration(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn SetDesiredDeceleration(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn DesiredRotation(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn SetDesiredRotation(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InertiaRotationBehavior {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.InertiaRotationBehavior;{424cfb2e-bbfd-4625-ae78-20c65bf1efaf})");
}
unsafe impl ::windows::runtime::Interface for InertiaRotationBehavior {
    type Vtable = IInertiaRotationBehavior_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x424cfb2e_bbfd_4625_ae78_20c65bf1efaf);
}
impl ::windows::runtime::RuntimeName for InertiaRotationBehavior {
    const NAME: &'static str = "Windows.UI.Xaml.Input.InertiaRotationBehavior";
}
impl ::core::convert::From<InertiaRotationBehavior> for ::windows::runtime::IUnknown {
    fn from(value: InertiaRotationBehavior) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InertiaRotationBehavior> for ::windows::runtime::IUnknown {
    fn from(value: &InertiaRotationBehavior) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InertiaRotationBehavior {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a InertiaRotationBehavior {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InertiaRotationBehavior> for ::windows::runtime::IInspectable {
    fn from(value: InertiaRotationBehavior) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InertiaRotationBehavior> for ::windows::runtime::IInspectable {
    fn from(value: &InertiaRotationBehavior) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for InertiaRotationBehavior {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a InertiaRotationBehavior {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for InertiaRotationBehavior {}
unsafe impl ::core::marker::Sync for InertiaRotationBehavior {}
#[doc = "*Required features: `UI_Xaml_Input`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct InertiaTranslationBehavior(pub ::windows::runtime::IInspectable);
impl InertiaTranslationBehavior {
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn DesiredDeceleration(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn SetDesiredDeceleration(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn DesiredDisplacement(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn SetDesiredDisplacement(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InertiaTranslationBehavior {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.InertiaTranslationBehavior;{45d3a512-3b32-4882-a4c2-ecfa2d4b6df0})");
}
unsafe impl ::windows::runtime::Interface for InertiaTranslationBehavior {
    type Vtable = IInertiaTranslationBehavior_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x45d3a512_3b32_4882_a4c2_ecfa2d4b6df0);
}
impl ::windows::runtime::RuntimeName for InertiaTranslationBehavior {
    const NAME: &'static str = "Windows.UI.Xaml.Input.InertiaTranslationBehavior";
}
impl ::core::convert::From<InertiaTranslationBehavior> for ::windows::runtime::IUnknown {
    fn from(value: InertiaTranslationBehavior) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InertiaTranslationBehavior> for ::windows::runtime::IUnknown {
    fn from(value: &InertiaTranslationBehavior) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InertiaTranslationBehavior {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a InertiaTranslationBehavior {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InertiaTranslationBehavior> for ::windows::runtime::IInspectable {
    fn from(value: InertiaTranslationBehavior) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InertiaTranslationBehavior> for ::windows::runtime::IInspectable {
    fn from(value: &InertiaTranslationBehavior) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for InertiaTranslationBehavior {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a InertiaTranslationBehavior {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for InertiaTranslationBehavior {}
unsafe impl ::core::marker::Sync for InertiaTranslationBehavior {}
#[doc = "*Required features: `UI_Xaml_Input`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct InputScope(pub ::windows::runtime::IInspectable);
impl InputScope {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<InputScope, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Xaml_Input`, `Foundation_Collections`*"]
    pub fn Names(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVector<InputScopeName>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<InputScopeName>>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn GetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn SetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, dp: Param0, value: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), dp.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn ClearValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), dp.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn ReadLocalValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn GetAnimationBaseValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    #[doc = "*Required features: `UI_Xaml_Input`, `UI_Core`*"]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn RegisterPropertyChangedCallback<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>, Param1: ::windows::runtime::IntoParam<'a, super::DependencyPropertyChangedCallback>>(&self, dp: Param0, callback: Param1) -> ::windows::runtime::Result<i64> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject2>(self)?;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), dp.into_param().abi(), callback.into_param().abi(), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn UnregisterPropertyChangedCallback<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(&self, dp: Param0, token: i64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), dp.into_param().abi(), token).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InputScope {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.InputScope;{5c0f85f3-f9d8-4220-b666-045d074d9bfa})");
}
unsafe impl ::windows::runtime::Interface for InputScope {
    type Vtable = IInputScope_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x5c0f85f3_f9d8_4220_b666_045d074d9bfa);
}
impl ::windows::runtime::RuntimeName for InputScope {
    const NAME: &'static str = "Windows.UI.Xaml.Input.InputScope";
}
impl ::core::convert::From<InputScope> for ::windows::runtime::IUnknown {
    fn from(value: InputScope) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InputScope> for ::windows::runtime::IUnknown {
    fn from(value: &InputScope) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InputScope {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a InputScope {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InputScope> for ::windows::runtime::IInspectable {
    fn from(value: InputScope) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InputScope> for ::windows::runtime::IInspectable {
    fn from(value: &InputScope) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for InputScope {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a InputScope {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<InputScope> for super::DependencyObject {
    fn from(value: InputScope) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&InputScope> for super::DependencyObject {
    fn from(value: &InputScope) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for InputScope {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for &InputScope {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for InputScope {}
unsafe impl ::core::marker::Sync for InputScope {}
#[doc = "*Required features: `UI_Xaml_Input`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct InputScopeName(pub ::windows::runtime::IInspectable);
impl InputScopeName {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<InputScopeName, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn NameValue(&self) -> ::windows::runtime::Result<InputScopeNameValue> {
        let this = self;
        unsafe {
            let mut result__: InputScopeNameValue = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InputScopeNameValue>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn SetNameValue(&self, value: InputScopeNameValue) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn CreateInstance(namevalue: InputScopeNameValue) -> ::windows::runtime::Result<InputScopeName> {
        Self::IInputScopeNameFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), namevalue, &mut result__).from_abi::<InputScopeName>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn GetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn SetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, dp: Param0, value: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), dp.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn ClearValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), dp.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn ReadLocalValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn GetAnimationBaseValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    #[doc = "*Required features: `UI_Xaml_Input`, `UI_Core`*"]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn RegisterPropertyChangedCallback<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>, Param1: ::windows::runtime::IntoParam<'a, super::DependencyPropertyChangedCallback>>(&self, dp: Param0, callback: Param1) -> ::windows::runtime::Result<i64> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject2>(self)?;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), dp.into_param().abi(), callback.into_param().abi(), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn UnregisterPropertyChangedCallback<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(&self, dp: Param0, token: i64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), dp.into_param().abi(), token).ok() }
    }
    pub fn IInputScopeNameFactory<R, F: FnOnce(&IInputScopeNameFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<InputScopeName, IInputScopeNameFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InputScopeName {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.InputScopeName;{fd3e6997-08fb-4cba-a021-792d7589fd5a})");
}
unsafe impl ::windows::runtime::Interface for InputScopeName {
    type Vtable = IInputScopeName_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xfd3e6997_08fb_4cba_a021_792d7589fd5a);
}
impl ::windows::runtime::RuntimeName for InputScopeName {
    const NAME: &'static str = "Windows.UI.Xaml.Input.InputScopeName";
}
impl ::core::convert::From<InputScopeName> for ::windows::runtime::IUnknown {
    fn from(value: InputScopeName) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InputScopeName> for ::windows::runtime::IUnknown {
    fn from(value: &InputScopeName) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InputScopeName {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a InputScopeName {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InputScopeName> for ::windows::runtime::IInspectable {
    fn from(value: InputScopeName) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InputScopeName> for ::windows::runtime::IInspectable {
    fn from(value: &InputScopeName) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for InputScopeName {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a InputScopeName {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<InputScopeName> for super::DependencyObject {
    fn from(value: InputScopeName) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&InputScopeName> for super::DependencyObject {
    fn from(value: &InputScopeName) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for InputScopeName {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for &InputScopeName {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for InputScopeName {}
unsafe impl ::core::marker::Sync for InputScopeName {}
#[doc = "*Required features: `UI_Xaml_Input`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct InputScopeNameValue(pub i32);
impl InputScopeNameValue {
    pub const Default: InputScopeNameValue = InputScopeNameValue(0i32);
    pub const Url: InputScopeNameValue = InputScopeNameValue(1i32);
    pub const EmailSmtpAddress: InputScopeNameValue = InputScopeNameValue(5i32);
    pub const PersonalFullName: InputScopeNameValue = InputScopeNameValue(7i32);
    pub const CurrencyAmountAndSymbol: InputScopeNameValue = InputScopeNameValue(20i32);
    pub const CurrencyAmount: InputScopeNameValue = InputScopeNameValue(21i32);
    pub const DateMonthNumber: InputScopeNameValue = InputScopeNameValue(23i32);
    pub const DateDayNumber: InputScopeNameValue = InputScopeNameValue(24i32);
    pub const DateYear: InputScopeNameValue = InputScopeNameValue(25i32);
    pub const Digits: InputScopeNameValue = InputScopeNameValue(28i32);
    pub const Number: InputScopeNameValue = InputScopeNameValue(29i32);
    pub const Password: InputScopeNameValue = InputScopeNameValue(31i32);
    pub const TelephoneNumber: InputScopeNameValue = InputScopeNameValue(32i32);
    pub const TelephoneCountryCode: InputScopeNameValue = InputScopeNameValue(33i32);
    pub const TelephoneAreaCode: InputScopeNameValue = InputScopeNameValue(34i32);
    pub const TelephoneLocalNumber: InputScopeNameValue = InputScopeNameValue(35i32);
    pub const TimeHour: InputScopeNameValue = InputScopeNameValue(37i32);
    pub const TimeMinutesOrSeconds: InputScopeNameValue = InputScopeNameValue(38i32);
    pub const NumberFullWidth: InputScopeNameValue = InputScopeNameValue(39i32);
    pub const AlphanumericHalfWidth: InputScopeNameValue = InputScopeNameValue(40i32);
    pub const AlphanumericFullWidth: InputScopeNameValue = InputScopeNameValue(41i32);
    pub const Hiragana: InputScopeNameValue = InputScopeNameValue(44i32);
    pub const KatakanaHalfWidth: InputScopeNameValue = InputScopeNameValue(45i32);
    pub const KatakanaFullWidth: InputScopeNameValue = InputScopeNameValue(46i32);
    pub const Hanja: InputScopeNameValue = InputScopeNameValue(47i32);
    pub const HangulHalfWidth: InputScopeNameValue = InputScopeNameValue(48i32);
    pub const HangulFullWidth: InputScopeNameValue = InputScopeNameValue(49i32);
    pub const Search: InputScopeNameValue = InputScopeNameValue(50i32);
    pub const Formula: InputScopeNameValue = InputScopeNameValue(51i32);
    pub const SearchIncremental: InputScopeNameValue = InputScopeNameValue(52i32);
    pub const ChineseHalfWidth: InputScopeNameValue = InputScopeNameValue(53i32);
    pub const ChineseFullWidth: InputScopeNameValue = InputScopeNameValue(54i32);
    pub const NativeScript: InputScopeNameValue = InputScopeNameValue(55i32);
    pub const Text: InputScopeNameValue = InputScopeNameValue(57i32);
    pub const Chat: InputScopeNameValue = InputScopeNameValue(58i32);
    pub const NameOrPhoneNumber: InputScopeNameValue = InputScopeNameValue(59i32);
    pub const EmailNameOrAddress: InputScopeNameValue = InputScopeNameValue(60i32);
    pub const Private: InputScopeNameValue = InputScopeNameValue(61i32);
    pub const Maps: InputScopeNameValue = InputScopeNameValue(62i32);
    pub const NumericPassword: InputScopeNameValue = InputScopeNameValue(63i32);
    pub const NumericPin: InputScopeNameValue = InputScopeNameValue(64i32);
    pub const AlphanumericPin: InputScopeNameValue = InputScopeNameValue(65i32);
    pub const FormulaNumber: InputScopeNameValue = InputScopeNameValue(67i32);
    pub const ChatWithoutEmoji: InputScopeNameValue = InputScopeNameValue(68i32);
}
impl ::core::convert::From<i32> for InputScopeNameValue {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InputScopeNameValue {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for InputScopeNameValue {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Input.InputScopeNameValue;i4)");
}
impl ::windows::runtime::DefaultType for InputScopeNameValue {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Xaml_Input`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct KeyEventHandler(::windows::runtime::IUnknown);
impl KeyEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows::runtime::IInspectable>, &::core::option::Option<KeyRoutedEventArgs>) -> ::windows::runtime::Result<()> + 'static>(invoke: F) -> Self {
        let com = KeyEventHandler_box::<F> {
            vtable: &KeyEventHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::runtime::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn Invoke<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, KeyRoutedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).3)(::core::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for KeyEventHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"delegate({7c63d2e5-7a0e-4e12-b96a-7715aa6ff1c8})");
}
unsafe impl ::windows::runtime::Interface for KeyEventHandler {
    type Vtable = KeyEventHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x7c63d2e5_7a0e_4e12_b96a_7715aa6ff1c8);
}
#[repr(C)]
#[doc(hidden)]
pub struct KeyEventHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr, e: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
struct KeyEventHandler_box<F: FnMut(&::core::option::Option<::windows::runtime::IInspectable>, &::core::option::Option<KeyRoutedEventArgs>) -> ::windows::runtime::Result<()> + 'static> {
    vtable: *const KeyEventHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows::runtime::IInspectable>, &::core::option::Option<KeyRoutedEventArgs>) -> ::windows::runtime::Result<()> + 'static> KeyEventHandler_box<F> {
    const VTABLE: KeyEventHandler_abi = KeyEventHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<KeyEventHandler as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::runtime::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr, e: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ((*this).invoke)(
            &*(&sender as *const <::windows::runtime::IInspectable as ::windows::runtime::Abi>::Abi as *const <::windows::runtime::IInspectable as ::windows::runtime::DefaultType>::DefaultType),
            &*(&e as *const <KeyRoutedEventArgs as ::windows::runtime::Abi>::Abi as *const <KeyRoutedEventArgs as ::windows::runtime::DefaultType>::DefaultType),
        )
        .into()
    }
}
#[doc = "*Required features: `UI_Xaml_Input`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct KeyRoutedEventArgs(pub ::windows::runtime::IInspectable);
impl KeyRoutedEventArgs {
    #[cfg(feature = "System")]
    #[doc = "*Required features: `UI_Xaml_Input`, `System`*"]
    pub fn Key(&self) -> ::windows::runtime::Result<super::super::super::System::VirtualKey> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::System::VirtualKey = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::VirtualKey>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    #[doc = "*Required features: `UI_Xaml_Input`, `UI_Core`*"]
    pub fn KeyStatus(&self) -> ::windows::runtime::Result<super::super::Core::CorePhysicalKeyStatus> {
        let this = self;
        unsafe {
            let mut result__: super::super::Core::CorePhysicalKeyStatus = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Core::CorePhysicalKeyStatus>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn Handled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `UI_Xaml_Input`, `System`*"]
    pub fn OriginalKey(&self) -> ::windows::runtime::Result<super::super::super::System::VirtualKey> {
        let this = &::windows::runtime::Interface::cast::<IKeyRoutedEventArgs2>(self)?;
        unsafe {
            let mut result__: super::super::super::System::VirtualKey = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::VirtualKey>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn DeviceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IKeyRoutedEventArgs3>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn OriginalSource(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IRoutedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for KeyRoutedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.KeyRoutedEventArgs;{d4cd3dfe-4079-42e9-a39a-3095d3f049c6})");
}
unsafe impl ::windows::runtime::Interface for KeyRoutedEventArgs {
    type Vtable = IKeyRoutedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd4cd3dfe_4079_42e9_a39a_3095d3f049c6);
}
impl ::windows::runtime::RuntimeName for KeyRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.KeyRoutedEventArgs";
}
impl ::core::convert::From<KeyRoutedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: KeyRoutedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&KeyRoutedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &KeyRoutedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for KeyRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a KeyRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<KeyRoutedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: KeyRoutedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&KeyRoutedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &KeyRoutedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for KeyRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a KeyRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<KeyRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: KeyRoutedEventArgs) -> Self {
        ::core::convert::Into::<super::RoutedEventArgs>::into(&value)
    }
}
impl ::core::convert::From<&KeyRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: &KeyRoutedEventArgs) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::RoutedEventArgs> for KeyRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::RoutedEventArgs> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::RoutedEventArgs> for &KeyRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::RoutedEventArgs> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for KeyRoutedEventArgs {}
unsafe impl ::core::marker::Sync for KeyRoutedEventArgs {}
#[doc = "*Required features: `UI_Xaml_Input`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KeyTipPlacementMode(pub i32);
impl KeyTipPlacementMode {
    pub const Auto: KeyTipPlacementMode = KeyTipPlacementMode(0i32);
    pub const Bottom: KeyTipPlacementMode = KeyTipPlacementMode(1i32);
    pub const Top: KeyTipPlacementMode = KeyTipPlacementMode(2i32);
    pub const Left: KeyTipPlacementMode = KeyTipPlacementMode(3i32);
    pub const Right: KeyTipPlacementMode = KeyTipPlacementMode(4i32);
    pub const Center: KeyTipPlacementMode = KeyTipPlacementMode(5i32);
    pub const Hidden: KeyTipPlacementMode = KeyTipPlacementMode(6i32);
}
impl ::core::convert::From<i32> for KeyTipPlacementMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KeyTipPlacementMode {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for KeyTipPlacementMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Input.KeyTipPlacementMode;i4)");
}
impl ::windows::runtime::DefaultType for KeyTipPlacementMode {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Xaml_Input`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct KeyboardAccelerator(pub ::windows::runtime::IInspectable);
impl KeyboardAccelerator {
    #[cfg(feature = "System")]
    #[doc = "*Required features: `UI_Xaml_Input`, `System`*"]
    pub fn Key(&self) -> ::windows::runtime::Result<super::super::super::System::VirtualKey> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::System::VirtualKey = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::VirtualKey>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `UI_Xaml_Input`, `System`*"]
    pub fn SetKey(&self, value: super::super::super::System::VirtualKey) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `UI_Xaml_Input`, `System`*"]
    pub fn Modifiers(&self) -> ::windows::runtime::Result<super::super::super::System::VirtualKeyModifiers> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::System::VirtualKeyModifiers = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::VirtualKeyModifiers>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `UI_Xaml_Input`, `System`*"]
    pub fn SetModifiers(&self, value: super::super::super::System::VirtualKeyModifiers) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn IsEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn SetIsEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn ScopeOwner(&self) -> ::windows::runtime::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn SetScopeOwner<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Input`, `Foundation`*"]
    pub fn Invoked<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<KeyboardAccelerator, KeyboardAcceleratorInvokedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Input`, `Foundation`*"]
    pub fn RemoveInvoked<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn KeyProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IKeyboardAcceleratorStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn ModifiersProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IKeyboardAcceleratorStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn IsEnabledProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IKeyboardAcceleratorStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn ScopeOwnerProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IKeyboardAcceleratorStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn new() -> ::windows::runtime::Result<KeyboardAccelerator> {
        Self::IKeyboardAcceleratorFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::runtime::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<KeyboardAccelerator>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn GetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn SetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, dp: Param0, value: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), dp.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn ClearValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), dp.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn ReadLocalValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn GetAnimationBaseValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    #[doc = "*Required features: `UI_Xaml_Input`, `UI_Core`*"]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn RegisterPropertyChangedCallback<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>, Param1: ::windows::runtime::IntoParam<'a, super::DependencyPropertyChangedCallback>>(&self, dp: Param0, callback: Param1) -> ::windows::runtime::Result<i64> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject2>(self)?;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), dp.into_param().abi(), callback.into_param().abi(), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn UnregisterPropertyChangedCallback<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(&self, dp: Param0, token: i64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), dp.into_param().abi(), token).ok() }
    }
    pub fn IKeyboardAcceleratorStatics<R, F: FnOnce(&IKeyboardAcceleratorStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<KeyboardAccelerator, IKeyboardAcceleratorStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IKeyboardAcceleratorFactory<R, F: FnOnce(&IKeyboardAcceleratorFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<KeyboardAccelerator, IKeyboardAcceleratorFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for KeyboardAccelerator {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.KeyboardAccelerator;{92e6181e-19ae-465a-9b3c-a71ee9ea7420})");
}
unsafe impl ::windows::runtime::Interface for KeyboardAccelerator {
    type Vtable = IKeyboardAccelerator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x92e6181e_19ae_465a_9b3c_a71ee9ea7420);
}
impl ::windows::runtime::RuntimeName for KeyboardAccelerator {
    const NAME: &'static str = "Windows.UI.Xaml.Input.KeyboardAccelerator";
}
impl ::core::convert::From<KeyboardAccelerator> for ::windows::runtime::IUnknown {
    fn from(value: KeyboardAccelerator) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&KeyboardAccelerator> for ::windows::runtime::IUnknown {
    fn from(value: &KeyboardAccelerator) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for KeyboardAccelerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a KeyboardAccelerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<KeyboardAccelerator> for ::windows::runtime::IInspectable {
    fn from(value: KeyboardAccelerator) -> Self {
        value.0
    }
}
impl ::core::convert::From<&KeyboardAccelerator> for ::windows::runtime::IInspectable {
    fn from(value: &KeyboardAccelerator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for KeyboardAccelerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a KeyboardAccelerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<KeyboardAccelerator> for super::DependencyObject {
    fn from(value: KeyboardAccelerator) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&KeyboardAccelerator> for super::DependencyObject {
    fn from(value: &KeyboardAccelerator) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for KeyboardAccelerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for &KeyboardAccelerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for KeyboardAccelerator {}
unsafe impl ::core::marker::Sync for KeyboardAccelerator {}
#[doc = "*Required features: `UI_Xaml_Input`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct KeyboardAcceleratorInvokedEventArgs(pub ::windows::runtime::IInspectable);
impl KeyboardAcceleratorInvokedEventArgs {
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn Handled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn Element(&self) -> ::windows::runtime::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn KeyboardAccelerator(&self) -> ::windows::runtime::Result<KeyboardAccelerator> {
        let this = &::windows::runtime::Interface::cast::<IKeyboardAcceleratorInvokedEventArgs2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<KeyboardAccelerator>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for KeyboardAcceleratorInvokedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.KeyboardAcceleratorInvokedEventArgs;{c00b03f2-04e7-4415-b17d-d76b9490de2b})");
}
unsafe impl ::windows::runtime::Interface for KeyboardAcceleratorInvokedEventArgs {
    type Vtable = IKeyboardAcceleratorInvokedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc00b03f2_04e7_4415_b17d_d76b9490de2b);
}
impl ::windows::runtime::RuntimeName for KeyboardAcceleratorInvokedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.KeyboardAcceleratorInvokedEventArgs";
}
impl ::core::convert::From<KeyboardAcceleratorInvokedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: KeyboardAcceleratorInvokedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&KeyboardAcceleratorInvokedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &KeyboardAcceleratorInvokedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for KeyboardAcceleratorInvokedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a KeyboardAcceleratorInvokedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<KeyboardAcceleratorInvokedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: KeyboardAcceleratorInvokedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&KeyboardAcceleratorInvokedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &KeyboardAcceleratorInvokedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for KeyboardAcceleratorInvokedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a KeyboardAcceleratorInvokedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for KeyboardAcceleratorInvokedEventArgs {}
unsafe impl ::core::marker::Sync for KeyboardAcceleratorInvokedEventArgs {}
#[doc = "*Required features: `UI_Xaml_Input`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KeyboardAcceleratorPlacementMode(pub i32);
impl KeyboardAcceleratorPlacementMode {
    pub const Auto: KeyboardAcceleratorPlacementMode = KeyboardAcceleratorPlacementMode(0i32);
    pub const Hidden: KeyboardAcceleratorPlacementMode = KeyboardAcceleratorPlacementMode(1i32);
}
impl ::core::convert::From<i32> for KeyboardAcceleratorPlacementMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KeyboardAcceleratorPlacementMode {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for KeyboardAcceleratorPlacementMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Input.KeyboardAcceleratorPlacementMode;i4)");
}
impl ::windows::runtime::DefaultType for KeyboardAcceleratorPlacementMode {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Xaml_Input`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KeyboardNavigationMode(pub i32);
impl KeyboardNavigationMode {
    pub const Local: KeyboardNavigationMode = KeyboardNavigationMode(0i32);
    pub const Cycle: KeyboardNavigationMode = KeyboardNavigationMode(1i32);
    pub const Once: KeyboardNavigationMode = KeyboardNavigationMode(2i32);
}
impl ::core::convert::From<i32> for KeyboardNavigationMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KeyboardNavigationMode {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for KeyboardNavigationMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Input.KeyboardNavigationMode;i4)");
}
impl ::windows::runtime::DefaultType for KeyboardNavigationMode {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Xaml_Input`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LosingFocusEventArgs(pub ::windows::runtime::IInspectable);
impl LosingFocusEventArgs {
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn OldFocusedElement(&self) -> ::windows::runtime::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn NewFocusedElement(&self) -> ::windows::runtime::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn SetNewFocusedElement<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn FocusState(&self) -> ::windows::runtime::Result<super::FocusState> {
        let this = self;
        unsafe {
            let mut result__: super::FocusState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::FocusState>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn Direction(&self) -> ::windows::runtime::Result<FocusNavigationDirection> {
        let this = self;
        unsafe {
            let mut result__: FocusNavigationDirection = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FocusNavigationDirection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn Handled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn InputDevice(&self) -> ::windows::runtime::Result<FocusInputDeviceKind> {
        let this = self;
        unsafe {
            let mut result__: FocusInputDeviceKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FocusInputDeviceKind>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn Cancel(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn SetCancel(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn TryCancel(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ILosingFocusEventArgs2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn TrySetNewFocusedElement<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>>(&self, element: Param0) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ILosingFocusEventArgs2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn CorrelationId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = &::windows::runtime::Interface::cast::<ILosingFocusEventArgs3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn OriginalSource(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IRoutedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for LosingFocusEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.LosingFocusEventArgs;{f9f683c7-d789-472b-aa93-6d4105e6dabe})");
}
unsafe impl ::windows::runtime::Interface for LosingFocusEventArgs {
    type Vtable = ILosingFocusEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf9f683c7_d789_472b_aa93_6d4105e6dabe);
}
impl ::windows::runtime::RuntimeName for LosingFocusEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.LosingFocusEventArgs";
}
impl ::core::convert::From<LosingFocusEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: LosingFocusEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LosingFocusEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &LosingFocusEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for LosingFocusEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a LosingFocusEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LosingFocusEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: LosingFocusEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LosingFocusEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &LosingFocusEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for LosingFocusEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a LosingFocusEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<LosingFocusEventArgs> for super::RoutedEventArgs {
    fn from(value: LosingFocusEventArgs) -> Self {
        ::core::convert::Into::<super::RoutedEventArgs>::into(&value)
    }
}
impl ::core::convert::From<&LosingFocusEventArgs> for super::RoutedEventArgs {
    fn from(value: &LosingFocusEventArgs) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::RoutedEventArgs> for LosingFocusEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::RoutedEventArgs> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::RoutedEventArgs> for &LosingFocusEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::RoutedEventArgs> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for LosingFocusEventArgs {}
unsafe impl ::core::marker::Sync for LosingFocusEventArgs {}
#[doc = "*Required features: `UI_Xaml_Input`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ManipulationCompletedEventHandler(::windows::runtime::IUnknown);
impl ManipulationCompletedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows::runtime::IInspectable>, &::core::option::Option<ManipulationCompletedRoutedEventArgs>) -> ::windows::runtime::Result<()> + 'static>(invoke: F) -> Self {
        let com = ManipulationCompletedEventHandler_box::<F> {
            vtable: &ManipulationCompletedEventHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::runtime::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn Invoke<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, ManipulationCompletedRoutedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).3)(::core::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ManipulationCompletedEventHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"delegate({38ef4b0f-14f8-42df-9a1e-a4bcc4af77f4})");
}
unsafe impl ::windows::runtime::Interface for ManipulationCompletedEventHandler {
    type Vtable = ManipulationCompletedEventHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x38ef4b0f_14f8_42df_9a1e_a4bcc4af77f4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ManipulationCompletedEventHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr, e: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
struct ManipulationCompletedEventHandler_box<F: FnMut(&::core::option::Option<::windows::runtime::IInspectable>, &::core::option::Option<ManipulationCompletedRoutedEventArgs>) -> ::windows::runtime::Result<()> + 'static> {
    vtable: *const ManipulationCompletedEventHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows::runtime::IInspectable>, &::core::option::Option<ManipulationCompletedRoutedEventArgs>) -> ::windows::runtime::Result<()> + 'static> ManipulationCompletedEventHandler_box<F> {
    const VTABLE: ManipulationCompletedEventHandler_abi = ManipulationCompletedEventHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<ManipulationCompletedEventHandler as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::runtime::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr, e: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ((*this).invoke)(
            &*(&sender as *const <::windows::runtime::IInspectable as ::windows::runtime::Abi>::Abi as *const <::windows::runtime::IInspectable as ::windows::runtime::DefaultType>::DefaultType),
            &*(&e as *const <ManipulationCompletedRoutedEventArgs as ::windows::runtime::Abi>::Abi as *const <ManipulationCompletedRoutedEventArgs as ::windows::runtime::DefaultType>::DefaultType),
        )
        .into()
    }
}
#[doc = "*Required features: `UI_Xaml_Input`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ManipulationCompletedRoutedEventArgs(pub ::windows::runtime::IInspectable);
impl ManipulationCompletedRoutedEventArgs {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ManipulationCompletedRoutedEventArgs, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn Container(&self) -> ::windows::runtime::Result<super::UIElement> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::UIElement>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Input`, `Foundation`*"]
    pub fn Position(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn IsInertial(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Input"))]
    #[doc = "*Required features: `UI_Xaml_Input`, `Foundation`, `UI_Input`*"]
    pub fn Cumulative(&self) -> ::windows::runtime::Result<super::super::Input::ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__: super::super::Input::ManipulationDelta = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Input::ManipulationDelta>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Input"))]
    #[doc = "*Required features: `UI_Xaml_Input`, `Foundation`, `UI_Input`*"]
    pub fn Velocities(&self) -> ::windows::runtime::Result<super::super::Input::ManipulationVelocities> {
        let this = self;
        unsafe {
            let mut result__: super::super::Input::ManipulationVelocities = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Input::ManipulationVelocities>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn Handled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Devices_Input")]
    #[doc = "*Required features: `UI_Xaml_Input`, `Devices_Input`*"]
    pub fn PointerDeviceType(&self) -> ::windows::runtime::Result<super::super::super::Devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Devices::Input::PointerDeviceType = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Devices::Input::PointerDeviceType>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn OriginalSource(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IRoutedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ManipulationCompletedRoutedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.ManipulationCompletedRoutedEventArgs;{b5ad9b23-2f41-498e-8319-015ee8a75346})");
}
unsafe impl ::windows::runtime::Interface for ManipulationCompletedRoutedEventArgs {
    type Vtable = IManipulationCompletedRoutedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xb5ad9b23_2f41_498e_8319_015ee8a75346);
}
impl ::windows::runtime::RuntimeName for ManipulationCompletedRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.ManipulationCompletedRoutedEventArgs";
}
impl ::core::convert::From<ManipulationCompletedRoutedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: ManipulationCompletedRoutedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ManipulationCompletedRoutedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &ManipulationCompletedRoutedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ManipulationCompletedRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ManipulationCompletedRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ManipulationCompletedRoutedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: ManipulationCompletedRoutedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ManipulationCompletedRoutedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &ManipulationCompletedRoutedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ManipulationCompletedRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ManipulationCompletedRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ManipulationCompletedRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: ManipulationCompletedRoutedEventArgs) -> Self {
        ::core::convert::Into::<super::RoutedEventArgs>::into(&value)
    }
}
impl ::core::convert::From<&ManipulationCompletedRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: &ManipulationCompletedRoutedEventArgs) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::RoutedEventArgs> for ManipulationCompletedRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::RoutedEventArgs> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::RoutedEventArgs> for &ManipulationCompletedRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::RoutedEventArgs> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for ManipulationCompletedRoutedEventArgs {}
unsafe impl ::core::marker::Sync for ManipulationCompletedRoutedEventArgs {}
#[doc = "*Required features: `UI_Xaml_Input`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ManipulationDeltaEventHandler(::windows::runtime::IUnknown);
impl ManipulationDeltaEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows::runtime::IInspectable>, &::core::option::Option<ManipulationDeltaRoutedEventArgs>) -> ::windows::runtime::Result<()> + 'static>(invoke: F) -> Self {
        let com = ManipulationDeltaEventHandler_box::<F> {
            vtable: &ManipulationDeltaEventHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::runtime::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn Invoke<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, ManipulationDeltaRoutedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).3)(::core::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ManipulationDeltaEventHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"delegate({aa1160cb-dfb9-4c56-abdc-711b63c8eb94})");
}
unsafe impl ::windows::runtime::Interface for ManipulationDeltaEventHandler {
    type Vtable = ManipulationDeltaEventHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xaa1160cb_dfb9_4c56_abdc_711b63c8eb94);
}
#[repr(C)]
#[doc(hidden)]
pub struct ManipulationDeltaEventHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr, e: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
struct ManipulationDeltaEventHandler_box<F: FnMut(&::core::option::Option<::windows::runtime::IInspectable>, &::core::option::Option<ManipulationDeltaRoutedEventArgs>) -> ::windows::runtime::Result<()> + 'static> {
    vtable: *const ManipulationDeltaEventHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows::runtime::IInspectable>, &::core::option::Option<ManipulationDeltaRoutedEventArgs>) -> ::windows::runtime::Result<()> + 'static> ManipulationDeltaEventHandler_box<F> {
    const VTABLE: ManipulationDeltaEventHandler_abi = ManipulationDeltaEventHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<ManipulationDeltaEventHandler as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::runtime::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr, e: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ((*this).invoke)(
            &*(&sender as *const <::windows::runtime::IInspectable as ::windows::runtime::Abi>::Abi as *const <::windows::runtime::IInspectable as ::windows::runtime::DefaultType>::DefaultType),
            &*(&e as *const <ManipulationDeltaRoutedEventArgs as ::windows::runtime::Abi>::Abi as *const <ManipulationDeltaRoutedEventArgs as ::windows::runtime::DefaultType>::DefaultType),
        )
        .into()
    }
}
#[doc = "*Required features: `UI_Xaml_Input`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ManipulationDeltaRoutedEventArgs(pub ::windows::runtime::IInspectable);
impl ManipulationDeltaRoutedEventArgs {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ManipulationDeltaRoutedEventArgs, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn Container(&self) -> ::windows::runtime::Result<super::UIElement> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::UIElement>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Input`, `Foundation`*"]
    pub fn Position(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn IsInertial(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Input"))]
    #[doc = "*Required features: `UI_Xaml_Input`, `Foundation`, `UI_Input`*"]
    pub fn Delta(&self) -> ::windows::runtime::Result<super::super::Input::ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__: super::super::Input::ManipulationDelta = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Input::ManipulationDelta>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Input"))]
    #[doc = "*Required features: `UI_Xaml_Input`, `Foundation`, `UI_Input`*"]
    pub fn Cumulative(&self) -> ::windows::runtime::Result<super::super::Input::ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__: super::super::Input::ManipulationDelta = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Input::ManipulationDelta>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Input"))]
    #[doc = "*Required features: `UI_Xaml_Input`, `Foundation`, `UI_Input`*"]
    pub fn Velocities(&self) -> ::windows::runtime::Result<super::super::Input::ManipulationVelocities> {
        let this = self;
        unsafe {
            let mut result__: super::super::Input::ManipulationVelocities = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Input::ManipulationVelocities>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn Handled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Devices_Input")]
    #[doc = "*Required features: `UI_Xaml_Input`, `Devices_Input`*"]
    pub fn PointerDeviceType(&self) -> ::windows::runtime::Result<super::super::super::Devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Devices::Input::PointerDeviceType = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Devices::Input::PointerDeviceType>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn Complete(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn OriginalSource(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IRoutedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ManipulationDeltaRoutedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.ManipulationDeltaRoutedEventArgs;{400d5794-4c6f-491d-82d6-3517109399c6})");
}
unsafe impl ::windows::runtime::Interface for ManipulationDeltaRoutedEventArgs {
    type Vtable = IManipulationDeltaRoutedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x400d5794_4c6f_491d_82d6_3517109399c6);
}
impl ::windows::runtime::RuntimeName for ManipulationDeltaRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.ManipulationDeltaRoutedEventArgs";
}
impl ::core::convert::From<ManipulationDeltaRoutedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: ManipulationDeltaRoutedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ManipulationDeltaRoutedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &ManipulationDeltaRoutedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ManipulationDeltaRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ManipulationDeltaRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ManipulationDeltaRoutedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: ManipulationDeltaRoutedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ManipulationDeltaRoutedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &ManipulationDeltaRoutedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ManipulationDeltaRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ManipulationDeltaRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ManipulationDeltaRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: ManipulationDeltaRoutedEventArgs) -> Self {
        ::core::convert::Into::<super::RoutedEventArgs>::into(&value)
    }
}
impl ::core::convert::From<&ManipulationDeltaRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: &ManipulationDeltaRoutedEventArgs) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::RoutedEventArgs> for ManipulationDeltaRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::RoutedEventArgs> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::RoutedEventArgs> for &ManipulationDeltaRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::RoutedEventArgs> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for ManipulationDeltaRoutedEventArgs {}
unsafe impl ::core::marker::Sync for ManipulationDeltaRoutedEventArgs {}
#[doc = "*Required features: `UI_Xaml_Input`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ManipulationInertiaStartingEventHandler(::windows::runtime::IUnknown);
impl ManipulationInertiaStartingEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows::runtime::IInspectable>, &::core::option::Option<ManipulationInertiaStartingRoutedEventArgs>) -> ::windows::runtime::Result<()> + 'static>(invoke: F) -> Self {
        let com = ManipulationInertiaStartingEventHandler_box::<F> {
            vtable: &ManipulationInertiaStartingEventHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::runtime::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn Invoke<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, ManipulationInertiaStartingRoutedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).3)(::core::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ManipulationInertiaStartingEventHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"delegate({d39d6322-7c9c-481b-827b-c8b2d9bb6fc7})");
}
unsafe impl ::windows::runtime::Interface for ManipulationInertiaStartingEventHandler {
    type Vtable = ManipulationInertiaStartingEventHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd39d6322_7c9c_481b_827b_c8b2d9bb6fc7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ManipulationInertiaStartingEventHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr, e: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
struct ManipulationInertiaStartingEventHandler_box<F: FnMut(&::core::option::Option<::windows::runtime::IInspectable>, &::core::option::Option<ManipulationInertiaStartingRoutedEventArgs>) -> ::windows::runtime::Result<()> + 'static> {
    vtable: *const ManipulationInertiaStartingEventHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows::runtime::IInspectable>, &::core::option::Option<ManipulationInertiaStartingRoutedEventArgs>) -> ::windows::runtime::Result<()> + 'static> ManipulationInertiaStartingEventHandler_box<F> {
    const VTABLE: ManipulationInertiaStartingEventHandler_abi = ManipulationInertiaStartingEventHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<ManipulationInertiaStartingEventHandler as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::runtime::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr, e: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ((*this).invoke)(
            &*(&sender as *const <::windows::runtime::IInspectable as ::windows::runtime::Abi>::Abi as *const <::windows::runtime::IInspectable as ::windows::runtime::DefaultType>::DefaultType),
            &*(&e as *const <ManipulationInertiaStartingRoutedEventArgs as ::windows::runtime::Abi>::Abi as *const <ManipulationInertiaStartingRoutedEventArgs as ::windows::runtime::DefaultType>::DefaultType),
        )
        .into()
    }
}
#[doc = "*Required features: `UI_Xaml_Input`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ManipulationInertiaStartingRoutedEventArgs(pub ::windows::runtime::IInspectable);
impl ManipulationInertiaStartingRoutedEventArgs {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ManipulationInertiaStartingRoutedEventArgs, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn Container(&self) -> ::windows::runtime::Result<super::UIElement> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::UIElement>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn ExpansionBehavior(&self) -> ::windows::runtime::Result<InertiaExpansionBehavior> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InertiaExpansionBehavior>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn SetExpansionBehavior<'a, Param0: ::windows::runtime::IntoParam<'a, InertiaExpansionBehavior>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn RotationBehavior(&self) -> ::windows::runtime::Result<InertiaRotationBehavior> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InertiaRotationBehavior>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn SetRotationBehavior<'a, Param0: ::windows::runtime::IntoParam<'a, InertiaRotationBehavior>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn TranslationBehavior(&self) -> ::windows::runtime::Result<InertiaTranslationBehavior> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InertiaTranslationBehavior>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn SetTranslationBehavior<'a, Param0: ::windows::runtime::IntoParam<'a, InertiaTranslationBehavior>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn Handled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Devices_Input")]
    #[doc = "*Required features: `UI_Xaml_Input`, `Devices_Input`*"]
    pub fn PointerDeviceType(&self) -> ::windows::runtime::Result<super::super::super::Devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Devices::Input::PointerDeviceType = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Devices::Input::PointerDeviceType>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Input"))]
    #[doc = "*Required features: `UI_Xaml_Input`, `Foundation`, `UI_Input`*"]
    pub fn Delta(&self) -> ::windows::runtime::Result<super::super::Input::ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__: super::super::Input::ManipulationDelta = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Input::ManipulationDelta>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Input"))]
    #[doc = "*Required features: `UI_Xaml_Input`, `Foundation`, `UI_Input`*"]
    pub fn Cumulative(&self) -> ::windows::runtime::Result<super::super::Input::ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__: super::super::Input::ManipulationDelta = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Input::ManipulationDelta>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Input"))]
    #[doc = "*Required features: `UI_Xaml_Input`, `Foundation`, `UI_Input`*"]
    pub fn Velocities(&self) -> ::windows::runtime::Result<super::super::Input::ManipulationVelocities> {
        let this = self;
        unsafe {
            let mut result__: super::super::Input::ManipulationVelocities = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Input::ManipulationVelocities>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn OriginalSource(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IRoutedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ManipulationInertiaStartingRoutedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.ManipulationInertiaStartingRoutedEventArgs;{246a91a9-ca43-4c0b-acef-81e8b8147520})");
}
unsafe impl ::windows::runtime::Interface for ManipulationInertiaStartingRoutedEventArgs {
    type Vtable = IManipulationInertiaStartingRoutedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x246a91a9_ca43_4c0b_acef_81e8b8147520);
}
impl ::windows::runtime::RuntimeName for ManipulationInertiaStartingRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.ManipulationInertiaStartingRoutedEventArgs";
}
impl ::core::convert::From<ManipulationInertiaStartingRoutedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: ManipulationInertiaStartingRoutedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ManipulationInertiaStartingRoutedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &ManipulationInertiaStartingRoutedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ManipulationInertiaStartingRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ManipulationInertiaStartingRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ManipulationInertiaStartingRoutedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: ManipulationInertiaStartingRoutedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ManipulationInertiaStartingRoutedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &ManipulationInertiaStartingRoutedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ManipulationInertiaStartingRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ManipulationInertiaStartingRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ManipulationInertiaStartingRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: ManipulationInertiaStartingRoutedEventArgs) -> Self {
        ::core::convert::Into::<super::RoutedEventArgs>::into(&value)
    }
}
impl ::core::convert::From<&ManipulationInertiaStartingRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: &ManipulationInertiaStartingRoutedEventArgs) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::RoutedEventArgs> for ManipulationInertiaStartingRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::RoutedEventArgs> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::RoutedEventArgs> for &ManipulationInertiaStartingRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::RoutedEventArgs> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for ManipulationInertiaStartingRoutedEventArgs {}
unsafe impl ::core::marker::Sync for ManipulationInertiaStartingRoutedEventArgs {}
#[doc = "*Required features: `UI_Xaml_Input`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ManipulationModes(pub u32);
impl ManipulationModes {
    pub const None: ManipulationModes = ManipulationModes(0u32);
    pub const TranslateX: ManipulationModes = ManipulationModes(1u32);
    pub const TranslateY: ManipulationModes = ManipulationModes(2u32);
    pub const TranslateRailsX: ManipulationModes = ManipulationModes(4u32);
    pub const TranslateRailsY: ManipulationModes = ManipulationModes(8u32);
    pub const Rotate: ManipulationModes = ManipulationModes(16u32);
    pub const Scale: ManipulationModes = ManipulationModes(32u32);
    pub const TranslateInertia: ManipulationModes = ManipulationModes(64u32);
    pub const RotateInertia: ManipulationModes = ManipulationModes(128u32);
    pub const ScaleInertia: ManipulationModes = ManipulationModes(256u32);
    pub const All: ManipulationModes = ManipulationModes(65535u32);
    pub const System: ManipulationModes = ManipulationModes(65536u32);
}
impl ::core::convert::From<u32> for ManipulationModes {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ManipulationModes {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ManipulationModes {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Input.ManipulationModes;u4)");
}
impl ::windows::runtime::DefaultType for ManipulationModes {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for ManipulationModes {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for ManipulationModes {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for ManipulationModes {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for ManipulationModes {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for ManipulationModes {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `UI_Xaml_Input`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ManipulationPivot(pub ::windows::runtime::IInspectable);
impl ManipulationPivot {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ManipulationPivot, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Input`, `Foundation`*"]
    pub fn Center(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Input`, `Foundation`*"]
    pub fn SetCenter<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Point>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn Radius(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn SetRadius(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Input`, `Foundation`*"]
    pub fn CreateInstanceWithCenterAndRadius<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Point>>(center: Param0, radius: f64) -> ::windows::runtime::Result<ManipulationPivot> {
        Self::IManipulationPivotFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), center.into_param().abi(), radius, &mut result__).from_abi::<ManipulationPivot>(result__)
        })
    }
    pub fn IManipulationPivotFactory<R, F: FnOnce(&IManipulationPivotFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ManipulationPivot, IManipulationPivotFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ManipulationPivot {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.ManipulationPivot;{2e3838a5-e6c2-4998-82ac-18748b141666})");
}
unsafe impl ::windows::runtime::Interface for ManipulationPivot {
    type Vtable = IManipulationPivot_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x2e3838a5_e6c2_4998_82ac_18748b141666);
}
impl ::windows::runtime::RuntimeName for ManipulationPivot {
    const NAME: &'static str = "Windows.UI.Xaml.Input.ManipulationPivot";
}
impl ::core::convert::From<ManipulationPivot> for ::windows::runtime::IUnknown {
    fn from(value: ManipulationPivot) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ManipulationPivot> for ::windows::runtime::IUnknown {
    fn from(value: &ManipulationPivot) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ManipulationPivot {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ManipulationPivot {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ManipulationPivot> for ::windows::runtime::IInspectable {
    fn from(value: ManipulationPivot) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ManipulationPivot> for ::windows::runtime::IInspectable {
    fn from(value: &ManipulationPivot) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ManipulationPivot {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ManipulationPivot {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ManipulationPivot {}
unsafe impl ::core::marker::Sync for ManipulationPivot {}
#[doc = "*Required features: `UI_Xaml_Input`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ManipulationStartedEventHandler(::windows::runtime::IUnknown);
impl ManipulationStartedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows::runtime::IInspectable>, &::core::option::Option<ManipulationStartedRoutedEventArgs>) -> ::windows::runtime::Result<()> + 'static>(invoke: F) -> Self {
        let com = ManipulationStartedEventHandler_box::<F> {
            vtable: &ManipulationStartedEventHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::runtime::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn Invoke<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, ManipulationStartedRoutedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).3)(::core::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ManipulationStartedEventHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"delegate({f88345f8-e0a3-4be2-b90c-dc20e6d8beb0})");
}
unsafe impl ::windows::runtime::Interface for ManipulationStartedEventHandler {
    type Vtable = ManipulationStartedEventHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf88345f8_e0a3_4be2_b90c_dc20e6d8beb0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ManipulationStartedEventHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr, e: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
struct ManipulationStartedEventHandler_box<F: FnMut(&::core::option::Option<::windows::runtime::IInspectable>, &::core::option::Option<ManipulationStartedRoutedEventArgs>) -> ::windows::runtime::Result<()> + 'static> {
    vtable: *const ManipulationStartedEventHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows::runtime::IInspectable>, &::core::option::Option<ManipulationStartedRoutedEventArgs>) -> ::windows::runtime::Result<()> + 'static> ManipulationStartedEventHandler_box<F> {
    const VTABLE: ManipulationStartedEventHandler_abi = ManipulationStartedEventHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<ManipulationStartedEventHandler as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::runtime::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr, e: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ((*this).invoke)(
            &*(&sender as *const <::windows::runtime::IInspectable as ::windows::runtime::Abi>::Abi as *const <::windows::runtime::IInspectable as ::windows::runtime::DefaultType>::DefaultType),
            &*(&e as *const <ManipulationStartedRoutedEventArgs as ::windows::runtime::Abi>::Abi as *const <ManipulationStartedRoutedEventArgs as ::windows::runtime::DefaultType>::DefaultType),
        )
        .into()
    }
}
#[doc = "*Required features: `UI_Xaml_Input`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ManipulationStartedRoutedEventArgs(pub ::windows::runtime::IInspectable);
impl ManipulationStartedRoutedEventArgs {
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn Container(&self) -> ::windows::runtime::Result<super::UIElement> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::UIElement>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Input`, `Foundation`*"]
    pub fn Position(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn Handled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Devices_Input")]
    #[doc = "*Required features: `UI_Xaml_Input`, `Devices_Input`*"]
    pub fn PointerDeviceType(&self) -> ::windows::runtime::Result<super::super::super::Devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Devices::Input::PointerDeviceType = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Devices::Input::PointerDeviceType>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Input"))]
    #[doc = "*Required features: `UI_Xaml_Input`, `Foundation`, `UI_Input`*"]
    pub fn Cumulative(&self) -> ::windows::runtime::Result<super::super::Input::ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__: super::super::Input::ManipulationDelta = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Input::ManipulationDelta>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn Complete(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn new() -> ::windows::runtime::Result<ManipulationStartedRoutedEventArgs> {
        Self::IManipulationStartedRoutedEventArgsFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::runtime::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<ManipulationStartedRoutedEventArgs>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn OriginalSource(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IRoutedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    pub fn IManipulationStartedRoutedEventArgsFactory<R, F: FnOnce(&IManipulationStartedRoutedEventArgsFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ManipulationStartedRoutedEventArgs, IManipulationStartedRoutedEventArgsFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ManipulationStartedRoutedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.ManipulationStartedRoutedEventArgs;{5db1aa05-9f80-48b6-ae6c-4f119de8ff13})");
}
unsafe impl ::windows::runtime::Interface for ManipulationStartedRoutedEventArgs {
    type Vtable = IManipulationStartedRoutedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x5db1aa05_9f80_48b6_ae6c_4f119de8ff13);
}
impl ::windows::runtime::RuntimeName for ManipulationStartedRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.ManipulationStartedRoutedEventArgs";
}
impl ::core::convert::From<ManipulationStartedRoutedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: ManipulationStartedRoutedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ManipulationStartedRoutedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &ManipulationStartedRoutedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ManipulationStartedRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ManipulationStartedRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ManipulationStartedRoutedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: ManipulationStartedRoutedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ManipulationStartedRoutedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &ManipulationStartedRoutedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ManipulationStartedRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ManipulationStartedRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ManipulationStartedRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: ManipulationStartedRoutedEventArgs) -> Self {
        ::core::convert::Into::<super::RoutedEventArgs>::into(&value)
    }
}
impl ::core::convert::From<&ManipulationStartedRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: &ManipulationStartedRoutedEventArgs) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::RoutedEventArgs> for ManipulationStartedRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::RoutedEventArgs> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::RoutedEventArgs> for &ManipulationStartedRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::RoutedEventArgs> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for ManipulationStartedRoutedEventArgs {}
unsafe impl ::core::marker::Sync for ManipulationStartedRoutedEventArgs {}
#[doc = "*Required features: `UI_Xaml_Input`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ManipulationStartingEventHandler(::windows::runtime::IUnknown);
impl ManipulationStartingEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows::runtime::IInspectable>, &::core::option::Option<ManipulationStartingRoutedEventArgs>) -> ::windows::runtime::Result<()> + 'static>(invoke: F) -> Self {
        let com = ManipulationStartingEventHandler_box::<F> {
            vtable: &ManipulationStartingEventHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::runtime::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn Invoke<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, ManipulationStartingRoutedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).3)(::core::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ManipulationStartingEventHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"delegate({10d0b04e-bfe4-42cb-823c-3fecd8770ef8})");
}
unsafe impl ::windows::runtime::Interface for ManipulationStartingEventHandler {
    type Vtable = ManipulationStartingEventHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x10d0b04e_bfe4_42cb_823c_3fecd8770ef8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ManipulationStartingEventHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr, e: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
struct ManipulationStartingEventHandler_box<F: FnMut(&::core::option::Option<::windows::runtime::IInspectable>, &::core::option::Option<ManipulationStartingRoutedEventArgs>) -> ::windows::runtime::Result<()> + 'static> {
    vtable: *const ManipulationStartingEventHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows::runtime::IInspectable>, &::core::option::Option<ManipulationStartingRoutedEventArgs>) -> ::windows::runtime::Result<()> + 'static> ManipulationStartingEventHandler_box<F> {
    const VTABLE: ManipulationStartingEventHandler_abi = ManipulationStartingEventHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<ManipulationStartingEventHandler as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::runtime::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr, e: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ((*this).invoke)(
            &*(&sender as *const <::windows::runtime::IInspectable as ::windows::runtime::Abi>::Abi as *const <::windows::runtime::IInspectable as ::windows::runtime::DefaultType>::DefaultType),
            &*(&e as *const <ManipulationStartingRoutedEventArgs as ::windows::runtime::Abi>::Abi as *const <ManipulationStartingRoutedEventArgs as ::windows::runtime::DefaultType>::DefaultType),
        )
        .into()
    }
}
#[doc = "*Required features: `UI_Xaml_Input`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ManipulationStartingRoutedEventArgs(pub ::windows::runtime::IInspectable);
impl ManipulationStartingRoutedEventArgs {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ManipulationStartingRoutedEventArgs, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn Mode(&self) -> ::windows::runtime::Result<ManipulationModes> {
        let this = self;
        unsafe {
            let mut result__: ManipulationModes = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ManipulationModes>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn SetMode(&self, value: ManipulationModes) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn Container(&self) -> ::windows::runtime::Result<super::UIElement> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::UIElement>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn SetContainer<'a, Param0: ::windows::runtime::IntoParam<'a, super::UIElement>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn Pivot(&self) -> ::windows::runtime::Result<ManipulationPivot> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ManipulationPivot>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn SetPivot<'a, Param0: ::windows::runtime::IntoParam<'a, ManipulationPivot>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn Handled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn OriginalSource(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IRoutedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ManipulationStartingRoutedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.ManipulationStartingRoutedEventArgs;{18d636b7-53a4-4c15-a498-f3a9ca212a42})");
}
unsafe impl ::windows::runtime::Interface for ManipulationStartingRoutedEventArgs {
    type Vtable = IManipulationStartingRoutedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x18d636b7_53a4_4c15_a498_f3a9ca212a42);
}
impl ::windows::runtime::RuntimeName for ManipulationStartingRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.ManipulationStartingRoutedEventArgs";
}
impl ::core::convert::From<ManipulationStartingRoutedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: ManipulationStartingRoutedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ManipulationStartingRoutedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &ManipulationStartingRoutedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ManipulationStartingRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ManipulationStartingRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ManipulationStartingRoutedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: ManipulationStartingRoutedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ManipulationStartingRoutedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &ManipulationStartingRoutedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ManipulationStartingRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ManipulationStartingRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ManipulationStartingRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: ManipulationStartingRoutedEventArgs) -> Self {
        ::core::convert::Into::<super::RoutedEventArgs>::into(&value)
    }
}
impl ::core::convert::From<&ManipulationStartingRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: &ManipulationStartingRoutedEventArgs) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::RoutedEventArgs> for ManipulationStartingRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::RoutedEventArgs> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::RoutedEventArgs> for &ManipulationStartingRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::RoutedEventArgs> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for ManipulationStartingRoutedEventArgs {}
unsafe impl ::core::marker::Sync for ManipulationStartingRoutedEventArgs {}
#[doc = "*Required features: `UI_Xaml_Input`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct NoFocusCandidateFoundEventArgs(pub ::windows::runtime::IInspectable);
impl NoFocusCandidateFoundEventArgs {
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn Direction(&self) -> ::windows::runtime::Result<FocusNavigationDirection> {
        let this = self;
        unsafe {
            let mut result__: FocusNavigationDirection = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FocusNavigationDirection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn Handled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn InputDevice(&self) -> ::windows::runtime::Result<FocusInputDeviceKind> {
        let this = self;
        unsafe {
            let mut result__: FocusInputDeviceKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FocusInputDeviceKind>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn OriginalSource(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IRoutedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for NoFocusCandidateFoundEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.NoFocusCandidateFoundEventArgs;{ec3601a7-1007-48f9-b6b3-ed0bea53937d})");
}
unsafe impl ::windows::runtime::Interface for NoFocusCandidateFoundEventArgs {
    type Vtable = INoFocusCandidateFoundEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xec3601a7_1007_48f9_b6b3_ed0bea53937d);
}
impl ::windows::runtime::RuntimeName for NoFocusCandidateFoundEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.NoFocusCandidateFoundEventArgs";
}
impl ::core::convert::From<NoFocusCandidateFoundEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: NoFocusCandidateFoundEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&NoFocusCandidateFoundEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &NoFocusCandidateFoundEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for NoFocusCandidateFoundEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a NoFocusCandidateFoundEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<NoFocusCandidateFoundEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: NoFocusCandidateFoundEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&NoFocusCandidateFoundEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &NoFocusCandidateFoundEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for NoFocusCandidateFoundEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a NoFocusCandidateFoundEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<NoFocusCandidateFoundEventArgs> for super::RoutedEventArgs {
    fn from(value: NoFocusCandidateFoundEventArgs) -> Self {
        ::core::convert::Into::<super::RoutedEventArgs>::into(&value)
    }
}
impl ::core::convert::From<&NoFocusCandidateFoundEventArgs> for super::RoutedEventArgs {
    fn from(value: &NoFocusCandidateFoundEventArgs) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::RoutedEventArgs> for NoFocusCandidateFoundEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::RoutedEventArgs> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::RoutedEventArgs> for &NoFocusCandidateFoundEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::RoutedEventArgs> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for NoFocusCandidateFoundEventArgs {}
unsafe impl ::core::marker::Sync for NoFocusCandidateFoundEventArgs {}
#[doc = "*Required features: `UI_Xaml_Input`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct Pointer(pub ::windows::runtime::IInspectable);
impl Pointer {
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn PointerId(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Devices_Input")]
    #[doc = "*Required features: `UI_Xaml_Input`, `Devices_Input`*"]
    pub fn PointerDeviceType(&self) -> ::windows::runtime::Result<super::super::super::Devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Devices::Input::PointerDeviceType = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Devices::Input::PointerDeviceType>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn IsInContact(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn IsInRange(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Pointer {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.Pointer;{5ee8f39f-747d-4171-90e6-cd37a9dffb11})");
}
unsafe impl ::windows::runtime::Interface for Pointer {
    type Vtable = IPointer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x5ee8f39f_747d_4171_90e6_cd37a9dffb11);
}
impl ::windows::runtime::RuntimeName for Pointer {
    const NAME: &'static str = "Windows.UI.Xaml.Input.Pointer";
}
impl ::core::convert::From<Pointer> for ::windows::runtime::IUnknown {
    fn from(value: Pointer) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Pointer> for ::windows::runtime::IUnknown {
    fn from(value: &Pointer) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Pointer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a Pointer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Pointer> for ::windows::runtime::IInspectable {
    fn from(value: Pointer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Pointer> for ::windows::runtime::IInspectable {
    fn from(value: &Pointer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Pointer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Pointer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for Pointer {}
unsafe impl ::core::marker::Sync for Pointer {}
#[doc = "*Required features: `UI_Xaml_Input`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PointerEventHandler(::windows::runtime::IUnknown);
impl PointerEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows::runtime::IInspectable>, &::core::option::Option<PointerRoutedEventArgs>) -> ::windows::runtime::Result<()> + 'static>(invoke: F) -> Self {
        let com = PointerEventHandler_box::<F> {
            vtable: &PointerEventHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::runtime::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn Invoke<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, PointerRoutedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).3)(::core::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PointerEventHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"delegate({e4385929-c004-4bcf-8970-359486e39f88})");
}
unsafe impl ::windows::runtime::Interface for PointerEventHandler {
    type Vtable = PointerEventHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xe4385929_c004_4bcf_8970_359486e39f88);
}
#[repr(C)]
#[doc(hidden)]
pub struct PointerEventHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr, e: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
struct PointerEventHandler_box<F: FnMut(&::core::option::Option<::windows::runtime::IInspectable>, &::core::option::Option<PointerRoutedEventArgs>) -> ::windows::runtime::Result<()> + 'static> {
    vtable: *const PointerEventHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows::runtime::IInspectable>, &::core::option::Option<PointerRoutedEventArgs>) -> ::windows::runtime::Result<()> + 'static> PointerEventHandler_box<F> {
    const VTABLE: PointerEventHandler_abi = PointerEventHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<PointerEventHandler as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::runtime::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr, e: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ((*this).invoke)(
            &*(&sender as *const <::windows::runtime::IInspectable as ::windows::runtime::Abi>::Abi as *const <::windows::runtime::IInspectable as ::windows::runtime::DefaultType>::DefaultType),
            &*(&e as *const <PointerRoutedEventArgs as ::windows::runtime::Abi>::Abi as *const <PointerRoutedEventArgs as ::windows::runtime::DefaultType>::DefaultType),
        )
        .into()
    }
}
#[doc = "*Required features: `UI_Xaml_Input`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PointerRoutedEventArgs(pub ::windows::runtime::IInspectable);
impl PointerRoutedEventArgs {
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn Pointer(&self) -> ::windows::runtime::Result<Pointer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Pointer>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `UI_Xaml_Input`, `System`*"]
    pub fn KeyModifiers(&self) -> ::windows::runtime::Result<super::super::super::System::VirtualKeyModifiers> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::System::VirtualKeyModifiers = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::VirtualKeyModifiers>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn Handled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "UI_Input")]
    #[doc = "*Required features: `UI_Xaml_Input`, `UI_Input`*"]
    pub fn GetCurrentPoint<'a, Param0: ::windows::runtime::IntoParam<'a, super::UIElement>>(&self, relativeto: Param0) -> ::windows::runtime::Result<super::super::Input::PointerPoint> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), relativeto.into_param().abi(), &mut result__).from_abi::<super::super::Input::PointerPoint>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Input"))]
    #[doc = "*Required features: `UI_Xaml_Input`, `Foundation_Collections`, `UI_Input`*"]
    pub fn GetIntermediatePoints<'a, Param0: ::windows::runtime::IntoParam<'a, super::UIElement>>(&self, relativeto: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVector<super::super::Input::PointerPoint>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), relativeto.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<super::super::Input::PointerPoint>>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn IsGenerated(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IPointerRoutedEventArgs2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn OriginalSource(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IRoutedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PointerRoutedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.PointerRoutedEventArgs;{da628f0a-9752-49e2-bde2-49eccab9194d})");
}
unsafe impl ::windows::runtime::Interface for PointerRoutedEventArgs {
    type Vtable = IPointerRoutedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xda628f0a_9752_49e2_bde2_49eccab9194d);
}
impl ::windows::runtime::RuntimeName for PointerRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.PointerRoutedEventArgs";
}
impl ::core::convert::From<PointerRoutedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: PointerRoutedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PointerRoutedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &PointerRoutedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PointerRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PointerRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PointerRoutedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: PointerRoutedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PointerRoutedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &PointerRoutedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PointerRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PointerRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<PointerRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: PointerRoutedEventArgs) -> Self {
        ::core::convert::Into::<super::RoutedEventArgs>::into(&value)
    }
}
impl ::core::convert::From<&PointerRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: &PointerRoutedEventArgs) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::RoutedEventArgs> for PointerRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::RoutedEventArgs> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::RoutedEventArgs> for &PointerRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::RoutedEventArgs> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for PointerRoutedEventArgs {}
unsafe impl ::core::marker::Sync for PointerRoutedEventArgs {}
#[doc = "*Required features: `UI_Xaml_Input`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ProcessKeyboardAcceleratorEventArgs(pub ::windows::runtime::IInspectable);
impl ProcessKeyboardAcceleratorEventArgs {
    #[cfg(feature = "System")]
    #[doc = "*Required features: `UI_Xaml_Input`, `System`*"]
    pub fn Key(&self) -> ::windows::runtime::Result<super::super::super::System::VirtualKey> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::System::VirtualKey = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::VirtualKey>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `UI_Xaml_Input`, `System`*"]
    pub fn Modifiers(&self) -> ::windows::runtime::Result<super::super::super::System::VirtualKeyModifiers> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::System::VirtualKeyModifiers = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::VirtualKeyModifiers>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn Handled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ProcessKeyboardAcceleratorEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.ProcessKeyboardAcceleratorEventArgs;{fb79c216-972b-440c-9b83-2b4198dcf09d})");
}
unsafe impl ::windows::runtime::Interface for ProcessKeyboardAcceleratorEventArgs {
    type Vtable = IProcessKeyboardAcceleratorEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xfb79c216_972b_440c_9b83_2b4198dcf09d);
}
impl ::windows::runtime::RuntimeName for ProcessKeyboardAcceleratorEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.ProcessKeyboardAcceleratorEventArgs";
}
impl ::core::convert::From<ProcessKeyboardAcceleratorEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: ProcessKeyboardAcceleratorEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ProcessKeyboardAcceleratorEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &ProcessKeyboardAcceleratorEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ProcessKeyboardAcceleratorEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ProcessKeyboardAcceleratorEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ProcessKeyboardAcceleratorEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: ProcessKeyboardAcceleratorEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ProcessKeyboardAcceleratorEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &ProcessKeyboardAcceleratorEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ProcessKeyboardAcceleratorEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ProcessKeyboardAcceleratorEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ProcessKeyboardAcceleratorEventArgs {}
unsafe impl ::core::marker::Sync for ProcessKeyboardAcceleratorEventArgs {}
#[doc = "*Required features: `UI_Xaml_Input`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RightTappedEventHandler(::windows::runtime::IUnknown);
impl RightTappedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows::runtime::IInspectable>, &::core::option::Option<RightTappedRoutedEventArgs>) -> ::windows::runtime::Result<()> + 'static>(invoke: F) -> Self {
        let com = RightTappedEventHandler_box::<F> {
            vtable: &RightTappedEventHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::runtime::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn Invoke<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, RightTappedRoutedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).3)(::core::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RightTappedEventHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"delegate({2532a062-f447-4950-9c46-f1e34a2c2238})");
}
unsafe impl ::windows::runtime::Interface for RightTappedEventHandler {
    type Vtable = RightTappedEventHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x2532a062_f447_4950_9c46_f1e34a2c2238);
}
#[repr(C)]
#[doc(hidden)]
pub struct RightTappedEventHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr, e: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
struct RightTappedEventHandler_box<F: FnMut(&::core::option::Option<::windows::runtime::IInspectable>, &::core::option::Option<RightTappedRoutedEventArgs>) -> ::windows::runtime::Result<()> + 'static> {
    vtable: *const RightTappedEventHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows::runtime::IInspectable>, &::core::option::Option<RightTappedRoutedEventArgs>) -> ::windows::runtime::Result<()> + 'static> RightTappedEventHandler_box<F> {
    const VTABLE: RightTappedEventHandler_abi = RightTappedEventHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<RightTappedEventHandler as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::runtime::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr, e: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ((*this).invoke)(
            &*(&sender as *const <::windows::runtime::IInspectable as ::windows::runtime::Abi>::Abi as *const <::windows::runtime::IInspectable as ::windows::runtime::DefaultType>::DefaultType),
            &*(&e as *const <RightTappedRoutedEventArgs as ::windows::runtime::Abi>::Abi as *const <RightTappedRoutedEventArgs as ::windows::runtime::DefaultType>::DefaultType),
        )
        .into()
    }
}
#[doc = "*Required features: `UI_Xaml_Input`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RightTappedRoutedEventArgs(pub ::windows::runtime::IInspectable);
impl RightTappedRoutedEventArgs {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<RightTappedRoutedEventArgs, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Devices_Input")]
    #[doc = "*Required features: `UI_Xaml_Input`, `Devices_Input`*"]
    pub fn PointerDeviceType(&self) -> ::windows::runtime::Result<super::super::super::Devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Devices::Input::PointerDeviceType = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Devices::Input::PointerDeviceType>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn Handled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Input`, `Foundation`*"]
    pub fn GetPosition<'a, Param0: ::windows::runtime::IntoParam<'a, super::UIElement>>(&self, relativeto: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), relativeto.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn OriginalSource(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IRoutedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RightTappedRoutedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.RightTappedRoutedEventArgs;{6834869d-7bd5-4033-b237-172f79abe393})");
}
unsafe impl ::windows::runtime::Interface for RightTappedRoutedEventArgs {
    type Vtable = IRightTappedRoutedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x6834869d_7bd5_4033_b237_172f79abe393);
}
impl ::windows::runtime::RuntimeName for RightTappedRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.RightTappedRoutedEventArgs";
}
impl ::core::convert::From<RightTappedRoutedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: RightTappedRoutedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RightTappedRoutedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &RightTappedRoutedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for RightTappedRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a RightTappedRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RightTappedRoutedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: RightTappedRoutedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RightTappedRoutedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &RightTappedRoutedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for RightTappedRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a RightTappedRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<RightTappedRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: RightTappedRoutedEventArgs) -> Self {
        ::core::convert::Into::<super::RoutedEventArgs>::into(&value)
    }
}
impl ::core::convert::From<&RightTappedRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: &RightTappedRoutedEventArgs) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::RoutedEventArgs> for RightTappedRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::RoutedEventArgs> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::RoutedEventArgs> for &RightTappedRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::RoutedEventArgs> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for RightTappedRoutedEventArgs {}
unsafe impl ::core::marker::Sync for RightTappedRoutedEventArgs {}
#[doc = "*Required features: `UI_Xaml_Input`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct StandardUICommand(pub ::windows::runtime::IInspectable);
impl StandardUICommand {
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<StandardUICommandKind> {
        let this = self;
        unsafe {
            let mut result__: StandardUICommandKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StandardUICommandKind>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn KindProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IStandardUICommandStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn new() -> ::windows::runtime::Result<StandardUICommand> {
        Self::IStandardUICommandFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::runtime::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<StandardUICommand>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn CreateInstanceWithKind(kind: StandardUICommandKind) -> ::windows::runtime::Result<StandardUICommand> {
        Self::IStandardUICommandFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), kind, ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::runtime::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<StandardUICommand>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn SetKind(&self, value: StandardUICommandKind) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IStandardUICommand2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn GetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn SetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, dp: Param0, value: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), dp.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn ClearValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), dp.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn ReadLocalValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn GetAnimationBaseValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    #[doc = "*Required features: `UI_Xaml_Input`, `UI_Core`*"]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn RegisterPropertyChangedCallback<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>, Param1: ::windows::runtime::IntoParam<'a, super::DependencyPropertyChangedCallback>>(&self, dp: Param0, callback: Param1) -> ::windows::runtime::Result<i64> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject2>(self)?;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), dp.into_param().abi(), callback.into_param().abi(), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn UnregisterPropertyChangedCallback<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(&self, dp: Param0, token: i64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), dp.into_param().abi(), token).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Input`, `Foundation`*"]
    pub fn CanExecuteChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventHandler<::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<ICommand>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Input`, `Foundation`*"]
    pub fn RemoveCanExecuteChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ICommand>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn CanExecute<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, parameter: Param0) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommand>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), parameter.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn Execute<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, parameter: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ICommand>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), parameter.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn Label(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IXamlUICommand>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn SetLabel<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXamlUICommand>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Controls")]
    #[doc = "*Required features: `UI_Xaml_Input`, `UI_Xaml_Controls`*"]
    pub fn IconSource(&self) -> ::windows::runtime::Result<super::Controls::IconSource> {
        let this = &::windows::runtime::Interface::cast::<IXamlUICommand>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Controls::IconSource>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Controls")]
    #[doc = "*Required features: `UI_Xaml_Input`, `UI_Xaml_Controls`*"]
    pub fn SetIconSource<'a, Param0: ::windows::runtime::IntoParam<'a, super::Controls::IconSource>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXamlUICommand>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Xaml_Input`, `Foundation_Collections`*"]
    pub fn KeyboardAccelerators(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVector<KeyboardAccelerator>> {
        let this = &::windows::runtime::Interface::cast::<IXamlUICommand>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<KeyboardAccelerator>>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn AccessKey(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IXamlUICommand>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn SetAccessKey<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXamlUICommand>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn Description(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IXamlUICommand>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn SetDescription<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXamlUICommand>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn Command(&self) -> ::windows::runtime::Result<ICommand> {
        let this = &::windows::runtime::Interface::cast::<IXamlUICommand>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ICommand>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn SetCommand<'a, Param0: ::windows::runtime::IntoParam<'a, ICommand>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXamlUICommand>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Input`, `Foundation`*"]
    pub fn ExecuteRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<XamlUICommand, ExecuteRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IXamlUICommand>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Input`, `Foundation`*"]
    pub fn RemoveExecuteRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXamlUICommand>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).18)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Input`, `Foundation`*"]
    pub fn CanExecuteRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<XamlUICommand, CanExecuteRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IXamlUICommand>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Input`, `Foundation`*"]
    pub fn RemoveCanExecuteRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXamlUICommand>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).20)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn NotifyCanExecuteChanged(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IXamlUICommand>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).21)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn IStandardUICommandStatics<R, F: FnOnce(&IStandardUICommandStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<StandardUICommand, IStandardUICommandStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IStandardUICommandFactory<R, F: FnOnce(&IStandardUICommandFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<StandardUICommand, IStandardUICommandFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for StandardUICommand {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.StandardUICommand;{d2bf7f43-0504-52d0-8aa6-0cb0f756eb27})");
}
unsafe impl ::windows::runtime::Interface for StandardUICommand {
    type Vtable = IStandardUICommand_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd2bf7f43_0504_52d0_8aa6_0cb0f756eb27);
}
impl ::windows::runtime::RuntimeName for StandardUICommand {
    const NAME: &'static str = "Windows.UI.Xaml.Input.StandardUICommand";
}
impl ::core::convert::From<StandardUICommand> for ::windows::runtime::IUnknown {
    fn from(value: StandardUICommand) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&StandardUICommand> for ::windows::runtime::IUnknown {
    fn from(value: &StandardUICommand) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for StandardUICommand {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a StandardUICommand {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<StandardUICommand> for ::windows::runtime::IInspectable {
    fn from(value: StandardUICommand) -> Self {
        value.0
    }
}
impl ::core::convert::From<&StandardUICommand> for ::windows::runtime::IInspectable {
    fn from(value: &StandardUICommand) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for StandardUICommand {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a StandardUICommand {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<StandardUICommand> for ICommand {
    type Error = ::windows::runtime::Error;
    fn try_from(value: StandardUICommand) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&StandardUICommand> for ICommand {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &StandardUICommand) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ICommand> for StandardUICommand {
    fn into_param(self) -> ::windows::runtime::Param<'a, ICommand> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ICommand> for &StandardUICommand {
    fn into_param(self) -> ::windows::runtime::Param<'a, ICommand> {
        ::core::convert::TryInto::<ICommand>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::From<StandardUICommand> for XamlUICommand {
    fn from(value: StandardUICommand) -> Self {
        ::core::convert::Into::<XamlUICommand>::into(&value)
    }
}
impl ::core::convert::From<&StandardUICommand> for XamlUICommand {
    fn from(value: &StandardUICommand) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, XamlUICommand> for StandardUICommand {
    fn into_param(self) -> ::windows::runtime::Param<'a, XamlUICommand> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<XamlUICommand>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, XamlUICommand> for &StandardUICommand {
    fn into_param(self) -> ::windows::runtime::Param<'a, XamlUICommand> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<XamlUICommand>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<StandardUICommand> for super::DependencyObject {
    fn from(value: StandardUICommand) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&StandardUICommand> for super::DependencyObject {
    fn from(value: &StandardUICommand) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for StandardUICommand {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for &StandardUICommand {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for StandardUICommand {}
unsafe impl ::core::marker::Sync for StandardUICommand {}
#[doc = "*Required features: `UI_Xaml_Input`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct StandardUICommandKind(pub i32);
impl StandardUICommandKind {
    pub const None: StandardUICommandKind = StandardUICommandKind(0i32);
    pub const Cut: StandardUICommandKind = StandardUICommandKind(1i32);
    pub const Copy: StandardUICommandKind = StandardUICommandKind(2i32);
    pub const Paste: StandardUICommandKind = StandardUICommandKind(3i32);
    pub const SelectAll: StandardUICommandKind = StandardUICommandKind(4i32);
    pub const Delete: StandardUICommandKind = StandardUICommandKind(5i32);
    pub const Share: StandardUICommandKind = StandardUICommandKind(6i32);
    pub const Save: StandardUICommandKind = StandardUICommandKind(7i32);
    pub const Open: StandardUICommandKind = StandardUICommandKind(8i32);
    pub const Close: StandardUICommandKind = StandardUICommandKind(9i32);
    pub const Pause: StandardUICommandKind = StandardUICommandKind(10i32);
    pub const Play: StandardUICommandKind = StandardUICommandKind(11i32);
    pub const Stop: StandardUICommandKind = StandardUICommandKind(12i32);
    pub const Forward: StandardUICommandKind = StandardUICommandKind(13i32);
    pub const Backward: StandardUICommandKind = StandardUICommandKind(14i32);
    pub const Undo: StandardUICommandKind = StandardUICommandKind(15i32);
    pub const Redo: StandardUICommandKind = StandardUICommandKind(16i32);
}
impl ::core::convert::From<i32> for StandardUICommandKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for StandardUICommandKind {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for StandardUICommandKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Input.StandardUICommandKind;i4)");
}
impl ::windows::runtime::DefaultType for StandardUICommandKind {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Xaml_Input`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct TappedEventHandler(::windows::runtime::IUnknown);
impl TappedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows::runtime::IInspectable>, &::core::option::Option<TappedRoutedEventArgs>) -> ::windows::runtime::Result<()> + 'static>(invoke: F) -> Self {
        let com = TappedEventHandler_box::<F> {
            vtable: &TappedEventHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::runtime::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn Invoke<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, TappedRoutedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).3)(::core::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for TappedEventHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"delegate({68d940cc-9ff0-49ce-b141-3f07ec477b97})");
}
unsafe impl ::windows::runtime::Interface for TappedEventHandler {
    type Vtable = TappedEventHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x68d940cc_9ff0_49ce_b141_3f07ec477b97);
}
#[repr(C)]
#[doc(hidden)]
pub struct TappedEventHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr, e: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
struct TappedEventHandler_box<F: FnMut(&::core::option::Option<::windows::runtime::IInspectable>, &::core::option::Option<TappedRoutedEventArgs>) -> ::windows::runtime::Result<()> + 'static> {
    vtable: *const TappedEventHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows::runtime::IInspectable>, &::core::option::Option<TappedRoutedEventArgs>) -> ::windows::runtime::Result<()> + 'static> TappedEventHandler_box<F> {
    const VTABLE: TappedEventHandler_abi = TappedEventHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<TappedEventHandler as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::runtime::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr, e: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ((*this).invoke)(
            &*(&sender as *const <::windows::runtime::IInspectable as ::windows::runtime::Abi>::Abi as *const <::windows::runtime::IInspectable as ::windows::runtime::DefaultType>::DefaultType),
            &*(&e as *const <TappedRoutedEventArgs as ::windows::runtime::Abi>::Abi as *const <TappedRoutedEventArgs as ::windows::runtime::DefaultType>::DefaultType),
        )
        .into()
    }
}
#[doc = "*Required features: `UI_Xaml_Input`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct TappedRoutedEventArgs(pub ::windows::runtime::IInspectable);
impl TappedRoutedEventArgs {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<TappedRoutedEventArgs, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Devices_Input")]
    #[doc = "*Required features: `UI_Xaml_Input`, `Devices_Input`*"]
    pub fn PointerDeviceType(&self) -> ::windows::runtime::Result<super::super::super::Devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Devices::Input::PointerDeviceType = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Devices::Input::PointerDeviceType>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn Handled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Input`, `Foundation`*"]
    pub fn GetPosition<'a, Param0: ::windows::runtime::IntoParam<'a, super::UIElement>>(&self, relativeto: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), relativeto.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn OriginalSource(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IRoutedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for TappedRoutedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.TappedRoutedEventArgs;{a099e6be-e624-459a-bb1d-e05c73e2cc66})");
}
unsafe impl ::windows::runtime::Interface for TappedRoutedEventArgs {
    type Vtable = ITappedRoutedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa099e6be_e624_459a_bb1d_e05c73e2cc66);
}
impl ::windows::runtime::RuntimeName for TappedRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.TappedRoutedEventArgs";
}
impl ::core::convert::From<TappedRoutedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: TappedRoutedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&TappedRoutedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &TappedRoutedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for TappedRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a TappedRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<TappedRoutedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: TappedRoutedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&TappedRoutedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &TappedRoutedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for TappedRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a TappedRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<TappedRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: TappedRoutedEventArgs) -> Self {
        ::core::convert::Into::<super::RoutedEventArgs>::into(&value)
    }
}
impl ::core::convert::From<&TappedRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: &TappedRoutedEventArgs) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::RoutedEventArgs> for TappedRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::RoutedEventArgs> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::RoutedEventArgs> for &TappedRoutedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::RoutedEventArgs> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for TappedRoutedEventArgs {}
unsafe impl ::core::marker::Sync for TappedRoutedEventArgs {}
#[doc = "*Required features: `UI_Xaml_Input`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct XYFocusKeyboardNavigationMode(pub i32);
impl XYFocusKeyboardNavigationMode {
    pub const Auto: XYFocusKeyboardNavigationMode = XYFocusKeyboardNavigationMode(0i32);
    pub const Enabled: XYFocusKeyboardNavigationMode = XYFocusKeyboardNavigationMode(1i32);
    pub const Disabled: XYFocusKeyboardNavigationMode = XYFocusKeyboardNavigationMode(2i32);
}
impl ::core::convert::From<i32> for XYFocusKeyboardNavigationMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for XYFocusKeyboardNavigationMode {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for XYFocusKeyboardNavigationMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Input.XYFocusKeyboardNavigationMode;i4)");
}
impl ::windows::runtime::DefaultType for XYFocusKeyboardNavigationMode {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Xaml_Input`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct XYFocusNavigationStrategy(pub i32);
impl XYFocusNavigationStrategy {
    pub const Auto: XYFocusNavigationStrategy = XYFocusNavigationStrategy(0i32);
    pub const Projection: XYFocusNavigationStrategy = XYFocusNavigationStrategy(1i32);
    pub const NavigationDirectionDistance: XYFocusNavigationStrategy = XYFocusNavigationStrategy(2i32);
    pub const RectilinearDistance: XYFocusNavigationStrategy = XYFocusNavigationStrategy(3i32);
}
impl ::core::convert::From<i32> for XYFocusNavigationStrategy {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for XYFocusNavigationStrategy {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for XYFocusNavigationStrategy {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Input.XYFocusNavigationStrategy;i4)");
}
impl ::windows::runtime::DefaultType for XYFocusNavigationStrategy {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Xaml_Input`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct XYFocusNavigationStrategyOverride(pub i32);
impl XYFocusNavigationStrategyOverride {
    pub const None: XYFocusNavigationStrategyOverride = XYFocusNavigationStrategyOverride(0i32);
    pub const Auto: XYFocusNavigationStrategyOverride = XYFocusNavigationStrategyOverride(1i32);
    pub const Projection: XYFocusNavigationStrategyOverride = XYFocusNavigationStrategyOverride(2i32);
    pub const NavigationDirectionDistance: XYFocusNavigationStrategyOverride = XYFocusNavigationStrategyOverride(3i32);
    pub const RectilinearDistance: XYFocusNavigationStrategyOverride = XYFocusNavigationStrategyOverride(4i32);
}
impl ::core::convert::From<i32> for XYFocusNavigationStrategyOverride {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for XYFocusNavigationStrategyOverride {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for XYFocusNavigationStrategyOverride {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Input.XYFocusNavigationStrategyOverride;i4)");
}
impl ::windows::runtime::DefaultType for XYFocusNavigationStrategyOverride {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Xaml_Input`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct XamlUICommand(pub ::windows::runtime::IInspectable);
impl XamlUICommand {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Input`, `Foundation`*"]
    pub fn CanExecuteChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventHandler<::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<ICommand>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Input`, `Foundation`*"]
    pub fn RemoveCanExecuteChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ICommand>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn CanExecute<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, parameter: Param0) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommand>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), parameter.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn Execute<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, parameter: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ICommand>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), parameter.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn Label(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn SetLabel<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Controls")]
    #[doc = "*Required features: `UI_Xaml_Input`, `UI_Xaml_Controls`*"]
    pub fn IconSource(&self) -> ::windows::runtime::Result<super::Controls::IconSource> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Controls::IconSource>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Controls")]
    #[doc = "*Required features: `UI_Xaml_Input`, `UI_Xaml_Controls`*"]
    pub fn SetIconSource<'a, Param0: ::windows::runtime::IntoParam<'a, super::Controls::IconSource>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Xaml_Input`, `Foundation_Collections`*"]
    pub fn KeyboardAccelerators(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVector<KeyboardAccelerator>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<KeyboardAccelerator>>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn AccessKey(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn SetAccessKey<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn Description(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn SetDescription<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn Command(&self) -> ::windows::runtime::Result<ICommand> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ICommand>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn SetCommand<'a, Param0: ::windows::runtime::IntoParam<'a, ICommand>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Input`, `Foundation`*"]
    pub fn ExecuteRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<XamlUICommand, ExecuteRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Input`, `Foundation`*"]
    pub fn RemoveExecuteRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).18)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Input`, `Foundation`*"]
    pub fn CanExecuteRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<XamlUICommand, CanExecuteRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Input`, `Foundation`*"]
    pub fn RemoveCanExecuteRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).20)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn NotifyCanExecuteChanged(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).21)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn LabelProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IXamlUICommandStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn IconSourceProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IXamlUICommandStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn KeyboardAcceleratorsProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IXamlUICommandStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn AccessKeyProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IXamlUICommandStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn DescriptionProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IXamlUICommandStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn CommandProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IXamlUICommandStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn new() -> ::windows::runtime::Result<XamlUICommand> {
        Self::IXamlUICommandFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::runtime::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<XamlUICommand>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn GetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn SetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, dp: Param0, value: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), dp.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn ClearValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), dp.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn ReadLocalValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn GetAnimationBaseValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    #[doc = "*Required features: `UI_Xaml_Input`, `UI_Core`*"]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn RegisterPropertyChangedCallback<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>, Param1: ::windows::runtime::IntoParam<'a, super::DependencyPropertyChangedCallback>>(&self, dp: Param0, callback: Param1) -> ::windows::runtime::Result<i64> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject2>(self)?;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), dp.into_param().abi(), callback.into_param().abi(), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Input`*"]
    pub fn UnregisterPropertyChangedCallback<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(&self, dp: Param0, token: i64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), dp.into_param().abi(), token).ok() }
    }
    pub fn IXamlUICommandStatics<R, F: FnOnce(&IXamlUICommandStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<XamlUICommand, IXamlUICommandStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IXamlUICommandFactory<R, F: FnOnce(&IXamlUICommandFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<XamlUICommand, IXamlUICommandFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for XamlUICommand {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.XamlUICommand;{8494f8d4-ead1-5f01-ad2e-a8cad4f9dc0e})");
}
unsafe impl ::windows::runtime::Interface for XamlUICommand {
    type Vtable = IXamlUICommand_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8494f8d4_ead1_5f01_ad2e_a8cad4f9dc0e);
}
impl ::windows::runtime::RuntimeName for XamlUICommand {
    const NAME: &'static str = "Windows.UI.Xaml.Input.XamlUICommand";
}
impl ::core::convert::From<XamlUICommand> for ::windows::runtime::IUnknown {
    fn from(value: XamlUICommand) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&XamlUICommand> for ::windows::runtime::IUnknown {
    fn from(value: &XamlUICommand) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for XamlUICommand {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a XamlUICommand {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<XamlUICommand> for ::windows::runtime::IInspectable {
    fn from(value: XamlUICommand) -> Self {
        value.0
    }
}
impl ::core::convert::From<&XamlUICommand> for ::windows::runtime::IInspectable {
    fn from(value: &XamlUICommand) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for XamlUICommand {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a XamlUICommand {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<XamlUICommand> for ICommand {
    type Error = ::windows::runtime::Error;
    fn try_from(value: XamlUICommand) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XamlUICommand> for ICommand {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &XamlUICommand) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ICommand> for XamlUICommand {
    fn into_param(self) -> ::windows::runtime::Param<'a, ICommand> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ICommand> for &XamlUICommand {
    fn into_param(self) -> ::windows::runtime::Param<'a, ICommand> {
        ::core::convert::TryInto::<ICommand>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::From<XamlUICommand> for super::DependencyObject {
    fn from(value: XamlUICommand) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&XamlUICommand> for super::DependencyObject {
    fn from(value: &XamlUICommand) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for XamlUICommand {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for &XamlUICommand {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for XamlUICommand {}
unsafe impl ::core::marker::Sync for XamlUICommand {}
