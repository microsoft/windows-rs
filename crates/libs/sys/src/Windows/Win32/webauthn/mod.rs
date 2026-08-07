#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("webauthn.dll" "system" fn WebAuthNAuthenticatorGetAssertion(hwnd : super::HWND, pwszrpid : windows_sys::core::PCWSTR, pwebauthnclientdata : *const WEBAUTHN_CLIENT_DATA, pwebauthngetassertionoptions : *const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS, ppwebauthnassertion : *mut PWEBAUTHN_ASSERTION) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("webauthn.dll" "system" fn WebAuthNAuthenticatorMakeCredential(hwnd : super::HWND, prpinformation : *const WEBAUTHN_RP_ENTITY_INFORMATION, puserinformation : *const WEBAUTHN_USER_ENTITY_INFORMATION, ppubkeycredparams : *const WEBAUTHN_COSE_CREDENTIAL_PARAMETERS, pwebauthnclientdata : *const WEBAUTHN_CLIENT_DATA, pwebauthnmakecredentialoptions : *const WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS, ppwebauthncredentialattestation : *mut PWEBAUTHN_CREDENTIAL_ATTESTATION) -> windows_sys::core::HRESULT);
windows_link::link!("webauthn.dll" "system" fn WebAuthNCancelCurrentOperation(pcancellationid : *const windows_sys::core::GUID) -> windows_sys::core::HRESULT);
windows_link::link!("webauthn.dll" "system" fn WebAuthNDeletePlatformCredential(cbcredentialid : u32, pbcredentialid : *const u8) -> windows_sys::core::HRESULT);
#[cfg(feature = "minwindef")]
windows_link::link!("webauthn.dll" "system" fn WebAuthNFreeAssertion(pwebauthnassertion : *const WEBAUTHN_ASSERTION));
#[cfg(feature = "minwindef")]
windows_link::link!("webauthn.dll" "system" fn WebAuthNFreeAuthenticatorList(pauthenticatordetailslist : *const WEBAUTHN_AUTHENTICATOR_DETAILS_LIST));
#[cfg(feature = "minwindef")]
windows_link::link!("webauthn.dll" "system" fn WebAuthNFreeCredentialAttestation(pwebauthncredentialattestation : *const WEBAUTHN_CREDENTIAL_ATTESTATION));
#[cfg(feature = "minwindef")]
windows_link::link!("webauthn.dll" "system" fn WebAuthNFreePlatformCredentialList(pcredentialdetailslist : *const WEBAUTHN_CREDENTIAL_DETAILS_LIST));
windows_link::link!("webauthn.dll" "system" fn WebAuthNGetApiVersionNumber() -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("webauthn.dll" "system" fn WebAuthNGetAuthenticatorList(pwebauthngetauthenticatorlistoptions : *const WEBAUTHN_AUTHENTICATOR_DETAILS_OPTIONS, ppauthenticatordetailslist : *mut PWEBAUTHN_AUTHENTICATOR_DETAILS_LIST) -> windows_sys::core::HRESULT);
windows_link::link!("webauthn.dll" "system" fn WebAuthNGetCancellationId(pcancellationid : *mut windows_sys::core::GUID) -> windows_sys::core::HRESULT);
windows_link::link!("webauthn.dll" "system" fn WebAuthNGetErrorName(hr : windows_sys::core::HRESULT) -> windows_sys::core::PCWSTR);
#[cfg(feature = "minwindef")]
windows_link::link!("webauthn.dll" "system" fn WebAuthNGetPlatformCredentialList(pgetcredentialsoptions : *const WEBAUTHN_GET_CREDENTIALS_OPTIONS, ppcredentialdetailslist : *mut PWEBAUTHN_CREDENTIAL_DETAILS_LIST) -> windows_sys::core::HRESULT);
windows_link::link!("webauthn.dll" "system" fn WebAuthNGetW3CExceptionDOMError(hr : windows_sys::core::HRESULT) -> windows_sys::core::HRESULT);
windows_link::link!("webauthn.dll" "system" fn WebAuthNIsUserVerifyingPlatformAuthenticatorAvailable(pbisuserverifyingplatformauthenticatoravailable : *mut windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct CTAPCBOR_HYBRID_STORAGE_LINKED_DATA {
    pub dwVersion: u32,
    pub cbContactId: u32,
    pub pbContactId: super::PBYTE,
    pub cbLinkId: u32,
    pub pbLinkId: super::PBYTE,
    pub cbLinkSecret: u32,
    pub pbLinkSecret: super::PBYTE,
    pub cbPublicKey: u32,
    pub pbPublicKey: super::PBYTE,
    pub pwszAuthenticatorName: windows_sys::core::PCWSTR,
    pub wEncodedTunnelServerDomain: u16,
}
pub const CTAPCBOR_HYBRID_STORAGE_LINKED_DATA_CURRENT_VERSION: i32 = 1;
pub const CTAPCBOR_HYBRID_STORAGE_LINKED_DATA_VERSION_1: i32 = 1;
pub type EXPERIMENTAL_PCWEBAUTHN_CTAPCBOR_AUTHENTICATOR_OPTIONS = *const EXPERIMENTAL_WEBAUTHN_CTAPCBOR_AUTHENTICATOR_OPTIONS;
#[cfg(feature = "minwindef")]
pub type EXPERIMENTAL_PCWEBAUTHN_CTAPCBOR_ECC_PUBLIC_KEY = *const EXPERIMENTAL_WEBAUTHN_CTAPCBOR_ECC_PUBLIC_KEY;
#[cfg(feature = "minwindef")]
pub type EXPERIMENTAL_PCWEBAUTHN_CTAPCBOR_GET_ASSERTION_REQUEST = *const EXPERIMENTAL_WEBAUTHN_CTAPCBOR_GET_ASSERTION_REQUEST;
#[cfg(feature = "minwindef")]
pub type EXPERIMENTAL_PCWEBAUTHN_CTAPCBOR_GET_ASSERTION_RESPONSE = *const EXPERIMENTAL_WEBAUTHN_CTAPCBOR_GET_ASSERTION_RESPONSE;
#[cfg(feature = "minwindef")]
pub type EXPERIMENTAL_PCWEBAUTHN_CTAPCBOR_HMAC_SALT_EXTENSION = *const EXPERIMENTAL_WEBAUTHN_CTAPCBOR_HMAC_SALT_EXTENSION;
#[cfg(feature = "minwindef")]
pub type EXPERIMENTAL_PCWEBAUTHN_CTAPCBOR_MAKE_CREDENTIAL_REQUEST = *const EXPERIMENTAL_WEBAUTHN_CTAPCBOR_MAKE_CREDENTIAL_REQUEST;
#[cfg(feature = "minwindef")]
pub type EXPERIMENTAL_PCWEBAUTHN_PLUGIN_ADD_AUTHENTICATOR_OPTIONS = *const EXPERIMENTAL_WEBAUTHN_PLUGIN_ADD_AUTHENTICATOR_OPTIONS;
#[cfg(feature = "minwindef")]
pub type EXPERIMENTAL_PCWEBAUTHN_PLUGIN_ADD_AUTHENTICATOR_RESPONSE = *const EXPERIMENTAL_WEBAUTHN_PLUGIN_ADD_AUTHENTICATOR_RESPONSE;
#[cfg(feature = "minwindef")]
pub type EXPERIMENTAL_PCWEBAUTHN_PLUGIN_CREDENTIAL_DETAILS = *const EXPERIMENTAL_WEBAUTHN_PLUGIN_CREDENTIAL_DETAILS;
#[cfg(feature = "minwindef")]
pub type EXPERIMENTAL_PCWEBAUTHN_PLUGIN_CREDENTIAL_DETAILS_LIST = *const EXPERIMENTAL_WEBAUTHN_PLUGIN_CREDENTIAL_DETAILS_LIST;
#[cfg(feature = "windef")]
pub type EXPERIMENTAL_PCWEBAUTHN_PLUGIN_PERFORM_UV = *const EXPERIMENTAL_WEBAUTHN_PLUGIN_PERFORM_UV;
#[cfg(feature = "minwindef")]
pub type EXPERIMENTAL_PCWEBAUTHN_PLUGIN_PERFORM_UV_RESPONSE = *const EXPERIMENTAL_WEBAUTHN_PLUGIN_PERFORM_UV_RESPONSE;
#[cfg(feature = "minwindef")]
pub type EXPERIMENTAL_PCWEBAUTHN_PLUGIN_UPDATE_AUTHENTICATOR_DETAILS = *const EXPERIMENTAL_WEBAUTHN_PLUGIN_UPDATE_AUTHENTICATOR_DETAILS;
pub type EXPERIMENTAL_PLUGIN_AUTHENTICATOR_STATE = i32;
pub type EXPERIMENTAL_PWEBAUTHN_CTAPCBOR_AUTHENTICATOR_OPTIONS = *mut EXPERIMENTAL_WEBAUTHN_CTAPCBOR_AUTHENTICATOR_OPTIONS;
#[cfg(feature = "minwindef")]
pub type EXPERIMENTAL_PWEBAUTHN_CTAPCBOR_ECC_PUBLIC_KEY = *mut EXPERIMENTAL_WEBAUTHN_CTAPCBOR_ECC_PUBLIC_KEY;
#[cfg(feature = "minwindef")]
pub type EXPERIMENTAL_PWEBAUTHN_CTAPCBOR_GET_ASSERTION_REQUEST = *mut EXPERIMENTAL_WEBAUTHN_CTAPCBOR_GET_ASSERTION_REQUEST;
#[cfg(feature = "minwindef")]
pub type EXPERIMENTAL_PWEBAUTHN_CTAPCBOR_GET_ASSERTION_RESPONSE = *mut EXPERIMENTAL_WEBAUTHN_CTAPCBOR_GET_ASSERTION_RESPONSE;
#[cfg(feature = "minwindef")]
pub type EXPERIMENTAL_PWEBAUTHN_CTAPCBOR_HMAC_SALT_EXTENSION = *mut EXPERIMENTAL_WEBAUTHN_CTAPCBOR_HMAC_SALT_EXTENSION;
#[cfg(feature = "minwindef")]
pub type EXPERIMENTAL_PWEBAUTHN_CTAPCBOR_MAKE_CREDENTIAL_REQUEST = *mut EXPERIMENTAL_WEBAUTHN_CTAPCBOR_MAKE_CREDENTIAL_REQUEST;
#[cfg(feature = "minwindef")]
pub type EXPERIMENTAL_PWEBAUTHN_PLUGIN_ADD_AUTHENTICATOR_OPTIONS = *mut EXPERIMENTAL_WEBAUTHN_PLUGIN_ADD_AUTHENTICATOR_OPTIONS;
#[cfg(feature = "minwindef")]
pub type EXPERIMENTAL_PWEBAUTHN_PLUGIN_ADD_AUTHENTICATOR_RESPONSE = *mut EXPERIMENTAL_WEBAUTHN_PLUGIN_ADD_AUTHENTICATOR_RESPONSE;
#[cfg(feature = "minwindef")]
pub type EXPERIMENTAL_PWEBAUTHN_PLUGIN_CREDENTIAL_DETAILS = *mut EXPERIMENTAL_WEBAUTHN_PLUGIN_CREDENTIAL_DETAILS;
#[cfg(feature = "minwindef")]
pub type EXPERIMENTAL_PWEBAUTHN_PLUGIN_CREDENTIAL_DETAILS_LIST = *mut EXPERIMENTAL_WEBAUTHN_PLUGIN_CREDENTIAL_DETAILS_LIST;
#[cfg(feature = "minwindef")]
pub type EXPERIMENTAL_PWEBAUTHN_PLUGIN_PERFORM_UV_RESPONSE = *mut EXPERIMENTAL_WEBAUTHN_PLUGIN_PERFORM_UV_RESPONSE;
#[cfg(feature = "windef")]
pub type EXPERIMENTAL_PWEBAUTHN_PLUGIN_PERFROM_UV = *mut EXPERIMENTAL_WEBAUTHN_PLUGIN_PERFORM_UV;
#[cfg(feature = "minwindef")]
pub type EXPERIMENTAL_PWEBAUTHN_PLUGIN_UPDATE_AUTHENTICATOR_DETAILS = *mut EXPERIMENTAL_WEBAUTHN_PLUGIN_UPDATE_AUTHENTICATOR_DETAILS;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct EXPERIMENTAL_WEBAUTHN_CTAPCBOR_AUTHENTICATOR_OPTIONS {
    pub dwVersion: u32,
    pub lUp: i32,
    pub lUv: i32,
    pub lRequireResidentKey: i32,
}
pub const EXPERIMENTAL_WEBAUTHN_CTAPCBOR_AUTHENTICATOR_OPTIONS_CURRENT_VERSION: i32 = 1;
pub const EXPERIMENTAL_WEBAUTHN_CTAPCBOR_AUTHENTICATOR_OPTIONS_VERSION_1: i32 = 1;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct EXPERIMENTAL_WEBAUTHN_CTAPCBOR_ECC_PUBLIC_KEY {
    pub dwVersion: u32,
    pub lKty: i32,
    pub lAlg: i32,
    pub lCrv: i32,
    pub cbX: u32,
    pub pbX: super::PBYTE,
    pub cbY: u32,
    pub pbY: super::PBYTE,
}
pub const EXPERIMENTAL_WEBAUTHN_CTAPCBOR_ECC_PUBLIC_KEY_CURRENT_VERSION: i32 = 1;
pub const EXPERIMENTAL_WEBAUTHN_CTAPCBOR_ECC_PUBLIC_KEY_VERSION_1: i32 = 1;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct EXPERIMENTAL_WEBAUTHN_CTAPCBOR_GET_ASSERTION_REQUEST {
    pub dwVersion: u32,
    pub pwszRpId: windows_sys::core::PCWSTR,
    pub cbRpId: u32,
    pub pbRpId: super::PBYTE,
    pub cbClientDataHash: u32,
    pub pbClientDataHash: super::PBYTE,
    pub CredentialList: WEBAUTHN_CREDENTIAL_LIST,
    pub cbCborExtensionsMap: u32,
    pub pbCborExtensionsMap: super::PBYTE,
    pub pAuthenticatorOptions: EXPERIMENTAL_PWEBAUTHN_CTAPCBOR_AUTHENTICATOR_OPTIONS,
    pub fEmptyPinAuth: windows_sys::core::BOOL,
    pub cbPinAuth: u32,
    pub pbPinAuth: super::PBYTE,
    pub pHmacSaltExtension: EXPERIMENTAL_PWEBAUTHN_CTAPCBOR_HMAC_SALT_EXTENSION,
    pub cbHmacSecretSaltValues: u32,
    pub pbHmacSecretSaltValues: super::PBYTE,
    pub dwPinProtocol: u32,
    pub lCredBlobExt: i32,
    pub lLargeBlobKeyExt: i32,
    pub dwCredLargeBlobOperation: u32,
    pub cbCredLargeBlobCompressed: u32,
    pub pbCredLargeBlobCompressed: super::PBYTE,
    pub dwCredLargeBlobOriginalSize: u32,
    pub cbJsonExt: u32,
    pub pbJsonExt: super::PBYTE,
}
pub const EXPERIMENTAL_WEBAUTHN_CTAPCBOR_GET_ASSERTION_REQUEST_CURRENT_VERSION: i32 = 1;
pub const EXPERIMENTAL_WEBAUTHN_CTAPCBOR_GET_ASSERTION_REQUEST_VERSION_1: i32 = 1;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct EXPERIMENTAL_WEBAUTHN_CTAPCBOR_GET_ASSERTION_RESPONSE {
    pub WebAuthNAssertion: WEBAUTHN_ASSERTION,
    pub pUserInformation: PCWEBAUTHN_USER_ENTITY_INFORMATION,
    pub dwNumberOfCredentials: u32,
    pub lUserSelected: i32,
    pub cbLargeBlobKey: u32,
    pub pbLargeBlobKey: super::PBYTE,
    pub cbUnsignedExtensionOutputs: u32,
    pub pbUnsignedExtensionOutputs: super::PBYTE,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct EXPERIMENTAL_WEBAUTHN_CTAPCBOR_HMAC_SALT_EXTENSION {
    pub dwVersion: u32,
    pub pKeyAgreement: EXPERIMENTAL_PWEBAUTHN_CTAPCBOR_ECC_PUBLIC_KEY,
    pub cbEncryptedSalt: u32,
    pub pbEncryptedSalt: super::PBYTE,
    pub cbSaltAuth: u32,
    pub pbSaltAuth: super::PBYTE,
}
pub const EXPERIMENTAL_WEBAUTHN_CTAPCBOR_HMAC_SALT_EXTENSION_CURRENT_VERSION: i32 = 1;
pub const EXPERIMENTAL_WEBAUTHN_CTAPCBOR_HMAC_SALT_EXTENSION_VERSION_1: i32 = 1;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct EXPERIMENTAL_WEBAUTHN_CTAPCBOR_MAKE_CREDENTIAL_REQUEST {
    pub dwVersion: u32,
    pub cbRpId: u32,
    pub pbRpId: super::PBYTE,
    pub cbClientDataHash: u32,
    pub pbClientDataHash: super::PBYTE,
    pub pRpInformation: PCWEBAUTHN_RP_ENTITY_INFORMATION,
    pub pUserInformation: PCWEBAUTHN_USER_ENTITY_INFORMATION,
    pub WebAuthNCredentialParameters: WEBAUTHN_COSE_CREDENTIAL_PARAMETERS,
    pub CredentialList: WEBAUTHN_CREDENTIAL_LIST,
    pub cbCborExtensionsMap: u32,
    pub pbCborExtensionsMap: super::PBYTE,
    pub pAuthenticatorOptions: EXPERIMENTAL_PWEBAUTHN_CTAPCBOR_AUTHENTICATOR_OPTIONS,
    pub fEmptyPinAuth: windows_sys::core::BOOL,
    pub cbPinAuth: u32,
    pub pbPinAuth: super::PBYTE,
    pub lHmacSecretExt: i32,
    pub pHmacSecretMcExtension: EXPERIMENTAL_PWEBAUTHN_CTAPCBOR_HMAC_SALT_EXTENSION,
    pub lPrfExt: i32,
    pub cbHmacSecretSaltValues: u32,
    pub pbHmacSecretSaltValues: super::PBYTE,
    pub dwCredProtect: u32,
    pub dwPinProtocol: u32,
    pub dwEnterpriseAttestation: u32,
    pub cbCredBlobExt: u32,
    pub pbCredBlobExt: super::PBYTE,
    pub lLargeBlobKeyExt: i32,
    pub dwLargeBlobSupport: u32,
    pub lMinPinLengthExt: i32,
    pub cbJsonExt: u32,
    pub pbJsonExt: super::PBYTE,
}
pub const EXPERIMENTAL_WEBAUTHN_CTAPCBOR_MAKE_CREDENTIAL_REQUEST_CURRENT_VERSION: i32 = 1;
pub const EXPERIMENTAL_WEBAUTHN_CTAPCBOR_MAKE_CREDENTIAL_REQUEST_VERSION_1: i32 = 1;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct EXPERIMENTAL_WEBAUTHN_PLUGIN_ADD_AUTHENTICATOR_OPTIONS {
    pub pwszAuthenticatorName: windows_sys::core::PCWSTR,
    pub pwszPluginClsId: windows_sys::core::PCWSTR,
    pub pwszPluginRpId: windows_sys::core::PCWSTR,
    pub pwszLightThemeLogo: windows_sys::core::PCWSTR,
    pub pwszDarkThemeLogo: windows_sys::core::PCWSTR,
    pub cbAuthenticatorInfo: u32,
    pub pbAuthenticatorInfo: super::PBYTE,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct EXPERIMENTAL_WEBAUTHN_PLUGIN_ADD_AUTHENTICATOR_RESPONSE {
    pub cbOpSignPubKey: u32,
    pub pbOpSignPubKey: super::PBYTE,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct EXPERIMENTAL_WEBAUTHN_PLUGIN_CREDENTIAL_DETAILS {
    pub cbCredentialId: u32,
    pub pbCredentialId: super::PBYTE,
    pub pwszRpId: windows_sys::core::PWSTR,
    pub pwszRpName: windows_sys::core::PWSTR,
    pub cbUserId: u32,
    pub pbUserId: super::PBYTE,
    pub pwszUserName: windows_sys::core::PWSTR,
    pub pwszUserDisplayName: windows_sys::core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct EXPERIMENTAL_WEBAUTHN_PLUGIN_CREDENTIAL_DETAILS_LIST {
    pub pwszPluginClsId: windows_sys::core::PWSTR,
    pub cCredentialDetails: u32,
    pub pCredentialDetails: *mut EXPERIMENTAL_PWEBAUTHN_PLUGIN_CREDENTIAL_DETAILS,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct EXPERIMENTAL_WEBAUTHN_PLUGIN_PERFORM_UV {
    pub hwnd: super::HWND,
    pub transactionId: *mut windows_sys::core::GUID,
    pub r#type: EXPERIMENTAL_WEBAUTHN_PLUGIN_PERFORM_UV_OPERATION_TYPE,
    pub pwszUsername: windows_sys::core::PCWSTR,
    pub pwszContext: windows_sys::core::PCWSTR,
}
pub type EXPERIMENTAL_WEBAUTHN_PLUGIN_PERFORM_UV_OPERATION_TYPE = i32;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct EXPERIMENTAL_WEBAUTHN_PLUGIN_PERFORM_UV_RESPONSE {
    pub cbResponse: u32,
    pub pbResponse: super::PBYTE,
}
pub type EXPERIMENTAL_WEBAUTHN_PLUGIN_STATUS_CHANGE_CALLBACK = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void)>;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct EXPERIMENTAL_WEBAUTHN_PLUGIN_UPDATE_AUTHENTICATOR_DETAILS {
    pub pwszAuthenticatorName: windows_sys::core::PCWSTR,
    pub pwszPluginClsId: windows_sys::core::PCWSTR,
    pub pwszNewPluginClsId: windows_sys::core::PCWSTR,
    pub pwszLightThemeLogo: windows_sys::core::PCWSTR,
    pub pwszDarkThemeLogo: windows_sys::core::PCWSTR,
    pub cbAuthenticatorInfo: u32,
    pub pbAuthenticatorInfo: super::PBYTE,
}
pub const GetPubKey: EXPERIMENTAL_WEBAUTHN_PLUGIN_PERFORM_UV_OPERATION_TYPE = 3;
pub const GetUvCount: EXPERIMENTAL_WEBAUTHN_PLUGIN_PERFORM_UV_OPERATION_TYPE = 2;
#[cfg(feature = "minwindef")]
pub type PCCTAPCBOR_HYBRID_STORAGE_LINKED_DATA = *const CTAPCBOR_HYBRID_STORAGE_LINKED_DATA;
#[cfg(feature = "minwindef")]
pub type PCTAPCBOR_HYBRID_STORAGE_LINKED_DATA = *mut CTAPCBOR_HYBRID_STORAGE_LINKED_DATA;
#[cfg(feature = "minwindef")]
pub type PCWEBAUTHN_ASSERTION = *const WEBAUTHN_ASSERTION;
#[cfg(feature = "minwindef")]
pub type PCWEBAUTHN_AUTHENTICATOR_DETAILS = *const WEBAUTHN_AUTHENTICATOR_DETAILS;
#[cfg(feature = "minwindef")]
pub type PCWEBAUTHN_AUTHENTICATOR_DETAILS_LIST = *const WEBAUTHN_AUTHENTICATOR_DETAILS_LIST;
pub type PCWEBAUTHN_AUTHENTICATOR_DETAILS_OPTIONS = *const WEBAUTHN_AUTHENTICATOR_DETAILS_OPTIONS;
#[cfg(feature = "minwindef")]
pub type PCWEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS = *const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS;
#[cfg(feature = "minwindef")]
pub type PCWEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS = *const WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS;
#[cfg(feature = "minwindef")]
pub type PCWEBAUTHN_CLIENT_DATA = *const WEBAUTHN_CLIENT_DATA;
#[cfg(feature = "minwindef")]
pub type PCWEBAUTHN_COMMON_ATTESTATION = *const WEBAUTHN_COMMON_ATTESTATION;
pub type PCWEBAUTHN_COSE_CREDENTIAL_PARAMETER = *const WEBAUTHN_COSE_CREDENTIAL_PARAMETER;
pub type PCWEBAUTHN_COSE_CREDENTIAL_PARAMETERS = *const WEBAUTHN_COSE_CREDENTIAL_PARAMETERS;
#[cfg(feature = "minwindef")]
pub type PCWEBAUTHN_CREDENTIAL = *const WEBAUTHN_CREDENTIAL;
#[cfg(feature = "minwindef")]
pub type PCWEBAUTHN_CREDENTIALS = *const WEBAUTHN_CREDENTIALS;
#[cfg(feature = "minwindef")]
pub type PCWEBAUTHN_CREDENTIAL_ATTESTATION = *const WEBAUTHN_CREDENTIAL_ATTESTATION;
#[cfg(feature = "minwindef")]
pub type PCWEBAUTHN_CREDENTIAL_DETAILS = *const WEBAUTHN_CREDENTIAL_DETAILS;
#[cfg(feature = "minwindef")]
pub type PCWEBAUTHN_CREDENTIAL_DETAILS_LIST = *const WEBAUTHN_CREDENTIAL_DETAILS_LIST;
#[cfg(feature = "minwindef")]
pub type PCWEBAUTHN_CREDENTIAL_EX = *const WEBAUTHN_CREDENTIAL_EX;
#[cfg(feature = "minwindef")]
pub type PCWEBAUTHN_CREDENTIAL_LIST = *const WEBAUTHN_CREDENTIAL_LIST;
#[cfg(feature = "minwindef")]
pub type PCWEBAUTHN_CRED_BLOB_EXTENSION = *const WEBAUTHN_CRED_BLOB_EXTENSION;
pub type PCWEBAUTHN_CRED_PROTECT_EXTENSION_IN = *const WEBAUTHN_CRED_PROTECT_EXTENSION_IN;
#[cfg(feature = "minwindef")]
pub type PCWEBAUTHN_CRED_WITH_HMAC_SECRET_SALT = *const WEBAUTHN_CRED_WITH_HMAC_SECRET_SALT;
pub type PCWEBAUTHN_EXTENSION = *const WEBAUTHN_EXTENSION;
pub type PCWEBAUTHN_EXTENSIONS = *const WEBAUTHN_EXTENSIONS;
pub type PCWEBAUTHN_GET_CREDENTIALS_OPTIONS = *const WEBAUTHN_GET_CREDENTIALS_OPTIONS;
#[cfg(feature = "minwindef")]
pub type PCWEBAUTHN_HMAC_SECRET_SALT = *const WEBAUTHN_HMAC_SECRET_SALT;
#[cfg(feature = "minwindef")]
pub type PCWEBAUTHN_HMAC_SECRET_SALT_VALUES = *const WEBAUTHN_HMAC_SECRET_SALT_VALUES;
pub type PCWEBAUTHN_RP_ENTITY_INFORMATION = *const WEBAUTHN_RP_ENTITY_INFORMATION;
#[cfg(feature = "minwindef")]
pub type PCWEBAUTHN_USER_ENTITY_INFORMATION = *const WEBAUTHN_USER_ENTITY_INFORMATION;
#[cfg(feature = "minwindef")]
pub type PWEBAUTHN_ASSERTION = *mut WEBAUTHN_ASSERTION;
#[cfg(feature = "minwindef")]
pub type PWEBAUTHN_AUTHENTICATOR_DETAILS = *mut WEBAUTHN_AUTHENTICATOR_DETAILS;
#[cfg(feature = "minwindef")]
pub type PWEBAUTHN_AUTHENTICATOR_DETAILS_LIST = *mut WEBAUTHN_AUTHENTICATOR_DETAILS_LIST;
pub type PWEBAUTHN_AUTHENTICATOR_DETAILS_OPTIONS = *mut WEBAUTHN_AUTHENTICATOR_DETAILS_OPTIONS;
#[cfg(feature = "minwindef")]
pub type PWEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS = *mut WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS;
#[cfg(feature = "minwindef")]
pub type PWEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS = *mut WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS;
#[cfg(feature = "minwindef")]
pub type PWEBAUTHN_CLIENT_DATA = *mut WEBAUTHN_CLIENT_DATA;
#[cfg(feature = "minwindef")]
pub type PWEBAUTHN_COMMON_ATTESTATION = *mut WEBAUTHN_COMMON_ATTESTATION;
pub type PWEBAUTHN_COSE_CREDENTIAL_PARAMETER = *mut WEBAUTHN_COSE_CREDENTIAL_PARAMETER;
pub type PWEBAUTHN_COSE_CREDENTIAL_PARAMETERS = *mut WEBAUTHN_COSE_CREDENTIAL_PARAMETERS;
#[cfg(feature = "minwindef")]
pub type PWEBAUTHN_CREDENTIAL = *mut WEBAUTHN_CREDENTIAL;
#[cfg(feature = "minwindef")]
pub type PWEBAUTHN_CREDENTIALS = *mut WEBAUTHN_CREDENTIALS;
#[cfg(feature = "minwindef")]
pub type PWEBAUTHN_CREDENTIAL_ATTESTATION = *mut WEBAUTHN_CREDENTIAL_ATTESTATION;
#[cfg(feature = "minwindef")]
pub type PWEBAUTHN_CREDENTIAL_DETAILS = *mut WEBAUTHN_CREDENTIAL_DETAILS;
#[cfg(feature = "minwindef")]
pub type PWEBAUTHN_CREDENTIAL_DETAILS_LIST = *mut WEBAUTHN_CREDENTIAL_DETAILS_LIST;
#[cfg(feature = "minwindef")]
pub type PWEBAUTHN_CREDENTIAL_EX = *mut WEBAUTHN_CREDENTIAL_EX;
#[cfg(feature = "minwindef")]
pub type PWEBAUTHN_CREDENTIAL_LIST = *mut WEBAUTHN_CREDENTIAL_LIST;
#[cfg(feature = "minwindef")]
pub type PWEBAUTHN_CRED_BLOB_EXTENSION = *mut WEBAUTHN_CRED_BLOB_EXTENSION;
pub type PWEBAUTHN_CRED_PROTECT_EXTENSION_IN = *mut WEBAUTHN_CRED_PROTECT_EXTENSION_IN;
#[cfg(feature = "minwindef")]
pub type PWEBAUTHN_CRED_WITH_HMAC_SECRET_SALT = *mut WEBAUTHN_CRED_WITH_HMAC_SECRET_SALT;
pub type PWEBAUTHN_EXTENSION = *mut WEBAUTHN_EXTENSION;
pub type PWEBAUTHN_EXTENSIONS = *mut WEBAUTHN_EXTENSIONS;
pub type PWEBAUTHN_GET_CREDENTIALS_OPTIONS = *mut WEBAUTHN_GET_CREDENTIALS_OPTIONS;
#[cfg(feature = "minwindef")]
pub type PWEBAUTHN_HMAC_SECRET_SALT = *mut WEBAUTHN_HMAC_SECRET_SALT;
#[cfg(feature = "minwindef")]
pub type PWEBAUTHN_HMAC_SECRET_SALT_VALUES = *mut WEBAUTHN_HMAC_SECRET_SALT_VALUES;
pub type PWEBAUTHN_RP_ENTITY_INFORMATION = *mut WEBAUTHN_RP_ENTITY_INFORMATION;
#[cfg(feature = "minwindef")]
pub type PWEBAUTHN_USER_ENTITY_INFORMATION = *mut WEBAUTHN_USER_ENTITY_INFORMATION;
#[cfg(feature = "minwindef")]
pub type PWEBAUTHN_X5C = *mut WEBAUTHN_X5C;
pub const PerformUv: EXPERIMENTAL_WEBAUTHN_PLUGIN_PERFORM_UV_OPERATION_TYPE = 1;
pub const PluginAuthenticatorState_Disabled: EXPERIMENTAL_PLUGIN_AUTHENTICATOR_STATE = 1;
pub const PluginAuthenticatorState_Enabled: EXPERIMENTAL_PLUGIN_AUTHENTICATOR_STATE = 2;
pub const PluginAuthenticatorState_Unknown: EXPERIMENTAL_PLUGIN_AUTHENTICATOR_STATE = 0;
pub const WEBAUTHN_API_CURRENT_VERSION: i32 = 9;
pub const WEBAUTHN_API_VERSION_1: i32 = 1;
pub const WEBAUTHN_API_VERSION_2: i32 = 2;
pub const WEBAUTHN_API_VERSION_3: i32 = 3;
pub const WEBAUTHN_API_VERSION_4: i32 = 4;
pub const WEBAUTHN_API_VERSION_5: i32 = 5;
pub const WEBAUTHN_API_VERSION_6: i32 = 6;
pub const WEBAUTHN_API_VERSION_7: i32 = 7;
pub const WEBAUTHN_API_VERSION_8: i32 = 8;
pub const WEBAUTHN_API_VERSION_9: i32 = 9;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct WEBAUTHN_ASSERTION {
    pub dwVersion: u32,
    pub cbAuthenticatorData: u32,
    pub pbAuthenticatorData: super::PBYTE,
    pub cbSignature: u32,
    pub pbSignature: super::PBYTE,
    pub Credential: WEBAUTHN_CREDENTIAL,
    pub cbUserId: u32,
    pub pbUserId: super::PBYTE,
    pub Extensions: WEBAUTHN_EXTENSIONS,
    pub cbCredLargeBlob: u32,
    pub pbCredLargeBlob: super::PBYTE,
    pub dwCredLargeBlobStatus: u32,
    pub pHmacSecret: PWEBAUTHN_HMAC_SECRET_SALT,
    pub dwUsedTransport: u32,
    pub cbUnsignedExtensionOutputs: u32,
    pub pbUnsignedExtensionOutputs: super::PBYTE,
    pub cbClientDataJSON: u32,
    pub pbClientDataJSON: super::PBYTE,
    pub cbAuthenticationResponseJSON: u32,
    pub pbAuthenticationResponseJSON: super::PBYTE,
}
pub const WEBAUTHN_ASSERTION_CURRENT_VERSION: i32 = 6;
pub const WEBAUTHN_ASSERTION_VERSION_1: i32 = 1;
pub const WEBAUTHN_ASSERTION_VERSION_2: i32 = 2;
pub const WEBAUTHN_ASSERTION_VERSION_3: i32 = 3;
pub const WEBAUTHN_ASSERTION_VERSION_4: i32 = 4;
pub const WEBAUTHN_ASSERTION_VERSION_5: i32 = 5;
pub const WEBAUTHN_ASSERTION_VERSION_6: i32 = 6;
pub const WEBAUTHN_ATTESTATION_CONVEYANCE_PREFERENCE_ANY: i32 = 0;
pub const WEBAUTHN_ATTESTATION_CONVEYANCE_PREFERENCE_DIRECT: i32 = 3;
pub const WEBAUTHN_ATTESTATION_CONVEYANCE_PREFERENCE_INDIRECT: i32 = 2;
pub const WEBAUTHN_ATTESTATION_CONVEYANCE_PREFERENCE_NONE: i32 = 1;
pub const WEBAUTHN_ATTESTATION_DECODE_COMMON: i32 = 1;
pub const WEBAUTHN_ATTESTATION_DECODE_NONE: i32 = 0;
pub const WEBAUTHN_ATTESTATION_TYPE_NONE: windows_sys::core::PCWSTR = windows_sys::core::w!("none");
pub const WEBAUTHN_ATTESTATION_TYPE_PACKED: windows_sys::core::PCWSTR = windows_sys::core::w!("packed");
pub const WEBAUTHN_ATTESTATION_TYPE_TPM: windows_sys::core::PCWSTR = windows_sys::core::w!("tpm");
pub const WEBAUTHN_ATTESTATION_TYPE_U2F: windows_sys::core::PCWSTR = windows_sys::core::w!("fido-u2f");
pub const WEBAUTHN_ATTESTATION_VER_TPM_2_0: windows_sys::core::PCWSTR = windows_sys::core::w!("2.0");
pub const WEBAUTHN_AUTHENTICATOR_ATTACHMENT_ANY: i32 = 0;
pub const WEBAUTHN_AUTHENTICATOR_ATTACHMENT_CROSS_PLATFORM: i32 = 2;
pub const WEBAUTHN_AUTHENTICATOR_ATTACHMENT_CROSS_PLATFORM_U2F_V2: i32 = 3;
pub const WEBAUTHN_AUTHENTICATOR_ATTACHMENT_PLATFORM: i32 = 1;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct WEBAUTHN_AUTHENTICATOR_DETAILS {
    pub dwVersion: u32,
    pub cbAuthenticatorId: u32,
    pub pbAuthenticatorId: super::PBYTE,
    pub pwszAuthenticatorName: windows_sys::core::PCWSTR,
    pub cbAuthenticatorLogo: u32,
    pub pbAuthenticatorLogo: super::PBYTE,
    pub bLocked: windows_sys::core::BOOL,
}
pub const WEBAUTHN_AUTHENTICATOR_DETAILS_CURRENT_VERSION: i32 = 1;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct WEBAUTHN_AUTHENTICATOR_DETAILS_LIST {
    pub cAuthenticatorDetails: u32,
    pub ppAuthenticatorDetails: *mut PWEBAUTHN_AUTHENTICATOR_DETAILS,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WEBAUTHN_AUTHENTICATOR_DETAILS_OPTIONS {
    pub dwVersion: u32,
}
pub const WEBAUTHN_AUTHENTICATOR_DETAILS_OPTIONS_CURRENT_VERSION: i32 = 1;
pub const WEBAUTHN_AUTHENTICATOR_DETAILS_OPTIONS_VERSION_1: i32 = 1;
pub const WEBAUTHN_AUTHENTICATOR_DETAILS_VERSION_1: i32 = 1;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS {
    pub dwVersion: u32,
    pub dwTimeoutMilliseconds: u32,
    pub CredentialList: WEBAUTHN_CREDENTIALS,
    pub Extensions: WEBAUTHN_EXTENSIONS,
    pub dwAuthenticatorAttachment: u32,
    pub dwUserVerificationRequirement: u32,
    pub dwFlags: u32,
    pub pwszU2fAppId: windows_sys::core::PCWSTR,
    pub pbU2fAppId: *mut windows_sys::core::BOOL,
    pub pCancellationId: *mut windows_sys::core::GUID,
    pub pAllowCredentialList: PWEBAUTHN_CREDENTIAL_LIST,
    pub dwCredLargeBlobOperation: u32,
    pub cbCredLargeBlob: u32,
    pub pbCredLargeBlob: super::PBYTE,
    pub pHmacSecretSaltValues: PWEBAUTHN_HMAC_SECRET_SALT_VALUES,
    pub bBrowserInPrivateMode: windows_sys::core::BOOL,
    pub pLinkedDevice: PCTAPCBOR_HYBRID_STORAGE_LINKED_DATA,
    pub bAutoFill: windows_sys::core::BOOL,
    pub cbJsonExt: u32,
    pub pbJsonExt: super::PBYTE,
    pub cCredentialHints: u32,
    pub ppwszCredentialHints: *mut windows_sys::core::PCWSTR,
    pub pwszRemoteWebOrigin: windows_sys::core::PCWSTR,
    pub cbPublicKeyCredentialRequestOptionsJSON: u32,
    pub pbPublicKeyCredentialRequestOptionsJSON: super::PBYTE,
    pub cbAuthenticatorId: u32,
    pub pbAuthenticatorId: super::PBYTE,
}
pub const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_CURRENT_VERSION: i32 = 9;
pub const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_VERSION_1: i32 = 1;
pub const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_VERSION_2: i32 = 2;
pub const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_VERSION_3: i32 = 3;
pub const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_VERSION_4: i32 = 4;
pub const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_VERSION_5: i32 = 5;
pub const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_VERSION_6: i32 = 6;
pub const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_VERSION_7: i32 = 7;
pub const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_VERSION_8: i32 = 8;
pub const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_VERSION_9: i32 = 9;
pub const WEBAUTHN_AUTHENTICATOR_HMAC_SECRET_VALUES_FLAG: i32 = 1048576;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS {
    pub dwVersion: u32,
    pub dwTimeoutMilliseconds: u32,
    pub CredentialList: WEBAUTHN_CREDENTIALS,
    pub Extensions: WEBAUTHN_EXTENSIONS,
    pub dwAuthenticatorAttachment: u32,
    pub bRequireResidentKey: windows_sys::core::BOOL,
    pub dwUserVerificationRequirement: u32,
    pub dwAttestationConveyancePreference: u32,
    pub dwFlags: u32,
    pub pCancellationId: *mut windows_sys::core::GUID,
    pub pExcludeCredentialList: PWEBAUTHN_CREDENTIAL_LIST,
    pub dwEnterpriseAttestation: u32,
    pub dwLargeBlobSupport: u32,
    pub bPreferResidentKey: windows_sys::core::BOOL,
    pub bBrowserInPrivateMode: windows_sys::core::BOOL,
    pub bEnablePrf: windows_sys::core::BOOL,
    pub pLinkedDevice: PCTAPCBOR_HYBRID_STORAGE_LINKED_DATA,
    pub cbJsonExt: u32,
    pub pbJsonExt: super::PBYTE,
    pub pPRFGlobalEval: PWEBAUTHN_HMAC_SECRET_SALT,
    pub cCredentialHints: u32,
    pub ppwszCredentialHints: *mut windows_sys::core::PCWSTR,
    pub bThirdPartyPayment: windows_sys::core::BOOL,
    pub pwszRemoteWebOrigin: windows_sys::core::PCWSTR,
    pub cbPublicKeyCredentialCreationOptionsJSON: u32,
    pub pbPublicKeyCredentialCreationOptionsJSON: super::PBYTE,
    pub cbAuthenticatorId: u32,
    pub pbAuthenticatorId: super::PBYTE,
}
pub const WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS_CURRENT_VERSION: i32 = 9;
pub const WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS_VERSION_1: i32 = 1;
pub const WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS_VERSION_2: i32 = 2;
pub const WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS_VERSION_3: i32 = 3;
pub const WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS_VERSION_4: i32 = 4;
pub const WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS_VERSION_5: i32 = 5;
pub const WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS_VERSION_6: i32 = 6;
pub const WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS_VERSION_7: i32 = 7;
pub const WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS_VERSION_8: i32 = 8;
pub const WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS_VERSION_9: i32 = 9;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct WEBAUTHN_CLIENT_DATA {
    pub dwVersion: u32,
    pub cbClientDataJSON: u32,
    pub pbClientDataJSON: super::PBYTE,
    pub pwszHashAlgId: windows_sys::core::PCWSTR,
}
pub const WEBAUTHN_CLIENT_DATA_CURRENT_VERSION: i32 = 1;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct WEBAUTHN_COMMON_ATTESTATION {
    pub dwVersion: u32,
    pub pwszAlg: windows_sys::core::PCWSTR,
    pub lAlg: i32,
    pub cbSignature: u32,
    pub pbSignature: super::PBYTE,
    pub cX5c: u32,
    pub pX5c: PWEBAUTHN_X5C,
    pub pwszVer: windows_sys::core::PCWSTR,
    pub cbCertInfo: u32,
    pub pbCertInfo: super::PBYTE,
    pub cbPubArea: u32,
    pub pbPubArea: super::PBYTE,
}
pub const WEBAUTHN_COMMON_ATTESTATION_CURRENT_VERSION: i32 = 1;
pub const WEBAUTHN_COSE_ALGORITHM_ECDSA_P256_WITH_SHA256: i32 = -7;
pub const WEBAUTHN_COSE_ALGORITHM_ECDSA_P384_WITH_SHA384: i32 = -35;
pub const WEBAUTHN_COSE_ALGORITHM_ECDSA_P521_WITH_SHA512: i32 = -36;
pub const WEBAUTHN_COSE_ALGORITHM_RSASSA_PKCS1_V1_5_WITH_SHA256: i32 = -257;
pub const WEBAUTHN_COSE_ALGORITHM_RSASSA_PKCS1_V1_5_WITH_SHA384: i32 = -258;
pub const WEBAUTHN_COSE_ALGORITHM_RSASSA_PKCS1_V1_5_WITH_SHA512: i32 = -259;
pub const WEBAUTHN_COSE_ALGORITHM_RSA_PSS_WITH_SHA256: i32 = -37;
pub const WEBAUTHN_COSE_ALGORITHM_RSA_PSS_WITH_SHA384: i32 = -38;
pub const WEBAUTHN_COSE_ALGORITHM_RSA_PSS_WITH_SHA512: i32 = -39;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WEBAUTHN_COSE_CREDENTIAL_PARAMETER {
    pub dwVersion: u32,
    pub pwszCredentialType: windows_sys::core::PCWSTR,
    pub lAlg: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WEBAUTHN_COSE_CREDENTIAL_PARAMETERS {
    pub cCredentialParameters: u32,
    pub pCredentialParameters: PWEBAUTHN_COSE_CREDENTIAL_PARAMETER,
}
pub const WEBAUTHN_COSE_CREDENTIAL_PARAMETER_CURRENT_VERSION: i32 = 1;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct WEBAUTHN_CREDENTIAL {
    pub dwVersion: u32,
    pub cbId: u32,
    pub pbId: super::PBYTE,
    pub pwszCredentialType: windows_sys::core::PCWSTR,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct WEBAUTHN_CREDENTIALS {
    pub cCredentials: u32,
    pub pCredentials: PWEBAUTHN_CREDENTIAL,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct WEBAUTHN_CREDENTIAL_ATTESTATION {
    pub dwVersion: u32,
    pub pwszFormatType: windows_sys::core::PCWSTR,
    pub cbAuthenticatorData: u32,
    pub pbAuthenticatorData: super::PBYTE,
    pub cbAttestation: u32,
    pub pbAttestation: super::PBYTE,
    pub dwAttestationDecodeType: u32,
    pub pvAttestationDecode: *mut core::ffi::c_void,
    pub cbAttestationObject: u32,
    pub pbAttestationObject: super::PBYTE,
    pub cbCredentialId: u32,
    pub pbCredentialId: super::PBYTE,
    pub Extensions: WEBAUTHN_EXTENSIONS,
    pub dwUsedTransport: u32,
    pub bEpAtt: windows_sys::core::BOOL,
    pub bLargeBlobSupported: windows_sys::core::BOOL,
    pub bResidentKey: windows_sys::core::BOOL,
    pub bPrfEnabled: windows_sys::core::BOOL,
    pub cbUnsignedExtensionOutputs: u32,
    pub pbUnsignedExtensionOutputs: super::PBYTE,
    pub pHmacSecret: PWEBAUTHN_HMAC_SECRET_SALT,
    pub bThirdPartyPayment: windows_sys::core::BOOL,
    pub dwTransports: u32,
    pub cbClientDataJSON: u32,
    pub pbClientDataJSON: super::PBYTE,
    pub cbRegistrationResponseJSON: u32,
    pub pbRegistrationResponseJSON: super::PBYTE,
}
pub const WEBAUTHN_CREDENTIAL_ATTESTATION_CURRENT_VERSION: i32 = 8;
pub const WEBAUTHN_CREDENTIAL_ATTESTATION_VERSION_1: i32 = 1;
pub const WEBAUTHN_CREDENTIAL_ATTESTATION_VERSION_2: i32 = 2;
pub const WEBAUTHN_CREDENTIAL_ATTESTATION_VERSION_3: i32 = 3;
pub const WEBAUTHN_CREDENTIAL_ATTESTATION_VERSION_4: i32 = 4;
pub const WEBAUTHN_CREDENTIAL_ATTESTATION_VERSION_5: i32 = 5;
pub const WEBAUTHN_CREDENTIAL_ATTESTATION_VERSION_6: i32 = 6;
pub const WEBAUTHN_CREDENTIAL_ATTESTATION_VERSION_7: i32 = 7;
pub const WEBAUTHN_CREDENTIAL_ATTESTATION_VERSION_8: i32 = 8;
pub const WEBAUTHN_CREDENTIAL_CURRENT_VERSION: i32 = 1;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct WEBAUTHN_CREDENTIAL_DETAILS {
    pub dwVersion: u32,
    pub cbCredentialID: u32,
    pub pbCredentialID: super::PBYTE,
    pub pRpInformation: PWEBAUTHN_RP_ENTITY_INFORMATION,
    pub pUserInformation: PWEBAUTHN_USER_ENTITY_INFORMATION,
    pub bRemovable: windows_sys::core::BOOL,
    pub bBackedUp: windows_sys::core::BOOL,
    pub pwszAuthenticatorName: windows_sys::core::PCWSTR,
    pub cbAuthenticatorLogo: u32,
    pub pbAuthenticatorLogo: super::PBYTE,
    pub bThirdPartyPayment: windows_sys::core::BOOL,
    pub dwTransports: u32,
}
pub const WEBAUTHN_CREDENTIAL_DETAILS_CURRENT_VERSION: i32 = 4;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct WEBAUTHN_CREDENTIAL_DETAILS_LIST {
    pub cCredentialDetails: u32,
    pub ppCredentialDetails: *mut PWEBAUTHN_CREDENTIAL_DETAILS,
}
pub const WEBAUTHN_CREDENTIAL_DETAILS_VERSION_1: i32 = 1;
pub const WEBAUTHN_CREDENTIAL_DETAILS_VERSION_2: i32 = 2;
pub const WEBAUTHN_CREDENTIAL_DETAILS_VERSION_3: i32 = 3;
pub const WEBAUTHN_CREDENTIAL_DETAILS_VERSION_4: i32 = 4;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct WEBAUTHN_CREDENTIAL_EX {
    pub dwVersion: u32,
    pub cbId: u32,
    pub pbId: super::PBYTE,
    pub pwszCredentialType: windows_sys::core::PCWSTR,
    pub dwTransports: u32,
}
pub const WEBAUTHN_CREDENTIAL_EX_CURRENT_VERSION: i32 = 1;
pub const WEBAUTHN_CREDENTIAL_HINT_CLIENT_DEVICE: windows_sys::core::PCWSTR = windows_sys::core::w!("client-device");
pub const WEBAUTHN_CREDENTIAL_HINT_HYBRID: windows_sys::core::PCWSTR = windows_sys::core::w!("hybrid");
pub const WEBAUTHN_CREDENTIAL_HINT_SECURITY_KEY: windows_sys::core::PCWSTR = windows_sys::core::w!("security-key");
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct WEBAUTHN_CREDENTIAL_LIST {
    pub cCredentials: u32,
    pub ppCredentials: *mut PWEBAUTHN_CREDENTIAL_EX,
}
pub const WEBAUTHN_CREDENTIAL_TYPE_PUBLIC_KEY: windows_sys::core::PCWSTR = windows_sys::core::w!("public-key");
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct WEBAUTHN_CRED_BLOB_EXTENSION {
    pub cbCredBlob: u32,
    pub pbCredBlob: super::PBYTE,
}
pub const WEBAUTHN_CRED_LARGE_BLOB_OPERATION_DELETE: i32 = 3;
pub const WEBAUTHN_CRED_LARGE_BLOB_OPERATION_GET: i32 = 1;
pub const WEBAUTHN_CRED_LARGE_BLOB_OPERATION_NONE: i32 = 0;
pub const WEBAUTHN_CRED_LARGE_BLOB_OPERATION_SET: i32 = 2;
pub const WEBAUTHN_CRED_LARGE_BLOB_STATUS_AUTHENTICATOR_ERROR: i32 = 9;
pub const WEBAUTHN_CRED_LARGE_BLOB_STATUS_INVALID_DATA: i32 = 3;
pub const WEBAUTHN_CRED_LARGE_BLOB_STATUS_INVALID_PARAMETER: i32 = 4;
pub const WEBAUTHN_CRED_LARGE_BLOB_STATUS_LACK_OF_SPACE: i32 = 7;
pub const WEBAUTHN_CRED_LARGE_BLOB_STATUS_MULTIPLE_CREDENTIALS: i32 = 6;
pub const WEBAUTHN_CRED_LARGE_BLOB_STATUS_NONE: i32 = 0;
pub const WEBAUTHN_CRED_LARGE_BLOB_STATUS_NOT_FOUND: i32 = 5;
pub const WEBAUTHN_CRED_LARGE_BLOB_STATUS_NOT_SUPPORTED: i32 = 2;
pub const WEBAUTHN_CRED_LARGE_BLOB_STATUS_PLATFORM_ERROR: i32 = 8;
pub const WEBAUTHN_CRED_LARGE_BLOB_STATUS_SUCCESS: i32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WEBAUTHN_CRED_PROTECT_EXTENSION_IN {
    pub dwCredProtect: u32,
    pub bRequireCredProtect: windows_sys::core::BOOL,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct WEBAUTHN_CRED_WITH_HMAC_SECRET_SALT {
    pub cbCredID: u32,
    pub pbCredID: super::PBYTE,
    pub pHmacSecretSalt: PWEBAUTHN_HMAC_SECRET_SALT,
}
pub const WEBAUTHN_CTAP_ONE_HMAC_SECRET_LENGTH: i32 = 32;
pub const WEBAUTHN_CTAP_TRANSPORT_BLE: i32 = 4;
pub const WEBAUTHN_CTAP_TRANSPORT_BLE_STRING: windows_sys::core::PCSTR = windows_sys::core::s!("ble");
pub const WEBAUTHN_CTAP_TRANSPORT_FLAGS_MASK: i32 = 127;
pub const WEBAUTHN_CTAP_TRANSPORT_HYBRID: i32 = 32;
pub const WEBAUTHN_CTAP_TRANSPORT_HYBRID_STRING: windows_sys::core::PCSTR = windows_sys::core::s!("hybrid");
pub const WEBAUTHN_CTAP_TRANSPORT_INTERNAL: i32 = 16;
pub const WEBAUTHN_CTAP_TRANSPORT_INTERNAL_STRING: windows_sys::core::PCSTR = windows_sys::core::s!("internal");
pub const WEBAUTHN_CTAP_TRANSPORT_NFC: i32 = 2;
pub const WEBAUTHN_CTAP_TRANSPORT_NFC_STRING: windows_sys::core::PCSTR = windows_sys::core::s!("nfc");
pub const WEBAUTHN_CTAP_TRANSPORT_SMART_CARD: i32 = 64;
pub const WEBAUTHN_CTAP_TRANSPORT_SMART_CARD_STRING: windows_sys::core::PCSTR = windows_sys::core::s!("smart-card");
pub const WEBAUTHN_CTAP_TRANSPORT_TEST: i32 = 8;
pub const WEBAUTHN_CTAP_TRANSPORT_USB: i32 = 1;
pub const WEBAUTHN_CTAP_TRANSPORT_USB_STRING: windows_sys::core::PCSTR = windows_sys::core::s!("usb");
pub const WEBAUTHN_ENTERPRISE_ATTESTATION_NONE: i32 = 0;
pub const WEBAUTHN_ENTERPRISE_ATTESTATION_PLATFORM_MANAGED: i32 = 2;
pub const WEBAUTHN_ENTERPRISE_ATTESTATION_VENDOR_FACILITATED: i32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WEBAUTHN_EXTENSION {
    pub pwszExtensionIdentifier: windows_sys::core::PCWSTR,
    pub cbExtension: u32,
    pub pvExtension: *mut core::ffi::c_void,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WEBAUTHN_EXTENSIONS {
    pub cExtensions: u32,
    pub pExtensions: PWEBAUTHN_EXTENSION,
}
pub const WEBAUTHN_EXTENSIONS_IDENTIFIER_CRED_BLOB: windows_sys::core::PCWSTR = windows_sys::core::w!("credBlob");
pub const WEBAUTHN_EXTENSIONS_IDENTIFIER_CRED_PROTECT: windows_sys::core::PCWSTR = windows_sys::core::w!("credProtect");
pub const WEBAUTHN_EXTENSIONS_IDENTIFIER_HMAC_SECRET: windows_sys::core::PCWSTR = windows_sys::core::w!("hmac-secret");
pub const WEBAUTHN_EXTENSIONS_IDENTIFIER_MIN_PIN_LENGTH: windows_sys::core::PCWSTR = windows_sys::core::w!("minPinLength");
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WEBAUTHN_GET_CREDENTIALS_OPTIONS {
    pub dwVersion: u32,
    pub pwszRpId: windows_sys::core::PCWSTR,
    pub bBrowserInPrivateMode: windows_sys::core::BOOL,
}
pub const WEBAUTHN_GET_CREDENTIALS_OPTIONS_CURRENT_VERSION: i32 = 1;
pub const WEBAUTHN_GET_CREDENTIALS_OPTIONS_VERSION_1: i32 = 1;
pub const WEBAUTHN_HASH_ALGORITHM_SHA_256: windows_sys::core::PCWSTR = windows_sys::core::w!("SHA-256");
pub const WEBAUTHN_HASH_ALGORITHM_SHA_384: windows_sys::core::PCWSTR = windows_sys::core::w!("SHA-384");
pub const WEBAUTHN_HASH_ALGORITHM_SHA_512: windows_sys::core::PCWSTR = windows_sys::core::w!("SHA-512");
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct WEBAUTHN_HMAC_SECRET_SALT {
    pub cbFirst: u32,
    pub pbFirst: super::PBYTE,
    pub cbSecond: u32,
    pub pbSecond: super::PBYTE,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct WEBAUTHN_HMAC_SECRET_SALT_VALUES {
    pub pGlobalHmacSalt: PWEBAUTHN_HMAC_SECRET_SALT,
    pub cCredWithHmacSecretSaltList: u32,
    pub pCredWithHmacSecretSaltList: PWEBAUTHN_CRED_WITH_HMAC_SECRET_SALT,
}
pub const WEBAUTHN_LARGE_BLOB_SUPPORT_NONE: i32 = 0;
pub const WEBAUTHN_LARGE_BLOB_SUPPORT_PREFERRED: i32 = 2;
pub const WEBAUTHN_LARGE_BLOB_SUPPORT_REQUIRED: i32 = 1;
pub const WEBAUTHN_MAX_USER_ID_LENGTH: i32 = 64;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WEBAUTHN_RP_ENTITY_INFORMATION {
    pub dwVersion: u32,
    pub pwszId: windows_sys::core::PCWSTR,
    pub pwszName: windows_sys::core::PCWSTR,
    pub pwszIcon: windows_sys::core::PCWSTR,
}
pub const WEBAUTHN_RP_ENTITY_INFORMATION_CURRENT_VERSION: i32 = 1;
pub const WEBAUTHN_RP_ENTITY_INFORMATION_VERSION_1: i32 = 1;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct WEBAUTHN_USER_ENTITY_INFORMATION {
    pub dwVersion: u32,
    pub cbId: u32,
    pub pbId: super::PBYTE,
    pub pwszName: windows_sys::core::PCWSTR,
    pub pwszIcon: windows_sys::core::PCWSTR,
    pub pwszDisplayName: windows_sys::core::PCWSTR,
}
pub const WEBAUTHN_USER_ENTITY_INFORMATION_CURRENT_VERSION: i32 = 1;
pub const WEBAUTHN_USER_ENTITY_INFORMATION_VERSION_1: i32 = 1;
pub const WEBAUTHN_USER_VERIFICATION_ANY: i32 = 0;
pub const WEBAUTHN_USER_VERIFICATION_OPTIONAL: i32 = 1;
pub const WEBAUTHN_USER_VERIFICATION_OPTIONAL_WITH_CREDENTIAL_ID_LIST: i32 = 2;
pub const WEBAUTHN_USER_VERIFICATION_REQUIRED: i32 = 3;
pub const WEBAUTHN_USER_VERIFICATION_REQUIREMENT_ANY: i32 = 0;
pub const WEBAUTHN_USER_VERIFICATION_REQUIREMENT_DISCOURAGED: i32 = 3;
pub const WEBAUTHN_USER_VERIFICATION_REQUIREMENT_PREFERRED: i32 = 2;
pub const WEBAUTHN_USER_VERIFICATION_REQUIREMENT_REQUIRED: i32 = 1;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct WEBAUTHN_X5C {
    pub cbData: u32,
    pub pbData: super::PBYTE,
}
