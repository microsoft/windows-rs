#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct CredentialPromptType(pub i32);
impl CredentialPromptType {
    pub const PromptIfNeeded: CredentialPromptType = CredentialPromptType(0i32);
    pub const RetypeCredentials: CredentialPromptType = CredentialPromptType(1i32);
    pub const DoNotPrompt: CredentialPromptType = CredentialPromptType(2i32);
}
#[repr(transparent)]
pub struct IOnlineIdAuthenticator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOnlineIdServiceTicket(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOnlineIdServiceTicketRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOnlineIdServiceTicketRequestFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOnlineIdSystemAuthenticatorForUser(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOnlineIdSystemAuthenticatorStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOnlineIdSystemIdentity(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOnlineIdSystemTicketResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserIdentity(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct OnlineIdAuthenticator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct OnlineIdServiceTicket(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct OnlineIdServiceTicketRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct OnlineIdSystemAuthenticatorForUser(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct OnlineIdSystemIdentity(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct OnlineIdSystemTicketResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct OnlineIdSystemTicketStatus(pub i32);
impl OnlineIdSystemTicketStatus {
    pub const Success: OnlineIdSystemTicketStatus = OnlineIdSystemTicketStatus(0i32);
    pub const Error: OnlineIdSystemTicketStatus = OnlineIdSystemTicketStatus(1i32);
    pub const ServiceConnectionError: OnlineIdSystemTicketStatus = OnlineIdSystemTicketStatus(2i32);
}
#[repr(transparent)]
pub struct SignOutUserOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserAuthenticationOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserIdentity(pub *mut ::core::ffi::c_void);
