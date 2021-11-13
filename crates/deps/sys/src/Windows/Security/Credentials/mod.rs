#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "Security_Credentials_UI")]
pub mod UI;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ICredentialFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICredentialFactory {}
impl ::core::clone::Clone for ICredentialFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IKeyCredential(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IKeyCredential {}
impl ::core::clone::Clone for IKeyCredential {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IKeyCredentialAttestationResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IKeyCredentialAttestationResult {}
impl ::core::clone::Clone for IKeyCredentialAttestationResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IKeyCredentialManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IKeyCredentialManagerStatics {}
impl ::core::clone::Clone for IKeyCredentialManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IKeyCredentialOperationResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IKeyCredentialOperationResult {}
impl ::core::clone::Clone for IKeyCredentialOperationResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IKeyCredentialRetrievalResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IKeyCredentialRetrievalResult {}
impl ::core::clone::Clone for IKeyCredentialRetrievalResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPasswordCredential(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPasswordCredential {}
impl ::core::clone::Clone for IPasswordCredential {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPasswordVault(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPasswordVault {}
impl ::core::clone::Clone for IPasswordVault {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebAccount(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebAccount {}
impl ::core::clone::Clone for IWebAccount {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebAccount2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebAccount2 {}
impl ::core::clone::Clone for IWebAccount2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebAccountFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebAccountFactory {}
impl ::core::clone::Clone for IWebAccountFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebAccountProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebAccountProvider {}
impl ::core::clone::Clone for IWebAccountProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebAccountProvider2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebAccountProvider2 {}
impl ::core::clone::Clone for IWebAccountProvider2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebAccountProvider3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebAccountProvider3 {}
impl ::core::clone::Clone for IWebAccountProvider3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebAccountProvider4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebAccountProvider4 {}
impl ::core::clone::Clone for IWebAccountProvider4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebAccountProviderFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebAccountProviderFactory {}
impl ::core::clone::Clone for IWebAccountProviderFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct KeyCredential(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for KeyCredential {}
impl ::core::clone::Clone for KeyCredential {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct KeyCredentialAttestationResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for KeyCredentialAttestationResult {}
impl ::core::clone::Clone for KeyCredentialAttestationResult {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for KeyCredentialOperationResult {}
impl ::core::clone::Clone for KeyCredentialOperationResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct KeyCredentialRetrievalResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for KeyCredentialRetrievalResult {}
impl ::core::clone::Clone for KeyCredentialRetrievalResult {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for PasswordCredential {}
impl ::core::clone::Clone for PasswordCredential {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PasswordCredentialPropertyStore(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PasswordCredentialPropertyStore {}
impl ::core::clone::Clone for PasswordCredentialPropertyStore {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PasswordVault(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PasswordVault {}
impl ::core::clone::Clone for PasswordVault {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebAccount(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebAccount {}
impl ::core::clone::Clone for WebAccount {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for WebAccountProvider {}
impl ::core::clone::Clone for WebAccountProvider {
    fn clone(&self) -> Self {
        *self
    }
}
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
