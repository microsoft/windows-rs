#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn WebAuthNAuthenticatorGetAssertion<P1>(hwnd: super::windef::HWND, pwszrpid: P1, pwebauthnclientdata: *const WEBAUTHN_CLIENT_DATA, pwebauthngetassertionoptions: Option<*const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS>) -> windows_core::Result<PWEBAUTHN_ASSERTION>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("webauthn.dll" "system" fn WebAuthNAuthenticatorGetAssertion(hwnd : super::windef::HWND, pwszrpid : windows_core::PCWSTR, pwebauthnclientdata : *const WEBAUTHN_CLIENT_DATA, pwebauthngetassertionoptions : *const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS, ppwebauthnassertion : *mut PWEBAUTHN_ASSERTION) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WebAuthNAuthenticatorGetAssertion(hwnd, pwszrpid.param().abi(), pwebauthnclientdata, pwebauthngetassertionoptions.unwrap_or(core::mem::zeroed()) as _, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn WebAuthNAuthenticatorMakeCredential(hwnd: super::windef::HWND, prpinformation: *const WEBAUTHN_RP_ENTITY_INFORMATION, puserinformation: *const WEBAUTHN_USER_ENTITY_INFORMATION, ppubkeycredparams: *const WEBAUTHN_COSE_CREDENTIAL_PARAMETERS, pwebauthnclientdata: *const WEBAUTHN_CLIENT_DATA, pwebauthnmakecredentialoptions: Option<*const WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS>) -> windows_core::Result<PWEBAUTHN_CREDENTIAL_ATTESTATION> {
    windows_core::link!("webauthn.dll" "system" fn WebAuthNAuthenticatorMakeCredential(hwnd : super::windef::HWND, prpinformation : *const WEBAUTHN_RP_ENTITY_INFORMATION, puserinformation : *const WEBAUTHN_USER_ENTITY_INFORMATION, ppubkeycredparams : *const WEBAUTHN_COSE_CREDENTIAL_PARAMETERS, pwebauthnclientdata : *const WEBAUTHN_CLIENT_DATA, pwebauthnmakecredentialoptions : *const WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS, ppwebauthncredentialattestation : *mut PWEBAUTHN_CREDENTIAL_ATTESTATION) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WebAuthNAuthenticatorMakeCredential(hwnd, prpinformation, puserinformation, ppubkeycredparams, pwebauthnclientdata, pwebauthnmakecredentialoptions.unwrap_or(core::mem::zeroed()) as _, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn WebAuthNCancelCurrentOperation(pcancellationid: *const windows_core::GUID) -> windows_core::HRESULT {
    windows_core::link!("webauthn.dll" "system" fn WebAuthNCancelCurrentOperation(pcancellationid : *const windows_core::GUID) -> windows_core::HRESULT);
    unsafe { WebAuthNCancelCurrentOperation(pcancellationid) }
}
#[inline]
pub unsafe fn WebAuthNDeletePlatformCredential(pbcredentialid: &[u8]) -> windows_core::HRESULT {
    windows_core::link!("webauthn.dll" "system" fn WebAuthNDeletePlatformCredential(cbcredentialid : u32, pbcredentialid : *const u8) -> windows_core::HRESULT);
    unsafe { WebAuthNDeletePlatformCredential(pbcredentialid.len().try_into().unwrap(), pbcredentialid.as_ptr()) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn WebAuthNFreeAssertion(pwebauthnassertion: *const WEBAUTHN_ASSERTION) {
    windows_core::link!("webauthn.dll" "system" fn WebAuthNFreeAssertion(pwebauthnassertion : *const WEBAUTHN_ASSERTION));
    unsafe { WebAuthNFreeAssertion(pwebauthnassertion) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn WebAuthNFreeAuthenticatorList(pauthenticatordetailslist: *const WEBAUTHN_AUTHENTICATOR_DETAILS_LIST) {
    windows_core::link!("webauthn.dll" "system" fn WebAuthNFreeAuthenticatorList(pauthenticatordetailslist : *const WEBAUTHN_AUTHENTICATOR_DETAILS_LIST));
    unsafe { WebAuthNFreeAuthenticatorList(pauthenticatordetailslist) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn WebAuthNFreeCredentialAttestation(pwebauthncredentialattestation: Option<*const WEBAUTHN_CREDENTIAL_ATTESTATION>) {
    windows_core::link!("webauthn.dll" "system" fn WebAuthNFreeCredentialAttestation(pwebauthncredentialattestation : *const WEBAUTHN_CREDENTIAL_ATTESTATION));
    unsafe { WebAuthNFreeCredentialAttestation(pwebauthncredentialattestation.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn WebAuthNFreePlatformCredentialList(pcredentialdetailslist: *const WEBAUTHN_CREDENTIAL_DETAILS_LIST) {
    windows_core::link!("webauthn.dll" "system" fn WebAuthNFreePlatformCredentialList(pcredentialdetailslist : *const WEBAUTHN_CREDENTIAL_DETAILS_LIST));
    unsafe { WebAuthNFreePlatformCredentialList(pcredentialdetailslist) }
}
#[inline]
pub unsafe fn WebAuthNGetApiVersionNumber() -> u32 {
    windows_core::link!("webauthn.dll" "system" fn WebAuthNGetApiVersionNumber() -> u32);
    unsafe { WebAuthNGetApiVersionNumber() }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn WebAuthNGetAuthenticatorList(pwebauthngetauthenticatorlistoptions: Option<*const WEBAUTHN_AUTHENTICATOR_DETAILS_OPTIONS>) -> windows_core::Result<PWEBAUTHN_AUTHENTICATOR_DETAILS_LIST> {
    windows_core::link!("webauthn.dll" "system" fn WebAuthNGetAuthenticatorList(pwebauthngetauthenticatorlistoptions : *const WEBAUTHN_AUTHENTICATOR_DETAILS_OPTIONS, ppauthenticatordetailslist : *mut PWEBAUTHN_AUTHENTICATOR_DETAILS_LIST) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WebAuthNGetAuthenticatorList(pwebauthngetauthenticatorlistoptions.unwrap_or(core::mem::zeroed()) as _, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn WebAuthNGetCancellationId() -> windows_core::Result<windows_core::GUID> {
    windows_core::link!("webauthn.dll" "system" fn WebAuthNGetCancellationId(pcancellationid : *mut windows_core::GUID) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WebAuthNGetCancellationId(&mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn WebAuthNGetErrorName(hr: windows_core::HRESULT) -> windows_core::PCWSTR {
    windows_core::link!("webauthn.dll" "system" fn WebAuthNGetErrorName(hr : windows_core::HRESULT) -> windows_core::PCWSTR);
    unsafe { WebAuthNGetErrorName(hr) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn WebAuthNGetPlatformCredentialList(pgetcredentialsoptions: *const WEBAUTHN_GET_CREDENTIALS_OPTIONS) -> windows_core::Result<PWEBAUTHN_CREDENTIAL_DETAILS_LIST> {
    windows_core::link!("webauthn.dll" "system" fn WebAuthNGetPlatformCredentialList(pgetcredentialsoptions : *const WEBAUTHN_GET_CREDENTIALS_OPTIONS, ppcredentialdetailslist : *mut PWEBAUTHN_CREDENTIAL_DETAILS_LIST) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WebAuthNGetPlatformCredentialList(pgetcredentialsoptions, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn WebAuthNGetW3CExceptionDOMError(hr: windows_core::HRESULT) -> windows_core::HRESULT {
    windows_core::link!("webauthn.dll" "system" fn WebAuthNGetW3CExceptionDOMError(hr : windows_core::HRESULT) -> windows_core::HRESULT);
    unsafe { WebAuthNGetW3CExceptionDOMError(hr) }
}
#[inline]
pub unsafe fn WebAuthNIsUserVerifyingPlatformAuthenticatorAvailable() -> windows_core::Result<windows_core::BOOL> {
    windows_core::link!("webauthn.dll" "system" fn WebAuthNIsUserVerifyingPlatformAuthenticatorAvailable(pbisuserverifyingplatformauthenticatoravailable : *mut windows_core::BOOL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WebAuthNIsUserVerifyingPlatformAuthenticatorAvailable(&mut result__).map(|| result__)
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CTAPCBOR_HYBRID_STORAGE_LINKED_DATA {
    pub dwVersion: u32,
    pub cbContactId: u32,
    pub pbContactId: super::minwindef::PBYTE,
    pub cbLinkId: u32,
    pub pbLinkId: super::minwindef::PBYTE,
    pub cbLinkSecret: u32,
    pub pbLinkSecret: super::minwindef::PBYTE,
    pub cbPublicKey: u32,
    pub pbPublicKey: super::minwindef::PBYTE,
    pub pwszAuthenticatorName: windows_core::PCWSTR,
    pub wEncodedTunnelServerDomain: u16,
}
pub const CTAPCBOR_HYBRID_STORAGE_LINKED_DATA_CURRENT_VERSION: u32 = 1;
pub const CTAPCBOR_HYBRID_STORAGE_LINKED_DATA_VERSION_1: u32 = 1;
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EXPERIMENTAL_WEBAUTHN_CTAPCBOR_AUTHENTICATOR_OPTIONS {
    pub dwVersion: u32,
    pub lUp: i32,
    pub lUv: i32,
    pub lRequireResidentKey: i32,
}
pub const EXPERIMENTAL_WEBAUTHN_CTAPCBOR_AUTHENTICATOR_OPTIONS_CURRENT_VERSION: u32 = 1;
pub const EXPERIMENTAL_WEBAUTHN_CTAPCBOR_AUTHENTICATOR_OPTIONS_VERSION_1: u32 = 1;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EXPERIMENTAL_WEBAUTHN_CTAPCBOR_ECC_PUBLIC_KEY {
    pub dwVersion: u32,
    pub lKty: i32,
    pub lAlg: i32,
    pub lCrv: i32,
    pub cbX: u32,
    pub pbX: super::minwindef::PBYTE,
    pub cbY: u32,
    pub pbY: super::minwindef::PBYTE,
}
pub const EXPERIMENTAL_WEBAUTHN_CTAPCBOR_ECC_PUBLIC_KEY_CURRENT_VERSION: u32 = 1;
pub const EXPERIMENTAL_WEBAUTHN_CTAPCBOR_ECC_PUBLIC_KEY_VERSION_1: u32 = 1;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EXPERIMENTAL_WEBAUTHN_CTAPCBOR_GET_ASSERTION_REQUEST {
    pub dwVersion: u32,
    pub pwszRpId: windows_core::PCWSTR,
    pub cbRpId: u32,
    pub pbRpId: super::minwindef::PBYTE,
    pub cbClientDataHash: u32,
    pub pbClientDataHash: super::minwindef::PBYTE,
    pub CredentialList: WEBAUTHN_CREDENTIAL_LIST,
    pub cbCborExtensionsMap: u32,
    pub pbCborExtensionsMap: super::minwindef::PBYTE,
    pub pAuthenticatorOptions: EXPERIMENTAL_PWEBAUTHN_CTAPCBOR_AUTHENTICATOR_OPTIONS,
    pub fEmptyPinAuth: windows_core::BOOL,
    pub cbPinAuth: u32,
    pub pbPinAuth: super::minwindef::PBYTE,
    pub pHmacSaltExtension: EXPERIMENTAL_PWEBAUTHN_CTAPCBOR_HMAC_SALT_EXTENSION,
    pub cbHmacSecretSaltValues: u32,
    pub pbHmacSecretSaltValues: super::minwindef::PBYTE,
    pub dwPinProtocol: u32,
    pub lCredBlobExt: i32,
    pub lLargeBlobKeyExt: i32,
    pub dwCredLargeBlobOperation: u32,
    pub cbCredLargeBlobCompressed: u32,
    pub pbCredLargeBlobCompressed: super::minwindef::PBYTE,
    pub dwCredLargeBlobOriginalSize: u32,
    pub cbJsonExt: u32,
    pub pbJsonExt: super::minwindef::PBYTE,
}
pub const EXPERIMENTAL_WEBAUTHN_CTAPCBOR_GET_ASSERTION_REQUEST_CURRENT_VERSION: u32 = 1;
pub const EXPERIMENTAL_WEBAUTHN_CTAPCBOR_GET_ASSERTION_REQUEST_VERSION_1: u32 = 1;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EXPERIMENTAL_WEBAUTHN_CTAPCBOR_GET_ASSERTION_RESPONSE {
    pub WebAuthNAssertion: WEBAUTHN_ASSERTION,
    pub pUserInformation: PCWEBAUTHN_USER_ENTITY_INFORMATION,
    pub dwNumberOfCredentials: u32,
    pub lUserSelected: i32,
    pub cbLargeBlobKey: u32,
    pub pbLargeBlobKey: super::minwindef::PBYTE,
    pub cbUnsignedExtensionOutputs: u32,
    pub pbUnsignedExtensionOutputs: super::minwindef::PBYTE,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EXPERIMENTAL_WEBAUTHN_CTAPCBOR_HMAC_SALT_EXTENSION {
    pub dwVersion: u32,
    pub pKeyAgreement: EXPERIMENTAL_PWEBAUTHN_CTAPCBOR_ECC_PUBLIC_KEY,
    pub cbEncryptedSalt: u32,
    pub pbEncryptedSalt: super::minwindef::PBYTE,
    pub cbSaltAuth: u32,
    pub pbSaltAuth: super::minwindef::PBYTE,
}
pub const EXPERIMENTAL_WEBAUTHN_CTAPCBOR_HMAC_SALT_EXTENSION_CURRENT_VERSION: u32 = 1;
pub const EXPERIMENTAL_WEBAUTHN_CTAPCBOR_HMAC_SALT_EXTENSION_VERSION_1: u32 = 1;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EXPERIMENTAL_WEBAUTHN_CTAPCBOR_MAKE_CREDENTIAL_REQUEST {
    pub dwVersion: u32,
    pub cbRpId: u32,
    pub pbRpId: super::minwindef::PBYTE,
    pub cbClientDataHash: u32,
    pub pbClientDataHash: super::minwindef::PBYTE,
    pub pRpInformation: PCWEBAUTHN_RP_ENTITY_INFORMATION,
    pub pUserInformation: PCWEBAUTHN_USER_ENTITY_INFORMATION,
    pub WebAuthNCredentialParameters: WEBAUTHN_COSE_CREDENTIAL_PARAMETERS,
    pub CredentialList: WEBAUTHN_CREDENTIAL_LIST,
    pub cbCborExtensionsMap: u32,
    pub pbCborExtensionsMap: super::minwindef::PBYTE,
    pub pAuthenticatorOptions: EXPERIMENTAL_PWEBAUTHN_CTAPCBOR_AUTHENTICATOR_OPTIONS,
    pub fEmptyPinAuth: windows_core::BOOL,
    pub cbPinAuth: u32,
    pub pbPinAuth: super::minwindef::PBYTE,
    pub lHmacSecretExt: i32,
    pub pHmacSecretMcExtension: EXPERIMENTAL_PWEBAUTHN_CTAPCBOR_HMAC_SALT_EXTENSION,
    pub lPrfExt: i32,
    pub cbHmacSecretSaltValues: u32,
    pub pbHmacSecretSaltValues: super::minwindef::PBYTE,
    pub dwCredProtect: u32,
    pub dwPinProtocol: u32,
    pub dwEnterpriseAttestation: u32,
    pub cbCredBlobExt: u32,
    pub pbCredBlobExt: super::minwindef::PBYTE,
    pub lLargeBlobKeyExt: i32,
    pub dwLargeBlobSupport: u32,
    pub lMinPinLengthExt: i32,
    pub cbJsonExt: u32,
    pub pbJsonExt: super::minwindef::PBYTE,
}
pub const EXPERIMENTAL_WEBAUTHN_CTAPCBOR_MAKE_CREDENTIAL_REQUEST_CURRENT_VERSION: u32 = 1;
pub const EXPERIMENTAL_WEBAUTHN_CTAPCBOR_MAKE_CREDENTIAL_REQUEST_VERSION_1: u32 = 1;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EXPERIMENTAL_WEBAUTHN_PLUGIN_ADD_AUTHENTICATOR_OPTIONS {
    pub pwszAuthenticatorName: windows_core::PCWSTR,
    pub pwszPluginClsId: windows_core::PCWSTR,
    pub pwszPluginRpId: windows_core::PCWSTR,
    pub pwszLightThemeLogo: windows_core::PCWSTR,
    pub pwszDarkThemeLogo: windows_core::PCWSTR,
    pub cbAuthenticatorInfo: u32,
    pub pbAuthenticatorInfo: super::minwindef::PBYTE,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EXPERIMENTAL_WEBAUTHN_PLUGIN_ADD_AUTHENTICATOR_RESPONSE {
    pub cbOpSignPubKey: u32,
    pub pbOpSignPubKey: super::minwindef::PBYTE,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EXPERIMENTAL_WEBAUTHN_PLUGIN_CREDENTIAL_DETAILS {
    pub cbCredentialId: u32,
    pub pbCredentialId: super::minwindef::PBYTE,
    pub pwszRpId: windows_core::PWSTR,
    pub pwszRpName: windows_core::PWSTR,
    pub cbUserId: u32,
    pub pbUserId: super::minwindef::PBYTE,
    pub pwszUserName: windows_core::PWSTR,
    pub pwszUserDisplayName: windows_core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EXPERIMENTAL_WEBAUTHN_PLUGIN_CREDENTIAL_DETAILS_LIST {
    pub pwszPluginClsId: windows_core::PWSTR,
    pub cCredentialDetails: u32,
    pub pCredentialDetails: *mut EXPERIMENTAL_PWEBAUTHN_PLUGIN_CREDENTIAL_DETAILS,
}
#[cfg(feature = "minwindef")]
impl Default for EXPERIMENTAL_WEBAUTHN_PLUGIN_CREDENTIAL_DETAILS_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EXPERIMENTAL_WEBAUTHN_PLUGIN_PERFORM_UV {
    pub hwnd: super::windef::HWND,
    pub transactionId: *mut windows_core::GUID,
    pub r#type: EXPERIMENTAL_WEBAUTHN_PLUGIN_PERFORM_UV_OPERATION_TYPE,
    pub pwszUsername: windows_core::PCWSTR,
    pub pwszContext: windows_core::PCWSTR,
}
#[cfg(feature = "windef")]
impl Default for EXPERIMENTAL_WEBAUTHN_PLUGIN_PERFORM_UV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type EXPERIMENTAL_WEBAUTHN_PLUGIN_PERFORM_UV_OPERATION_TYPE = i32;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EXPERIMENTAL_WEBAUTHN_PLUGIN_PERFORM_UV_RESPONSE {
    pub cbResponse: u32,
    pub pbResponse: super::minwindef::PBYTE,
}
pub type EXPERIMENTAL_WEBAUTHN_PLUGIN_STATUS_CHANGE_CALLBACK = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void)>;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EXPERIMENTAL_WEBAUTHN_PLUGIN_UPDATE_AUTHENTICATOR_DETAILS {
    pub pwszAuthenticatorName: windows_core::PCWSTR,
    pub pwszPluginClsId: windows_core::PCWSTR,
    pub pwszNewPluginClsId: windows_core::PCWSTR,
    pub pwszLightThemeLogo: windows_core::PCWSTR,
    pub pwszDarkThemeLogo: windows_core::PCWSTR,
    pub cbAuthenticatorInfo: u32,
    pub pbAuthenticatorInfo: super::minwindef::PBYTE,
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
pub const WEBAUTHN_API_CURRENT_VERSION: u32 = 9;
pub const WEBAUTHN_API_VERSION_1: u32 = 1;
pub const WEBAUTHN_API_VERSION_2: u32 = 2;
pub const WEBAUTHN_API_VERSION_3: u32 = 3;
pub const WEBAUTHN_API_VERSION_4: u32 = 4;
pub const WEBAUTHN_API_VERSION_5: u32 = 5;
pub const WEBAUTHN_API_VERSION_6: u32 = 6;
pub const WEBAUTHN_API_VERSION_7: u32 = 7;
pub const WEBAUTHN_API_VERSION_8: u32 = 8;
pub const WEBAUTHN_API_VERSION_9: u32 = 9;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WEBAUTHN_ASSERTION {
    pub dwVersion: u32,
    pub cbAuthenticatorData: u32,
    pub pbAuthenticatorData: super::minwindef::PBYTE,
    pub cbSignature: u32,
    pub pbSignature: super::minwindef::PBYTE,
    pub Credential: WEBAUTHN_CREDENTIAL,
    pub cbUserId: u32,
    pub pbUserId: super::minwindef::PBYTE,
    pub Extensions: WEBAUTHN_EXTENSIONS,
    pub cbCredLargeBlob: u32,
    pub pbCredLargeBlob: super::minwindef::PBYTE,
    pub dwCredLargeBlobStatus: u32,
    pub pHmacSecret: PWEBAUTHN_HMAC_SECRET_SALT,
    pub dwUsedTransport: u32,
    pub cbUnsignedExtensionOutputs: u32,
    pub pbUnsignedExtensionOutputs: super::minwindef::PBYTE,
    pub cbClientDataJSON: u32,
    pub pbClientDataJSON: super::minwindef::PBYTE,
    pub cbAuthenticationResponseJSON: u32,
    pub pbAuthenticationResponseJSON: super::minwindef::PBYTE,
}
pub const WEBAUTHN_ASSERTION_CURRENT_VERSION: u32 = 6;
pub const WEBAUTHN_ASSERTION_VERSION_1: u32 = 1;
pub const WEBAUTHN_ASSERTION_VERSION_2: u32 = 2;
pub const WEBAUTHN_ASSERTION_VERSION_3: u32 = 3;
pub const WEBAUTHN_ASSERTION_VERSION_4: u32 = 4;
pub const WEBAUTHN_ASSERTION_VERSION_5: u32 = 5;
pub const WEBAUTHN_ASSERTION_VERSION_6: u32 = 6;
pub const WEBAUTHN_ATTESTATION_CONVEYANCE_PREFERENCE_ANY: u32 = 0;
pub const WEBAUTHN_ATTESTATION_CONVEYANCE_PREFERENCE_DIRECT: u32 = 3;
pub const WEBAUTHN_ATTESTATION_CONVEYANCE_PREFERENCE_INDIRECT: u32 = 2;
pub const WEBAUTHN_ATTESTATION_CONVEYANCE_PREFERENCE_NONE: u32 = 1;
pub const WEBAUTHN_ATTESTATION_DECODE_COMMON: u32 = 1;
pub const WEBAUTHN_ATTESTATION_DECODE_NONE: u32 = 0;
pub const WEBAUTHN_ATTESTATION_TYPE_NONE: windows_core::PCWSTR = windows_core::w!("none");
pub const WEBAUTHN_ATTESTATION_TYPE_PACKED: windows_core::PCWSTR = windows_core::w!("packed");
pub const WEBAUTHN_ATTESTATION_TYPE_TPM: windows_core::PCWSTR = windows_core::w!("tpm");
pub const WEBAUTHN_ATTESTATION_TYPE_U2F: windows_core::PCWSTR = windows_core::w!("fido-u2f");
pub const WEBAUTHN_ATTESTATION_VER_TPM_2_0: windows_core::PCWSTR = windows_core::w!("2.0");
pub const WEBAUTHN_AUTHENTICATOR_ATTACHMENT_ANY: u32 = 0;
pub const WEBAUTHN_AUTHENTICATOR_ATTACHMENT_CROSS_PLATFORM: u32 = 2;
pub const WEBAUTHN_AUTHENTICATOR_ATTACHMENT_CROSS_PLATFORM_U2F_V2: u32 = 3;
pub const WEBAUTHN_AUTHENTICATOR_ATTACHMENT_PLATFORM: u32 = 1;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WEBAUTHN_AUTHENTICATOR_DETAILS {
    pub dwVersion: u32,
    pub cbAuthenticatorId: u32,
    pub pbAuthenticatorId: super::minwindef::PBYTE,
    pub pwszAuthenticatorName: windows_core::PCWSTR,
    pub cbAuthenticatorLogo: u32,
    pub pbAuthenticatorLogo: super::minwindef::PBYTE,
    pub bLocked: windows_core::BOOL,
}
pub const WEBAUTHN_AUTHENTICATOR_DETAILS_CURRENT_VERSION: u32 = 1;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WEBAUTHN_AUTHENTICATOR_DETAILS_LIST {
    pub cAuthenticatorDetails: u32,
    pub ppAuthenticatorDetails: *mut PWEBAUTHN_AUTHENTICATOR_DETAILS,
}
#[cfg(feature = "minwindef")]
impl Default for WEBAUTHN_AUTHENTICATOR_DETAILS_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WEBAUTHN_AUTHENTICATOR_DETAILS_OPTIONS {
    pub dwVersion: u32,
}
pub const WEBAUTHN_AUTHENTICATOR_DETAILS_OPTIONS_CURRENT_VERSION: u32 = 1;
pub const WEBAUTHN_AUTHENTICATOR_DETAILS_OPTIONS_VERSION_1: u32 = 1;
pub const WEBAUTHN_AUTHENTICATOR_DETAILS_VERSION_1: u32 = 1;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS {
    pub dwVersion: u32,
    pub dwTimeoutMilliseconds: u32,
    pub CredentialList: WEBAUTHN_CREDENTIALS,
    pub Extensions: WEBAUTHN_EXTENSIONS,
    pub dwAuthenticatorAttachment: u32,
    pub dwUserVerificationRequirement: u32,
    pub dwFlags: u32,
    pub pwszU2fAppId: windows_core::PCWSTR,
    pub pbU2fAppId: *mut windows_core::BOOL,
    pub pCancellationId: *mut windows_core::GUID,
    pub pAllowCredentialList: PWEBAUTHN_CREDENTIAL_LIST,
    pub dwCredLargeBlobOperation: u32,
    pub cbCredLargeBlob: u32,
    pub pbCredLargeBlob: super::minwindef::PBYTE,
    pub pHmacSecretSaltValues: PWEBAUTHN_HMAC_SECRET_SALT_VALUES,
    pub bBrowserInPrivateMode: windows_core::BOOL,
    pub pLinkedDevice: PCTAPCBOR_HYBRID_STORAGE_LINKED_DATA,
    pub bAutoFill: windows_core::BOOL,
    pub cbJsonExt: u32,
    pub pbJsonExt: super::minwindef::PBYTE,
    pub cCredentialHints: u32,
    pub ppwszCredentialHints: *mut windows_core::PCWSTR,
    pub pwszRemoteWebOrigin: windows_core::PCWSTR,
    pub cbPublicKeyCredentialRequestOptionsJSON: u32,
    pub pbPublicKeyCredentialRequestOptionsJSON: super::minwindef::PBYTE,
    pub cbAuthenticatorId: u32,
    pub pbAuthenticatorId: super::minwindef::PBYTE,
}
#[cfg(feature = "minwindef")]
impl Default for WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_CURRENT_VERSION: u32 = 9;
pub const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_VERSION_1: u32 = 1;
pub const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_VERSION_2: u32 = 2;
pub const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_VERSION_3: u32 = 3;
pub const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_VERSION_4: u32 = 4;
pub const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_VERSION_5: u32 = 5;
pub const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_VERSION_6: u32 = 6;
pub const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_VERSION_7: u32 = 7;
pub const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_VERSION_8: u32 = 8;
pub const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS_VERSION_9: u32 = 9;
pub const WEBAUTHN_AUTHENTICATOR_HMAC_SECRET_VALUES_FLAG: u32 = 1048576;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS {
    pub dwVersion: u32,
    pub dwTimeoutMilliseconds: u32,
    pub CredentialList: WEBAUTHN_CREDENTIALS,
    pub Extensions: WEBAUTHN_EXTENSIONS,
    pub dwAuthenticatorAttachment: u32,
    pub bRequireResidentKey: windows_core::BOOL,
    pub dwUserVerificationRequirement: u32,
    pub dwAttestationConveyancePreference: u32,
    pub dwFlags: u32,
    pub pCancellationId: *mut windows_core::GUID,
    pub pExcludeCredentialList: PWEBAUTHN_CREDENTIAL_LIST,
    pub dwEnterpriseAttestation: u32,
    pub dwLargeBlobSupport: u32,
    pub bPreferResidentKey: windows_core::BOOL,
    pub bBrowserInPrivateMode: windows_core::BOOL,
    pub bEnablePrf: windows_core::BOOL,
    pub pLinkedDevice: PCTAPCBOR_HYBRID_STORAGE_LINKED_DATA,
    pub cbJsonExt: u32,
    pub pbJsonExt: super::minwindef::PBYTE,
    pub pPRFGlobalEval: PWEBAUTHN_HMAC_SECRET_SALT,
    pub cCredentialHints: u32,
    pub ppwszCredentialHints: *mut windows_core::PCWSTR,
    pub bThirdPartyPayment: windows_core::BOOL,
    pub pwszRemoteWebOrigin: windows_core::PCWSTR,
    pub cbPublicKeyCredentialCreationOptionsJSON: u32,
    pub pbPublicKeyCredentialCreationOptionsJSON: super::minwindef::PBYTE,
    pub cbAuthenticatorId: u32,
    pub pbAuthenticatorId: super::minwindef::PBYTE,
}
#[cfg(feature = "minwindef")]
impl Default for WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS_CURRENT_VERSION: u32 = 9;
pub const WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS_VERSION_1: u32 = 1;
pub const WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS_VERSION_2: u32 = 2;
pub const WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS_VERSION_3: u32 = 3;
pub const WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS_VERSION_4: u32 = 4;
pub const WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS_VERSION_5: u32 = 5;
pub const WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS_VERSION_6: u32 = 6;
pub const WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS_VERSION_7: u32 = 7;
pub const WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS_VERSION_8: u32 = 8;
pub const WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS_VERSION_9: u32 = 9;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WEBAUTHN_CLIENT_DATA {
    pub dwVersion: u32,
    pub cbClientDataJSON: u32,
    pub pbClientDataJSON: super::minwindef::PBYTE,
    pub pwszHashAlgId: windows_core::PCWSTR,
}
pub const WEBAUTHN_CLIENT_DATA_CURRENT_VERSION: u32 = 1;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WEBAUTHN_COMMON_ATTESTATION {
    pub dwVersion: u32,
    pub pwszAlg: windows_core::PCWSTR,
    pub lAlg: i32,
    pub cbSignature: u32,
    pub pbSignature: super::minwindef::PBYTE,
    pub cX5c: u32,
    pub pX5c: PWEBAUTHN_X5C,
    pub pwszVer: windows_core::PCWSTR,
    pub cbCertInfo: u32,
    pub pbCertInfo: super::minwindef::PBYTE,
    pub cbPubArea: u32,
    pub pbPubArea: super::minwindef::PBYTE,
}
pub const WEBAUTHN_COMMON_ATTESTATION_CURRENT_VERSION: u32 = 1;
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WEBAUTHN_COSE_CREDENTIAL_PARAMETER {
    pub dwVersion: u32,
    pub pwszCredentialType: windows_core::PCWSTR,
    pub lAlg: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WEBAUTHN_COSE_CREDENTIAL_PARAMETERS {
    pub cCredentialParameters: u32,
    pub pCredentialParameters: PWEBAUTHN_COSE_CREDENTIAL_PARAMETER,
}
pub const WEBAUTHN_COSE_CREDENTIAL_PARAMETER_CURRENT_VERSION: u32 = 1;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WEBAUTHN_CREDENTIAL {
    pub dwVersion: u32,
    pub cbId: u32,
    pub pbId: super::minwindef::PBYTE,
    pub pwszCredentialType: windows_core::PCWSTR,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WEBAUTHN_CREDENTIALS {
    pub cCredentials: u32,
    pub pCredentials: PWEBAUTHN_CREDENTIAL,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WEBAUTHN_CREDENTIAL_ATTESTATION {
    pub dwVersion: u32,
    pub pwszFormatType: windows_core::PCWSTR,
    pub cbAuthenticatorData: u32,
    pub pbAuthenticatorData: super::minwindef::PBYTE,
    pub cbAttestation: u32,
    pub pbAttestation: super::minwindef::PBYTE,
    pub dwAttestationDecodeType: u32,
    pub pvAttestationDecode: *mut core::ffi::c_void,
    pub cbAttestationObject: u32,
    pub pbAttestationObject: super::minwindef::PBYTE,
    pub cbCredentialId: u32,
    pub pbCredentialId: super::minwindef::PBYTE,
    pub Extensions: WEBAUTHN_EXTENSIONS,
    pub dwUsedTransport: u32,
    pub bEpAtt: windows_core::BOOL,
    pub bLargeBlobSupported: windows_core::BOOL,
    pub bResidentKey: windows_core::BOOL,
    pub bPrfEnabled: windows_core::BOOL,
    pub cbUnsignedExtensionOutputs: u32,
    pub pbUnsignedExtensionOutputs: super::minwindef::PBYTE,
    pub pHmacSecret: PWEBAUTHN_HMAC_SECRET_SALT,
    pub bThirdPartyPayment: windows_core::BOOL,
    pub dwTransports: u32,
    pub cbClientDataJSON: u32,
    pub pbClientDataJSON: super::minwindef::PBYTE,
    pub cbRegistrationResponseJSON: u32,
    pub pbRegistrationResponseJSON: super::minwindef::PBYTE,
}
#[cfg(feature = "minwindef")]
impl Default for WEBAUTHN_CREDENTIAL_ATTESTATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WEBAUTHN_CREDENTIAL_ATTESTATION_CURRENT_VERSION: u32 = 8;
pub const WEBAUTHN_CREDENTIAL_ATTESTATION_VERSION_1: u32 = 1;
pub const WEBAUTHN_CREDENTIAL_ATTESTATION_VERSION_2: u32 = 2;
pub const WEBAUTHN_CREDENTIAL_ATTESTATION_VERSION_3: u32 = 3;
pub const WEBAUTHN_CREDENTIAL_ATTESTATION_VERSION_4: u32 = 4;
pub const WEBAUTHN_CREDENTIAL_ATTESTATION_VERSION_5: u32 = 5;
pub const WEBAUTHN_CREDENTIAL_ATTESTATION_VERSION_6: u32 = 6;
pub const WEBAUTHN_CREDENTIAL_ATTESTATION_VERSION_7: u32 = 7;
pub const WEBAUTHN_CREDENTIAL_ATTESTATION_VERSION_8: u32 = 8;
pub const WEBAUTHN_CREDENTIAL_CURRENT_VERSION: u32 = 1;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WEBAUTHN_CREDENTIAL_DETAILS {
    pub dwVersion: u32,
    pub cbCredentialID: u32,
    pub pbCredentialID: super::minwindef::PBYTE,
    pub pRpInformation: PWEBAUTHN_RP_ENTITY_INFORMATION,
    pub pUserInformation: PWEBAUTHN_USER_ENTITY_INFORMATION,
    pub bRemovable: windows_core::BOOL,
    pub bBackedUp: windows_core::BOOL,
    pub pwszAuthenticatorName: windows_core::PCWSTR,
    pub cbAuthenticatorLogo: u32,
    pub pbAuthenticatorLogo: super::minwindef::PBYTE,
    pub bThirdPartyPayment: windows_core::BOOL,
    pub dwTransports: u32,
}
pub const WEBAUTHN_CREDENTIAL_DETAILS_CURRENT_VERSION: u32 = 4;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WEBAUTHN_CREDENTIAL_DETAILS_LIST {
    pub cCredentialDetails: u32,
    pub ppCredentialDetails: *mut PWEBAUTHN_CREDENTIAL_DETAILS,
}
#[cfg(feature = "minwindef")]
impl Default for WEBAUTHN_CREDENTIAL_DETAILS_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WEBAUTHN_CREDENTIAL_DETAILS_VERSION_1: u32 = 1;
pub const WEBAUTHN_CREDENTIAL_DETAILS_VERSION_2: u32 = 2;
pub const WEBAUTHN_CREDENTIAL_DETAILS_VERSION_3: u32 = 3;
pub const WEBAUTHN_CREDENTIAL_DETAILS_VERSION_4: u32 = 4;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WEBAUTHN_CREDENTIAL_EX {
    pub dwVersion: u32,
    pub cbId: u32,
    pub pbId: super::minwindef::PBYTE,
    pub pwszCredentialType: windows_core::PCWSTR,
    pub dwTransports: u32,
}
pub const WEBAUTHN_CREDENTIAL_EX_CURRENT_VERSION: u32 = 1;
pub const WEBAUTHN_CREDENTIAL_HINT_CLIENT_DEVICE: windows_core::PCWSTR = windows_core::w!("client-device");
pub const WEBAUTHN_CREDENTIAL_HINT_HYBRID: windows_core::PCWSTR = windows_core::w!("hybrid");
pub const WEBAUTHN_CREDENTIAL_HINT_SECURITY_KEY: windows_core::PCWSTR = windows_core::w!("security-key");
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WEBAUTHN_CREDENTIAL_LIST {
    pub cCredentials: u32,
    pub ppCredentials: *mut PWEBAUTHN_CREDENTIAL_EX,
}
#[cfg(feature = "minwindef")]
impl Default for WEBAUTHN_CREDENTIAL_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WEBAUTHN_CREDENTIAL_TYPE_PUBLIC_KEY: windows_core::PCWSTR = windows_core::w!("public-key");
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WEBAUTHN_CRED_BLOB_EXTENSION {
    pub cbCredBlob: u32,
    pub pbCredBlob: super::minwindef::PBYTE,
}
pub const WEBAUTHN_CRED_LARGE_BLOB_OPERATION_DELETE: u32 = 3;
pub const WEBAUTHN_CRED_LARGE_BLOB_OPERATION_GET: u32 = 1;
pub const WEBAUTHN_CRED_LARGE_BLOB_OPERATION_NONE: u32 = 0;
pub const WEBAUTHN_CRED_LARGE_BLOB_OPERATION_SET: u32 = 2;
pub const WEBAUTHN_CRED_LARGE_BLOB_STATUS_AUTHENTICATOR_ERROR: u32 = 9;
pub const WEBAUTHN_CRED_LARGE_BLOB_STATUS_INVALID_DATA: u32 = 3;
pub const WEBAUTHN_CRED_LARGE_BLOB_STATUS_INVALID_PARAMETER: u32 = 4;
pub const WEBAUTHN_CRED_LARGE_BLOB_STATUS_LACK_OF_SPACE: u32 = 7;
pub const WEBAUTHN_CRED_LARGE_BLOB_STATUS_MULTIPLE_CREDENTIALS: u32 = 6;
pub const WEBAUTHN_CRED_LARGE_BLOB_STATUS_NONE: u32 = 0;
pub const WEBAUTHN_CRED_LARGE_BLOB_STATUS_NOT_FOUND: u32 = 5;
pub const WEBAUTHN_CRED_LARGE_BLOB_STATUS_NOT_SUPPORTED: u32 = 2;
pub const WEBAUTHN_CRED_LARGE_BLOB_STATUS_PLATFORM_ERROR: u32 = 8;
pub const WEBAUTHN_CRED_LARGE_BLOB_STATUS_SUCCESS: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WEBAUTHN_CRED_PROTECT_EXTENSION_IN {
    pub dwCredProtect: u32,
    pub bRequireCredProtect: windows_core::BOOL,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WEBAUTHN_CRED_WITH_HMAC_SECRET_SALT {
    pub cbCredID: u32,
    pub pbCredID: super::minwindef::PBYTE,
    pub pHmacSecretSalt: PWEBAUTHN_HMAC_SECRET_SALT,
}
pub const WEBAUTHN_CTAP_ONE_HMAC_SECRET_LENGTH: u32 = 32;
pub const WEBAUTHN_CTAP_TRANSPORT_BLE: u32 = 4;
pub const WEBAUTHN_CTAP_TRANSPORT_BLE_STRING: windows_core::PCSTR = windows_core::s!("ble");
pub const WEBAUTHN_CTAP_TRANSPORT_FLAGS_MASK: u32 = 127;
pub const WEBAUTHN_CTAP_TRANSPORT_HYBRID: u32 = 32;
pub const WEBAUTHN_CTAP_TRANSPORT_HYBRID_STRING: windows_core::PCSTR = windows_core::s!("hybrid");
pub const WEBAUTHN_CTAP_TRANSPORT_INTERNAL: u32 = 16;
pub const WEBAUTHN_CTAP_TRANSPORT_INTERNAL_STRING: windows_core::PCSTR = windows_core::s!("internal");
pub const WEBAUTHN_CTAP_TRANSPORT_NFC: u32 = 2;
pub const WEBAUTHN_CTAP_TRANSPORT_NFC_STRING: windows_core::PCSTR = windows_core::s!("nfc");
pub const WEBAUTHN_CTAP_TRANSPORT_SMART_CARD: u32 = 64;
pub const WEBAUTHN_CTAP_TRANSPORT_SMART_CARD_STRING: windows_core::PCSTR = windows_core::s!("smart-card");
pub const WEBAUTHN_CTAP_TRANSPORT_TEST: u32 = 8;
pub const WEBAUTHN_CTAP_TRANSPORT_USB: u32 = 1;
pub const WEBAUTHN_CTAP_TRANSPORT_USB_STRING: windows_core::PCSTR = windows_core::s!("usb");
pub const WEBAUTHN_ENTERPRISE_ATTESTATION_NONE: u32 = 0;
pub const WEBAUTHN_ENTERPRISE_ATTESTATION_PLATFORM_MANAGED: u32 = 2;
pub const WEBAUTHN_ENTERPRISE_ATTESTATION_VENDOR_FACILITATED: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WEBAUTHN_EXTENSION {
    pub pwszExtensionIdentifier: windows_core::PCWSTR,
    pub cbExtension: u32,
    pub pvExtension: *mut core::ffi::c_void,
}
impl Default for WEBAUTHN_EXTENSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WEBAUTHN_EXTENSIONS {
    pub cExtensions: u32,
    pub pExtensions: PWEBAUTHN_EXTENSION,
}
pub const WEBAUTHN_EXTENSIONS_IDENTIFIER_CRED_BLOB: windows_core::PCWSTR = windows_core::w!("credBlob");
pub const WEBAUTHN_EXTENSIONS_IDENTIFIER_CRED_PROTECT: windows_core::PCWSTR = windows_core::w!("credProtect");
pub const WEBAUTHN_EXTENSIONS_IDENTIFIER_HMAC_SECRET: windows_core::PCWSTR = windows_core::w!("hmac-secret");
pub const WEBAUTHN_EXTENSIONS_IDENTIFIER_MIN_PIN_LENGTH: windows_core::PCWSTR = windows_core::w!("minPinLength");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WEBAUTHN_GET_CREDENTIALS_OPTIONS {
    pub dwVersion: u32,
    pub pwszRpId: windows_core::PCWSTR,
    pub bBrowserInPrivateMode: windows_core::BOOL,
}
pub const WEBAUTHN_GET_CREDENTIALS_OPTIONS_CURRENT_VERSION: u32 = 1;
pub const WEBAUTHN_GET_CREDENTIALS_OPTIONS_VERSION_1: u32 = 1;
pub const WEBAUTHN_HASH_ALGORITHM_SHA_256: windows_core::PCWSTR = windows_core::w!("SHA-256");
pub const WEBAUTHN_HASH_ALGORITHM_SHA_384: windows_core::PCWSTR = windows_core::w!("SHA-384");
pub const WEBAUTHN_HASH_ALGORITHM_SHA_512: windows_core::PCWSTR = windows_core::w!("SHA-512");
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WEBAUTHN_HMAC_SECRET_SALT {
    pub cbFirst: u32,
    pub pbFirst: super::minwindef::PBYTE,
    pub cbSecond: u32,
    pub pbSecond: super::minwindef::PBYTE,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WEBAUTHN_HMAC_SECRET_SALT_VALUES {
    pub pGlobalHmacSalt: PWEBAUTHN_HMAC_SECRET_SALT,
    pub cCredWithHmacSecretSaltList: u32,
    pub pCredWithHmacSecretSaltList: PWEBAUTHN_CRED_WITH_HMAC_SECRET_SALT,
}
pub const WEBAUTHN_LARGE_BLOB_SUPPORT_NONE: u32 = 0;
pub const WEBAUTHN_LARGE_BLOB_SUPPORT_PREFERRED: u32 = 2;
pub const WEBAUTHN_LARGE_BLOB_SUPPORT_REQUIRED: u32 = 1;
pub const WEBAUTHN_MAX_USER_ID_LENGTH: u32 = 64;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WEBAUTHN_RP_ENTITY_INFORMATION {
    pub dwVersion: u32,
    pub pwszId: windows_core::PCWSTR,
    pub pwszName: windows_core::PCWSTR,
    pub pwszIcon: windows_core::PCWSTR,
}
pub const WEBAUTHN_RP_ENTITY_INFORMATION_CURRENT_VERSION: u32 = 1;
pub const WEBAUTHN_RP_ENTITY_INFORMATION_VERSION_1: u32 = 1;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WEBAUTHN_USER_ENTITY_INFORMATION {
    pub dwVersion: u32,
    pub cbId: u32,
    pub pbId: super::minwindef::PBYTE,
    pub pwszName: windows_core::PCWSTR,
    pub pwszIcon: windows_core::PCWSTR,
    pub pwszDisplayName: windows_core::PCWSTR,
}
pub const WEBAUTHN_USER_ENTITY_INFORMATION_CURRENT_VERSION: u32 = 1;
pub const WEBAUTHN_USER_ENTITY_INFORMATION_VERSION_1: u32 = 1;
pub const WEBAUTHN_USER_VERIFICATION_ANY: u32 = 0;
pub const WEBAUTHN_USER_VERIFICATION_OPTIONAL: u32 = 1;
pub const WEBAUTHN_USER_VERIFICATION_OPTIONAL_WITH_CREDENTIAL_ID_LIST: u32 = 2;
pub const WEBAUTHN_USER_VERIFICATION_REQUIRED: u32 = 3;
pub const WEBAUTHN_USER_VERIFICATION_REQUIREMENT_ANY: u32 = 0;
pub const WEBAUTHN_USER_VERIFICATION_REQUIREMENT_DISCOURAGED: u32 = 3;
pub const WEBAUTHN_USER_VERIFICATION_REQUIREMENT_PREFERRED: u32 = 2;
pub const WEBAUTHN_USER_VERIFICATION_REQUIREMENT_REQUIRED: u32 = 1;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WEBAUTHN_X5C {
    pub cbData: u32,
    pub pbData: super::minwindef::PBYTE,
}
