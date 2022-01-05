#[cfg(feature = "implement_exclusive")]
pub trait IDeviceAccountConfigurationImpl: Sized {
    fn AccountName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAccountName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DeviceAccountTypeId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDeviceAccountTypeId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ServerType(&self) -> ::windows::core::Result<DeviceAccountServerType>;
    fn SetServerType(&self, value: DeviceAccountServerType) -> ::windows::core::Result<()>;
    fn EmailAddress(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetEmailAddress(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Domain(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDomain(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn EmailSyncEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetEmailSyncEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn ContactsSyncEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetContactsSyncEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn CalendarSyncEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetCalendarSyncEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IncomingServerAddress(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetIncomingServerAddress(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IncomingServerPort(&self) -> ::windows::core::Result<i32>;
    fn SetIncomingServerPort(&self, value: i32) -> ::windows::core::Result<()>;
    fn IncomingServerRequiresSsl(&self) -> ::windows::core::Result<bool>;
    fn SetIncomingServerRequiresSsl(&self, value: bool) -> ::windows::core::Result<()>;
    fn IncomingServerUsername(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetIncomingServerUsername(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn OutgoingServerAddress(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetOutgoingServerAddress(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn OutgoingServerPort(&self) -> ::windows::core::Result<i32>;
    fn SetOutgoingServerPort(&self, value: i32) -> ::windows::core::Result<()>;
    fn OutgoingServerRequiresSsl(&self) -> ::windows::core::Result<bool>;
    fn SetOutgoingServerRequiresSsl(&self, value: bool) -> ::windows::core::Result<()>;
    fn OutgoingServerUsername(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetOutgoingServerUsername(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceAccountConfiguration2Impl: Sized {
    fn IncomingServerCredential(&self) -> ::windows::core::Result<super::super::super::Security::Credentials::PasswordCredential>;
    fn SetIncomingServerCredential(&self, value: &::core::option::Option<super::super::super::Security::Credentials::PasswordCredential>) -> ::windows::core::Result<()>;
    fn OutgoingServerCredential(&self) -> ::windows::core::Result<super::super::super::Security::Credentials::PasswordCredential>;
    fn SetOutgoingServerCredential(&self, value: &::core::option::Option<super::super::super::Security::Credentials::PasswordCredential>) -> ::windows::core::Result<()>;
    fn OAuthRefreshToken(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetOAuthRefreshToken(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IsExternallyManaged(&self) -> ::windows::core::Result<bool>;
    fn SetIsExternallyManaged(&self, value: bool) -> ::windows::core::Result<()>;
    fn AccountIconId(&self) -> ::windows::core::Result<DeviceAccountIconId>;
    fn SetAccountIconId(&self, value: DeviceAccountIconId) -> ::windows::core::Result<()>;
    fn AuthenticationType(&self) -> ::windows::core::Result<DeviceAccountAuthenticationType>;
    fn SetAuthenticationType(&self, value: DeviceAccountAuthenticationType) -> ::windows::core::Result<()>;
    fn IsSsoAuthenticationSupported(&self) -> ::windows::core::Result<bool>;
    fn SsoAccountId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSsoAccountId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AlwaysDownloadFullMessage(&self) -> ::windows::core::Result<bool>;
    fn SetAlwaysDownloadFullMessage(&self, value: bool) -> ::windows::core::Result<()>;
    fn DoesPolicyAllowMailSync(&self) -> ::windows::core::Result<bool>;
    fn SyncScheduleKind(&self) -> ::windows::core::Result<DeviceAccountSyncScheduleKind>;
    fn SetSyncScheduleKind(&self, value: DeviceAccountSyncScheduleKind) -> ::windows::core::Result<()>;
    fn MailAgeFilter(&self) -> ::windows::core::Result<DeviceAccountMailAgeFilter>;
    fn SetMailAgeFilter(&self, value: DeviceAccountMailAgeFilter) -> ::windows::core::Result<()>;
    fn IsClientAuthenticationCertificateRequired(&self) -> ::windows::core::Result<bool>;
    fn SetIsClientAuthenticationCertificateRequired(&self, value: bool) -> ::windows::core::Result<()>;
    fn AutoSelectAuthenticationCertificate(&self) -> ::windows::core::Result<bool>;
    fn SetAutoSelectAuthenticationCertificate(&self, value: bool) -> ::windows::core::Result<()>;
    fn AuthenticationCertificateId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAuthenticationCertificateId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn CardDavSyncScheduleKind(&self) -> ::windows::core::Result<DeviceAccountSyncScheduleKind>;
    fn SetCardDavSyncScheduleKind(&self, value: DeviceAccountSyncScheduleKind) -> ::windows::core::Result<()>;
    fn CalDavSyncScheduleKind(&self) -> ::windows::core::Result<DeviceAccountSyncScheduleKind>;
    fn SetCalDavSyncScheduleKind(&self, value: DeviceAccountSyncScheduleKind) -> ::windows::core::Result<()>;
    fn CardDavServerUrl(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn SetCardDavServerUrl(&self, value: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn CardDavRequiresSsl(&self) -> ::windows::core::Result<bool>;
    fn SetCardDavRequiresSsl(&self, value: bool) -> ::windows::core::Result<()>;
    fn CalDavServerUrl(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn SetCalDavServerUrl(&self, value: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn CalDavRequiresSsl(&self) -> ::windows::core::Result<bool>;
    fn SetCalDavRequiresSsl(&self, value: bool) -> ::windows::core::Result<()>;
    fn WasModifiedByUser(&self) -> ::windows::core::Result<bool>;
    fn SetWasModifiedByUser(&self, value: bool) -> ::windows::core::Result<()>;
    fn WasIncomingServerCertificateHashConfirmed(&self) -> ::windows::core::Result<bool>;
    fn SetWasIncomingServerCertificateHashConfirmed(&self, value: bool) -> ::windows::core::Result<()>;
    fn IncomingServerCertificateHash(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetIncomingServerCertificateHash(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IsOutgoingServerAuthenticationRequired(&self) -> ::windows::core::Result<bool>;
    fn SetIsOutgoingServerAuthenticationRequired(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsOutgoingServerAuthenticationEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsOutgoingServerAuthenticationEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn WasOutgoingServerCertificateHashConfirmed(&self) -> ::windows::core::Result<bool>;
    fn SetWasOutgoingServerCertificateHashConfirmed(&self, value: bool) -> ::windows::core::Result<()>;
    fn OutgoingServerCertificateHash(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetOutgoingServerCertificateHash(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IsSyncScheduleManagedBySystem(&self) -> ::windows::core::Result<bool>;
    fn SetIsSyncScheduleManagedBySystem(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserDataAccountSystemAccessManagerStaticsImpl: Sized {
    fn AddAndShowDeviceAccountsAsync(&self, accounts: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<DeviceAccountConfiguration>>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserDataAccountSystemAccessManagerStatics2Impl: Sized {
    fn SuppressLocalAccountWithAccountAsync(&self, userdataaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn CreateDeviceAccountAsync(&self, account: &::core::option::Option<DeviceAccountConfiguration>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn DeleteDeviceAccountAsync(&self, accountid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn GetDeviceAccountConfigurationAsync(&self, accountid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<DeviceAccountConfiguration>>;
}
