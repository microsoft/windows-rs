#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AuthenticationProtocol(pub i32);
impl AuthenticationProtocol {
    pub const Basic: AuthenticationProtocol = AuthenticationProtocol(0i32);
    pub const Digest: AuthenticationProtocol = AuthenticationProtocol(1i32);
    pub const Ntlm: AuthenticationProtocol = AuthenticationProtocol(2i32);
    pub const Kerberos: AuthenticationProtocol = AuthenticationProtocol(3i32);
    pub const Negotiate: AuthenticationProtocol = AuthenticationProtocol(4i32);
    pub const CredSsp: AuthenticationProtocol = AuthenticationProtocol(5i32);
    pub const Custom: AuthenticationProtocol = AuthenticationProtocol(6i32);
}
#[repr(transparent)]
pub struct CredentialPickerOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CredentialPickerResults(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CredentialSaveOption(pub i32);
impl CredentialSaveOption {
    pub const Unselected: CredentialSaveOption = CredentialSaveOption(0i32);
    pub const Selected: CredentialSaveOption = CredentialSaveOption(1i32);
    pub const Hidden: CredentialSaveOption = CredentialSaveOption(2i32);
}
#[repr(transparent)]
pub struct ICredentialPickerOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICredentialPickerResults(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICredentialPickerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserConsentVerifierStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserConsentVerificationResult(pub i32);
impl UserConsentVerificationResult {
    pub const Verified: UserConsentVerificationResult = UserConsentVerificationResult(0i32);
    pub const DeviceNotPresent: UserConsentVerificationResult = UserConsentVerificationResult(1i32);
    pub const NotConfiguredForUser: UserConsentVerificationResult = UserConsentVerificationResult(2i32);
    pub const DisabledByPolicy: UserConsentVerificationResult = UserConsentVerificationResult(3i32);
    pub const DeviceBusy: UserConsentVerificationResult = UserConsentVerificationResult(4i32);
    pub const RetriesExhausted: UserConsentVerificationResult = UserConsentVerificationResult(5i32);
    pub const Canceled: UserConsentVerificationResult = UserConsentVerificationResult(6i32);
}
#[repr(transparent)]
pub struct UserConsentVerifierAvailability(pub i32);
impl UserConsentVerifierAvailability {
    pub const Available: UserConsentVerifierAvailability = UserConsentVerifierAvailability(0i32);
    pub const DeviceNotPresent: UserConsentVerifierAvailability = UserConsentVerifierAvailability(1i32);
    pub const NotConfiguredForUser: UserConsentVerifierAvailability = UserConsentVerifierAvailability(2i32);
    pub const DisabledByPolicy: UserConsentVerifierAvailability = UserConsentVerifierAvailability(3i32);
    pub const DeviceBusy: UserConsentVerifierAvailability = UserConsentVerifierAvailability(4i32);
}
