#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"UI_ApplicationSettings\"`*"]
#[repr(transparent)]
pub struct AccountsSettingsPane(::windows::core::IUnknown);
impl AccountsSettingsPane {
    #[doc = "*Required features: `\"UI_ApplicationSettings\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AccountCommandsRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AccountsSettingsPane, AccountsSettingsPaneCommandsRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccountCommandsRequested)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ApplicationSettings\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAccountCommandsRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAccountCommandsRequested)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_ApplicationSettings\"`*"]
    pub fn GetForCurrentView() -> ::windows::core::Result<AccountsSettingsPane> {
        Self::IAccountsSettingsPaneStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetForCurrentView)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AccountsSettingsPane>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_ApplicationSettings\"`*"]
    pub fn Show() -> ::windows::core::Result<()> {
        Self::IAccountsSettingsPaneStatics(|this| unsafe { (::windows::core::Interface::vtable(this).Show)(::core::mem::transmute_copy(this)).ok() })
    }
    #[doc = "*Required features: `\"UI_ApplicationSettings\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ShowManageAccountsAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        Self::IAccountsSettingsPaneStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShowManageAccountsAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_ApplicationSettings\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ShowAddAccountAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        Self::IAccountsSettingsPaneStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShowAddAccountAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_ApplicationSettings\"`, `\"Foundation\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn ShowManageAccountsForUserAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::User>>(user: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        Self::IAccountsSettingsPaneStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShowManageAccountsForUserAsync)(::core::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_ApplicationSettings\"`, `\"Foundation\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn ShowAddAccountForUserAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::User>>(user: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        Self::IAccountsSettingsPaneStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShowAddAccountForUserAsync)(::core::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAccountsSettingsPaneStatics<R, F: FnOnce(&IAccountsSettingsPaneStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AccountsSettingsPane, IAccountsSettingsPaneStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IAccountsSettingsPaneStatics2<R, F: FnOnce(&IAccountsSettingsPaneStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AccountsSettingsPane, IAccountsSettingsPaneStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IAccountsSettingsPaneStatics3<R, F: FnOnce(&IAccountsSettingsPaneStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AccountsSettingsPane, IAccountsSettingsPaneStatics3> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AccountsSettingsPane {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AccountsSettingsPane {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AccountsSettingsPane {}
impl ::core::fmt::Debug for AccountsSettingsPane {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AccountsSettingsPane").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AccountsSettingsPane {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.ApplicationSettings.AccountsSettingsPane;{81ea942c-4f09-4406-a538-838d9b14b7e6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AccountsSettingsPane {
    type Vtable = IAccountsSettingsPane_Vtbl;
    const IID: ::windows::core::GUID = <IAccountsSettingsPane as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AccountsSettingsPane {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.AccountsSettingsPane";
}
impl ::core::convert::From<AccountsSettingsPane> for ::windows::core::IUnknown {
    fn from(value: AccountsSettingsPane) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AccountsSettingsPane> for ::windows::core::IUnknown {
    fn from(value: &AccountsSettingsPane) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AccountsSettingsPane {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AccountsSettingsPane {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AccountsSettingsPane> for ::windows::core::IInspectable {
    fn from(value: AccountsSettingsPane) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AccountsSettingsPane> for ::windows::core::IInspectable {
    fn from(value: &AccountsSettingsPane) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AccountsSettingsPane {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AccountsSettingsPane {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"UI_ApplicationSettings\"`*"]
#[repr(transparent)]
pub struct AccountsSettingsPaneCommandsRequestedEventArgs(::windows::core::IUnknown);
impl AccountsSettingsPaneCommandsRequestedEventArgs {
    #[doc = "*Required features: `\"UI_ApplicationSettings\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn WebAccountProviderCommands(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<WebAccountProviderCommand>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WebAccountProviderCommands)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<WebAccountProviderCommand>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ApplicationSettings\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn WebAccountCommands(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<WebAccountCommand>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WebAccountCommands)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<WebAccountCommand>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ApplicationSettings\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CredentialCommands(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<CredentialCommand>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CredentialCommands)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<CredentialCommand>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ApplicationSettings\"`, `\"Foundation_Collections\"`, `\"UI_Popups\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Popups"))]
    pub fn Commands(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SettingsCommand>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Commands)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<SettingsCommand>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ApplicationSettings\"`*"]
    pub fn HeaderText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HeaderText)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ApplicationSettings\"`*"]
    pub fn SetHeaderText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHeaderText)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_ApplicationSettings\"`*"]
    pub fn GetDeferral(&self) -> ::windows::core::Result<AccountsSettingsPaneEventDeferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AccountsSettingsPaneEventDeferral>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ApplicationSettings\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IAccountsSettingsPaneCommandsRequestedEventArgs2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).User)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
}
impl ::core::clone::Clone for AccountsSettingsPaneCommandsRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AccountsSettingsPaneCommandsRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AccountsSettingsPaneCommandsRequestedEventArgs {}
impl ::core::fmt::Debug for AccountsSettingsPaneCommandsRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AccountsSettingsPaneCommandsRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AccountsSettingsPaneCommandsRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.ApplicationSettings.AccountsSettingsPaneCommandsRequestedEventArgs;{3b68c099-db19-45d0-9abf-95d3773c9330})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AccountsSettingsPaneCommandsRequestedEventArgs {
    type Vtable = IAccountsSettingsPaneCommandsRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IAccountsSettingsPaneCommandsRequestedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AccountsSettingsPaneCommandsRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.AccountsSettingsPaneCommandsRequestedEventArgs";
}
impl ::core::convert::From<AccountsSettingsPaneCommandsRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AccountsSettingsPaneCommandsRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AccountsSettingsPaneCommandsRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AccountsSettingsPaneCommandsRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AccountsSettingsPaneCommandsRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AccountsSettingsPaneCommandsRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AccountsSettingsPaneCommandsRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AccountsSettingsPaneCommandsRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AccountsSettingsPaneCommandsRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AccountsSettingsPaneCommandsRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AccountsSettingsPaneCommandsRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AccountsSettingsPaneCommandsRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"UI_ApplicationSettings\"`*"]
#[repr(transparent)]
pub struct AccountsSettingsPaneEventDeferral(::windows::core::IUnknown);
impl AccountsSettingsPaneEventDeferral {
    #[doc = "*Required features: `\"UI_ApplicationSettings\"`*"]
    pub fn Complete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Complete)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for AccountsSettingsPaneEventDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AccountsSettingsPaneEventDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AccountsSettingsPaneEventDeferral {}
