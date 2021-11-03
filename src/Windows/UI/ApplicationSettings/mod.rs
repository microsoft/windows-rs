#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `UI_ApplicationSettings`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct AccountsSettingsPane(::windows::runtime::IInspectable);
impl AccountsSettingsPane {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_ApplicationSettings`, `Foundation`*"]
    pub fn AccountCommandsRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<AccountsSettingsPane, AccountsSettingsPaneCommandsRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_ApplicationSettings`, `Foundation`*"]
    pub fn RemoveAccountCommandsRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_ApplicationSettings`*"]
    pub fn GetForCurrentView() -> ::windows::runtime::Result<AccountsSettingsPane> {
        Self::IAccountsSettingsPaneStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AccountsSettingsPane>(result__)
        })
    }
    #[doc = "*Required features: `UI_ApplicationSettings`*"]
    pub fn Show() -> ::windows::runtime::Result<()> {
        Self::IAccountsSettingsPaneStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this)).ok() })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_ApplicationSettings`, `Foundation`*"]
    pub fn ShowManageAccountsAsync() -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        Self::IAccountsSettingsPaneStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_ApplicationSettings`, `Foundation`*"]
    pub fn ShowAddAccountAsync() -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        Self::IAccountsSettingsPaneStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "System"))]
    #[doc = "*Required features: `UI_ApplicationSettings`, `Foundation`, `System`*"]
    pub fn ShowManageAccountsForUserAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::User>>(user: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        Self::IAccountsSettingsPaneStatics3(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "System"))]
    #[doc = "*Required features: `UI_ApplicationSettings`, `Foundation`, `System`*"]
    pub fn ShowAddAccountForUserAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::User>>(user: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        Self::IAccountsSettingsPaneStatics3(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    pub fn IAccountsSettingsPaneStatics<R, F: FnOnce(&IAccountsSettingsPaneStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<AccountsSettingsPane, IAccountsSettingsPaneStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAccountsSettingsPaneStatics2<R, F: FnOnce(&IAccountsSettingsPaneStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<AccountsSettingsPane, IAccountsSettingsPaneStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAccountsSettingsPaneStatics3<R, F: FnOnce(&IAccountsSettingsPaneStatics3) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<AccountsSettingsPane, IAccountsSettingsPaneStatics3> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AccountsSettingsPane {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.ApplicationSettings.AccountsSettingsPane;{81ea942c-4f09-4406-a538-838d9b14b7e6})");
}
unsafe impl ::windows::runtime::Interface for AccountsSettingsPane {
    type Vtable = IAccountsSettingsPane_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2179634220, 20233, 17414, [165, 56, 131, 141, 155, 20, 183, 230]);
}
impl ::windows::runtime::RuntimeName for AccountsSettingsPane {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.AccountsSettingsPane";
}
#[doc = "*Required features: `UI_ApplicationSettings`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct AccountsSettingsPaneCommandsRequestedEventArgs(::windows::runtime::IInspectable);
impl AccountsSettingsPaneCommandsRequestedEventArgs {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_ApplicationSettings`, `Foundation_Collections`*"]
    pub fn WebAccountProviderCommands(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<WebAccountProviderCommand>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<WebAccountProviderCommand>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_ApplicationSettings`, `Foundation_Collections`*"]
    pub fn WebAccountCommands(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<WebAccountCommand>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<WebAccountCommand>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_ApplicationSettings`, `Foundation_Collections`*"]
    pub fn CredentialCommands(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<CredentialCommand>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<CredentialCommand>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Popups"))]
    #[doc = "*Required features: `UI_ApplicationSettings`, `Foundation_Collections`, `UI_Popups`*"]
    pub fn Commands(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<SettingsCommand>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<SettingsCommand>>(result__)
        }
    }
    #[doc = "*Required features: `UI_ApplicationSettings`*"]
    pub fn HeaderText(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_ApplicationSettings`*"]
    pub fn SetHeaderText<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_ApplicationSettings`*"]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<AccountsSettingsPaneEventDeferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AccountsSettingsPaneEventDeferral>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `UI_ApplicationSettings`, `System`*"]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = &::windows::runtime::Interface::cast::<IAccountsSettingsPaneCommandsRequestedEventArgs2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AccountsSettingsPaneCommandsRequestedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.ApplicationSettings.AccountsSettingsPaneCommandsRequestedEventArgs;{3b68c099-db19-45d0-9abf-95d3773c9330})");
}
unsafe impl ::windows::runtime::Interface for AccountsSettingsPaneCommandsRequestedEventArgs {
    type Vtable = IAccountsSettingsPaneCommandsRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(996720793, 56089, 17872, [154, 191, 149, 211, 119, 60, 147, 48]);
}
impl ::windows::runtime::RuntimeName for AccountsSettingsPaneCommandsRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.AccountsSettingsPaneCommandsRequestedEventArgs";
}
#[doc = "*Required features: `UI_ApplicationSettings`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct AccountsSettingsPaneEventDeferral(::windows::runtime::IInspectable);
impl AccountsSettingsPaneEventDeferral {
    #[doc = "*Required features: `UI_ApplicationSettings`*"]
    pub fn Complete(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AccountsSettingsPaneEventDeferral {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.ApplicationSettings.AccountsSettingsPaneEventDeferral;{cbf25d3f-e5ba-40ef-93da-65e096e5fb04})");
}
unsafe impl ::windows::runtime::Interface for AccountsSettingsPaneEventDeferral {
    type Vtable = IAccountsSettingsPaneEventDeferral_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3421658431, 58810, 16623, [147, 218, 101, 224, 150, 229, 251, 4]);
}
impl ::windows::runtime::RuntimeName for AccountsSettingsPaneEventDeferral {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.AccountsSettingsPaneEventDeferral";
}
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct ApplicationsSettingsContract(pub u8);
#[doc = "*Required features: `UI_ApplicationSettings`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct CredentialCommand(::windows::runtime::IInspectable);
impl CredentialCommand {
    #[cfg(feature = "Security_Credentials")]
    #[doc = "*Required features: `UI_ApplicationSettings`, `Security_Credentials`*"]
    pub fn PasswordCredential(&self) -> ::windows::runtime::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Credentials::PasswordCredential>(result__)
        }
    }
    #[doc = "*Required features: `UI_ApplicationSettings`*"]
    pub fn CredentialDeleted(&self) -> ::windows::runtime::Result<CredentialCommandCredentialDeletedHandler> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CredentialCommandCredentialDeletedHandler>(result__)
        }
    }
    #[cfg(feature = "Security_Credentials")]
    #[doc = "*Required features: `UI_ApplicationSettings`, `Security_Credentials`*"]
    pub fn CreateCredentialCommand<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Security::Credentials::PasswordCredential>>(passwordcredential: Param0) -> ::windows::runtime::Result<CredentialCommand> {
        Self::ICredentialCommandFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), passwordcredential.into_param().abi(), &mut result__).from_abi::<CredentialCommand>(result__)
        })
    }
    #[cfg(feature = "Security_Credentials")]
    #[doc = "*Required features: `UI_ApplicationSettings`, `Security_Credentials`*"]
    pub fn CreateCredentialCommandWithHandler<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Security::Credentials::PasswordCredential>, Param1: ::windows::runtime::IntoParam<'a, CredentialCommandCredentialDeletedHandler>>(passwordcredential: Param0, deleted: Param1) -> ::windows::runtime::Result<CredentialCommand> {
        Self::ICredentialCommandFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), passwordcredential.into_param().abi(), deleted.into_param().abi(), &mut result__).from_abi::<CredentialCommand>(result__)
        })
    }
    pub fn ICredentialCommandFactory<R, F: FnOnce(&ICredentialCommandFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<CredentialCommand, ICredentialCommandFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CredentialCommand {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.ApplicationSettings.CredentialCommand;{a5f665e6-6143-4a7a-a971-b017ba978ce2})");
}
unsafe impl ::windows::runtime::Interface for CredentialCommand {
    type Vtable = ICredentialCommand_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2784388582, 24899, 19066, [169, 113, 176, 23, 186, 151, 140, 226]);
}
impl ::windows::runtime::RuntimeName for CredentialCommand {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.CredentialCommand";
}
#[doc = "*Required features: `UI_ApplicationSettings`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct CredentialCommandCredentialDeletedHandler(::windows::runtime::IUnknown);
impl CredentialCommandCredentialDeletedHandler {
    pub fn new<F: FnMut(&::std::option::Option<CredentialCommand>) -> ::windows::runtime::Result<()> + 'static>(invoke: F) -> Self {
        let com = CredentialCommandCredentialDeletedHandler_box::<F> {
            vtable: &CredentialCommandCredentialDeletedHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { std::mem::transmute(::std::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `UI_ApplicationSettings`*"]
    pub fn Invoke<'a, Param0: ::windows::runtime::IntoParam<'a, CredentialCommand>>(&self, command: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).3)(::std::mem::transmute_copy(this), command.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CredentialCommandCredentialDeletedHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"delegate({61c0e185-0977-4678-b4e2-98727afbeed9})");
}
unsafe impl ::windows::runtime::Interface for CredentialCommandCredentialDeletedHandler {
    type Vtable = CredentialCommandCredentialDeletedHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1640030597, 2423, 18040, [180, 226, 152, 114, 122, 251, 238, 217]);
}
#[repr(C)]
#[doc(hidden)]
pub struct CredentialCommandCredentialDeletedHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, command: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
struct CredentialCommandCredentialDeletedHandler_box<F: FnMut(&::std::option::Option<CredentialCommand>) -> ::windows::runtime::Result<()> + 'static> {
    vtable: *const CredentialCommandCredentialDeletedHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<F: FnMut(&::std::option::Option<CredentialCommand>) -> ::windows::runtime::Result<()> + 'static> CredentialCommandCredentialDeletedHandler_box<F> {
    const VTABLE: CredentialCommandCredentialDeletedHandler_abi = CredentialCommandCredentialDeletedHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<CredentialCommandCredentialDeletedHandler as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::std::ptr::null_mut()
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
            Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::runtime::RawPtr, command: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ((*this).invoke)(&*(&command as *const <CredentialCommand as ::windows::runtime::Abi>::Abi as *const <CredentialCommand as ::windows::runtime::Abi>::DefaultType)).into()
    }
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IAccountsSettingsPane(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAccountsSettingsPane {
    type Vtable = IAccountsSettingsPane_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2179634220, 20233, 17414, [165, 56, 131, 141, 155, 20, 183, 230]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccountsSettingsPane_abi(
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
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IAccountsSettingsPaneCommandsRequestedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAccountsSettingsPaneCommandsRequestedEventArgs {
    type Vtable = IAccountsSettingsPaneCommandsRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(996720793, 56089, 17872, [154, 191, 149, 211, 119, 60, 147, 48]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccountsSettingsPaneCommandsRequestedEventArgs_abi(
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
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Popups"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Popups")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IAccountsSettingsPaneCommandsRequestedEventArgs2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAccountsSettingsPaneCommandsRequestedEventArgs2 {
    type Vtable = IAccountsSettingsPaneCommandsRequestedEventArgs2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(909081517, 20023, 18791, [140, 64, 231, 142, 231, 161, 229, 187]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccountsSettingsPaneCommandsRequestedEventArgs2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IAccountsSettingsPaneEventDeferral(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAccountsSettingsPaneEventDeferral {
    type Vtable = IAccountsSettingsPaneEventDeferral_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3421658431, 58810, 16623, [147, 218, 101, 224, 150, 229, 251, 4]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccountsSettingsPaneEventDeferral_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IAccountsSettingsPaneStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAccountsSettingsPaneStatics {
    type Vtable = IAccountsSettingsPaneStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1444907872, 45292, 16720, [168, 220, 32, 142, 228, 75, 6, 138]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccountsSettingsPaneStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IAccountsSettingsPaneStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAccountsSettingsPaneStatics2 {
    type Vtable = IAccountsSettingsPaneStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3525179330, 52749, 18511, [184, 232, 232, 35, 194, 21, 118, 94]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccountsSettingsPaneStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IAccountsSettingsPaneStatics3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAccountsSettingsPaneStatics3 {
    type Vtable = IAccountsSettingsPaneStatics3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(138478680, 41658, 19567, [180, 172, 72, 245, 20, 51, 18, 22]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccountsSettingsPaneStatics3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "System"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, user: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))] usize,
    #[cfg(all(feature = "Foundation", feature = "System"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, user: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ICredentialCommand(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICredentialCommand {
    type Vtable = ICredentialCommand_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2784388582, 24899, 19066, [169, 113, 176, 23, 186, 151, 140, 226]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICredentialCommand_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Security_Credentials")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ICredentialCommandFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICredentialCommandFactory {
    type Vtable = ICredentialCommandFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(669551639, 48190, 19328, [148, 149, 78, 215, 32, 228, 138, 145]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICredentialCommandFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Security_Credentials")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, passwordcredential: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))] usize,
    #[cfg(feature = "Security_Credentials")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, passwordcredential: ::windows::runtime::RawPtr, deleted: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ISettingsCommandFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISettingsCommandFactory {
    type Vtable = ISettingsCommandFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1759599411, 7299, 17210, [170, 90, 206, 238, 165, 189, 71, 100]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISettingsCommandFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI_Popups")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, settingscommandid: ::windows::runtime::RawPtr, label: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, handler: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Popups"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ISettingsCommandStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISettingsCommandStatics {
    type Vtable = ISettingsCommandStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1956309332, 12137, 19223, [138, 186, 208, 92, 229, 119, 142, 70]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISettingsCommandStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI_Popups")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Popups"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ISettingsPane(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISettingsPane {
    type Vtable = ISettingsPane_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2983004466, 17776, 19561, [141, 56, 137, 68, 101, 97, 172, 224]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISettingsPane_abi(
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
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ISettingsPaneCommandsRequest(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISettingsPaneCommandsRequest {
    type Vtable = ISettingsPaneCommandsRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1155474350, 23918, 16488, [161, 104, 244, 118, 67, 24, 33, 20]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISettingsPaneCommandsRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Popups"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Popups")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ISettingsPaneCommandsRequestedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISettingsPaneCommandsRequestedEventArgs {
    type Vtable = ISettingsPaneCommandsRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(543120676, 6984, 17961, [166, 202, 47, 223, 237, 175, 183, 93]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISettingsPaneCommandsRequestedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ISettingsPaneStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISettingsPaneStatics {
    type Vtable = ISettingsPaneStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(476730053, 65305, 18203, [186, 107, 248, 243, 86, 148, 173, 154]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISettingsPaneStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SettingsEdgeLocation) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IWebAccountCommand(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWebAccountCommand {
    type Vtable = IWebAccountCommand_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3399717784, 40186, 16966, [176, 196, 169, 19, 163, 137, 101, 65]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountCommand_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Security_Credentials")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SupportedWebAccountActions) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IWebAccountCommandFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWebAccountCommandFactory {
    type Vtable = IWebAccountCommandFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3215379967, 12077, 17141, [129, 222, 29, 86, 186, 252, 73, 109]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountCommandFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Security_Credentials")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, webaccount: ::windows::runtime::RawPtr, invoked: ::windows::runtime::RawPtr, actions: SupportedWebAccountActions, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IWebAccountInvokedArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWebAccountInvokedArgs {
    type Vtable = IWebAccountInvokedArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3886795840, 41432, 19549, [154, 127, 29, 52, 178, 249, 10, 210]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountInvokedArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut WebAccountAction) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IWebAccountProviderCommand(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWebAccountProviderCommand {
    type Vtable = IWebAccountProviderCommand_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3600539034, 41126, 20123, [136, 220, 199, 30, 117, 122, 53, 1]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountProviderCommand_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Security_Credentials")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IWebAccountProviderCommandFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWebAccountProviderCommandFactory {
    type Vtable = IWebAccountProviderCommandFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3580201499, 45430, 18294, [132, 105, 169, 211, 255, 11, 63, 89]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountProviderCommandFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Security_Credentials")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, webaccountprovider: ::windows::runtime::RawPtr, invoked: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))] usize,
);
#[cfg(feature = "UI_Popups")]
#[doc = "*Required features: `UI_ApplicationSettings`, `UI_Popups`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct SettingsCommand(::windows::runtime::IInspectable);
#[cfg(feature = "UI_Popups")]
impl SettingsCommand {
    #[cfg(feature = "UI_Popups")]
    #[doc = "*Required features: `UI_ApplicationSettings`, `UI_Popups`*"]
    pub fn Label(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "UI_Popups")]
    #[doc = "*Required features: `UI_ApplicationSettings`, `UI_Popups`*"]
    pub fn SetLabel<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Popups")]
    #[doc = "*Required features: `UI_ApplicationSettings`, `UI_Popups`*"]
    pub fn Invoked(&self) -> ::windows::runtime::Result<super::Popups::UICommandInvokedHandler> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::Popups::UICommandInvokedHandler>(result__)
        }
    }
    #[cfg(feature = "UI_Popups")]
    #[doc = "*Required features: `UI_ApplicationSettings`, `UI_Popups`*"]
    pub fn SetInvoked<'a, Param0: ::windows::runtime::IntoParam<'a, super::Popups::UICommandInvokedHandler>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Popups")]
    #[doc = "*Required features: `UI_ApplicationSettings`, `UI_Popups`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Popups")]
    #[doc = "*Required features: `UI_ApplicationSettings`, `UI_Popups`*"]
    pub fn SetId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Popups")]
    #[doc = "*Required features: `UI_ApplicationSettings`, `UI_Popups`*"]
    pub fn CreateSettingsCommand<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, super::Popups::UICommandInvokedHandler>>(settingscommandid: Param0, label: Param1, handler: Param2) -> ::windows::runtime::Result<SettingsCommand> {
        Self::ISettingsCommandFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), settingscommandid.into_param().abi(), label.into_param().abi(), handler.into_param().abi(), &mut result__).from_abi::<SettingsCommand>(result__)
        })
    }
    #[cfg(feature = "UI_Popups")]
    #[doc = "*Required features: `UI_ApplicationSettings`, `UI_Popups`*"]
    pub fn AccountsCommand() -> ::windows::runtime::Result<SettingsCommand> {
        Self::ISettingsCommandStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SettingsCommand>(result__)
        })
    }
    pub fn ISettingsCommandFactory<R, F: FnOnce(&ISettingsCommandFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SettingsCommand, ISettingsCommandFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ISettingsCommandStatics<R, F: FnOnce(&ISettingsCommandStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SettingsCommand, ISettingsCommandStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "UI_Popups")]
unsafe impl ::windows::runtime::RuntimeType for SettingsCommand {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.ApplicationSettings.SettingsCommand;{4ff93a75-4145-47ff-ac7f-dff1c1fa5b0f})");
}
#[cfg(feature = "UI_Popups")]
unsafe impl ::windows::runtime::Interface for SettingsCommand {
    type Vtable = super::Popups::IUICommand_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1341733493, 16709, 18431, [172, 127, 223, 241, 193, 250, 91, 15]);
}
#[cfg(feature = "UI_Popups")]
impl ::windows::runtime::RuntimeName for SettingsCommand {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.SettingsCommand";
}
#[cfg(feature = "UI_Popups")]
impl ::std::convert::From<SettingsCommand> for super::Popups::IUICommand {
    fn from(value: SettingsCommand) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "UI_Popups")]
impl ::std::convert::From<&SettingsCommand> for super::Popups::IUICommand {
    fn from(value: &SettingsCommand) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "UI_Popups")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Popups::IUICommand> for SettingsCommand {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Popups::IUICommand> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Popups::IUICommand>::into(self))
    }
}
#[cfg(feature = "UI_Popups")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Popups::IUICommand> for &SettingsCommand {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Popups::IUICommand> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Popups::IUICommand>::into(::std::clone::Clone::clone(self)))
    }
}
#[doc = "*Required features: `UI_ApplicationSettings`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SettingsEdgeLocation(pub i32);
impl SettingsEdgeLocation {
    pub const Right: SettingsEdgeLocation = SettingsEdgeLocation(0i32);
    pub const Left: SettingsEdgeLocation = SettingsEdgeLocation(1i32);
}
impl ::std::convert::From<i32> for SettingsEdgeLocation {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SettingsEdgeLocation {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SettingsEdgeLocation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.ApplicationSettings.SettingsEdgeLocation;i4)");
}
#[doc = "*Required features: `UI_ApplicationSettings`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct SettingsPane(::windows::runtime::IInspectable);
impl SettingsPane {
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_ApplicationSettings`, `Foundation`*"]
    pub fn CommandsRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<SettingsPane, SettingsPaneCommandsRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_ApplicationSettings`, `Foundation`*"]
    pub fn RemoveCommandsRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `UI_ApplicationSettings`*"]
    pub fn GetForCurrentView() -> ::windows::runtime::Result<SettingsPane> {
        Self::ISettingsPaneStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SettingsPane>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `UI_ApplicationSettings`*"]
    pub fn Show() -> ::windows::runtime::Result<()> {
        Self::ISettingsPaneStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this)).ok() })
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `UI_ApplicationSettings`*"]
    pub fn Edge() -> ::windows::runtime::Result<SettingsEdgeLocation> {
        Self::ISettingsPaneStatics(|this| unsafe {
            let mut result__: SettingsEdgeLocation = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SettingsEdgeLocation>(result__)
        })
    }
    pub fn ISettingsPaneStatics<R, F: FnOnce(&ISettingsPaneStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SettingsPane, ISettingsPaneStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SettingsPane {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.ApplicationSettings.SettingsPane;{b1cd0932-4570-4c69-8d38-89446561ace0})");
}
unsafe impl ::windows::runtime::Interface for SettingsPane {
    type Vtable = ISettingsPane_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2983004466, 17776, 19561, [141, 56, 137, 68, 101, 97, 172, 224]);
}
impl ::windows::runtime::RuntimeName for SettingsPane {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.SettingsPane";
}
#[doc = "*Required features: `UI_ApplicationSettings`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct SettingsPaneCommandsRequest(::windows::runtime::IInspectable);
impl SettingsPaneCommandsRequest {
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Popups"))]
    #[doc = "*Required features: `UI_ApplicationSettings`, `Foundation_Collections`, `UI_Popups`*"]
    pub fn ApplicationCommands(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<SettingsCommand>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<SettingsCommand>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SettingsPaneCommandsRequest {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.ApplicationSettings.SettingsPaneCommandsRequest;{44df23ae-5d6e-4068-a168-f47643182114})");
}
unsafe impl ::windows::runtime::Interface for SettingsPaneCommandsRequest {
    type Vtable = ISettingsPaneCommandsRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1155474350, 23918, 16488, [161, 104, 244, 118, 67, 24, 33, 20]);
}
impl ::windows::runtime::RuntimeName for SettingsPaneCommandsRequest {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.SettingsPaneCommandsRequest";
}
#[doc = "*Required features: `UI_ApplicationSettings`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct SettingsPaneCommandsRequestedEventArgs(::windows::runtime::IInspectable);
impl SettingsPaneCommandsRequestedEventArgs {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `UI_ApplicationSettings`*"]
    pub fn Request(&self) -> ::windows::runtime::Result<SettingsPaneCommandsRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SettingsPaneCommandsRequest>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SettingsPaneCommandsRequestedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.ApplicationSettings.SettingsPaneCommandsRequestedEventArgs;{205f5d24-1b48-4629-a6ca-2fdfedafb75d})");
}
unsafe impl ::windows::runtime::Interface for SettingsPaneCommandsRequestedEventArgs {
    type Vtable = ISettingsPaneCommandsRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(543120676, 6984, 17961, [166, 202, 47, 223, 237, 175, 183, 93]);
}
impl ::windows::runtime::RuntimeName for SettingsPaneCommandsRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.SettingsPaneCommandsRequestedEventArgs";
}
#[doc = "*Required features: `UI_ApplicationSettings`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SupportedWebAccountActions(pub u32);
impl SupportedWebAccountActions {
    pub const None: SupportedWebAccountActions = SupportedWebAccountActions(0u32);
    pub const Reconnect: SupportedWebAccountActions = SupportedWebAccountActions(1u32);
    pub const Remove: SupportedWebAccountActions = SupportedWebAccountActions(2u32);
    pub const ViewDetails: SupportedWebAccountActions = SupportedWebAccountActions(4u32);
    pub const Manage: SupportedWebAccountActions = SupportedWebAccountActions(8u32);
    pub const More: SupportedWebAccountActions = SupportedWebAccountActions(16u32);
}
impl ::std::convert::From<u32> for SupportedWebAccountActions {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SupportedWebAccountActions {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SupportedWebAccountActions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.ApplicationSettings.SupportedWebAccountActions;u4)");
}
impl ::std::ops::BitOr for SupportedWebAccountActions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for SupportedWebAccountActions {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for SupportedWebAccountActions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for SupportedWebAccountActions {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for SupportedWebAccountActions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `UI_ApplicationSettings`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WebAccountAction(pub i32);
impl WebAccountAction {
    pub const Reconnect: WebAccountAction = WebAccountAction(0i32);
    pub const Remove: WebAccountAction = WebAccountAction(1i32);
    pub const ViewDetails: WebAccountAction = WebAccountAction(2i32);
    pub const Manage: WebAccountAction = WebAccountAction(3i32);
    pub const More: WebAccountAction = WebAccountAction(4i32);
}
impl ::std::convert::From<i32> for WebAccountAction {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WebAccountAction {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for WebAccountAction {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.ApplicationSettings.WebAccountAction;i4)");
}
#[doc = "*Required features: `UI_ApplicationSettings`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct WebAccountCommand(::windows::runtime::IInspectable);
impl WebAccountCommand {
    #[cfg(feature = "Security_Credentials")]
    #[doc = "*Required features: `UI_ApplicationSettings`, `Security_Credentials`*"]
    pub fn WebAccount(&self) -> ::windows::runtime::Result<super::super::Security::Credentials::WebAccount> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Credentials::WebAccount>(result__)
        }
    }
    #[doc = "*Required features: `UI_ApplicationSettings`*"]
    pub fn Invoked(&self) -> ::windows::runtime::Result<WebAccountCommandInvokedHandler> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<WebAccountCommandInvokedHandler>(result__)
        }
    }
    #[doc = "*Required features: `UI_ApplicationSettings`*"]
    pub fn Actions(&self) -> ::windows::runtime::Result<SupportedWebAccountActions> {
        let this = self;
        unsafe {
            let mut result__: SupportedWebAccountActions = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SupportedWebAccountActions>(result__)
        }
    }
    #[cfg(feature = "Security_Credentials")]
    #[doc = "*Required features: `UI_ApplicationSettings`, `Security_Credentials`*"]
    pub fn CreateWebAccountCommand<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Security::Credentials::WebAccount>, Param1: ::windows::runtime::IntoParam<'a, WebAccountCommandInvokedHandler>>(webaccount: Param0, invoked: Param1, actions: SupportedWebAccountActions) -> ::windows::runtime::Result<WebAccountCommand> {
        Self::IWebAccountCommandFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), webaccount.into_param().abi(), invoked.into_param().abi(), actions, &mut result__).from_abi::<WebAccountCommand>(result__)
        })
    }
    pub fn IWebAccountCommandFactory<R, F: FnOnce(&IWebAccountCommandFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<WebAccountCommand, IWebAccountCommandFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for WebAccountCommand {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.ApplicationSettings.WebAccountCommand;{caa39398-9cfa-4246-b0c4-a913a3896541})");
}
unsafe impl ::windows::runtime::Interface for WebAccountCommand {
    type Vtable = IWebAccountCommand_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3399717784, 40186, 16966, [176, 196, 169, 19, 163, 137, 101, 65]);
}
impl ::windows::runtime::RuntimeName for WebAccountCommand {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.WebAccountCommand";
}
#[doc = "*Required features: `UI_ApplicationSettings`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct WebAccountCommandInvokedHandler(::windows::runtime::IUnknown);
impl WebAccountCommandInvokedHandler {
    pub fn new<F: FnMut(&::std::option::Option<WebAccountCommand>, &::std::option::Option<WebAccountInvokedArgs>) -> ::windows::runtime::Result<()> + 'static>(invoke: F) -> Self {
        let com = WebAccountCommandInvokedHandler_box::<F> {
            vtable: &WebAccountCommandInvokedHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { std::mem::transmute(::std::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `UI_ApplicationSettings`*"]
    pub fn Invoke<'a, Param0: ::windows::runtime::IntoParam<'a, WebAccountCommand>, Param1: ::windows::runtime::IntoParam<'a, WebAccountInvokedArgs>>(&self, command: Param0, args: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).3)(::std::mem::transmute_copy(this), command.into_param().abi(), args.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for WebAccountCommandInvokedHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"delegate({1ee6e459-1705-4a9a-b599-a0c3d6921973})");
}
unsafe impl ::windows::runtime::Interface for WebAccountCommandInvokedHandler {
    type Vtable = WebAccountCommandInvokedHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(518448217, 5893, 19098, [181, 153, 160, 195, 214, 146, 25, 115]);
}
#[repr(C)]
#[doc(hidden)]
pub struct WebAccountCommandInvokedHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, command: ::windows::runtime::RawPtr, args: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
struct WebAccountCommandInvokedHandler_box<F: FnMut(&::std::option::Option<WebAccountCommand>, &::std::option::Option<WebAccountInvokedArgs>) -> ::windows::runtime::Result<()> + 'static> {
    vtable: *const WebAccountCommandInvokedHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<F: FnMut(&::std::option::Option<WebAccountCommand>, &::std::option::Option<WebAccountInvokedArgs>) -> ::windows::runtime::Result<()> + 'static> WebAccountCommandInvokedHandler_box<F> {
    const VTABLE: WebAccountCommandInvokedHandler_abi = WebAccountCommandInvokedHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<WebAccountCommandInvokedHandler as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::std::ptr::null_mut()
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
            Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::runtime::RawPtr, command: ::windows::runtime::RawPtr, args: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ((*this).invoke)(&*(&command as *const <WebAccountCommand as ::windows::runtime::Abi>::Abi as *const <WebAccountCommand as ::windows::runtime::Abi>::DefaultType), &*(&args as *const <WebAccountInvokedArgs as ::windows::runtime::Abi>::Abi as *const <WebAccountInvokedArgs as ::windows::runtime::Abi>::DefaultType)).into()
    }
}
#[doc = "*Required features: `UI_ApplicationSettings`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct WebAccountInvokedArgs(::windows::runtime::IInspectable);
impl WebAccountInvokedArgs {
    #[doc = "*Required features: `UI_ApplicationSettings`*"]
    pub fn Action(&self) -> ::windows::runtime::Result<WebAccountAction> {
        let this = self;
        unsafe {
            let mut result__: WebAccountAction = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<WebAccountAction>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for WebAccountInvokedArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.ApplicationSettings.WebAccountInvokedArgs;{e7abcc40-a1d8-4c5d-9a7f-1d34b2f90ad2})");
}
unsafe impl ::windows::runtime::Interface for WebAccountInvokedArgs {
    type Vtable = IWebAccountInvokedArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3886795840, 41432, 19549, [154, 127, 29, 52, 178, 249, 10, 210]);
}
impl ::windows::runtime::RuntimeName for WebAccountInvokedArgs {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.WebAccountInvokedArgs";
}
#[doc = "*Required features: `UI_ApplicationSettings`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct WebAccountProviderCommand(::windows::runtime::IInspectable);
impl WebAccountProviderCommand {
    #[cfg(feature = "Security_Credentials")]
    #[doc = "*Required features: `UI_ApplicationSettings`, `Security_Credentials`*"]
    pub fn WebAccountProvider(&self) -> ::windows::runtime::Result<super::super::Security::Credentials::WebAccountProvider> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Credentials::WebAccountProvider>(result__)
        }
    }
    #[doc = "*Required features: `UI_ApplicationSettings`*"]
    pub fn Invoked(&self) -> ::windows::runtime::Result<WebAccountProviderCommandInvokedHandler> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<WebAccountProviderCommandInvokedHandler>(result__)
        }
    }
    #[cfg(feature = "Security_Credentials")]
    #[doc = "*Required features: `UI_ApplicationSettings`, `Security_Credentials`*"]
    pub fn CreateWebAccountProviderCommand<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Security::Credentials::WebAccountProvider>, Param1: ::windows::runtime::IntoParam<'a, WebAccountProviderCommandInvokedHandler>>(webaccountprovider: Param0, invoked: Param1) -> ::windows::runtime::Result<WebAccountProviderCommand> {
        Self::IWebAccountProviderCommandFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), webaccountprovider.into_param().abi(), invoked.into_param().abi(), &mut result__).from_abi::<WebAccountProviderCommand>(result__)
        })
    }
    pub fn IWebAccountProviderCommandFactory<R, F: FnOnce(&IWebAccountProviderCommandFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<WebAccountProviderCommand, IWebAccountProviderCommandFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for WebAccountProviderCommand {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.ApplicationSettings.WebAccountProviderCommand;{d69bdd9a-a0a6-4e9b-88dc-c71e757a3501})");
}
unsafe impl ::windows::runtime::Interface for WebAccountProviderCommand {
    type Vtable = IWebAccountProviderCommand_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3600539034, 41126, 20123, [136, 220, 199, 30, 117, 122, 53, 1]);
}
impl ::windows::runtime::RuntimeName for WebAccountProviderCommand {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.WebAccountProviderCommand";
}
#[doc = "*Required features: `UI_ApplicationSettings`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct WebAccountProviderCommandInvokedHandler(::windows::runtime::IUnknown);
impl WebAccountProviderCommandInvokedHandler {
    pub fn new<F: FnMut(&::std::option::Option<WebAccountProviderCommand>) -> ::windows::runtime::Result<()> + 'static>(invoke: F) -> Self {
        let com = WebAccountProviderCommandInvokedHandler_box::<F> {
            vtable: &WebAccountProviderCommandInvokedHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { std::mem::transmute(::std::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `UI_ApplicationSettings`*"]
    pub fn Invoke<'a, Param0: ::windows::runtime::IntoParam<'a, WebAccountProviderCommand>>(&self, command: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).3)(::std::mem::transmute_copy(this), command.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for WebAccountProviderCommandInvokedHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"delegate({b7de5527-4c8f-42dd-84da-5ec493abdb9a})");
}
unsafe impl ::windows::runtime::Interface for WebAccountProviderCommandInvokedHandler {
    type Vtable = WebAccountProviderCommandInvokedHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3084801319, 19599, 17117, [132, 218, 94, 196, 147, 171, 219, 154]);
}
#[repr(C)]
#[doc(hidden)]
pub struct WebAccountProviderCommandInvokedHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, command: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
struct WebAccountProviderCommandInvokedHandler_box<F: FnMut(&::std::option::Option<WebAccountProviderCommand>) -> ::windows::runtime::Result<()> + 'static> {
    vtable: *const WebAccountProviderCommandInvokedHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<F: FnMut(&::std::option::Option<WebAccountProviderCommand>) -> ::windows::runtime::Result<()> + 'static> WebAccountProviderCommandInvokedHandler_box<F> {
    const VTABLE: WebAccountProviderCommandInvokedHandler_abi = WebAccountProviderCommandInvokedHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<WebAccountProviderCommandInvokedHandler as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::std::ptr::null_mut()
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
            Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::runtime::RawPtr, command: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ((*this).invoke)(&*(&command as *const <WebAccountProviderCommand as ::windows::runtime::Abi>::Abi as *const <WebAccountProviderCommand as ::windows::runtime::Abi>::DefaultType)).into()
    }
}
