#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct CredentialPromptType(i32);
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
pub struct OnlineIdSystemAuthenticator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct OnlineIdSystemAuthenticatorForUser(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct OnlineIdSystemIdentity(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct OnlineIdSystemTicketResult(pub *mut ::core::ffi::c_void);
pub struct OnlineIdSystemTicketStatus(i32);
#[repr(transparent)]
pub struct SignOutUserOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserAuthenticationOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserIdentity(pub *mut ::core::ffi::c_void);
