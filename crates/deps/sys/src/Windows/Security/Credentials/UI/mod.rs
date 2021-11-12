#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AuthenticationProtocol(pub i32);
impl AuthenticationProtocol {
    pub const Basic: Self = Self(0i32);
    pub const Digest: Self = Self(1i32);
    pub const Ntlm: Self = Self(2i32);
    pub const Kerberos: Self = Self(3i32);
    pub const Negotiate: Self = Self(4i32);
    pub const CredSsp: Self = Self(5i32);
    pub const Custom: Self = Self(6i32);
}
impl ::core::marker::Copy for AuthenticationProtocol {}
impl ::core::clone::Clone for AuthenticationProtocol {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CredentialPickerOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CredentialPickerOptions {}
impl ::core::clone::Clone for CredentialPickerOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CredentialPickerResults(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CredentialPickerResults {}
impl ::core::clone::Clone for CredentialPickerResults {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CredentialSaveOption(pub i32);
impl CredentialSaveOption {
    pub const Unselected: Self = Self(0i32);
    pub const Selected: Self = Self(1i32);
    pub const Hidden: Self = Self(2i32);
}
impl ::core::marker::Copy for CredentialSaveOption {}
impl ::core::clone::Clone for CredentialSaveOption {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICredentialPickerOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICredentialPickerOptions {}
impl ::core::clone::Clone for ICredentialPickerOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICredentialPickerResults(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICredentialPickerResults {}
impl ::core::clone::Clone for ICredentialPickerResults {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICredentialPickerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICredentialPickerStatics {}
impl ::core::clone::Clone for ICredentialPickerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUserConsentVerifierStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUserConsentVerifierStatics {}
impl ::core::clone::Clone for IUserConsentVerifierStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UserConsentVerificationResult(pub i32);
impl UserConsentVerificationResult {
    pub const Verified: Self = Self(0i32);
    pub const DeviceNotPresent: Self = Self(1i32);
    pub const NotConfiguredForUser: Self = Self(2i32);
    pub const DisabledByPolicy: Self = Self(3i32);
    pub const DeviceBusy: Self = Self(4i32);
    pub const RetriesExhausted: Self = Self(5i32);
    pub const Canceled: Self = Self(6i32);
}
impl ::core::marker::Copy for UserConsentVerificationResult {}
impl ::core::clone::Clone for UserConsentVerificationResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UserConsentVerifierAvailability(pub i32);
impl UserConsentVerifierAvailability {
    pub const Available: Self = Self(0i32);
    pub const DeviceNotPresent: Self = Self(1i32);
    pub const NotConfiguredForUser: Self = Self(2i32);
    pub const DisabledByPolicy: Self = Self(3i32);
    pub const DeviceBusy: Self = Self(4i32);
}
impl ::core::marker::Copy for UserConsentVerifierAvailability {}
impl ::core::clone::Clone for UserConsentVerifierAvailability {
    fn clone(&self) -> Self {
        *self
    }
}
