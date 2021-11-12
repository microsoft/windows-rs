#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
#[repr(transparent)]
pub struct MicrosoftAccountMultiFactorAuthenticationType(pub i32);
impl MicrosoftAccountMultiFactorAuthenticationType {
    pub const User: MicrosoftAccountMultiFactorAuthenticationType = MicrosoftAccountMultiFactorAuthenticationType(0i32);
    pub const Device: MicrosoftAccountMultiFactorAuthenticationType = MicrosoftAccountMultiFactorAuthenticationType(1i32);
}
#[repr(transparent)]
pub struct MicrosoftAccountMultiFactorGetSessionsResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MicrosoftAccountMultiFactorOneTimeCodedInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MicrosoftAccountMultiFactorServiceResponse(pub i32);
impl MicrosoftAccountMultiFactorServiceResponse {
    pub const Success: MicrosoftAccountMultiFactorServiceResponse = MicrosoftAccountMultiFactorServiceResponse(0i32);
    pub const Error: MicrosoftAccountMultiFactorServiceResponse = MicrosoftAccountMultiFactorServiceResponse(1i32);
    pub const NoNetworkConnection: MicrosoftAccountMultiFactorServiceResponse = MicrosoftAccountMultiFactorServiceResponse(2i32);
    pub const ServiceUnavailable: MicrosoftAccountMultiFactorServiceResponse = MicrosoftAccountMultiFactorServiceResponse(3i32);
    pub const TotpSetupDenied: MicrosoftAccountMultiFactorServiceResponse = MicrosoftAccountMultiFactorServiceResponse(4i32);
    pub const NgcNotSetup: MicrosoftAccountMultiFactorServiceResponse = MicrosoftAccountMultiFactorServiceResponse(5i32);
    pub const SessionAlreadyDenied: MicrosoftAccountMultiFactorServiceResponse = MicrosoftAccountMultiFactorServiceResponse(6i32);
    pub const SessionAlreadyApproved: MicrosoftAccountMultiFactorServiceResponse = MicrosoftAccountMultiFactorServiceResponse(7i32);
    pub const SessionExpired: MicrosoftAccountMultiFactorServiceResponse = MicrosoftAccountMultiFactorServiceResponse(8i32);
    pub const NgcNonceExpired: MicrosoftAccountMultiFactorServiceResponse = MicrosoftAccountMultiFactorServiceResponse(9i32);
    pub const InvalidSessionId: MicrosoftAccountMultiFactorServiceResponse = MicrosoftAccountMultiFactorServiceResponse(10i32);
    pub const InvalidSessionType: MicrosoftAccountMultiFactorServiceResponse = MicrosoftAccountMultiFactorServiceResponse(11i32);
    pub const InvalidOperation: MicrosoftAccountMultiFactorServiceResponse = MicrosoftAccountMultiFactorServiceResponse(12i32);
    pub const InvalidStateTransition: MicrosoftAccountMultiFactorServiceResponse = MicrosoftAccountMultiFactorServiceResponse(13i32);
    pub const DeviceNotFound: MicrosoftAccountMultiFactorServiceResponse = MicrosoftAccountMultiFactorServiceResponse(14i32);
    pub const FlowDisabled: MicrosoftAccountMultiFactorServiceResponse = MicrosoftAccountMultiFactorServiceResponse(15i32);
    pub const SessionNotApproved: MicrosoftAccountMultiFactorServiceResponse = MicrosoftAccountMultiFactorServiceResponse(16i32);
    pub const OperationCanceledByUser: MicrosoftAccountMultiFactorServiceResponse = MicrosoftAccountMultiFactorServiceResponse(17i32);
    pub const NgcDisabledByServer: MicrosoftAccountMultiFactorServiceResponse = MicrosoftAccountMultiFactorServiceResponse(18i32);
    pub const NgcKeyNotFoundOnServer: MicrosoftAccountMultiFactorServiceResponse = MicrosoftAccountMultiFactorServiceResponse(19i32);
    pub const UIRequired: MicrosoftAccountMultiFactorServiceResponse = MicrosoftAccountMultiFactorServiceResponse(20i32);
    pub const DeviceIdChanged: MicrosoftAccountMultiFactorServiceResponse = MicrosoftAccountMultiFactorServiceResponse(21i32);
}
#[repr(transparent)]
pub struct MicrosoftAccountMultiFactorSessionApprovalStatus(pub i32);
impl MicrosoftAccountMultiFactorSessionApprovalStatus {
    pub const Pending: MicrosoftAccountMultiFactorSessionApprovalStatus = MicrosoftAccountMultiFactorSessionApprovalStatus(0i32);
    pub const Approved: MicrosoftAccountMultiFactorSessionApprovalStatus = MicrosoftAccountMultiFactorSessionApprovalStatus(1i32);
    pub const Denied: MicrosoftAccountMultiFactorSessionApprovalStatus = MicrosoftAccountMultiFactorSessionApprovalStatus(2i32);
}
#[repr(transparent)]
pub struct MicrosoftAccountMultiFactorSessionAuthenticationStatus(pub i32);
impl MicrosoftAccountMultiFactorSessionAuthenticationStatus {
    pub const Authenticated: MicrosoftAccountMultiFactorSessionAuthenticationStatus = MicrosoftAccountMultiFactorSessionAuthenticationStatus(0i32);
    pub const Unauthenticated: MicrosoftAccountMultiFactorSessionAuthenticationStatus = MicrosoftAccountMultiFactorSessionAuthenticationStatus(1i32);
}
#[repr(transparent)]
pub struct MicrosoftAccountMultiFactorSessionInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo(pub *mut ::core::ffi::c_void);
