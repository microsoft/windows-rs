#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IMicrosoftAccountMultiFactorAuthenticationManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMicrosoftAccountMultiFactorAuthenticatorStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMicrosoftAccountMultiFactorGetSessionsResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMicrosoftAccountMultiFactorOneTimeCodedInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMicrosoftAccountMultiFactorSessionInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MicrosoftAccountMultiFactorAuthenticationManager(pub *mut ::core::ffi::c_void);
pub struct MicrosoftAccountMultiFactorAuthenticationType(i32);
#[repr(transparent)]
pub struct MicrosoftAccountMultiFactorGetSessionsResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MicrosoftAccountMultiFactorOneTimeCodedInfo(pub *mut ::core::ffi::c_void);
pub struct MicrosoftAccountMultiFactorServiceResponse(i32);
pub struct MicrosoftAccountMultiFactorSessionApprovalStatus(i32);
pub struct MicrosoftAccountMultiFactorSessionAuthenticationStatus(i32);
#[repr(transparent)]
pub struct MicrosoftAccountMultiFactorSessionInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo(pub *mut ::core::ffi::c_void);
