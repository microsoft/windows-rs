#[cfg(feature = "implement_exclusive")]
pub trait IAccountsSettingsPaneImpl: Sized {
    fn AccountCommandsRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AccountsSettingsPane, AccountsSettingsPaneCommandsRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAccountCommandsRequested(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAccountsSettingsPaneCommandsRequestedEventArgsImpl: Sized {
    fn WebAccountProviderCommands(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<WebAccountProviderCommand>>;
    fn WebAccountCommands(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<WebAccountCommand>>;
    fn CredentialCommands(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<CredentialCommand>>;
    fn Commands(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SettingsCommand>>;
    fn HeaderText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetHeaderText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn GetDeferral(&self) -> ::windows::core::Result<AccountsSettingsPaneEventDeferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAccountsSettingsPaneCommandsRequestedEventArgs2Impl: Sized {
    fn User(&self) -> ::windows::core::Result<super::super::System::User>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAccountsSettingsPaneEventDeferralImpl: Sized {
    fn Complete(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAccountsSettingsPaneStaticsImpl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<AccountsSettingsPane>;
    fn Show(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAccountsSettingsPaneStatics2Impl: Sized + IAccountsSettingsPaneStaticsImpl {
    fn ShowManageAccountsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ShowAddAccountAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAccountsSettingsPaneStatics3Impl: Sized {
    fn ShowManageAccountsForUserAsync(&self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ShowAddAccountForUserAsync(&self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICredentialCommandImpl: Sized {
    fn PasswordCredential(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential>;
    fn CredentialDeleted(&self) -> ::windows::core::Result<CredentialCommandCredentialDeletedHandler>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICredentialCommandFactoryImpl: Sized {
    fn CreateCredentialCommand(&self, passwordcredential: &::core::option::Option<super::super::Security::Credentials::PasswordCredential>) -> ::windows::core::Result<CredentialCommand>;
    fn CreateCredentialCommandWithHandler(&self, passwordcredential: &::core::option::Option<super::super::Security::Credentials::PasswordCredential>, deleted: &::core::option::Option<CredentialCommandCredentialDeletedHandler>) -> ::windows::core::Result<CredentialCommand>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISettingsCommandFactoryImpl: Sized {
    fn CreateSettingsCommand(&self, settingscommandid: &::core::option::Option<::windows::core::IInspectable>, label: &::windows::core::HSTRING, handler: &::core::option::Option<super::Popups::UICommandInvokedHandler>) -> ::windows::core::Result<SettingsCommand>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISettingsCommandStaticsImpl: Sized {
    fn AccountsCommand(&self) -> ::windows::core::Result<SettingsCommand>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISettingsPaneImpl: Sized {
    fn CommandsRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<SettingsPane, SettingsPaneCommandsRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCommandsRequested(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISettingsPaneCommandsRequestImpl: Sized {
    fn ApplicationCommands(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SettingsCommand>>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISettingsPaneCommandsRequestedEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<SettingsPaneCommandsRequest>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISettingsPaneStaticsImpl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<SettingsPane>;
    fn Show(&self) -> ::windows::core::Result<()>;
    fn Edge(&self) -> ::windows::core::Result<SettingsEdgeLocation>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAccountCommandImpl: Sized {
    fn WebAccount(&self) -> ::windows::core::Result<super::super::Security::Credentials::WebAccount>;
    fn Invoked(&self) -> ::windows::core::Result<WebAccountCommandInvokedHandler>;
    fn Actions(&self) -> ::windows::core::Result<SupportedWebAccountActions>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAccountCommandFactoryImpl: Sized {
    fn CreateWebAccountCommand(&self, webaccount: &::core::option::Option<super::super::Security::Credentials::WebAccount>, invoked: &::core::option::Option<WebAccountCommandInvokedHandler>, actions: SupportedWebAccountActions) -> ::windows::core::Result<WebAccountCommand>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAccountInvokedArgsImpl: Sized {
    fn Action(&self) -> ::windows::core::Result<WebAccountAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAccountProviderCommandImpl: Sized {
    fn WebAccountProvider(&self) -> ::windows::core::Result<super::super::Security::Credentials::WebAccountProvider>;
    fn Invoked(&self) -> ::windows::core::Result<WebAccountProviderCommandInvokedHandler>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAccountProviderCommandFactoryImpl: Sized {
    fn CreateWebAccountProviderCommand(&self, webaccountprovider: &::core::option::Option<super::super::Security::Credentials::WebAccountProvider>, invoked: &::core::option::Option<WebAccountProviderCommandInvokedHandler>) -> ::windows::core::Result<WebAccountProviderCommand>;
}