impl ::core::fmt::Debug for AccountsSettingsPaneEventDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AccountsSettingsPaneEventDeferral").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AccountsSettingsPaneEventDeferral {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.ApplicationSettings.AccountsSettingsPaneEventDeferral;{cbf25d3f-e5ba-40ef-93da-65e096e5fb04})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AccountsSettingsPaneEventDeferral {
    type Vtable = IAccountsSettingsPaneEventDeferral_Vtbl;
    const IID: ::windows::core::GUID = <IAccountsSettingsPaneEventDeferral as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AccountsSettingsPaneEventDeferral {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.AccountsSettingsPaneEventDeferral";
}
impl ::core::convert::From<AccountsSettingsPaneEventDeferral> for ::windows::core::IUnknown {
    fn from(value: AccountsSettingsPaneEventDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AccountsSettingsPaneEventDeferral> for ::windows::core::IUnknown {
    fn from(value: &AccountsSettingsPaneEventDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AccountsSettingsPaneEventDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AccountsSettingsPaneEventDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AccountsSettingsPaneEventDeferral> for ::windows::core::IInspectable {
    fn from(value: AccountsSettingsPaneEventDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AccountsSettingsPaneEventDeferral> for ::windows::core::IInspectable {
    fn from(value: &AccountsSettingsPaneEventDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AccountsSettingsPaneEventDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AccountsSettingsPaneEventDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"UI_ApplicationSettings\"`*"]
#[repr(transparent)]
pub struct CredentialCommand(::windows::core::IUnknown);
impl CredentialCommand {
    #[doc = "*Required features: `\"UI_ApplicationSettings\"`, `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn PasswordCredential(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PasswordCredential)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Credentials::PasswordCredential>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ApplicationSettings\"`*"]
    pub fn CredentialDeleted(&self) -> ::windows::core::Result<CredentialCommandCredentialDeletedHandler> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CredentialDeleted)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CredentialCommandCredentialDeletedHandler>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ApplicationSettings\"`, `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn CreateCredentialCommand<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::Credentials::PasswordCredential>>(passwordcredential: Param0) -> ::windows::core::Result<CredentialCommand> {
        Self::ICredentialCommandFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateCredentialCommand)(::core::mem::transmute_copy(this), passwordcredential.into_param().abi(), &mut result__).from_abi::<CredentialCommand>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_ApplicationSettings\"`, `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn CreateCredentialCommandWithHandler<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::Credentials::PasswordCredential>, Param1: ::windows::core::IntoParam<'a, CredentialCommandCredentialDeletedHandler>>(passwordcredential: Param0, deleted: Param1) -> ::windows::core::Result<CredentialCommand> {
        Self::ICredentialCommandFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateCredentialCommandWithHandler)(::core::mem::transmute_copy(this), passwordcredential.into_param().abi(), deleted.into_param().abi(), &mut result__).from_abi::<CredentialCommand>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICredentialCommandFactory<R, F: FnOnce(&ICredentialCommandFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CredentialCommand, ICredentialCommandFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for CredentialCommand {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CredentialCommand {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CredentialCommand {}
impl ::core::fmt::Debug for CredentialCommand {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CredentialCommand").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CredentialCommand {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.ApplicationSettings.CredentialCommand;{a5f665e6-6143-4a7a-a971-b017ba978ce2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CredentialCommand {
    type Vtable = ICredentialCommand_Vtbl;
    const IID: ::windows::core::GUID = <ICredentialCommand as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CredentialCommand {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.CredentialCommand";
}
impl ::core::convert::From<CredentialCommand> for ::windows::core::IUnknown {
    fn from(value: CredentialCommand) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CredentialCommand> for ::windows::core::IUnknown {
    fn from(value: &CredentialCommand) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CredentialCommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CredentialCommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CredentialCommand> for ::windows::core::IInspectable {
    fn from(value: CredentialCommand) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CredentialCommand> for ::windows::core::IInspectable {
    fn from(value: &CredentialCommand) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CredentialCommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CredentialCommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"UI_ApplicationSettings\"`*"]
#[repr(transparent)]
pub struct CredentialCommandCredentialDeletedHandler(pub ::windows::core::IUnknown);
impl CredentialCommandCredentialDeletedHandler {
    pub fn new<F: FnMut(&::core::option::Option<CredentialCommand>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = CredentialCommandCredentialDeletedHandlerBox::<F> { vtable: &CredentialCommandCredentialDeletedHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `\"UI_ApplicationSettings\"`*"]
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, CredentialCommand>>(&self, command: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Invoke)(::core::mem::transmute_copy(this), command.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct CredentialCommandCredentialDeletedHandlerBox<F: FnMut(&::core::option::Option<CredentialCommand>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const CredentialCommandCredentialDeletedHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<CredentialCommand>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> CredentialCommandCredentialDeletedHandlerBox<F> {
    const VTABLE: CredentialCommandCredentialDeletedHandler_Vtbl = CredentialCommandCredentialDeletedHandler_Vtbl { base: ::windows::core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<CredentialCommandCredentialDeletedHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
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
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, command: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&command)).into()
    }
}
impl ::core::clone::Clone for CredentialCommandCredentialDeletedHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CredentialCommandCredentialDeletedHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CredentialCommandCredentialDeletedHandler {}
impl ::core::fmt::Debug for CredentialCommandCredentialDeletedHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CredentialCommandCredentialDeletedHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for CredentialCommandCredentialDeletedHandler {
    type Vtable = CredentialCommandCredentialDeletedHandler_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x61c0e185_0977_4678_b4e2_98727afbeed9);
}
unsafe impl ::windows::core::RuntimeType for CredentialCommandCredentialDeletedHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{61c0e185-0977-4678-b4e2-98727afbeed9}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct CredentialCommandCredentialDeletedHandler_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, command: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAccountsSettingsPane(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAccountsSettingsPane {
    type Vtable = IAccountsSettingsPane_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x81ea942c_4f09_4406_a538_838d9b14b7e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccountsSettingsPane_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub AccountCommandsRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AccountCommandsRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAccountCommandsRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAccountCommandsRequested: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAccountsSettingsPaneCommandsRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAccountsSettingsPaneCommandsRequestedEventArgs {
    type Vtable = IAccountsSettingsPaneCommandsRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3b68c099_db19_45d0_9abf_95d3773c9330);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccountsSettingsPaneCommandsRequestedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub WebAccountProviderCommands: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    WebAccountProviderCommands: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub WebAccountCommands: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    WebAccountCommands: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CredentialCommands: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CredentialCommands: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Popups"))]
    pub Commands: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Popups")))]
    Commands: usize,
    pub HeaderText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetHeaderText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAccountsSettingsPaneCommandsRequestedEventArgs2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAccountsSettingsPaneCommandsRequestedEventArgs2 {
    type Vtable = IAccountsSettingsPaneCommandsRequestedEventArgs2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x362f7bad_4e37_4967_8c40_e78ee7a1e5bb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccountsSettingsPaneCommandsRequestedEventArgs2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAccountsSettingsPaneEventDeferral(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAccountsSettingsPaneEventDeferral {
    type Vtable = IAccountsSettingsPaneEventDeferral_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcbf25d3f_e5ba_40ef_93da_65e096e5fb04);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccountsSettingsPaneEventDeferral_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAccountsSettingsPaneStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAccountsSettingsPaneStatics {
    type Vtable = IAccountsSettingsPaneStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x561f8b60_b0ec_4150_a8dc_208ee44b068a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccountsSettingsPaneStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Show: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAccountsSettingsPaneStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAccountsSettingsPaneStatics2 {
    type Vtable = IAccountsSettingsPaneStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd21df7c2_ce0d_484f_b8e8_e823c215765e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccountsSettingsPaneStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub ShowManageAccountsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowManageAccountsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowAddAccountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowAddAccountAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAccountsSettingsPaneStatics3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAccountsSettingsPaneStatics3 {
    type Vtable = IAccountsSettingsPaneStatics3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x08410458_a2ba_4c6f_b4ac_48f514331216);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccountsSettingsPaneStatics3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub ShowManageAccountsForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    ShowManageAccountsForUserAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub ShowAddAccountForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    ShowAddAccountForUserAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICredentialCommand(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICredentialCommand {
    type Vtable = ICredentialCommand_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa5f665e6_6143_4a7a_a971_b017ba978ce2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICredentialCommand_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Security_Credentials")]
    pub PasswordCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    PasswordCredential: usize,
    pub CredentialDeleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICredentialCommandFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICredentialCommandFactory {
    type Vtable = ICredentialCommandFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27e88c17_bc3e_4b80_9495_4ed720e48a91);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICredentialCommandFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Security_Credentials")]
    pub CreateCredentialCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, passwordcredential: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    CreateCredentialCommand: usize,
    #[cfg(feature = "Security_Credentials")]
    pub CreateCredentialCommandWithHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, passwordcredential: ::windows::core::RawPtr, deleted: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    CreateCredentialCommandWithHandler: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISettingsCommandFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISettingsCommandFactory {
    type Vtable = ISettingsCommandFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x68e15b33_1c83_433a_aa5a_ceeea5bd4764);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISettingsCommandFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "UI_Popups")]
    pub CreateSettingsCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settingscommandid: *mut ::core::ffi::c_void, label: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, handler: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Popups"))]
    CreateSettingsCommand: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISettingsCommandStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISettingsCommandStatics {
    type Vtable = ISettingsCommandStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x749ae954_2f69_4b17_8aba_d05ce5778e46);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISettingsCommandStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "UI_Popups")]
    pub AccountsCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Popups"))]
    AccountsCommand: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ISettingsPane(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for ISettingsPane {
    type Vtable = ISettingsPane_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb1cd0932_4570_4c69_8d38_89446561ace0);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ISettingsPane_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub CommandsRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    CommandsRequested: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveCommandsRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveCommandsRequested: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ISettingsPaneCommandsRequest(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for ISettingsPaneCommandsRequest {
    type Vtable = ISettingsPaneCommandsRequest_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x44df23ae_5d6e_4068_a168_f47643182114);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ISettingsPaneCommandsRequest_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Popups", feature = "deprecated"))]
    pub ApplicationCommands: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Popups", feature = "deprecated")))]
    ApplicationCommands: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ISettingsPaneCommandsRequestedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for ISettingsPaneCommandsRequestedEventArgs {
    type Vtable = ISettingsPaneCommandsRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x205f5d24_1b48_4629_a6ca_2fdfedafb75d);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ISettingsPaneCommandsRequestedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Request: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ISettingsPaneStatics(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for ISettingsPaneStatics {
    type Vtable = ISettingsPaneStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1c6a52c5_ff19_471b_ba6b_f8f35694ad9a);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ISettingsPaneStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    GetForCurrentView: usize,
    #[cfg(feature = "deprecated")]
    pub Show: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Show: usize,
    #[cfg(feature = "deprecated")]
    pub Edge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SettingsEdgeLocation) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Edge: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAccountCommand(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebAccountCommand {
    type Vtable = IWebAccountCommand_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcaa39398_9cfa_4246_b0c4_a913a3896541);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountCommand_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Security_Credentials")]
    pub WebAccount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    WebAccount: usize,
    pub Invoked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Actions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SupportedWebAccountActions) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAccountCommandFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebAccountCommandFactory {
    type Vtable = IWebAccountCommandFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbfa6cdff_2f2d_42f5_81de_1d56bafc496d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountCommandFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Security_Credentials")]
    pub CreateWebAccountCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, webaccount: ::windows::core::RawPtr, invoked: ::windows::core::RawPtr, actions: SupportedWebAccountActions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    CreateWebAccountCommand: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAccountInvokedArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebAccountInvokedArgs {
    type Vtable = IWebAccountInvokedArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe7abcc40_a1d8_4c5d_9a7f_1d34b2f90ad2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountInvokedArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Action: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WebAccountAction) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAccountProviderCommand(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebAccountProviderCommand {
    type Vtable = IWebAccountProviderCommand_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd69bdd9a_a0a6_4e9b_88dc_c71e757a3501);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountProviderCommand_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Security_Credentials")]
    pub WebAccountProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    WebAccountProvider: usize,
    pub Invoked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAccountProviderCommandFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebAccountProviderCommandFactory {
    type Vtable = IWebAccountProviderCommandFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd5658a1b_b176_4776_8469_a9d3ff0b3f59);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountProviderCommandFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Security_Credentials")]
    pub CreateWebAccountProviderCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, webaccountprovider: ::windows::core::RawPtr, invoked: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    CreateWebAccountProviderCommand: usize,
}
#[doc = "*Required features: `\"UI_ApplicationSettings\"`, `\"UI_Popups\"`*"]
#[cfg(feature = "UI_Popups")]
#[repr(transparent)]
pub struct SettingsCommand(::windows::core::IUnknown);
#[cfg(feature = "UI_Popups")]
impl SettingsCommand {
    #[doc = "*Required features: `\"UI_ApplicationSettings\"`, `\"UI_Popups\"`*"]
    #[cfg(feature = "UI_Popups")]
    pub fn CreateSettingsCommand<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, super::Popups::UICommandInvokedHandler>>(settingscommandid: Param0, label: Param1, handler: Param2) -> ::windows::core::Result<SettingsCommand> {
        Self::ISettingsCommandFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateSettingsCommand)(::core::mem::transmute_copy(this), settingscommandid.into_param().abi(), label.into_param().abi(), handler.into_param().abi(), &mut result__).from_abi::<SettingsCommand>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_ApplicationSettings\"`, `\"UI_Popups\"`*"]
    #[cfg(feature = "UI_Popups")]
    pub fn AccountsCommand() -> ::windows::core::Result<SettingsCommand> {
        Self::ISettingsCommandStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccountsCommand)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SettingsCommand>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_ApplicationSettings\"`, `\"UI_Popups\"`*"]
    #[cfg(feature = "UI_Popups")]
    pub fn Label(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Label)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ApplicationSettings\"`, `\"UI_Popups\"`*"]
    #[cfg(feature = "UI_Popups")]
    pub fn SetLabel<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetLabel)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_ApplicationSettings\"`, `\"UI_Popups\"`*"]
    #[cfg(feature = "UI_Popups")]
    pub fn Invoked(&self) -> ::windows::core::Result<super::Popups::UICommandInvokedHandler> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Invoked)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Popups::UICommandInvokedHandler>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ApplicationSettings\"`, `\"UI_Popups\"`*"]
    #[cfg(feature = "UI_Popups")]
    pub fn SetInvoked<'a, Param0: ::windows::core::IntoParam<'a, super::Popups::UICommandInvokedHandler>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetInvoked)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_ApplicationSettings\"`, `\"UI_Popups\"`*"]
    #[cfg(feature = "UI_Popups")]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Id)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ApplicationSettings\"`, `\"UI_Popups\"`*"]
    #[cfg(feature = "UI_Popups")]
    pub fn SetId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetId)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc(hidden)]
    pub fn ISettingsCommandFactory<R, F: FnOnce(&ISettingsCommandFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SettingsCommand, ISettingsCommandFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn ISettingsCommandStatics<R, F: FnOnce(&ISettingsCommandStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SettingsCommand, ISettingsCommandStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "UI_Popups")]
impl ::core::clone::Clone for SettingsCommand {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "UI_Popups")]
impl ::core::cmp::PartialEq for SettingsCommand {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "UI_Popups")]
impl ::core::cmp::Eq for SettingsCommand {}
#[cfg(feature = "UI_Popups")]
impl ::core::fmt::Debug for SettingsCommand {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SettingsCommand").field(&self.0).finish()
    }
}
#[cfg(feature = "UI_Popups")]
unsafe impl ::windows::core::RuntimeType for SettingsCommand {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.ApplicationSettings.SettingsCommand;{4ff93a75-4145-47ff-ac7f-dff1c1fa5b0f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "UI_Popups")]
unsafe impl ::windows::core::Interface for SettingsCommand {
    type Vtable = super::Popups::IUICommand_Vtbl;
    const IID: ::windows::core::GUID = <super::Popups::IUICommand as ::windows::core::Interface>::IID;
}
#[cfg(feature = "UI_Popups")]
impl ::windows::core::RuntimeName for SettingsCommand {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.SettingsCommand";
}
#[cfg(feature = "UI_Popups")]
impl ::core::convert::From<SettingsCommand> for ::windows::core::IUnknown {
    fn from(value: SettingsCommand) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "UI_Popups")]
impl ::core::convert::From<&SettingsCommand> for ::windows::core::IUnknown {
    fn from(value: &SettingsCommand) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "UI_Popups")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SettingsCommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "UI_Popups")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SettingsCommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "UI_Popups")]
impl ::core::convert::From<SettingsCommand> for ::windows::core::IInspectable {
    fn from(value: SettingsCommand) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "UI_Popups")]
impl ::core::convert::From<&SettingsCommand> for ::windows::core::IInspectable {
    fn from(value: &SettingsCommand) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "UI_Popups")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SettingsCommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "UI_Popups")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SettingsCommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "UI_Popups")]
