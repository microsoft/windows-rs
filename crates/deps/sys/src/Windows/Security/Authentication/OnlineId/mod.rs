#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct CredentialPromptType(pub i32);
impl CredentialPromptType {
    pub const PromptIfNeeded: Self = Self(0i32);
    pub const RetypeCredentials: Self = Self(1i32);
    pub const DoNotPrompt: Self = Self(2i32);
}
impl ::core::marker::Copy for CredentialPromptType {}
impl ::core::clone::Clone for CredentialPromptType {
    fn clone(&self) -> Self {
        *self
    }
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
    pub const Success: Self = Self(0i32);
    pub const Error: Self = Self(1i32);
    pub const ServiceConnectionError: Self = Self(2i32);
}
impl ::core::marker::Copy for OnlineIdSystemTicketStatus {}
impl ::core::clone::Clone for OnlineIdSystemTicketStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SignOutUserOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserAuthenticationOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserIdentity(pub *mut ::core::ffi::c_void);
