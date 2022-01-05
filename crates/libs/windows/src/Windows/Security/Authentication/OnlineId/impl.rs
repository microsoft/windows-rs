#[cfg(feature = "implement_exclusive")]
pub trait IOnlineIdAuthenticatorImpl: Sized {
    fn AuthenticateUserAsync(&self, request: &::core::option::Option<OnlineIdServiceTicketRequest>) -> ::windows::core::Result<UserAuthenticationOperation>;
    fn AuthenticateUserAsyncAdvanced(&self, requests: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<OnlineIdServiceTicketRequest>>, credentialprompttype: CredentialPromptType) -> ::windows::core::Result<UserAuthenticationOperation>;
    fn SignOutUserAsync(&self) -> ::windows::core::Result<SignOutUserOperation>;
    fn SetApplicationId(&self, value: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn ApplicationId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn CanSignOut(&self) -> ::windows::core::Result<bool>;
    fn AuthenticatedSafeCustomerId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IOnlineIdServiceTicketImpl: Sized {
    fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Request(&self) -> ::windows::core::Result<OnlineIdServiceTicketRequest>;
    fn ErrorCode(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IOnlineIdServiceTicketRequestImpl: Sized {
    fn Service(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Policy(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IOnlineIdServiceTicketRequestFactoryImpl: Sized {
    fn CreateOnlineIdServiceTicketRequest(&self, service: &::windows::core::HSTRING, policy: &::windows::core::HSTRING) -> ::windows::core::Result<OnlineIdServiceTicketRequest>;
    fn CreateOnlineIdServiceTicketRequestAdvanced(&self, service: &::windows::core::HSTRING) -> ::windows::core::Result<OnlineIdServiceTicketRequest>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IOnlineIdSystemAuthenticatorForUserImpl: Sized {
    fn GetTicketAsync(&self, request: &::core::option::Option<OnlineIdServiceTicketRequest>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<OnlineIdSystemTicketResult>>;
    fn SetApplicationId(&self, value: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn ApplicationId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn User(&self) -> ::windows::core::Result<super::super::super::System::User>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IOnlineIdSystemAuthenticatorStaticsImpl: Sized {
    fn Default(&self) -> ::windows::core::Result<OnlineIdSystemAuthenticatorForUser>;
    fn GetForUser(&self, user: &::core::option::Option<super::super::super::System::User>) -> ::windows::core::Result<OnlineIdSystemAuthenticatorForUser>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IOnlineIdSystemIdentityImpl: Sized {
    fn Ticket(&self) -> ::windows::core::Result<OnlineIdServiceTicket>;
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IOnlineIdSystemTicketResultImpl: Sized {
    fn Identity(&self) -> ::windows::core::Result<OnlineIdSystemIdentity>;
    fn Status(&self) -> ::windows::core::Result<OnlineIdSystemTicketStatus>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserIdentityImpl: Sized {
    fn Tickets(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<OnlineIdServiceTicket>>;
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SafeCustomerId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SignInName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FirstName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LastName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsBetaAccount(&self) -> ::windows::core::Result<bool>;
    fn IsConfirmedPC(&self) -> ::windows::core::Result<bool>;
}
