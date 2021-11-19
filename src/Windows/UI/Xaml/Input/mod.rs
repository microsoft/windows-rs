#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AccessKeyDisplayDismissedEventArgs(pub ::windows::core::IInspectable);
impl AccessKeyDisplayDismissedEventArgs {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AccessKeyDisplayDismissedEventArgs, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for AccessKeyDisplayDismissedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.AccessKeyDisplayDismissedEventArgs;{8a610dc6-d72d-4ca8-9f66-556f35b513da})");
}
unsafe impl ::windows::core::Interface for AccessKeyDisplayDismissedEventArgs {
    type Vtable = IAccessKeyDisplayDismissedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8a610dc6_d72d_4ca8_9f66_556f35b513da);
}
impl ::windows::core::RuntimeName for AccessKeyDisplayDismissedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.AccessKeyDisplayDismissedEventArgs";
}
impl ::core::convert::From<AccessKeyDisplayDismissedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AccessKeyDisplayDismissedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AccessKeyDisplayDismissedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AccessKeyDisplayDismissedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AccessKeyDisplayDismissedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AccessKeyDisplayDismissedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AccessKeyDisplayDismissedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AccessKeyDisplayDismissedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AccessKeyDisplayDismissedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AccessKeyDisplayDismissedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AccessKeyDisplayDismissedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AccessKeyDisplayDismissedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AccessKeyDisplayDismissedEventArgs {}
unsafe impl ::core::marker::Sync for AccessKeyDisplayDismissedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AccessKeyDisplayRequestedEventArgs(pub ::windows::core::IInspectable);
impl AccessKeyDisplayRequestedEventArgs {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AccessKeyDisplayRequestedEventArgs, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn PressedKeys(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AccessKeyDisplayRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.AccessKeyDisplayRequestedEventArgs;{0c079e55-13fe-4d03-a61d-e12f06567286})");
}
unsafe impl ::windows::core::Interface for AccessKeyDisplayRequestedEventArgs {
    type Vtable = IAccessKeyDisplayRequestedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c079e55_13fe_4d03_a61d_e12f06567286);
}
impl ::windows::core::RuntimeName for AccessKeyDisplayRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.AccessKeyDisplayRequestedEventArgs";
}
impl ::core::convert::From<AccessKeyDisplayRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AccessKeyDisplayRequestedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AccessKeyDisplayRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AccessKeyDisplayRequestedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AccessKeyDisplayRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AccessKeyDisplayRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AccessKeyDisplayRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AccessKeyDisplayRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AccessKeyDisplayRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AccessKeyDisplayRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AccessKeyDisplayRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AccessKeyDisplayRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AccessKeyDisplayRequestedEventArgs {}
unsafe impl ::core::marker::Sync for AccessKeyDisplayRequestedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AccessKeyInvokedEventArgs(pub ::windows::core::IInspectable);
impl AccessKeyInvokedEventArgs {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AccessKeyInvokedEventArgs, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
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
unsafe impl ::windows::core::RuntimeType for AccessKeyInvokedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.AccessKeyInvokedEventArgs;{cfe9cd97-c718-4091-b7dd-adf1c072b1e1})");
}
unsafe impl ::windows::core::Interface for AccessKeyInvokedEventArgs {
    type Vtable = IAccessKeyInvokedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcfe9cd97_c718_4091_b7dd_adf1c072b1e1);
}
impl ::windows::core::RuntimeName for AccessKeyInvokedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.AccessKeyInvokedEventArgs";
}
impl ::core::convert::From<AccessKeyInvokedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AccessKeyInvokedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AccessKeyInvokedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AccessKeyInvokedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AccessKeyInvokedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AccessKeyInvokedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AccessKeyInvokedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AccessKeyInvokedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AccessKeyInvokedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AccessKeyInvokedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AccessKeyInvokedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AccessKeyInvokedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AccessKeyInvokedEventArgs {}
unsafe impl ::core::marker::Sync for AccessKeyInvokedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AccessKeyManager(pub ::windows::core::IInspectable);
impl AccessKeyManager {
    pub fn IsDisplayModeEnabled() -> ::windows::core::Result<bool> {
        Self::IAccessKeyManagerStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn IsDisplayModeEnabledChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, ::windows::core::IInspectable>>>(handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        Self::IAccessKeyManagerStatics(|this| unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveIsDisplayModeEnabledChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IAccessKeyManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    pub fn ExitDisplayMode() -> ::windows::core::Result<()> {
        Self::IAccessKeyManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this)).ok() })
    }
    pub fn AreKeyTipsEnabled() -> ::windows::core::Result<bool> {
        Self::IAccessKeyManagerStatics2(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn SetAreKeyTipsEnabled(value: bool) -> ::windows::core::Result<()> {
        Self::IAccessKeyManagerStatics2(|this| unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() })
    }
    pub fn IAccessKeyManagerStatics<R, F: FnOnce(&IAccessKeyManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AccessKeyManager, IAccessKeyManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAccessKeyManagerStatics2<R, F: FnOnce(&IAccessKeyManagerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AccessKeyManager, IAccessKeyManagerStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for AccessKeyManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.AccessKeyManager;{ecc973b0-2ee9-4b1c-98d7-6e0e816d334b})");
}
unsafe impl ::windows::core::Interface for AccessKeyManager {
    type Vtable = IAccessKeyManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xecc973b0_2ee9_4b1c_98d7_6e0e816d334b);
}
impl ::windows::core::RuntimeName for AccessKeyManager {
    const NAME: &'static str = "Windows.UI.Xaml.Input.AccessKeyManager";
}
impl ::core::convert::From<AccessKeyManager> for ::windows::core::IUnknown {
    fn from(value: AccessKeyManager) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AccessKeyManager> for ::windows::core::IUnknown {
    fn from(value: &AccessKeyManager) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AccessKeyManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AccessKeyManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AccessKeyManager> for ::windows::core::IInspectable {
    fn from(value: AccessKeyManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AccessKeyManager> for ::windows::core::IInspectable {
    fn from(value: &AccessKeyManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AccessKeyManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AccessKeyManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AccessKeyManager {}
unsafe impl ::core::marker::Sync for AccessKeyManager {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CanExecuteRequestedEventArgs(pub ::windows::core::IInspectable);
impl CanExecuteRequestedEventArgs {
    pub fn Parameter(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn CanExecute(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetCanExecute(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for CanExecuteRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.CanExecuteRequestedEventArgs;{c8e75256-1950-505d-993b-75907ef96830})");
}
unsafe impl ::windows::core::Interface for CanExecuteRequestedEventArgs {
    type Vtable = ICanExecuteRequestedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc8e75256_1950_505d_993b_75907ef96830);
}
impl ::windows::core::RuntimeName for CanExecuteRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.CanExecuteRequestedEventArgs";
}
impl ::core::convert::From<CanExecuteRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: CanExecuteRequestedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CanExecuteRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CanExecuteRequestedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CanExecuteRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CanExecuteRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CanExecuteRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: CanExecuteRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CanExecuteRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CanExecuteRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CanExecuteRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CanExecuteRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CanExecuteRequestedEventArgs {}
unsafe impl ::core::marker::Sync for CanExecuteRequestedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CharacterReceivedRoutedEventArgs(pub ::windows::core::IInspectable);
impl CharacterReceivedRoutedEventArgs {
    pub fn Character(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    pub fn KeyStatus(&self) -> ::windows::core::Result<super::super::Core::CorePhysicalKeyStatus> {
        let this = self;
        unsafe {
            let mut result__: super::super::Core::CorePhysicalKeyStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Core::CorePhysicalKeyStatus>(result__)
        }
    }
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for CharacterReceivedRoutedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.CharacterReceivedRoutedEventArgs;{7849fd82-48e4-444d-9419-93ab8892c107})");
}
unsafe impl ::windows::core::Interface for CharacterReceivedRoutedEventArgs {
    type Vtable = ICharacterReceivedRoutedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7849fd82_48e4_444d_9419_93ab8892c107);
}
impl ::windows::core::RuntimeName for CharacterReceivedRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.CharacterReceivedRoutedEventArgs";
}
impl ::core::convert::From<CharacterReceivedRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: CharacterReceivedRoutedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CharacterReceivedRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CharacterReceivedRoutedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CharacterReceivedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CharacterReceivedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CharacterReceivedRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: CharacterReceivedRoutedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CharacterReceivedRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CharacterReceivedRoutedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CharacterReceivedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CharacterReceivedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<CharacterReceivedRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: CharacterReceivedRoutedEventArgs) -> Self {
        ::core::convert::Into::<super::RoutedEventArgs>::into(&value)
    }
}
impl ::core::convert::From<&CharacterReceivedRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: &CharacterReceivedRoutedEventArgs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for CharacterReceivedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for &CharacterReceivedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for CharacterReceivedRoutedEventArgs {}
unsafe impl ::core::marker::Sync for CharacterReceivedRoutedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ContextRequestedEventArgs(pub ::windows::core::IInspectable);
impl ContextRequestedEventArgs {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ContextRequestedEventArgs, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
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
    #[cfg(feature = "Foundation")]
    pub fn TryGetPosition<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(&self, relativeto: Param0, point: &mut super::super::super::Foundation::Point) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), relativeto.into_param().abi(), point, &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ContextRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.ContextRequestedEventArgs;{42618e0a-1cb6-46fb-8374-0aec68aa5e51})");
}
unsafe impl ::windows::core::Interface for ContextRequestedEventArgs {
    type Vtable = IContextRequestedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x42618e0a_1cb6_46fb_8374_0aec68aa5e51);
}
impl ::windows::core::RuntimeName for ContextRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.ContextRequestedEventArgs";
}
impl ::core::convert::From<ContextRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ContextRequestedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ContextRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ContextRequestedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ContextRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ContextRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ContextRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ContextRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ContextRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ContextRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ContextRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ContextRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ContextRequestedEventArgs> for super::RoutedEventArgs {
    fn from(value: ContextRequestedEventArgs) -> Self {
        ::core::convert::Into::<super::RoutedEventArgs>::into(&value)
    }
}
impl ::core::convert::From<&ContextRequestedEventArgs> for super::RoutedEventArgs {
    fn from(value: &ContextRequestedEventArgs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for ContextRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for &ContextRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for ContextRequestedEventArgs {}
unsafe impl ::core::marker::Sync for ContextRequestedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DoubleTappedEventHandler(::windows::core::IUnknown);
impl DoubleTappedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<DoubleTappedRoutedEventArgs>) -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = DoubleTappedEventHandler_box::<F> {
            vtable: &DoubleTappedEventHandler_box::<F>::VTABLE,
            count: ::windows::core::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, DoubleTappedRoutedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for DoubleTappedEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"delegate({3124d025-04a7-4d45-825e-8204a624dbf4})");
}
unsafe impl ::windows::core::Interface for DoubleTappedEventHandler {
    type Vtable = DoubleTappedEventHandler_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3124d025_04a7_4d45_825e_8204a624dbf4);
}
#[repr(C)]
#[doc(hidden)]
pub struct DoubleTappedEventHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sender: ::windows::core::RawPtr, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(C)]
struct DoubleTappedEventHandler_box<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<DoubleTappedRoutedEventArgs>) -> ::windows::core::Result<()> + 'static> {
    vtable: *const DoubleTappedEventHandler_abi,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<DoubleTappedRoutedEventArgs>) -> ::windows::core::Result<()> + 'static> DoubleTappedEventHandler_box<F> {
    const VTABLE: DoubleTappedEventHandler_abi = DoubleTappedEventHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<DoubleTappedEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID {
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
    unsafe extern "system" fn Invoke(this: ::windows::core::RawPtr, sender: ::windows::core::RawPtr, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(
            &*(&sender as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
            &*(&e as *const <DoubleTappedRoutedEventArgs as ::windows::core::Abi>::Abi as *const <DoubleTappedRoutedEventArgs as ::windows::core::DefaultType>::DefaultType),
        )
        .into()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DoubleTappedRoutedEventArgs(pub ::windows::core::IInspectable);
impl DoubleTappedRoutedEventArgs {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DoubleTappedRoutedEventArgs, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Devices_Input")]
    pub fn PointerDeviceType(&self) -> ::windows::core::Result<super::super::super::Devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Devices::Input::PointerDeviceType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Devices::Input::PointerDeviceType>(result__)
        }
    }
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetPosition<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(&self, relativeto: Param0) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), relativeto.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for DoubleTappedRoutedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.DoubleTappedRoutedEventArgs;{af404424-26df-44f4-8714-9359249b62d3})");
}
unsafe impl ::windows::core::Interface for DoubleTappedRoutedEventArgs {
    type Vtable = IDoubleTappedRoutedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaf404424_26df_44f4_8714_9359249b62d3);
}
impl ::windows::core::RuntimeName for DoubleTappedRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.DoubleTappedRoutedEventArgs";
}
impl ::core::convert::From<DoubleTappedRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: DoubleTappedRoutedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DoubleTappedRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &DoubleTappedRoutedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DoubleTappedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DoubleTappedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DoubleTappedRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: DoubleTappedRoutedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DoubleTappedRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &DoubleTappedRoutedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DoubleTappedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DoubleTappedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<DoubleTappedRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: DoubleTappedRoutedEventArgs) -> Self {
        ::core::convert::Into::<super::RoutedEventArgs>::into(&value)
    }
}
impl ::core::convert::From<&DoubleTappedRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: &DoubleTappedRoutedEventArgs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for DoubleTappedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for &DoubleTappedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for DoubleTappedRoutedEventArgs {}
unsafe impl ::core::marker::Sync for DoubleTappedRoutedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ExecuteRequestedEventArgs(pub ::windows::core::IInspectable);
impl ExecuteRequestedEventArgs {
    pub fn Parameter(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ExecuteRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.ExecuteRequestedEventArgs;{e07fa734-a0b6-5755-9e87-24f54cca9372})");
}
unsafe impl ::windows::core::Interface for ExecuteRequestedEventArgs {
    type Vtable = IExecuteRequestedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe07fa734_a0b6_5755_9e87_24f54cca9372);
}
impl ::windows::core::RuntimeName for ExecuteRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.ExecuteRequestedEventArgs";
}
impl ::core::convert::From<ExecuteRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ExecuteRequestedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ExecuteRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ExecuteRequestedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ExecuteRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ExecuteRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ExecuteRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ExecuteRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ExecuteRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ExecuteRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ExecuteRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ExecuteRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ExecuteRequestedEventArgs {}
unsafe impl ::core::marker::Sync for ExecuteRequestedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct FindNextElementOptions(pub ::windows::core::IInspectable);
impl FindNextElementOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<FindNextElementOptions, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn SearchRoot(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyObject>(result__)
        }
    }
    pub fn SetSearchRoot<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn ExclusionRect(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetExclusionRect<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Rect>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn HintRect(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetHintRect<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Rect>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn XYFocusNavigationStrategyOverride(&self) -> ::windows::core::Result<XYFocusNavigationStrategyOverride> {
        let this = self;
        unsafe {
            let mut result__: XYFocusNavigationStrategyOverride = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XYFocusNavigationStrategyOverride>(result__)
        }
    }
    pub fn SetXYFocusNavigationStrategyOverride(&self, value: XYFocusNavigationStrategyOverride) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for FindNextElementOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.FindNextElementOptions;{d88ae22b-46c2-41fc-897e-b5961977b89d})");
}
unsafe impl ::windows::core::Interface for FindNextElementOptions {
    type Vtable = IFindNextElementOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd88ae22b_46c2_41fc_897e_b5961977b89d);
}
impl ::windows::core::RuntimeName for FindNextElementOptions {
    const NAME: &'static str = "Windows.UI.Xaml.Input.FindNextElementOptions";
}
impl ::core::convert::From<FindNextElementOptions> for ::windows::core::IUnknown {
    fn from(value: FindNextElementOptions) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&FindNextElementOptions> for ::windows::core::IUnknown {
    fn from(value: &FindNextElementOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FindNextElementOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a FindNextElementOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<FindNextElementOptions> for ::windows::core::IInspectable {
    fn from(value: FindNextElementOptions) -> Self {
        value.0
    }
}
impl ::core::convert::From<&FindNextElementOptions> for ::windows::core::IInspectable {
    fn from(value: &FindNextElementOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FindNextElementOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a FindNextElementOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for FindNextElementOptions {}
unsafe impl ::core::marker::Sync for FindNextElementOptions {}
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
unsafe impl ::windows::core::Abi for FocusInputDeviceKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for FocusInputDeviceKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Input.FocusInputDeviceKind;i4)");
}
impl ::windows::core::DefaultType for FocusInputDeviceKind {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct FocusManager(pub ::windows::core::IInspectable);
impl FocusManager {
    pub fn GetFocusedElement() -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IFocusManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    pub fn TryMoveFocus(focusnavigationdirection: FocusNavigationDirection) -> ::windows::core::Result<bool> {
        Self::IFocusManagerStatics2(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), focusnavigationdirection, &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn FindNextFocusableElement(focusnavigationdirection: FocusNavigationDirection) -> ::windows::core::Result<super::UIElement> {
        Self::IFocusManagerStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), focusnavigationdirection, &mut result__).from_abi::<super::UIElement>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn FindNextFocusableElementWithHint<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Rect>>(focusnavigationdirection: FocusNavigationDirection, hintrect: Param1) -> ::windows::core::Result<super::UIElement> {
        Self::IFocusManagerStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), focusnavigationdirection, hintrect.into_param().abi(), &mut result__).from_abi::<super::UIElement>(result__)
        })
    }
    pub fn TryMoveFocusWithOptions<'a, Param1: ::windows::core::IntoParam<'a, FindNextElementOptions>>(focusnavigationdirection: FocusNavigationDirection, focusnavigationoptions: Param1) -> ::windows::core::Result<bool> {
        Self::IFocusManagerStatics4(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), focusnavigationdirection, focusnavigationoptions.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn FindNextElement(focusnavigationdirection: FocusNavigationDirection) -> ::windows::core::Result<super::DependencyObject> {
        Self::IFocusManagerStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), focusnavigationdirection, &mut result__).from_abi::<super::DependencyObject>(result__)
        })
    }
    pub fn FindFirstFocusableElement<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(searchscope: Param0) -> ::windows::core::Result<super::DependencyObject> {
        Self::IFocusManagerStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), searchscope.into_param().abi(), &mut result__).from_abi::<super::DependencyObject>(result__)
        })
    }
    pub fn FindLastFocusableElement<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(searchscope: Param0) -> ::windows::core::Result<super::DependencyObject> {
        Self::IFocusManagerStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), searchscope.into_param().abi(), &mut result__).from_abi::<super::DependencyObject>(result__)
        })
    }
    pub fn FindNextElementWithOptions<'a, Param1: ::windows::core::IntoParam<'a, FindNextElementOptions>>(focusnavigationdirection: FocusNavigationDirection, focusnavigationoptions: Param1) -> ::windows::core::Result<super::DependencyObject> {
        Self::IFocusManagerStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), focusnavigationdirection, focusnavigationoptions.into_param().abi(), &mut result__).from_abi::<super::DependencyObject>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn TryFocusAsync<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: super::FocusState) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<FocusMovementResult>> {
        Self::IFocusManagerStatics5(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), element.into_param().abi(), value, &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<FocusMovementResult>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn TryMoveFocusAsync(focusnavigationdirection: FocusNavigationDirection) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<FocusMovementResult>> {
        Self::IFocusManagerStatics5(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), focusnavigationdirection, &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<FocusMovementResult>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn TryMoveFocusWithOptionsAsync<'a, Param1: ::windows::core::IntoParam<'a, FindNextElementOptions>>(focusnavigationdirection: FocusNavigationDirection, focusnavigationoptions: Param1) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<FocusMovementResult>> {
        Self::IFocusManagerStatics5(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), focusnavigationdirection, focusnavigationoptions.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<FocusMovementResult>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn GotFocus<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventHandler<FocusManagerGotFocusEventArgs>>>(handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        Self::IFocusManagerStatics6(|this| unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveGotFocus<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IFocusManagerStatics6(|this| unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    pub fn LostFocus<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventHandler<FocusManagerLostFocusEventArgs>>>(handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        Self::IFocusManagerStatics6(|this| unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveLostFocus<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IFocusManagerStatics6(|this| unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    pub fn GettingFocus<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventHandler<GettingFocusEventArgs>>>(handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        Self::IFocusManagerStatics6(|this| unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveGettingFocus<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IFocusManagerStatics6(|this| unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    pub fn LosingFocus<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventHandler<LosingFocusEventArgs>>>(handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        Self::IFocusManagerStatics6(|this| unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveLosingFocus<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IFocusManagerStatics6(|this| unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    pub fn GetFocusedElement2<'a, Param0: ::windows::core::IntoParam<'a, super::XamlRoot>>(xamlroot: Param0) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IFocusManagerStatics7(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), xamlroot.into_param().abi(), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    pub fn IFocusManagerStatics<R, F: FnOnce(&IFocusManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<FocusManager, IFocusManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IFocusManagerStatics2<R, F: FnOnce(&IFocusManagerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<FocusManager, IFocusManagerStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IFocusManagerStatics3<R, F: FnOnce(&IFocusManagerStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<FocusManager, IFocusManagerStatics3> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IFocusManagerStatics4<R, F: FnOnce(&IFocusManagerStatics4) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<FocusManager, IFocusManagerStatics4> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IFocusManagerStatics5<R, F: FnOnce(&IFocusManagerStatics5) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<FocusManager, IFocusManagerStatics5> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IFocusManagerStatics6<R, F: FnOnce(&IFocusManagerStatics6) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<FocusManager, IFocusManagerStatics6> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IFocusManagerStatics7<R, F: FnOnce(&IFocusManagerStatics7) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<FocusManager, IFocusManagerStatics7> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for FocusManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.FocusManager;{c843f50b-3b83-4da1-9d6f-557c1169f341})");
}
unsafe impl ::windows::core::Interface for FocusManager {
    type Vtable = IFocusManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc843f50b_3b83_4da1_9d6f_557c1169f341);
}
impl ::windows::core::RuntimeName for FocusManager {
    const NAME: &'static str = "Windows.UI.Xaml.Input.FocusManager";
}
impl ::core::convert::From<FocusManager> for ::windows::core::IUnknown {
    fn from(value: FocusManager) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&FocusManager> for ::windows::core::IUnknown {
    fn from(value: &FocusManager) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FocusManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a FocusManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<FocusManager> for ::windows::core::IInspectable {
    fn from(value: FocusManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&FocusManager> for ::windows::core::IInspectable {
    fn from(value: &FocusManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FocusManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a FocusManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for FocusManager {}
unsafe impl ::core::marker::Sync for FocusManager {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct FocusManagerGotFocusEventArgs(pub ::windows::core::IInspectable);
impl FocusManagerGotFocusEventArgs {
    pub fn NewFocusedElement(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyObject>(result__)
        }
    }
    pub fn CorrelationId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for FocusManagerGotFocusEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.FocusManagerGotFocusEventArgs;{97aa5d83-535b-507a-868e-62b706f06b61})");
}
unsafe impl ::windows::core::Interface for FocusManagerGotFocusEventArgs {
    type Vtable = IFocusManagerGotFocusEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x97aa5d83_535b_507a_868e_62b706f06b61);
}
impl ::windows::core::RuntimeName for FocusManagerGotFocusEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.FocusManagerGotFocusEventArgs";
}
impl ::core::convert::From<FocusManagerGotFocusEventArgs> for ::windows::core::IUnknown {
    fn from(value: FocusManagerGotFocusEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&FocusManagerGotFocusEventArgs> for ::windows::core::IUnknown {
    fn from(value: &FocusManagerGotFocusEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FocusManagerGotFocusEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a FocusManagerGotFocusEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<FocusManagerGotFocusEventArgs> for ::windows::core::IInspectable {
    fn from(value: FocusManagerGotFocusEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&FocusManagerGotFocusEventArgs> for ::windows::core::IInspectable {
    fn from(value: &FocusManagerGotFocusEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FocusManagerGotFocusEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a FocusManagerGotFocusEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for FocusManagerGotFocusEventArgs {}
unsafe impl ::core::marker::Sync for FocusManagerGotFocusEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct FocusManagerLostFocusEventArgs(pub ::windows::core::IInspectable);
impl FocusManagerLostFocusEventArgs {
    pub fn OldFocusedElement(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyObject>(result__)
        }
    }
    pub fn CorrelationId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for FocusManagerLostFocusEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.FocusManagerLostFocusEventArgs;{3e157e7a-9578-5cd3-aaa8-051b3d391978})");
}
unsafe impl ::windows::core::Interface for FocusManagerLostFocusEventArgs {
    type Vtable = IFocusManagerLostFocusEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3e157e7a_9578_5cd3_aaa8_051b3d391978);
}
impl ::windows::core::RuntimeName for FocusManagerLostFocusEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.FocusManagerLostFocusEventArgs";
}
impl ::core::convert::From<FocusManagerLostFocusEventArgs> for ::windows::core::IUnknown {
    fn from(value: FocusManagerLostFocusEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&FocusManagerLostFocusEventArgs> for ::windows::core::IUnknown {
    fn from(value: &FocusManagerLostFocusEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FocusManagerLostFocusEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a FocusManagerLostFocusEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<FocusManagerLostFocusEventArgs> for ::windows::core::IInspectable {
    fn from(value: FocusManagerLostFocusEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&FocusManagerLostFocusEventArgs> for ::windows::core::IInspectable {
    fn from(value: &FocusManagerLostFocusEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FocusManagerLostFocusEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a FocusManagerLostFocusEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for FocusManagerLostFocusEventArgs {}
unsafe impl ::core::marker::Sync for FocusManagerLostFocusEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct FocusMovementResult(pub ::windows::core::IInspectable);
impl FocusMovementResult {
    pub fn Succeeded(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for FocusMovementResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.FocusMovementResult;{06dfead3-c2ae-44bb-bfab-9c73de8407a4})");
}
unsafe impl ::windows::core::Interface for FocusMovementResult {
    type Vtable = IFocusMovementResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x06dfead3_c2ae_44bb_bfab_9c73de8407a4);
}
impl ::windows::core::RuntimeName for FocusMovementResult {
    const NAME: &'static str = "Windows.UI.Xaml.Input.FocusMovementResult";
}
impl ::core::convert::From<FocusMovementResult> for ::windows::core::IUnknown {
    fn from(value: FocusMovementResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&FocusMovementResult> for ::windows::core::IUnknown {
    fn from(value: &FocusMovementResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FocusMovementResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a FocusMovementResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<FocusMovementResult> for ::windows::core::IInspectable {
    fn from(value: FocusMovementResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&FocusMovementResult> for ::windows::core::IInspectable {
    fn from(value: &FocusMovementResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FocusMovementResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a FocusMovementResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for FocusMovementResult {}
unsafe impl ::core::marker::Sync for FocusMovementResult {}
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
unsafe impl ::windows::core::Abi for FocusNavigationDirection {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for FocusNavigationDirection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Input.FocusNavigationDirection;i4)");
}
impl ::windows::core::DefaultType for FocusNavigationDirection {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GettingFocusEventArgs(pub ::windows::core::IInspectable);
impl GettingFocusEventArgs {
    pub fn OldFocusedElement(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyObject>(result__)
        }
    }
    pub fn NewFocusedElement(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyObject>(result__)
        }
    }
    pub fn SetNewFocusedElement<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn FocusState(&self) -> ::windows::core::Result<super::FocusState> {
        let this = self;
        unsafe {
            let mut result__: super::FocusState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::FocusState>(result__)
        }
    }
    pub fn Direction(&self) -> ::windows::core::Result<FocusNavigationDirection> {
        let this = self;
        unsafe {
            let mut result__: FocusNavigationDirection = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FocusNavigationDirection>(result__)
        }
    }
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn InputDevice(&self) -> ::windows::core::Result<FocusInputDeviceKind> {
        let this = self;
        unsafe {
            let mut result__: FocusInputDeviceKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FocusInputDeviceKind>(result__)
        }
    }
    pub fn Cancel(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetCancel(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn TryCancel(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IGettingFocusEventArgs2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn TrySetNewFocusedElement<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(&self, element: Param0) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IGettingFocusEventArgs2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn CorrelationId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = &::windows::core::Interface::cast::<IGettingFocusEventArgs3>(self)?;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for GettingFocusEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.GettingFocusEventArgs;{fa05b9ce-c67c-4be8-8fd4-c44d67877e0d})");
}
unsafe impl ::windows::core::Interface for GettingFocusEventArgs {
    type Vtable = IGettingFocusEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfa05b9ce_c67c_4be8_8fd4_c44d67877e0d);
}
impl ::windows::core::RuntimeName for GettingFocusEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.GettingFocusEventArgs";
}
impl ::core::convert::From<GettingFocusEventArgs> for ::windows::core::IUnknown {
    fn from(value: GettingFocusEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GettingFocusEventArgs> for ::windows::core::IUnknown {
    fn from(value: &GettingFocusEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GettingFocusEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GettingFocusEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GettingFocusEventArgs> for ::windows::core::IInspectable {
    fn from(value: GettingFocusEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GettingFocusEventArgs> for ::windows::core::IInspectable {
    fn from(value: &GettingFocusEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GettingFocusEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GettingFocusEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<GettingFocusEventArgs> for super::RoutedEventArgs {
    fn from(value: GettingFocusEventArgs) -> Self {
        ::core::convert::Into::<super::RoutedEventArgs>::into(&value)
    }
}
impl ::core::convert::From<&GettingFocusEventArgs> for super::RoutedEventArgs {
    fn from(value: &GettingFocusEventArgs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for GettingFocusEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for &GettingFocusEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for GettingFocusEventArgs {}
unsafe impl ::core::marker::Sync for GettingFocusEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct HoldingEventHandler(::windows::core::IUnknown);
impl HoldingEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<HoldingRoutedEventArgs>) -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = HoldingEventHandler_box::<F> {
            vtable: &HoldingEventHandler_box::<F>::VTABLE,
            count: ::windows::core::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, HoldingRoutedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for HoldingEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"delegate({ecae8ccd-8e5e-4fbe-9846-30a6370afcdf})");
}
unsafe impl ::windows::core::Interface for HoldingEventHandler {
    type Vtable = HoldingEventHandler_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xecae8ccd_8e5e_4fbe_9846_30a6370afcdf);
}
#[repr(C)]
#[doc(hidden)]
pub struct HoldingEventHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sender: ::windows::core::RawPtr, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(C)]
struct HoldingEventHandler_box<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<HoldingRoutedEventArgs>) -> ::windows::core::Result<()> + 'static> {
    vtable: *const HoldingEventHandler_abi,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<HoldingRoutedEventArgs>) -> ::windows::core::Result<()> + 'static> HoldingEventHandler_box<F> {
    const VTABLE: HoldingEventHandler_abi = HoldingEventHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<HoldingEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID {
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
    unsafe extern "system" fn Invoke(this: ::windows::core::RawPtr, sender: ::windows::core::RawPtr, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(
            &*(&sender as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
            &*(&e as *const <HoldingRoutedEventArgs as ::windows::core::Abi>::Abi as *const <HoldingRoutedEventArgs as ::windows::core::DefaultType>::DefaultType),
        )
        .into()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct HoldingRoutedEventArgs(pub ::windows::core::IInspectable);
impl HoldingRoutedEventArgs {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HoldingRoutedEventArgs, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Devices_Input")]
    pub fn PointerDeviceType(&self) -> ::windows::core::Result<super::super::super::Devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Devices::Input::PointerDeviceType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Devices::Input::PointerDeviceType>(result__)
        }
    }
    #[cfg(feature = "UI_Input")]
    pub fn HoldingState(&self) -> ::windows::core::Result<super::super::Input::HoldingState> {
        let this = self;
        unsafe {
            let mut result__: super::super::Input::HoldingState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Input::HoldingState>(result__)
        }
    }
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetPosition<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(&self, relativeto: Param0) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), relativeto.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for HoldingRoutedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.HoldingRoutedEventArgs;{c246ff23-d80d-44de-8db9-0d815e269ac0})");
}
unsafe impl ::windows::core::Interface for HoldingRoutedEventArgs {
    type Vtable = IHoldingRoutedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc246ff23_d80d_44de_8db9_0d815e269ac0);
}
impl ::windows::core::RuntimeName for HoldingRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.HoldingRoutedEventArgs";
}
impl ::core::convert::From<HoldingRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: HoldingRoutedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&HoldingRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &HoldingRoutedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HoldingRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a HoldingRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<HoldingRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: HoldingRoutedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&HoldingRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &HoldingRoutedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HoldingRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a HoldingRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<HoldingRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: HoldingRoutedEventArgs) -> Self {
        ::core::convert::Into::<super::RoutedEventArgs>::into(&value)
    }
}
impl ::core::convert::From<&HoldingRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: &HoldingRoutedEventArgs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for HoldingRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for &HoldingRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for HoldingRoutedEventArgs {}
unsafe impl ::core::marker::Sync for HoldingRoutedEventArgs {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IAccessKeyDisplayDismissedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAccessKeyDisplayDismissedEventArgs {
    type Vtable = IAccessKeyDisplayDismissedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8a610dc6_d72d_4ca8_9f66_556f35b513da);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessKeyDisplayDismissedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAccessKeyDisplayRequestedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAccessKeyDisplayRequestedEventArgs {
    type Vtable = IAccessKeyDisplayRequestedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c079e55_13fe_4d03_a61d_e12f06567286);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessKeyDisplayRequestedEventArgs_abi(
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
pub struct IAccessKeyInvokedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAccessKeyInvokedEventArgs {
    type Vtable = IAccessKeyInvokedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcfe9cd97_c718_4091_b7dd_adf1c072b1e1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessKeyInvokedEventArgs_abi(
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
pub struct IAccessKeyManager(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAccessKeyManager {
    type Vtable = IAccessKeyManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xecc973b0_2ee9_4b1c_98d7_6e0e816d334b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessKeyManager_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAccessKeyManagerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAccessKeyManagerStatics {
    type Vtable = IAccessKeyManagerStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4ca0efe6_d9c8_4ebc_b4c7_30d1838a81f1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessKeyManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAccessKeyManagerStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAccessKeyManagerStatics2 {
    type Vtable = IAccessKeyManagerStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x962bb594_2ab3_47c5_954b_7092f355f797);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessKeyManagerStatics2_abi(
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
pub struct ICanExecuteRequestedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICanExecuteRequestedEventArgs {
    type Vtable = ICanExecuteRequestedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc8e75256_1950_505d_993b_75907ef96830);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanExecuteRequestedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICharacterReceivedRoutedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICharacterReceivedRoutedEventArgs {
    type Vtable = ICharacterReceivedRoutedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7849fd82_48e4_444d_9419_93ab8892c107);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICharacterReceivedRoutedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Core")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Core::CorePhysicalKeyStatus) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Core"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ICommand(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICommand {
    type Vtable = ICommand_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe5af3542_ca67_4081_995b_709dd13792df);
}
impl ICommand {
    #[cfg(feature = "Foundation")]
    pub fn CanExecuteChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveCanExecuteChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn CanExecute<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, parameter: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), parameter.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn Execute<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, parameter: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), parameter.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for ICommand {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{e5af3542-ca67-4081-995b-709dd13792df}");
}
impl ::core::convert::From<ICommand> for ::windows::core::IUnknown {
    fn from(value: ICommand) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ICommand> for ::windows::core::IUnknown {
    fn from(value: &ICommand) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ICommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ICommand> for ::windows::core::IInspectable {
    fn from(value: ICommand) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICommand> for ::windows::core::IInspectable {
    fn from(value: &ICommand) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ICommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ICommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommand_abi(
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
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, parameter: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, parameter: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContextRequestedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IContextRequestedEventArgs {
    type Vtable = IContextRequestedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x42618e0a_1cb6_46fb_8374_0aec68aa5e51);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContextRequestedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, relativeto: ::windows::core::RawPtr, point: *mut super::super::super::Foundation::Point, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDoubleTappedRoutedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDoubleTappedRoutedEventArgs {
    type Vtable = IDoubleTappedRoutedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaf404424_26df_44f4_8714_9359249b62d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDoubleTappedRoutedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Input")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Input"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, relativeto: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IExecuteRequestedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IExecuteRequestedEventArgs {
    type Vtable = IExecuteRequestedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe07fa734_a0b6_5755_9e87_24f54cca9372);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExecuteRequestedEventArgs_abi(
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
pub struct IFindNextElementOptions(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFindNextElementOptions {
    type Vtable = IFindNextElementOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd88ae22b_46c2_41fc_897e_b5961977b89d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFindNextElementOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut XYFocusNavigationStrategyOverride) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: XYFocusNavigationStrategyOverride) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFocusManager(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFocusManager {
    type Vtable = IFocusManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc843f50b_3b83_4da1_9d6f_557c1169f341);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusManager_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFocusManagerGotFocusEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFocusManagerGotFocusEventArgs {
    type Vtable = IFocusManagerGotFocusEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x97aa5d83_535b_507a_868e_62b706f06b61);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusManagerGotFocusEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFocusManagerLostFocusEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFocusManagerLostFocusEventArgs {
    type Vtable = IFocusManagerLostFocusEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3e157e7a_9578_5cd3_aaa8_051b3d391978);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusManagerLostFocusEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFocusManagerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFocusManagerStatics {
    type Vtable = IFocusManagerStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1eccd326_8182_4482_826a_0918e9ed9af7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusManagerStatics_abi(
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
pub struct IFocusManagerStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFocusManagerStatics2 {
    type Vtable = IFocusManagerStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa920d761_dd87_4f31_beda_ef417fe7c04a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusManagerStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, focusnavigationdirection: FocusNavigationDirection, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFocusManagerStatics3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFocusManagerStatics3 {
    type Vtable = IFocusManagerStatics3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x60805ebf_b149_417d_83f1_baeb560e2a47);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusManagerStatics3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, focusnavigationdirection: FocusNavigationDirection, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, focusnavigationdirection: FocusNavigationDirection, hintrect: super::super::super::Foundation::Rect, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFocusManagerStatics4(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFocusManagerStatics4 {
    type Vtable = IFocusManagerStatics4_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x29276e9c_1c6c_414a_ba1c_96efd5962bcd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusManagerStatics4_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, focusnavigationdirection: FocusNavigationDirection, focusnavigationoptions: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, focusnavigationdirection: FocusNavigationDirection, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, searchscope: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, searchscope: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, focusnavigationdirection: FocusNavigationDirection, focusnavigationoptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFocusManagerStatics5(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFocusManagerStatics5 {
    type Vtable = IFocusManagerStatics5_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x280edc61_207a_4d7b_b98f_ce165e1b2015);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusManagerStatics5_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, value: super::FocusState, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, focusnavigationdirection: FocusNavigationDirection, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, focusnavigationdirection: FocusNavigationDirection, focusnavigationoptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFocusManagerStatics6(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFocusManagerStatics6 {
    type Vtable = IFocusManagerStatics6_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3546a1b6_20bf_5007_929d_e6d32e16afe4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusManagerStatics6_abi(
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFocusManagerStatics7(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFocusManagerStatics7 {
    type Vtable = IFocusManagerStatics7_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x95d6fa97_f0fc_5c32_b29d_07c04ec966b0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusManagerStatics7_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, xamlroot: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFocusMovementResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFocusMovementResult {
    type Vtable = IFocusMovementResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x06dfead3_c2ae_44bb_bfab_9c73de8407a4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusMovementResult_abi(
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
pub struct IGettingFocusEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGettingFocusEventArgs {
    type Vtable = IGettingFocusEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfa05b9ce_c67c_4be8_8fd4_c44d67877e0d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGettingFocusEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::FocusState) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut FocusNavigationDirection) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut FocusInputDeviceKind) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGettingFocusEventArgs2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGettingFocusEventArgs2 {
    type Vtable = IGettingFocusEventArgs2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x88754d7b_b4b9_4959_8bce_89bf212ed4eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGettingFocusEventArgs2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGettingFocusEventArgs3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGettingFocusEventArgs3 {
    type Vtable = IGettingFocusEventArgs3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e024891_db3f_5e78_b75a_62bfc3510735);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGettingFocusEventArgs3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHoldingRoutedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IHoldingRoutedEventArgs {
    type Vtable = IHoldingRoutedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc246ff23_d80d_44de_8db9_0d815e269ac0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHoldingRoutedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Input")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Input"))] usize,
    #[cfg(feature = "UI_Input")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Input::HoldingState) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Input"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, relativeto: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInertiaExpansionBehavior(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInertiaExpansionBehavior {
    type Vtable = IInertiaExpansionBehavior_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x751d87e5_8d42_44c5_965e_3cd30cc9d6f7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInertiaExpansionBehavior_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInertiaRotationBehavior(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInertiaRotationBehavior {
    type Vtable = IInertiaRotationBehavior_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x424cfb2e_bbfd_4625_ae78_20c65bf1efaf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInertiaRotationBehavior_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInertiaTranslationBehavior(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInertiaTranslationBehavior {
    type Vtable = IInertiaTranslationBehavior_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x45d3a512_3b32_4882_a4c2_ecfa2d4b6df0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInertiaTranslationBehavior_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInputScope(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInputScope {
    type Vtable = IInputScope_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c0f85f3_f9d8_4220_b666_045d074d9bfa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputScope_abi(
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
pub struct IInputScopeName(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInputScopeName {
    type Vtable = IInputScopeName_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfd3e6997_08fb_4cba_a021_792d7589fd5a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputScopeName_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut InputScopeNameValue) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: InputScopeNameValue) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInputScopeNameFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInputScopeNameFactory {
    type Vtable = IInputScopeNameFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4a40bb52_4bd7_4e54_8617_1cda8a1eda7f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputScopeNameFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, namevalue: InputScopeNameValue, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IKeyRoutedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IKeyRoutedEventArgs {
    type Vtable = IKeyRoutedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd4cd3dfe_4079_42e9_a39a_3095d3f049c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyRoutedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::System::VirtualKey) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "UI_Core")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Core::CorePhysicalKeyStatus) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Core"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IKeyRoutedEventArgs2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IKeyRoutedEventArgs2 {
    type Vtable = IKeyRoutedEventArgs2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1b02d57a_9634_4f14_91b2_133e42fdb3cd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyRoutedEventArgs2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::System::VirtualKey) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IKeyRoutedEventArgs3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IKeyRoutedEventArgs3 {
    type Vtable = IKeyRoutedEventArgs3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2779f5b4_ca41_411b_a8ef_f4fc78e78057);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyRoutedEventArgs3_abi(
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
pub struct IKeyboardAccelerator(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IKeyboardAccelerator {
    type Vtable = IKeyboardAccelerator_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x92e6181e_19ae_465a_9b3c_a71ee9ea7420);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyboardAccelerator_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::System::VirtualKey) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::super::System::VirtualKey) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IKeyboardAcceleratorFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IKeyboardAcceleratorFactory {
    type Vtable = IKeyboardAcceleratorFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x44d88a99_4bfd_4a47_a893_515f388623f6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyboardAcceleratorFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, baseinterface: ::windows::core::RawPtr, innerinterface: *mut ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IKeyboardAcceleratorInvokedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IKeyboardAcceleratorInvokedEventArgs {
    type Vtable = IKeyboardAcceleratorInvokedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc00b03f2_04e7_4415_b17d_d76b9490de2b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyboardAcceleratorInvokedEventArgs_abi(
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
pub struct IKeyboardAcceleratorInvokedEventArgs2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IKeyboardAcceleratorInvokedEventArgs2 {
    type Vtable = IKeyboardAcceleratorInvokedEventArgs2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbefca4b8_5907_48ee_8e21_9c969078fa11);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyboardAcceleratorInvokedEventArgs2_abi(
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
pub struct IKeyboardAcceleratorStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IKeyboardAcceleratorStatics {
    type Vtable = IKeyboardAcceleratorStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3bd43d51_9bb3_456d_bf15_804adfb86261);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyboardAcceleratorStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILosingFocusEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILosingFocusEventArgs {
    type Vtable = ILosingFocusEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf9f683c7_d789_472b_aa93_6d4105e6dabe);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILosingFocusEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::FocusState) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut FocusNavigationDirection) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut FocusInputDeviceKind) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILosingFocusEventArgs2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILosingFocusEventArgs2 {
    type Vtable = ILosingFocusEventArgs2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0493fad9_c27f_469f_8e62_52b3a4f7cd54);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILosingFocusEventArgs2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILosingFocusEventArgs3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILosingFocusEventArgs3 {
    type Vtable = ILosingFocusEventArgs3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc98900bd_0b79_566e_ad1f_436fa513ae22);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILosingFocusEventArgs3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IManipulationCompletedRoutedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IManipulationCompletedRoutedEventArgs {
    type Vtable = IManipulationCompletedRoutedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5ad9b23_2f41_498e_8319_015ee8a75346);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationCompletedRoutedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "UI_Input"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Input::ManipulationDelta) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Input")))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Input"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Input::ManipulationVelocities) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Input")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Input")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Input"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IManipulationDeltaRoutedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IManipulationDeltaRoutedEventArgs {
    type Vtable = IManipulationDeltaRoutedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x400d5794_4c6f_491d_82d6_3517109399c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationDeltaRoutedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "UI_Input"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Input::ManipulationDelta) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Input")))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Input"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Input::ManipulationDelta) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Input")))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Input"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Input::ManipulationVelocities) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Input")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Input")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Input"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IManipulationInertiaStartingRoutedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IManipulationInertiaStartingRoutedEventArgs {
    type Vtable = IManipulationInertiaStartingRoutedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x246a91a9_ca43_4c0b_acef_81e8b8147520);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationInertiaStartingRoutedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Input")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Input"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Input"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Input::ManipulationDelta) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Input")))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Input"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Input::ManipulationDelta) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Input")))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Input"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Input::ManipulationVelocities) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Input")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IManipulationPivot(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IManipulationPivot {
    type Vtable = IManipulationPivot_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2e3838a5_e6c2_4998_82ac_18748b141666);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationPivot_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IManipulationPivotFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IManipulationPivotFactory {
    type Vtable = IManipulationPivotFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d05b039_3702_4396_ad9b_a825efa63a3b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationPivotFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, center: super::super::super::Foundation::Point, radius: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IManipulationStartedRoutedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IManipulationStartedRoutedEventArgs {
    type Vtable = IManipulationStartedRoutedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5db1aa05_9f80_48b6_ae6c_4f119de8ff13);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationStartedRoutedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Input")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Input"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Input"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Input::ManipulationDelta) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Input")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IManipulationStartedRoutedEventArgsFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IManipulationStartedRoutedEventArgsFactory {
    type Vtable = IManipulationStartedRoutedEventArgsFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x84c1daa7_7272_4463_b6c3_a40b9ba151fc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationStartedRoutedEventArgsFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, baseinterface: ::windows::core::RawPtr, innerinterface: *mut ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IManipulationStartingRoutedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IManipulationStartingRoutedEventArgs {
    type Vtable = IManipulationStartingRoutedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x18d636b7_53a4_4c15_a498_f3a9ca212a42);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationStartingRoutedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ManipulationModes) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ManipulationModes) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct INoFocusCandidateFoundEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for INoFocusCandidateFoundEventArgs {
    type Vtable = INoFocusCandidateFoundEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xec3601a7_1007_48f9_b6b3_ed0bea53937d);
}
#[repr(C)]
#[doc(hidden)]
pub struct INoFocusCandidateFoundEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut FocusNavigationDirection) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut FocusInputDeviceKind) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPointer(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPointer {
    type Vtable = IPointer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5ee8f39f_747d_4171_90e6_cd37a9dffb11);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointer_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Input")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Input"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPointerRoutedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPointerRoutedEventArgs {
    type Vtable = IPointerRoutedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xda628f0a_9752_49e2_bde2_49eccab9194d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerRoutedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Input")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, relativeto: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Input"))] usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Input"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, relativeto: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Input")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPointerRoutedEventArgs2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPointerRoutedEventArgs2 {
    type Vtable = IPointerRoutedEventArgs2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0821f294_1de6_4711_ba7c_8d4b8b0911d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerRoutedEventArgs2_abi(
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
pub struct IProcessKeyboardAcceleratorEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IProcessKeyboardAcceleratorEventArgs {
    type Vtable = IProcessKeyboardAcceleratorEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb79c216_972b_440c_9b83_2b4198dcf09d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessKeyboardAcceleratorEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::System::VirtualKey) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRightTappedRoutedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IRightTappedRoutedEventArgs {
    type Vtable = IRightTappedRoutedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6834869d_7bd5_4033_b237_172f79abe393);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRightTappedRoutedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Input")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Input"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, relativeto: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IStandardUICommand(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStandardUICommand {
    type Vtable = IStandardUICommand_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd2bf7f43_0504_52d0_8aa6_0cb0f756eb27);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStandardUICommand_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut StandardUICommandKind) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IStandardUICommand2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStandardUICommand2 {
    type Vtable = IStandardUICommand2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe3666069_f9e4_51eb_885b_7a620a0782ea);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStandardUICommand2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: StandardUICommandKind) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IStandardUICommandFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStandardUICommandFactory {
    type Vtable = IStandardUICommandFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f1a7590_dce1_56e4_ab63_f5ce3ce4ebf6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStandardUICommandFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, baseinterface: ::windows::core::RawPtr, innerinterface: *mut ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, kind: StandardUICommandKind, baseinterface: ::windows::core::RawPtr, innerinterface: *mut ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IStandardUICommandStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStandardUICommandStatics {
    type Vtable = IStandardUICommandStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7ea87ed9_2978_5533_9b2e_6759ce88569f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStandardUICommandStatics_abi(
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
pub struct ITappedRoutedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITappedRoutedEventArgs {
    type Vtable = ITappedRoutedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa099e6be_e624_459a_bb1d_e05c73e2cc66);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITappedRoutedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Input")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Input"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, relativeto: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXamlUICommand(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXamlUICommand {
    type Vtable = IXamlUICommand_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8494f8d4_ead1_5f01_ad2e_a8cad4f9dc0e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlUICommand_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Controls")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))] usize,
    #[cfg(feature = "UI_Xaml_Controls")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXamlUICommandFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXamlUICommandFactory {
    type Vtable = IXamlUICommandFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1eec08c3_e061_5e10_9f2a_2baa840885c2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlUICommandFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, baseinterface: ::windows::core::RawPtr, innerinterface: *mut ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXamlUICommandStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXamlUICommandStatics {
    type Vtable = IXamlUICommandStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x66bc457c_1a0c_58ed_876e_71533f966db6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlUICommandStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct InertiaExpansionBehavior(pub ::windows::core::IInspectable);
impl InertiaExpansionBehavior {
    pub fn DesiredDeceleration(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetDesiredDeceleration(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn DesiredExpansion(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetDesiredExpansion(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for InertiaExpansionBehavior {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.InertiaExpansionBehavior;{751d87e5-8d42-44c5-965e-3cd30cc9d6f7})");
}
unsafe impl ::windows::core::Interface for InertiaExpansionBehavior {
    type Vtable = IInertiaExpansionBehavior_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x751d87e5_8d42_44c5_965e_3cd30cc9d6f7);
}
impl ::windows::core::RuntimeName for InertiaExpansionBehavior {
    const NAME: &'static str = "Windows.UI.Xaml.Input.InertiaExpansionBehavior";
}
impl ::core::convert::From<InertiaExpansionBehavior> for ::windows::core::IUnknown {
    fn from(value: InertiaExpansionBehavior) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InertiaExpansionBehavior> for ::windows::core::IUnknown {
    fn from(value: &InertiaExpansionBehavior) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InertiaExpansionBehavior {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InertiaExpansionBehavior {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InertiaExpansionBehavior> for ::windows::core::IInspectable {
    fn from(value: InertiaExpansionBehavior) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InertiaExpansionBehavior> for ::windows::core::IInspectable {
    fn from(value: &InertiaExpansionBehavior) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InertiaExpansionBehavior {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InertiaExpansionBehavior {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for InertiaExpansionBehavior {}
unsafe impl ::core::marker::Sync for InertiaExpansionBehavior {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct InertiaRotationBehavior(pub ::windows::core::IInspectable);
impl InertiaRotationBehavior {
    pub fn DesiredDeceleration(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetDesiredDeceleration(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn DesiredRotation(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetDesiredRotation(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for InertiaRotationBehavior {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.InertiaRotationBehavior;{424cfb2e-bbfd-4625-ae78-20c65bf1efaf})");
}
unsafe impl ::windows::core::Interface for InertiaRotationBehavior {
    type Vtable = IInertiaRotationBehavior_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x424cfb2e_bbfd_4625_ae78_20c65bf1efaf);
}
impl ::windows::core::RuntimeName for InertiaRotationBehavior {
    const NAME: &'static str = "Windows.UI.Xaml.Input.InertiaRotationBehavior";
}
impl ::core::convert::From<InertiaRotationBehavior> for ::windows::core::IUnknown {
    fn from(value: InertiaRotationBehavior) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InertiaRotationBehavior> for ::windows::core::IUnknown {
    fn from(value: &InertiaRotationBehavior) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InertiaRotationBehavior {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InertiaRotationBehavior {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InertiaRotationBehavior> for ::windows::core::IInspectable {
    fn from(value: InertiaRotationBehavior) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InertiaRotationBehavior> for ::windows::core::IInspectable {
    fn from(value: &InertiaRotationBehavior) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InertiaRotationBehavior {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InertiaRotationBehavior {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for InertiaRotationBehavior {}
unsafe impl ::core::marker::Sync for InertiaRotationBehavior {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct InertiaTranslationBehavior(pub ::windows::core::IInspectable);
impl InertiaTranslationBehavior {
    pub fn DesiredDeceleration(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetDesiredDeceleration(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn DesiredDisplacement(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetDesiredDisplacement(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for InertiaTranslationBehavior {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.InertiaTranslationBehavior;{45d3a512-3b32-4882-a4c2-ecfa2d4b6df0})");
}
unsafe impl ::windows::core::Interface for InertiaTranslationBehavior {
    type Vtable = IInertiaTranslationBehavior_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x45d3a512_3b32_4882_a4c2_ecfa2d4b6df0);
}
impl ::windows::core::RuntimeName for InertiaTranslationBehavior {
    const NAME: &'static str = "Windows.UI.Xaml.Input.InertiaTranslationBehavior";
}
impl ::core::convert::From<InertiaTranslationBehavior> for ::windows::core::IUnknown {
    fn from(value: InertiaTranslationBehavior) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InertiaTranslationBehavior> for ::windows::core::IUnknown {
    fn from(value: &InertiaTranslationBehavior) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InertiaTranslationBehavior {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InertiaTranslationBehavior {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InertiaTranslationBehavior> for ::windows::core::IInspectable {
    fn from(value: InertiaTranslationBehavior) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InertiaTranslationBehavior> for ::windows::core::IInspectable {
    fn from(value: &InertiaTranslationBehavior) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InertiaTranslationBehavior {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InertiaTranslationBehavior {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for InertiaTranslationBehavior {}
unsafe impl ::core::marker::Sync for InertiaTranslationBehavior {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct InputScope(pub ::windows::core::IInspectable);
impl InputScope {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<InputScope, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Names(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<InputScopeName>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<InputScopeName>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for InputScope {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.InputScope;{5c0f85f3-f9d8-4220-b666-045d074d9bfa})");
}
unsafe impl ::windows::core::Interface for InputScope {
    type Vtable = IInputScope_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c0f85f3_f9d8_4220_b666_045d074d9bfa);
}
impl ::windows::core::RuntimeName for InputScope {
    const NAME: &'static str = "Windows.UI.Xaml.Input.InputScope";
}
impl ::core::convert::From<InputScope> for ::windows::core::IUnknown {
    fn from(value: InputScope) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InputScope> for ::windows::core::IUnknown {
    fn from(value: &InputScope) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InputScope {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InputScope {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InputScope> for ::windows::core::IInspectable {
    fn from(value: InputScope) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InputScope> for ::windows::core::IInspectable {
    fn from(value: &InputScope) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InputScope {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InputScope {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<InputScope> for super::DependencyObject {
    fn from(value: InputScope) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&InputScope> for super::DependencyObject {
    fn from(value: &InputScope) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for InputScope {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &InputScope {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for InputScope {}
unsafe impl ::core::marker::Sync for InputScope {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct InputScopeName(pub ::windows::core::IInspectable);
impl InputScopeName {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<InputScopeName, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn NameValue(&self) -> ::windows::core::Result<InputScopeNameValue> {
        let this = self;
        unsafe {
            let mut result__: InputScopeNameValue = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InputScopeNameValue>(result__)
        }
    }
    pub fn SetNameValue(&self, value: InputScopeNameValue) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn CreateInstance(namevalue: InputScopeNameValue) -> ::windows::core::Result<InputScopeName> {
        Self::IInputScopeNameFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), namevalue, &mut result__).from_abi::<InputScopeName>(result__)
        })
    }
    pub fn IInputScopeNameFactory<R, F: FnOnce(&IInputScopeNameFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<InputScopeName, IInputScopeNameFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for InputScopeName {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.InputScopeName;{fd3e6997-08fb-4cba-a021-792d7589fd5a})");
}
unsafe impl ::windows::core::Interface for InputScopeName {
    type Vtable = IInputScopeName_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfd3e6997_08fb_4cba_a021_792d7589fd5a);
}
impl ::windows::core::RuntimeName for InputScopeName {
    const NAME: &'static str = "Windows.UI.Xaml.Input.InputScopeName";
}
impl ::core::convert::From<InputScopeName> for ::windows::core::IUnknown {
    fn from(value: InputScopeName) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InputScopeName> for ::windows::core::IUnknown {
    fn from(value: &InputScopeName) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InputScopeName {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InputScopeName {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InputScopeName> for ::windows::core::IInspectable {
    fn from(value: InputScopeName) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InputScopeName> for ::windows::core::IInspectable {
    fn from(value: &InputScopeName) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InputScopeName {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InputScopeName {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<InputScopeName> for super::DependencyObject {
    fn from(value: InputScopeName) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&InputScopeName> for super::DependencyObject {
    fn from(value: &InputScopeName) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for InputScopeName {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &InputScopeName {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for InputScopeName {}
unsafe impl ::core::marker::Sync for InputScopeName {}
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
unsafe impl ::windows::core::Abi for InputScopeNameValue {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for InputScopeNameValue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Input.InputScopeNameValue;i4)");
}
impl ::windows::core::DefaultType for InputScopeNameValue {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct KeyEventHandler(::windows::core::IUnknown);
impl KeyEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<KeyRoutedEventArgs>) -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = KeyEventHandler_box::<F> {
            vtable: &KeyEventHandler_box::<F>::VTABLE,
            count: ::windows::core::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, KeyRoutedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for KeyEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"delegate({7c63d2e5-7a0e-4e12-b96a-7715aa6ff1c8})");
}
unsafe impl ::windows::core::Interface for KeyEventHandler {
    type Vtable = KeyEventHandler_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7c63d2e5_7a0e_4e12_b96a_7715aa6ff1c8);
}
#[repr(C)]
#[doc(hidden)]
pub struct KeyEventHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sender: ::windows::core::RawPtr, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(C)]
struct KeyEventHandler_box<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<KeyRoutedEventArgs>) -> ::windows::core::Result<()> + 'static> {
    vtable: *const KeyEventHandler_abi,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<KeyRoutedEventArgs>) -> ::windows::core::Result<()> + 'static> KeyEventHandler_box<F> {
    const VTABLE: KeyEventHandler_abi = KeyEventHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<KeyEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID {
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
    unsafe extern "system" fn Invoke(this: ::windows::core::RawPtr, sender: ::windows::core::RawPtr, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(
            &*(&sender as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
            &*(&e as *const <KeyRoutedEventArgs as ::windows::core::Abi>::Abi as *const <KeyRoutedEventArgs as ::windows::core::DefaultType>::DefaultType),
        )
        .into()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct KeyRoutedEventArgs(pub ::windows::core::IInspectable);
impl KeyRoutedEventArgs {
    #[cfg(feature = "System")]
    pub fn Key(&self) -> ::windows::core::Result<super::super::super::System::VirtualKey> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::System::VirtualKey = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::VirtualKey>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    pub fn KeyStatus(&self) -> ::windows::core::Result<super::super::Core::CorePhysicalKeyStatus> {
        let this = self;
        unsafe {
            let mut result__: super::super::Core::CorePhysicalKeyStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Core::CorePhysicalKeyStatus>(result__)
        }
    }
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "System")]
    pub fn OriginalKey(&self) -> ::windows::core::Result<super::super::super::System::VirtualKey> {
        let this = &::windows::core::Interface::cast::<IKeyRoutedEventArgs2>(self)?;
        unsafe {
            let mut result__: super::super::super::System::VirtualKey = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::VirtualKey>(result__)
        }
    }
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IKeyRoutedEventArgs3>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for KeyRoutedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.KeyRoutedEventArgs;{d4cd3dfe-4079-42e9-a39a-3095d3f049c6})");
}
unsafe impl ::windows::core::Interface for KeyRoutedEventArgs {
    type Vtable = IKeyRoutedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd4cd3dfe_4079_42e9_a39a_3095d3f049c6);
}
impl ::windows::core::RuntimeName for KeyRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.KeyRoutedEventArgs";
}
impl ::core::convert::From<KeyRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: KeyRoutedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&KeyRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &KeyRoutedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for KeyRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a KeyRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<KeyRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: KeyRoutedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&KeyRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &KeyRoutedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for KeyRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a KeyRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<KeyRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: KeyRoutedEventArgs) -> Self {
        ::core::convert::Into::<super::RoutedEventArgs>::into(&value)
    }
}
impl ::core::convert::From<&KeyRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: &KeyRoutedEventArgs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for KeyRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for &KeyRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for KeyRoutedEventArgs {}
unsafe impl ::core::marker::Sync for KeyRoutedEventArgs {}
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
unsafe impl ::windows::core::Abi for KeyTipPlacementMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for KeyTipPlacementMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Input.KeyTipPlacementMode;i4)");
}
impl ::windows::core::DefaultType for KeyTipPlacementMode {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct KeyboardAccelerator(pub ::windows::core::IInspectable);
impl KeyboardAccelerator {
    #[cfg(feature = "System")]
    pub fn Key(&self) -> ::windows::core::Result<super::super::super::System::VirtualKey> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::System::VirtualKey = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::VirtualKey>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn SetKey(&self, value: super::super::super::System::VirtualKey) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "System")]
    pub fn Modifiers(&self) -> ::windows::core::Result<super::super::super::System::VirtualKeyModifiers> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::System::VirtualKeyModifiers = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::VirtualKeyModifiers>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn SetModifiers(&self, value: super::super::super::System::VirtualKeyModifiers) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ScopeOwner(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyObject>(result__)
        }
    }
    pub fn SetScopeOwner<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Invoked<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<KeyboardAccelerator, KeyboardAcceleratorInvokedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveInvoked<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn KeyProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IKeyboardAcceleratorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn ModifiersProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IKeyboardAcceleratorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IsEnabledProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IKeyboardAcceleratorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn ScopeOwnerProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IKeyboardAcceleratorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn new() -> ::windows::core::Result<KeyboardAccelerator> {
        Self::IKeyboardAcceleratorFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<KeyboardAccelerator>(result__)
        })
    }
    pub fn IKeyboardAcceleratorStatics<R, F: FnOnce(&IKeyboardAcceleratorStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<KeyboardAccelerator, IKeyboardAcceleratorStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IKeyboardAcceleratorFactory<R, F: FnOnce(&IKeyboardAcceleratorFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<KeyboardAccelerator, IKeyboardAcceleratorFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for KeyboardAccelerator {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.KeyboardAccelerator;{92e6181e-19ae-465a-9b3c-a71ee9ea7420})");
}
unsafe impl ::windows::core::Interface for KeyboardAccelerator {
    type Vtable = IKeyboardAccelerator_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x92e6181e_19ae_465a_9b3c_a71ee9ea7420);
}
impl ::windows::core::RuntimeName for KeyboardAccelerator {
    const NAME: &'static str = "Windows.UI.Xaml.Input.KeyboardAccelerator";
}
impl ::core::convert::From<KeyboardAccelerator> for ::windows::core::IUnknown {
    fn from(value: KeyboardAccelerator) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&KeyboardAccelerator> for ::windows::core::IUnknown {
    fn from(value: &KeyboardAccelerator) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for KeyboardAccelerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a KeyboardAccelerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<KeyboardAccelerator> for ::windows::core::IInspectable {
    fn from(value: KeyboardAccelerator) -> Self {
        value.0
    }
}
impl ::core::convert::From<&KeyboardAccelerator> for ::windows::core::IInspectable {
    fn from(value: &KeyboardAccelerator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for KeyboardAccelerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a KeyboardAccelerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<KeyboardAccelerator> for super::DependencyObject {
    fn from(value: KeyboardAccelerator) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&KeyboardAccelerator> for super::DependencyObject {
    fn from(value: &KeyboardAccelerator) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for KeyboardAccelerator {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &KeyboardAccelerator {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for KeyboardAccelerator {}
unsafe impl ::core::marker::Sync for KeyboardAccelerator {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct KeyboardAcceleratorInvokedEventArgs(pub ::windows::core::IInspectable);
impl KeyboardAcceleratorInvokedEventArgs {
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
    pub fn Element(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyObject>(result__)
        }
    }
    pub fn KeyboardAccelerator(&self) -> ::windows::core::Result<KeyboardAccelerator> {
        let this = &::windows::core::Interface::cast::<IKeyboardAcceleratorInvokedEventArgs2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<KeyboardAccelerator>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for KeyboardAcceleratorInvokedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.KeyboardAcceleratorInvokedEventArgs;{c00b03f2-04e7-4415-b17d-d76b9490de2b})");
}
unsafe impl ::windows::core::Interface for KeyboardAcceleratorInvokedEventArgs {
    type Vtable = IKeyboardAcceleratorInvokedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc00b03f2_04e7_4415_b17d_d76b9490de2b);
}
impl ::windows::core::RuntimeName for KeyboardAcceleratorInvokedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.KeyboardAcceleratorInvokedEventArgs";
}
impl ::core::convert::From<KeyboardAcceleratorInvokedEventArgs> for ::windows::core::IUnknown {
    fn from(value: KeyboardAcceleratorInvokedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&KeyboardAcceleratorInvokedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &KeyboardAcceleratorInvokedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for KeyboardAcceleratorInvokedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a KeyboardAcceleratorInvokedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<KeyboardAcceleratorInvokedEventArgs> for ::windows::core::IInspectable {
    fn from(value: KeyboardAcceleratorInvokedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&KeyboardAcceleratorInvokedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &KeyboardAcceleratorInvokedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for KeyboardAcceleratorInvokedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a KeyboardAcceleratorInvokedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for KeyboardAcceleratorInvokedEventArgs {}
unsafe impl ::core::marker::Sync for KeyboardAcceleratorInvokedEventArgs {}
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
unsafe impl ::windows::core::Abi for KeyboardAcceleratorPlacementMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for KeyboardAcceleratorPlacementMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Input.KeyboardAcceleratorPlacementMode;i4)");
}
impl ::windows::core::DefaultType for KeyboardAcceleratorPlacementMode {
    type DefaultType = Self;
}
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
unsafe impl ::windows::core::Abi for KeyboardNavigationMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for KeyboardNavigationMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Input.KeyboardNavigationMode;i4)");
}
impl ::windows::core::DefaultType for KeyboardNavigationMode {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LosingFocusEventArgs(pub ::windows::core::IInspectable);
impl LosingFocusEventArgs {
    pub fn OldFocusedElement(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyObject>(result__)
        }
    }
    pub fn NewFocusedElement(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyObject>(result__)
        }
    }
    pub fn SetNewFocusedElement<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn FocusState(&self) -> ::windows::core::Result<super::FocusState> {
        let this = self;
        unsafe {
            let mut result__: super::FocusState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::FocusState>(result__)
        }
    }
    pub fn Direction(&self) -> ::windows::core::Result<FocusNavigationDirection> {
        let this = self;
        unsafe {
            let mut result__: FocusNavigationDirection = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FocusNavigationDirection>(result__)
        }
    }
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn InputDevice(&self) -> ::windows::core::Result<FocusInputDeviceKind> {
        let this = self;
        unsafe {
            let mut result__: FocusInputDeviceKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FocusInputDeviceKind>(result__)
        }
    }
    pub fn Cancel(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetCancel(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn TryCancel(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ILosingFocusEventArgs2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn TrySetNewFocusedElement<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(&self, element: Param0) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ILosingFocusEventArgs2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn CorrelationId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = &::windows::core::Interface::cast::<ILosingFocusEventArgs3>(self)?;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for LosingFocusEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.LosingFocusEventArgs;{f9f683c7-d789-472b-aa93-6d4105e6dabe})");
}
unsafe impl ::windows::core::Interface for LosingFocusEventArgs {
    type Vtable = ILosingFocusEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf9f683c7_d789_472b_aa93_6d4105e6dabe);
}
impl ::windows::core::RuntimeName for LosingFocusEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.LosingFocusEventArgs";
}
impl ::core::convert::From<LosingFocusEventArgs> for ::windows::core::IUnknown {
    fn from(value: LosingFocusEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LosingFocusEventArgs> for ::windows::core::IUnknown {
    fn from(value: &LosingFocusEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LosingFocusEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LosingFocusEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LosingFocusEventArgs> for ::windows::core::IInspectable {
    fn from(value: LosingFocusEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LosingFocusEventArgs> for ::windows::core::IInspectable {
    fn from(value: &LosingFocusEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LosingFocusEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LosingFocusEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<LosingFocusEventArgs> for super::RoutedEventArgs {
    fn from(value: LosingFocusEventArgs) -> Self {
        ::core::convert::Into::<super::RoutedEventArgs>::into(&value)
    }
}
impl ::core::convert::From<&LosingFocusEventArgs> for super::RoutedEventArgs {
    fn from(value: &LosingFocusEventArgs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for LosingFocusEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for &LosingFocusEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for LosingFocusEventArgs {}
unsafe impl ::core::marker::Sync for LosingFocusEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ManipulationCompletedEventHandler(::windows::core::IUnknown);
impl ManipulationCompletedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<ManipulationCompletedRoutedEventArgs>) -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = ManipulationCompletedEventHandler_box::<F> {
            vtable: &ManipulationCompletedEventHandler_box::<F>::VTABLE,
            count: ::windows::core::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, ManipulationCompletedRoutedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for ManipulationCompletedEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"delegate({38ef4b0f-14f8-42df-9a1e-a4bcc4af77f4})");
}
unsafe impl ::windows::core::Interface for ManipulationCompletedEventHandler {
    type Vtable = ManipulationCompletedEventHandler_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38ef4b0f_14f8_42df_9a1e_a4bcc4af77f4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ManipulationCompletedEventHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sender: ::windows::core::RawPtr, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(C)]
struct ManipulationCompletedEventHandler_box<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<ManipulationCompletedRoutedEventArgs>) -> ::windows::core::Result<()> + 'static> {
    vtable: *const ManipulationCompletedEventHandler_abi,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<ManipulationCompletedRoutedEventArgs>) -> ::windows::core::Result<()> + 'static> ManipulationCompletedEventHandler_box<F> {
    const VTABLE: ManipulationCompletedEventHandler_abi = ManipulationCompletedEventHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<ManipulationCompletedEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID {
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
    unsafe extern "system" fn Invoke(this: ::windows::core::RawPtr, sender: ::windows::core::RawPtr, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(
            &*(&sender as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
            &*(&e as *const <ManipulationCompletedRoutedEventArgs as ::windows::core::Abi>::Abi as *const <ManipulationCompletedRoutedEventArgs as ::windows::core::DefaultType>::DefaultType),
        )
        .into()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ManipulationCompletedRoutedEventArgs(pub ::windows::core::IInspectable);
impl ManipulationCompletedRoutedEventArgs {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ManipulationCompletedRoutedEventArgs, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Container(&self) -> ::windows::core::Result<super::UIElement> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::UIElement>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
    pub fn IsInertial(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Input"))]
    pub fn Cumulative(&self) -> ::windows::core::Result<super::super::Input::ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__: super::super::Input::ManipulationDelta = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Input::ManipulationDelta>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Input"))]
    pub fn Velocities(&self) -> ::windows::core::Result<super::super::Input::ManipulationVelocities> {
        let this = self;
        unsafe {
            let mut result__: super::super::Input::ManipulationVelocities = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Input::ManipulationVelocities>(result__)
        }
    }
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Devices_Input")]
    pub fn PointerDeviceType(&self) -> ::windows::core::Result<super::super::super::Devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Devices::Input::PointerDeviceType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Devices::Input::PointerDeviceType>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ManipulationCompletedRoutedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.ManipulationCompletedRoutedEventArgs;{b5ad9b23-2f41-498e-8319-015ee8a75346})");
}
unsafe impl ::windows::core::Interface for ManipulationCompletedRoutedEventArgs {
    type Vtable = IManipulationCompletedRoutedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5ad9b23_2f41_498e_8319_015ee8a75346);
}
impl ::windows::core::RuntimeName for ManipulationCompletedRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.ManipulationCompletedRoutedEventArgs";
}
impl ::core::convert::From<ManipulationCompletedRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ManipulationCompletedRoutedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ManipulationCompletedRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ManipulationCompletedRoutedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ManipulationCompletedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ManipulationCompletedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ManipulationCompletedRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ManipulationCompletedRoutedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ManipulationCompletedRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ManipulationCompletedRoutedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ManipulationCompletedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ManipulationCompletedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ManipulationCompletedRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: ManipulationCompletedRoutedEventArgs) -> Self {
        ::core::convert::Into::<super::RoutedEventArgs>::into(&value)
    }
}
impl ::core::convert::From<&ManipulationCompletedRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: &ManipulationCompletedRoutedEventArgs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for ManipulationCompletedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for &ManipulationCompletedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for ManipulationCompletedRoutedEventArgs {}
unsafe impl ::core::marker::Sync for ManipulationCompletedRoutedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ManipulationDeltaEventHandler(::windows::core::IUnknown);
impl ManipulationDeltaEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<ManipulationDeltaRoutedEventArgs>) -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = ManipulationDeltaEventHandler_box::<F> {
            vtable: &ManipulationDeltaEventHandler_box::<F>::VTABLE,
            count: ::windows::core::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, ManipulationDeltaRoutedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for ManipulationDeltaEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"delegate({aa1160cb-dfb9-4c56-abdc-711b63c8eb94})");
}
unsafe impl ::windows::core::Interface for ManipulationDeltaEventHandler {
    type Vtable = ManipulationDeltaEventHandler_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa1160cb_dfb9_4c56_abdc_711b63c8eb94);
}
#[repr(C)]
#[doc(hidden)]
pub struct ManipulationDeltaEventHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sender: ::windows::core::RawPtr, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(C)]
struct ManipulationDeltaEventHandler_box<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<ManipulationDeltaRoutedEventArgs>) -> ::windows::core::Result<()> + 'static> {
    vtable: *const ManipulationDeltaEventHandler_abi,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<ManipulationDeltaRoutedEventArgs>) -> ::windows::core::Result<()> + 'static> ManipulationDeltaEventHandler_box<F> {
    const VTABLE: ManipulationDeltaEventHandler_abi = ManipulationDeltaEventHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<ManipulationDeltaEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID {
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
    unsafe extern "system" fn Invoke(this: ::windows::core::RawPtr, sender: ::windows::core::RawPtr, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(
            &*(&sender as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
            &*(&e as *const <ManipulationDeltaRoutedEventArgs as ::windows::core::Abi>::Abi as *const <ManipulationDeltaRoutedEventArgs as ::windows::core::DefaultType>::DefaultType),
        )
        .into()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ManipulationDeltaRoutedEventArgs(pub ::windows::core::IInspectable);
impl ManipulationDeltaRoutedEventArgs {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ManipulationDeltaRoutedEventArgs, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Container(&self) -> ::windows::core::Result<super::UIElement> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::UIElement>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
    pub fn IsInertial(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Input"))]
    pub fn Delta(&self) -> ::windows::core::Result<super::super::Input::ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__: super::super::Input::ManipulationDelta = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Input::ManipulationDelta>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Input"))]
    pub fn Cumulative(&self) -> ::windows::core::Result<super::super::Input::ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__: super::super::Input::ManipulationDelta = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Input::ManipulationDelta>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Input"))]
    pub fn Velocities(&self) -> ::windows::core::Result<super::super::Input::ManipulationVelocities> {
        let this = self;
        unsafe {
            let mut result__: super::super::Input::ManipulationVelocities = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Input::ManipulationVelocities>(result__)
        }
    }
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Devices_Input")]
    pub fn PointerDeviceType(&self) -> ::windows::core::Result<super::super::super::Devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Devices::Input::PointerDeviceType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Devices::Input::PointerDeviceType>(result__)
        }
    }
    pub fn Complete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for ManipulationDeltaRoutedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.ManipulationDeltaRoutedEventArgs;{400d5794-4c6f-491d-82d6-3517109399c6})");
}
unsafe impl ::windows::core::Interface for ManipulationDeltaRoutedEventArgs {
    type Vtable = IManipulationDeltaRoutedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x400d5794_4c6f_491d_82d6_3517109399c6);
}
impl ::windows::core::RuntimeName for ManipulationDeltaRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.ManipulationDeltaRoutedEventArgs";
}
impl ::core::convert::From<ManipulationDeltaRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ManipulationDeltaRoutedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ManipulationDeltaRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ManipulationDeltaRoutedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ManipulationDeltaRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ManipulationDeltaRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ManipulationDeltaRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ManipulationDeltaRoutedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ManipulationDeltaRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ManipulationDeltaRoutedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ManipulationDeltaRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ManipulationDeltaRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ManipulationDeltaRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: ManipulationDeltaRoutedEventArgs) -> Self {
        ::core::convert::Into::<super::RoutedEventArgs>::into(&value)
    }
}
impl ::core::convert::From<&ManipulationDeltaRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: &ManipulationDeltaRoutedEventArgs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for ManipulationDeltaRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for &ManipulationDeltaRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for ManipulationDeltaRoutedEventArgs {}
unsafe impl ::core::marker::Sync for ManipulationDeltaRoutedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ManipulationInertiaStartingEventHandler(::windows::core::IUnknown);
impl ManipulationInertiaStartingEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<ManipulationInertiaStartingRoutedEventArgs>) -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = ManipulationInertiaStartingEventHandler_box::<F> {
            vtable: &ManipulationInertiaStartingEventHandler_box::<F>::VTABLE,
            count: ::windows::core::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, ManipulationInertiaStartingRoutedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for ManipulationInertiaStartingEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"delegate({d39d6322-7c9c-481b-827b-c8b2d9bb6fc7})");
}
unsafe impl ::windows::core::Interface for ManipulationInertiaStartingEventHandler {
    type Vtable = ManipulationInertiaStartingEventHandler_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd39d6322_7c9c_481b_827b_c8b2d9bb6fc7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ManipulationInertiaStartingEventHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sender: ::windows::core::RawPtr, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(C)]
struct ManipulationInertiaStartingEventHandler_box<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<ManipulationInertiaStartingRoutedEventArgs>) -> ::windows::core::Result<()> + 'static> {
    vtable: *const ManipulationInertiaStartingEventHandler_abi,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<ManipulationInertiaStartingRoutedEventArgs>) -> ::windows::core::Result<()> + 'static> ManipulationInertiaStartingEventHandler_box<F> {
    const VTABLE: ManipulationInertiaStartingEventHandler_abi = ManipulationInertiaStartingEventHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<ManipulationInertiaStartingEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID {
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
    unsafe extern "system" fn Invoke(this: ::windows::core::RawPtr, sender: ::windows::core::RawPtr, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(
            &*(&sender as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
            &*(&e as *const <ManipulationInertiaStartingRoutedEventArgs as ::windows::core::Abi>::Abi as *const <ManipulationInertiaStartingRoutedEventArgs as ::windows::core::DefaultType>::DefaultType),
        )
        .into()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ManipulationInertiaStartingRoutedEventArgs(pub ::windows::core::IInspectable);
impl ManipulationInertiaStartingRoutedEventArgs {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ManipulationInertiaStartingRoutedEventArgs, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Container(&self) -> ::windows::core::Result<super::UIElement> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::UIElement>(result__)
        }
    }
    pub fn ExpansionBehavior(&self) -> ::windows::core::Result<InertiaExpansionBehavior> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InertiaExpansionBehavior>(result__)
        }
    }
    pub fn SetExpansionBehavior<'a, Param0: ::windows::core::IntoParam<'a, InertiaExpansionBehavior>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn RotationBehavior(&self) -> ::windows::core::Result<InertiaRotationBehavior> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InertiaRotationBehavior>(result__)
        }
    }
    pub fn SetRotationBehavior<'a, Param0: ::windows::core::IntoParam<'a, InertiaRotationBehavior>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn TranslationBehavior(&self) -> ::windows::core::Result<InertiaTranslationBehavior> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InertiaTranslationBehavior>(result__)
        }
    }
    pub fn SetTranslationBehavior<'a, Param0: ::windows::core::IntoParam<'a, InertiaTranslationBehavior>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Devices_Input")]
    pub fn PointerDeviceType(&self) -> ::windows::core::Result<super::super::super::Devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Devices::Input::PointerDeviceType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Devices::Input::PointerDeviceType>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Input"))]
    pub fn Delta(&self) -> ::windows::core::Result<super::super::Input::ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__: super::super::Input::ManipulationDelta = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Input::ManipulationDelta>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Input"))]
    pub fn Cumulative(&self) -> ::windows::core::Result<super::super::Input::ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__: super::super::Input::ManipulationDelta = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Input::ManipulationDelta>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Input"))]
    pub fn Velocities(&self) -> ::windows::core::Result<super::super::Input::ManipulationVelocities> {
        let this = self;
        unsafe {
            let mut result__: super::super::Input::ManipulationVelocities = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Input::ManipulationVelocities>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ManipulationInertiaStartingRoutedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.ManipulationInertiaStartingRoutedEventArgs;{246a91a9-ca43-4c0b-acef-81e8b8147520})");
}
unsafe impl ::windows::core::Interface for ManipulationInertiaStartingRoutedEventArgs {
    type Vtable = IManipulationInertiaStartingRoutedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x246a91a9_ca43_4c0b_acef_81e8b8147520);
}
impl ::windows::core::RuntimeName for ManipulationInertiaStartingRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.ManipulationInertiaStartingRoutedEventArgs";
}
impl ::core::convert::From<ManipulationInertiaStartingRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ManipulationInertiaStartingRoutedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ManipulationInertiaStartingRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ManipulationInertiaStartingRoutedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ManipulationInertiaStartingRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ManipulationInertiaStartingRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ManipulationInertiaStartingRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ManipulationInertiaStartingRoutedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ManipulationInertiaStartingRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ManipulationInertiaStartingRoutedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ManipulationInertiaStartingRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ManipulationInertiaStartingRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ManipulationInertiaStartingRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: ManipulationInertiaStartingRoutedEventArgs) -> Self {
        ::core::convert::Into::<super::RoutedEventArgs>::into(&value)
    }
}
impl ::core::convert::From<&ManipulationInertiaStartingRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: &ManipulationInertiaStartingRoutedEventArgs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for ManipulationInertiaStartingRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for &ManipulationInertiaStartingRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for ManipulationInertiaStartingRoutedEventArgs {}
unsafe impl ::core::marker::Sync for ManipulationInertiaStartingRoutedEventArgs {}
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
unsafe impl ::windows::core::Abi for ManipulationModes {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ManipulationModes {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Input.ManipulationModes;u4)");
}
impl ::windows::core::DefaultType for ManipulationModes {
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
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ManipulationPivot(pub ::windows::core::IInspectable);
impl ManipulationPivot {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ManipulationPivot, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    pub fn Center(&self) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetCenter<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Point>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn Radius(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetRadius(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn CreateInstanceWithCenterAndRadius<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Point>>(center: Param0, radius: f64) -> ::windows::core::Result<ManipulationPivot> {
        Self::IManipulationPivotFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), center.into_param().abi(), radius, &mut result__).from_abi::<ManipulationPivot>(result__)
        })
    }
    pub fn IManipulationPivotFactory<R, F: FnOnce(&IManipulationPivotFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ManipulationPivot, IManipulationPivotFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for ManipulationPivot {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.ManipulationPivot;{2e3838a5-e6c2-4998-82ac-18748b141666})");
}
unsafe impl ::windows::core::Interface for ManipulationPivot {
    type Vtable = IManipulationPivot_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2e3838a5_e6c2_4998_82ac_18748b141666);
}
impl ::windows::core::RuntimeName for ManipulationPivot {
    const NAME: &'static str = "Windows.UI.Xaml.Input.ManipulationPivot";
}
impl ::core::convert::From<ManipulationPivot> for ::windows::core::IUnknown {
    fn from(value: ManipulationPivot) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ManipulationPivot> for ::windows::core::IUnknown {
    fn from(value: &ManipulationPivot) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ManipulationPivot {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ManipulationPivot {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ManipulationPivot> for ::windows::core::IInspectable {
    fn from(value: ManipulationPivot) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ManipulationPivot> for ::windows::core::IInspectable {
    fn from(value: &ManipulationPivot) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ManipulationPivot {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ManipulationPivot {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ManipulationPivot {}
unsafe impl ::core::marker::Sync for ManipulationPivot {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ManipulationStartedEventHandler(::windows::core::IUnknown);
impl ManipulationStartedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<ManipulationStartedRoutedEventArgs>) -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = ManipulationStartedEventHandler_box::<F> {
            vtable: &ManipulationStartedEventHandler_box::<F>::VTABLE,
            count: ::windows::core::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, ManipulationStartedRoutedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for ManipulationStartedEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"delegate({f88345f8-e0a3-4be2-b90c-dc20e6d8beb0})");
}
unsafe impl ::windows::core::Interface for ManipulationStartedEventHandler {
    type Vtable = ManipulationStartedEventHandler_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf88345f8_e0a3_4be2_b90c_dc20e6d8beb0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ManipulationStartedEventHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sender: ::windows::core::RawPtr, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(C)]
struct ManipulationStartedEventHandler_box<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<ManipulationStartedRoutedEventArgs>) -> ::windows::core::Result<()> + 'static> {
    vtable: *const ManipulationStartedEventHandler_abi,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<ManipulationStartedRoutedEventArgs>) -> ::windows::core::Result<()> + 'static> ManipulationStartedEventHandler_box<F> {
    const VTABLE: ManipulationStartedEventHandler_abi = ManipulationStartedEventHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<ManipulationStartedEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID {
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
    unsafe extern "system" fn Invoke(this: ::windows::core::RawPtr, sender: ::windows::core::RawPtr, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(
            &*(&sender as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
            &*(&e as *const <ManipulationStartedRoutedEventArgs as ::windows::core::Abi>::Abi as *const <ManipulationStartedRoutedEventArgs as ::windows::core::DefaultType>::DefaultType),
        )
        .into()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ManipulationStartedRoutedEventArgs(pub ::windows::core::IInspectable);
impl ManipulationStartedRoutedEventArgs {
    pub fn Container(&self) -> ::windows::core::Result<super::UIElement> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::UIElement>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Devices_Input")]
    pub fn PointerDeviceType(&self) -> ::windows::core::Result<super::super::super::Devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Devices::Input::PointerDeviceType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Devices::Input::PointerDeviceType>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Input"))]
    pub fn Cumulative(&self) -> ::windows::core::Result<super::super::Input::ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__: super::super::Input::ManipulationDelta = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Input::ManipulationDelta>(result__)
        }
    }
    pub fn Complete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn new() -> ::windows::core::Result<ManipulationStartedRoutedEventArgs> {
        Self::IManipulationStartedRoutedEventArgsFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<ManipulationStartedRoutedEventArgs>(result__)
        })
    }
    pub fn IManipulationStartedRoutedEventArgsFactory<R, F: FnOnce(&IManipulationStartedRoutedEventArgsFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ManipulationStartedRoutedEventArgs, IManipulationStartedRoutedEventArgsFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for ManipulationStartedRoutedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.ManipulationStartedRoutedEventArgs;{5db1aa05-9f80-48b6-ae6c-4f119de8ff13})");
}
unsafe impl ::windows::core::Interface for ManipulationStartedRoutedEventArgs {
    type Vtable = IManipulationStartedRoutedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5db1aa05_9f80_48b6_ae6c_4f119de8ff13);
}
impl ::windows::core::RuntimeName for ManipulationStartedRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.ManipulationStartedRoutedEventArgs";
}
impl ::core::convert::From<ManipulationStartedRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ManipulationStartedRoutedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ManipulationStartedRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ManipulationStartedRoutedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ManipulationStartedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ManipulationStartedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ManipulationStartedRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ManipulationStartedRoutedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ManipulationStartedRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ManipulationStartedRoutedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ManipulationStartedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ManipulationStartedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ManipulationStartedRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: ManipulationStartedRoutedEventArgs) -> Self {
        ::core::convert::Into::<super::RoutedEventArgs>::into(&value)
    }
}
impl ::core::convert::From<&ManipulationStartedRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: &ManipulationStartedRoutedEventArgs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for ManipulationStartedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for &ManipulationStartedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for ManipulationStartedRoutedEventArgs {}
unsafe impl ::core::marker::Sync for ManipulationStartedRoutedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ManipulationStartingEventHandler(::windows::core::IUnknown);
impl ManipulationStartingEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<ManipulationStartingRoutedEventArgs>) -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = ManipulationStartingEventHandler_box::<F> {
            vtable: &ManipulationStartingEventHandler_box::<F>::VTABLE,
            count: ::windows::core::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, ManipulationStartingRoutedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for ManipulationStartingEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"delegate({10d0b04e-bfe4-42cb-823c-3fecd8770ef8})");
}
unsafe impl ::windows::core::Interface for ManipulationStartingEventHandler {
    type Vtable = ManipulationStartingEventHandler_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x10d0b04e_bfe4_42cb_823c_3fecd8770ef8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ManipulationStartingEventHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sender: ::windows::core::RawPtr, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(C)]
struct ManipulationStartingEventHandler_box<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<ManipulationStartingRoutedEventArgs>) -> ::windows::core::Result<()> + 'static> {
    vtable: *const ManipulationStartingEventHandler_abi,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<ManipulationStartingRoutedEventArgs>) -> ::windows::core::Result<()> + 'static> ManipulationStartingEventHandler_box<F> {
    const VTABLE: ManipulationStartingEventHandler_abi = ManipulationStartingEventHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<ManipulationStartingEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID {
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
    unsafe extern "system" fn Invoke(this: ::windows::core::RawPtr, sender: ::windows::core::RawPtr, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(
            &*(&sender as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
            &*(&e as *const <ManipulationStartingRoutedEventArgs as ::windows::core::Abi>::Abi as *const <ManipulationStartingRoutedEventArgs as ::windows::core::DefaultType>::DefaultType),
        )
        .into()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ManipulationStartingRoutedEventArgs(pub ::windows::core::IInspectable);
impl ManipulationStartingRoutedEventArgs {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ManipulationStartingRoutedEventArgs, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Mode(&self) -> ::windows::core::Result<ManipulationModes> {
        let this = self;
        unsafe {
            let mut result__: ManipulationModes = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ManipulationModes>(result__)
        }
    }
    pub fn SetMode(&self, value: ManipulationModes) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Container(&self) -> ::windows::core::Result<super::UIElement> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::UIElement>(result__)
        }
    }
    pub fn SetContainer<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn Pivot(&self) -> ::windows::core::Result<ManipulationPivot> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ManipulationPivot>(result__)
        }
    }
    pub fn SetPivot<'a, Param0: ::windows::core::IntoParam<'a, ManipulationPivot>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for ManipulationStartingRoutedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.ManipulationStartingRoutedEventArgs;{18d636b7-53a4-4c15-a498-f3a9ca212a42})");
}
unsafe impl ::windows::core::Interface for ManipulationStartingRoutedEventArgs {
    type Vtable = IManipulationStartingRoutedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x18d636b7_53a4_4c15_a498_f3a9ca212a42);
}
impl ::windows::core::RuntimeName for ManipulationStartingRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.ManipulationStartingRoutedEventArgs";
}
impl ::core::convert::From<ManipulationStartingRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ManipulationStartingRoutedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ManipulationStartingRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ManipulationStartingRoutedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ManipulationStartingRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ManipulationStartingRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ManipulationStartingRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ManipulationStartingRoutedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ManipulationStartingRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ManipulationStartingRoutedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ManipulationStartingRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ManipulationStartingRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ManipulationStartingRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: ManipulationStartingRoutedEventArgs) -> Self {
        ::core::convert::Into::<super::RoutedEventArgs>::into(&value)
    }
}
impl ::core::convert::From<&ManipulationStartingRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: &ManipulationStartingRoutedEventArgs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for ManipulationStartingRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for &ManipulationStartingRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for ManipulationStartingRoutedEventArgs {}
unsafe impl ::core::marker::Sync for ManipulationStartingRoutedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct NoFocusCandidateFoundEventArgs(pub ::windows::core::IInspectable);
impl NoFocusCandidateFoundEventArgs {
    pub fn Direction(&self) -> ::windows::core::Result<FocusNavigationDirection> {
        let this = self;
        unsafe {
            let mut result__: FocusNavigationDirection = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FocusNavigationDirection>(result__)
        }
    }
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn InputDevice(&self) -> ::windows::core::Result<FocusInputDeviceKind> {
        let this = self;
        unsafe {
            let mut result__: FocusInputDeviceKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FocusInputDeviceKind>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for NoFocusCandidateFoundEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.NoFocusCandidateFoundEventArgs;{ec3601a7-1007-48f9-b6b3-ed0bea53937d})");
}
unsafe impl ::windows::core::Interface for NoFocusCandidateFoundEventArgs {
    type Vtable = INoFocusCandidateFoundEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xec3601a7_1007_48f9_b6b3_ed0bea53937d);
}
impl ::windows::core::RuntimeName for NoFocusCandidateFoundEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.NoFocusCandidateFoundEventArgs";
}
impl ::core::convert::From<NoFocusCandidateFoundEventArgs> for ::windows::core::IUnknown {
    fn from(value: NoFocusCandidateFoundEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&NoFocusCandidateFoundEventArgs> for ::windows::core::IUnknown {
    fn from(value: &NoFocusCandidateFoundEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for NoFocusCandidateFoundEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a NoFocusCandidateFoundEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<NoFocusCandidateFoundEventArgs> for ::windows::core::IInspectable {
    fn from(value: NoFocusCandidateFoundEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&NoFocusCandidateFoundEventArgs> for ::windows::core::IInspectable {
    fn from(value: &NoFocusCandidateFoundEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for NoFocusCandidateFoundEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a NoFocusCandidateFoundEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<NoFocusCandidateFoundEventArgs> for super::RoutedEventArgs {
    fn from(value: NoFocusCandidateFoundEventArgs) -> Self {
        ::core::convert::Into::<super::RoutedEventArgs>::into(&value)
    }
}
impl ::core::convert::From<&NoFocusCandidateFoundEventArgs> for super::RoutedEventArgs {
    fn from(value: &NoFocusCandidateFoundEventArgs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for NoFocusCandidateFoundEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for &NoFocusCandidateFoundEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for NoFocusCandidateFoundEventArgs {}
unsafe impl ::core::marker::Sync for NoFocusCandidateFoundEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct Pointer(pub ::windows::core::IInspectable);
impl Pointer {
    pub fn PointerId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Devices_Input")]
    pub fn PointerDeviceType(&self) -> ::windows::core::Result<super::super::super::Devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Devices::Input::PointerDeviceType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Devices::Input::PointerDeviceType>(result__)
        }
    }
    pub fn IsInContact(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsInRange(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for Pointer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.Pointer;{5ee8f39f-747d-4171-90e6-cd37a9dffb11})");
}
unsafe impl ::windows::core::Interface for Pointer {
    type Vtable = IPointer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5ee8f39f_747d_4171_90e6_cd37a9dffb11);
}
impl ::windows::core::RuntimeName for Pointer {
    const NAME: &'static str = "Windows.UI.Xaml.Input.Pointer";
}
impl ::core::convert::From<Pointer> for ::windows::core::IUnknown {
    fn from(value: Pointer) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Pointer> for ::windows::core::IUnknown {
    fn from(value: &Pointer) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Pointer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Pointer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Pointer> for ::windows::core::IInspectable {
    fn from(value: Pointer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Pointer> for ::windows::core::IInspectable {
    fn from(value: &Pointer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Pointer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Pointer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for Pointer {}
unsafe impl ::core::marker::Sync for Pointer {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PointerEventHandler(::windows::core::IUnknown);
impl PointerEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<PointerRoutedEventArgs>) -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = PointerEventHandler_box::<F> {
            vtable: &PointerEventHandler_box::<F>::VTABLE,
            count: ::windows::core::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, PointerRoutedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for PointerEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"delegate({e4385929-c004-4bcf-8970-359486e39f88})");
}
unsafe impl ::windows::core::Interface for PointerEventHandler {
    type Vtable = PointerEventHandler_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe4385929_c004_4bcf_8970_359486e39f88);
}
#[repr(C)]
#[doc(hidden)]
pub struct PointerEventHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sender: ::windows::core::RawPtr, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(C)]
struct PointerEventHandler_box<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<PointerRoutedEventArgs>) -> ::windows::core::Result<()> + 'static> {
    vtable: *const PointerEventHandler_abi,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<PointerRoutedEventArgs>) -> ::windows::core::Result<()> + 'static> PointerEventHandler_box<F> {
    const VTABLE: PointerEventHandler_abi = PointerEventHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<PointerEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID {
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
    unsafe extern "system" fn Invoke(this: ::windows::core::RawPtr, sender: ::windows::core::RawPtr, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(
            &*(&sender as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
            &*(&e as *const <PointerRoutedEventArgs as ::windows::core::Abi>::Abi as *const <PointerRoutedEventArgs as ::windows::core::DefaultType>::DefaultType),
        )
        .into()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PointerRoutedEventArgs(pub ::windows::core::IInspectable);
impl PointerRoutedEventArgs {
    pub fn Pointer(&self) -> ::windows::core::Result<Pointer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Pointer>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn KeyModifiers(&self) -> ::windows::core::Result<super::super::super::System::VirtualKeyModifiers> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::System::VirtualKeyModifiers = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::VirtualKeyModifiers>(result__)
        }
    }
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "UI_Input")]
    pub fn GetCurrentPoint<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(&self, relativeto: Param0) -> ::windows::core::Result<super::super::Input::PointerPoint> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), relativeto.into_param().abi(), &mut result__).from_abi::<super::super::Input::PointerPoint>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Input"))]
    pub fn GetIntermediatePoints<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(&self, relativeto: Param0) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<super::super::Input::PointerPoint>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), relativeto.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<super::super::Input::PointerPoint>>(result__)
        }
    }
    pub fn IsGenerated(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IPointerRoutedEventArgs2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PointerRoutedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.PointerRoutedEventArgs;{da628f0a-9752-49e2-bde2-49eccab9194d})");
}
unsafe impl ::windows::core::Interface for PointerRoutedEventArgs {
    type Vtable = IPointerRoutedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xda628f0a_9752_49e2_bde2_49eccab9194d);
}
impl ::windows::core::RuntimeName for PointerRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.PointerRoutedEventArgs";
}
impl ::core::convert::From<PointerRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: PointerRoutedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PointerRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PointerRoutedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PointerRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PointerRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PointerRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: PointerRoutedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PointerRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PointerRoutedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PointerRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PointerRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<PointerRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: PointerRoutedEventArgs) -> Self {
        ::core::convert::Into::<super::RoutedEventArgs>::into(&value)
    }
}
impl ::core::convert::From<&PointerRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: &PointerRoutedEventArgs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for PointerRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for &PointerRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for PointerRoutedEventArgs {}
unsafe impl ::core::marker::Sync for PointerRoutedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ProcessKeyboardAcceleratorEventArgs(pub ::windows::core::IInspectable);
impl ProcessKeyboardAcceleratorEventArgs {
    #[cfg(feature = "System")]
    pub fn Key(&self) -> ::windows::core::Result<super::super::super::System::VirtualKey> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::System::VirtualKey = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::VirtualKey>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn Modifiers(&self) -> ::windows::core::Result<super::super::super::System::VirtualKeyModifiers> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::System::VirtualKeyModifiers = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::VirtualKeyModifiers>(result__)
        }
    }
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for ProcessKeyboardAcceleratorEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.ProcessKeyboardAcceleratorEventArgs;{fb79c216-972b-440c-9b83-2b4198dcf09d})");
}
unsafe impl ::windows::core::Interface for ProcessKeyboardAcceleratorEventArgs {
    type Vtable = IProcessKeyboardAcceleratorEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb79c216_972b_440c_9b83_2b4198dcf09d);
}
impl ::windows::core::RuntimeName for ProcessKeyboardAcceleratorEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.ProcessKeyboardAcceleratorEventArgs";
}
impl ::core::convert::From<ProcessKeyboardAcceleratorEventArgs> for ::windows::core::IUnknown {
    fn from(value: ProcessKeyboardAcceleratorEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ProcessKeyboardAcceleratorEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ProcessKeyboardAcceleratorEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ProcessKeyboardAcceleratorEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ProcessKeyboardAcceleratorEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ProcessKeyboardAcceleratorEventArgs> for ::windows::core::IInspectable {
    fn from(value: ProcessKeyboardAcceleratorEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ProcessKeyboardAcceleratorEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ProcessKeyboardAcceleratorEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ProcessKeyboardAcceleratorEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ProcessKeyboardAcceleratorEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ProcessKeyboardAcceleratorEventArgs {}
unsafe impl ::core::marker::Sync for ProcessKeyboardAcceleratorEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RightTappedEventHandler(::windows::core::IUnknown);
impl RightTappedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<RightTappedRoutedEventArgs>) -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = RightTappedEventHandler_box::<F> {
            vtable: &RightTappedEventHandler_box::<F>::VTABLE,
            count: ::windows::core::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, RightTappedRoutedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for RightTappedEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"delegate({2532a062-f447-4950-9c46-f1e34a2c2238})");
}
unsafe impl ::windows::core::Interface for RightTappedEventHandler {
    type Vtable = RightTappedEventHandler_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2532a062_f447_4950_9c46_f1e34a2c2238);
}
#[repr(C)]
#[doc(hidden)]
pub struct RightTappedEventHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sender: ::windows::core::RawPtr, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(C)]
struct RightTappedEventHandler_box<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<RightTappedRoutedEventArgs>) -> ::windows::core::Result<()> + 'static> {
    vtable: *const RightTappedEventHandler_abi,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<RightTappedRoutedEventArgs>) -> ::windows::core::Result<()> + 'static> RightTappedEventHandler_box<F> {
    const VTABLE: RightTappedEventHandler_abi = RightTappedEventHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<RightTappedEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID {
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
    unsafe extern "system" fn Invoke(this: ::windows::core::RawPtr, sender: ::windows::core::RawPtr, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(
            &*(&sender as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
            &*(&e as *const <RightTappedRoutedEventArgs as ::windows::core::Abi>::Abi as *const <RightTappedRoutedEventArgs as ::windows::core::DefaultType>::DefaultType),
        )
        .into()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RightTappedRoutedEventArgs(pub ::windows::core::IInspectable);
impl RightTappedRoutedEventArgs {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<RightTappedRoutedEventArgs, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Devices_Input")]
    pub fn PointerDeviceType(&self) -> ::windows::core::Result<super::super::super::Devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Devices::Input::PointerDeviceType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Devices::Input::PointerDeviceType>(result__)
        }
    }
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetPosition<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(&self, relativeto: Param0) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), relativeto.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for RightTappedRoutedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.RightTappedRoutedEventArgs;{6834869d-7bd5-4033-b237-172f79abe393})");
}
unsafe impl ::windows::core::Interface for RightTappedRoutedEventArgs {
    type Vtable = IRightTappedRoutedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6834869d_7bd5_4033_b237_172f79abe393);
}
impl ::windows::core::RuntimeName for RightTappedRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.RightTappedRoutedEventArgs";
}
impl ::core::convert::From<RightTappedRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: RightTappedRoutedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RightTappedRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &RightTappedRoutedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RightTappedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a RightTappedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RightTappedRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: RightTappedRoutedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RightTappedRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &RightTappedRoutedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RightTappedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a RightTappedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<RightTappedRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: RightTappedRoutedEventArgs) -> Self {
        ::core::convert::Into::<super::RoutedEventArgs>::into(&value)
    }
}
impl ::core::convert::From<&RightTappedRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: &RightTappedRoutedEventArgs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for RightTappedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for &RightTappedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for RightTappedRoutedEventArgs {}
unsafe impl ::core::marker::Sync for RightTappedRoutedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct StandardUICommand(pub ::windows::core::IInspectable);
impl StandardUICommand {
    pub fn Kind(&self) -> ::windows::core::Result<StandardUICommandKind> {
        let this = self;
        unsafe {
            let mut result__: StandardUICommandKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StandardUICommandKind>(result__)
        }
    }
    pub fn KindProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IStandardUICommandStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn new() -> ::windows::core::Result<StandardUICommand> {
        Self::IStandardUICommandFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<StandardUICommand>(result__)
        })
    }
    pub fn CreateInstanceWithKind(kind: StandardUICommandKind) -> ::windows::core::Result<StandardUICommand> {
        Self::IStandardUICommandFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), kind, ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<StandardUICommand>(result__)
        })
    }
    pub fn SetKind(&self, value: StandardUICommandKind) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IStandardUICommand2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IStandardUICommandStatics<R, F: FnOnce(&IStandardUICommandStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StandardUICommand, IStandardUICommandStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IStandardUICommandFactory<R, F: FnOnce(&IStandardUICommandFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StandardUICommand, IStandardUICommandFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for StandardUICommand {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.StandardUICommand;{d2bf7f43-0504-52d0-8aa6-0cb0f756eb27})");
}
unsafe impl ::windows::core::Interface for StandardUICommand {
    type Vtable = IStandardUICommand_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd2bf7f43_0504_52d0_8aa6_0cb0f756eb27);
}
impl ::windows::core::RuntimeName for StandardUICommand {
    const NAME: &'static str = "Windows.UI.Xaml.Input.StandardUICommand";
}
impl ::core::convert::From<StandardUICommand> for ::windows::core::IUnknown {
    fn from(value: StandardUICommand) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&StandardUICommand> for ::windows::core::IUnknown {
    fn from(value: &StandardUICommand) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StandardUICommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StandardUICommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<StandardUICommand> for ::windows::core::IInspectable {
    fn from(value: StandardUICommand) -> Self {
        value.0
    }
}
impl ::core::convert::From<&StandardUICommand> for ::windows::core::IInspectable {
    fn from(value: &StandardUICommand) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StandardUICommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StandardUICommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<StandardUICommand> for ICommand {
    type Error = ::windows::core::Error;
    fn try_from(value: StandardUICommand) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&StandardUICommand> for ICommand {
    type Error = ::windows::core::Error;
    fn try_from(value: &StandardUICommand) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICommand> for StandardUICommand {
    fn into_param(self) -> ::windows::core::Param<'a, ICommand> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICommand> for &StandardUICommand {
    fn into_param(self) -> ::windows::core::Param<'a, ICommand> {
        ::core::convert::TryInto::<ICommand>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<StandardUICommand> for XamlUICommand {
    fn from(value: StandardUICommand) -> Self {
        ::core::convert::Into::<XamlUICommand>::into(&value)
    }
}
impl ::core::convert::From<&StandardUICommand> for XamlUICommand {
    fn from(value: &StandardUICommand) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, XamlUICommand> for StandardUICommand {
    fn into_param(self) -> ::windows::core::Param<'a, XamlUICommand> {
        ::windows::core::Param::Owned(::core::convert::Into::<XamlUICommand>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, XamlUICommand> for &StandardUICommand {
    fn into_param(self) -> ::windows::core::Param<'a, XamlUICommand> {
        ::windows::core::Param::Owned(::core::convert::Into::<XamlUICommand>::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<StandardUICommand> for super::DependencyObject {
    fn from(value: StandardUICommand) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&StandardUICommand> for super::DependencyObject {
    fn from(value: &StandardUICommand) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for StandardUICommand {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &StandardUICommand {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for StandardUICommand {}
unsafe impl ::core::marker::Sync for StandardUICommand {}
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
unsafe impl ::windows::core::Abi for StandardUICommandKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for StandardUICommandKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Input.StandardUICommandKind;i4)");
}
impl ::windows::core::DefaultType for StandardUICommandKind {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct TappedEventHandler(::windows::core::IUnknown);
impl TappedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<TappedRoutedEventArgs>) -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = TappedEventHandler_box::<F> {
            vtable: &TappedEventHandler_box::<F>::VTABLE,
            count: ::windows::core::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, TappedRoutedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for TappedEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"delegate({68d940cc-9ff0-49ce-b141-3f07ec477b97})");
}
unsafe impl ::windows::core::Interface for TappedEventHandler {
    type Vtable = TappedEventHandler_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x68d940cc_9ff0_49ce_b141_3f07ec477b97);
}
#[repr(C)]
#[doc(hidden)]
pub struct TappedEventHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sender: ::windows::core::RawPtr, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(C)]
struct TappedEventHandler_box<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<TappedRoutedEventArgs>) -> ::windows::core::Result<()> + 'static> {
    vtable: *const TappedEventHandler_abi,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<TappedRoutedEventArgs>) -> ::windows::core::Result<()> + 'static> TappedEventHandler_box<F> {
    const VTABLE: TappedEventHandler_abi = TappedEventHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<TappedEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID {
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
    unsafe extern "system" fn Invoke(this: ::windows::core::RawPtr, sender: ::windows::core::RawPtr, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(
            &*(&sender as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
            &*(&e as *const <TappedRoutedEventArgs as ::windows::core::Abi>::Abi as *const <TappedRoutedEventArgs as ::windows::core::DefaultType>::DefaultType),
        )
        .into()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct TappedRoutedEventArgs(pub ::windows::core::IInspectable);
impl TappedRoutedEventArgs {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TappedRoutedEventArgs, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Devices_Input")]
    pub fn PointerDeviceType(&self) -> ::windows::core::Result<super::super::super::Devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Devices::Input::PointerDeviceType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Devices::Input::PointerDeviceType>(result__)
        }
    }
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetPosition<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(&self, relativeto: Param0) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), relativeto.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for TappedRoutedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.TappedRoutedEventArgs;{a099e6be-e624-459a-bb1d-e05c73e2cc66})");
}
unsafe impl ::windows::core::Interface for TappedRoutedEventArgs {
    type Vtable = ITappedRoutedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa099e6be_e624_459a_bb1d_e05c73e2cc66);
}
impl ::windows::core::RuntimeName for TappedRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.TappedRoutedEventArgs";
}
impl ::core::convert::From<TappedRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: TappedRoutedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&TappedRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &TappedRoutedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TappedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TappedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<TappedRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: TappedRoutedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&TappedRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &TappedRoutedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TappedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TappedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<TappedRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: TappedRoutedEventArgs) -> Self {
        ::core::convert::Into::<super::RoutedEventArgs>::into(&value)
    }
}
impl ::core::convert::From<&TappedRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: &TappedRoutedEventArgs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for TappedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for &TappedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for TappedRoutedEventArgs {}
unsafe impl ::core::marker::Sync for TappedRoutedEventArgs {}
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
unsafe impl ::windows::core::Abi for XYFocusKeyboardNavigationMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for XYFocusKeyboardNavigationMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Input.XYFocusKeyboardNavigationMode;i4)");
}
impl ::windows::core::DefaultType for XYFocusKeyboardNavigationMode {
    type DefaultType = Self;
}
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
unsafe impl ::windows::core::Abi for XYFocusNavigationStrategy {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for XYFocusNavigationStrategy {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Input.XYFocusNavigationStrategy;i4)");
}
impl ::windows::core::DefaultType for XYFocusNavigationStrategy {
    type DefaultType = Self;
}
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
unsafe impl ::windows::core::Abi for XYFocusNavigationStrategyOverride {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for XYFocusNavigationStrategyOverride {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Input.XYFocusNavigationStrategyOverride;i4)");
}
impl ::windows::core::DefaultType for XYFocusNavigationStrategyOverride {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct XamlUICommand(pub ::windows::core::IInspectable);
impl XamlUICommand {
    #[cfg(feature = "Foundation")]
    pub fn CanExecuteChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICommand>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveCanExecuteChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICommand>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn CanExecute<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, parameter: Param0) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommand>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), parameter.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn Execute<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, parameter: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICommand>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), parameter.into_param().abi()).ok() }
    }
    pub fn Label(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetLabel<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Controls")]
    pub fn IconSource(&self) -> ::windows::core::Result<super::Controls::IconSource> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Controls::IconSource>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Controls")]
    pub fn SetIconSource<'a, Param0: ::windows::core::IntoParam<'a, super::Controls::IconSource>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn KeyboardAccelerators(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<KeyboardAccelerator>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<KeyboardAccelerator>>(result__)
        }
    }
    pub fn AccessKey(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetAccessKey<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn Command(&self) -> ::windows::core::Result<ICommand> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ICommand>(result__)
        }
    }
    pub fn SetCommand<'a, Param0: ::windows::core::IntoParam<'a, ICommand>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn ExecuteRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<XamlUICommand, ExecuteRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveExecuteRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn CanExecuteRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<XamlUICommand, CanExecuteRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveCanExecuteRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn NotifyCanExecuteChanged(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn LabelProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IXamlUICommandStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IconSourceProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IXamlUICommandStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn KeyboardAcceleratorsProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IXamlUICommandStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn AccessKeyProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IXamlUICommandStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn DescriptionProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IXamlUICommandStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn CommandProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IXamlUICommandStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn new() -> ::windows::core::Result<XamlUICommand> {
        Self::IXamlUICommandFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<XamlUICommand>(result__)
        })
    }
    pub fn IXamlUICommandStatics<R, F: FnOnce(&IXamlUICommandStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<XamlUICommand, IXamlUICommandStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IXamlUICommandFactory<R, F: FnOnce(&IXamlUICommandFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<XamlUICommand, IXamlUICommandFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for XamlUICommand {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.XamlUICommand;{8494f8d4-ead1-5f01-ad2e-a8cad4f9dc0e})");
}
unsafe impl ::windows::core::Interface for XamlUICommand {
    type Vtable = IXamlUICommand_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8494f8d4_ead1_5f01_ad2e_a8cad4f9dc0e);
}
impl ::windows::core::RuntimeName for XamlUICommand {
    const NAME: &'static str = "Windows.UI.Xaml.Input.XamlUICommand";
}
impl ::core::convert::From<XamlUICommand> for ::windows::core::IUnknown {
    fn from(value: XamlUICommand) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&XamlUICommand> for ::windows::core::IUnknown {
    fn from(value: &XamlUICommand) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for XamlUICommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a XamlUICommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<XamlUICommand> for ::windows::core::IInspectable {
    fn from(value: XamlUICommand) -> Self {
        value.0
    }
}
impl ::core::convert::From<&XamlUICommand> for ::windows::core::IInspectable {
    fn from(value: &XamlUICommand) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for XamlUICommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a XamlUICommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<XamlUICommand> for ICommand {
    type Error = ::windows::core::Error;
    fn try_from(value: XamlUICommand) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XamlUICommand> for ICommand {
    type Error = ::windows::core::Error;
    fn try_from(value: &XamlUICommand) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICommand> for XamlUICommand {
    fn into_param(self) -> ::windows::core::Param<'a, ICommand> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICommand> for &XamlUICommand {
    fn into_param(self) -> ::windows::core::Param<'a, ICommand> {
        ::core::convert::TryInto::<ICommand>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<XamlUICommand> for super::DependencyObject {
    fn from(value: XamlUICommand) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&XamlUICommand> for super::DependencyObject {
    fn from(value: &XamlUICommand) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for XamlUICommand {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &XamlUICommand {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for XamlUICommand {}
unsafe impl ::core::marker::Sync for XamlUICommand {}
