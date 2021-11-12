#![allow(non_snake_case, non_camel_case_types)]
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
#[repr(C)]
pub struct KeyCredentialAttestationStatus(i32);
#[repr(C)]
pub struct KeyCredentialCreationOption(i32);
#[repr(transparent)]
pub struct KeyCredentialOperationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct KeyCredentialRetrievalResult(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct KeyCredentialStatus(i32);
#[repr(transparent)]
pub struct PasswordCredential(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PasswordCredentialPropertyStore(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PasswordVault(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebAccount(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct WebAccountPictureSize(i32);
#[repr(transparent)]
pub struct WebAccountProvider(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct WebAccountState(i32);
