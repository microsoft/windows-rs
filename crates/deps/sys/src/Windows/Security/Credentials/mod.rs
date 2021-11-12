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
    pub const Success: KeyCredentialAttestationStatus = KeyCredentialAttestationStatus(0i32);
    pub const UnknownError: KeyCredentialAttestationStatus = KeyCredentialAttestationStatus(1i32);
    pub const NotSupported: KeyCredentialAttestationStatus = KeyCredentialAttestationStatus(2i32);
    pub const TemporaryFailure: KeyCredentialAttestationStatus = KeyCredentialAttestationStatus(3i32);
}
#[repr(transparent)]
pub struct KeyCredentialCreationOption(pub i32);
impl KeyCredentialCreationOption {
    pub const ReplaceExisting: KeyCredentialCreationOption = KeyCredentialCreationOption(0i32);
    pub const FailIfExists: KeyCredentialCreationOption = KeyCredentialCreationOption(1i32);
}
#[repr(transparent)]
pub struct KeyCredentialOperationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct KeyCredentialRetrievalResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct KeyCredentialStatus(pub i32);
impl KeyCredentialStatus {
    pub const Success: KeyCredentialStatus = KeyCredentialStatus(0i32);
    pub const UnknownError: KeyCredentialStatus = KeyCredentialStatus(1i32);
    pub const NotFound: KeyCredentialStatus = KeyCredentialStatus(2i32);
    pub const UserCanceled: KeyCredentialStatus = KeyCredentialStatus(3i32);
    pub const UserPrefersPassword: KeyCredentialStatus = KeyCredentialStatus(4i32);
    pub const CredentialAlreadyExists: KeyCredentialStatus = KeyCredentialStatus(5i32);
    pub const SecurityDeviceLocked: KeyCredentialStatus = KeyCredentialStatus(6i32);
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
    pub const Size64x64: WebAccountPictureSize = WebAccountPictureSize(64i32);
    pub const Size208x208: WebAccountPictureSize = WebAccountPictureSize(208i32);
    pub const Size424x424: WebAccountPictureSize = WebAccountPictureSize(424i32);
    pub const Size1080x1080: WebAccountPictureSize = WebAccountPictureSize(1080i32);
}
#[repr(transparent)]
pub struct WebAccountProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebAccountState(pub i32);
impl WebAccountState {
    pub const None: WebAccountState = WebAccountState(0i32);
    pub const Connected: WebAccountState = WebAccountState(1i32);
    pub const Error: WebAccountState = WebAccountState(2i32);
}
