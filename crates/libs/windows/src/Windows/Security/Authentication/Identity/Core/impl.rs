#[cfg(feature = "implement_exclusive")]
pub trait IMicrosoftAccountMultiFactorAuthenticationManagerImpl: Sized {
    fn GetOneTimePassCodeAsync(&self, useraccountid: &::windows::core::HSTRING, codelength: u32) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorOneTimeCodedInfo>>;
    fn AddDeviceAsync(&self, useraccountid: &::windows::core::HSTRING, authenticationtoken: &::windows::core::HSTRING, wnschannelid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>>;
    fn RemoveDeviceAsync(&self, useraccountid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>>;
    fn UpdateWnsChannelAsync(&self, useraccountid: &::windows::core::HSTRING, channeluri: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>>;
    fn GetSessionsAsync(&self, useraccountidlist: &::core::option::Option<super::super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorGetSessionsResult>>;
    fn GetSessionsAndUnregisteredAccountsAsync(&self, useraccountidlist: &::core::option::Option<super::super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo>>;
    fn ApproveSessionUsingAuthSessionInfoAsync(&self, sessionauthentictionstatus: MicrosoftAccountMultiFactorSessionAuthenticationStatus, authenticationsessioninfo: &::core::option::Option<MicrosoftAccountMultiFactorSessionInfo>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>>;
    fn ApproveSessionAsync(&self, sessionauthentictionstatus: MicrosoftAccountMultiFactorSessionAuthenticationStatus, useraccountid: &::windows::core::HSTRING, sessionid: &::windows::core::HSTRING, sessionauthenticationtype: MicrosoftAccountMultiFactorAuthenticationType) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>>;
    fn DenySessionUsingAuthSessionInfoAsync(&self, authenticationsessioninfo: &::core::option::Option<MicrosoftAccountMultiFactorSessionInfo>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>>;
    fn DenySessionAsync(&self, useraccountid: &::windows::core::HSTRING, sessionid: &::windows::core::HSTRING, sessionauthenticationtype: MicrosoftAccountMultiFactorAuthenticationType) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMicrosoftAccountMultiFactorAuthenticatorStaticsImpl: Sized {
    fn Current(&self) -> ::windows::core::Result<MicrosoftAccountMultiFactorAuthenticationManager>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMicrosoftAccountMultiFactorGetSessionsResultImpl: Sized {
    fn Sessions(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<MicrosoftAccountMultiFactorSessionInfo>>;
    fn ServiceResponse(&self) -> ::windows::core::Result<MicrosoftAccountMultiFactorServiceResponse>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMicrosoftAccountMultiFactorOneTimeCodedInfoImpl: Sized {
    fn Code(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TimeInterval(&self) -> ::windows::core::Result<super::super::super::super::Foundation::TimeSpan>;
    fn TimeToLive(&self) -> ::windows::core::Result<super::super::super::super::Foundation::TimeSpan>;
    fn ServiceResponse(&self) -> ::windows::core::Result<MicrosoftAccountMultiFactorServiceResponse>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMicrosoftAccountMultiFactorSessionInfoImpl: Sized {
    fn UserAccountId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SessionId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplaySessionId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ApprovalStatus(&self) -> ::windows::core::Result<MicrosoftAccountMultiFactorSessionApprovalStatus>;
    fn AuthenticationType(&self) -> ::windows::core::Result<MicrosoftAccountMultiFactorAuthenticationType>;
    fn RequestTime(&self) -> ::windows::core::Result<super::super::super::super::Foundation::DateTime>;
    fn ExpirationTime(&self) -> ::windows::core::Result<super::super::super::super::Foundation::DateTime>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfoImpl: Sized {
    fn Sessions(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<MicrosoftAccountMultiFactorSessionInfo>>;
    fn UnregisteredAccounts(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn ServiceResponse(&self) -> ::windows::core::Result<MicrosoftAccountMultiFactorServiceResponse>;
}
