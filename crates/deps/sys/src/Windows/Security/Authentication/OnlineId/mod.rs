#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct CredentialPromptType(i32);
pub struct IOnlineIdAuthenticator(pub *mut ::core::ffi::c_void);
pub struct IOnlineIdServiceTicket(pub *mut ::core::ffi::c_void);
pub struct IOnlineIdServiceTicketRequest(pub *mut ::core::ffi::c_void);
pub struct IOnlineIdServiceTicketRequestFactory(pub *mut ::core::ffi::c_void);
pub struct IOnlineIdSystemAuthenticatorForUser(pub *mut ::core::ffi::c_void);
pub struct IOnlineIdSystemAuthenticatorStatics(pub *mut ::core::ffi::c_void);
pub struct IOnlineIdSystemIdentity(pub *mut ::core::ffi::c_void);
pub struct IOnlineIdSystemTicketResult(pub *mut ::core::ffi::c_void);
pub struct IUserIdentity(pub *mut ::core::ffi::c_void);
pub struct OnlineIdAuthenticator(i32);
pub struct OnlineIdServiceTicket(i32);
pub struct OnlineIdServiceTicketRequest(i32);
pub struct OnlineIdSystemAuthenticator(i32);
pub struct OnlineIdSystemAuthenticatorForUser(i32);
pub struct OnlineIdSystemIdentity(i32);
pub struct OnlineIdSystemTicketResult(i32);
pub struct OnlineIdSystemTicketStatus(i32);
pub struct SignOutUserOperation(i32);
pub struct UserAuthenticationOperation(i32);
pub struct UserIdentity(i32);
