#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "Security_Credentials_UI")]
pub mod UI;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ICredentialFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKeyCredential(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKeyCredentialAttestationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKeyCredentialManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKeyCredentialOperationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKeyCredentialRetrievalResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPasswordCredential(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPasswordVault(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebAccount(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebAccount2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebAccountFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebAccountProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebAccountProvider2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebAccountProvider3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebAccountProvider4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebAccountProviderFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct KeyCredential(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct KeyCredentialAttestationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct KeyCredentialAttestationStatus(pub i32);
impl KeyCredentialAttestationStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownError: Self = Self(1i32);
    pub const NotSupported: Self = Self(2i32);
    pub const TemporaryFailure: Self = Self(3i32);
}
impl ::core::marker::Copy for KeyCredentialAttestationStatus {}
impl ::core::clone::Clone for KeyCredentialAttestationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct KeyCredentialCreationOption(pub i32);
impl KeyCredentialCreationOption {
    pub const ReplaceExisting: Self = Self(0i32);
    pub const FailIfExists: Self = Self(1i32);
}
impl ::core::marker::Copy for KeyCredentialCreationOption {}
impl ::core::clone::Clone for KeyCredentialCreationOption {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct KeyCredentialOperationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct KeyCredentialRetrievalResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct KeyCredentialStatus(pub i32);
impl KeyCredentialStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownError: Self = Self(1i32);
    pub const NotFound: Self = Self(2i32);
    pub const UserCanceled: Self = Self(3i32);
    pub const UserPrefersPassword: Self = Self(4i32);
    pub const CredentialAlreadyExists: Self = Self(5i32);
    pub const SecurityDeviceLocked: Self = Self(6i32);
}
impl ::core::marker::Copy for KeyCredentialStatus {}
impl ::core::clone::Clone for KeyCredentialStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PasswordCredential(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PasswordCredentialPropertyStore(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PasswordVault(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebAccount(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebAccountPictureSize(pub i32);
impl WebAccountPictureSize {
    pub const Size64x64: Self = Self(64i32);
    pub const Size208x208: Self = Self(208i32);
    pub const Size424x424: Self = Self(424i32);
    pub const Size1080x1080: Self = Self(1080i32);
}
impl ::core::marker::Copy for WebAccountPictureSize {}
impl ::core::clone::Clone for WebAccountPictureSize {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebAccountProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebAccountState(pub i32);
impl WebAccountState {
    pub const None: Self = Self(0i32);
    pub const Connected: Self = Self(1i32);
    pub const Error: Self = Self(2i32);
}
impl ::core::marker::Copy for WebAccountState {}
impl ::core::clone::Clone for WebAccountState {
    fn clone(&self) -> Self {
        *self
    }
}
