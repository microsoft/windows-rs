#[doc(hidden)]
#[repr(transparent)]
pub struct IAccountsSettingsPane(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAccountsSettingsPane {
    type Vtable = IAccountsSettingsPane_Vtbl;
}
impl ::core::clone::Clone for IAccountsSettingsPane {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAccountsSettingsPane {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x81ea942c_4f09_4406_a538_838d9b14b7e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccountsSettingsPane_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub AccountCommandsRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
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
}
impl ::core::clone::Clone for IAccountsSettingsPaneCommandsRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAccountsSettingsPaneCommandsRequestedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3b68c099_db19_45d0_9abf_95d3773c9330);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccountsSettingsPaneCommandsRequestedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub WebAccountProviderCommands: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    WebAccountProviderCommands: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub WebAccountCommands: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    WebAccountCommands: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CredentialCommands: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CredentialCommands: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Popups"))]
    pub Commands: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Popups")))]
    Commands: usize,
    pub HeaderText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetHeaderText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAccountsSettingsPaneCommandsRequestedEventArgs2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAccountsSettingsPaneCommandsRequestedEventArgs2 {
    type Vtable = IAccountsSettingsPaneCommandsRequestedEventArgs2_Vtbl;
}
impl ::core::clone::Clone for IAccountsSettingsPaneCommandsRequestedEventArgs2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAccountsSettingsPaneCommandsRequestedEventArgs2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x362f7bad_4e37_4967_8c40_e78ee7a1e5bb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccountsSettingsPaneCommandsRequestedEventArgs2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAccountsSettingsPaneEventDeferral(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAccountsSettingsPaneEventDeferral {
    type Vtable = IAccountsSettingsPaneEventDeferral_Vtbl;
}
impl ::core::clone::Clone for IAccountsSettingsPaneEventDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAccountsSettingsPaneEventDeferral {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcbf25d3f_e5ba_40ef_93da_65e096e5fb04);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccountsSettingsPaneEventDeferral_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAccountsSettingsPaneStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAccountsSettingsPaneStatics {
    type Vtable = IAccountsSettingsPaneStatics_Vtbl;
}
impl ::core::clone::Clone for IAccountsSettingsPaneStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAccountsSettingsPaneStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x561f8b60_b0ec_4150_a8dc_208ee44b068a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccountsSettingsPaneStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Show: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAccountsSettingsPaneStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAccountsSettingsPaneStatics2 {
    type Vtable = IAccountsSettingsPaneStatics2_Vtbl;
}
impl ::core::clone::Clone for IAccountsSettingsPaneStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAccountsSettingsPaneStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd21df7c2_ce0d_484f_b8e8_e823c215765e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccountsSettingsPaneStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub ShowManageAccountsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowManageAccountsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowAddAccountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowAddAccountAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAccountsSettingsPaneStatics3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAccountsSettingsPaneStatics3 {
    type Vtable = IAccountsSettingsPaneStatics3_Vtbl;
}
impl ::core::clone::Clone for IAccountsSettingsPaneStatics3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAccountsSettingsPaneStatics3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x08410458_a2ba_4c6f_b4ac_48f514331216);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccountsSettingsPaneStatics3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub ShowManageAccountsForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    ShowManageAccountsForUserAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub ShowAddAccountForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    ShowAddAccountForUserAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICredentialCommand(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICredentialCommand {
    type Vtable = ICredentialCommand_Vtbl;
}
impl ::core::clone::Clone for ICredentialCommand {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICredentialCommand {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa5f665e6_6143_4a7a_a971_b017ba978ce2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICredentialCommand_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Security_Credentials")]
    pub PasswordCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    PasswordCredential: usize,
    pub CredentialDeleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICredentialCommandFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICredentialCommandFactory {
    type Vtable = ICredentialCommandFactory_Vtbl;
}
impl ::core::clone::Clone for ICredentialCommandFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICredentialCommandFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27e88c17_bc3e_4b80_9495_4ed720e48a91);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICredentialCommandFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Security_Credentials")]
    pub CreateCredentialCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, passwordcredential: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    CreateCredentialCommand: usize,
    #[cfg(feature = "Security_Credentials")]
    pub CreateCredentialCommandWithHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, passwordcredential: *mut ::core::ffi::c_void, deleted: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    CreateCredentialCommandWithHandler: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISettingsCommandFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISettingsCommandFactory {
    type Vtable = ISettingsCommandFactory_Vtbl;
}
impl ::core::clone::Clone for ISettingsCommandFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISettingsCommandFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x68e15b33_1c83_433a_aa5a_ceeea5bd4764);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISettingsCommandFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "UI_Popups")]
    pub CreateSettingsCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settingscommandid: *mut ::core::ffi::c_void, label: ::std::mem::MaybeUninit<::windows::core::HSTRING>, handler: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Popups"))]
    CreateSettingsCommand: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISettingsCommandStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISettingsCommandStatics {
    type Vtable = ISettingsCommandStatics_Vtbl;
}
impl ::core::clone::Clone for ISettingsCommandStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISettingsCommandStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x749ae954_2f69_4b17_8aba_d05ce5778e46);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISettingsCommandStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "UI_Popups")]
    pub AccountsCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
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
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for ISettingsPane {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::ComInterface for ISettingsPane {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb1cd0932_4570_4c69_8d38_89446561ace0);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ISettingsPane_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub CommandsRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
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
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for ISettingsPaneCommandsRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::ComInterface for ISettingsPaneCommandsRequest {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x44df23ae_5d6e_4068_a168_f47643182114);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ISettingsPaneCommandsRequest_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Popups", feature = "deprecated"))]
    pub ApplicationCommands: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
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
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for ISettingsPaneCommandsRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::ComInterface for ISettingsPaneCommandsRequestedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x205f5d24_1b48_4629_a6ca_2fdfedafb75d);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ISettingsPaneCommandsRequestedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
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
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for ISettingsPaneStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::ComInterface for ISettingsPaneStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1c6a52c5_ff19_471b_ba6b_f8f35694ad9a);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ISettingsPaneStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
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
}
impl ::core::clone::Clone for IWebAccountCommand {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWebAccountCommand {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcaa39398_9cfa_4246_b0c4_a913a3896541);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountCommand_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Security_Credentials")]
    pub WebAccount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    WebAccount: usize,
    pub Invoked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Actions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SupportedWebAccountActions) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAccountCommandFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebAccountCommandFactory {
    type Vtable = IWebAccountCommandFactory_Vtbl;
}
impl ::core::clone::Clone for IWebAccountCommandFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWebAccountCommandFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbfa6cdff_2f2d_42f5_81de_1d56bafc496d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountCommandFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Security_Credentials")]
    pub CreateWebAccountCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, webaccount: *mut ::core::ffi::c_void, invoked: *mut ::core::ffi::c_void, actions: SupportedWebAccountActions, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    CreateWebAccountCommand: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAccountInvokedArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebAccountInvokedArgs {
    type Vtable = IWebAccountInvokedArgs_Vtbl;
}
impl ::core::clone::Clone for IWebAccountInvokedArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWebAccountInvokedArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe7abcc40_a1d8_4c5d_9a7f_1d34b2f90ad2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountInvokedArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Action: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WebAccountAction) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAccountProviderCommand(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebAccountProviderCommand {
    type Vtable = IWebAccountProviderCommand_Vtbl;
}
impl ::core::clone::Clone for IWebAccountProviderCommand {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWebAccountProviderCommand {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd69bdd9a_a0a6_4e9b_88dc_c71e757a3501);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountProviderCommand_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Security_Credentials")]
    pub WebAccountProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    WebAccountProvider: usize,
    pub Invoked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAccountProviderCommandFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebAccountProviderCommandFactory {
    type Vtable = IWebAccountProviderCommandFactory_Vtbl;
}
impl ::core::clone::Clone for IWebAccountProviderCommandFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWebAccountProviderCommandFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd5658a1b_b176_4776_8469_a9d3ff0b3f59);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountProviderCommandFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Security_Credentials")]
    pub CreateWebAccountProviderCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, webaccountprovider: *mut ::core::ffi::c_void, invoked: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    CreateWebAccountProviderCommand: usize,
}
#[doc = "*Required features: `\"UI_ApplicationSettings\"`*"]
#[repr(transparent)]
pub struct AccountsSettingsPane(::windows::core::IUnknown);
impl AccountsSettingsPane {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AccountCommandsRequested(&self, handler: &super::super::Foundation::TypedEventHandler<AccountsSettingsPane, AccountsSettingsPaneCommandsRequestedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).AccountCommandsRequested)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAccountCommandsRequested(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAccountCommandsRequested)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn GetForCurrentView() -> ::windows::core::Result<AccountsSettingsPane> {
        Self::IAccountsSettingsPaneStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<AccountsSettingsPane>();
            (::windows::core::Interface::vtable(this).GetForCurrentView)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Show() -> ::windows::core::Result<()> {
        Self::IAccountsSettingsPaneStatics(|this| unsafe { (::windows::core::Interface::vtable(this).Show)(::windows::core::Interface::as_raw(this)).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ShowManageAccountsAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        Self::IAccountsSettingsPaneStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).ShowManageAccountsAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ShowAddAccountAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        Self::IAccountsSettingsPaneStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).ShowAddAccountAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn ShowManageAccountsForUserAsync(user: &super::super::System::User) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        Self::IAccountsSettingsPaneStatics3(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).ShowManageAccountsForUserAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(user), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn ShowAddAccountForUserAsync(user: &super::super::System::User) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        Self::IAccountsSettingsPaneStatics3(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).ShowAddAccountForUserAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(user), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAccountsSettingsPaneStatics<R, F: FnOnce(&IAccountsSettingsPaneStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<AccountsSettingsPane, IAccountsSettingsPaneStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IAccountsSettingsPaneStatics2<R, F: FnOnce(&IAccountsSettingsPaneStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<AccountsSettingsPane, IAccountsSettingsPaneStatics2> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IAccountsSettingsPaneStatics3<R, F: FnOnce(&IAccountsSettingsPaneStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<AccountsSettingsPane, IAccountsSettingsPaneStatics3> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows::core::RuntimeType for AccountsSettingsPane {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.ApplicationSettings.AccountsSettingsPane;{81ea942c-4f09-4406-a538-838d9b14b7e6})");
}
impl ::core::clone::Clone for AccountsSettingsPane {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AccountsSettingsPane {
    type Vtable = IAccountsSettingsPane_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AccountsSettingsPane {
    const IID: ::windows::core::GUID = <IAccountsSettingsPane as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AccountsSettingsPane {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.AccountsSettingsPane";
}
::windows::imp::interface_hierarchy!(AccountsSettingsPane, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"UI_ApplicationSettings\"`*"]
#[repr(transparent)]
pub struct AccountsSettingsPaneCommandsRequestedEventArgs(::windows::core::IUnknown);
impl AccountsSettingsPaneCommandsRequestedEventArgs {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn WebAccountProviderCommands(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<WebAccountProviderCommand>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<WebAccountProviderCommand>>();
            (::windows::core::Interface::vtable(this).WebAccountProviderCommands)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn WebAccountCommands(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<WebAccountCommand>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<WebAccountCommand>>();
            (::windows::core::Interface::vtable(this).WebAccountCommands)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CredentialCommands(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<CredentialCommand>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<CredentialCommand>>();
            (::windows::core::Interface::vtable(this).CredentialCommands)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"UI_Popups\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Popups"))]
    pub fn Commands(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SettingsCommand>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<SettingsCommand>>();
            (::windows::core::Interface::vtable(this).Commands)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HeaderText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).HeaderText)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetHeaderText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHeaderText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn GetDeferral(&self) -> ::windows::core::Result<AccountsSettingsPaneEventDeferral> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AccountsSettingsPaneEventDeferral>();
            (::windows::core::Interface::vtable(this).GetDeferral)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::ComInterface::cast::<IAccountsSettingsPaneCommandsRequestedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::System::User>();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows::core::RuntimeType for AccountsSettingsPaneCommandsRequestedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.ApplicationSettings.AccountsSettingsPaneCommandsRequestedEventArgs;{3b68c099-db19-45d0-9abf-95d3773c9330})");
}
impl ::core::clone::Clone for AccountsSettingsPaneCommandsRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AccountsSettingsPaneCommandsRequestedEventArgs {
    type Vtable = IAccountsSettingsPaneCommandsRequestedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AccountsSettingsPaneCommandsRequestedEventArgs {
    const IID: ::windows::core::GUID = <IAccountsSettingsPaneCommandsRequestedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AccountsSettingsPaneCommandsRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.AccountsSettingsPaneCommandsRequestedEventArgs";
}
::windows::imp::interface_hierarchy!(AccountsSettingsPaneCommandsRequestedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"UI_ApplicationSettings\"`*"]
#[repr(transparent)]
pub struct AccountsSettingsPaneEventDeferral(::windows::core::IUnknown);
impl AccountsSettingsPaneEventDeferral {
    pub fn Complete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Complete)(::windows::core::Interface::as_raw(this)).ok() }
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
impl ::windows::core::RuntimeType for AccountsSettingsPaneEventDeferral {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.ApplicationSettings.AccountsSettingsPaneEventDeferral;{cbf25d3f-e5ba-40ef-93da-65e096e5fb04})");
}
impl ::core::clone::Clone for AccountsSettingsPaneEventDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AccountsSettingsPaneEventDeferral {
    type Vtable = IAccountsSettingsPaneEventDeferral_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AccountsSettingsPaneEventDeferral {
    const IID: ::windows::core::GUID = <IAccountsSettingsPaneEventDeferral as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AccountsSettingsPaneEventDeferral {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.AccountsSettingsPaneEventDeferral";
}
::windows::imp::interface_hierarchy!(AccountsSettingsPaneEventDeferral, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"UI_ApplicationSettings\"`*"]
#[repr(transparent)]
pub struct CredentialCommand(::windows::core::IUnknown);
impl CredentialCommand {
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn PasswordCredential(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Security::Credentials::PasswordCredential>();
            (::windows::core::Interface::vtable(this).PasswordCredential)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CredentialDeleted(&self) -> ::windows::core::Result<CredentialCommandCredentialDeletedHandler> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<CredentialCommandCredentialDeletedHandler>();
            (::windows::core::Interface::vtable(this).CredentialDeleted)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn CreateCredentialCommand(passwordcredential: &super::super::Security::Credentials::PasswordCredential) -> ::windows::core::Result<CredentialCommand> {
        Self::ICredentialCommandFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<CredentialCommand>();
            (::windows::core::Interface::vtable(this).CreateCredentialCommand)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(passwordcredential), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn CreateCredentialCommandWithHandler(passwordcredential: &super::super::Security::Credentials::PasswordCredential, deleted: &CredentialCommandCredentialDeletedHandler) -> ::windows::core::Result<CredentialCommand> {
        Self::ICredentialCommandFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<CredentialCommand>();
            (::windows::core::Interface::vtable(this).CreateCredentialCommandWithHandler)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(passwordcredential), ::core::mem::transmute_copy(deleted), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICredentialCommandFactory<R, F: FnOnce(&ICredentialCommandFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<CredentialCommand, ICredentialCommandFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows::core::RuntimeType for CredentialCommand {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.ApplicationSettings.CredentialCommand;{a5f665e6-6143-4a7a-a971-b017ba978ce2})");
}
impl ::core::clone::Clone for CredentialCommand {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for CredentialCommand {
    type Vtable = ICredentialCommand_Vtbl;
}
unsafe impl ::windows::core::ComInterface for CredentialCommand {
    const IID: ::windows::core::GUID = <ICredentialCommand as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for CredentialCommand {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.CredentialCommand";
}
::windows::imp::interface_hierarchy!(CredentialCommand, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"UI_ApplicationSettings\"`, `\"UI_Popups\"`*"]
#[cfg(feature = "UI_Popups")]
#[repr(transparent)]
pub struct SettingsCommand(::windows::core::IUnknown);
#[cfg(feature = "UI_Popups")]
impl SettingsCommand {
    #[doc = "*Required features: `\"UI_Popups\"`*"]
    #[cfg(feature = "UI_Popups")]
    pub fn CreateSettingsCommand<P0>(settingscommandid: P0, label: &::windows::core::HSTRING, handler: &super::Popups::UICommandInvokedHandler) -> ::windows::core::Result<SettingsCommand>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        Self::ISettingsCommandFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<SettingsCommand>();
            (::windows::core::Interface::vtable(this).CreateSettingsCommand)(::windows::core::Interface::as_raw(this), settingscommandid.into_param().abi(), ::core::mem::transmute_copy(label), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Popups\"`*"]
    #[cfg(feature = "UI_Popups")]
    pub fn AccountsCommand() -> ::windows::core::Result<SettingsCommand> {
        Self::ISettingsCommandStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<SettingsCommand>();
            (::windows::core::Interface::vtable(this).AccountsCommand)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Popups\"`*"]
    #[cfg(feature = "UI_Popups")]
    pub fn Label(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Label)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Popups\"`*"]
    #[cfg(feature = "UI_Popups")]
    pub fn SetLabel(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetLabel)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"UI_Popups\"`*"]
    #[cfg(feature = "UI_Popups")]
    pub fn Invoked(&self) -> ::windows::core::Result<super::Popups::UICommandInvokedHandler> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::Popups::UICommandInvokedHandler>();
            (::windows::core::Interface::vtable(this).Invoked)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Popups\"`*"]
    #[cfg(feature = "UI_Popups")]
    pub fn SetInvoked(&self, value: &super::Popups::UICommandInvokedHandler) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetInvoked)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"UI_Popups\"`*"]
    #[cfg(feature = "UI_Popups")]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).Id)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Popups\"`*"]
    #[cfg(feature = "UI_Popups")]
    pub fn SetId<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetId)(::windows::core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc(hidden)]
    pub fn ISettingsCommandFactory<R, F: FnOnce(&ISettingsCommandFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<SettingsCommand, ISettingsCommandFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ISettingsCommandStatics<R, F: FnOnce(&ISettingsCommandStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<SettingsCommand, ISettingsCommandStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows::core::RuntimeType for SettingsCommand {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.ApplicationSettings.SettingsCommand;{4ff93a75-4145-47ff-ac7f-dff1c1fa5b0f})");
}
#[cfg(feature = "UI_Popups")]
impl ::core::clone::Clone for SettingsCommand {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "UI_Popups")]
unsafe impl ::windows::core::Interface for SettingsCommand {
    type Vtable = super::Popups::IUICommand_Vtbl;
}
#[cfg(feature = "UI_Popups")]
unsafe impl ::windows::core::ComInterface for SettingsCommand {
    const IID: ::windows::core::GUID = <super::Popups::IUICommand as ::windows::core::ComInterface>::IID;
}
#[cfg(feature = "UI_Popups")]
impl ::windows::core::RuntimeName for SettingsCommand {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.SettingsCommand";
}
#[cfg(feature = "UI_Popups")]
::windows::imp::interface_hierarchy!(SettingsCommand, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "UI_Popups")]
impl ::windows::core::CanTryInto<super::Popups::IUICommand> for SettingsCommand {}
#[doc = "*Required features: `\"UI_ApplicationSettings\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct SettingsPane(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl SettingsPane {
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn CommandsRequested(&self, handler: &super::super::Foundation::TypedEventHandler<SettingsPane, SettingsPaneCommandsRequestedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).CommandsRequested)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveCommandsRequested(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveCommandsRequested)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn GetForCurrentView() -> ::windows::core::Result<SettingsPane> {
        Self::ISettingsPaneStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<SettingsPane>();
            (::windows::core::Interface::vtable(this).GetForCurrentView)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Show() -> ::windows::core::Result<()> {
        Self::ISettingsPaneStatics(|this| unsafe { (::windows::core::Interface::vtable(this).Show)(::windows::core::Interface::as_raw(this)).ok() })
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Edge() -> ::windows::core::Result<SettingsEdgeLocation> {
        Self::ISettingsPaneStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<SettingsEdgeLocation>();
            (::windows::core::Interface::vtable(this).Edge)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn ISettingsPaneStatics<R, F: FnOnce(&ISettingsPaneStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<SettingsPane, ISettingsPaneStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows::core::RuntimeType for SettingsPane {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.ApplicationSettings.SettingsPane;{b1cd0932-4570-4c69-8d38-89446561ace0})");
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SettingsPane {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for SettingsPane {
    type Vtable = ISettingsPane_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::ComInterface for SettingsPane {
    const IID: ::windows::core::GUID = <ISettingsPane as ::windows::core::ComInterface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for SettingsPane {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.SettingsPane";
}
#[cfg(feature = "deprecated")]
::windows::imp::interface_hierarchy!(SettingsPane, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"UI_ApplicationSettings\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct SettingsPaneCommandsRequest(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl SettingsPaneCommandsRequest {
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"UI_Popups\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Popups", feature = "deprecated"))]
    pub fn ApplicationCommands(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SettingsCommand>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<SettingsCommand>>();
            (::windows::core::Interface::vtable(this).ApplicationCommands)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows::core::RuntimeType for SettingsPaneCommandsRequest {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.ApplicationSettings.SettingsPaneCommandsRequest;{44df23ae-5d6e-4068-a168-f47643182114})");
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SettingsPaneCommandsRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for SettingsPaneCommandsRequest {
    type Vtable = ISettingsPaneCommandsRequest_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::ComInterface for SettingsPaneCommandsRequest {
    const IID: ::windows::core::GUID = <ISettingsPaneCommandsRequest as ::windows::core::ComInterface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for SettingsPaneCommandsRequest {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.SettingsPaneCommandsRequest";
}
#[cfg(feature = "deprecated")]
::windows::imp::interface_hierarchy!(SettingsPaneCommandsRequest, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"UI_ApplicationSettings\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct SettingsPaneCommandsRequestedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl SettingsPaneCommandsRequestedEventArgs {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Request(&self) -> ::windows::core::Result<SettingsPaneCommandsRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SettingsPaneCommandsRequest>();
            (::windows::core::Interface::vtable(this).Request)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows::core::RuntimeType for SettingsPaneCommandsRequestedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.ApplicationSettings.SettingsPaneCommandsRequestedEventArgs;{205f5d24-1b48-4629-a6ca-2fdfedafb75d})");
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SettingsPaneCommandsRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for SettingsPaneCommandsRequestedEventArgs {
    type Vtable = ISettingsPaneCommandsRequestedEventArgs_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::ComInterface for SettingsPaneCommandsRequestedEventArgs {
    const IID: ::windows::core::GUID = <ISettingsPaneCommandsRequestedEventArgs as ::windows::core::ComInterface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for SettingsPaneCommandsRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.SettingsPaneCommandsRequestedEventArgs";
}
#[cfg(feature = "deprecated")]
::windows::imp::interface_hierarchy!(SettingsPaneCommandsRequestedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"UI_ApplicationSettings\"`*"]
#[repr(transparent)]
pub struct WebAccountCommand(::windows::core::IUnknown);
impl WebAccountCommand {
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn WebAccount(&self) -> ::windows::core::Result<super::super::Security::Credentials::WebAccount> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Security::Credentials::WebAccount>();
            (::windows::core::Interface::vtable(this).WebAccount)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Invoked(&self) -> ::windows::core::Result<WebAccountCommandInvokedHandler> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<WebAccountCommandInvokedHandler>();
            (::windows::core::Interface::vtable(this).Invoked)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Actions(&self) -> ::windows::core::Result<SupportedWebAccountActions> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SupportedWebAccountActions>();
            (::windows::core::Interface::vtable(this).Actions)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn CreateWebAccountCommand(webaccount: &super::super::Security::Credentials::WebAccount, invoked: &WebAccountCommandInvokedHandler, actions: SupportedWebAccountActions) -> ::windows::core::Result<WebAccountCommand> {
        Self::IWebAccountCommandFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<WebAccountCommand>();
            (::windows::core::Interface::vtable(this).CreateWebAccountCommand)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(webaccount), ::core::mem::transmute_copy(invoked), actions, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWebAccountCommandFactory<R, F: FnOnce(&IWebAccountCommandFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<WebAccountCommand, IWebAccountCommandFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows::core::RuntimeType for WebAccountCommand {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.ApplicationSettings.WebAccountCommand;{caa39398-9cfa-4246-b0c4-a913a3896541})");
}
impl ::core::clone::Clone for WebAccountCommand {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for WebAccountCommand {
    type Vtable = IWebAccountCommand_Vtbl;
}
unsafe impl ::windows::core::ComInterface for WebAccountCommand {
    const IID: ::windows::core::GUID = <IWebAccountCommand as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for WebAccountCommand {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.WebAccountCommand";
}
::windows::imp::interface_hierarchy!(WebAccountCommand, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"UI_ApplicationSettings\"`*"]
#[repr(transparent)]
pub struct WebAccountInvokedArgs(::windows::core::IUnknown);
impl WebAccountInvokedArgs {
    pub fn Action(&self) -> ::windows::core::Result<WebAccountAction> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<WebAccountAction>();
            (::windows::core::Interface::vtable(this).Action)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows::core::RuntimeType for WebAccountInvokedArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.ApplicationSettings.WebAccountInvokedArgs;{e7abcc40-a1d8-4c5d-9a7f-1d34b2f90ad2})");
}
impl ::core::clone::Clone for WebAccountInvokedArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for WebAccountInvokedArgs {
    type Vtable = IWebAccountInvokedArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for WebAccountInvokedArgs {
    const IID: ::windows::core::GUID = <IWebAccountInvokedArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for WebAccountInvokedArgs {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.WebAccountInvokedArgs";
}
::windows::imp::interface_hierarchy!(WebAccountInvokedArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"UI_ApplicationSettings\"`*"]
#[repr(transparent)]
pub struct WebAccountProviderCommand(::windows::core::IUnknown);
impl WebAccountProviderCommand {
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn WebAccountProvider(&self) -> ::windows::core::Result<super::super::Security::Credentials::WebAccountProvider> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Security::Credentials::WebAccountProvider>();
            (::windows::core::Interface::vtable(this).WebAccountProvider)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Invoked(&self) -> ::windows::core::Result<WebAccountProviderCommandInvokedHandler> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<WebAccountProviderCommandInvokedHandler>();
            (::windows::core::Interface::vtable(this).Invoked)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn CreateWebAccountProviderCommand(webaccountprovider: &super::super::Security::Credentials::WebAccountProvider, invoked: &WebAccountProviderCommandInvokedHandler) -> ::windows::core::Result<WebAccountProviderCommand> {
        Self::IWebAccountProviderCommandFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<WebAccountProviderCommand>();
            (::windows::core::Interface::vtable(this).CreateWebAccountProviderCommand)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(webaccountprovider), ::core::mem::transmute_copy(invoked), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWebAccountProviderCommandFactory<R, F: FnOnce(&IWebAccountProviderCommandFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<WebAccountProviderCommand, IWebAccountProviderCommandFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows::core::RuntimeType for WebAccountProviderCommand {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.ApplicationSettings.WebAccountProviderCommand;{d69bdd9a-a0a6-4e9b-88dc-c71e757a3501})");
}
impl ::core::clone::Clone for WebAccountProviderCommand {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for WebAccountProviderCommand {
    type Vtable = IWebAccountProviderCommand_Vtbl;
}
unsafe impl ::windows::core::ComInterface for WebAccountProviderCommand {
    const IID: ::windows::core::GUID = <IWebAccountProviderCommand as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for WebAccountProviderCommand {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.WebAccountProviderCommand";
}
::windows::imp::interface_hierarchy!(WebAccountProviderCommand, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"UI_ApplicationSettings\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows::core::TypeKind for SettingsEdgeLocation {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for SettingsEdgeLocation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SettingsEdgeLocation").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeType for SettingsEdgeLocation {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.UI.ApplicationSettings.SettingsEdgeLocation;i4)");
}
#[doc = "*Required features: `\"UI_ApplicationSettings\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows::core::TypeKind for SupportedWebAccountActions {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SupportedWebAccountActions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SupportedWebAccountActions").field(&self.0).finish()
    }
}
impl SupportedWebAccountActions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
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
impl ::windows::core::RuntimeType for SupportedWebAccountActions {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.UI.ApplicationSettings.SupportedWebAccountActions;u4)");
}
#[doc = "*Required features: `\"UI_ApplicationSettings\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows::core::TypeKind for WebAccountAction {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WebAccountAction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebAccountAction").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for WebAccountAction {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.UI.ApplicationSettings.WebAccountAction;i4)");
}
#[doc = "*Required features: `\"UI_ApplicationSettings\"`*"]
#[repr(transparent)]
pub struct CredentialCommandCredentialDeletedHandler(pub ::windows::core::IUnknown);
impl CredentialCommandCredentialDeletedHandler {
    pub fn new<F: FnMut(::core::option::Option<&CredentialCommand>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = CredentialCommandCredentialDeletedHandlerBox::<F> { vtable: &CredentialCommandCredentialDeletedHandlerBox::<F>::VTABLE, count: ::windows::imp::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke(&self, command: &CredentialCommand) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Invoke)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(command)).ok() }
    }
}
#[repr(C)]
struct CredentialCommandCredentialDeletedHandlerBox<F: FnMut(::core::option::Option<&CredentialCommand>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const CredentialCommandCredentialDeletedHandler_Vtbl,
    invoke: F,
    count: ::windows::imp::RefCount,
}
impl<F: FnMut(::core::option::Option<&CredentialCommand>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> CredentialCommandCredentialDeletedHandlerBox<F> {
    const VTABLE: CredentialCommandCredentialDeletedHandler_Vtbl = CredentialCommandCredentialDeletedHandler_Vtbl {
        base__: ::windows::core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<CredentialCommandCredentialDeletedHandler as ::windows::core::ComInterface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::ComInterface>::IID || iid == &<::windows::imp::IAgileObject as ::windows::core::ComInterface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = ::std::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, command: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::windows::core::from_raw_borrowed(&command)).into()
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
}
impl ::core::clone::Clone for CredentialCommandCredentialDeletedHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for CredentialCommandCredentialDeletedHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x61c0e185_0977_4678_b4e2_98727afbeed9);
}
impl ::windows::core::RuntimeType for CredentialCommandCredentialDeletedHandler {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{61c0e185-0977-4678-b4e2-98727afbeed9}");
}
#[repr(C)]
#[doc(hidden)]
pub struct CredentialCommandCredentialDeletedHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, command: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_ApplicationSettings\"`*"]
#[repr(transparent)]
pub struct WebAccountCommandInvokedHandler(pub ::windows::core::IUnknown);
impl WebAccountCommandInvokedHandler {
    pub fn new<F: FnMut(::core::option::Option<&WebAccountCommand>, ::core::option::Option<&WebAccountInvokedArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = WebAccountCommandInvokedHandlerBox::<F> { vtable: &WebAccountCommandInvokedHandlerBox::<F>::VTABLE, count: ::windows::imp::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke(&self, command: &WebAccountCommand, args: &WebAccountInvokedArgs) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Invoke)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(command), ::core::mem::transmute_copy(args)).ok() }
    }
}
#[repr(C)]
struct WebAccountCommandInvokedHandlerBox<F: FnMut(::core::option::Option<&WebAccountCommand>, ::core::option::Option<&WebAccountInvokedArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const WebAccountCommandInvokedHandler_Vtbl,
    invoke: F,
    count: ::windows::imp::RefCount,
}
impl<F: FnMut(::core::option::Option<&WebAccountCommand>, ::core::option::Option<&WebAccountInvokedArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> WebAccountCommandInvokedHandlerBox<F> {
    const VTABLE: WebAccountCommandInvokedHandler_Vtbl = WebAccountCommandInvokedHandler_Vtbl {
        base__: ::windows::core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<WebAccountCommandInvokedHandler as ::windows::core::ComInterface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::ComInterface>::IID || iid == &<::windows::imp::IAgileObject as ::windows::core::ComInterface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = ::std::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, command: *mut ::core::ffi::c_void, args: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::windows::core::from_raw_borrowed(&command), ::windows::core::from_raw_borrowed(&args)).into()
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
}
impl ::core::clone::Clone for WebAccountCommandInvokedHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for WebAccountCommandInvokedHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1ee6e459_1705_4a9a_b599_a0c3d6921973);
}
impl ::windows::core::RuntimeType for WebAccountCommandInvokedHandler {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{1ee6e459-1705-4a9a-b599-a0c3d6921973}");
}
#[repr(C)]
#[doc(hidden)]
pub struct WebAccountCommandInvokedHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, command: *mut ::core::ffi::c_void, args: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_ApplicationSettings\"`*"]
#[repr(transparent)]
pub struct WebAccountProviderCommandInvokedHandler(pub ::windows::core::IUnknown);
impl WebAccountProviderCommandInvokedHandler {
    pub fn new<F: FnMut(::core::option::Option<&WebAccountProviderCommand>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = WebAccountProviderCommandInvokedHandlerBox::<F> { vtable: &WebAccountProviderCommandInvokedHandlerBox::<F>::VTABLE, count: ::windows::imp::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke(&self, command: &WebAccountProviderCommand) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Invoke)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(command)).ok() }
    }
}
#[repr(C)]
struct WebAccountProviderCommandInvokedHandlerBox<F: FnMut(::core::option::Option<&WebAccountProviderCommand>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const WebAccountProviderCommandInvokedHandler_Vtbl,
    invoke: F,
    count: ::windows::imp::RefCount,
}
impl<F: FnMut(::core::option::Option<&WebAccountProviderCommand>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> WebAccountProviderCommandInvokedHandlerBox<F> {
    const VTABLE: WebAccountProviderCommandInvokedHandler_Vtbl = WebAccountProviderCommandInvokedHandler_Vtbl {
        base__: ::windows::core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<WebAccountProviderCommandInvokedHandler as ::windows::core::ComInterface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::ComInterface>::IID || iid == &<::windows::imp::IAgileObject as ::windows::core::ComInterface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = ::std::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, command: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::windows::core::from_raw_borrowed(&command)).into()
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
}
impl ::core::clone::Clone for WebAccountProviderCommandInvokedHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for WebAccountProviderCommandInvokedHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb7de5527_4c8f_42dd_84da_5ec493abdb9a);
}
impl ::windows::core::RuntimeType for WebAccountProviderCommandInvokedHandler {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{b7de5527-4c8f-42dd-84da-5ec493abdb9a}");
}
#[repr(C)]
#[doc(hidden)]
pub struct WebAccountProviderCommandInvokedHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, command: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
