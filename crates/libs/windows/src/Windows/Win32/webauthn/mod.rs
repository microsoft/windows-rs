#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn WebAuthNAuthenticatorGetAssertion<P1>(hwnd: super::HWND, pwszrpid: P1, pwebauthnclientdata: PCWEBAUTHN_CLIENT_DATA, pwebauthngetassertionoptions: Option<PCWEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS>) -> windows_core::Result<PWEBAUTHN_ASSERTION>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("webauthn.dll" "system" fn WebAuthNAuthenticatorGetAssertion(hwnd : super::HWND, pwszrpid : windows_core::PCWSTR, pwebauthnclientdata : PCWEBAUTHN_CLIENT_DATA, pwebauthngetassertionoptions : PCWEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS, ppwebauthnassertion : *mut PWEBAUTHN_ASSERTION) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WebAuthNAuthenticatorGetAssertion(hwnd, pwszrpid.param().abi(), pwebauthnclientdata, pwebauthngetassertionoptions.unwrap_or(core::mem::zeroed()) as _, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn WebAuthNAuthenticatorMakeCredential(hwnd: super::HWND, prpinformation: PCWEBAUTHN_RP_ENTITY_INFORMATION, puserinformation: PCWEBAUTHN_USER_ENTITY_INFORMATION, ppubkeycredparams: PCWEBAUTHN_COSE_CREDENTIAL_PARAMETERS, pwebauthnclientdata: PCWEBAUTHN_CLIENT_DATA, pwebauthnmakecredentialoptions: Option<PCWEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS>) -> windows_core::Result<PWEBAUTHN_CREDENTIAL_ATTESTATION> {
    windows_core::link!("webauthn.dll" "system" fn WebAuthNAuthenticatorMakeCredential(hwnd : super::HWND, prpinformation : PCWEBAUTHN_RP_ENTITY_INFORMATION, puserinformation : PCWEBAUTHN_USER_ENTITY_INFORMATION, ppubkeycredparams : PCWEBAUTHN_COSE_CREDENTIAL_PARAMETERS, pwebauthnclientdata : PCWEBAUTHN_CLIENT_DATA, pwebauthnmakecredentialoptions : PCWEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS, ppwebauthncredentialattestation : *mut PWEBAUTHN_CREDENTIAL_ATTESTATION) -> windows_core::HRESULT);
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
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn WebAuthNDeletePlatformCredential(pbcredentialid: &[u8]) -> windows_core::HRESULT {
    windows_core::link!("webauthn.dll" "system" fn WebAuthNDeletePlatformCredential(cbcredentialid : super::DWORD, pbcredentialid : *const super::BYTE) -> windows_core::HRESULT);
    unsafe { WebAuthNDeletePlatformCredential(pbcredentialid.len().try_into().unwrap(), core::mem::transmute(pbcredentialid.as_ptr())) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn WebAuthNFreeAssertion(pwebauthnassertion: PWEBAUTHN_ASSERTION) {
    windows_core::link!("webauthn.dll" "system" fn WebAuthNFreeAssertion(pwebauthnassertion : PWEBAUTHN_ASSERTION));
    unsafe { WebAuthNFreeAssertion(pwebauthnassertion) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn WebAuthNFreeAuthenticatorList(pauthenticatordetailslist: PWEBAUTHN_AUTHENTICATOR_DETAILS_LIST) {
    windows_core::link!("webauthn.dll" "system" fn WebAuthNFreeAuthenticatorList(pauthenticatordetailslist : PWEBAUTHN_AUTHENTICATOR_DETAILS_LIST));
    unsafe { WebAuthNFreeAuthenticatorList(pauthenticatordetailslist) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn WebAuthNFreeCredentialAttestation(pwebauthncredentialattestation: Option<PWEBAUTHN_CREDENTIAL_ATTESTATION>) {
    windows_core::link!("webauthn.dll" "system" fn WebAuthNFreeCredentialAttestation(pwebauthncredentialattestation : PWEBAUTHN_CREDENTIAL_ATTESTATION));
    unsafe { WebAuthNFreeCredentialAttestation(pwebauthncredentialattestation.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn WebAuthNFreePlatformCredentialList(pcredentialdetailslist: PWEBAUTHN_CREDENTIAL_DETAILS_LIST) {
    windows_core::link!("webauthn.dll" "system" fn WebAuthNFreePlatformCredentialList(pcredentialdetailslist : PWEBAUTHN_CREDENTIAL_DETAILS_LIST));
    unsafe { WebAuthNFreePlatformCredentialList(pcredentialdetailslist) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn WebAuthNGetApiVersionNumber() -> super::DWORD {
    windows_core::link!("webauthn.dll" "system" fn WebAuthNGetApiVersionNumber() -> super::DWORD);
    unsafe { WebAuthNGetApiVersionNumber() }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn WebAuthNGetAuthenticatorList(pwebauthngetauthenticatorlistoptions: Option<PCWEBAUTHN_AUTHENTICATOR_DETAILS_OPTIONS>) -> windows_core::Result<PWEBAUTHN_AUTHENTICATOR_DETAILS_LIST> {
    windows_core::link!("webauthn.dll" "system" fn WebAuthNGetAuthenticatorList(pwebauthngetauthenticatorlistoptions : PCWEBAUTHN_AUTHENTICATOR_DETAILS_OPTIONS, ppauthenticatordetailslist : *mut PWEBAUTHN_AUTHENTICATOR_DETAILS_LIST) -> windows_core::HRESULT);
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
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn WebAuthNGetPlatformCredentialList(pgetcredentialsoptions: PCWEBAUTHN_GET_CREDENTIALS_OPTIONS) -> windows_core::Result<PWEBAUTHN_CREDENTIAL_DETAILS_LIST> {
    windows_core::link!("webauthn.dll" "system" fn WebAuthNGetPlatformCredentialList(pgetcredentialsoptions : PCWEBAUTHN_GET_CREDENTIALS_OPTIONS, ppcredentialdetailslist : *mut PWEBAUTHN_CREDENTIAL_DETAILS_LIST) -> windows_core::HRESULT);
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
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CTAPCBOR_HYBRID_STORAGE_LINKED_DATA {
    pub dwVersion: super::DWORD,
    pub cbContactId: super::DWORD,
    pub pbContactId: super::PBYTE,
    pub cbLinkId: super::DWORD,
    pub pbLinkId: super::PBYTE,
    pub cbLinkSecret: super::DWORD,
    pub pbLinkSecret: super::PBYTE,
    pub cbPublicKey: super::DWORD,
    pub pbPublicKey: super::PBYTE,
    pub pwszAuthenticatorName: windows_core::PCWSTR,
    pub wEncodedTunnelServerDomain: super::WORD,
}
pub const CTAPCBOR_HYBRID_STORAGE_LINKED_DATA_CURRENT_VERSION: i32 = 1;
pub const CTAPCBOR_HYBRID_STORAGE_LINKED_DATA_VERSION_1: i32 = 1;
#[cfg(feature = "shared")]
pub type EXPERIMENTAL_PCWEBAUTHN_CTAPCBOR_AUTHENTICATOR_OPTIONS = *const EXPERIMENTAL_WEBAUTHN_CTAPCBOR_AUTHENTICATOR_OPTIONS;
#[cfg(feature = "shared")]
pub type EXPERIMENTAL_PCWEBAUTHN_CTAPCBOR_ECC_PUBLIC_KEY = *const EXPERIMENTAL_WEBAUTHN_CTAPCBOR_ECC_PUBLIC_KEY;
#[cfg(feature = "shared")]
pub type EXPERIMENTAL_PCWEBAUTHN_CTAPCBOR_GET_ASSERTION_REQUEST = *const EXPERIMENTAL_WEBAUTHN_CTAPCBOR_GET_ASSERTION_REQUEST;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type EXPERIMENTAL_PCWEBAUTHN_CTAPCBOR_GET_ASSERTION_RESPONSE = *const EXPERIMENTAL_WEBAUTHN_CTAPCBOR_GET_ASSERTION_RESPONSE;
#[cfg(feature = "shared")]
pub type EXPERIMENTAL_PCWEBAUTHN_CTAPCBOR_HMAC_SALT_EXTENSION = *const EXPERIMENTAL_WEBAUTHN_CTAPCBOR_HMAC_SALT_EXTENSION;
#[cfg(feature = "shared")]
pub type EXPERIMENTAL_PCWEBAUTHN_CTAPCBOR_MAKE_CREDENTIAL_REQUEST = *const EXPERIMENTAL_WEBAUTHN_CTAPCBOR_MAKE_CREDENTIAL_REQUEST;
#[cfg(feature = "shared")]
pub type EXPERIMENTAL_PCWEBAUTHN_PLUGIN_ADD_AUTHENTICATOR_OPTIONS = *const EXPERIMENTAL_WEBAUTHN_PLUGIN_ADD_AUTHENTICATOR_OPTIONS;
#[cfg(feature = "shared")]
pub type EXPERIMENTAL_PCWEBAUTHN_PLUGIN_ADD_AUTHENTICATOR_RESPONSE = *const EXPERIMENTAL_WEBAUTHN_PLUGIN_ADD_AUTHENTICATOR_RESPONSE;
#[cfg(feature = "shared")]
pub type EXPERIMENTAL_PCWEBAUTHN_PLUGIN_CREDENTIAL_DETAILS = *const EXPERIMENTAL_WEBAUTHN_PLUGIN_CREDENTIAL_DETAILS;
#[cfg(feature = "shared")]
pub type EXPERIMENTAL_PCWEBAUTHN_PLUGIN_CREDENTIAL_DETAILS_LIST = *const EXPERIMENTAL_WEBAUTHN_PLUGIN_CREDENTIAL_DETAILS_LIST;
#[cfg(feature = "windef")]
pub type EXPERIMENTAL_PCWEBAUTHN_PLUGIN_PERFORM_UV = *const EXPERIMENTAL_WEBAUTHN_PLUGIN_PERFORM_UV;
#[cfg(feature = "shared")]
pub type EXPERIMENTAL_PCWEBAUTHN_PLUGIN_PERFORM_UV_RESPONSE = *const EXPERIMENTAL_WEBAUTHN_PLUGIN_PERFORM_UV_RESPONSE;
#[cfg(feature = "shared")]
pub type EXPERIMENTAL_PCWEBAUTHN_PLUGIN_UPDATE_AUTHENTICATOR_DETAILS = *const EXPERIMENTAL_WEBAUTHN_PLUGIN_UPDATE_AUTHENTICATOR_DETAILS;
pub type EXPERIMENTAL_PLUGIN_AUTHENTICATOR_STATE = i32;
#[cfg(feature = "shared")]
pub type EXPERIMENTAL_PWEBAUTHN_CTAPCBOR_AUTHENTICATOR_OPTIONS = *mut EXPERIMENTAL_WEBAUTHN_CTAPCBOR_AUTHENTICATOR_OPTIONS;
#[cfg(feature = "shared")]
pub type EXPERIMENTAL_PWEBAUTHN_CTAPCBOR_ECC_PUBLIC_KEY = *mut EXPERIMENTAL_WEBAUTHN_CTAPCBOR_ECC_PUBLIC_KEY;
#[cfg(feature = "shared")]
pub type EXPERIMENTAL_PWEBAUTHN_CTAPCBOR_GET_ASSERTION_REQUEST = *mut EXPERIMENTAL_WEBAUTHN_CTAPCBOR_GET_ASSERTION_REQUEST;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type EXPERIMENTAL_PWEBAUTHN_CTAPCBOR_GET_ASSERTION_RESPONSE = *mut EXPERIMENTAL_WEBAUTHN_CTAPCBOR_GET_ASSERTION_RESPONSE;
#[cfg(feature = "shared")]
pub type EXPERIMENTAL_PWEBAUTHN_CTAPCBOR_HMAC_SALT_EXTENSION = *mut EXPERIMENTAL_WEBAUTHN_CTAPCBOR_HMAC_SALT_EXTENSION;
#[cfg(feature = "shared")]
pub type EXPERIMENTAL_PWEBAUTHN_CTAPCBOR_MAKE_CREDENTIAL_REQUEST = *mut EXPERIMENTAL_WEBAUTHN_CTAPCBOR_MAKE_CREDENTIAL_REQUEST;
#[cfg(feature = "shared")]
pub type EXPERIMENTAL_PWEBAUTHN_PLUGIN_ADD_AUTHENTICATOR_OPTIONS = *mut EXPERIMENTAL_WEBAUTHN_PLUGIN_ADD_AUTHENTICATOR_OPTIONS;
#[cfg(feature = "shared")]
pub type EXPERIMENTAL_PWEBAUTHN_PLUGIN_ADD_AUTHENTICATOR_RESPONSE = *mut EXPERIMENTAL_WEBAUTHN_PLUGIN_ADD_AUTHENTICATOR_RESPONSE;
#[cfg(feature = "shared")]
pub type EXPERIMENTAL_PWEBAUTHN_PLUGIN_CREDENTIAL_DETAILS = *mut EXPERIMENTAL_WEBAUTHN_PLUGIN_CREDENTIAL_DETAILS;
#[cfg(feature = "shared")]
pub type EXPERIMENTAL_PWEBAUTHN_PLUGIN_CREDENTIAL_DETAILS_LIST = *mut EXPERIMENTAL_WEBAUTHN_PLUGIN_CREDENTIAL_DETAILS_LIST;
#[cfg(feature = "shared")]
pub type EXPERIMENTAL_PWEBAUTHN_PLUGIN_PERFORM_UV_RESPONSE = *mut EXPERIMENTAL_WEBAUTHN_PLUGIN_PERFORM_UV_RESPONSE;
#[cfg(feature = "windef")]
pub type EXPERIMENTAL_PWEBAUTHN_PLUGIN_PERFROM_UV = *mut EXPERIMENTAL_WEBAUTHN_PLUGIN_PERFORM_UV;
#[cfg(feature = "shared")]
pub type EXPERIMENTAL_PWEBAUTHN_PLUGIN_UPDATE_AUTHENTICATOR_DETAILS = *mut EXPERIMENTAL_WEBAUTHN_PLUGIN_UPDATE_AUTHENTICATOR_DETAILS;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EXPERIMENTAL_WEBAUTHN_CTAPCBOR_AUTHENTICATOR_OPTIONS {
    pub dwVersion: super::DWORD,
    pub lUp: super::LONG,
    pub lUv: super::LONG,
    pub lRequireResidentKey: super::LONG,
}
pub const EXPERIMENTAL_WEBAUTHN_CTAPCBOR_AUTHENTICATOR_OPTIONS_CURRENT_VERSION: i32 = 1;
pub const EXPERIMENTAL_WEBAUTHN_CTAPCBOR_AUTHENTICATOR_OPTIONS_VERSION_1: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EXPERIMENTAL_WEBAUTHN_CTAPCBOR_ECC_PUBLIC_KEY {
    pub dwVersion: super::DWORD,
    pub lKty: super::LONG,
    pub lAlg: super::LONG,
    pub lCrv: super::LONG,
    pub cbX: super::DWORD,
    pub pbX: super::PBYTE,
    pub cbY: super::DWORD,
    pub pbY: super::PBYTE,
}
pub const EXPERIMENTAL_WEBAUTHN_CTAPCBOR_ECC_PUBLIC_KEY_CURRENT_VERSION: i32 = 1;
pub const EXPERIMENTAL_WEBAUTHN_CTAPCBOR_ECC_PUBLIC_KEY_VERSION_1: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EXPERIMENTAL_WEBAUTHN_CTAPCBOR_GET_ASSERTION_REQUEST {
    pub dwVersion: super::DWORD,
    pub pwszRpId: windows_core::PCWSTR,
    pub cbRpId: super::DWORD,
    pub pbRpId: super::PBYTE,
    pub cbClientDataHash: super::DWORD,
    pub pbClientDataHash: super::PBYTE,
    pub CredentialList: WEBAUTHN_CREDENTIAL_LIST,
    pub cbCborExtensionsMap: super::DWORD,
    pub pbCborExtensionsMap: super::PBYTE,
    pub pAuthenticatorOptions: EXPERIMENTAL_PWEBAUTHN_CTAPCBOR_AUTHENTICATOR_OPTIONS,
    pub fEmptyPinAuth: windows_core::BOOL,
    pub cbPinAuth: super::DWORD,
    pub pbPinAuth: super::PBYTE,
    pub pHmacSaltExtension: EXPERIMENTAL_PWEBAUTHN_CTAPCBOR_HMAC_SALT_EXTENSION,
    pub cbHmacSecretSaltValues: super::DWORD,
    pub pbHmacSecretSaltValues: super::PBYTE,
    pub dwPinProtocol: super::DWORD,
    pub lCredBlobExt: super::LONG,
    pub lLargeBlobKeyExt: super::LONG,
    pub dwCredLargeBlobOperation: super::DWORD,
    pub cbCredLargeBlobCompressed: super::DWORD,
    pub pbCredLargeBlobCompressed: super::PBYTE,
    pub dwCredLargeBlobOriginalSize: super::DWORD,
    pub cbJsonExt: super::DWORD,
    pub pbJsonExt: super::PBYTE,
}
pub const EXPERIMENTAL_WEBAUTHN_CTAPCBOR_GET_ASSERTION_REQUEST_CURRENT_VERSION: i32 = 1;
pub const EXPERIMENTAL_WEBAUTHN_CTAPCBOR_GET_ASSERTION_REQUEST_VERSION_1: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EXPERIMENTAL_WEBAUTHN_CTAPCBOR_GET_ASSERTION_RESPONSE {
    pub WebAuthNAssertion: WEBAUTHN_ASSERTION,
    pub pUserInformation: PCWEBAUTHN_USER_ENTITY_INFORMATION,
    pub dwNumberOfCredentials: super::DWORD,
    pub lUserSelected: super::LONG,
    pub cbLargeBlobKey: super::DWORD,
    pub pbLargeBlobKey: super::PBYTE,
    pub cbUnsignedExtensionOutputs: super::DWORD,
    pub pbUnsignedExtensionOutputs: super::PBYTE,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EXPERIMENTAL_WEBAUTHN_CTAPCBOR_HMAC_SALT_EXTENSION {
    pub dwVersion: super::DWORD,
    pub pKeyAgreement: EXPERIMENTAL_PWEBAUTHN_CTAPCBOR_ECC_PUBLIC_KEY,
    pub cbEncryptedSalt: super::DWORD,
    pub pbEncryptedSalt: super::PBYTE,
    pub cbSaltAuth: super::DWORD,
    pub pbSaltAuth: super::PBYTE,
}
pub const EXPERIMENTAL_WEBAUTHN_CTAPCBOR_HMAC_SALT_EXTENSION_CURRENT_VERSION: i32 = 1;
pub const EXPERIMENTAL_WEBAUTHN_CTAPCBOR_HMAC_SALT_EXTENSION_VERSION_1: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EXPERIMENTAL_WEBAUTHN_CTAPCBOR_MAKE_CREDENTIAL_REQUEST {
    pub dwVersion: super::DWORD,
    pub cbRpId: super::DWORD,
    pub pbRpId: super::PBYTE,
    pub cbClientDataHash: super::DWORD,
    pub pbClientDataHash: super::PBYTE,
    pub pRpInformation: PCWEBAUTHN_RP_ENTITY_INFORMATION,
    pub pUserInformation: PCWEBAUTHN_USER_ENTITY_INFORMATION,
    pub WebAuthNCredentialParameters: WEBAUTHN_COSE_CREDENTIAL_PARAMETERS,
    pub CredentialList: WEBAUTHN_CREDENTIAL_LIST,
    pub cbCborExtensionsMap: super::DWORD,
    pub pbCborExtensionsMap: super::PBYTE,
    pub pAuthenticatorOptions: EXPERIMENTAL_PWEBAUTHN_CTAPCBOR_AUTHENTICATOR_OPTIONS,
    pub fEmptyPinAuth: windows_core::BOOL,
    pub cbPinAuth: super::DWORD,
    pub pbPinAuth: super::PBYTE,
    pub lHmacSecretExt: super::LONG,
    pub pHmacSecretMcExtension: EXPERIMENTAL_PWEBAUTHN_CTAPCBOR_HMAC_SALT_EXTENSION,
    pub lPrfExt: super::LONG,
    pub cbHmacSecretSaltValues: super::DWORD,
    pub pbHmacSecretSaltValues: super::PBYTE,
    pub dwCredProtect: super::DWORD,
    pub dwPinProtocol: super::DWORD,
    pub dwEnterpriseAttestation: super::DWORD,
    pub cbCredBlobExt: super::DWORD,
    pub pbCredBlobExt: super::PBYTE,
    pub lLargeBlobKeyExt: super::LONG,
    pub dwLargeBlobSupport: super::DWORD,
    pub lMinPinLengthExt: super::LONG,
    pub cbJsonExt: super::DWORD,
    pub pbJsonExt: super::PBYTE,
}
pub const EXPERIMENTAL_WEBAUTHN_CTAPCBOR_MAKE_CREDENTIAL_REQUEST_CURRENT_VERSION: i32 = 1;
pub const EXPERIMENTAL_WEBAUTHN_CTAPCBOR_MAKE_CREDENTIAL_REQUEST_VERSION_1: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EXPERIMENTAL_WEBAUTHN_PLUGIN_ADD_AUTHENTICATOR_OPTIONS {
    pub pwszAuthenticatorName: windows_core::PCWSTR,
    pub pwszPluginClsId: windows_core::PCWSTR,
    pub pwszPluginRpId: windows_core::PCWSTR,
    pub pwszLightThemeLogo: windows_core::PCWSTR,
    pub pwszDarkThemeLogo: windows_core::PCWSTR,
    pub cbAuthenticatorInfo: super::DWORD,
    pub pbAuthenticatorInfo: super::PBYTE,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EXPERIMENTAL_WEBAUTHN_PLUGIN_ADD_AUTHENTICATOR_RESPONSE {
    pub cbOpSignPubKey: super::DWORD,
    pub pbOpSignPubKey: super::PBYTE,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EXPERIMENTAL_WEBAUTHN_PLUGIN_CREDENTIAL_DETAILS {
    pub cbCredentialId: super::DWORD,
    pub pbCredentialId: super::PBYTE,
    pub pwszRpId: windows_core::PWSTR,
    pub pwszRpName: windows_core::PWSTR,
    pub cbUserId: super::DWORD,
    pub pbUserId: super::PBYTE,
    pub pwszUserName: windows_core::PWSTR,
    pub pwszUserDisplayName: windows_core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EXPERIMENTAL_WEBAUTHN_PLUGIN_CREDENTIAL_DETAILS_LIST {
    pub pwszPluginClsId: windows_core::PWSTR,
    pub cCredentialDetails: super::DWORD,
    pub pCredentialDetails: *mut EXPERIMENTAL_PWEBAUTHN_PLUGIN_CREDENTIAL_DETAILS,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EXPERIMENTAL_WEBAUTHN_PLUGIN_PERFORM_UV {
    pub hwnd: super::HWND,
    pub transactionId: *mut windows_core::GUID,
    pub r#type: EXPERIMENTAL_WEBAUTHN_PLUGIN_PERFORM_UV_OPERATION_TYPE,
    pub pwszUsername: windows_core::PCWSTR,
    pub pwszContext: windows_core::PCWSTR,
}
pub type EXPERIMENTAL_WEBAUTHN_PLUGIN_PERFORM_UV_OPERATION_TYPE = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EXPERIMENTAL_WEBAUTHN_PLUGIN_PERFORM_UV_RESPONSE {
    pub cbResponse: super::DWORD,
    pub pbResponse: super::PBYTE,
}
pub type EXPERIMENTAL_WEBAUTHN_PLUGIN_STATUS_CHANGE_CALLBACK = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void)>;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EXPERIMENTAL_WEBAUTHN_PLUGIN_UPDATE_AUTHENTICATOR_DETAILS {
    pub pwszAuthenticatorName: windows_core::PCWSTR,
    pub pwszPluginClsId: windows_core::PCWSTR,
    pub pwszNewPluginClsId: windows_core::PCWSTR,
    pub pwszLightThemeLogo: windows_core::PCWSTR,
    pub pwszDarkThemeLogo: windows_core::PCWSTR,
    pub cbAuthenticatorInfo: super::DWORD,
    pub pbAuthenticatorInfo: super::PBYTE,
}
pub const GetPubKey: EXPERIMENTAL_WEBAUTHN_PLUGIN_PERFORM_UV_OPERATION_TYPE = 3;
pub const GetUvCount: EXPERIMENTAL_WEBAUTHN_PLUGIN_PERFORM_UV_OPERATION_TYPE = 2;
#[cfg(feature = "shared")]
pub type PCCTAPCBOR_HYBRID_STORAGE_LINKED_DATA = *const CTAPCBOR_HYBRID_STORAGE_LINKED_DATA;
#[cfg(feature = "shared")]
pub type PCTAPCBOR_HYBRID_STORAGE_LINKED_DATA = *mut CTAPCBOR_HYBRID_STORAGE_LINKED_DATA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PCWEBAUTHN_ASSERTION = *const WEBAUTHN_ASSERTION;
#[cfg(feature = "shared")]
pub type PCWEBAUTHN_AUTHENTICATOR_DETAILS = *const WEBAUTHN_AUTHENTICATOR_DETAILS;
#[cfg(feature = "shared")]
pub type PCWEBAUTHN_AUTHENTICATOR_DETAILS_LIST = *const WEBAUTHN_AUTHENTICATOR_DETAILS_LIST;
#[cfg(feature = "shared")]
pub type PCWEBAUTHN_AUTHENTICATOR_DETAILS_OPTIONS = *const WEBAUTHN_AUTHENTICATOR_DETAILS_OPTIONS;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PCWEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS = *const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PCWEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS = *const WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS;
#[cfg(feature = "shared")]
pub type PCWEBAUTHN_CLIENT_DATA = *const WEBAUTHN_CLIENT_DATA;
#[cfg(feature = "shared")]
pub type PCWEBAUTHN_COMMON_ATTESTATION = *const WEBAUTHN_COMMON_ATTESTATION;
#[cfg(feature = "shared")]
pub type PCWEBAUTHN_COSE_CREDENTIAL_PARAMETER = *const WEBAUTHN_COSE_CREDENTIAL_PARAMETER;
#[cfg(feature = "shared")]
pub type PCWEBAUTHN_COSE_CREDENTIAL_PARAMETERS = *const WEBAUTHN_COSE_CREDENTIAL_PARAMETERS;
#[cfg(feature = "shared")]
pub type PCWEBAUTHN_CREDENTIAL = *const WEBAUTHN_CREDENTIAL;
#[cfg(feature = "shared")]
pub type PCWEBAUTHN_CREDENTIALS = *const WEBAUTHN_CREDENTIALS;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PCWEBAUTHN_CREDENTIAL_ATTESTATION = *const WEBAUTHN_CREDENTIAL_ATTESTATION;
#[cfg(feature = "shared")]
pub type PCWEBAUTHN_CREDENTIAL_DETAILS = *const WEBAUTHN_CREDENTIAL_DETAILS;
#[cfg(feature = "shared")]
pub type PCWEBAUTHN_CREDENTIAL_DETAILS_LIST = *const WEBAUTHN_CREDENTIAL_DETAILS_LIST;
#[cfg(feature = "shared")]
pub type PCWEBAUTHN_CREDENTIAL_EX = *const WEBAUTHN_CREDENTIAL_EX;
#[cfg(feature = "shared")]
pub type PCWEBAUTHN_CREDENTIAL_LIST = *const WEBAUTHN_CREDENTIAL_LIST;
#[cfg(feature = "shared")]
pub type PCWEBAUTHN_CRED_BLOB_EXTENSION = *const WEBAUTHN_CRED_BLOB_EXTENSION;
#[cfg(feature = "shared")]
pub type PCWEBAUTHN_CRED_PROTECT_EXTENSION_IN = *const WEBAUTHN_CRED_PROTECT_EXTENSION_IN;
#[cfg(feature = "shared")]
pub type PCWEBAUTHN_CRED_WITH_HMAC_SECRET_SALT = *const WEBAUTHN_CRED_WITH_HMAC_SECRET_SALT;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PCWEBAUTHN_EXTENSION = *const WEBAUTHN_EXTENSION;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PCWEBAUTHN_EXTENSIONS = *const WEBAUTHN_EXTENSIONS;
#[cfg(feature = "shared")]
pub type PCWEBAUTHN_GET_CREDENTIALS_OPTIONS = *const WEBAUTHN_GET_CREDENTIALS_OPTIONS;
#[cfg(feature = "shared")]
pub type PCWEBAUTHN_HMAC_SECRET_SALT = *const WEBAUTHN_HMAC_SECRET_SALT;
#[cfg(feature = "shared")]
pub type PCWEBAUTHN_HMAC_SECRET_SALT_VALUES = *const WEBAUTHN_HMAC_SECRET_SALT_VALUES;
#[cfg(feature = "shared")]
pub type PCWEBAUTHN_RP_ENTITY_INFORMATION = *const WEBAUTHN_RP_ENTITY_INFORMATION;
#[cfg(feature = "shared")]
pub type PCWEBAUTHN_USER_ENTITY_INFORMATION = *const WEBAUTHN_USER_ENTITY_INFORMATION;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PWEBAUTHN_ASSERTION = *mut WEBAUTHN_ASSERTION;
#[cfg(feature = "shared")]
pub type PWEBAUTHN_AUTHENTICATOR_DETAILS = *mut WEBAUTHN_AUTHENTICATOR_DETAILS;
#[cfg(feature = "shared")]
pub type PWEBAUTHN_AUTHENTICATOR_DETAILS_LIST = *mut WEBAUTHN_AUTHENTICATOR_DETAILS_LIST;
#[cfg(feature = "shared")]
pub type PWEBAUTHN_AUTHENTICATOR_DETAILS_OPTIONS = *mut WEBAUTHN_AUTHENTICATOR_DETAILS_OPTIONS;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PWEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS = *mut WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PWEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS = *mut WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS;
#[cfg(feature = "shared")]
pub type PWEBAUTHN_CLIENT_DATA = *mut WEBAUTHN_CLIENT_DATA;
#[cfg(feature = "shared")]
pub type PWEBAUTHN_COMMON_ATTESTATION = *mut WEBAUTHN_COMMON_ATTESTATION;
#[cfg(feature = "shared")]
pub type PWEBAUTHN_COSE_CREDENTIAL_PARAMETER = *mut WEBAUTHN_COSE_CREDENTIAL_PARAMETER;
#[cfg(feature = "shared")]
pub type PWEBAUTHN_COSE_CREDENTIAL_PARAMETERS = *mut WEBAUTHN_COSE_CREDENTIAL_PARAMETERS;
#[cfg(feature = "shared")]
pub type PWEBAUTHN_CREDENTIAL = *mut WEBAUTHN_CREDENTIAL;
#[cfg(feature = "shared")]
pub type PWEBAUTHN_CREDENTIALS = *mut WEBAUTHN_CREDENTIALS;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PWEBAUTHN_CREDENTIAL_ATTESTATION = *mut WEBAUTHN_CREDENTIAL_ATTESTATION;
#[cfg(feature = "shared")]
pub type PWEBAUTHN_CREDENTIAL_DETAILS = *mut WEBAUTHN_CREDENTIAL_DETAILS;
#[cfg(feature = "shared")]
pub type PWEBAUTHN_CREDENTIAL_DETAILS_LIST = *mut WEBAUTHN_CREDENTIAL_DETAILS_LIST;
#[cfg(feature = "shared")]
pub type PWEBAUTHN_CREDENTIAL_EX = *mut WEBAUTHN_CREDENTIAL_EX;
#[cfg(feature = "shared")]
pub type PWEBAUTHN_CREDENTIAL_LIST = *mut WEBAUTHN_CREDENTIAL_LIST;
#[cfg(feature = "shared")]
pub type PWEBAUTHN_CRED_BLOB_EXTENSION = *mut WEBAUTHN_CRED_BLOB_EXTENSION;
#[cfg(feature = "shared")]
pub type PWEBAUTHN_CRED_PROTECT_EXTENSION_IN = *mut WEBAUTHN_CRED_PROTECT_EXTENSION_IN;
#[cfg(feature = "shared")]
pub type PWEBAUTHN_CRED_WITH_HMAC_SECRET_SALT = *mut WEBAUTHN_CRED_WITH_HMAC_SECRET_SALT;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PWEBAUTHN_EXTENSION = *mut WEBAUTHN_EXTENSION;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PWEBAUTHN_EXTENSIONS = *mut WEBAUTHN_EXTENSIONS;
#[cfg(feature = "shared")]
pub type PWEBAUTHN_GET_CREDENTIALS_OPTIONS = *mut WEBAUTHN_GET_CREDENTIALS_OPTIONS;
#[cfg(feature = "shared")]
pub type PWEBAUTHN_HMAC_SECRET_SALT = *mut WEBAUTHN_HMAC_SECRET_SALT;
#[cfg(feature = "shared")]
pub type PWEBAUTHN_HMAC_SECRET_SALT_VALUES = *mut WEBAUTHN_HMAC_SECRET_SALT_VALUES;
#[cfg(feature = "shared")]
pub type PWEBAUTHN_RP_ENTITY_INFORMATION = *mut WEBAUTHN_RP_ENTITY_INFORMATION;
#[cfg(feature = "shared")]
pub type PWEBAUTHN_USER_ENTITY_INFORMATION = *mut WEBAUTHN_USER_ENTITY_INFORMATION;
#[cfg(feature = "shared")]
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
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WEBAUTHN_ASSERTION {
    pub dwVersion: super::DWORD,
    pub cbAuthenticatorData: super::DWORD,
    pub pbAuthenticatorData: super::PBYTE,
    pub cbSignature: super::DWORD,
    pub pbSignature: super::PBYTE,
    pub Credential: WEBAUTHN_CREDENTIAL,
    pub cbUserId: super::DWORD,
    pub pbUserId: super::PBYTE,
    pub Extensions: WEBAUTHN_EXTENSIONS,
    pub cbCredLargeBlob: super::DWORD,
    pub pbCredLargeBlob: super::PBYTE,
    pub dwCredLargeBlobStatus: super::DWORD,
    pub pHmacSecret: PWEBAUTHN_HMAC_SECRET_SALT,
    pub dwUsedTransport: super::DWORD,
    pub cbUnsignedExtensionOutputs: super::DWORD,
    pub pbUnsignedExtensionOutputs: super::PBYTE,
    pub cbClientDataJSON: super::DWORD,
    pub pbClientDataJSON: super::PBYTE,
    pub cbAuthenticationResponseJSON: super::DWORD,
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
pub const WEBAUTHN_ATTESTATION_TYPE_NONE: windows_core::PCWSTR = windows_core::w!("none");
pub const WEBAUTHN_ATTESTATION_TYPE_PACKED: windows_core::PCWSTR = windows_core::w!("packed");
pub const WEBAUTHN_ATTESTATION_TYPE_TPM: windows_core::PCWSTR = windows_core::w!("tpm");
pub const WEBAUTHN_ATTESTATION_TYPE_U2F: windows_core::PCWSTR = windows_core::w!("fido-u2f");
pub const WEBAUTHN_ATTESTATION_VER_TPM_2_0: windows_core::PCWSTR = windows_core::w!("2.0");
pub const WEBAUTHN_AUTHENTICATOR_ATTACHMENT_ANY: i32 = 0;
pub const WEBAUTHN_AUTHENTICATOR_ATTACHMENT_CROSS_PLATFORM: i32 = 2;
pub const WEBAUTHN_AUTHENTICATOR_ATTACHMENT_CROSS_PLATFORM_U2F_V2: i32 = 3;
pub const WEBAUTHN_AUTHENTICATOR_ATTACHMENT_PLATFORM: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WEBAUTHN_AUTHENTICATOR_DETAILS {
    pub dwVersion: super::DWORD,
    pub cbAuthenticatorId: super::DWORD,
    pub pbAuthenticatorId: super::PBYTE,
    pub pwszAuthenticatorName: windows_core::PCWSTR,
    pub cbAuthenticatorLogo: super::DWORD,
    pub pbAuthenticatorLogo: super::PBYTE,
    pub bLocked: windows_core::BOOL,
}
pub const WEBAUTHN_AUTHENTICATOR_DETAILS_CURRENT_VERSION: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WEBAUTHN_AUTHENTICATOR_DETAILS_LIST {
    pub cAuthenticatorDetails: super::DWORD,
    pub ppAuthenticatorDetails: *mut PWEBAUTHN_AUTHENTICATOR_DETAILS,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WEBAUTHN_AUTHENTICATOR_DETAILS_OPTIONS {
    pub dwVersion: super::DWORD,
}
pub const WEBAUTHN_AUTHENTICATOR_DETAILS_OPTIONS_CURRENT_VERSION: i32 = 1;
pub const WEBAUTHN_AUTHENTICATOR_DETAILS_OPTIONS_VERSION_1: i32 = 1;
pub const WEBAUTHN_AUTHENTICATOR_DETAILS_VERSION_1: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS {
    pub dwVersion: super::DWORD,
    pub dwTimeoutMilliseconds: super::DWORD,
    pub CredentialList: WEBAUTHN_CREDENTIALS,
    pub Extensions: WEBAUTHN_EXTENSIONS,
    pub dwAuthenticatorAttachment: super::DWORD,
    pub dwUserVerificationRequirement: super::DWORD,
    pub dwFlags: super::DWORD,
    pub pwszU2fAppId: windows_core::PCWSTR,
    pub pbU2fAppId: *mut windows_core::BOOL,
    pub pCancellationId: *mut windows_core::GUID,
    pub pAllowCredentialList: PWEBAUTHN_CREDENTIAL_LIST,
    pub dwCredLargeBlobOperation: super::DWORD,
    pub cbCredLargeBlob: super::DWORD,
    pub pbCredLargeBlob: super::PBYTE,
    pub pHmacSecretSaltValues: PWEBAUTHN_HMAC_SECRET_SALT_VALUES,
    pub bBrowserInPrivateMode: windows_core::BOOL,
    pub pLinkedDevice: PCTAPCBOR_HYBRID_STORAGE_LINKED_DATA,
    pub bAutoFill: windows_core::BOOL,
    pub cbJsonExt: super::DWORD,
    pub pbJsonExt: super::PBYTE,
    pub cCredentialHints: super::DWORD,
    pub ppwszCredentialHints: *mut windows_core::PCWSTR,
    pub pwszRemoteWebOrigin: windows_core::PCWSTR,
    pub cbPublicKeyCredentialRequestOptionsJSON: super::DWORD,
    pub pbPublicKeyCredentialRequestOptionsJSON: super::PBYTE,
    pub cbAuthenticatorId: super::DWORD,
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
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS {
    pub dwVersion: super::DWORD,
    pub dwTimeoutMilliseconds: super::DWORD,
    pub CredentialList: WEBAUTHN_CREDENTIALS,
    pub Extensions: WEBAUTHN_EXTENSIONS,
    pub dwAuthenticatorAttachment: super::DWORD,
    pub bRequireResidentKey: windows_core::BOOL,
    pub dwUserVerificationRequirement: super::DWORD,
    pub dwAttestationConveyancePreference: super::DWORD,
    pub dwFlags: super::DWORD,
    pub pCancellationId: *mut windows_core::GUID,
    pub pExcludeCredentialList: PWEBAUTHN_CREDENTIAL_LIST,
    pub dwEnterpriseAttestation: super::DWORD,
    pub dwLargeBlobSupport: super::DWORD,
    pub bPreferResidentKey: windows_core::BOOL,
    pub bBrowserInPrivateMode: windows_core::BOOL,
    pub bEnablePrf: windows_core::BOOL,
    pub pLinkedDevice: PCTAPCBOR_HYBRID_STORAGE_LINKED_DATA,
    pub cbJsonExt: super::DWORD,
    pub pbJsonExt: super::PBYTE,
    pub pPRFGlobalEval: PWEBAUTHN_HMAC_SECRET_SALT,
    pub cCredentialHints: super::DWORD,
    pub ppwszCredentialHints: *mut windows_core::PCWSTR,
    pub bThirdPartyPayment: windows_core::BOOL,
    pub pwszRemoteWebOrigin: windows_core::PCWSTR,
    pub cbPublicKeyCredentialCreationOptionsJSON: super::DWORD,
    pub pbPublicKeyCredentialCreationOptionsJSON: super::PBYTE,
    pub cbAuthenticatorId: super::DWORD,
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
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WEBAUTHN_CLIENT_DATA {
    pub dwVersion: super::DWORD,
    pub cbClientDataJSON: super::DWORD,
    pub pbClientDataJSON: super::PBYTE,
    pub pwszHashAlgId: windows_core::PCWSTR,
}
pub const WEBAUTHN_CLIENT_DATA_CURRENT_VERSION: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WEBAUTHN_COMMON_ATTESTATION {
    pub dwVersion: super::DWORD,
    pub pwszAlg: windows_core::PCWSTR,
    pub lAlg: super::LONG,
    pub cbSignature: super::DWORD,
    pub pbSignature: super::PBYTE,
    pub cX5c: super::DWORD,
    pub pX5c: PWEBAUTHN_X5C,
    pub pwszVer: windows_core::PCWSTR,
    pub cbCertInfo: super::DWORD,
    pub pbCertInfo: super::PBYTE,
    pub cbPubArea: super::DWORD,
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
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WEBAUTHN_COSE_CREDENTIAL_PARAMETER {
    pub dwVersion: super::DWORD,
    pub pwszCredentialType: windows_core::PCWSTR,
    pub lAlg: super::LONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WEBAUTHN_COSE_CREDENTIAL_PARAMETERS {
    pub cCredentialParameters: super::DWORD,
    pub pCredentialParameters: PWEBAUTHN_COSE_CREDENTIAL_PARAMETER,
}
pub const WEBAUTHN_COSE_CREDENTIAL_PARAMETER_CURRENT_VERSION: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WEBAUTHN_CREDENTIAL {
    pub dwVersion: super::DWORD,
    pub cbId: super::DWORD,
    pub pbId: super::PBYTE,
    pub pwszCredentialType: windows_core::PCWSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WEBAUTHN_CREDENTIALS {
    pub cCredentials: super::DWORD,
    pub pCredentials: PWEBAUTHN_CREDENTIAL,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WEBAUTHN_CREDENTIAL_ATTESTATION {
    pub dwVersion: super::DWORD,
    pub pwszFormatType: windows_core::PCWSTR,
    pub cbAuthenticatorData: super::DWORD,
    pub pbAuthenticatorData: super::PBYTE,
    pub cbAttestation: super::DWORD,
    pub pbAttestation: super::PBYTE,
    pub dwAttestationDecodeType: super::DWORD,
    pub pvAttestationDecode: super::PVOID,
    pub cbAttestationObject: super::DWORD,
    pub pbAttestationObject: super::PBYTE,
    pub cbCredentialId: super::DWORD,
    pub pbCredentialId: super::PBYTE,
    pub Extensions: WEBAUTHN_EXTENSIONS,
    pub dwUsedTransport: super::DWORD,
    pub bEpAtt: windows_core::BOOL,
    pub bLargeBlobSupported: windows_core::BOOL,
    pub bResidentKey: windows_core::BOOL,
    pub bPrfEnabled: windows_core::BOOL,
    pub cbUnsignedExtensionOutputs: super::DWORD,
    pub pbUnsignedExtensionOutputs: super::PBYTE,
    pub pHmacSecret: PWEBAUTHN_HMAC_SECRET_SALT,
    pub bThirdPartyPayment: windows_core::BOOL,
    pub dwTransports: super::DWORD,
    pub cbClientDataJSON: super::DWORD,
    pub pbClientDataJSON: super::PBYTE,
    pub cbRegistrationResponseJSON: super::DWORD,
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
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WEBAUTHN_CREDENTIAL_DETAILS {
    pub dwVersion: super::DWORD,
    pub cbCredentialID: super::DWORD,
    pub pbCredentialID: super::PBYTE,
    pub pRpInformation: PWEBAUTHN_RP_ENTITY_INFORMATION,
    pub pUserInformation: PWEBAUTHN_USER_ENTITY_INFORMATION,
    pub bRemovable: windows_core::BOOL,
    pub bBackedUp: windows_core::BOOL,
    pub pwszAuthenticatorName: windows_core::PCWSTR,
    pub cbAuthenticatorLogo: super::DWORD,
    pub pbAuthenticatorLogo: super::PBYTE,
    pub bThirdPartyPayment: windows_core::BOOL,
    pub dwTransports: super::DWORD,
}
pub const WEBAUTHN_CREDENTIAL_DETAILS_CURRENT_VERSION: i32 = 4;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WEBAUTHN_CREDENTIAL_DETAILS_LIST {
    pub cCredentialDetails: super::DWORD,
    pub ppCredentialDetails: *mut PWEBAUTHN_CREDENTIAL_DETAILS,
}
pub const WEBAUTHN_CREDENTIAL_DETAILS_VERSION_1: i32 = 1;
pub const WEBAUTHN_CREDENTIAL_DETAILS_VERSION_2: i32 = 2;
pub const WEBAUTHN_CREDENTIAL_DETAILS_VERSION_3: i32 = 3;
pub const WEBAUTHN_CREDENTIAL_DETAILS_VERSION_4: i32 = 4;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WEBAUTHN_CREDENTIAL_EX {
    pub dwVersion: super::DWORD,
    pub cbId: super::DWORD,
    pub pbId: super::PBYTE,
    pub pwszCredentialType: windows_core::PCWSTR,
    pub dwTransports: super::DWORD,
}
pub const WEBAUTHN_CREDENTIAL_EX_CURRENT_VERSION: i32 = 1;
pub const WEBAUTHN_CREDENTIAL_HINT_CLIENT_DEVICE: windows_core::PCWSTR = windows_core::w!("client-device");
pub const WEBAUTHN_CREDENTIAL_HINT_HYBRID: windows_core::PCWSTR = windows_core::w!("hybrid");
pub const WEBAUTHN_CREDENTIAL_HINT_SECURITY_KEY: windows_core::PCWSTR = windows_core::w!("security-key");
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WEBAUTHN_CREDENTIAL_LIST {
    pub cCredentials: super::DWORD,
    pub ppCredentials: *mut PWEBAUTHN_CREDENTIAL_EX,
}
pub const WEBAUTHN_CREDENTIAL_TYPE_PUBLIC_KEY: windows_core::PCWSTR = windows_core::w!("public-key");
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WEBAUTHN_CRED_BLOB_EXTENSION {
    pub cbCredBlob: super::DWORD,
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
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WEBAUTHN_CRED_PROTECT_EXTENSION_IN {
    pub dwCredProtect: super::DWORD,
    pub bRequireCredProtect: windows_core::BOOL,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WEBAUTHN_CRED_WITH_HMAC_SECRET_SALT {
    pub cbCredID: super::DWORD,
    pub pbCredID: super::PBYTE,
    pub pHmacSecretSalt: PWEBAUTHN_HMAC_SECRET_SALT,
}
pub const WEBAUTHN_CTAP_ONE_HMAC_SECRET_LENGTH: i32 = 32;
pub const WEBAUTHN_CTAP_TRANSPORT_BLE: i32 = 4;
pub const WEBAUTHN_CTAP_TRANSPORT_BLE_STRING: windows_core::PCSTR = windows_core::s!("ble");
pub const WEBAUTHN_CTAP_TRANSPORT_FLAGS_MASK: i32 = 127;
pub const WEBAUTHN_CTAP_TRANSPORT_HYBRID: i32 = 32;
pub const WEBAUTHN_CTAP_TRANSPORT_HYBRID_STRING: windows_core::PCSTR = windows_core::s!("hybrid");
pub const WEBAUTHN_CTAP_TRANSPORT_INTERNAL: i32 = 16;
pub const WEBAUTHN_CTAP_TRANSPORT_INTERNAL_STRING: windows_core::PCSTR = windows_core::s!("internal");
pub const WEBAUTHN_CTAP_TRANSPORT_NFC: i32 = 2;
pub const WEBAUTHN_CTAP_TRANSPORT_NFC_STRING: windows_core::PCSTR = windows_core::s!("nfc");
pub const WEBAUTHN_CTAP_TRANSPORT_SMART_CARD: i32 = 64;
pub const WEBAUTHN_CTAP_TRANSPORT_SMART_CARD_STRING: windows_core::PCSTR = windows_core::s!("smart-card");
pub const WEBAUTHN_CTAP_TRANSPORT_TEST: i32 = 8;
pub const WEBAUTHN_CTAP_TRANSPORT_USB: i32 = 1;
pub const WEBAUTHN_CTAP_TRANSPORT_USB_STRING: windows_core::PCSTR = windows_core::s!("usb");
pub const WEBAUTHN_ENTERPRISE_ATTESTATION_NONE: i32 = 0;
pub const WEBAUTHN_ENTERPRISE_ATTESTATION_PLATFORM_MANAGED: i32 = 2;
pub const WEBAUTHN_ENTERPRISE_ATTESTATION_VENDOR_FACILITATED: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WEBAUTHN_EXTENSION {
    pub pwszExtensionIdentifier: windows_core::PCWSTR,
    pub cbExtension: super::DWORD,
    pub pvExtension: super::PVOID,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WEBAUTHN_EXTENSIONS {
    pub cExtensions: super::DWORD,
    pub pExtensions: PWEBAUTHN_EXTENSION,
}
pub const WEBAUTHN_EXTENSIONS_IDENTIFIER_CRED_BLOB: windows_core::PCWSTR = windows_core::w!("credBlob");
pub const WEBAUTHN_EXTENSIONS_IDENTIFIER_CRED_PROTECT: windows_core::PCWSTR = windows_core::w!("credProtect");
pub const WEBAUTHN_EXTENSIONS_IDENTIFIER_HMAC_SECRET: windows_core::PCWSTR = windows_core::w!("hmac-secret");
pub const WEBAUTHN_EXTENSIONS_IDENTIFIER_MIN_PIN_LENGTH: windows_core::PCWSTR = windows_core::w!("minPinLength");
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WEBAUTHN_GET_CREDENTIALS_OPTIONS {
    pub dwVersion: super::DWORD,
    pub pwszRpId: windows_core::PCWSTR,
    pub bBrowserInPrivateMode: windows_core::BOOL,
}
pub const WEBAUTHN_GET_CREDENTIALS_OPTIONS_CURRENT_VERSION: i32 = 1;
pub const WEBAUTHN_GET_CREDENTIALS_OPTIONS_VERSION_1: i32 = 1;
pub const WEBAUTHN_HASH_ALGORITHM_SHA_256: windows_core::PCWSTR = windows_core::w!("SHA-256");
pub const WEBAUTHN_HASH_ALGORITHM_SHA_384: windows_core::PCWSTR = windows_core::w!("SHA-384");
pub const WEBAUTHN_HASH_ALGORITHM_SHA_512: windows_core::PCWSTR = windows_core::w!("SHA-512");
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WEBAUTHN_HMAC_SECRET_SALT {
    pub cbFirst: super::DWORD,
    pub pbFirst: super::PBYTE,
    pub cbSecond: super::DWORD,
    pub pbSecond: super::PBYTE,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WEBAUTHN_HMAC_SECRET_SALT_VALUES {
    pub pGlobalHmacSalt: PWEBAUTHN_HMAC_SECRET_SALT,
    pub cCredWithHmacSecretSaltList: super::DWORD,
    pub pCredWithHmacSecretSaltList: PWEBAUTHN_CRED_WITH_HMAC_SECRET_SALT,
}
pub const WEBAUTHN_LARGE_BLOB_SUPPORT_NONE: i32 = 0;
pub const WEBAUTHN_LARGE_BLOB_SUPPORT_PREFERRED: i32 = 2;
pub const WEBAUTHN_LARGE_BLOB_SUPPORT_REQUIRED: i32 = 1;
pub const WEBAUTHN_MAX_USER_ID_LENGTH: i32 = 64;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WEBAUTHN_RP_ENTITY_INFORMATION {
    pub dwVersion: super::DWORD,
    pub pwszId: windows_core::PCWSTR,
    pub pwszName: windows_core::PCWSTR,
    pub pwszIcon: windows_core::PCWSTR,
}
pub const WEBAUTHN_RP_ENTITY_INFORMATION_CURRENT_VERSION: i32 = 1;
pub const WEBAUTHN_RP_ENTITY_INFORMATION_VERSION_1: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WEBAUTHN_USER_ENTITY_INFORMATION {
    pub dwVersion: super::DWORD,
    pub cbId: super::DWORD,
    pub pbId: super::PBYTE,
    pub pwszName: windows_core::PCWSTR,
    pub pwszIcon: windows_core::PCWSTR,
    pub pwszDisplayName: windows_core::PCWSTR,
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
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WEBAUTHN_X5C {
    pub cbData: super::DWORD,
    pub pbData: super::PBYTE,
}
