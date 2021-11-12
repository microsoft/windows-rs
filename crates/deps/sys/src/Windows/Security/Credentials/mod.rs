#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Security_Credentials_UI")]
pub mod UI;
#[link(name = "windows")]
extern "system" {}
pub struct ICredentialFactory(pub *mut ::core::ffi::c_void);
pub struct IKeyCredential(pub *mut ::core::ffi::c_void);
pub struct IKeyCredentialAttestationResult(pub *mut ::core::ffi::c_void);
pub struct IKeyCredentialManagerStatics(pub *mut ::core::ffi::c_void);
pub struct IKeyCredentialOperationResult(pub *mut ::core::ffi::c_void);
pub struct IKeyCredentialRetrievalResult(pub *mut ::core::ffi::c_void);
pub struct IPasswordCredential(pub *mut ::core::ffi::c_void);
pub struct IPasswordVault(pub *mut ::core::ffi::c_void);
pub struct IWebAccount(pub *mut ::core::ffi::c_void);
pub struct IWebAccount2(pub *mut ::core::ffi::c_void);
pub struct IWebAccountFactory(pub *mut ::core::ffi::c_void);
pub struct IWebAccountProvider(pub *mut ::core::ffi::c_void);
pub struct IWebAccountProvider2(pub *mut ::core::ffi::c_void);
pub struct IWebAccountProvider3(pub *mut ::core::ffi::c_void);
pub struct IWebAccountProvider4(pub *mut ::core::ffi::c_void);
pub struct IWebAccountProviderFactory(pub *mut ::core::ffi::c_void);
pub struct KeyCredential(i32);
pub struct KeyCredentialAttestationResult(i32);
pub struct KeyCredentialAttestationStatus(i32);
pub struct KeyCredentialCreationOption(i32);
pub struct KeyCredentialManager(i32);
pub struct KeyCredentialOperationResult(i32);
pub struct KeyCredentialRetrievalResult(i32);
pub struct KeyCredentialStatus(i32);
pub struct PasswordCredential(i32);
pub struct PasswordCredentialPropertyStore(i32);
pub struct PasswordVault(i32);
pub struct WebAccount(i32);
pub struct WebAccountPictureSize(i32);
pub struct WebAccountProvider(i32);
pub struct WebAccountState(i32);
