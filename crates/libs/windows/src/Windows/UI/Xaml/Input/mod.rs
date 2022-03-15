#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
pub struct AccessKeyDisplayDismissedEventArgs(::windows::core::IUnknown);
impl AccessKeyDisplayDismissedEventArgs {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AccessKeyDisplayDismissedEventArgs, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AccessKeyDisplayDismissedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AccessKeyDisplayDismissedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AccessKeyDisplayDismissedEventArgs {}
impl ::core::fmt::Debug for AccessKeyDisplayDismissedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AccessKeyDisplayDismissedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AccessKeyDisplayDismissedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.AccessKeyDisplayDismissedEventArgs;{8a610dc6-d72d-4ca8-9f66-556f35b513da})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AccessKeyDisplayDismissedEventArgs {
    type Vtable = IAccessKeyDisplayDismissedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IAccessKeyDisplayDismissedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AccessKeyDisplayDismissedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.AccessKeyDisplayDismissedEventArgs";
}
impl ::core::convert::From<AccessKeyDisplayDismissedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AccessKeyDisplayDismissedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AccessKeyDisplayDismissedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AccessKeyDisplayDismissedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AccessKeyDisplayDismissedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AccessKeyDisplayDismissedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AccessKeyDisplayDismissedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AccessKeyDisplayDismissedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AccessKeyDisplayDismissedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AccessKeyDisplayDismissedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AccessKeyDisplayDismissedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AccessKeyDisplayDismissedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AccessKeyDisplayDismissedEventArgs {}
unsafe impl ::core::marker::Sync for AccessKeyDisplayDismissedEventArgs {}
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
pub struct AccessKeyDisplayRequestedEventArgs(::windows::core::IUnknown);
impl AccessKeyDisplayRequestedEventArgs {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AccessKeyDisplayRequestedEventArgs, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn PressedKeys(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PressedKeys)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for AccessKeyDisplayRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AccessKeyDisplayRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AccessKeyDisplayRequestedEventArgs {}
impl ::core::fmt::Debug for AccessKeyDisplayRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AccessKeyDisplayRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AccessKeyDisplayRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.AccessKeyDisplayRequestedEventArgs;{0c079e55-13fe-4d03-a61d-e12f06567286})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AccessKeyDisplayRequestedEventArgs {
    type Vtable = IAccessKeyDisplayRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IAccessKeyDisplayRequestedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AccessKeyDisplayRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.AccessKeyDisplayRequestedEventArgs";
}
impl ::core::convert::From<AccessKeyDisplayRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AccessKeyDisplayRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AccessKeyDisplayRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AccessKeyDisplayRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AccessKeyDisplayRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AccessKeyDisplayRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AccessKeyDisplayRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AccessKeyDisplayRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AccessKeyDisplayRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AccessKeyDisplayRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AccessKeyDisplayRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AccessKeyDisplayRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AccessKeyDisplayRequestedEventArgs {}
unsafe impl ::core::marker::Sync for AccessKeyDisplayRequestedEventArgs {}
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
pub struct AccessKeyInvokedEventArgs(::windows::core::IUnknown);
impl AccessKeyInvokedEventArgs {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AccessKeyInvokedEventArgs, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Handled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHandled)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::clone::Clone for AccessKeyInvokedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AccessKeyInvokedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AccessKeyInvokedEventArgs {}
impl ::core::fmt::Debug for AccessKeyInvokedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AccessKeyInvokedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AccessKeyInvokedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.AccessKeyInvokedEventArgs;{cfe9cd97-c718-4091-b7dd-adf1c072b1e1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AccessKeyInvokedEventArgs {
    type Vtable = IAccessKeyInvokedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IAccessKeyInvokedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AccessKeyInvokedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.AccessKeyInvokedEventArgs";
}
impl ::core::convert::From<AccessKeyInvokedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AccessKeyInvokedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AccessKeyInvokedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AccessKeyInvokedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AccessKeyInvokedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AccessKeyInvokedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AccessKeyInvokedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AccessKeyInvokedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AccessKeyInvokedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AccessKeyInvokedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AccessKeyInvokedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AccessKeyInvokedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AccessKeyInvokedEventArgs {}
unsafe impl ::core::marker::Sync for AccessKeyInvokedEventArgs {}
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
pub struct AccessKeyManager(::windows::core::IUnknown);
impl AccessKeyManager {
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn IsDisplayModeEnabled() -> ::windows::core::Result<bool> {
        Self::IAccessKeyManagerStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsDisplayModeEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn IsDisplayModeEnabledChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, ::windows::core::IInspectable>>>(handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        Self::IAccessKeyManagerStatics(|this| unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsDisplayModeEnabledChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveIsDisplayModeEnabledChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IAccessKeyManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveIsDisplayModeEnabledChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn ExitDisplayMode() -> ::windows::core::Result<()> {
        Self::IAccessKeyManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).ExitDisplayMode)(::core::mem::transmute_copy(this)).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn AreKeyTipsEnabled() -> ::windows::core::Result<bool> {
        Self::IAccessKeyManagerStatics2(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AreKeyTipsEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn SetAreKeyTipsEnabled(value: bool) -> ::windows::core::Result<()> {
        Self::IAccessKeyManagerStatics2(|this| unsafe { (::windows::core::Interface::vtable(this).SetAreKeyTipsEnabled)(::core::mem::transmute_copy(this), value).ok() })
    }
    #[doc(hidden)]
    pub fn IAccessKeyManagerStatics<R, F: FnOnce(&IAccessKeyManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AccessKeyManager, IAccessKeyManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IAccessKeyManagerStatics2<R, F: FnOnce(&IAccessKeyManagerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AccessKeyManager, IAccessKeyManagerStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AccessKeyManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AccessKeyManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AccessKeyManager {}
impl ::core::fmt::Debug for AccessKeyManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AccessKeyManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AccessKeyManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.AccessKeyManager;{ecc973b0-2ee9-4b1c-98d7-6e0e816d334b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AccessKeyManager {
    type Vtable = IAccessKeyManager_Vtbl;
    const IID: ::windows::core::GUID = <IAccessKeyManager as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AccessKeyManager {
    const NAME: &'static str = "Windows.UI.Xaml.Input.AccessKeyManager";
}
impl ::core::convert::From<AccessKeyManager> for ::windows::core::IUnknown {
    fn from(value: AccessKeyManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AccessKeyManager> for ::windows::core::IUnknown {
    fn from(value: &AccessKeyManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AccessKeyManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AccessKeyManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AccessKeyManager> for ::windows::core::IInspectable {
    fn from(value: AccessKeyManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AccessKeyManager> for ::windows::core::IInspectable {
    fn from(value: &AccessKeyManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AccessKeyManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AccessKeyManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AccessKeyManager {}
unsafe impl ::core::marker::Sync for AccessKeyManager {}
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
pub struct CanExecuteRequestedEventArgs(::windows::core::IUnknown);
impl CanExecuteRequestedEventArgs {
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn Parameter(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Parameter)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn CanExecute(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanExecute)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn SetCanExecute(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCanExecute)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::clone::Clone for CanExecuteRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CanExecuteRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CanExecuteRequestedEventArgs {}
impl ::core::fmt::Debug for CanExecuteRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanExecuteRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanExecuteRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.CanExecuteRequestedEventArgs;{c8e75256-1950-505d-993b-75907ef96830})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CanExecuteRequestedEventArgs {
    type Vtable = ICanExecuteRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <ICanExecuteRequestedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CanExecuteRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.CanExecuteRequestedEventArgs";
}
impl ::core::convert::From<CanExecuteRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: CanExecuteRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CanExecuteRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CanExecuteRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CanExecuteRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CanExecuteRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CanExecuteRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: CanExecuteRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CanExecuteRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CanExecuteRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CanExecuteRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CanExecuteRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CanExecuteRequestedEventArgs {}
unsafe impl ::core::marker::Sync for CanExecuteRequestedEventArgs {}
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
pub struct CharacterReceivedRoutedEventArgs(::windows::core::IUnknown);
impl CharacterReceivedRoutedEventArgs {
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn Character(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Character)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn KeyStatus(&self) -> ::windows::core::Result<super::super::Core::CorePhysicalKeyStatus> {
        let this = self;
        unsafe {
            let mut result__: super::super::Core::CorePhysicalKeyStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyStatus)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Core::CorePhysicalKeyStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Handled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHandled)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::clone::Clone for CharacterReceivedRoutedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CharacterReceivedRoutedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CharacterReceivedRoutedEventArgs {}
impl ::core::fmt::Debug for CharacterReceivedRoutedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CharacterReceivedRoutedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CharacterReceivedRoutedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.CharacterReceivedRoutedEventArgs;{7849fd82-48e4-444d-9419-93ab8892c107})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CharacterReceivedRoutedEventArgs {
    type Vtable = ICharacterReceivedRoutedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <ICharacterReceivedRoutedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CharacterReceivedRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.CharacterReceivedRoutedEventArgs";
}
impl ::core::convert::From<CharacterReceivedRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: CharacterReceivedRoutedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CharacterReceivedRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CharacterReceivedRoutedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CharacterReceivedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CharacterReceivedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CharacterReceivedRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: CharacterReceivedRoutedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CharacterReceivedRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CharacterReceivedRoutedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CharacterReceivedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CharacterReceivedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CharacterReceivedRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: CharacterReceivedRoutedEventArgs) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&CharacterReceivedRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: &CharacterReceivedRoutedEventArgs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for CharacterReceivedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for &CharacterReceivedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
unsafe impl ::core::marker::Send for CharacterReceivedRoutedEventArgs {}
unsafe impl ::core::marker::Sync for CharacterReceivedRoutedEventArgs {}
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
pub struct ContextRequestedEventArgs(::windows::core::IUnknown);
impl ContextRequestedEventArgs {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ContextRequestedEventArgs, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Handled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHandled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryGetPosition<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(&self, relativeto: Param0, point: &mut super::super::super::Foundation::Point) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryGetPosition)(::core::mem::transmute_copy(this), relativeto.into_param().abi(), point, &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for ContextRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContextRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContextRequestedEventArgs {}
impl ::core::fmt::Debug for ContextRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContextRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContextRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.ContextRequestedEventArgs;{42618e0a-1cb6-46fb-8374-0aec68aa5e51})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ContextRequestedEventArgs {
    type Vtable = IContextRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IContextRequestedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContextRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.ContextRequestedEventArgs";
}
impl ::core::convert::From<ContextRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ContextRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContextRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ContextRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ContextRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ContextRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ContextRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ContextRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContextRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ContextRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ContextRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ContextRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ContextRequestedEventArgs> for super::RoutedEventArgs {
    fn from(value: ContextRequestedEventArgs) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ContextRequestedEventArgs> for super::RoutedEventArgs {
    fn from(value: &ContextRequestedEventArgs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for ContextRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for &ContextRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
unsafe impl ::core::marker::Send for ContextRequestedEventArgs {}
unsafe impl ::core::marker::Sync for ContextRequestedEventArgs {}
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
pub struct DoubleTappedEventHandler(pub ::windows::core::IUnknown);
impl DoubleTappedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<DoubleTappedRoutedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = DoubleTappedEventHandlerBox::<F> { vtable: &DoubleTappedEventHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, DoubleTappedRoutedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Invoke)(::core::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct DoubleTappedEventHandlerBox<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<DoubleTappedRoutedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const DoubleTappedEventHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<DoubleTappedRoutedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> DoubleTappedEventHandlerBox<F> {
    const VTABLE: DoubleTappedEventHandler_Vtbl = DoubleTappedEventHandler_Vtbl { base: ::windows::core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<DoubleTappedEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
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
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender), ::core::mem::transmute(&e)).into()
    }
}
impl ::core::clone::Clone for DoubleTappedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DoubleTappedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DoubleTappedEventHandler {}
impl ::core::fmt::Debug for DoubleTappedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DoubleTappedEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for DoubleTappedEventHandler {
    type Vtable = DoubleTappedEventHandler_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3124d025_04a7_4d45_825e_8204a624dbf4);
}
unsafe impl ::windows::core::RuntimeType for DoubleTappedEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{3124d025-04a7-4d45-825e-8204a624dbf4}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct DoubleTappedEventHandler_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
pub struct DoubleTappedRoutedEventArgs(::windows::core::IUnknown);
impl DoubleTappedRoutedEventArgs {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DoubleTappedRoutedEventArgs, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"Devices_Input\"`*"]
    #[cfg(feature = "Devices_Input")]
    pub fn PointerDeviceType(&self) -> ::windows::core::Result<super::super::super::Devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Devices::Input::PointerDeviceType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PointerDeviceType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Devices::Input::PointerDeviceType>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Handled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHandled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetPosition<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(&self, relativeto: Param0) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetPosition)(::core::mem::transmute_copy(this), relativeto.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
}
impl ::core::clone::Clone for DoubleTappedRoutedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DoubleTappedRoutedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DoubleTappedRoutedEventArgs {}
impl ::core::fmt::Debug for DoubleTappedRoutedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DoubleTappedRoutedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DoubleTappedRoutedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.DoubleTappedRoutedEventArgs;{af404424-26df-44f4-8714-9359249b62d3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for DoubleTappedRoutedEventArgs {
    type Vtable = IDoubleTappedRoutedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IDoubleTappedRoutedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DoubleTappedRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.DoubleTappedRoutedEventArgs";
}
impl ::core::convert::From<DoubleTappedRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: DoubleTappedRoutedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DoubleTappedRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &DoubleTappedRoutedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DoubleTappedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DoubleTappedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DoubleTappedRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: DoubleTappedRoutedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DoubleTappedRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &DoubleTappedRoutedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DoubleTappedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DoubleTappedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DoubleTappedRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: DoubleTappedRoutedEventArgs) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&DoubleTappedRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: &DoubleTappedRoutedEventArgs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for DoubleTappedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for &DoubleTappedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
unsafe impl ::core::marker::Send for DoubleTappedRoutedEventArgs {}
unsafe impl ::core::marker::Sync for DoubleTappedRoutedEventArgs {}
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
pub struct ExecuteRequestedEventArgs(::windows::core::IUnknown);
impl ExecuteRequestedEventArgs {
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn Parameter(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Parameter)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
impl ::core::clone::Clone for ExecuteRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ExecuteRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ExecuteRequestedEventArgs {}
impl ::core::fmt::Debug for ExecuteRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ExecuteRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ExecuteRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.ExecuteRequestedEventArgs;{e07fa734-a0b6-5755-9e87-24f54cca9372})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ExecuteRequestedEventArgs {
    type Vtable = IExecuteRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IExecuteRequestedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ExecuteRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.ExecuteRequestedEventArgs";
}
impl ::core::convert::From<ExecuteRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ExecuteRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ExecuteRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ExecuteRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ExecuteRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ExecuteRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ExecuteRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ExecuteRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ExecuteRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ExecuteRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ExecuteRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ExecuteRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ExecuteRequestedEventArgs {}
unsafe impl ::core::marker::Sync for ExecuteRequestedEventArgs {}
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
pub struct FindNextElementOptions(::windows::core::IUnknown);
impl FindNextElementOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<FindNextElementOptions, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn SearchRoot(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SearchRoot)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn SetSearchRoot<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSearchRoot)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ExclusionRect(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExclusionRect)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetExclusionRect<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Rect>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetExclusionRect)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn HintRect(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HintRect)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetHintRect<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Rect>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHintRect)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn XYFocusNavigationStrategyOverride(&self) -> ::windows::core::Result<XYFocusNavigationStrategyOverride> {
        let this = self;
        unsafe {
            let mut result__: XYFocusNavigationStrategyOverride = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XYFocusNavigationStrategyOverride)(::core::mem::transmute_copy(this), &mut result__).from_abi::<XYFocusNavigationStrategyOverride>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn SetXYFocusNavigationStrategyOverride(&self, value: XYFocusNavigationStrategyOverride) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetXYFocusNavigationStrategyOverride)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::clone::Clone for FindNextElementOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FindNextElementOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FindNextElementOptions {}
impl ::core::fmt::Debug for FindNextElementOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FindNextElementOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FindNextElementOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.FindNextElementOptions;{d88ae22b-46c2-41fc-897e-b5961977b89d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for FindNextElementOptions {
    type Vtable = IFindNextElementOptions_Vtbl;
    const IID: ::windows::core::GUID = <IFindNextElementOptions as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for FindNextElementOptions {
    const NAME: &'static str = "Windows.UI.Xaml.Input.FindNextElementOptions";
}
impl ::core::convert::From<FindNextElementOptions> for ::windows::core::IUnknown {
    fn from(value: FindNextElementOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FindNextElementOptions> for ::windows::core::IUnknown {
    fn from(value: &FindNextElementOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FindNextElementOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a FindNextElementOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FindNextElementOptions> for ::windows::core::IInspectable {
    fn from(value: FindNextElementOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FindNextElementOptions> for ::windows::core::IInspectable {
    fn from(value: &FindNextElementOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FindNextElementOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a FindNextElementOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for FindNextElementOptions {}
unsafe impl ::core::marker::Sync for FindNextElementOptions {}
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FocusInputDeviceKind(pub i32);
impl FocusInputDeviceKind {
    pub const None: Self = Self(0i32);
    pub const Mouse: Self = Self(1i32);
    pub const Touch: Self = Self(2i32);
    pub const Pen: Self = Self(3i32);
    pub const Keyboard: Self = Self(4i32);
    pub const GameController: Self = Self(5i32);
}
impl ::core::marker::Copy for FocusInputDeviceKind {}
impl ::core::clone::Clone for FocusInputDeviceKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FocusInputDeviceKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FocusInputDeviceKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for FocusInputDeviceKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FocusInputDeviceKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FocusInputDeviceKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Input.FocusInputDeviceKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
pub struct FocusManager(::windows::core::IUnknown);
impl FocusManager {
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn GetFocusedElement() -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IFocusManagerStatics(|this| unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetFocusedElement)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn TryMoveFocus(focusnavigationdirection: FocusNavigationDirection) -> ::windows::core::Result<bool> {
        Self::IFocusManagerStatics2(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryMoveFocus)(::core::mem::transmute_copy(this), focusnavigationdirection, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn FindNextFocusableElement(focusnavigationdirection: FocusNavigationDirection) -> ::windows::core::Result<super::UIElement> {
        Self::IFocusManagerStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindNextFocusableElement)(::core::mem::transmute_copy(this), focusnavigationdirection, &mut result__).from_abi::<super::UIElement>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FindNextFocusableElementWithHint<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Rect>>(focusnavigationdirection: FocusNavigationDirection, hintrect: Param1) -> ::windows::core::Result<super::UIElement> {
        Self::IFocusManagerStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindNextFocusableElementWithHint)(::core::mem::transmute_copy(this), focusnavigationdirection, hintrect.into_param().abi(), &mut result__).from_abi::<super::UIElement>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn TryMoveFocusWithOptions<'a, Param1: ::windows::core::IntoParam<'a, FindNextElementOptions>>(focusnavigationdirection: FocusNavigationDirection, focusnavigationoptions: Param1) -> ::windows::core::Result<bool> {
        Self::IFocusManagerStatics4(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryMoveFocusWithOptions)(::core::mem::transmute_copy(this), focusnavigationdirection, focusnavigationoptions.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn FindNextElement(focusnavigationdirection: FocusNavigationDirection) -> ::windows::core::Result<super::DependencyObject> {
        Self::IFocusManagerStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindNextElement)(::core::mem::transmute_copy(this), focusnavigationdirection, &mut result__).from_abi::<super::DependencyObject>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn FindFirstFocusableElement<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(searchscope: Param0) -> ::windows::core::Result<super::DependencyObject> {
        Self::IFocusManagerStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindFirstFocusableElement)(::core::mem::transmute_copy(this), searchscope.into_param().abi(), &mut result__).from_abi::<super::DependencyObject>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn FindLastFocusableElement<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(searchscope: Param0) -> ::windows::core::Result<super::DependencyObject> {
        Self::IFocusManagerStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindLastFocusableElement)(::core::mem::transmute_copy(this), searchscope.into_param().abi(), &mut result__).from_abi::<super::DependencyObject>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn FindNextElementWithOptions<'a, Param1: ::windows::core::IntoParam<'a, FindNextElementOptions>>(focusnavigationdirection: FocusNavigationDirection, focusnavigationoptions: Param1) -> ::windows::core::Result<super::DependencyObject> {
        Self::IFocusManagerStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindNextElementWithOptions)(::core::mem::transmute_copy(this), focusnavigationdirection, focusnavigationoptions.into_param().abi(), &mut result__).from_abi::<super::DependencyObject>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryFocusAsync<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: super::FocusState) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<FocusMovementResult>> {
        Self::IFocusManagerStatics5(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryFocusAsync)(::core::mem::transmute_copy(this), element.into_param().abi(), value, &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<FocusMovementResult>>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryMoveFocusAsync(focusnavigationdirection: FocusNavigationDirection) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<FocusMovementResult>> {
        Self::IFocusManagerStatics5(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryMoveFocusAsync)(::core::mem::transmute_copy(this), focusnavigationdirection, &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<FocusMovementResult>>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryMoveFocusWithOptionsAsync<'a, Param1: ::windows::core::IntoParam<'a, FindNextElementOptions>>(focusnavigationdirection: FocusNavigationDirection, focusnavigationoptions: Param1) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<FocusMovementResult>> {
        Self::IFocusManagerStatics5(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryMoveFocusWithOptionsAsync)(::core::mem::transmute_copy(this), focusnavigationdirection, focusnavigationoptions.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<FocusMovementResult>>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GotFocus<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventHandler<FocusManagerGotFocusEventArgs>>>(handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        Self::IFocusManagerStatics6(|this| unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GotFocus)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveGotFocus<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IFocusManagerStatics6(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveGotFocus)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LostFocus<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventHandler<FocusManagerLostFocusEventArgs>>>(handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        Self::IFocusManagerStatics6(|this| unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LostFocus)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveLostFocus<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IFocusManagerStatics6(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveLostFocus)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GettingFocus<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventHandler<GettingFocusEventArgs>>>(handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        Self::IFocusManagerStatics6(|this| unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GettingFocus)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveGettingFocus<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IFocusManagerStatics6(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveGettingFocus)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LosingFocus<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventHandler<LosingFocusEventArgs>>>(handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        Self::IFocusManagerStatics6(|this| unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LosingFocus)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveLosingFocus<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IFocusManagerStatics6(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveLosingFocus)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn GetFocusedElement2<'a, Param0: ::windows::core::IntoParam<'a, super::XamlRoot>>(xamlroot: Param0) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IFocusManagerStatics7(|this| unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetFocusedElement)(::core::mem::transmute_copy(this), xamlroot.into_param().abi(), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IFocusManagerStatics<R, F: FnOnce(&IFocusManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<FocusManager, IFocusManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IFocusManagerStatics2<R, F: FnOnce(&IFocusManagerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<FocusManager, IFocusManagerStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IFocusManagerStatics3<R, F: FnOnce(&IFocusManagerStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<FocusManager, IFocusManagerStatics3> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IFocusManagerStatics4<R, F: FnOnce(&IFocusManagerStatics4) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<FocusManager, IFocusManagerStatics4> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IFocusManagerStatics5<R, F: FnOnce(&IFocusManagerStatics5) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<FocusManager, IFocusManagerStatics5> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IFocusManagerStatics6<R, F: FnOnce(&IFocusManagerStatics6) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<FocusManager, IFocusManagerStatics6> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IFocusManagerStatics7<R, F: FnOnce(&IFocusManagerStatics7) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<FocusManager, IFocusManagerStatics7> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for FocusManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FocusManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FocusManager {}
impl ::core::fmt::Debug for FocusManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FocusManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FocusManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.FocusManager;{c843f50b-3b83-4da1-9d6f-557c1169f341})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for FocusManager {
    type Vtable = IFocusManager_Vtbl;
    const IID: ::windows::core::GUID = <IFocusManager as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for FocusManager {
    const NAME: &'static str = "Windows.UI.Xaml.Input.FocusManager";
}
impl ::core::convert::From<FocusManager> for ::windows::core::IUnknown {
    fn from(value: FocusManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FocusManager> for ::windows::core::IUnknown {
    fn from(value: &FocusManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FocusManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a FocusManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FocusManager> for ::windows::core::IInspectable {
    fn from(value: FocusManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FocusManager> for ::windows::core::IInspectable {
    fn from(value: &FocusManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FocusManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a FocusManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for FocusManager {}
unsafe impl ::core::marker::Sync for FocusManager {}
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
pub struct FocusManagerGotFocusEventArgs(::windows::core::IUnknown);
impl FocusManagerGotFocusEventArgs {
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn NewFocusedElement(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NewFocusedElement)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn CorrelationId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CorrelationId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
}
impl ::core::clone::Clone for FocusManagerGotFocusEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FocusManagerGotFocusEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FocusManagerGotFocusEventArgs {}
impl ::core::fmt::Debug for FocusManagerGotFocusEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FocusManagerGotFocusEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FocusManagerGotFocusEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.FocusManagerGotFocusEventArgs;{97aa5d83-535b-507a-868e-62b706f06b61})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for FocusManagerGotFocusEventArgs {
    type Vtable = IFocusManagerGotFocusEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IFocusManagerGotFocusEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for FocusManagerGotFocusEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.FocusManagerGotFocusEventArgs";
}
impl ::core::convert::From<FocusManagerGotFocusEventArgs> for ::windows::core::IUnknown {
    fn from(value: FocusManagerGotFocusEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FocusManagerGotFocusEventArgs> for ::windows::core::IUnknown {
    fn from(value: &FocusManagerGotFocusEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FocusManagerGotFocusEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a FocusManagerGotFocusEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FocusManagerGotFocusEventArgs> for ::windows::core::IInspectable {
    fn from(value: FocusManagerGotFocusEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FocusManagerGotFocusEventArgs> for ::windows::core::IInspectable {
    fn from(value: &FocusManagerGotFocusEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FocusManagerGotFocusEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a FocusManagerGotFocusEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for FocusManagerGotFocusEventArgs {}
unsafe impl ::core::marker::Sync for FocusManagerGotFocusEventArgs {}
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
pub struct FocusManagerLostFocusEventArgs(::windows::core::IUnknown);
impl FocusManagerLostFocusEventArgs {
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn OldFocusedElement(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OldFocusedElement)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn CorrelationId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CorrelationId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
}
impl ::core::clone::Clone for FocusManagerLostFocusEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FocusManagerLostFocusEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FocusManagerLostFocusEventArgs {}
impl ::core::fmt::Debug for FocusManagerLostFocusEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FocusManagerLostFocusEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FocusManagerLostFocusEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.FocusManagerLostFocusEventArgs;{3e157e7a-9578-5cd3-aaa8-051b3d391978})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for FocusManagerLostFocusEventArgs {
    type Vtable = IFocusManagerLostFocusEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IFocusManagerLostFocusEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for FocusManagerLostFocusEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.FocusManagerLostFocusEventArgs";
}
impl ::core::convert::From<FocusManagerLostFocusEventArgs> for ::windows::core::IUnknown {
    fn from(value: FocusManagerLostFocusEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FocusManagerLostFocusEventArgs> for ::windows::core::IUnknown {
    fn from(value: &FocusManagerLostFocusEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FocusManagerLostFocusEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a FocusManagerLostFocusEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FocusManagerLostFocusEventArgs> for ::windows::core::IInspectable {
    fn from(value: FocusManagerLostFocusEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FocusManagerLostFocusEventArgs> for ::windows::core::IInspectable {
    fn from(value: &FocusManagerLostFocusEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FocusManagerLostFocusEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a FocusManagerLostFocusEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for FocusManagerLostFocusEventArgs {}
unsafe impl ::core::marker::Sync for FocusManagerLostFocusEventArgs {}
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
pub struct FocusMovementResult(::windows::core::IUnknown);
impl FocusMovementResult {
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn Succeeded(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Succeeded)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for FocusMovementResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FocusMovementResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FocusMovementResult {}
impl ::core::fmt::Debug for FocusMovementResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FocusMovementResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FocusMovementResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.FocusMovementResult;{06dfead3-c2ae-44bb-bfab-9c73de8407a4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for FocusMovementResult {
    type Vtable = IFocusMovementResult_Vtbl;
    const IID: ::windows::core::GUID = <IFocusMovementResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for FocusMovementResult {
    const NAME: &'static str = "Windows.UI.Xaml.Input.FocusMovementResult";
}
impl ::core::convert::From<FocusMovementResult> for ::windows::core::IUnknown {
    fn from(value: FocusMovementResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FocusMovementResult> for ::windows::core::IUnknown {
    fn from(value: &FocusMovementResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FocusMovementResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a FocusMovementResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FocusMovementResult> for ::windows::core::IInspectable {
    fn from(value: FocusMovementResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FocusMovementResult> for ::windows::core::IInspectable {
    fn from(value: &FocusMovementResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FocusMovementResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a FocusMovementResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for FocusMovementResult {}
unsafe impl ::core::marker::Sync for FocusMovementResult {}
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FocusNavigationDirection(pub i32);
impl FocusNavigationDirection {
    pub const Next: Self = Self(0i32);
    pub const Previous: Self = Self(1i32);
    pub const Up: Self = Self(2i32);
    pub const Down: Self = Self(3i32);
    pub const Left: Self = Self(4i32);
    pub const Right: Self = Self(5i32);
    pub const None: Self = Self(6i32);
}
impl ::core::marker::Copy for FocusNavigationDirection {}
impl ::core::clone::Clone for FocusNavigationDirection {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FocusNavigationDirection {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FocusNavigationDirection {
    type Abi = Self;
}
impl ::core::fmt::Debug for FocusNavigationDirection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FocusNavigationDirection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FocusNavigationDirection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Input.FocusNavigationDirection;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
pub struct GettingFocusEventArgs(::windows::core::IUnknown);
impl GettingFocusEventArgs {
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn OldFocusedElement(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OldFocusedElement)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn NewFocusedElement(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NewFocusedElement)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn SetNewFocusedElement<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetNewFocusedElement)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn FocusState(&self) -> ::windows::core::Result<super::FocusState> {
        let this = self;
        unsafe {
            let mut result__: super::FocusState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FocusState)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::FocusState>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn Direction(&self) -> ::windows::core::Result<FocusNavigationDirection> {
        let this = self;
        unsafe {
            let mut result__: FocusNavigationDirection = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Direction)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FocusNavigationDirection>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Handled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHandled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn InputDevice(&self) -> ::windows::core::Result<FocusInputDeviceKind> {
        let this = self;
        unsafe {
            let mut result__: FocusInputDeviceKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InputDevice)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FocusInputDeviceKind>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn Cancel(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Cancel)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn SetCancel(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCancel)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn TryCancel(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IGettingFocusEventArgs2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryCancel)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn TrySetNewFocusedElement<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(&self, element: Param0) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IGettingFocusEventArgs2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TrySetNewFocusedElement)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn CorrelationId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = &::windows::core::Interface::cast::<IGettingFocusEventArgs3>(self)?;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CorrelationId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
}
impl ::core::clone::Clone for GettingFocusEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GettingFocusEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GettingFocusEventArgs {}
impl ::core::fmt::Debug for GettingFocusEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GettingFocusEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GettingFocusEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.GettingFocusEventArgs;{fa05b9ce-c67c-4be8-8fd4-c44d67877e0d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for GettingFocusEventArgs {
    type Vtable = IGettingFocusEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IGettingFocusEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for GettingFocusEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.GettingFocusEventArgs";
}
impl ::core::convert::From<GettingFocusEventArgs> for ::windows::core::IUnknown {
    fn from(value: GettingFocusEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GettingFocusEventArgs> for ::windows::core::IUnknown {
    fn from(value: &GettingFocusEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GettingFocusEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GettingFocusEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GettingFocusEventArgs> for ::windows::core::IInspectable {
    fn from(value: GettingFocusEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GettingFocusEventArgs> for ::windows::core::IInspectable {
    fn from(value: &GettingFocusEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GettingFocusEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GettingFocusEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GettingFocusEventArgs> for super::RoutedEventArgs {
    fn from(value: GettingFocusEventArgs) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&GettingFocusEventArgs> for super::RoutedEventArgs {
    fn from(value: &GettingFocusEventArgs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for GettingFocusEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for &GettingFocusEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
unsafe impl ::core::marker::Send for GettingFocusEventArgs {}
unsafe impl ::core::marker::Sync for GettingFocusEventArgs {}
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
pub struct HoldingEventHandler(pub ::windows::core::IUnknown);
impl HoldingEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<HoldingRoutedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = HoldingEventHandlerBox::<F> { vtable: &HoldingEventHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, HoldingRoutedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Invoke)(::core::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct HoldingEventHandlerBox<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<HoldingRoutedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const HoldingEventHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<HoldingRoutedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> HoldingEventHandlerBox<F> {
    const VTABLE: HoldingEventHandler_Vtbl = HoldingEventHandler_Vtbl { base: ::windows::core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<HoldingEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
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
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender), ::core::mem::transmute(&e)).into()
    }
}
impl ::core::clone::Clone for HoldingEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HoldingEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HoldingEventHandler {}
impl ::core::fmt::Debug for HoldingEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HoldingEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for HoldingEventHandler {
    type Vtable = HoldingEventHandler_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xecae8ccd_8e5e_4fbe_9846_30a6370afcdf);
}
unsafe impl ::windows::core::RuntimeType for HoldingEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{ecae8ccd-8e5e-4fbe-9846-30a6370afcdf}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct HoldingEventHandler_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
pub struct HoldingRoutedEventArgs(::windows::core::IUnknown);
impl HoldingRoutedEventArgs {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HoldingRoutedEventArgs, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"Devices_Input\"`*"]
    #[cfg(feature = "Devices_Input")]
    pub fn PointerDeviceType(&self) -> ::windows::core::Result<super::super::super::Devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Devices::Input::PointerDeviceType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PointerDeviceType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Devices::Input::PointerDeviceType>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"UI_Input\"`*"]
    #[cfg(feature = "UI_Input")]
    pub fn HoldingState(&self) -> ::windows::core::Result<super::super::Input::HoldingState> {
        let this = self;
        unsafe {
            let mut result__: super::super::Input::HoldingState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HoldingState)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Input::HoldingState>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Handled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHandled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetPosition<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(&self, relativeto: Param0) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetPosition)(::core::mem::transmute_copy(this), relativeto.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
}
impl ::core::clone::Clone for HoldingRoutedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HoldingRoutedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HoldingRoutedEventArgs {}
impl ::core::fmt::Debug for HoldingRoutedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HoldingRoutedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HoldingRoutedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.HoldingRoutedEventArgs;{c246ff23-d80d-44de-8db9-0d815e269ac0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for HoldingRoutedEventArgs {
    type Vtable = IHoldingRoutedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IHoldingRoutedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HoldingRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.HoldingRoutedEventArgs";
}
impl ::core::convert::From<HoldingRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: HoldingRoutedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HoldingRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &HoldingRoutedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HoldingRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a HoldingRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HoldingRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: HoldingRoutedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HoldingRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &HoldingRoutedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HoldingRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a HoldingRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HoldingRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: HoldingRoutedEventArgs) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&HoldingRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: &HoldingRoutedEventArgs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for HoldingRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for &HoldingRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
unsafe impl ::core::marker::Send for HoldingRoutedEventArgs {}
unsafe impl ::core::marker::Sync for HoldingRoutedEventArgs {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAccessKeyDisplayDismissedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAccessKeyDisplayDismissedEventArgs {
    type Vtable = IAccessKeyDisplayDismissedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8a610dc6_d72d_4ca8_9f66_556f35b513da);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessKeyDisplayDismissedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAccessKeyDisplayRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAccessKeyDisplayRequestedEventArgs {
    type Vtable = IAccessKeyDisplayRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c079e55_13fe_4d03_a61d_e12f06567286);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessKeyDisplayRequestedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub PressedKeys: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAccessKeyInvokedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAccessKeyInvokedEventArgs {
    type Vtable = IAccessKeyInvokedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcfe9cd97_c718_4091_b7dd_adf1c072b1e1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessKeyInvokedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAccessKeyManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAccessKeyManager {
    type Vtable = IAccessKeyManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xecc973b0_2ee9_4b1c_98d7_6e0e816d334b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessKeyManager_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAccessKeyManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAccessKeyManagerStatics {
    type Vtable = IAccessKeyManagerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4ca0efe6_d9c8_4ebc_b4c7_30d1838a81f1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessKeyManagerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsDisplayModeEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub IsDisplayModeEnabledChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsDisplayModeEnabledChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveIsDisplayModeEnabledChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveIsDisplayModeEnabledChanged: usize,
    pub ExitDisplayMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAccessKeyManagerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAccessKeyManagerStatics2 {
    type Vtable = IAccessKeyManagerStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x962bb594_2ab3_47c5_954b_7092f355f797);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessKeyManagerStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub AreKeyTipsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAreKeyTipsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanExecuteRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICanExecuteRequestedEventArgs {
    type Vtable = ICanExecuteRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc8e75256_1950_505d_993b_75907ef96830);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanExecuteRequestedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Parameter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CanExecute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetCanExecute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICharacterReceivedRoutedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICharacterReceivedRoutedEventArgs {
    type Vtable = ICharacterReceivedRoutedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7849fd82_48e4_444d_9419_93ab8892c107);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICharacterReceivedRoutedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Character: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Core")]
    pub KeyStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Core::CorePhysicalKeyStatus) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    KeyStatus: usize,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
pub struct ICommand(::windows::core::IUnknown);
impl ICommand {
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CanExecuteChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanExecuteChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCanExecuteChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveCanExecuteChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn CanExecute<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, parameter: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanExecute)(::core::mem::transmute_copy(this), parameter.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn Execute<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, parameter: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Execute)(::core::mem::transmute_copy(this), parameter.into_param().abi()).ok() }
    }
}
impl ::core::convert::From<ICommand> for ::windows::core::IUnknown {
    fn from(value: ICommand) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICommand> for ::windows::core::IUnknown {
    fn from(value: &ICommand) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ICommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ICommand> for ::windows::core::IInspectable {
    fn from(value: ICommand) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICommand> for ::windows::core::IInspectable {
    fn from(value: &ICommand) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ICommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ICommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICommand {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICommand {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICommand {}
impl ::core::fmt::Debug for ICommand {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICommand").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ICommand {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{e5af3542-ca67-4081-995b-709dd13792df}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ICommand {
    type Vtable = ICommand_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe5af3542_ca67_4081_995b_709dd13792df);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommand_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub CanExecuteChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CanExecuteChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCanExecuteChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCanExecuteChanged: usize,
    pub CanExecute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameter: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Execute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameter: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContextRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IContextRequestedEventArgs {
    type Vtable = IContextRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x42618e0a_1cb6_46fb_8374_0aec68aa5e51);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContextRequestedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TryGetPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relativeto: ::windows::core::RawPtr, point: *mut super::super::super::Foundation::Point, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryGetPosition: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDoubleTappedRoutedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDoubleTappedRoutedEventArgs {
    type Vtable = IDoubleTappedRoutedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaf404424_26df_44f4_8714_9359249b62d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDoubleTappedRoutedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Devices_Input")]
    pub PointerDeviceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Input"))]
    PointerDeviceType: usize,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relativeto: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetPosition: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IExecuteRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IExecuteRequestedEventArgs {
    type Vtable = IExecuteRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe07fa734_a0b6_5755_9e87_24f54cca9372);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExecuteRequestedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Parameter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFindNextElementOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFindNextElementOptions {
    type Vtable = IFindNextElementOptions_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd88ae22b_46c2_41fc_897e_b5961977b89d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFindNextElementOptions_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SearchRoot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetSearchRoot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ExclusionRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ExclusionRect: usize,
    #[cfg(feature = "Foundation")]
    pub SetExclusionRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetExclusionRect: usize,
    #[cfg(feature = "Foundation")]
    pub HintRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HintRect: usize,
    #[cfg(feature = "Foundation")]
    pub SetHintRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetHintRect: usize,
    pub XYFocusNavigationStrategyOverride: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut XYFocusNavigationStrategyOverride) -> ::windows::core::HRESULT,
    pub SetXYFocusNavigationStrategyOverride: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: XYFocusNavigationStrategyOverride) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFocusManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFocusManager {
    type Vtable = IFocusManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc843f50b_3b83_4da1_9d6f_557c1169f341);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusManager_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFocusManagerGotFocusEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFocusManagerGotFocusEventArgs {
    type Vtable = IFocusManagerGotFocusEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x97aa5d83_535b_507a_868e_62b706f06b61);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusManagerGotFocusEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub NewFocusedElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CorrelationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFocusManagerLostFocusEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFocusManagerLostFocusEventArgs {
    type Vtable = IFocusManagerLostFocusEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3e157e7a_9578_5cd3_aaa8_051b3d391978);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusManagerLostFocusEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub OldFocusedElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CorrelationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFocusManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFocusManagerStatics {
    type Vtable = IFocusManagerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1eccd326_8182_4482_826a_0918e9ed9af7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusManagerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetFocusedElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFocusManagerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFocusManagerStatics2 {
    type Vtable = IFocusManagerStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa920d761_dd87_4f31_beda_ef417fe7c04a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusManagerStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub TryMoveFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, focusnavigationdirection: FocusNavigationDirection, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFocusManagerStatics3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFocusManagerStatics3 {
    type Vtable = IFocusManagerStatics3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x60805ebf_b149_417d_83f1_baeb560e2a47);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusManagerStatics3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub FindNextFocusableElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, focusnavigationdirection: FocusNavigationDirection, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FindNextFocusableElementWithHint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, focusnavigationdirection: FocusNavigationDirection, hintrect: super::super::super::Foundation::Rect, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FindNextFocusableElementWithHint: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFocusManagerStatics4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFocusManagerStatics4 {
    type Vtable = IFocusManagerStatics4_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x29276e9c_1c6c_414a_ba1c_96efd5962bcd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusManagerStatics4_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub TryMoveFocusWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, focusnavigationdirection: FocusNavigationDirection, focusnavigationoptions: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub FindNextElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, focusnavigationdirection: FocusNavigationDirection, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub FindFirstFocusableElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, searchscope: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub FindLastFocusableElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, searchscope: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub FindNextElementWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, focusnavigationdirection: FocusNavigationDirection, focusnavigationoptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFocusManagerStatics5(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFocusManagerStatics5 {
    type Vtable = IFocusManagerStatics5_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x280edc61_207a_4d7b_b98f_ce165e1b2015);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusManagerStatics5_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub TryFocusAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: super::FocusState, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryFocusAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryMoveFocusAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, focusnavigationdirection: FocusNavigationDirection, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryMoveFocusAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryMoveFocusWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, focusnavigationdirection: FocusNavigationDirection, focusnavigationoptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryMoveFocusWithOptionsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFocusManagerStatics6(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFocusManagerStatics6 {
    type Vtable = IFocusManagerStatics6_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3546a1b6_20bf_5007_929d_e6d32e16afe4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusManagerStatics6_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub GotFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GotFocus: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveGotFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveGotFocus: usize,
    #[cfg(feature = "Foundation")]
    pub LostFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LostFocus: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveLostFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveLostFocus: usize,
    #[cfg(feature = "Foundation")]
    pub GettingFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GettingFocus: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveGettingFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveGettingFocus: usize,
    #[cfg(feature = "Foundation")]
    pub LosingFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LosingFocus: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveLosingFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveLosingFocus: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFocusManagerStatics7(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFocusManagerStatics7 {
    type Vtable = IFocusManagerStatics7_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x95d6fa97_f0fc_5c32_b29d_07c04ec966b0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusManagerStatics7_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetFocusedElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xamlroot: ::windows::core::RawPtr, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFocusMovementResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFocusMovementResult {
    type Vtable = IFocusMovementResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x06dfead3_c2ae_44bb_bfab_9c73de8407a4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusMovementResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Succeeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGettingFocusEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGettingFocusEventArgs {
    type Vtable = IGettingFocusEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfa05b9ce_c67c_4be8_8fd4_c44d67877e0d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGettingFocusEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub OldFocusedElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub NewFocusedElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetNewFocusedElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub FocusState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::FocusState) -> ::windows::core::HRESULT,
    pub Direction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FocusNavigationDirection) -> ::windows::core::HRESULT,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub InputDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FocusInputDeviceKind) -> ::windows::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetCancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGettingFocusEventArgs2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGettingFocusEventArgs2 {
    type Vtable = IGettingFocusEventArgs2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x88754d7b_b4b9_4959_8bce_89bf212ed4eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGettingFocusEventArgs2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub TryCancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub TrySetNewFocusedElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGettingFocusEventArgs3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGettingFocusEventArgs3 {
    type Vtable = IGettingFocusEventArgs3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e024891_db3f_5e78_b75a_62bfc3510735);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGettingFocusEventArgs3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CorrelationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHoldingRoutedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHoldingRoutedEventArgs {
    type Vtable = IHoldingRoutedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc246ff23_d80d_44de_8db9_0d815e269ac0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHoldingRoutedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Devices_Input")]
    pub PointerDeviceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Input"))]
    PointerDeviceType: usize,
    #[cfg(feature = "UI_Input")]
    pub HoldingState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Input::HoldingState) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Input"))]
    HoldingState: usize,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relativeto: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetPosition: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInertiaExpansionBehavior(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInertiaExpansionBehavior {
    type Vtable = IInertiaExpansionBehavior_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x751d87e5_8d42_44c5_965e_3cd30cc9d6f7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInertiaExpansionBehavior_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub DesiredDeceleration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetDesiredDeceleration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub DesiredExpansion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetDesiredExpansion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInertiaRotationBehavior(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInertiaRotationBehavior {
    type Vtable = IInertiaRotationBehavior_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x424cfb2e_bbfd_4625_ae78_20c65bf1efaf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInertiaRotationBehavior_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub DesiredDeceleration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetDesiredDeceleration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub DesiredRotation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetDesiredRotation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInertiaTranslationBehavior(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInertiaTranslationBehavior {
    type Vtable = IInertiaTranslationBehavior_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x45d3a512_3b32_4882_a4c2_ecfa2d4b6df0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInertiaTranslationBehavior_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub DesiredDeceleration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetDesiredDeceleration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub DesiredDisplacement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetDesiredDisplacement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInputScope(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInputScope {
    type Vtable = IInputScope_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c0f85f3_f9d8_4220_b666_045d074d9bfa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputScope_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Names: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Names: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInputScopeName(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInputScopeName {
    type Vtable = IInputScopeName_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfd3e6997_08fb_4cba_a021_792d7589fd5a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputScopeName_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub NameValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InputScopeNameValue) -> ::windows::core::HRESULT,
    pub SetNameValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: InputScopeNameValue) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInputScopeNameFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInputScopeNameFactory {
    type Vtable = IInputScopeNameFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4a40bb52_4bd7_4e54_8617_1cda8a1eda7f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputScopeNameFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namevalue: InputScopeNameValue, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyRoutedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IKeyRoutedEventArgs {
    type Vtable = IKeyRoutedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd4cd3dfe_4079_42e9_a39a_3095d3f049c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyRoutedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "System")]
    pub Key: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::System::VirtualKey) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    Key: usize,
    #[cfg(feature = "UI_Core")]
    pub KeyStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Core::CorePhysicalKeyStatus) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    KeyStatus: usize,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyRoutedEventArgs2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IKeyRoutedEventArgs2 {
    type Vtable = IKeyRoutedEventArgs2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1b02d57a_9634_4f14_91b2_133e42fdb3cd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyRoutedEventArgs2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "System")]
    pub OriginalKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::System::VirtualKey) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    OriginalKey: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyRoutedEventArgs3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IKeyRoutedEventArgs3 {
    type Vtable = IKeyRoutedEventArgs3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2779f5b4_ca41_411b_a8ef_f4fc78e78057);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyRoutedEventArgs3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyboardAccelerator(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IKeyboardAccelerator {
    type Vtable = IKeyboardAccelerator_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x92e6181e_19ae_465a_9b3c_a71ee9ea7420);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyboardAccelerator_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "System")]
    pub Key: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::System::VirtualKey) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    Key: usize,
    #[cfg(feature = "System")]
    pub SetKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::System::VirtualKey) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetKey: usize,
    #[cfg(feature = "System")]
    pub Modifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    Modifiers: usize,
    #[cfg(feature = "System")]
    pub SetModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetModifiers: usize,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub ScopeOwner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetScopeOwner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Invoked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Invoked: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveInvoked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveInvoked: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyboardAcceleratorFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IKeyboardAcceleratorFactory {
    type Vtable = IKeyboardAcceleratorFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x44d88a99_4bfd_4a47_a893_515f388623f6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyboardAcceleratorFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyboardAcceleratorInvokedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IKeyboardAcceleratorInvokedEventArgs {
    type Vtable = IKeyboardAcceleratorInvokedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc00b03f2_04e7_4415_b17d_d76b9490de2b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyboardAcceleratorInvokedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub Element: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyboardAcceleratorInvokedEventArgs2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IKeyboardAcceleratorInvokedEventArgs2 {
    type Vtable = IKeyboardAcceleratorInvokedEventArgs2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbefca4b8_5907_48ee_8e21_9c969078fa11);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyboardAcceleratorInvokedEventArgs2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub KeyboardAccelerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyboardAcceleratorStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IKeyboardAcceleratorStatics {
    type Vtable = IKeyboardAcceleratorStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3bd43d51_9bb3_456d_bf15_804adfb86261);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyboardAcceleratorStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub KeyProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub ModifiersProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub IsEnabledProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub ScopeOwnerProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILosingFocusEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILosingFocusEventArgs {
    type Vtable = ILosingFocusEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf9f683c7_d789_472b_aa93_6d4105e6dabe);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILosingFocusEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub OldFocusedElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub NewFocusedElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetNewFocusedElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub FocusState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::FocusState) -> ::windows::core::HRESULT,
    pub Direction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FocusNavigationDirection) -> ::windows::core::HRESULT,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub InputDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FocusInputDeviceKind) -> ::windows::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetCancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILosingFocusEventArgs2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILosingFocusEventArgs2 {
    type Vtable = ILosingFocusEventArgs2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0493fad9_c27f_469f_8e62_52b3a4f7cd54);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILosingFocusEventArgs2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub TryCancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub TrySetNewFocusedElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILosingFocusEventArgs3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILosingFocusEventArgs3 {
    type Vtable = ILosingFocusEventArgs3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc98900bd_0b79_566e_ad1f_436fa513ae22);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILosingFocusEventArgs3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CorrelationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IManipulationCompletedRoutedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IManipulationCompletedRoutedEventArgs {
    type Vtable = IManipulationCompletedRoutedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5ad9b23_2f41_498e_8319_015ee8a75346);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationCompletedRoutedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Container: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    pub IsInertial: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "UI_Input"))]
    pub Cumulative: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Input::ManipulationDelta) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Input")))]
    Cumulative: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Input"))]
    pub Velocities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Input::ManipulationVelocities) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Input")))]
    Velocities: usize,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Input")]
    pub PointerDeviceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Input"))]
    PointerDeviceType: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IManipulationDeltaRoutedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IManipulationDeltaRoutedEventArgs {
    type Vtable = IManipulationDeltaRoutedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x400d5794_4c6f_491d_82d6_3517109399c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationDeltaRoutedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Container: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    pub IsInertial: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "UI_Input"))]
    pub Delta: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Input::ManipulationDelta) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Input")))]
    Delta: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Input"))]
    pub Cumulative: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Input::ManipulationDelta) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Input")))]
    Cumulative: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Input"))]
    pub Velocities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Input::ManipulationVelocities) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Input")))]
    Velocities: usize,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Input")]
    pub PointerDeviceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Input"))]
    PointerDeviceType: usize,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IManipulationInertiaStartingRoutedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IManipulationInertiaStartingRoutedEventArgs {
    type Vtable = IManipulationInertiaStartingRoutedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x246a91a9_ca43_4c0b_acef_81e8b8147520);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationInertiaStartingRoutedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Container: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub ExpansionBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetExpansionBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub RotationBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetRotationBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub TranslationBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetTranslationBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Input")]
    pub PointerDeviceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Input"))]
    PointerDeviceType: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Input"))]
    pub Delta: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Input::ManipulationDelta) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Input")))]
    Delta: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Input"))]
    pub Cumulative: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Input::ManipulationDelta) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Input")))]
    Cumulative: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Input"))]
    pub Velocities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Input::ManipulationVelocities) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Input")))]
    Velocities: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IManipulationPivot(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IManipulationPivot {
    type Vtable = IManipulationPivot_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2e3838a5_e6c2_4998_82ac_18748b141666);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationPivot_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Center: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Center: usize,
    #[cfg(feature = "Foundation")]
    pub SetCenter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetCenter: usize,
    pub Radius: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetRadius: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IManipulationPivotFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IManipulationPivotFactory {
    type Vtable = IManipulationPivotFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d05b039_3702_4396_ad9b_a825efa63a3b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationPivotFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub CreateInstanceWithCenterAndRadius: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, center: super::super::super::Foundation::Point, radius: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateInstanceWithCenterAndRadius: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IManipulationStartedRoutedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IManipulationStartedRoutedEventArgs {
    type Vtable = IManipulationStartedRoutedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5db1aa05_9f80_48b6_ae6c_4f119de8ff13);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationStartedRoutedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Container: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Input")]
    pub PointerDeviceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Input"))]
    PointerDeviceType: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Input"))]
    pub Cumulative: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Input::ManipulationDelta) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Input")))]
    Cumulative: usize,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IManipulationStartedRoutedEventArgsFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IManipulationStartedRoutedEventArgsFactory {
    type Vtable = IManipulationStartedRoutedEventArgsFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x84c1daa7_7272_4463_b6c3_a40b9ba151fc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationStartedRoutedEventArgsFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IManipulationStartingRoutedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IManipulationStartingRoutedEventArgs {
    type Vtable = IManipulationStartingRoutedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x18d636b7_53a4_4c15_a498_f3a9ca212a42);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationStartingRoutedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Mode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ManipulationModes) -> ::windows::core::HRESULT,
    pub SetMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ManipulationModes) -> ::windows::core::HRESULT,
    pub Container: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetContainer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Pivot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetPivot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INoFocusCandidateFoundEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for INoFocusCandidateFoundEventArgs {
    type Vtable = INoFocusCandidateFoundEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xec3601a7_1007_48f9_b6b3_ed0bea53937d);
}
#[repr(C)]
#[doc(hidden)]
pub struct INoFocusCandidateFoundEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Direction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FocusNavigationDirection) -> ::windows::core::HRESULT,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub InputDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FocusInputDeviceKind) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPointer(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPointer {
    type Vtable = IPointer_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5ee8f39f_747d_4171_90e6_cd37a9dffb11);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointer_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub PointerId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Input")]
    pub PointerDeviceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Input"))]
    PointerDeviceType: usize,
    pub IsInContact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsInRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPointerRoutedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPointerRoutedEventArgs {
    type Vtable = IPointerRoutedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xda628f0a_9752_49e2_bde2_49eccab9194d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerRoutedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Pointer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")]
    pub KeyModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    KeyModifiers: usize,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Input")]
    pub GetCurrentPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relativeto: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Input"))]
    GetCurrentPoint: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Input"))]
    pub GetIntermediatePoints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relativeto: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Input")))]
    GetIntermediatePoints: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPointerRoutedEventArgs2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPointerRoutedEventArgs2 {
    type Vtable = IPointerRoutedEventArgs2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0821f294_1de6_4711_ba7c_8d4b8b0911d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerRoutedEventArgs2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsGenerated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProcessKeyboardAcceleratorEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IProcessKeyboardAcceleratorEventArgs {
    type Vtable = IProcessKeyboardAcceleratorEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb79c216_972b_440c_9b83_2b4198dcf09d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessKeyboardAcceleratorEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "System")]
    pub Key: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::System::VirtualKey) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    Key: usize,
    #[cfg(feature = "System")]
    pub Modifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    Modifiers: usize,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRightTappedRoutedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRightTappedRoutedEventArgs {
    type Vtable = IRightTappedRoutedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6834869d_7bd5_4033_b237_172f79abe393);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRightTappedRoutedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Devices_Input")]
    pub PointerDeviceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Input"))]
    PointerDeviceType: usize,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relativeto: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetPosition: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStandardUICommand(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStandardUICommand {
    type Vtable = IStandardUICommand_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd2bf7f43_0504_52d0_8aa6_0cb0f756eb27);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStandardUICommand_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StandardUICommandKind) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStandardUICommand2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStandardUICommand2 {
    type Vtable = IStandardUICommand2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe3666069_f9e4_51eb_885b_7a620a0782ea);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStandardUICommand2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SetKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: StandardUICommandKind) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStandardUICommandFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStandardUICommandFactory {
    type Vtable = IStandardUICommandFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f1a7590_dce1_56e4_ab63_f5ce3ce4ebf6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStandardUICommandFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateInstanceWithKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, kind: StandardUICommandKind, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStandardUICommandStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStandardUICommandStatics {
    type Vtable = IStandardUICommandStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7ea87ed9_2978_5533_9b2e_6759ce88569f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStandardUICommandStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub KindProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITappedRoutedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITappedRoutedEventArgs {
    type Vtable = ITappedRoutedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa099e6be_e624_459a_bb1d_e05c73e2cc66);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITappedRoutedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Devices_Input")]
    pub PointerDeviceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Input"))]
    PointerDeviceType: usize,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relativeto: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetPosition: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlUICommand(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXamlUICommand {
    type Vtable = IXamlUICommand_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8494f8d4_ead1_5f01_ad2e_a8cad4f9dc0e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlUICommand_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Label: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetLabel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub IconSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    IconSource: usize,
    #[cfg(feature = "UI_Xaml_Controls")]
    pub SetIconSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Controls"))]
    SetIconSource: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub KeyboardAccelerators: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    KeyboardAccelerators: usize,
    pub AccessKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetAccessKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Command: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ExecuteRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ExecuteRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveExecuteRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveExecuteRequested: usize,
    #[cfg(feature = "Foundation")]
    pub CanExecuteRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CanExecuteRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCanExecuteRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCanExecuteRequested: usize,
    pub NotifyCanExecuteChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlUICommandFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXamlUICommandFactory {
    type Vtable = IXamlUICommandFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1eec08c3_e061_5e10_9f2a_2baa840885c2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlUICommandFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlUICommandStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXamlUICommandStatics {
    type Vtable = IXamlUICommandStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x66bc457c_1a0c_58ed_876e_71533f966db6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlUICommandStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub LabelProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub IconSourceProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub KeyboardAcceleratorsProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub AccessKeyProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub DescriptionProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CommandProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
pub struct InertiaExpansionBehavior(::windows::core::IUnknown);
impl InertiaExpansionBehavior {
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn DesiredDeceleration(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DesiredDeceleration)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn SetDesiredDeceleration(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDesiredDeceleration)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn DesiredExpansion(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DesiredExpansion)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn SetDesiredExpansion(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDesiredExpansion)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::clone::Clone for InertiaExpansionBehavior {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InertiaExpansionBehavior {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InertiaExpansionBehavior {}
impl ::core::fmt::Debug for InertiaExpansionBehavior {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InertiaExpansionBehavior").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InertiaExpansionBehavior {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.InertiaExpansionBehavior;{751d87e5-8d42-44c5-965e-3cd30cc9d6f7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for InertiaExpansionBehavior {
    type Vtable = IInertiaExpansionBehavior_Vtbl;
    const IID: ::windows::core::GUID = <IInertiaExpansionBehavior as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InertiaExpansionBehavior {
    const NAME: &'static str = "Windows.UI.Xaml.Input.InertiaExpansionBehavior";
}
impl ::core::convert::From<InertiaExpansionBehavior> for ::windows::core::IUnknown {
    fn from(value: InertiaExpansionBehavior) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InertiaExpansionBehavior> for ::windows::core::IUnknown {
    fn from(value: &InertiaExpansionBehavior) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InertiaExpansionBehavior {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InertiaExpansionBehavior {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InertiaExpansionBehavior> for ::windows::core::IInspectable {
    fn from(value: InertiaExpansionBehavior) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InertiaExpansionBehavior> for ::windows::core::IInspectable {
    fn from(value: &InertiaExpansionBehavior) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InertiaExpansionBehavior {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InertiaExpansionBehavior {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for InertiaExpansionBehavior {}
unsafe impl ::core::marker::Sync for InertiaExpansionBehavior {}
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
pub struct InertiaRotationBehavior(::windows::core::IUnknown);
impl InertiaRotationBehavior {
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn DesiredDeceleration(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DesiredDeceleration)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn SetDesiredDeceleration(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDesiredDeceleration)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn DesiredRotation(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DesiredRotation)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn SetDesiredRotation(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDesiredRotation)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::clone::Clone for InertiaRotationBehavior {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InertiaRotationBehavior {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InertiaRotationBehavior {}
impl ::core::fmt::Debug for InertiaRotationBehavior {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InertiaRotationBehavior").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InertiaRotationBehavior {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.InertiaRotationBehavior;{424cfb2e-bbfd-4625-ae78-20c65bf1efaf})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for InertiaRotationBehavior {
    type Vtable = IInertiaRotationBehavior_Vtbl;
    const IID: ::windows::core::GUID = <IInertiaRotationBehavior as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InertiaRotationBehavior {
    const NAME: &'static str = "Windows.UI.Xaml.Input.InertiaRotationBehavior";
}
impl ::core::convert::From<InertiaRotationBehavior> for ::windows::core::IUnknown {
    fn from(value: InertiaRotationBehavior) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InertiaRotationBehavior> for ::windows::core::IUnknown {
    fn from(value: &InertiaRotationBehavior) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InertiaRotationBehavior {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InertiaRotationBehavior {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InertiaRotationBehavior> for ::windows::core::IInspectable {
    fn from(value: InertiaRotationBehavior) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InertiaRotationBehavior> for ::windows::core::IInspectable {
    fn from(value: &InertiaRotationBehavior) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InertiaRotationBehavior {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InertiaRotationBehavior {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for InertiaRotationBehavior {}
unsafe impl ::core::marker::Sync for InertiaRotationBehavior {}
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
pub struct InertiaTranslationBehavior(::windows::core::IUnknown);
impl InertiaTranslationBehavior {
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn DesiredDeceleration(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DesiredDeceleration)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn SetDesiredDeceleration(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDesiredDeceleration)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn DesiredDisplacement(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DesiredDisplacement)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn SetDesiredDisplacement(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDesiredDisplacement)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::clone::Clone for InertiaTranslationBehavior {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InertiaTranslationBehavior {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InertiaTranslationBehavior {}
impl ::core::fmt::Debug for InertiaTranslationBehavior {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InertiaTranslationBehavior").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InertiaTranslationBehavior {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.InertiaTranslationBehavior;{45d3a512-3b32-4882-a4c2-ecfa2d4b6df0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for InertiaTranslationBehavior {
    type Vtable = IInertiaTranslationBehavior_Vtbl;
    const IID: ::windows::core::GUID = <IInertiaTranslationBehavior as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InertiaTranslationBehavior {
    const NAME: &'static str = "Windows.UI.Xaml.Input.InertiaTranslationBehavior";
}
impl ::core::convert::From<InertiaTranslationBehavior> for ::windows::core::IUnknown {
    fn from(value: InertiaTranslationBehavior) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InertiaTranslationBehavior> for ::windows::core::IUnknown {
    fn from(value: &InertiaTranslationBehavior) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InertiaTranslationBehavior {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InertiaTranslationBehavior {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InertiaTranslationBehavior> for ::windows::core::IInspectable {
    fn from(value: InertiaTranslationBehavior) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InertiaTranslationBehavior> for ::windows::core::IInspectable {
    fn from(value: &InertiaTranslationBehavior) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InertiaTranslationBehavior {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InertiaTranslationBehavior {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for InertiaTranslationBehavior {}
unsafe impl ::core::marker::Sync for InertiaTranslationBehavior {}
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
pub struct InputScope(::windows::core::IUnknown);
impl InputScope {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<InputScope, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Names(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<InputScopeName>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Names)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<InputScopeName>>(result__)
        }
    }
}
impl ::core::clone::Clone for InputScope {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InputScope {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InputScope {}
impl ::core::fmt::Debug for InputScope {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InputScope").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InputScope {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.InputScope;{5c0f85f3-f9d8-4220-b666-045d074d9bfa})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for InputScope {
    type Vtable = IInputScope_Vtbl;
    const IID: ::windows::core::GUID = <IInputScope as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InputScope {
    const NAME: &'static str = "Windows.UI.Xaml.Input.InputScope";
}
impl ::core::convert::From<InputScope> for ::windows::core::IUnknown {
    fn from(value: InputScope) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InputScope> for ::windows::core::IUnknown {
    fn from(value: &InputScope) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InputScope {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InputScope {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InputScope> for ::windows::core::IInspectable {
    fn from(value: InputScope) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InputScope> for ::windows::core::IInspectable {
    fn from(value: &InputScope) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InputScope {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InputScope {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InputScope> for super::DependencyObject {
    fn from(value: InputScope) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&InputScope> for super::DependencyObject {
    fn from(value: &InputScope) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for InputScope {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &InputScope {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for InputScope {}
unsafe impl ::core::marker::Sync for InputScope {}
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
pub struct InputScopeName(::windows::core::IUnknown);
impl InputScopeName {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<InputScopeName, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn NameValue(&self) -> ::windows::core::Result<InputScopeNameValue> {
        let this = self;
        unsafe {
            let mut result__: InputScopeNameValue = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NameValue)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InputScopeNameValue>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn SetNameValue(&self, value: InputScopeNameValue) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetNameValue)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn CreateInstance(namevalue: InputScopeNameValue) -> ::windows::core::Result<InputScopeName> {
        Self::IInputScopeNameFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(::core::mem::transmute_copy(this), namevalue, &mut result__).from_abi::<InputScopeName>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IInputScopeNameFactory<R, F: FnOnce(&IInputScopeNameFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<InputScopeName, IInputScopeNameFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for InputScopeName {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InputScopeName {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InputScopeName {}
impl ::core::fmt::Debug for InputScopeName {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InputScopeName").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InputScopeName {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.InputScopeName;{fd3e6997-08fb-4cba-a021-792d7589fd5a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for InputScopeName {
    type Vtable = IInputScopeName_Vtbl;
    const IID: ::windows::core::GUID = <IInputScopeName as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InputScopeName {
    const NAME: &'static str = "Windows.UI.Xaml.Input.InputScopeName";
}
impl ::core::convert::From<InputScopeName> for ::windows::core::IUnknown {
    fn from(value: InputScopeName) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InputScopeName> for ::windows::core::IUnknown {
    fn from(value: &InputScopeName) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InputScopeName {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InputScopeName {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InputScopeName> for ::windows::core::IInspectable {
    fn from(value: InputScopeName) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InputScopeName> for ::windows::core::IInspectable {
    fn from(value: &InputScopeName) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InputScopeName {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InputScopeName {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InputScopeName> for super::DependencyObject {
    fn from(value: InputScopeName) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&InputScopeName> for super::DependencyObject {
    fn from(value: &InputScopeName) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for InputScopeName {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &InputScopeName {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for InputScopeName {}
unsafe impl ::core::marker::Sync for InputScopeName {}
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InputScopeNameValue(pub i32);
impl InputScopeNameValue {
    pub const Default: Self = Self(0i32);
    pub const Url: Self = Self(1i32);
    pub const EmailSmtpAddress: Self = Self(5i32);
    pub const PersonalFullName: Self = Self(7i32);
    pub const CurrencyAmountAndSymbol: Self = Self(20i32);
    pub const CurrencyAmount: Self = Self(21i32);
    pub const DateMonthNumber: Self = Self(23i32);
    pub const DateDayNumber: Self = Self(24i32);
    pub const DateYear: Self = Self(25i32);
    pub const Digits: Self = Self(28i32);
    pub const Number: Self = Self(29i32);
    pub const Password: Self = Self(31i32);
    pub const TelephoneNumber: Self = Self(32i32);
    pub const TelephoneCountryCode: Self = Self(33i32);
    pub const TelephoneAreaCode: Self = Self(34i32);
    pub const TelephoneLocalNumber: Self = Self(35i32);
    pub const TimeHour: Self = Self(37i32);
    pub const TimeMinutesOrSeconds: Self = Self(38i32);
    pub const NumberFullWidth: Self = Self(39i32);
    pub const AlphanumericHalfWidth: Self = Self(40i32);
    pub const AlphanumericFullWidth: Self = Self(41i32);
    pub const Hiragana: Self = Self(44i32);
    pub const KatakanaHalfWidth: Self = Self(45i32);
    pub const KatakanaFullWidth: Self = Self(46i32);
    pub const Hanja: Self = Self(47i32);
    pub const HangulHalfWidth: Self = Self(48i32);
    pub const HangulFullWidth: Self = Self(49i32);
    pub const Search: Self = Self(50i32);
    pub const Formula: Self = Self(51i32);
    pub const SearchIncremental: Self = Self(52i32);
    pub const ChineseHalfWidth: Self = Self(53i32);
    pub const ChineseFullWidth: Self = Self(54i32);
    pub const NativeScript: Self = Self(55i32);
    pub const Text: Self = Self(57i32);
    pub const Chat: Self = Self(58i32);
    pub const NameOrPhoneNumber: Self = Self(59i32);
    pub const EmailNameOrAddress: Self = Self(60i32);
    pub const Private: Self = Self(61i32);
    pub const Maps: Self = Self(62i32);
    pub const NumericPassword: Self = Self(63i32);
    pub const NumericPin: Self = Self(64i32);
    pub const AlphanumericPin: Self = Self(65i32);
    pub const FormulaNumber: Self = Self(67i32);
    pub const ChatWithoutEmoji: Self = Self(68i32);
}
impl ::core::marker::Copy for InputScopeNameValue {}
impl ::core::clone::Clone for InputScopeNameValue {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InputScopeNameValue {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for InputScopeNameValue {
    type Abi = Self;
}
impl ::core::fmt::Debug for InputScopeNameValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InputScopeNameValue").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InputScopeNameValue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Input.InputScopeNameValue;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
pub struct KeyEventHandler(pub ::windows::core::IUnknown);
impl KeyEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<KeyRoutedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = KeyEventHandlerBox::<F> { vtable: &KeyEventHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, KeyRoutedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Invoke)(::core::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct KeyEventHandlerBox<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<KeyRoutedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const KeyEventHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<KeyRoutedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> KeyEventHandlerBox<F> {
    const VTABLE: KeyEventHandler_Vtbl = KeyEventHandler_Vtbl { base: ::windows::core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<KeyEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
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
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender), ::core::mem::transmute(&e)).into()
    }
}
impl ::core::clone::Clone for KeyEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for KeyEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for KeyEventHandler {}
impl ::core::fmt::Debug for KeyEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KeyEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for KeyEventHandler {
    type Vtable = KeyEventHandler_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7c63d2e5_7a0e_4e12_b96a_7715aa6ff1c8);
}
unsafe impl ::windows::core::RuntimeType for KeyEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{7c63d2e5-7a0e-4e12-b96a-7715aa6ff1c8}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct KeyEventHandler_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
pub struct KeyRoutedEventArgs(::windows::core::IUnknown);
impl KeyRoutedEventArgs {
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn Key(&self) -> ::windows::core::Result<super::super::super::System::VirtualKey> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::System::VirtualKey = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Key)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::VirtualKey>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn KeyStatus(&self) -> ::windows::core::Result<super::super::Core::CorePhysicalKeyStatus> {
        let this = self;
        unsafe {
            let mut result__: super::super::Core::CorePhysicalKeyStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyStatus)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Core::CorePhysicalKeyStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Handled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHandled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn OriginalKey(&self) -> ::windows::core::Result<super::super::super::System::VirtualKey> {
        let this = &::windows::core::Interface::cast::<IKeyRoutedEventArgs2>(self)?;
        unsafe {
            let mut result__: super::super::super::System::VirtualKey = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OriginalKey)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::VirtualKey>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IKeyRoutedEventArgs3>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for KeyRoutedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for KeyRoutedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for KeyRoutedEventArgs {}
impl ::core::fmt::Debug for KeyRoutedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KeyRoutedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for KeyRoutedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.KeyRoutedEventArgs;{d4cd3dfe-4079-42e9-a39a-3095d3f049c6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for KeyRoutedEventArgs {
    type Vtable = IKeyRoutedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IKeyRoutedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for KeyRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.KeyRoutedEventArgs";
}
impl ::core::convert::From<KeyRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: KeyRoutedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&KeyRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &KeyRoutedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for KeyRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a KeyRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<KeyRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: KeyRoutedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&KeyRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &KeyRoutedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for KeyRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a KeyRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<KeyRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: KeyRoutedEventArgs) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&KeyRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: &KeyRoutedEventArgs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for KeyRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for &KeyRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
unsafe impl ::core::marker::Send for KeyRoutedEventArgs {}
unsafe impl ::core::marker::Sync for KeyRoutedEventArgs {}
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct KeyTipPlacementMode(pub i32);
impl KeyTipPlacementMode {
    pub const Auto: Self = Self(0i32);
    pub const Bottom: Self = Self(1i32);
    pub const Top: Self = Self(2i32);
    pub const Left: Self = Self(3i32);
    pub const Right: Self = Self(4i32);
    pub const Center: Self = Self(5i32);
    pub const Hidden: Self = Self(6i32);
}
impl ::core::marker::Copy for KeyTipPlacementMode {}
impl ::core::clone::Clone for KeyTipPlacementMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KeyTipPlacementMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KeyTipPlacementMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for KeyTipPlacementMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KeyTipPlacementMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for KeyTipPlacementMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Input.KeyTipPlacementMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
pub struct KeyboardAccelerator(::windows::core::IUnknown);
impl KeyboardAccelerator {
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn Key(&self) -> ::windows::core::Result<super::super::super::System::VirtualKey> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::System::VirtualKey = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Key)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::VirtualKey>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn SetKey(&self, value: super::super::super::System::VirtualKey) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetKey)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn Modifiers(&self) -> ::windows::core::Result<super::super::super::System::VirtualKeyModifiers> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::System::VirtualKeyModifiers = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Modifiers)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::VirtualKeyModifiers>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn SetModifiers(&self, value: super::super::super::System::VirtualKeyModifiers) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetModifiers)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsEnabled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn ScopeOwner(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ScopeOwner)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn SetScopeOwner<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetScopeOwner)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Invoked<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<KeyboardAccelerator, KeyboardAcceleratorInvokedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Invoked)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveInvoked<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveInvoked)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn new() -> ::windows::core::Result<KeyboardAccelerator> {
        Self::IKeyboardAcceleratorFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(::core::mem::transmute_copy(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<KeyboardAccelerator>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn compose<T: ::windows::core::Compose>(compose: T) -> ::windows::core::Result<KeyboardAccelerator> {
        Self::IKeyboardAcceleratorFactory(|this| unsafe {
            let (derived__, base__) = ::windows::core::Compose::compose(compose);
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(::core::mem::transmute_copy(this), ::core::mem::transmute_copy(&derived__), base__ as *mut _ as _, &mut result__).from_abi::<KeyboardAccelerator>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn KeyProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IKeyboardAcceleratorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn ModifiersProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IKeyboardAcceleratorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ModifiersProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn IsEnabledProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IKeyboardAcceleratorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsEnabledProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn ScopeOwnerProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IKeyboardAcceleratorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ScopeOwnerProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IKeyboardAcceleratorFactory<R, F: FnOnce(&IKeyboardAcceleratorFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<KeyboardAccelerator, IKeyboardAcceleratorFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IKeyboardAcceleratorStatics<R, F: FnOnce(&IKeyboardAcceleratorStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<KeyboardAccelerator, IKeyboardAcceleratorStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for KeyboardAccelerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for KeyboardAccelerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for KeyboardAccelerator {}
impl ::core::fmt::Debug for KeyboardAccelerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KeyboardAccelerator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for KeyboardAccelerator {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.KeyboardAccelerator;{92e6181e-19ae-465a-9b3c-a71ee9ea7420})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for KeyboardAccelerator {
    type Vtable = IKeyboardAccelerator_Vtbl;
    const IID: ::windows::core::GUID = <IKeyboardAccelerator as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for KeyboardAccelerator {
    const NAME: &'static str = "Windows.UI.Xaml.Input.KeyboardAccelerator";
}
impl ::core::convert::From<KeyboardAccelerator> for ::windows::core::IUnknown {
    fn from(value: KeyboardAccelerator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&KeyboardAccelerator> for ::windows::core::IUnknown {
    fn from(value: &KeyboardAccelerator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for KeyboardAccelerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a KeyboardAccelerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<KeyboardAccelerator> for ::windows::core::IInspectable {
    fn from(value: KeyboardAccelerator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&KeyboardAccelerator> for ::windows::core::IInspectable {
    fn from(value: &KeyboardAccelerator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for KeyboardAccelerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a KeyboardAccelerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<KeyboardAccelerator> for super::DependencyObject {
    fn from(value: KeyboardAccelerator) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&KeyboardAccelerator> for super::DependencyObject {
    fn from(value: &KeyboardAccelerator) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for KeyboardAccelerator {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &KeyboardAccelerator {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for KeyboardAccelerator {}
unsafe impl ::core::marker::Sync for KeyboardAccelerator {}
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
pub struct KeyboardAcceleratorInvokedEventArgs(::windows::core::IUnknown);
impl KeyboardAcceleratorInvokedEventArgs {
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Handled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHandled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn Element(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Element)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn KeyboardAccelerator(&self) -> ::windows::core::Result<KeyboardAccelerator> {
        let this = &::windows::core::Interface::cast::<IKeyboardAcceleratorInvokedEventArgs2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyboardAccelerator)(::core::mem::transmute_copy(this), &mut result__).from_abi::<KeyboardAccelerator>(result__)
        }
    }
}
impl ::core::clone::Clone for KeyboardAcceleratorInvokedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for KeyboardAcceleratorInvokedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for KeyboardAcceleratorInvokedEventArgs {}
impl ::core::fmt::Debug for KeyboardAcceleratorInvokedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KeyboardAcceleratorInvokedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for KeyboardAcceleratorInvokedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.KeyboardAcceleratorInvokedEventArgs;{c00b03f2-04e7-4415-b17d-d76b9490de2b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for KeyboardAcceleratorInvokedEventArgs {
    type Vtable = IKeyboardAcceleratorInvokedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IKeyboardAcceleratorInvokedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for KeyboardAcceleratorInvokedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.KeyboardAcceleratorInvokedEventArgs";
}
impl ::core::convert::From<KeyboardAcceleratorInvokedEventArgs> for ::windows::core::IUnknown {
    fn from(value: KeyboardAcceleratorInvokedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&KeyboardAcceleratorInvokedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &KeyboardAcceleratorInvokedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for KeyboardAcceleratorInvokedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a KeyboardAcceleratorInvokedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<KeyboardAcceleratorInvokedEventArgs> for ::windows::core::IInspectable {
    fn from(value: KeyboardAcceleratorInvokedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&KeyboardAcceleratorInvokedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &KeyboardAcceleratorInvokedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for KeyboardAcceleratorInvokedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a KeyboardAcceleratorInvokedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for KeyboardAcceleratorInvokedEventArgs {}
unsafe impl ::core::marker::Sync for KeyboardAcceleratorInvokedEventArgs {}
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct KeyboardAcceleratorPlacementMode(pub i32);
impl KeyboardAcceleratorPlacementMode {
    pub const Auto: Self = Self(0i32);
    pub const Hidden: Self = Self(1i32);
}
impl ::core::marker::Copy for KeyboardAcceleratorPlacementMode {}
impl ::core::clone::Clone for KeyboardAcceleratorPlacementMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KeyboardAcceleratorPlacementMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KeyboardAcceleratorPlacementMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for KeyboardAcceleratorPlacementMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KeyboardAcceleratorPlacementMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for KeyboardAcceleratorPlacementMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Input.KeyboardAcceleratorPlacementMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct KeyboardNavigationMode(pub i32);
impl KeyboardNavigationMode {
    pub const Local: Self = Self(0i32);
    pub const Cycle: Self = Self(1i32);
    pub const Once: Self = Self(2i32);
}
impl ::core::marker::Copy for KeyboardNavigationMode {}
impl ::core::clone::Clone for KeyboardNavigationMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KeyboardNavigationMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KeyboardNavigationMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for KeyboardNavigationMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KeyboardNavigationMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for KeyboardNavigationMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Input.KeyboardNavigationMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
pub struct LosingFocusEventArgs(::windows::core::IUnknown);
impl LosingFocusEventArgs {
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn OldFocusedElement(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OldFocusedElement)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn NewFocusedElement(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NewFocusedElement)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn SetNewFocusedElement<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetNewFocusedElement)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn FocusState(&self) -> ::windows::core::Result<super::FocusState> {
        let this = self;
        unsafe {
            let mut result__: super::FocusState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FocusState)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::FocusState>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn Direction(&self) -> ::windows::core::Result<FocusNavigationDirection> {
        let this = self;
        unsafe {
            let mut result__: FocusNavigationDirection = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Direction)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FocusNavigationDirection>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Handled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHandled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn InputDevice(&self) -> ::windows::core::Result<FocusInputDeviceKind> {
        let this = self;
        unsafe {
            let mut result__: FocusInputDeviceKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InputDevice)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FocusInputDeviceKind>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn Cancel(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Cancel)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn SetCancel(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCancel)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn TryCancel(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ILosingFocusEventArgs2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryCancel)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn TrySetNewFocusedElement<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(&self, element: Param0) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ILosingFocusEventArgs2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TrySetNewFocusedElement)(::core::mem::transmute_copy(this), element.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn CorrelationId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = &::windows::core::Interface::cast::<ILosingFocusEventArgs3>(self)?;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CorrelationId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
}
impl ::core::clone::Clone for LosingFocusEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LosingFocusEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LosingFocusEventArgs {}
impl ::core::fmt::Debug for LosingFocusEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LosingFocusEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LosingFocusEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.LosingFocusEventArgs;{f9f683c7-d789-472b-aa93-6d4105e6dabe})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for LosingFocusEventArgs {
    type Vtable = ILosingFocusEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <ILosingFocusEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LosingFocusEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.LosingFocusEventArgs";
}
impl ::core::convert::From<LosingFocusEventArgs> for ::windows::core::IUnknown {
    fn from(value: LosingFocusEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LosingFocusEventArgs> for ::windows::core::IUnknown {
    fn from(value: &LosingFocusEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LosingFocusEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LosingFocusEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LosingFocusEventArgs> for ::windows::core::IInspectable {
    fn from(value: LosingFocusEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LosingFocusEventArgs> for ::windows::core::IInspectable {
    fn from(value: &LosingFocusEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LosingFocusEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LosingFocusEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LosingFocusEventArgs> for super::RoutedEventArgs {
    fn from(value: LosingFocusEventArgs) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&LosingFocusEventArgs> for super::RoutedEventArgs {
    fn from(value: &LosingFocusEventArgs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for LosingFocusEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for &LosingFocusEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
unsafe impl ::core::marker::Send for LosingFocusEventArgs {}
unsafe impl ::core::marker::Sync for LosingFocusEventArgs {}
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
pub struct ManipulationCompletedEventHandler(pub ::windows::core::IUnknown);
impl ManipulationCompletedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<ManipulationCompletedRoutedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = ManipulationCompletedEventHandlerBox::<F> { vtable: &ManipulationCompletedEventHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, ManipulationCompletedRoutedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Invoke)(::core::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct ManipulationCompletedEventHandlerBox<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<ManipulationCompletedRoutedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const ManipulationCompletedEventHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<ManipulationCompletedRoutedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> ManipulationCompletedEventHandlerBox<F> {
    const VTABLE: ManipulationCompletedEventHandler_Vtbl = ManipulationCompletedEventHandler_Vtbl { base: ::windows::core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<ManipulationCompletedEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
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
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender), ::core::mem::transmute(&e)).into()
    }
}
impl ::core::clone::Clone for ManipulationCompletedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ManipulationCompletedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ManipulationCompletedEventHandler {}
impl ::core::fmt::Debug for ManipulationCompletedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ManipulationCompletedEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ManipulationCompletedEventHandler {
    type Vtable = ManipulationCompletedEventHandler_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38ef4b0f_14f8_42df_9a1e_a4bcc4af77f4);
}
unsafe impl ::windows::core::RuntimeType for ManipulationCompletedEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{38ef4b0f-14f8-42df-9a1e-a4bcc4af77f4}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ManipulationCompletedEventHandler_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
pub struct ManipulationCompletedRoutedEventArgs(::windows::core::IUnknown);
impl ManipulationCompletedRoutedEventArgs {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ManipulationCompletedRoutedEventArgs, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn Container(&self) -> ::windows::core::Result<super::UIElement> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Container)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::UIElement>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Position)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn IsInertial(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsInertial)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"Foundation\"`, `\"UI_Input\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Input"))]
    pub fn Cumulative(&self) -> ::windows::core::Result<super::super::Input::ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__: super::super::Input::ManipulationDelta = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Cumulative)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Input::ManipulationDelta>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"Foundation\"`, `\"UI_Input\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Input"))]
    pub fn Velocities(&self) -> ::windows::core::Result<super::super::Input::ManipulationVelocities> {
        let this = self;
        unsafe {
            let mut result__: super::super::Input::ManipulationVelocities = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Velocities)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Input::ManipulationVelocities>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Handled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHandled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"Devices_Input\"`*"]
    #[cfg(feature = "Devices_Input")]
    pub fn PointerDeviceType(&self) -> ::windows::core::Result<super::super::super::Devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Devices::Input::PointerDeviceType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PointerDeviceType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Devices::Input::PointerDeviceType>(result__)
        }
    }
}
impl ::core::clone::Clone for ManipulationCompletedRoutedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ManipulationCompletedRoutedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ManipulationCompletedRoutedEventArgs {}
impl ::core::fmt::Debug for ManipulationCompletedRoutedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ManipulationCompletedRoutedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ManipulationCompletedRoutedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.ManipulationCompletedRoutedEventArgs;{b5ad9b23-2f41-498e-8319-015ee8a75346})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ManipulationCompletedRoutedEventArgs {
    type Vtable = IManipulationCompletedRoutedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IManipulationCompletedRoutedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ManipulationCompletedRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.ManipulationCompletedRoutedEventArgs";
}
impl ::core::convert::From<ManipulationCompletedRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ManipulationCompletedRoutedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ManipulationCompletedRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ManipulationCompletedRoutedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ManipulationCompletedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ManipulationCompletedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ManipulationCompletedRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ManipulationCompletedRoutedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ManipulationCompletedRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ManipulationCompletedRoutedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ManipulationCompletedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ManipulationCompletedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ManipulationCompletedRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: ManipulationCompletedRoutedEventArgs) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ManipulationCompletedRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: &ManipulationCompletedRoutedEventArgs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for ManipulationCompletedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for &ManipulationCompletedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
unsafe impl ::core::marker::Send for ManipulationCompletedRoutedEventArgs {}
unsafe impl ::core::marker::Sync for ManipulationCompletedRoutedEventArgs {}
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
pub struct ManipulationDeltaEventHandler(pub ::windows::core::IUnknown);
impl ManipulationDeltaEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<ManipulationDeltaRoutedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = ManipulationDeltaEventHandlerBox::<F> { vtable: &ManipulationDeltaEventHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, ManipulationDeltaRoutedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Invoke)(::core::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct ManipulationDeltaEventHandlerBox<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<ManipulationDeltaRoutedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const ManipulationDeltaEventHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<ManipulationDeltaRoutedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> ManipulationDeltaEventHandlerBox<F> {
    const VTABLE: ManipulationDeltaEventHandler_Vtbl = ManipulationDeltaEventHandler_Vtbl { base: ::windows::core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<ManipulationDeltaEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
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
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender), ::core::mem::transmute(&e)).into()
    }
}
impl ::core::clone::Clone for ManipulationDeltaEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ManipulationDeltaEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ManipulationDeltaEventHandler {}
impl ::core::fmt::Debug for ManipulationDeltaEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ManipulationDeltaEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ManipulationDeltaEventHandler {
    type Vtable = ManipulationDeltaEventHandler_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa1160cb_dfb9_4c56_abdc_711b63c8eb94);
}
unsafe impl ::windows::core::RuntimeType for ManipulationDeltaEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{aa1160cb-dfb9-4c56-abdc-711b63c8eb94}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ManipulationDeltaEventHandler_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
pub struct ManipulationDeltaRoutedEventArgs(::windows::core::IUnknown);
impl ManipulationDeltaRoutedEventArgs {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ManipulationDeltaRoutedEventArgs, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn Container(&self) -> ::windows::core::Result<super::UIElement> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Container)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::UIElement>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Position)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn IsInertial(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsInertial)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"Foundation\"`, `\"UI_Input\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Input"))]
    pub fn Delta(&self) -> ::windows::core::Result<super::super::Input::ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__: super::super::Input::ManipulationDelta = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Delta)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Input::ManipulationDelta>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"Foundation\"`, `\"UI_Input\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Input"))]
    pub fn Cumulative(&self) -> ::windows::core::Result<super::super::Input::ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__: super::super::Input::ManipulationDelta = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Cumulative)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Input::ManipulationDelta>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"Foundation\"`, `\"UI_Input\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Input"))]
    pub fn Velocities(&self) -> ::windows::core::Result<super::super::Input::ManipulationVelocities> {
        let this = self;
        unsafe {
            let mut result__: super::super::Input::ManipulationVelocities = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Velocities)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Input::ManipulationVelocities>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Handled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHandled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"Devices_Input\"`*"]
    #[cfg(feature = "Devices_Input")]
    pub fn PointerDeviceType(&self) -> ::windows::core::Result<super::super::super::Devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Devices::Input::PointerDeviceType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PointerDeviceType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Devices::Input::PointerDeviceType>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn Complete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Complete)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for ManipulationDeltaRoutedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ManipulationDeltaRoutedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ManipulationDeltaRoutedEventArgs {}
impl ::core::fmt::Debug for ManipulationDeltaRoutedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ManipulationDeltaRoutedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ManipulationDeltaRoutedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.ManipulationDeltaRoutedEventArgs;{400d5794-4c6f-491d-82d6-3517109399c6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ManipulationDeltaRoutedEventArgs {
    type Vtable = IManipulationDeltaRoutedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IManipulationDeltaRoutedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ManipulationDeltaRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.ManipulationDeltaRoutedEventArgs";
}
impl ::core::convert::From<ManipulationDeltaRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ManipulationDeltaRoutedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ManipulationDeltaRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ManipulationDeltaRoutedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ManipulationDeltaRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ManipulationDeltaRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ManipulationDeltaRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ManipulationDeltaRoutedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ManipulationDeltaRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ManipulationDeltaRoutedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ManipulationDeltaRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ManipulationDeltaRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ManipulationDeltaRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: ManipulationDeltaRoutedEventArgs) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ManipulationDeltaRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: &ManipulationDeltaRoutedEventArgs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for ManipulationDeltaRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for &ManipulationDeltaRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
unsafe impl ::core::marker::Send for ManipulationDeltaRoutedEventArgs {}
unsafe impl ::core::marker::Sync for ManipulationDeltaRoutedEventArgs {}
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
pub struct ManipulationInertiaStartingEventHandler(pub ::windows::core::IUnknown);
impl ManipulationInertiaStartingEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<ManipulationInertiaStartingRoutedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = ManipulationInertiaStartingEventHandlerBox::<F> { vtable: &ManipulationInertiaStartingEventHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, ManipulationInertiaStartingRoutedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Invoke)(::core::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct ManipulationInertiaStartingEventHandlerBox<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<ManipulationInertiaStartingRoutedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const ManipulationInertiaStartingEventHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<ManipulationInertiaStartingRoutedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> ManipulationInertiaStartingEventHandlerBox<F> {
    const VTABLE: ManipulationInertiaStartingEventHandler_Vtbl = ManipulationInertiaStartingEventHandler_Vtbl { base: ::windows::core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<ManipulationInertiaStartingEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
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
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender), ::core::mem::transmute(&e)).into()
    }
}
impl ::core::clone::Clone for ManipulationInertiaStartingEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ManipulationInertiaStartingEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ManipulationInertiaStartingEventHandler {}
impl ::core::fmt::Debug for ManipulationInertiaStartingEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ManipulationInertiaStartingEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ManipulationInertiaStartingEventHandler {
    type Vtable = ManipulationInertiaStartingEventHandler_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd39d6322_7c9c_481b_827b_c8b2d9bb6fc7);
}
unsafe impl ::windows::core::RuntimeType for ManipulationInertiaStartingEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{d39d6322-7c9c-481b-827b-c8b2d9bb6fc7}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ManipulationInertiaStartingEventHandler_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
pub struct ManipulationInertiaStartingRoutedEventArgs(::windows::core::IUnknown);
impl ManipulationInertiaStartingRoutedEventArgs {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ManipulationInertiaStartingRoutedEventArgs, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn Container(&self) -> ::windows::core::Result<super::UIElement> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Container)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::UIElement>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn ExpansionBehavior(&self) -> ::windows::core::Result<InertiaExpansionBehavior> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExpansionBehavior)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InertiaExpansionBehavior>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn SetExpansionBehavior<'a, Param0: ::windows::core::IntoParam<'a, InertiaExpansionBehavior>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetExpansionBehavior)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn RotationBehavior(&self) -> ::windows::core::Result<InertiaRotationBehavior> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RotationBehavior)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InertiaRotationBehavior>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn SetRotationBehavior<'a, Param0: ::windows::core::IntoParam<'a, InertiaRotationBehavior>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRotationBehavior)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn TranslationBehavior(&self) -> ::windows::core::Result<InertiaTranslationBehavior> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TranslationBehavior)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InertiaTranslationBehavior>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn SetTranslationBehavior<'a, Param0: ::windows::core::IntoParam<'a, InertiaTranslationBehavior>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTranslationBehavior)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Handled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHandled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"Devices_Input\"`*"]
    #[cfg(feature = "Devices_Input")]
    pub fn PointerDeviceType(&self) -> ::windows::core::Result<super::super::super::Devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Devices::Input::PointerDeviceType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PointerDeviceType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Devices::Input::PointerDeviceType>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"Foundation\"`, `\"UI_Input\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Input"))]
    pub fn Delta(&self) -> ::windows::core::Result<super::super::Input::ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__: super::super::Input::ManipulationDelta = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Delta)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Input::ManipulationDelta>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"Foundation\"`, `\"UI_Input\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Input"))]
    pub fn Cumulative(&self) -> ::windows::core::Result<super::super::Input::ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__: super::super::Input::ManipulationDelta = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Cumulative)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Input::ManipulationDelta>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"Foundation\"`, `\"UI_Input\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Input"))]
    pub fn Velocities(&self) -> ::windows::core::Result<super::super::Input::ManipulationVelocities> {
        let this = self;
        unsafe {
            let mut result__: super::super::Input::ManipulationVelocities = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Velocities)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Input::ManipulationVelocities>(result__)
        }
    }
}
impl ::core::clone::Clone for ManipulationInertiaStartingRoutedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ManipulationInertiaStartingRoutedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ManipulationInertiaStartingRoutedEventArgs {}
impl ::core::fmt::Debug for ManipulationInertiaStartingRoutedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ManipulationInertiaStartingRoutedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ManipulationInertiaStartingRoutedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.ManipulationInertiaStartingRoutedEventArgs;{246a91a9-ca43-4c0b-acef-81e8b8147520})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ManipulationInertiaStartingRoutedEventArgs {
    type Vtable = IManipulationInertiaStartingRoutedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IManipulationInertiaStartingRoutedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ManipulationInertiaStartingRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.ManipulationInertiaStartingRoutedEventArgs";
}
impl ::core::convert::From<ManipulationInertiaStartingRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ManipulationInertiaStartingRoutedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ManipulationInertiaStartingRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ManipulationInertiaStartingRoutedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ManipulationInertiaStartingRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ManipulationInertiaStartingRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ManipulationInertiaStartingRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ManipulationInertiaStartingRoutedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ManipulationInertiaStartingRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ManipulationInertiaStartingRoutedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ManipulationInertiaStartingRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ManipulationInertiaStartingRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ManipulationInertiaStartingRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: ManipulationInertiaStartingRoutedEventArgs) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ManipulationInertiaStartingRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: &ManipulationInertiaStartingRoutedEventArgs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for ManipulationInertiaStartingRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for &ManipulationInertiaStartingRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
unsafe impl ::core::marker::Send for ManipulationInertiaStartingRoutedEventArgs {}
unsafe impl ::core::marker::Sync for ManipulationInertiaStartingRoutedEventArgs {}
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ManipulationModes(pub u32);
impl ManipulationModes {
    pub const None: Self = Self(0u32);
    pub const TranslateX: Self = Self(1u32);
    pub const TranslateY: Self = Self(2u32);
    pub const TranslateRailsX: Self = Self(4u32);
    pub const TranslateRailsY: Self = Self(8u32);
    pub const Rotate: Self = Self(16u32);
    pub const Scale: Self = Self(32u32);
    pub const TranslateInertia: Self = Self(64u32);
    pub const RotateInertia: Self = Self(128u32);
    pub const ScaleInertia: Self = Self(256u32);
    pub const All: Self = Self(65535u32);
    pub const System: Self = Self(65536u32);
}
impl ::core::marker::Copy for ManipulationModes {}
impl ::core::clone::Clone for ManipulationModes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ManipulationModes {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ManipulationModes {
    type Abi = Self;
}
impl ::core::fmt::Debug for ManipulationModes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ManipulationModes").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for ManipulationModes {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for ManipulationModes {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for ManipulationModes {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for ManipulationModes {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for ManipulationModes {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for ManipulationModes {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Input.ManipulationModes;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
pub struct ManipulationPivot(::windows::core::IUnknown);
impl ManipulationPivot {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ManipulationPivot, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Center(&self) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Center)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetCenter<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Point>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCenter)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn Radius(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Radius)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn SetRadius(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRadius)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateInstanceWithCenterAndRadius<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Point>>(center: Param0, radius: f64) -> ::windows::core::Result<ManipulationPivot> {
        Self::IManipulationPivotFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstanceWithCenterAndRadius)(::core::mem::transmute_copy(this), center.into_param().abi(), radius, &mut result__).from_abi::<ManipulationPivot>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IManipulationPivotFactory<R, F: FnOnce(&IManipulationPivotFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ManipulationPivot, IManipulationPivotFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ManipulationPivot {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ManipulationPivot {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ManipulationPivot {}
impl ::core::fmt::Debug for ManipulationPivot {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ManipulationPivot").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ManipulationPivot {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.ManipulationPivot;{2e3838a5-e6c2-4998-82ac-18748b141666})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ManipulationPivot {
    type Vtable = IManipulationPivot_Vtbl;
    const IID: ::windows::core::GUID = <IManipulationPivot as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ManipulationPivot {
    const NAME: &'static str = "Windows.UI.Xaml.Input.ManipulationPivot";
}
impl ::core::convert::From<ManipulationPivot> for ::windows::core::IUnknown {
    fn from(value: ManipulationPivot) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ManipulationPivot> for ::windows::core::IUnknown {
    fn from(value: &ManipulationPivot) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ManipulationPivot {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ManipulationPivot {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ManipulationPivot> for ::windows::core::IInspectable {
    fn from(value: ManipulationPivot) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ManipulationPivot> for ::windows::core::IInspectable {
    fn from(value: &ManipulationPivot) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ManipulationPivot {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ManipulationPivot {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ManipulationPivot {}
unsafe impl ::core::marker::Sync for ManipulationPivot {}
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
pub struct ManipulationStartedEventHandler(pub ::windows::core::IUnknown);
impl ManipulationStartedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<ManipulationStartedRoutedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = ManipulationStartedEventHandlerBox::<F> { vtable: &ManipulationStartedEventHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, ManipulationStartedRoutedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Invoke)(::core::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct ManipulationStartedEventHandlerBox<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<ManipulationStartedRoutedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const ManipulationStartedEventHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<ManipulationStartedRoutedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> ManipulationStartedEventHandlerBox<F> {
    const VTABLE: ManipulationStartedEventHandler_Vtbl = ManipulationStartedEventHandler_Vtbl { base: ::windows::core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<ManipulationStartedEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
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
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender), ::core::mem::transmute(&e)).into()
    }
}
impl ::core::clone::Clone for ManipulationStartedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ManipulationStartedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ManipulationStartedEventHandler {}
impl ::core::fmt::Debug for ManipulationStartedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ManipulationStartedEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ManipulationStartedEventHandler {
    type Vtable = ManipulationStartedEventHandler_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf88345f8_e0a3_4be2_b90c_dc20e6d8beb0);
}
unsafe impl ::windows::core::RuntimeType for ManipulationStartedEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{f88345f8-e0a3-4be2-b90c-dc20e6d8beb0}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ManipulationStartedEventHandler_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
pub struct ManipulationStartedRoutedEventArgs(::windows::core::IUnknown);
impl ManipulationStartedRoutedEventArgs {
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn Container(&self) -> ::windows::core::Result<super::UIElement> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Container)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::UIElement>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Position)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Handled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHandled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"Devices_Input\"`*"]
    #[cfg(feature = "Devices_Input")]
    pub fn PointerDeviceType(&self) -> ::windows::core::Result<super::super::super::Devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Devices::Input::PointerDeviceType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PointerDeviceType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Devices::Input::PointerDeviceType>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"Foundation\"`, `\"UI_Input\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Input"))]
    pub fn Cumulative(&self) -> ::windows::core::Result<super::super::Input::ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__: super::super::Input::ManipulationDelta = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Cumulative)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Input::ManipulationDelta>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn Complete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Complete)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn new() -> ::windows::core::Result<ManipulationStartedRoutedEventArgs> {
        Self::IManipulationStartedRoutedEventArgsFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(::core::mem::transmute_copy(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<ManipulationStartedRoutedEventArgs>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn compose<T: ::windows::core::Compose>(compose: T) -> ::windows::core::Result<ManipulationStartedRoutedEventArgs> {
        Self::IManipulationStartedRoutedEventArgsFactory(|this| unsafe {
            let (derived__, base__) = ::windows::core::Compose::compose(compose);
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(::core::mem::transmute_copy(this), ::core::mem::transmute_copy(&derived__), base__ as *mut _ as _, &mut result__).from_abi::<ManipulationStartedRoutedEventArgs>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IManipulationStartedRoutedEventArgsFactory<R, F: FnOnce(&IManipulationStartedRoutedEventArgsFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ManipulationStartedRoutedEventArgs, IManipulationStartedRoutedEventArgsFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ManipulationStartedRoutedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ManipulationStartedRoutedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ManipulationStartedRoutedEventArgs {}
impl ::core::fmt::Debug for ManipulationStartedRoutedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ManipulationStartedRoutedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ManipulationStartedRoutedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.ManipulationStartedRoutedEventArgs;{5db1aa05-9f80-48b6-ae6c-4f119de8ff13})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ManipulationStartedRoutedEventArgs {
    type Vtable = IManipulationStartedRoutedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IManipulationStartedRoutedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ManipulationStartedRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.ManipulationStartedRoutedEventArgs";
}
impl ::core::convert::From<ManipulationStartedRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ManipulationStartedRoutedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ManipulationStartedRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ManipulationStartedRoutedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ManipulationStartedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ManipulationStartedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ManipulationStartedRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ManipulationStartedRoutedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ManipulationStartedRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ManipulationStartedRoutedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ManipulationStartedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ManipulationStartedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ManipulationStartedRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: ManipulationStartedRoutedEventArgs) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ManipulationStartedRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: &ManipulationStartedRoutedEventArgs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for ManipulationStartedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for &ManipulationStartedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
unsafe impl ::core::marker::Send for ManipulationStartedRoutedEventArgs {}
unsafe impl ::core::marker::Sync for ManipulationStartedRoutedEventArgs {}
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
pub struct ManipulationStartingEventHandler(pub ::windows::core::IUnknown);
impl ManipulationStartingEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<ManipulationStartingRoutedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = ManipulationStartingEventHandlerBox::<F> { vtable: &ManipulationStartingEventHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, ManipulationStartingRoutedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Invoke)(::core::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct ManipulationStartingEventHandlerBox<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<ManipulationStartingRoutedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const ManipulationStartingEventHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<ManipulationStartingRoutedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> ManipulationStartingEventHandlerBox<F> {
    const VTABLE: ManipulationStartingEventHandler_Vtbl = ManipulationStartingEventHandler_Vtbl { base: ::windows::core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<ManipulationStartingEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
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
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender), ::core::mem::transmute(&e)).into()
    }
}
impl ::core::clone::Clone for ManipulationStartingEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ManipulationStartingEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ManipulationStartingEventHandler {}
impl ::core::fmt::Debug for ManipulationStartingEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ManipulationStartingEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ManipulationStartingEventHandler {
    type Vtable = ManipulationStartingEventHandler_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x10d0b04e_bfe4_42cb_823c_3fecd8770ef8);
}
unsafe impl ::windows::core::RuntimeType for ManipulationStartingEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{10d0b04e-bfe4-42cb-823c-3fecd8770ef8}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ManipulationStartingEventHandler_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
pub struct ManipulationStartingRoutedEventArgs(::windows::core::IUnknown);
impl ManipulationStartingRoutedEventArgs {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ManipulationStartingRoutedEventArgs, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn Mode(&self) -> ::windows::core::Result<ManipulationModes> {
        let this = self;
        unsafe {
            let mut result__: ManipulationModes = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Mode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ManipulationModes>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn SetMode(&self, value: ManipulationModes) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMode)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn Container(&self) -> ::windows::core::Result<super::UIElement> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Container)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::UIElement>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn SetContainer<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetContainer)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn Pivot(&self) -> ::windows::core::Result<ManipulationPivot> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Pivot)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ManipulationPivot>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn SetPivot<'a, Param0: ::windows::core::IntoParam<'a, ManipulationPivot>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPivot)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Handled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHandled)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::clone::Clone for ManipulationStartingRoutedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ManipulationStartingRoutedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ManipulationStartingRoutedEventArgs {}
impl ::core::fmt::Debug for ManipulationStartingRoutedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ManipulationStartingRoutedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ManipulationStartingRoutedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.ManipulationStartingRoutedEventArgs;{18d636b7-53a4-4c15-a498-f3a9ca212a42})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ManipulationStartingRoutedEventArgs {
    type Vtable = IManipulationStartingRoutedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IManipulationStartingRoutedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ManipulationStartingRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.ManipulationStartingRoutedEventArgs";
}
impl ::core::convert::From<ManipulationStartingRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ManipulationStartingRoutedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ManipulationStartingRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ManipulationStartingRoutedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ManipulationStartingRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ManipulationStartingRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ManipulationStartingRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ManipulationStartingRoutedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ManipulationStartingRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ManipulationStartingRoutedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ManipulationStartingRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ManipulationStartingRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ManipulationStartingRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: ManipulationStartingRoutedEventArgs) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ManipulationStartingRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: &ManipulationStartingRoutedEventArgs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for ManipulationStartingRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for &ManipulationStartingRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
unsafe impl ::core::marker::Send for ManipulationStartingRoutedEventArgs {}
unsafe impl ::core::marker::Sync for ManipulationStartingRoutedEventArgs {}
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
pub struct NoFocusCandidateFoundEventArgs(::windows::core::IUnknown);
impl NoFocusCandidateFoundEventArgs {
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn Direction(&self) -> ::windows::core::Result<FocusNavigationDirection> {
        let this = self;
        unsafe {
            let mut result__: FocusNavigationDirection = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Direction)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FocusNavigationDirection>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Handled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHandled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn InputDevice(&self) -> ::windows::core::Result<FocusInputDeviceKind> {
        let this = self;
        unsafe {
            let mut result__: FocusInputDeviceKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InputDevice)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FocusInputDeviceKind>(result__)
        }
    }
}
impl ::core::clone::Clone for NoFocusCandidateFoundEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for NoFocusCandidateFoundEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for NoFocusCandidateFoundEventArgs {}
impl ::core::fmt::Debug for NoFocusCandidateFoundEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NoFocusCandidateFoundEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for NoFocusCandidateFoundEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.NoFocusCandidateFoundEventArgs;{ec3601a7-1007-48f9-b6b3-ed0bea53937d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for NoFocusCandidateFoundEventArgs {
    type Vtable = INoFocusCandidateFoundEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <INoFocusCandidateFoundEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for NoFocusCandidateFoundEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.NoFocusCandidateFoundEventArgs";
}
impl ::core::convert::From<NoFocusCandidateFoundEventArgs> for ::windows::core::IUnknown {
    fn from(value: NoFocusCandidateFoundEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NoFocusCandidateFoundEventArgs> for ::windows::core::IUnknown {
    fn from(value: &NoFocusCandidateFoundEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for NoFocusCandidateFoundEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a NoFocusCandidateFoundEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<NoFocusCandidateFoundEventArgs> for ::windows::core::IInspectable {
    fn from(value: NoFocusCandidateFoundEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NoFocusCandidateFoundEventArgs> for ::windows::core::IInspectable {
    fn from(value: &NoFocusCandidateFoundEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for NoFocusCandidateFoundEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a NoFocusCandidateFoundEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<NoFocusCandidateFoundEventArgs> for super::RoutedEventArgs {
    fn from(value: NoFocusCandidateFoundEventArgs) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&NoFocusCandidateFoundEventArgs> for super::RoutedEventArgs {
    fn from(value: &NoFocusCandidateFoundEventArgs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for NoFocusCandidateFoundEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for &NoFocusCandidateFoundEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
unsafe impl ::core::marker::Send for NoFocusCandidateFoundEventArgs {}
unsafe impl ::core::marker::Sync for NoFocusCandidateFoundEventArgs {}
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
pub struct Pointer(::windows::core::IUnknown);
impl Pointer {
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn PointerId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PointerId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"Devices_Input\"`*"]
    #[cfg(feature = "Devices_Input")]
    pub fn PointerDeviceType(&self) -> ::windows::core::Result<super::super::super::Devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Devices::Input::PointerDeviceType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PointerDeviceType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Devices::Input::PointerDeviceType>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn IsInContact(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsInContact)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn IsInRange(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsInRange)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for Pointer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Pointer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Pointer {}
impl ::core::fmt::Debug for Pointer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Pointer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Pointer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.Pointer;{5ee8f39f-747d-4171-90e6-cd37a9dffb11})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for Pointer {
    type Vtable = IPointer_Vtbl;
    const IID: ::windows::core::GUID = <IPointer as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Pointer {
    const NAME: &'static str = "Windows.UI.Xaml.Input.Pointer";
}
impl ::core::convert::From<Pointer> for ::windows::core::IUnknown {
    fn from(value: Pointer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Pointer> for ::windows::core::IUnknown {
    fn from(value: &Pointer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Pointer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Pointer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Pointer> for ::windows::core::IInspectable {
    fn from(value: Pointer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Pointer> for ::windows::core::IInspectable {
    fn from(value: &Pointer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Pointer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Pointer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for Pointer {}
unsafe impl ::core::marker::Sync for Pointer {}
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
pub struct PointerEventHandler(pub ::windows::core::IUnknown);
impl PointerEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<PointerRoutedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = PointerEventHandlerBox::<F> { vtable: &PointerEventHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, PointerRoutedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Invoke)(::core::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct PointerEventHandlerBox<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<PointerRoutedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const PointerEventHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<PointerRoutedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> PointerEventHandlerBox<F> {
    const VTABLE: PointerEventHandler_Vtbl = PointerEventHandler_Vtbl { base: ::windows::core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<PointerEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
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
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender), ::core::mem::transmute(&e)).into()
    }
}
impl ::core::clone::Clone for PointerEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PointerEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PointerEventHandler {}
impl ::core::fmt::Debug for PointerEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PointerEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for PointerEventHandler {
    type Vtable = PointerEventHandler_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe4385929_c004_4bcf_8970_359486e39f88);
}
unsafe impl ::windows::core::RuntimeType for PointerEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{e4385929-c004-4bcf-8970-359486e39f88}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct PointerEventHandler_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
pub struct PointerRoutedEventArgs(::windows::core::IUnknown);
impl PointerRoutedEventArgs {
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn Pointer(&self) -> ::windows::core::Result<Pointer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Pointer)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Pointer>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn KeyModifiers(&self) -> ::windows::core::Result<super::super::super::System::VirtualKeyModifiers> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::System::VirtualKeyModifiers = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyModifiers)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::VirtualKeyModifiers>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Handled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHandled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"UI_Input\"`*"]
    #[cfg(feature = "UI_Input")]
    pub fn GetCurrentPoint<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(&self, relativeto: Param0) -> ::windows::core::Result<super::super::Input::PointerPoint> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetCurrentPoint)(::core::mem::transmute_copy(this), relativeto.into_param().abi(), &mut result__).from_abi::<super::super::Input::PointerPoint>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"Foundation_Collections\"`, `\"UI_Input\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Input"))]
    pub fn GetIntermediatePoints<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(&self, relativeto: Param0) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<super::super::Input::PointerPoint>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetIntermediatePoints)(::core::mem::transmute_copy(this), relativeto.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<super::super::Input::PointerPoint>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn IsGenerated(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IPointerRoutedEventArgs2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsGenerated)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for PointerRoutedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PointerRoutedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PointerRoutedEventArgs {}
impl ::core::fmt::Debug for PointerRoutedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PointerRoutedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PointerRoutedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.PointerRoutedEventArgs;{da628f0a-9752-49e2-bde2-49eccab9194d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PointerRoutedEventArgs {
    type Vtable = IPointerRoutedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IPointerRoutedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PointerRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.PointerRoutedEventArgs";
}
impl ::core::convert::From<PointerRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: PointerRoutedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PointerRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PointerRoutedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PointerRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PointerRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PointerRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: PointerRoutedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PointerRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PointerRoutedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PointerRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PointerRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PointerRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: PointerRoutedEventArgs) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PointerRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: &PointerRoutedEventArgs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for PointerRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for &PointerRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
unsafe impl ::core::marker::Send for PointerRoutedEventArgs {}
unsafe impl ::core::marker::Sync for PointerRoutedEventArgs {}
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
pub struct ProcessKeyboardAcceleratorEventArgs(::windows::core::IUnknown);
impl ProcessKeyboardAcceleratorEventArgs {
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn Key(&self) -> ::windows::core::Result<super::super::super::System::VirtualKey> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::System::VirtualKey = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Key)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::VirtualKey>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn Modifiers(&self) -> ::windows::core::Result<super::super::super::System::VirtualKeyModifiers> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::System::VirtualKeyModifiers = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Modifiers)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::VirtualKeyModifiers>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Handled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHandled)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::clone::Clone for ProcessKeyboardAcceleratorEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ProcessKeyboardAcceleratorEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProcessKeyboardAcceleratorEventArgs {}
impl ::core::fmt::Debug for ProcessKeyboardAcceleratorEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProcessKeyboardAcceleratorEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ProcessKeyboardAcceleratorEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.ProcessKeyboardAcceleratorEventArgs;{fb79c216-972b-440c-9b83-2b4198dcf09d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ProcessKeyboardAcceleratorEventArgs {
    type Vtable = IProcessKeyboardAcceleratorEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IProcessKeyboardAcceleratorEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ProcessKeyboardAcceleratorEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.ProcessKeyboardAcceleratorEventArgs";
}
impl ::core::convert::From<ProcessKeyboardAcceleratorEventArgs> for ::windows::core::IUnknown {
    fn from(value: ProcessKeyboardAcceleratorEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProcessKeyboardAcceleratorEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ProcessKeyboardAcceleratorEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ProcessKeyboardAcceleratorEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ProcessKeyboardAcceleratorEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ProcessKeyboardAcceleratorEventArgs> for ::windows::core::IInspectable {
    fn from(value: ProcessKeyboardAcceleratorEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProcessKeyboardAcceleratorEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ProcessKeyboardAcceleratorEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ProcessKeyboardAcceleratorEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ProcessKeyboardAcceleratorEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ProcessKeyboardAcceleratorEventArgs {}
unsafe impl ::core::marker::Sync for ProcessKeyboardAcceleratorEventArgs {}
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
pub struct RightTappedEventHandler(pub ::windows::core::IUnknown);
impl RightTappedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<RightTappedRoutedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = RightTappedEventHandlerBox::<F> { vtable: &RightTappedEventHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, RightTappedRoutedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Invoke)(::core::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct RightTappedEventHandlerBox<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<RightTappedRoutedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const RightTappedEventHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<RightTappedRoutedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> RightTappedEventHandlerBox<F> {
    const VTABLE: RightTappedEventHandler_Vtbl = RightTappedEventHandler_Vtbl { base: ::windows::core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<RightTappedEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
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
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender), ::core::mem::transmute(&e)).into()
    }
}
impl ::core::clone::Clone for RightTappedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RightTappedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RightTappedEventHandler {}
impl ::core::fmt::Debug for RightTappedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RightTappedEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for RightTappedEventHandler {
    type Vtable = RightTappedEventHandler_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2532a062_f447_4950_9c46_f1e34a2c2238);
}
unsafe impl ::windows::core::RuntimeType for RightTappedEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{2532a062-f447-4950-9c46-f1e34a2c2238}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct RightTappedEventHandler_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
pub struct RightTappedRoutedEventArgs(::windows::core::IUnknown);
impl RightTappedRoutedEventArgs {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<RightTappedRoutedEventArgs, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"Devices_Input\"`*"]
    #[cfg(feature = "Devices_Input")]
    pub fn PointerDeviceType(&self) -> ::windows::core::Result<super::super::super::Devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Devices::Input::PointerDeviceType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PointerDeviceType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Devices::Input::PointerDeviceType>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Handled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHandled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetPosition<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(&self, relativeto: Param0) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetPosition)(::core::mem::transmute_copy(this), relativeto.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
}
impl ::core::clone::Clone for RightTappedRoutedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RightTappedRoutedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RightTappedRoutedEventArgs {}
impl ::core::fmt::Debug for RightTappedRoutedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RightTappedRoutedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RightTappedRoutedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.RightTappedRoutedEventArgs;{6834869d-7bd5-4033-b237-172f79abe393})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for RightTappedRoutedEventArgs {
    type Vtable = IRightTappedRoutedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IRightTappedRoutedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RightTappedRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.RightTappedRoutedEventArgs";
}
impl ::core::convert::From<RightTappedRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: RightTappedRoutedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RightTappedRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &RightTappedRoutedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RightTappedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a RightTappedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RightTappedRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: RightTappedRoutedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RightTappedRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &RightTappedRoutedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RightTappedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a RightTappedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RightTappedRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: RightTappedRoutedEventArgs) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&RightTappedRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: &RightTappedRoutedEventArgs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for RightTappedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for &RightTappedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
unsafe impl ::core::marker::Send for RightTappedRoutedEventArgs {}
unsafe impl ::core::marker::Sync for RightTappedRoutedEventArgs {}
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
pub struct StandardUICommand(::windows::core::IUnknown);
impl StandardUICommand {
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn Kind(&self) -> ::windows::core::Result<StandardUICommandKind> {
        let this = self;
        unsafe {
            let mut result__: StandardUICommandKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StandardUICommandKind>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn SetKind(&self, value: StandardUICommandKind) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IStandardUICommand2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetKind)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn new() -> ::windows::core::Result<StandardUICommand> {
        Self::IStandardUICommandFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(::core::mem::transmute_copy(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<StandardUICommand>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn compose<T: ::windows::core::Compose>(compose: T) -> ::windows::core::Result<StandardUICommand> {
        Self::IStandardUICommandFactory(|this| unsafe {
            let (derived__, base__) = ::windows::core::Compose::compose(compose);
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(::core::mem::transmute_copy(this), ::core::mem::transmute_copy(&derived__), base__ as *mut _ as _, &mut result__).from_abi::<StandardUICommand>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn CreateInstanceWithKind(kind: StandardUICommandKind) -> ::windows::core::Result<StandardUICommand> {
        Self::IStandardUICommandFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstanceWithKind)(::core::mem::transmute_copy(this), kind, ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<StandardUICommand>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn CreateInstanceWithKind_compose<T: ::windows::core::Compose>(kind: StandardUICommandKind, compose: T) -> ::windows::core::Result<StandardUICommand> {
        Self::IStandardUICommandFactory(|this| unsafe {
            let (derived__, base__) = ::windows::core::Compose::compose(compose);
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstanceWithKind)(::core::mem::transmute_copy(this), kind, ::core::mem::transmute_copy(&derived__), base__ as *mut _ as _, &mut result__).from_abi::<StandardUICommand>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn KindProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IStandardUICommandStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KindProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IStandardUICommandFactory<R, F: FnOnce(&IStandardUICommandFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StandardUICommand, IStandardUICommandFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IStandardUICommandStatics<R, F: FnOnce(&IStandardUICommandStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StandardUICommand, IStandardUICommandStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for StandardUICommand {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StandardUICommand {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StandardUICommand {}
impl ::core::fmt::Debug for StandardUICommand {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StandardUICommand").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StandardUICommand {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.StandardUICommand;{d2bf7f43-0504-52d0-8aa6-0cb0f756eb27})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for StandardUICommand {
    type Vtable = IStandardUICommand_Vtbl;
    const IID: ::windows::core::GUID = <IStandardUICommand as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StandardUICommand {
    const NAME: &'static str = "Windows.UI.Xaml.Input.StandardUICommand";
}
impl ::core::convert::From<StandardUICommand> for ::windows::core::IUnknown {
    fn from(value: StandardUICommand) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StandardUICommand> for ::windows::core::IUnknown {
    fn from(value: &StandardUICommand) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StandardUICommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StandardUICommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StandardUICommand> for ::windows::core::IInspectable {
    fn from(value: StandardUICommand) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StandardUICommand> for ::windows::core::IInspectable {
    fn from(value: &StandardUICommand) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StandardUICommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StandardUICommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&StandardUICommand> for XamlUICommand {
    fn from(value: &StandardUICommand) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, XamlUICommand> for StandardUICommand {
    fn into_param(self) -> ::windows::core::Param<'a, XamlUICommand> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, XamlUICommand> for &StandardUICommand {
    fn into_param(self) -> ::windows::core::Param<'a, XamlUICommand> {
        ::windows::core::Param::Owned(::core::convert::Into::<XamlUICommand>::into(self))
    }
}
impl ::core::convert::From<StandardUICommand> for super::DependencyObject {
    fn from(value: StandardUICommand) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&StandardUICommand> for super::DependencyObject {
    fn from(value: &StandardUICommand) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for StandardUICommand {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &StandardUICommand {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for StandardUICommand {}
unsafe impl ::core::marker::Sync for StandardUICommand {}
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct StandardUICommandKind(pub i32);
impl StandardUICommandKind {
    pub const None: Self = Self(0i32);
    pub const Cut: Self = Self(1i32);
    pub const Copy: Self = Self(2i32);
    pub const Paste: Self = Self(3i32);
    pub const SelectAll: Self = Self(4i32);
    pub const Delete: Self = Self(5i32);
    pub const Share: Self = Self(6i32);
    pub const Save: Self = Self(7i32);
    pub const Open: Self = Self(8i32);
    pub const Close: Self = Self(9i32);
    pub const Pause: Self = Self(10i32);
    pub const Play: Self = Self(11i32);
    pub const Stop: Self = Self(12i32);
    pub const Forward: Self = Self(13i32);
    pub const Backward: Self = Self(14i32);
    pub const Undo: Self = Self(15i32);
    pub const Redo: Self = Self(16i32);
}
impl ::core::marker::Copy for StandardUICommandKind {}
impl ::core::clone::Clone for StandardUICommandKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StandardUICommandKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for StandardUICommandKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for StandardUICommandKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StandardUICommandKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StandardUICommandKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Input.StandardUICommandKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
pub struct TappedEventHandler(pub ::windows::core::IUnknown);
impl TappedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<TappedRoutedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = TappedEventHandlerBox::<F> { vtable: &TappedEventHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, TappedRoutedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Invoke)(::core::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct TappedEventHandlerBox<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<TappedRoutedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const TappedEventHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<TappedRoutedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> TappedEventHandlerBox<F> {
    const VTABLE: TappedEventHandler_Vtbl = TappedEventHandler_Vtbl { base: ::windows::core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<TappedEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
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
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender), ::core::mem::transmute(&e)).into()
    }
}
impl ::core::clone::Clone for TappedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TappedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TappedEventHandler {}
impl ::core::fmt::Debug for TappedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TappedEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for TappedEventHandler {
    type Vtable = TappedEventHandler_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x68d940cc_9ff0_49ce_b141_3f07ec477b97);
}
unsafe impl ::windows::core::RuntimeType for TappedEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{68d940cc-9ff0-49ce-b141-3f07ec477b97}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct TappedEventHandler_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
pub struct TappedRoutedEventArgs(::windows::core::IUnknown);
impl TappedRoutedEventArgs {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TappedRoutedEventArgs, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"Devices_Input\"`*"]
    #[cfg(feature = "Devices_Input")]
    pub fn PointerDeviceType(&self) -> ::windows::core::Result<super::super::super::Devices::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Devices::Input::PointerDeviceType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PointerDeviceType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Devices::Input::PointerDeviceType>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Handled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHandled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetPosition<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(&self, relativeto: Param0) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetPosition)(::core::mem::transmute_copy(this), relativeto.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
}
impl ::core::clone::Clone for TappedRoutedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TappedRoutedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TappedRoutedEventArgs {}
impl ::core::fmt::Debug for TappedRoutedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TappedRoutedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TappedRoutedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.TappedRoutedEventArgs;{a099e6be-e624-459a-bb1d-e05c73e2cc66})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for TappedRoutedEventArgs {
    type Vtable = ITappedRoutedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <ITappedRoutedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for TappedRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.TappedRoutedEventArgs";
}
impl ::core::convert::From<TappedRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: TappedRoutedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TappedRoutedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &TappedRoutedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TappedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TappedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TappedRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: TappedRoutedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TappedRoutedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &TappedRoutedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TappedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TappedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TappedRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: TappedRoutedEventArgs) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&TappedRoutedEventArgs> for super::RoutedEventArgs {
    fn from(value: &TappedRoutedEventArgs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for TappedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::RoutedEventArgs> for &TappedRoutedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::RoutedEventArgs> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(self))
    }
}
unsafe impl ::core::marker::Send for TappedRoutedEventArgs {}
unsafe impl ::core::marker::Sync for TappedRoutedEventArgs {}
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct XYFocusKeyboardNavigationMode(pub i32);
impl XYFocusKeyboardNavigationMode {
    pub const Auto: Self = Self(0i32);
    pub const Enabled: Self = Self(1i32);
    pub const Disabled: Self = Self(2i32);
}
impl ::core::marker::Copy for XYFocusKeyboardNavigationMode {}
impl ::core::clone::Clone for XYFocusKeyboardNavigationMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XYFocusKeyboardNavigationMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for XYFocusKeyboardNavigationMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for XYFocusKeyboardNavigationMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XYFocusKeyboardNavigationMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for XYFocusKeyboardNavigationMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Input.XYFocusKeyboardNavigationMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct XYFocusNavigationStrategy(pub i32);
impl XYFocusNavigationStrategy {
    pub const Auto: Self = Self(0i32);
    pub const Projection: Self = Self(1i32);
    pub const NavigationDirectionDistance: Self = Self(2i32);
    pub const RectilinearDistance: Self = Self(3i32);
}
impl ::core::marker::Copy for XYFocusNavigationStrategy {}
impl ::core::clone::Clone for XYFocusNavigationStrategy {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XYFocusNavigationStrategy {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for XYFocusNavigationStrategy {
    type Abi = Self;
}
impl ::core::fmt::Debug for XYFocusNavigationStrategy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XYFocusNavigationStrategy").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for XYFocusNavigationStrategy {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Input.XYFocusNavigationStrategy;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct XYFocusNavigationStrategyOverride(pub i32);
impl XYFocusNavigationStrategyOverride {
    pub const None: Self = Self(0i32);
    pub const Auto: Self = Self(1i32);
    pub const Projection: Self = Self(2i32);
    pub const NavigationDirectionDistance: Self = Self(3i32);
    pub const RectilinearDistance: Self = Self(4i32);
}
impl ::core::marker::Copy for XYFocusNavigationStrategyOverride {}
impl ::core::clone::Clone for XYFocusNavigationStrategyOverride {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XYFocusNavigationStrategyOverride {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for XYFocusNavigationStrategyOverride {
    type Abi = Self;
}
impl ::core::fmt::Debug for XYFocusNavigationStrategyOverride {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XYFocusNavigationStrategyOverride").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for XYFocusNavigationStrategyOverride {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Input.XYFocusNavigationStrategyOverride;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
#[repr(transparent)]
pub struct XamlUICommand(::windows::core::IUnknown);
impl XamlUICommand {
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CanExecuteChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICommand>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanExecuteChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCanExecuteChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICommand>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveCanExecuteChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn CanExecute<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, parameter: Param0) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommand>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanExecute)(::core::mem::transmute_copy(this), parameter.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn Execute<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, parameter: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICommand>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Execute)(::core::mem::transmute_copy(this), parameter.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn Label(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Label)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn SetLabel<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetLabel)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"UI_Xaml_Controls\"`*"]
    #[cfg(feature = "UI_Xaml_Controls")]
    pub fn IconSource(&self) -> ::windows::core::Result<super::Controls::IconSource> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IconSource)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Controls::IconSource>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"UI_Xaml_Controls\"`*"]
    #[cfg(feature = "UI_Xaml_Controls")]
    pub fn SetIconSource<'a, Param0: ::windows::core::IntoParam<'a, super::Controls::IconSource>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIconSource)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn KeyboardAccelerators(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<KeyboardAccelerator>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyboardAccelerators)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<KeyboardAccelerator>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn AccessKey(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKey)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn SetAccessKey<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAccessKey)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Description)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDescription)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn Command(&self) -> ::windows::core::Result<ICommand> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Command)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ICommand>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn SetCommand<'a, Param0: ::windows::core::IntoParam<'a, ICommand>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCommand)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ExecuteRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<XamlUICommand, ExecuteRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExecuteRequested)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveExecuteRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveExecuteRequested)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CanExecuteRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<XamlUICommand, CanExecuteRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanExecuteRequested)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCanExecuteRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveCanExecuteRequested)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn NotifyCanExecuteChanged(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).NotifyCanExecuteChanged)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn new() -> ::windows::core::Result<XamlUICommand> {
        Self::IXamlUICommandFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(::core::mem::transmute_copy(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<XamlUICommand>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn compose<T: ::windows::core::Compose>(compose: T) -> ::windows::core::Result<XamlUICommand> {
        Self::IXamlUICommandFactory(|this| unsafe {
            let (derived__, base__) = ::windows::core::Compose::compose(compose);
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(::core::mem::transmute_copy(this), ::core::mem::transmute_copy(&derived__), base__ as *mut _ as _, &mut result__).from_abi::<XamlUICommand>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn LabelProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IXamlUICommandStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LabelProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn IconSourceProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IXamlUICommandStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IconSourceProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn KeyboardAcceleratorsProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IXamlUICommandStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyboardAcceleratorsProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn AccessKeyProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IXamlUICommandStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccessKeyProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn DescriptionProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IXamlUICommandStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DescriptionProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    pub fn CommandProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IXamlUICommandStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CommandProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IXamlUICommandFactory<R, F: FnOnce(&IXamlUICommandFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<XamlUICommand, IXamlUICommandFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IXamlUICommandStatics<R, F: FnOnce(&IXamlUICommandStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<XamlUICommand, IXamlUICommandStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for XamlUICommand {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XamlUICommand {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XamlUICommand {}
impl ::core::fmt::Debug for XamlUICommand {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XamlUICommand").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for XamlUICommand {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Input.XamlUICommand;{8494f8d4-ead1-5f01-ad2e-a8cad4f9dc0e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for XamlUICommand {
    type Vtable = IXamlUICommand_Vtbl;
    const IID: ::windows::core::GUID = <IXamlUICommand as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for XamlUICommand {
    const NAME: &'static str = "Windows.UI.Xaml.Input.XamlUICommand";
}
impl ::core::convert::From<XamlUICommand> for ::windows::core::IUnknown {
    fn from(value: XamlUICommand) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XamlUICommand> for ::windows::core::IUnknown {
    fn from(value: &XamlUICommand) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for XamlUICommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a XamlUICommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<XamlUICommand> for ::windows::core::IInspectable {
    fn from(value: XamlUICommand) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XamlUICommand> for ::windows::core::IInspectable {
    fn from(value: &XamlUICommand) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for XamlUICommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a XamlUICommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&XamlUICommand> for super::DependencyObject {
    fn from(value: &XamlUICommand) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for XamlUICommand {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &XamlUICommand {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for XamlUICommand {}
unsafe impl ::core::marker::Sync for XamlUICommand {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