impl ::core::convert::TryFrom<SettingsCommand> for super::Popups::IUICommand {
    type Error = ::windows::core::Error;
    fn try_from(value: SettingsCommand) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Popups")]
impl ::core::convert::TryFrom<&SettingsCommand> for super::Popups::IUICommand {
    type Error = ::windows::core::Error;
    fn try_from(value: &SettingsCommand) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Popups")]
impl<'a> ::windows::core::IntoParam<'a, super::Popups::IUICommand> for SettingsCommand {
    fn into_param(self) -> ::windows::core::Param<'a, super::Popups::IUICommand> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Popups")]
impl<'a> ::windows::core::IntoParam<'a, super::Popups::IUICommand> for &SettingsCommand {
    fn into_param(self) -> ::windows::core::Param<'a, super::Popups::IUICommand> {
        ::core::convert::TryInto::<super::Popups::IUICommand>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[doc = "*Required features: `\"UI_ApplicationSettings\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SettingsEdgeLocation(pub i32);
#[cfg(feature = "deprecated")]
impl SettingsEdgeLocation {
    pub const Right: Self = Self(0i32);
    pub const Left: Self = Self(1i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for SettingsEdgeLocation {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SettingsEdgeLocation {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "deprecated")]
impl ::core::default::Default for SettingsEdgeLocation {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Abi for SettingsEdgeLocation {
    type Abi = Self;
}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for SettingsEdgeLocation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SettingsEdgeLocation").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for SettingsEdgeLocation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.ApplicationSettings.SettingsEdgeLocation;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_ApplicationSettings\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct SettingsPane(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl SettingsPane {
    #[doc = "*Required features: `\"UI_ApplicationSettings\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn CommandsRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<SettingsPane, SettingsPaneCommandsRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CommandsRequested)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ApplicationSettings\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveCommandsRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveCommandsRequested)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_ApplicationSettings\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn GetForCurrentView() -> ::windows::core::Result<SettingsPane> {
        Self::ISettingsPaneStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetForCurrentView)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SettingsPane>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_ApplicationSettings\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Show() -> ::windows::core::Result<()> {
        Self::ISettingsPaneStatics(|this| unsafe { (::windows::core::Interface::vtable(this).Show)(::core::mem::transmute_copy(this)).ok() })
    }
    #[doc = "*Required features: `\"UI_ApplicationSettings\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Edge() -> ::windows::core::Result<SettingsEdgeLocation> {
        Self::ISettingsPaneStatics(|this| unsafe {
            let mut result__: SettingsEdgeLocation = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Edge)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SettingsEdgeLocation>(result__)
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn ISettingsPaneStatics<R, F: FnOnce(&ISettingsPaneStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SettingsPane, ISettingsPaneStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SettingsPane {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for SettingsPane {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for SettingsPane {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for SettingsPane {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SettingsPane").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for SettingsPane {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.ApplicationSettings.SettingsPane;{b1cd0932-4570-4c69-8d38-89446561ace0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for SettingsPane {
    type Vtable = ISettingsPane_Vtbl;
    const IID: ::windows::core::GUID = <ISettingsPane as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for SettingsPane {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.SettingsPane";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<SettingsPane> for ::windows::core::IUnknown {
    fn from(value: SettingsPane) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&SettingsPane> for ::windows::core::IUnknown {
    fn from(value: &SettingsPane) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SettingsPane {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SettingsPane {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<SettingsPane> for ::windows::core::IInspectable {
    fn from(value: SettingsPane) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&SettingsPane> for ::windows::core::IInspectable {
    fn from(value: &SettingsPane) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SettingsPane {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SettingsPane {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"UI_ApplicationSettings\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct SettingsPaneCommandsRequest(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl SettingsPaneCommandsRequest {
    #[doc = "*Required features: `\"UI_ApplicationSettings\"`, `\"Foundation_Collections\"`, `\"UI_Popups\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Popups", feature = "deprecated"))]
    pub fn ApplicationCommands(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SettingsCommand>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ApplicationCommands)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<SettingsCommand>>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SettingsPaneCommandsRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for SettingsPaneCommandsRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for SettingsPaneCommandsRequest {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for SettingsPaneCommandsRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SettingsPaneCommandsRequest").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for SettingsPaneCommandsRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.ApplicationSettings.SettingsPaneCommandsRequest;{44df23ae-5d6e-4068-a168-f47643182114})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for SettingsPaneCommandsRequest {
    type Vtable = ISettingsPaneCommandsRequest_Vtbl;
    const IID: ::windows::core::GUID = <ISettingsPaneCommandsRequest as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for SettingsPaneCommandsRequest {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.SettingsPaneCommandsRequest";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<SettingsPaneCommandsRequest> for ::windows::core::IUnknown {
    fn from(value: SettingsPaneCommandsRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&SettingsPaneCommandsRequest> for ::windows::core::IUnknown {
    fn from(value: &SettingsPaneCommandsRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SettingsPaneCommandsRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SettingsPaneCommandsRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<SettingsPaneCommandsRequest> for ::windows::core::IInspectable {
    fn from(value: SettingsPaneCommandsRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&SettingsPaneCommandsRequest> for ::windows::core::IInspectable {
    fn from(value: &SettingsPaneCommandsRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SettingsPaneCommandsRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SettingsPaneCommandsRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"UI_ApplicationSettings\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct SettingsPaneCommandsRequestedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl SettingsPaneCommandsRequestedEventArgs {
    #[doc = "*Required features: `\"UI_ApplicationSettings\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Request(&self) -> ::windows::core::Result<SettingsPaneCommandsRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Request)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SettingsPaneCommandsRequest>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SettingsPaneCommandsRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for SettingsPaneCommandsRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for SettingsPaneCommandsRequestedEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for SettingsPaneCommandsRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SettingsPaneCommandsRequestedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for SettingsPaneCommandsRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.ApplicationSettings.SettingsPaneCommandsRequestedEventArgs;{205f5d24-1b48-4629-a6ca-2fdfedafb75d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for SettingsPaneCommandsRequestedEventArgs {
    type Vtable = ISettingsPaneCommandsRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <ISettingsPaneCommandsRequestedEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for SettingsPaneCommandsRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.SettingsPaneCommandsRequestedEventArgs";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<SettingsPaneCommandsRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: SettingsPaneCommandsRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&SettingsPaneCommandsRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &SettingsPaneCommandsRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SettingsPaneCommandsRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SettingsPaneCommandsRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<SettingsPaneCommandsRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: SettingsPaneCommandsRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&SettingsPaneCommandsRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &SettingsPaneCommandsRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SettingsPaneCommandsRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SettingsPaneCommandsRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"UI_ApplicationSettings\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SupportedWebAccountActions(pub u32);
impl SupportedWebAccountActions {
    pub const None: Self = Self(0u32);
    pub const Reconnect: Self = Self(1u32);
    pub const Remove: Self = Self(2u32);
    pub const ViewDetails: Self = Self(4u32);
    pub const Manage: Self = Self(8u32);
    pub const More: Self = Self(16u32);
}
impl ::core::marker::Copy for SupportedWebAccountActions {}
impl ::core::clone::Clone for SupportedWebAccountActions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SupportedWebAccountActions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SupportedWebAccountActions {
    type Abi = Self;
}
impl ::core::fmt::Debug for SupportedWebAccountActions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SupportedWebAccountActions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for SupportedWebAccountActions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SupportedWebAccountActions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SupportedWebAccountActions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SupportedWebAccountActions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SupportedWebAccountActions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for SupportedWebAccountActions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.ApplicationSettings.SupportedWebAccountActions;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_ApplicationSettings\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WebAccountAction(pub i32);
impl WebAccountAction {
    pub const Reconnect: Self = Self(0i32);
    pub const Remove: Self = Self(1i32);
    pub const ViewDetails: Self = Self(2i32);
    pub const Manage: Self = Self(3i32);
    pub const More: Self = Self(4i32);
}
impl ::core::marker::Copy for WebAccountAction {}
impl ::core::clone::Clone for WebAccountAction {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WebAccountAction {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WebAccountAction {
    type Abi = Self;
}
impl ::core::fmt::Debug for WebAccountAction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebAccountAction").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebAccountAction {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.ApplicationSettings.WebAccountAction;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_ApplicationSettings\"`*"]
#[repr(transparent)]
pub struct WebAccountCommand(::windows::core::IUnknown);
impl WebAccountCommand {
    #[doc = "*Required features: `\"UI_ApplicationSettings\"`, `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn WebAccount(&self) -> ::windows::core::Result<super::super::Security::Credentials::WebAccount> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WebAccount)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Credentials::WebAccount>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ApplicationSettings\"`*"]
    pub fn Invoked(&self) -> ::windows::core::Result<WebAccountCommandInvokedHandler> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Invoked)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WebAccountCommandInvokedHandler>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ApplicationSettings\"`*"]
    pub fn Actions(&self) -> ::windows::core::Result<SupportedWebAccountActions> {
        let this = self;
        unsafe {
            let mut result__: SupportedWebAccountActions = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Actions)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SupportedWebAccountActions>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ApplicationSettings\"`, `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn CreateWebAccountCommand<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::Credentials::WebAccount>, Param1: ::windows::core::IntoParam<'a, WebAccountCommandInvokedHandler>>(webaccount: Param0, invoked: Param1, actions: SupportedWebAccountActions) -> ::windows::core::Result<WebAccountCommand> {
        Self::IWebAccountCommandFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateWebAccountCommand)(::core::mem::transmute_copy(this), webaccount.into_param().abi(), invoked.into_param().abi(), actions, &mut result__).from_abi::<WebAccountCommand>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWebAccountCommandFactory<R, F: FnOnce(&IWebAccountCommandFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WebAccountCommand, IWebAccountCommandFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for WebAccountCommand {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebAccountCommand {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebAccountCommand {}
impl ::core::fmt::Debug for WebAccountCommand {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebAccountCommand").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebAccountCommand {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.ApplicationSettings.WebAccountCommand;{caa39398-9cfa-4246-b0c4-a913a3896541})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for WebAccountCommand {
    type Vtable = IWebAccountCommand_Vtbl;
    const IID: ::windows::core::GUID = <IWebAccountCommand as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebAccountCommand {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.WebAccountCommand";
}
impl ::core::convert::From<WebAccountCommand> for ::windows::core::IUnknown {
    fn from(value: WebAccountCommand) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebAccountCommand> for ::windows::core::IUnknown {
    fn from(value: &WebAccountCommand) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WebAccountCommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WebAccountCommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebAccountCommand> for ::windows::core::IInspectable {
    fn from(value: WebAccountCommand) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebAccountCommand> for ::windows::core::IInspectable {
    fn from(value: &WebAccountCommand) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WebAccountCommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WebAccountCommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"UI_ApplicationSettings\"`*"]
#[repr(transparent)]
pub struct WebAccountCommandInvokedHandler(pub ::windows::core::IUnknown);
impl WebAccountCommandInvokedHandler {
    pub fn new<F: FnMut(&::core::option::Option<WebAccountCommand>, &::core::option::Option<WebAccountInvokedArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = WebAccountCommandInvokedHandlerBox::<F> { vtable: &WebAccountCommandInvokedHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `\"UI_ApplicationSettings\"`*"]
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, WebAccountCommand>, Param1: ::windows::core::IntoParam<'a, WebAccountInvokedArgs>>(&self, command: Param0, args: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Invoke)(::core::mem::transmute_copy(this), command.into_param().abi(), args.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct WebAccountCommandInvokedHandlerBox<F: FnMut(&::core::option::Option<WebAccountCommand>, &::core::option::Option<WebAccountInvokedArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const WebAccountCommandInvokedHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<WebAccountCommand>, &::core::option::Option<WebAccountInvokedArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> WebAccountCommandInvokedHandlerBox<F> {
    const VTABLE: WebAccountCommandInvokedHandler_Vtbl = WebAccountCommandInvokedHandler_Vtbl { base: ::windows::core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<WebAccountCommandInvokedHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
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
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, command: ::windows::core::RawPtr, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&command), ::core::mem::transmute(&args)).into()
    }
}
impl ::core::clone::Clone for WebAccountCommandInvokedHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebAccountCommandInvokedHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebAccountCommandInvokedHandler {}
impl ::core::fmt::Debug for WebAccountCommandInvokedHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebAccountCommandInvokedHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for WebAccountCommandInvokedHandler {
    type Vtable = WebAccountCommandInvokedHandler_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1ee6e459_1705_4a9a_b599_a0c3d6921973);
}
unsafe impl ::windows::core::RuntimeType for WebAccountCommandInvokedHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{1ee6e459-1705-4a9a-b599-a0c3d6921973}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct WebAccountCommandInvokedHandler_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, command: ::windows::core::RawPtr, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_ApplicationSettings\"`*"]
#[repr(transparent)]
pub struct WebAccountInvokedArgs(::windows::core::IUnknown);
impl WebAccountInvokedArgs {
    #[doc = "*Required features: `\"UI_ApplicationSettings\"`*"]
    pub fn Action(&self) -> ::windows::core::Result<WebAccountAction> {
        let this = self;
        unsafe {
            let mut result__: WebAccountAction = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Action)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WebAccountAction>(result__)
        }
    }
}
impl ::core::clone::Clone for WebAccountInvokedArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebAccountInvokedArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebAccountInvokedArgs {}
impl ::core::fmt::Debug for WebAccountInvokedArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebAccountInvokedArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebAccountInvokedArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.ApplicationSettings.WebAccountInvokedArgs;{e7abcc40-a1d8-4c5d-9a7f-1d34b2f90ad2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for WebAccountInvokedArgs {
    type Vtable = IWebAccountInvokedArgs_Vtbl;
    const IID: ::windows::core::GUID = <IWebAccountInvokedArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebAccountInvokedArgs {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.WebAccountInvokedArgs";
}
impl ::core::convert::From<WebAccountInvokedArgs> for ::windows::core::IUnknown {
    fn from(value: WebAccountInvokedArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebAccountInvokedArgs> for ::windows::core::IUnknown {
    fn from(value: &WebAccountInvokedArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WebAccountInvokedArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WebAccountInvokedArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebAccountInvokedArgs> for ::windows::core::IInspectable {
    fn from(value: WebAccountInvokedArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebAccountInvokedArgs> for ::windows::core::IInspectable {
    fn from(value: &WebAccountInvokedArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WebAccountInvokedArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WebAccountInvokedArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"UI_ApplicationSettings\"`*"]
#[repr(transparent)]
pub struct WebAccountProviderCommand(::windows::core::IUnknown);
impl WebAccountProviderCommand {
    #[doc = "*Required features: `\"UI_ApplicationSettings\"`, `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn WebAccountProvider(&self) -> ::windows::core::Result<super::super::Security::Credentials::WebAccountProvider> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WebAccountProvider)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Credentials::WebAccountProvider>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ApplicationSettings\"`*"]
    pub fn Invoked(&self) -> ::windows::core::Result<WebAccountProviderCommandInvokedHandler> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Invoked)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WebAccountProviderCommandInvokedHandler>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ApplicationSettings\"`, `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn CreateWebAccountProviderCommand<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::Credentials::WebAccountProvider>, Param1: ::windows::core::IntoParam<'a, WebAccountProviderCommandInvokedHandler>>(webaccountprovider: Param0, invoked: Param1) -> ::windows::core::Result<WebAccountProviderCommand> {
        Self::IWebAccountProviderCommandFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateWebAccountProviderCommand)(::core::mem::transmute_copy(this), webaccountprovider.into_param().abi(), invoked.into_param().abi(), &mut result__).from_abi::<WebAccountProviderCommand>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWebAccountProviderCommandFactory<R, F: FnOnce(&IWebAccountProviderCommandFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WebAccountProviderCommand, IWebAccountProviderCommandFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for WebAccountProviderCommand {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebAccountProviderCommand {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebAccountProviderCommand {}
impl ::core::fmt::Debug for WebAccountProviderCommand {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebAccountProviderCommand").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebAccountProviderCommand {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.ApplicationSettings.WebAccountProviderCommand;{d69bdd9a-a0a6-4e9b-88dc-c71e757a3501})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for WebAccountProviderCommand {
    type Vtable = IWebAccountProviderCommand_Vtbl;
    const IID: ::windows::core::GUID = <IWebAccountProviderCommand as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebAccountProviderCommand {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.WebAccountProviderCommand";
}
impl ::core::convert::From<WebAccountProviderCommand> for ::windows::core::IUnknown {
    fn from(value: WebAccountProviderCommand) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebAccountProviderCommand> for ::windows::core::IUnknown {
    fn from(value: &WebAccountProviderCommand) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WebAccountProviderCommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WebAccountProviderCommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebAccountProviderCommand> for ::windows::core::IInspectable {
    fn from(value: WebAccountProviderCommand) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebAccountProviderCommand> for ::windows::core::IInspectable {
    fn from(value: &WebAccountProviderCommand) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WebAccountProviderCommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WebAccountProviderCommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"UI_ApplicationSettings\"`*"]
#[repr(transparent)]
pub struct WebAccountProviderCommandInvokedHandler(pub ::windows::core::IUnknown);
impl WebAccountProviderCommandInvokedHandler {
    pub fn new<F: FnMut(&::core::option::Option<WebAccountProviderCommand>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = WebAccountProviderCommandInvokedHandlerBox::<F> { vtable: &WebAccountProviderCommandInvokedHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `\"UI_ApplicationSettings\"`*"]
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, WebAccountProviderCommand>>(&self, command: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Invoke)(::core::mem::transmute_copy(this), command.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct WebAccountProviderCommandInvokedHandlerBox<F: FnMut(&::core::option::Option<WebAccountProviderCommand>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const WebAccountProviderCommandInvokedHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<WebAccountProviderCommand>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> WebAccountProviderCommandInvokedHandlerBox<F> {
    const VTABLE: WebAccountProviderCommandInvokedHandler_Vtbl = WebAccountProviderCommandInvokedHandler_Vtbl { base: ::windows::core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<WebAccountProviderCommandInvokedHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
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
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, command: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&command)).into()
    }
}
impl ::core::clone::Clone for WebAccountProviderCommandInvokedHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebAccountProviderCommandInvokedHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebAccountProviderCommandInvokedHandler {}
impl ::core::fmt::Debug for WebAccountProviderCommandInvokedHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebAccountProviderCommandInvokedHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for WebAccountProviderCommandInvokedHandler {
    type Vtable = WebAccountProviderCommandInvokedHandler_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb7de5527_4c8f_42dd_84da_5ec493abdb9a);
}
unsafe impl ::windows::core::RuntimeType for WebAccountProviderCommandInvokedHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{b7de5527-4c8f-42dd-84da-5ec493abdb9a}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct WebAccountProviderCommandInvokedHandler_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, command: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
