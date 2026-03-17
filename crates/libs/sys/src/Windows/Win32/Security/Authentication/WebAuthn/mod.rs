windows_link::link!("webauthn.dll" "system" fn WebAuthNAuthenticatorGetAssertion(hwnd : super::super::super::Foundation:: HWND, pwszrpid : windows_sys::core::PCWSTR, pwebauthnclientdata : *const WEBAUTHN_CLIENT_DATA, pwebauthngetassertionoptions : *const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS, ppwebauthnassertion : *mut *mut WEBAUTHN_ASSERTION) -> windows_sys::core::HRESULT);
windows_link::link!("webauthn.dll" "system" fn WebAuthNAuthenticatorMakeCredential(hwnd : super::super::super::Foundation:: HWND, prpinformation : *const WEBAUTHN_RP_ENTITY_INFORMATION, puserinformation : *const WEBAUTHN_USER_ENTITY_INFORMATION, ppubkeycredparams : *const WEBAUTHN_COSE_CREDENTIAL_PARAMETERS, pwebauthnclientdata : *const WEBAUTHN_CLIENT_DATA, pwebauthnmakecredentialoptions : *const WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS, ppwebauthncredentialattestation : *mut *mut WEBAUTHN_CREDENTIAL_ATTESTATION) -> windows_sys::core::HRESULT);
windows_link::link!("webauthn.dll" "system" fn WebAuthNCancelCurrentOperation(pcancellationid : *const windows_sys::core::GUID) -> windows_sys::core::HRESULT);
windows_link::link!("webauthn.dll" "system" fn WebAuthNDeletePlatformCredential(cbcredentialid : u32, pbcredentialid : *const u8) -> windows_sys::core::HRESULT);
windows_link::link!("webauthn.dll" "system" fn WebAuthNFreeAssertion(pwebauthnassertion : *const WEBAUTHN_ASSERTION));
windows_link::link!("webauthn.dll" "system" fn WebAuthNFreeAuthenticatorList(pauthenticatordetailslist : *const WEBAUTHN_AUTHENTICATOR_DETAILS_LIST));
windows_link::link!("webauthn.dll" "system" fn WebAuthNFreeCredentialAttestation(pwebauthncredentialattestation : *const WEBAUTHN_CREDENTIAL_ATTESTATION));
windows_link::link!("webauthn.dll" "system" fn WebAuthNFreePlatformCredentialList(pcredentialdetailslist : *const WEBAUTHN_CREDENTIAL_DETAILS_LIST));
windows_link::link!("webauthn.dll" "system" fn WebAuthNGetApiVersionNumber() -> u32);
windows_link::link!("webauthn.dll" "system" fn WebAuthNGetAuthenticatorList(pwebauthngetauthenticatorlistoptions : *const WEBAUTHN_AUTHENTICATOR_DETAILS_OPTIONS, ppauthenticatordetailslist : *mut *mut WEBAUTHN_AUTHENTICATOR_DETAILS_LIST) -> windows_sys::core::HRESULT);
windows_link::link!("webauthn.dll" "system" fn WebAuthNGetCancellationId(pcancellationid : *mut windows_sys::core::GUID) -> windows_sys::core::HRESULT);
windows_link::link!("webauthn.dll" "system" fn WebAuthNGetErrorName(hr : windows_sys::core::HRESULT) -> windows_sys::core::PCWSTR);
windows_link::link!("webauthn.dll" "system" fn WebAuthNGetPlatformCredentialList(pgetcredentialsoptions : *const WEBAUTHN_GET_CREDENTIALS_OPTIONS, ppcredentialdetailslist : *mut *mut WEBAUTHN_CREDENTIAL_DETAILS_LIST) -> windows_sys::core::HRESULT);
windows_link::link!("webauthn.dll" "system" fn WebAuthNGetW3CExceptionDOMError(hr : windows_sys::core::HRESULT) -> windows_sys::core::HRESULT);
windows_link::link!("webauthn.dll" "system" fn WebAuthNIsUserVerifyingPlatformAuthenticatorAvailable(pbisuserverifyingplatformauthenticatoravailable : *mut windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
pub type AUTHENTICATOR_STATE = i32;
pub const AuthenticatorState_Disabled: AUTHENTICATOR_STATE = 0i32;
pub const AuthenticatorState_Enabled: AUTHENTICATOR_STATE = 1i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CTAPCBOR_HYBRID_STORAGE_LINKED_DATA {
    pub dwVersion: u32,
    pub cbContactId: u32,
    pub pbContactId: *mut u8,
    pub cbLinkId: u32,
    pub pbLinkId: *mut u8,
    pub cbLinkSecret: u32,
    pub pbLinkSecret: *mut u8,
    pub cbPublicKey: u32,
    pub pbPublicKey: *mut u8,
    pub pwszAuthenticatorName: windows_sys::core::PCWSTR,
    pub wEncodedTunnelServerDomain: u16,
}
impl Default for CTAPCBOR_HYBRID_STORAGE_LINKED_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CTAPCBOR_HYBRID_STORAGE_LINKED_DATA_CURRENT_VERSION: u32 = 1u32;
pub const CTAPCBOR_HYBRID_STORAGE_LINKED_DATA_VERSION_1: u32 = 1u32;
pub type EXPERIMENTAL_PLUGIN_AUTHENTICATOR_STATE = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct EXPERIMENTAL_WEBAUTHN_CTAPCBOR_AUTHENTICATOR_OPTIONS {
    pub dwVersion: u32,
    pub lUp: i32,
    pub lUv: i32,
    pub lRequireResidentKey: i32,
}
pub const EXPERIMENTAL_WEBAUTHN_CTAPCBOR_AUTHENTICATOR_OPTIONS_CURRENT_VERSION: u32 = 1u32;
pub const EXPERIMENTAL_WEBAUTHN_CTAPCBOR_AUTHENTICATOR_OPTIONS_VERSION_1: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EXPERIMENTAL_WEBAUTHN_CTAPCBOR_ECC_PUBLIC_KEY {
    pub dwVersion: u32,
    pub lKty: i32,
    pub lAlg: i32,
    pub lCrv: i32,
    pub cbX: u32,
    pub pbX: *mut u8,
    pub cbY: u32,
    pub pbY: *mut u8,
}
impl Default for EXPERIMENTAL_WEBAUTHN_CTAPCBOR_ECC_PUBLIC_KEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const EXPERIMENTAL_WEBAUTHN_CTAPCBOR_ECC_PUBLIC_KEY_CURRENT_VERSION: u32 = 1u32;
pub const EXPERIMENTAL_WEBAUTHN_CTAPCBOR_ECC_PUBLIC_KEY_VERSION_1: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EXPERIMENTAL_WEBAUTHN_CTAPCBOR_GET_ASSERTION_REQUEST {
    pub dwVersion: u32,
    pub pwszRpId: windows_sys::core::PCWSTR,
    pub cbRpId: u32,
    pub pbRpId: *mut u8,
    pub cbClientDataHash: u32,
    pub pbClientDataHash: *mut u8,
    pub CredentialList: WEBAUTHN_CREDENTIAL_LIST,
    pub cbCborExtensionsMap: u32,
    pub pbCborExtensionsMap: *mut u8,
    pub pAuthenticatorOptions: *mut EXPERIMENTAL_WEBAUTHN_CTAPCBOR_AUTHENTICATOR_OPTIONS,
    pub fEmptyPinAuth: windows_sys::core::BOOL,
    pub cbPinAuth: u32,
    pub pbPinAuth: *mut u8,
    pub pHmacSaltExtension: *mut EXPERIMENTAL_WEBAUTHN_CTAPCBOR_HMAC_SALT_EXTENSION,
    pub cbHmacSecretSaltValues: u32,
    pub pbHmacSecretSaltValues: *mut u8,
    pub dwPinProtocol: u32,
    pub lCredBlobExt: i32,
    pub lLargeBlobKeyExt: i32,
    pub dwCredLargeBlobOperation: u32,
    pub cbCredLargeBlobCompressed: u32,
    pub pbCredLargeBlobCompressed: *mut u8,
    pub dwCredLargeBlobOriginalSize: u32,
    pub cbJsonExt: u32,
    pub pbJsonExt: *mut u8,
}
impl Default for EXPERIMENTAL_WEBAUTHN_CTAPCBOR_GET_ASSERTION_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const EXPERIMENTAL_WEBAUTHN_CTAPCBOR_GET_ASSERTION_REQUEST_CURRENT_VERSION: u32 = 1u32;
pub const EXPERIMENTAL_WEBAUTHN_CTAPCBOR_GET_ASSERTION_REQUEST_VERSION_1: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EXPERIMENTAL_WEBAUTHN_CTAPCBOR_GET_ASSERTION_RESPONSE {
    pub WebAuthNAssertion: WEBAUTHN_ASSERTION,
    pub pUserInformation: *mut WEBAUTHN_USER_ENTITY_INFORMATION,
    pub dwNumberOfCredentials: u32,
    pub lUserSelected: i32,
    pub cbLargeBlobKey: u32,
    pub pbLargeBlobKey: *mut u8,
    pub cbUnsignedExtensionOutputs: u32,
    pub pbUnsignedExtensionOutputs: *mut u8,
}
impl Default for EXPERIMENTAL_WEBAUTHN_CTAPCBOR_GET_ASSERTION_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EXPERIMENTAL_WEBAUTHN_CTAPCBOR_HMAC_SALT_EXTENSION {
    pub dwVersion: u32,
    pub pKeyAgreement: *mut EXPERIMENTAL_WEBAUTHN_CTAPCBOR_ECC_PUBLIC_KEY,
    pub cbEncryptedSalt: u32,
    pub pbEncryptedSalt: *mut u8,
    pub cbSaltAuth: u32,
    pub pbSaltAuth: *mut u8,
}
impl Default for EXPERIMENTAL_WEBAUTHN_CTAPCBOR_HMAC_SALT_EXTENSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const EXPERIMENTAL_WEBAUTHN_CTAPCBOR_HMAC_SALT_EXTENSION_CURRENT_VERSION: u32 = 1u32;
pub const EXPERIMENTAL_WEBAUTHN_CTAPCBOR_HMAC_SALT_EXTENSION_VERSION_1: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EXPERIMENTAL_WEBAUTHN_CTAPCBOR_MAKE_CREDENTIAL_REQUEST {
    pub dwVersion: u32,
    pub cbRpId: u32,
    pub pbRpId: *mut u8,
    pub cbClientDataHash: u32,
    pub pbClientDataHash: *mut u8,
    pub pRpInformation: *mut WEBAUTHN_RP_ENTITY_INFORMATION,
    pub pUserInformation: *mut WEBAUTHN_USER_ENTITY_INFORMATION,
    pub WebAuthNCredentialParameters: WEBAUTHN_COSE_CREDENTIAL_PARAMETERS,
    pub CredentialList: WEBAUTHN_CREDENTIAL_LIST,
    pub cbCborExtensionsMap: u32,
    pub pbCborExtensionsMap: *mut u8,
    pub pAuthenticatorOptions: *mut EXPERIMENTAL_WEBAUTHN_CTAPCBOR_AUTHENTICATOR_OPTIONS,
    pub fEmptyPinAuth: windows_sys::core::BOOL,
    pub cbPinAuth: u32,
    pub pbPinAuth: *mut u8,
    pub lHmacSecretExt: i32,
    pub pHmacSecretMcExtension: *mut EXPERIMENTAL_WEBAUTHN_CTAPCBOR_HMAC_SALT_EXTENSION,
    pub lPrfExt: i32,
    pub cbHmacSecretSaltValues: u32,
    pub pbHmacSecretSaltValues: *mut u8,
    pub dwCredProtect: u32,
    pub dwPinProtocol: u32,
    pub dwEnterpriseAttestation: u32,
    pub cbCredBlobExt: u32,
    pub pbCredBlobExt: *mut u8,
    pub lLargeBlobKeyExt: i32,
    pub dwLargeBlobSupport: u32,
    pub lMinPinLengthExt: i32,
    pub cbJsonExt: u32,
    pub pbJsonExt: *mut u8,
}
impl Default for EXPERIMENTAL_WEBAUTHN_CTAPCBOR_MAKE_CREDENTIAL_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const EXPERIMENTAL_WEBAUTHN_CTAPCBOR_MAKE_CREDENTIAL_REQUEST_CURRENT_VERSION: u32 = 1u32;
pub const EXPERIMENTAL_WEBAUTHN_CTAPCBOR_MAKE_CREDENTIAL_REQUEST_VERSION_1: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EXPERIMENTAL_WEBAUTHN_PLUGIN_ADD_AUTHENTICATOR_OPTIONS {
    pub pwszAuthenticatorName: windows_sys::core::PCWSTR,
    pub pwszPluginClsId: windows_sys::core::PCWSTR,
    pub pwszPluginRpId: windows_sys::core::PCWSTR,
    pub pwszLightThemeLogo: windows_sys::core::PCWSTR,
    pub pwszDarkThemeLogo: windows_sys::core::PCWSTR,
    pub cbAuthenticatorInfo: u32,
    pub pbAuthenticatorInfo: *mut u8,
}
impl Default for EXPERIMENTAL_WEBAUTHN_PLUGIN_ADD_AUTHENTICATOR_OPTIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EXPERIMENTAL_WEBAUTHN_PLUGIN_ADD_AUTHENTICATOR_OPTIONS_2 {
    pub pwszAuthenticatorName: windows_sys::core::PCWSTR,
    pub pClsid: *const windows_sys::core::GUID,
    pub pwszPluginRpId: windows_sys::core::PCWSTR,
    pub pwszLightThemeLogoSvg: windows_sys::core::PCWSTR,
    pub pwszDarkThemeLogoSvg: windows_sys::core::PCWSTR,
    pub cbAuthenticatorInfo: u32,
    pub pbAuthenticatorInfo: *const u8,
    pub cSupportedRpIds: u32,
    pub ppwszSupportedRpIds: *const windows_sys::core::PCWSTR,
    pub pwszUserVerificationKeyName: windows_sys::core::PCWSTR,
}
impl Default for EXPERIMENTAL_WEBAUTHN_PLUGIN_ADD_AUTHENTICATOR_OPTIONS_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EXPERIMENTAL_WEBAUTHN_PLUGIN_ADD_AUTHENTICATOR_RESPONSE {
    pub cbOpSignPubKey: u32,
    pub pbOpSignPubKey: *mut u8,
}
impl Default for EXPERIMENTAL_WEBAUTHN_PLUGIN_ADD_AUTHENTICATOR_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EXPERIMENTAL_WEBAUTHN_PLUGIN_CANCEL_OPERATION_REQUEST {
    pub transactionId: windows_sys::core::GUID,
    pub cbRequestSignature: u32,
    pub pbRequestSignature: *mut u8,
}
impl Default for EXPERIMENTAL_WEBAUTHN_PLUGIN_CANCEL_OPERATION_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EXPERIMENTAL_WEBAUTHN_PLUGIN_CREDENTIAL_DETAILS {
    pub cbCredentialId: u32,
    pub pbCredentialId: *mut u8,
    pub pwszRpId: windows_sys::core::PWSTR,
    pub pwszRpName: windows_sys::core::PWSTR,
    pub cbUserId: u32,
    pub pbUserId: *mut u8,
    pub pwszUserName: windows_sys::core::PWSTR,
    pub pwszUserDisplayName: windows_sys::core::PWSTR,
}
impl Default for EXPERIMENTAL_WEBAUTHN_PLUGIN_CREDENTIAL_DETAILS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EXPERIMENTAL_WEBAUTHN_PLUGIN_CREDENTIAL_DETAILS_LIST {
    pub pwszPluginClsId: windows_sys::core::PWSTR,
    pub cCredentialDetails: u32,
    pub pCredentialDetails: *mut *mut EXPERIMENTAL_WEBAUTHN_PLUGIN_CREDENTIAL_DETAILS,
}
impl Default for EXPERIMENTAL_WEBAUTHN_PLUGIN_CREDENTIAL_DETAILS_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EXPERIMENTAL_WEBAUTHN_PLUGIN_OPERATION_REQUEST {
    pub hWnd: super::super::super::Foundation::HWND,
    pub transactionId: windows_sys::core::GUID,
    pub cbRequestSignature: u32,
    pub pbRequestSignature: *mut u8,
    pub cbEncodedRequest: u32,
    pub pbEncodedRequest: *mut u8,
}
impl Default for EXPERIMENTAL_WEBAUTHN_PLUGIN_OPERATION_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EXPERIMENTAL_WEBAUTHN_PLUGIN_OPERATION_RESPONSE {
    pub cbEncodedResponse: u32,
    pub pbEncodedResponse: *mut u8,
}
impl Default for EXPERIMENTAL_WEBAUTHN_PLUGIN_OPERATION_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EXPERIMENTAL_WEBAUTHN_PLUGIN_PERFORM_UV {
    pub hwnd: super::super::super::Foundation::HWND,
    pub transactionId: *mut windows_sys::core::GUID,
    pub r#type: EXPERIMENTAL_WEBAUTHN_PLUGIN_PERFORM_UV_OPERATION_TYPE,
    pub pwszUsername: windows_sys::core::PCWSTR,
    pub pwszContext: windows_sys::core::PCWSTR,
}
impl Default for EXPERIMENTAL_WEBAUTHN_PLUGIN_PERFORM_UV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type EXPERIMENTAL_WEBAUTHN_PLUGIN_PERFORM_UV_OPERATION_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EXPERIMENTAL_WEBAUTHN_PLUGIN_PERFORM_UV_RESPONSE {
    pub cbResponse: u32,
    pub pbResponse: *mut u8,
}
impl Default for EXPERIMENTAL_WEBAUTHN_PLUGIN_PERFORM_UV_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type EXPERIMENTAL_WEBAUTHN_PLUGIN_STATUS_CHANGE_CALLBACK = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void)>;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EXPERIMENTAL_WEBAUTHN_PLUGIN_UPDATE_AUTHENTICATOR_DETAILS {
    pub pwszAuthenticatorName: windows_sys::core::PCWSTR,
    pub pwszPluginClsId: windows_sys::core::PCWSTR,
    pub pwszNewPluginClsId: windows_sys::core::PCWSTR,
    pub pwszLightThemeLogo: windows_sys::core::PCWSTR,
    pub pwszDarkThemeLogo: windows_sys::core::PCWSTR,
    pub cbAuthenticatorInfo: u32,
    pub pbAuthenticatorInfo: *mut u8,
}
impl Default for EXPERIMENTAL_WEBAUTHN_PLUGIN_UPDATE_AUTHENTICATOR_DETAILS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EXPERIMENTAL_WEBAUTHN_PLUGIN_UPDATE_AUTHENTICATOR_DETAILS_2 {
    pub pwszAuthenticatorName: windows_sys::core::PCWSTR,
    pub pClsid: *const windows_sys::core::GUID,
    pub pClsidNew: *const windows_sys::core::GUID,
    pub pwszLightThemeLogoSvg: windows_sys::core::PCWSTR,
    pub pwszDarkThemeLogoSvg: windows_sys::core::PCWSTR,
    pub cbAuthenticatorInfo: u32,
    pub pbAuthenticatorInfo: *const u8,
    pub cSupportedRpIds: u32,
    pub ppwszSupportedRpIds: *const windows_sys::core::PCWSTR,
    pub pwszUserVerificationKeyName: windows_sys::core::PCWSTR,
}
impl Default for EXPERIMENTAL_WEBAUTHN_PLUGIN_UPDATE_AUTHENTICATOR_DETAILS_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EXPERIMENTAL_WEBAUTHN_PLUGIN_USER_VERIFICATION_REQUEST_2 {
    pub hwnd: super::super::super::Foundation::HWND,
    pub pGuidTransactionId: *const windows_sys::core::GUID,
    pub pwszUsername: windows_sys::core::PCWSTR,
    pub pwszDisplayHint: windows_sys::core::PCWSTR,
    pub cbBufferToSign: u32,
    pub pbBufferToSign: *mut u8,
}
impl Default for EXPERIMENTAL_WEBAUTHN_PLUGIN_USER_VERIFICATION_REQUEST_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const GetPubKey: EXPERIMENTAL_WEBAUTHN_PLUGIN_PERFORM_UV_OPERATION_TYPE = 3i32;
pub const GetPublicKey: WEBAUTHN_PLUGIN_PERFORM_UV_OPERATION_TYPE = 3i32;
pub const GetUserVerificationCount: WEBAUTHN_PLUGIN_PERFORM_UV_OPERATION_TYPE = 2i32;
pub const GetUvCount: EXPERIMENTAL_WEBAUTHN_PLUGIN_PERFORM_UV_OPERATION_TYPE = 2i32;
pub type PLUGIN_LOCK_STATUS = i32;
pub const PerformUserVerification: WEBAUTHN_PLUGIN_PERFORM_UV_OPERATION_TYPE = 1i32;
pub const PerformUv: EXPERIMENTAL_WEBAUTHN_PLUGIN_PERFORM_UV_OPERATION_TYPE = 1i32;
pub const PluginAuthenticatorState_Disabled: EXPERIMENTAL_PLUGIN_AUTHENTICATOR_STATE = 1i32;
pub const PluginAuthenticatorState_Enabled: EXPERIMENTAL_PLUGIN_AUTHENTICATOR_STATE = 2i32;
pub const PluginAuthenticatorState_Unknown: EXPERIMENTAL_PLUGIN_AUTHENTICATOR_STATE = 0i32;
pub const PluginLocked: PLUGIN_LOCK_STATUS = 0i32;
pub const PluginUnlocked: PLUGIN_LOCK_STATUS = 1i32;
pub const WEBAUTHN_API_CURRENT_VERSION: u32 = 9u32;
pub const WEBAUTHN_API_VERSION_1: u32 = 1u32;
pub const WEBAUTHN_API_VERSION_2: u32 = 2u32;
pub const WEBAUTHN_API_VERSION_3: u32 = 3u32;
pub const WEBAUTHN_API_VERSION_4: u32 = 4u32;
pub const WEBAUTHN_API_VERSION_5: u32 = 5u32;
pub const WEBAUTHN_API_VERSION_6: u32 = 6u32;
pub const WEBAUTHN_API_VERSION_7: u32 = 7u32;
pub const WEBAUTHN_API_VERSION_8: u32 = 8u32;
pub const WEBAUTHN_API_VERSION_9: u32 = 9u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WEBAUTHN_ASSERTION {
    pub dwVersion: u32,
    pub cbAuthenticatorData: u32,
    pub pbAuthenticatorData: *mut u8,
    pub cbSignature: u32,
    pub pbSignature: *mut u8,
    pub Credential: WEBAUTHN_CREDENTIAL,
    pub cbUserId: u32,
    pub pbUserId: *mut u8,
    pub Extensions: WEBAUTHN_EXTENSIONS,
    pub cbCredLargeBlob: u32,
    pub pbCredLargeBlob: *mut u8,
    pub dwCredLargeBlobStatus: u32,
    pub pHmacSecret: *mut WEBAUTHN_HMAC_SECRET_SALT,
    pub dwUsedTransport: u32,
    pub cbUnsignedExtensionOutputs: u32,
    pub pbUnsignedExtensionOutputs: *mut u8,
    pub cbClientDataJSON: u32,
    pub pbClientDataJSON: *mut u8,
    pub cbAuthenticationResponseJSON: u32,
    pub pbAuthenticationResponseJSON: *mut u8,
}
impl Default for WEBAUTHN_ASSERTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WEBAUTHN_ASSERTION_CURRENT_VERSION: u32 = 6u32;
pub const WEBAUTHN_ASSERTION_VERSION_1: u32 = 1u32;
pub const WEBAUTHN_ASSERTION_VERSION_2: u32 = 2u32;
pub const WEBAUTHN_ASSERTION_VERSION_3: u32 = 3u32;
pub const WEBAUTHN_ASSERTION_VERSION_4: u32 = 4u32;
pub const WEBAUTHN_ASSERTION_VERSION_5: u32 = 5u32;
pub const WEBAUTHN_ASSERTION_VERSION_6: u32 = 6u32;
pub const WEBAUTHN_ATTESTATION_CONVEYANCE_PREFERENCE_ANY: u32 = 0u32;
pub const WEBAUTHN_ATTESTATION_CONVEYANCE_PREFERENCE_DIRECT: u32 = 3u32;
pub const WEBAUTHN_ATTESTATION_CONVEYANCE_PREFERENCE_INDIRECT: u32 = 2u32;
pub const WEBAUTHN_ATTESTATION_CONVEYANCE_PREFERENCE_NONE: u32 = 1u32;
pub const WEBAUTHN_ATTESTATION_DECODE_COMMON: u32 = 1u32;
pub const WEBAUTHN_ATTESTATION_DECODE_NONE: u32 = 0u32;
pub const WEBAUTHN_ATTESTATION_TYPE_NONE: windows_sys::core::PCWSTR = windows_sys::core::w!("none");
pub const WEBAUTHN_ATTESTATION_TYPE_PACKED: windows_sys::core::PCWSTR = windows_sys::core::w!("packed");
pub const WEBAUTHN_ATTESTATION_TYPE_TPM: windows_sys::core::PCWSTR = windows_sys::core::w!("tpm");
pub const WEBAUTHN_ATTESTATION_TYPE_U2F: windows_sys::core::PCWSTR = windows_sys::core::w!("fido-u2f");
pub const WEBAUTHN_ATTESTATION_VER_TPM_2_0: windows_sys::core::PCWSTR = windows_sys::core::w!("2.0");
pub const WEBAUTHN_AUTHENTICATOR_ATTACHMENT_ANY: u32 = 0u32;
pub const WEBAUTHN_AUTHENTICATOR_ATTACHMENT_CROSS_PLATFORM: u32 = 2u32;
pub const WEBAUTHN_AUTHENTICATOR_ATTACHMENT_CROSS_PLATFORM_U2F_V2: u32 = 3u32;
pub const WEBAUTHN_AUTHENTICATOR_ATTACHMENT_PLATFORM: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WEBAUTHN_AUTHENTICATOR_DETAILS {
    pub dwVersion: u32,
    pub cbAuthenticatorId: u32,
    pub pbAuthenticatorId: *mut u8,
    pub pwszAuthenticatorName: windows_sys::core::PCWSTR,
    pub cbAuthenticatorLogo: u32,
    pub pbAuthenticatorLogo: *mut u8,
    pub bLocked: windows_sys::core::BOOL,
}
impl Default for WEBAUTHN_AUTHENTICATOR_DETAILS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WEBAUTHN_AUTHENTICATOR_DETAILS_CURRENT_VERSION: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WEBAUTHN_AUTHENTICATOR_DETAILS_LIST {
    pub cAuthenticatorDetails: u32,
    pub ppAuthenticatorDetails: *mut *mut WEBAUTHN_AUTHENTICATOR_DETAILS,
}
impl Default for WEBAUTHN_AUTHENTICATOR_DETAILS_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WEBAUTHN_AUTHENTICATOR_DETAILS_OPTIONS {
    pub dwVersion: u32,
}
pub const WEBAUTHN_AUTHENTICATOR_DETAILS_OPTIONS_CURRENT_VERSION: u32 = 1u32;
pub const WEBAUTHN_AUTHENTICATOR_DETAILS_OPTIONS_VERSION_1: u32 = 1u32;
pub const WEBAUTHN_AUTHENTICATOR_DETAILS_VERSION_1: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy)]
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
    pub pAllowCredentialList: *mut WEBAUTHN_CREDENTIAL_LIST,
    pub dwCredLargeBlobOperation: u32,
    pub cbCredLargeBlob: u32,
    pub pbCredLargeBlob: *mut u8,
    pub pHmacSecretSaltValues: *mut WEBAUTHN_HMAC_SECRET_SALT_VALUES,
    pub bBrowserInPrivateMode: windows_sys::core::BOOL,
    pub pLinkedDevice: *mut CTAPCBOR_HYBRID_STORAGE_LINKED_DATA,
    pub bAutoFill: windows_sys::core::BOOL,
    pub cbJsonExt: u32,
    pub pbJsonExt: *mut u8,
    pub cCredentialHints: u32,
    pub ppwszCredentialHints: *const windows_sys::core::PCWSTR,
    pub pwszRemoteWebOrigin: windows_sys::core::PCWSTR,
    pub cbPublicKeyCredentialRequestOptionsJSON: u32,
    pub pbPublicKeyCredentialRequestOptionsJSON: *mut u8,
    pub cbAuthenticatorId: u32,
    pub pbAuthenticatorId: *mut u8,
}
impl Default for WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_CURRENT_VERSION: u32 = 9u32;
pub const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_VERSION_1: u32 = 1u32;
pub const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_VERSION_2: u32 = 2u32;
pub const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_VERSION_3: u32 = 3u32;
pub const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_VERSION_4: u32 = 4u32;
pub const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_VERSION_5: u32 = 5u32;
pub const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_VERSION_6: u32 = 6u32;
pub const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_VERSION_7: u32 = 7u32;
pub const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_VERSION_8: u32 = 8u32;
pub const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_VERSION_9: u32 = 9u32;
pub const WEBAUTHN_AUTHENTICATOR_HMAC_SECRET_VALUES_FLAG: u32 = 1048576u32;
#[repr(C)]
#[derive(Clone, Copy)]
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
    pub pExcludeCredentialList: *mut WEBAUTHN_CREDENTIAL_LIST,
    pub dwEnterpriseAttestation: u32,
    pub dwLargeBlobSupport: u32,
    pub bPreferResidentKey: windows_sys::core::BOOL,
    pub bBrowserInPrivateMode: windows_sys::core::BOOL,
    pub bEnablePrf: windows_sys::core::BOOL,
    pub pLinkedDevice: *mut CTAPCBOR_HYBRID_STORAGE_LINKED_DATA,
    pub cbJsonExt: u32,
    pub pbJsonExt: *mut u8,
    pub pPRFGlobalEval: *mut WEBAUTHN_HMAC_SECRET_SALT,
    pub cCredentialHints: u32,
    pub ppwszCredentialHints: *const windows_sys::core::PCWSTR,
    pub bThirdPartyPayment: windows_sys::core::BOOL,
    pub pwszRemoteWebOrigin: windows_sys::core::PCWSTR,
    pub cbPublicKeyCredentialCreationOptionsJSON: u32,
    pub pbPublicKeyCredentialCreationOptionsJSON: *mut u8,
    pub cbAuthenticatorId: u32,
    pub pbAuthenticatorId: *mut u8,
}
impl Default for WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS_CURRENT_VERSION: u32 = 9u32;
pub const WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS_VERSION_1: u32 = 1u32;
pub const WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS_VERSION_2: u32 = 2u32;
pub const WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS_VERSION_3: u32 = 3u32;
pub const WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS_VERSION_4: u32 = 4u32;
pub const WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS_VERSION_5: u32 = 5u32;
pub const WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS_VERSION_6: u32 = 6u32;
pub const WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS_VERSION_7: u32 = 7u32;
pub const WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS_VERSION_8: u32 = 8u32;
pub const WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS_VERSION_9: u32 = 9u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WEBAUTHN_CLIENT_DATA {
    pub dwVersion: u32,
    pub cbClientDataJSON: u32,
    pub pbClientDataJSON: *mut u8,
    pub pwszHashAlgId: windows_sys::core::PCWSTR,
}
impl Default for WEBAUTHN_CLIENT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WEBAUTHN_CLIENT_DATA_CURRENT_VERSION: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WEBAUTHN_COMMON_ATTESTATION {
    pub dwVersion: u32,
    pub pwszAlg: windows_sys::core::PCWSTR,
    pub lAlg: i32,
    pub cbSignature: u32,
    pub pbSignature: *mut u8,
    pub cX5c: u32,
    pub pX5c: *mut WEBAUTHN_X5C,
    pub pwszVer: windows_sys::core::PCWSTR,
    pub cbCertInfo: u32,
    pub pbCertInfo: *mut u8,
    pub cbPubArea: u32,
    pub pbPubArea: *mut u8,
}
impl Default for WEBAUTHN_COMMON_ATTESTATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WEBAUTHN_COMMON_ATTESTATION_CURRENT_VERSION: u32 = 1u32;
pub const WEBAUTHN_COSE_ALGORITHM_ECDSA_P256_WITH_SHA256: i32 = -7i32;
pub const WEBAUTHN_COSE_ALGORITHM_ECDSA_P384_WITH_SHA384: i32 = -35i32;
pub const WEBAUTHN_COSE_ALGORITHM_ECDSA_P521_WITH_SHA512: i32 = -36i32;
pub const WEBAUTHN_COSE_ALGORITHM_RSASSA_PKCS1_V1_5_WITH_SHA256: i32 = -257i32;
pub const WEBAUTHN_COSE_ALGORITHM_RSASSA_PKCS1_V1_5_WITH_SHA384: i32 = -258i32;
pub const WEBAUTHN_COSE_ALGORITHM_RSASSA_PKCS1_V1_5_WITH_SHA512: i32 = -259i32;
pub const WEBAUTHN_COSE_ALGORITHM_RSA_PSS_WITH_SHA256: i32 = -37i32;
pub const WEBAUTHN_COSE_ALGORITHM_RSA_PSS_WITH_SHA384: i32 = -38i32;
pub const WEBAUTHN_COSE_ALGORITHM_RSA_PSS_WITH_SHA512: i32 = -39i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WEBAUTHN_COSE_CREDENTIAL_PARAMETER {
    pub dwVersion: u32,
    pub pwszCredentialType: windows_sys::core::PCWSTR,
    pub lAlg: i32,
}
impl Default for WEBAUTHN_COSE_CREDENTIAL_PARAMETER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WEBAUTHN_COSE_CREDENTIAL_PARAMETERS {
    pub cCredentialParameters: u32,
    pub pCredentialParameters: *mut WEBAUTHN_COSE_CREDENTIAL_PARAMETER,
}
impl Default for WEBAUTHN_COSE_CREDENTIAL_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WEBAUTHN_COSE_CREDENTIAL_PARAMETER_CURRENT_VERSION: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WEBAUTHN_CREDENTIAL {
    pub dwVersion: u32,
    pub cbId: u32,
    pub pbId: *mut u8,
    pub pwszCredentialType: windows_sys::core::PCWSTR,
}
impl Default for WEBAUTHN_CREDENTIAL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WEBAUTHN_CREDENTIALS {
    pub cCredentials: u32,
    pub pCredentials: *mut WEBAUTHN_CREDENTIAL,
}
impl Default for WEBAUTHN_CREDENTIALS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WEBAUTHN_CREDENTIAL_ATTESTATION {
    pub dwVersion: u32,
    pub pwszFormatType: windows_sys::core::PCWSTR,
    pub cbAuthenticatorData: u32,
    pub pbAuthenticatorData: *mut u8,
    pub cbAttestation: u32,
    pub pbAttestation: *mut u8,
    pub dwAttestationDecodeType: u32,
    pub pvAttestationDecode: *mut core::ffi::c_void,
    pub cbAttestationObject: u32,
    pub pbAttestationObject: *mut u8,
    pub cbCredentialId: u32,
    pub pbCredentialId: *mut u8,
    pub Extensions: WEBAUTHN_EXTENSIONS,
    pub dwUsedTransport: u32,
    pub bEpAtt: windows_sys::core::BOOL,
    pub bLargeBlobSupported: windows_sys::core::BOOL,
    pub bResidentKey: windows_sys::core::BOOL,
    pub bPrfEnabled: windows_sys::core::BOOL,
    pub cbUnsignedExtensionOutputs: u32,
    pub pbUnsignedExtensionOutputs: *mut u8,
    pub pHmacSecret: *mut WEBAUTHN_HMAC_SECRET_SALT,
    pub bThirdPartyPayment: windows_sys::core::BOOL,
    pub dwTransports: u32,
    pub cbClientDataJSON: u32,
    pub pbClientDataJSON: *mut u8,
    pub cbRegistrationResponseJSON: u32,
    pub pbRegistrationResponseJSON: *mut u8,
}
impl Default for WEBAUTHN_CREDENTIAL_ATTESTATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WEBAUTHN_CREDENTIAL_ATTESTATION_CURRENT_VERSION: u32 = 8u32;
pub const WEBAUTHN_CREDENTIAL_ATTESTATION_VERSION_1: u32 = 1u32;
pub const WEBAUTHN_CREDENTIAL_ATTESTATION_VERSION_2: u32 = 2u32;
pub const WEBAUTHN_CREDENTIAL_ATTESTATION_VERSION_3: u32 = 3u32;
pub const WEBAUTHN_CREDENTIAL_ATTESTATION_VERSION_4: u32 = 4u32;
pub const WEBAUTHN_CREDENTIAL_ATTESTATION_VERSION_5: u32 = 5u32;
pub const WEBAUTHN_CREDENTIAL_ATTESTATION_VERSION_6: u32 = 6u32;
pub const WEBAUTHN_CREDENTIAL_ATTESTATION_VERSION_7: u32 = 7u32;
pub const WEBAUTHN_CREDENTIAL_ATTESTATION_VERSION_8: u32 = 8u32;
pub const WEBAUTHN_CREDENTIAL_CURRENT_VERSION: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WEBAUTHN_CREDENTIAL_DETAILS {
    pub dwVersion: u32,
    pub cbCredentialID: u32,
    pub pbCredentialID: *mut u8,
    pub pRpInformation: *mut WEBAUTHN_RP_ENTITY_INFORMATION,
    pub pUserInformation: *mut WEBAUTHN_USER_ENTITY_INFORMATION,
    pub bRemovable: windows_sys::core::BOOL,
    pub bBackedUp: windows_sys::core::BOOL,
    pub pwszAuthenticatorName: windows_sys::core::PCWSTR,
    pub cbAuthenticatorLogo: u32,
    pub pbAuthenticatorLogo: *mut u8,
    pub bThirdPartyPayment: windows_sys::core::BOOL,
    pub dwTransports: u32,
}
impl Default for WEBAUTHN_CREDENTIAL_DETAILS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WEBAUTHN_CREDENTIAL_DETAILS_CURRENT_VERSION: u32 = 4u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WEBAUTHN_CREDENTIAL_DETAILS_LIST {
    pub cCredentialDetails: u32,
    pub ppCredentialDetails: *mut *mut WEBAUTHN_CREDENTIAL_DETAILS,
}
impl Default for WEBAUTHN_CREDENTIAL_DETAILS_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WEBAUTHN_CREDENTIAL_DETAILS_VERSION_1: u32 = 1u32;
pub const WEBAUTHN_CREDENTIAL_DETAILS_VERSION_2: u32 = 2u32;
pub const WEBAUTHN_CREDENTIAL_DETAILS_VERSION_3: u32 = 3u32;
pub const WEBAUTHN_CREDENTIAL_DETAILS_VERSION_4: u32 = 4u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WEBAUTHN_CREDENTIAL_EX {
    pub dwVersion: u32,
    pub cbId: u32,
    pub pbId: *mut u8,
    pub pwszCredentialType: windows_sys::core::PCWSTR,
    pub dwTransports: u32,
}
impl Default for WEBAUTHN_CREDENTIAL_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WEBAUTHN_CREDENTIAL_EX_CURRENT_VERSION: u32 = 1u32;
pub const WEBAUTHN_CREDENTIAL_HINT_CLIENT_DEVICE: windows_sys::core::PCWSTR = windows_sys::core::w!("client-device");
pub const WEBAUTHN_CREDENTIAL_HINT_HYBRID: windows_sys::core::PCWSTR = windows_sys::core::w!("hybrid");
pub const WEBAUTHN_CREDENTIAL_HINT_SECURITY_KEY: windows_sys::core::PCWSTR = windows_sys::core::w!("security-key");
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WEBAUTHN_CREDENTIAL_LIST {
    pub cCredentials: u32,
    pub ppCredentials: *mut *mut WEBAUTHN_CREDENTIAL_EX,
}
impl Default for WEBAUTHN_CREDENTIAL_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WEBAUTHN_CREDENTIAL_TYPE_PUBLIC_KEY: windows_sys::core::PCWSTR = windows_sys::core::w!("public-key");
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WEBAUTHN_CRED_BLOB_EXTENSION {
    pub cbCredBlob: u32,
    pub pbCredBlob: *mut u8,
}
impl Default for WEBAUTHN_CRED_BLOB_EXTENSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WEBAUTHN_CRED_LARGE_BLOB_OPERATION_DELETE: u32 = 3u32;
pub const WEBAUTHN_CRED_LARGE_BLOB_OPERATION_GET: u32 = 1u32;
pub const WEBAUTHN_CRED_LARGE_BLOB_OPERATION_NONE: u32 = 0u32;
pub const WEBAUTHN_CRED_LARGE_BLOB_OPERATION_SET: u32 = 2u32;
pub const WEBAUTHN_CRED_LARGE_BLOB_STATUS_AUTHENTICATOR_ERROR: u32 = 9u32;
pub const WEBAUTHN_CRED_LARGE_BLOB_STATUS_INVALID_DATA: u32 = 3u32;
pub const WEBAUTHN_CRED_LARGE_BLOB_STATUS_INVALID_PARAMETER: u32 = 4u32;
pub const WEBAUTHN_CRED_LARGE_BLOB_STATUS_LACK_OF_SPACE: u32 = 7u32;
pub const WEBAUTHN_CRED_LARGE_BLOB_STATUS_MULTIPLE_CREDENTIALS: u32 = 6u32;
pub const WEBAUTHN_CRED_LARGE_BLOB_STATUS_NONE: u32 = 0u32;
pub const WEBAUTHN_CRED_LARGE_BLOB_STATUS_NOT_FOUND: u32 = 5u32;
pub const WEBAUTHN_CRED_LARGE_BLOB_STATUS_NOT_SUPPORTED: u32 = 2u32;
pub const WEBAUTHN_CRED_LARGE_BLOB_STATUS_PLATFORM_ERROR: u32 = 8u32;
pub const WEBAUTHN_CRED_LARGE_BLOB_STATUS_SUCCESS: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WEBAUTHN_CRED_PROTECT_EXTENSION_IN {
    pub dwCredProtect: u32,
    pub bRequireCredProtect: windows_sys::core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WEBAUTHN_CRED_WITH_HMAC_SECRET_SALT {
    pub cbCredID: u32,
    pub pbCredID: *mut u8,
    pub pHmacSecretSalt: *mut WEBAUTHN_HMAC_SECRET_SALT,
}
impl Default for WEBAUTHN_CRED_WITH_HMAC_SECRET_SALT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WEBAUTHN_CTAPCBOR_AUTHENTICATOR_OPTIONS {
    pub dwVersion: u32,
    pub lUp: i32,
    pub lUv: i32,
    pub lRequireResidentKey: i32,
}
pub const WEBAUTHN_CTAPCBOR_AUTHENTICATOR_OPTIONS_CURRENT_VERSION: u32 = 1u32;
pub const WEBAUTHN_CTAPCBOR_AUTHENTICATOR_OPTIONS_VERSION_1: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WEBAUTHN_CTAPCBOR_ECC_PUBLIC_KEY {
    pub dwVersion: u32,
    pub lKty: i32,
    pub lAlg: i32,
    pub lCrv: i32,
    pub cbX: u32,
    pub pbX: *mut u8,
    pub cbY: u32,
    pub pbY: *mut u8,
}
impl Default for WEBAUTHN_CTAPCBOR_ECC_PUBLIC_KEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WEBAUTHN_CTAPCBOR_ECC_PUBLIC_KEY_CURRENT_VERSION: u32 = 1u32;
pub const WEBAUTHN_CTAPCBOR_ECC_PUBLIC_KEY_VERSION_1: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WEBAUTHN_CTAPCBOR_GET_ASSERTION_REQUEST {
    pub dwVersion: u32,
    pub pwszRpId: windows_sys::core::PCWSTR,
    pub cbRpId: u32,
    pub pbRpId: *mut u8,
    pub cbClientDataHash: u32,
    pub pbClientDataHash: *mut u8,
    pub CredentialList: WEBAUTHN_CREDENTIAL_LIST,
    pub cbCborExtensionsMap: u32,
    pub pbCborExtensionsMap: *mut u8,
    pub pAuthenticatorOptions: *mut WEBAUTHN_CTAPCBOR_AUTHENTICATOR_OPTIONS,
    pub fEmptyPinAuth: windows_sys::core::BOOL,
    pub cbPinAuth: u32,
    pub pbPinAuth: *mut u8,
    pub pHmacSaltExtension: *mut WEBAUTHN_CTAPCBOR_HMAC_SALT_EXTENSION,
    pub cbHmacSecretSaltValues: u32,
    pub pbHmacSecretSaltValues: *mut u8,
    pub dwPinProtocol: u32,
    pub lCredBlobExt: i32,
    pub lLargeBlobKeyExt: i32,
    pub dwCredLargeBlobOperation: u32,
    pub cbCredLargeBlobCompressed: u32,
    pub pbCredLargeBlobCompressed: *mut u8,
    pub dwCredLargeBlobOriginalSize: u32,
    pub cbJsonExt: u32,
    pub pbJsonExt: *mut u8,
}
impl Default for WEBAUTHN_CTAPCBOR_GET_ASSERTION_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WEBAUTHN_CTAPCBOR_GET_ASSERTION_REQUEST_CURRENT_VERSION: u32 = 1u32;
pub const WEBAUTHN_CTAPCBOR_GET_ASSERTION_REQUEST_VERSION_1: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WEBAUTHN_CTAPCBOR_GET_ASSERTION_RESPONSE {
    pub WebAuthNAssertion: WEBAUTHN_ASSERTION,
    pub pUserInformation: *mut WEBAUTHN_USER_ENTITY_INFORMATION,
    pub dwNumberOfCredentials: u32,
    pub lUserSelected: i32,
    pub cbLargeBlobKey: u32,
    pub pbLargeBlobKey: *mut u8,
    pub cbUnsignedExtensionOutputs: u32,
    pub pbUnsignedExtensionOutputs: *mut u8,
}
impl Default for WEBAUTHN_CTAPCBOR_GET_ASSERTION_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WEBAUTHN_CTAPCBOR_HMAC_SALT_EXTENSION {
    pub dwVersion: u32,
    pub pKeyAgreement: *mut WEBAUTHN_CTAPCBOR_ECC_PUBLIC_KEY,
    pub cbEncryptedSalt: u32,
    pub pbEncryptedSalt: *mut u8,
    pub cbSaltAuth: u32,
    pub pbSaltAuth: *mut u8,
}
impl Default for WEBAUTHN_CTAPCBOR_HMAC_SALT_EXTENSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WEBAUTHN_CTAPCBOR_HMAC_SALT_EXTENSION_CURRENT_VERSION: u32 = 1u32;
pub const WEBAUTHN_CTAPCBOR_HMAC_SALT_EXTENSION_VERSION_1: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WEBAUTHN_CTAPCBOR_MAKE_CREDENTIAL_REQUEST {
    pub dwVersion: u32,
    pub cbRpId: u32,
    pub pbRpId: *mut u8,
    pub cbClientDataHash: u32,
    pub pbClientDataHash: *mut u8,
    pub pRpInformation: *mut WEBAUTHN_RP_ENTITY_INFORMATION,
    pub pUserInformation: *mut WEBAUTHN_USER_ENTITY_INFORMATION,
    pub WebAuthNCredentialParameters: WEBAUTHN_COSE_CREDENTIAL_PARAMETERS,
    pub CredentialList: WEBAUTHN_CREDENTIAL_LIST,
    pub cbCborExtensionsMap: u32,
    pub pbCborExtensionsMap: *mut u8,
    pub pAuthenticatorOptions: *mut WEBAUTHN_CTAPCBOR_AUTHENTICATOR_OPTIONS,
    pub fEmptyPinAuth: windows_sys::core::BOOL,
    pub cbPinAuth: u32,
    pub pbPinAuth: *mut u8,
    pub lHmacSecretExt: i32,
    pub pHmacSecretMcExtension: *mut WEBAUTHN_CTAPCBOR_HMAC_SALT_EXTENSION,
    pub lPrfExt: i32,
    pub cbHmacSecretSaltValues: u32,
    pub pbHmacSecretSaltValues: *mut u8,
    pub dwCredProtect: u32,
    pub dwPinProtocol: u32,
    pub dwEnterpriseAttestation: u32,
    pub cbCredBlobExt: u32,
    pub pbCredBlobExt: *mut u8,
    pub lLargeBlobKeyExt: i32,
    pub dwLargeBlobSupport: u32,
    pub lMinPinLengthExt: i32,
    pub cbJsonExt: u32,
    pub pbJsonExt: *mut u8,
}
impl Default for WEBAUTHN_CTAPCBOR_MAKE_CREDENTIAL_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WEBAUTHN_CTAPCBOR_MAKE_CREDENTIAL_REQUEST_CURRENT_VERSION: u32 = 1u32;
pub const WEBAUTHN_CTAPCBOR_MAKE_CREDENTIAL_REQUEST_VERSION_1: u32 = 1u32;
pub const WEBAUTHN_CTAP_ONE_HMAC_SECRET_LENGTH: u32 = 32u32;
pub const WEBAUTHN_CTAP_TRANSPORT_BLE: u32 = 4u32;
pub const WEBAUTHN_CTAP_TRANSPORT_BLE_STRING: windows_sys::core::PCSTR = windows_sys::core::s!("ble");
pub const WEBAUTHN_CTAP_TRANSPORT_FLAGS_MASK: u32 = 127u32;
pub const WEBAUTHN_CTAP_TRANSPORT_HYBRID: u32 = 32u32;
pub const WEBAUTHN_CTAP_TRANSPORT_HYBRID_STRING: windows_sys::core::PCSTR = windows_sys::core::s!("hybrid");
pub const WEBAUTHN_CTAP_TRANSPORT_INTERNAL: u32 = 16u32;
pub const WEBAUTHN_CTAP_TRANSPORT_INTERNAL_STRING: windows_sys::core::PCSTR = windows_sys::core::s!("internal");
pub const WEBAUTHN_CTAP_TRANSPORT_NFC: u32 = 2u32;
pub const WEBAUTHN_CTAP_TRANSPORT_NFC_STRING: windows_sys::core::PCSTR = windows_sys::core::s!("nfc");
pub const WEBAUTHN_CTAP_TRANSPORT_SMART_CARD: u32 = 64u32;
pub const WEBAUTHN_CTAP_TRANSPORT_SMART_CARD_STRING: windows_sys::core::PCSTR = windows_sys::core::s!("smart-card");
pub const WEBAUTHN_CTAP_TRANSPORT_TEST: u32 = 8u32;
pub const WEBAUTHN_CTAP_TRANSPORT_USB: u32 = 1u32;
pub const WEBAUTHN_CTAP_TRANSPORT_USB_STRING: windows_sys::core::PCSTR = windows_sys::core::s!("usb");
pub const WEBAUTHN_ENTERPRISE_ATTESTATION_NONE: u32 = 0u32;
pub const WEBAUTHN_ENTERPRISE_ATTESTATION_PLATFORM_MANAGED: u32 = 2u32;
pub const WEBAUTHN_ENTERPRISE_ATTESTATION_VENDOR_FACILITATED: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WEBAUTHN_EXTENSION {
    pub pwszExtensionIdentifier: windows_sys::core::PCWSTR,
    pub cbExtension: u32,
    pub pvExtension: *mut core::ffi::c_void,
}
impl Default for WEBAUTHN_EXTENSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WEBAUTHN_EXTENSIONS {
    pub cExtensions: u32,
    pub pExtensions: *mut WEBAUTHN_EXTENSION,
}
impl Default for WEBAUTHN_EXTENSIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WEBAUTHN_EXTENSIONS_IDENTIFIER_CRED_BLOB: windows_sys::core::PCWSTR = windows_sys::core::w!("credBlob");
pub const WEBAUTHN_EXTENSIONS_IDENTIFIER_CRED_PROTECT: windows_sys::core::PCWSTR = windows_sys::core::w!("credProtect");
pub const WEBAUTHN_EXTENSIONS_IDENTIFIER_HMAC_SECRET: windows_sys::core::PCWSTR = windows_sys::core::w!("hmac-secret");
pub const WEBAUTHN_EXTENSIONS_IDENTIFIER_MIN_PIN_LENGTH: windows_sys::core::PCWSTR = windows_sys::core::w!("minPinLength");
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WEBAUTHN_GET_CREDENTIALS_OPTIONS {
    pub dwVersion: u32,
    pub pwszRpId: windows_sys::core::PCWSTR,
    pub bBrowserInPrivateMode: windows_sys::core::BOOL,
}
impl Default for WEBAUTHN_GET_CREDENTIALS_OPTIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WEBAUTHN_GET_CREDENTIALS_OPTIONS_CURRENT_VERSION: u32 = 1u32;
pub const WEBAUTHN_GET_CREDENTIALS_OPTIONS_VERSION_1: u32 = 1u32;
pub const WEBAUTHN_HASH_ALGORITHM_SHA_256: windows_sys::core::PCWSTR = windows_sys::core::w!("SHA-256");
pub const WEBAUTHN_HASH_ALGORITHM_SHA_384: windows_sys::core::PCWSTR = windows_sys::core::w!("SHA-384");
pub const WEBAUTHN_HASH_ALGORITHM_SHA_512: windows_sys::core::PCWSTR = windows_sys::core::w!("SHA-512");
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WEBAUTHN_HMAC_SECRET_SALT {
    pub cbFirst: u32,
    pub pbFirst: *mut u8,
    pub cbSecond: u32,
    pub pbSecond: *mut u8,
}
impl Default for WEBAUTHN_HMAC_SECRET_SALT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WEBAUTHN_HMAC_SECRET_SALT_VALUES {
    pub pGlobalHmacSalt: *mut WEBAUTHN_HMAC_SECRET_SALT,
    pub cCredWithHmacSecretSaltList: u32,
    pub pCredWithHmacSecretSaltList: *mut WEBAUTHN_CRED_WITH_HMAC_SECRET_SALT,
}
impl Default for WEBAUTHN_HMAC_SECRET_SALT_VALUES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WEBAUTHN_LARGE_BLOB_SUPPORT_NONE: u32 = 0u32;
pub const WEBAUTHN_LARGE_BLOB_SUPPORT_PREFERRED: u32 = 2u32;
pub const WEBAUTHN_LARGE_BLOB_SUPPORT_REQUIRED: u32 = 1u32;
pub const WEBAUTHN_MAX_USER_ID_LENGTH: u32 = 64u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WEBAUTHN_PLUGIN_ADD_AUTHENTICATOR_OPTIONS {
    pub pwszAuthenticatorName: windows_sys::core::PCWSTR,
    pub rclsid: *const windows_sys::core::GUID,
    pub pwszPluginRpId: windows_sys::core::PCWSTR,
    pub pwszLightThemeLogoSvg: windows_sys::core::PCWSTR,
    pub pwszDarkThemeLogoSvg: windows_sys::core::PCWSTR,
    pub cbAuthenticatorInfo: u32,
    pub pbAuthenticatorInfo: *const u8,
    pub cSupportedRpIds: u32,
    pub ppwszSupportedRpIds: *const windows_sys::core::PCWSTR,
}
impl Default for WEBAUTHN_PLUGIN_ADD_AUTHENTICATOR_OPTIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WEBAUTHN_PLUGIN_ADD_AUTHENTICATOR_RESPONSE {
    pub cbOpSignPubKey: u32,
    pub pbOpSignPubKey: *mut u8,
}
impl Default for WEBAUTHN_PLUGIN_ADD_AUTHENTICATOR_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WEBAUTHN_PLUGIN_CANCEL_OPERATION_REQUEST {
    pub transactionId: windows_sys::core::GUID,
    pub cbRequestSignature: u32,
    pub pbRequestSignature: *mut u8,
}
impl Default for WEBAUTHN_PLUGIN_CANCEL_OPERATION_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WEBAUTHN_PLUGIN_CREDENTIAL_DETAILS {
    pub cbCredentialId: u32,
    pub pbCredentialId: *const u8,
    pub pwszRpId: windows_sys::core::PCWSTR,
    pub pwszRpName: windows_sys::core::PCWSTR,
    pub cbUserId: u32,
    pub pbUserId: *const u8,
    pub pwszUserName: windows_sys::core::PCWSTR,
    pub pwszUserDisplayName: windows_sys::core::PCWSTR,
}
impl Default for WEBAUTHN_PLUGIN_CREDENTIAL_DETAILS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WEBAUTHN_PLUGIN_OPERATION_REQUEST {
    pub hWnd: super::super::super::Foundation::HWND,
    pub transactionId: windows_sys::core::GUID,
    pub cbRequestSignature: u32,
    pub pbRequestSignature: *mut u8,
    pub requestType: WEBAUTHN_PLUGIN_REQUEST_TYPE,
    pub cbEncodedRequest: u32,
    pub pbEncodedRequest: *mut u8,
}
impl Default for WEBAUTHN_PLUGIN_OPERATION_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WEBAUTHN_PLUGIN_OPERATION_RESPONSE {
    pub cbEncodedResponse: u32,
    pub pbEncodedResponse: *mut u8,
}
impl Default for WEBAUTHN_PLUGIN_OPERATION_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WEBAUTHN_PLUGIN_PERFORM_UV_OPERATION_TYPE = i32;
pub type WEBAUTHN_PLUGIN_REQUEST_TYPE = i32;
pub const WEBAUTHN_PLUGIN_REQUEST_TYPE_CTAP2_CBOR: WEBAUTHN_PLUGIN_REQUEST_TYPE = 1i32;
pub type WEBAUTHN_PLUGIN_STATUS_CHANGE_CALLBACK = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void)>;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WEBAUTHN_PLUGIN_UPDATE_AUTHENTICATOR_DETAILS {
    pub pwszAuthenticatorName: windows_sys::core::PCWSTR,
    pub rclsid: *const windows_sys::core::GUID,
    pub rclsidNew: *const windows_sys::core::GUID,
    pub pwszLightThemeLogoSvg: windows_sys::core::PCWSTR,
    pub pwszDarkThemeLogoSvg: windows_sys::core::PCWSTR,
    pub cbAuthenticatorInfo: u32,
    pub pbAuthenticatorInfo: *const u8,
    pub cSupportedRpIds: u32,
    pub ppwszSupportedRpIds: *const windows_sys::core::PCWSTR,
}
impl Default for WEBAUTHN_PLUGIN_UPDATE_AUTHENTICATOR_DETAILS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WEBAUTHN_PLUGIN_USER_VERIFICATION_REQUEST {
    pub hwnd: super::super::super::Foundation::HWND,
    pub rguidTransactionId: *const windows_sys::core::GUID,
    pub pwszUsername: windows_sys::core::PCWSTR,
    pub pwszDisplayHint: windows_sys::core::PCWSTR,
}
impl Default for WEBAUTHN_PLUGIN_USER_VERIFICATION_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WEBAUTHN_RP_ENTITY_INFORMATION {
    pub dwVersion: u32,
    pub pwszId: windows_sys::core::PCWSTR,
    pub pwszName: windows_sys::core::PCWSTR,
    pub pwszIcon: windows_sys::core::PCWSTR,
}
impl Default for WEBAUTHN_RP_ENTITY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WEBAUTHN_RP_ENTITY_INFORMATION_CURRENT_VERSION: u32 = 1u32;
pub const WEBAUTHN_RP_ENTITY_INFORMATION_VERSION_1: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WEBAUTHN_USER_ENTITY_INFORMATION {
    pub dwVersion: u32,
    pub cbId: u32,
    pub pbId: *mut u8,
    pub pwszName: windows_sys::core::PCWSTR,
    pub pwszIcon: windows_sys::core::PCWSTR,
    pub pwszDisplayName: windows_sys::core::PCWSTR,
}
impl Default for WEBAUTHN_USER_ENTITY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WEBAUTHN_USER_ENTITY_INFORMATION_CURRENT_VERSION: u32 = 1u32;
pub const WEBAUTHN_USER_ENTITY_INFORMATION_VERSION_1: u32 = 1u32;
pub const WEBAUTHN_USER_VERIFICATION_ANY: u32 = 0u32;
pub const WEBAUTHN_USER_VERIFICATION_OPTIONAL: u32 = 1u32;
pub const WEBAUTHN_USER_VERIFICATION_OPTIONAL_WITH_CREDENTIAL_ID_LIST: u32 = 2u32;
pub const WEBAUTHN_USER_VERIFICATION_REQUIRED: u32 = 3u32;
pub const WEBAUTHN_USER_VERIFICATION_REQUIREMENT_ANY: u32 = 0u32;
pub const WEBAUTHN_USER_VERIFICATION_REQUIREMENT_DISCOURAGED: u32 = 3u32;
pub const WEBAUTHN_USER_VERIFICATION_REQUIREMENT_PREFERRED: u32 = 2u32;
pub const WEBAUTHN_USER_VERIFICATION_REQUIREMENT_REQUIRED: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WEBAUTHN_X5C {
    pub cbData: u32,
    pub pbData: *mut u8,
}
impl Default for WEBAUTHN_X5C {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
