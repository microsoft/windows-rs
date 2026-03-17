#[inline]
pub unsafe fn WebAuthNAuthenticatorGetAssertion<P1>(hwnd: super::super::super::Foundation::HWND, pwszrpid: P1, pwebauthnclientdata: *const WEBAUTHN_CLIENT_DATA, pwebauthngetassertionoptions: Option<*const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS>) -> windows_core::Result<*mut WEBAUTHN_ASSERTION>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("webauthn.dll" "system" fn WebAuthNAuthenticatorGetAssertion(hwnd : super::super::super::Foundation:: HWND, pwszrpid : windows_core::PCWSTR, pwebauthnclientdata : *const WEBAUTHN_CLIENT_DATA, pwebauthngetassertionoptions : *const WEBAUTHN_AUTHENTICATOR_GET_ASSERTION_OPTIONS, ppwebauthnassertion : *mut *mut WEBAUTHN_ASSERTION) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WebAuthNAuthenticatorGetAssertion(hwnd, pwszrpid.param().abi(), pwebauthnclientdata, pwebauthngetassertionoptions.unwrap_or(core::mem::zeroed()) as _, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn WebAuthNAuthenticatorMakeCredential(hwnd: super::super::super::Foundation::HWND, prpinformation: *const WEBAUTHN_RP_ENTITY_INFORMATION, puserinformation: *const WEBAUTHN_USER_ENTITY_INFORMATION, ppubkeycredparams: *const WEBAUTHN_COSE_CREDENTIAL_PARAMETERS, pwebauthnclientdata: *const WEBAUTHN_CLIENT_DATA, pwebauthnmakecredentialoptions: Option<*const WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS>) -> windows_core::Result<*mut WEBAUTHN_CREDENTIAL_ATTESTATION> {
    windows_core::link!("webauthn.dll" "system" fn WebAuthNAuthenticatorMakeCredential(hwnd : super::super::super::Foundation:: HWND, prpinformation : *const WEBAUTHN_RP_ENTITY_INFORMATION, puserinformation : *const WEBAUTHN_USER_ENTITY_INFORMATION, ppubkeycredparams : *const WEBAUTHN_COSE_CREDENTIAL_PARAMETERS, pwebauthnclientdata : *const WEBAUTHN_CLIENT_DATA, pwebauthnmakecredentialoptions : *const WEBAUTHN_AUTHENTICATOR_MAKE_CREDENTIAL_OPTIONS, ppwebauthncredentialattestation : *mut *mut WEBAUTHN_CREDENTIAL_ATTESTATION) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WebAuthNAuthenticatorMakeCredential(hwnd, prpinformation, puserinformation, ppubkeycredparams, pwebauthnclientdata, pwebauthnmakecredentialoptions.unwrap_or(core::mem::zeroed()) as _, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn WebAuthNCancelCurrentOperation(pcancellationid: *const windows_core::GUID) -> windows_core::Result<()> {
    windows_core::link!("webauthn.dll" "system" fn WebAuthNCancelCurrentOperation(pcancellationid : *const windows_core::GUID) -> windows_core::HRESULT);
    unsafe { WebAuthNCancelCurrentOperation(pcancellationid).ok() }
}
#[inline]
pub unsafe fn WebAuthNDeletePlatformCredential(pbcredentialid: &[u8]) -> windows_core::Result<()> {
    windows_core::link!("webauthn.dll" "system" fn WebAuthNDeletePlatformCredential(cbcredentialid : u32, pbcredentialid : *const u8) -> windows_core::HRESULT);
    unsafe { WebAuthNDeletePlatformCredential(pbcredentialid.len().try_into().unwrap(), core::mem::transmute(pbcredentialid.as_ptr())).ok() }
}
#[inline]
pub unsafe fn WebAuthNFreeAssertion(pwebauthnassertion: *const WEBAUTHN_ASSERTION) {
    windows_core::link!("webauthn.dll" "system" fn WebAuthNFreeAssertion(pwebauthnassertion : *const WEBAUTHN_ASSERTION));
    unsafe { WebAuthNFreeAssertion(pwebauthnassertion) }
}
#[inline]
pub unsafe fn WebAuthNFreeAuthenticatorList(pauthenticatordetailslist: *const WEBAUTHN_AUTHENTICATOR_DETAILS_LIST) {
    windows_core::link!("webauthn.dll" "system" fn WebAuthNFreeAuthenticatorList(pauthenticatordetailslist : *const WEBAUTHN_AUTHENTICATOR_DETAILS_LIST));
    unsafe { WebAuthNFreeAuthenticatorList(pauthenticatordetailslist) }
}
#[inline]
pub unsafe fn WebAuthNFreeCredentialAttestation(pwebauthncredentialattestation: Option<*const WEBAUTHN_CREDENTIAL_ATTESTATION>) {
    windows_core::link!("webauthn.dll" "system" fn WebAuthNFreeCredentialAttestation(pwebauthncredentialattestation : *const WEBAUTHN_CREDENTIAL_ATTESTATION));
    unsafe { WebAuthNFreeCredentialAttestation(pwebauthncredentialattestation.unwrap_or(core::mem::zeroed()) as _) }
}
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
#[inline]
pub unsafe fn WebAuthNGetAuthenticatorList(pwebauthngetauthenticatorlistoptions: Option<*const WEBAUTHN_AUTHENTICATOR_DETAILS_OPTIONS>) -> windows_core::Result<*mut WEBAUTHN_AUTHENTICATOR_DETAILS_LIST> {
    windows_core::link!("webauthn.dll" "system" fn WebAuthNGetAuthenticatorList(pwebauthngetauthenticatorlistoptions : *const WEBAUTHN_AUTHENTICATOR_DETAILS_OPTIONS, ppauthenticatordetailslist : *mut *mut WEBAUTHN_AUTHENTICATOR_DETAILS_LIST) -> windows_core::HRESULT);
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
#[inline]
pub unsafe fn WebAuthNGetPlatformCredentialList(pgetcredentialsoptions: *const WEBAUTHN_GET_CREDENTIALS_OPTIONS) -> windows_core::Result<*mut WEBAUTHN_CREDENTIAL_DETAILS_LIST> {
    windows_core::link!("webauthn.dll" "system" fn WebAuthNGetPlatformCredentialList(pgetcredentialsoptions : *const WEBAUTHN_GET_CREDENTIALS_OPTIONS, ppcredentialdetailslist : *mut *mut WEBAUTHN_CREDENTIAL_DETAILS_LIST) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WebAuthNGetPlatformCredentialList(pgetcredentialsoptions, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn WebAuthNGetW3CExceptionDOMError(hr: windows_core::HRESULT) -> windows_core::Result<()> {
    windows_core::link!("webauthn.dll" "system" fn WebAuthNGetW3CExceptionDOMError(hr : windows_core::HRESULT) -> windows_core::HRESULT);
    unsafe { WebAuthNGetW3CExceptionDOMError(hr).ok() }
}
#[inline]
pub unsafe fn WebAuthNIsUserVerifyingPlatformAuthenticatorAvailable() -> windows_core::Result<windows_core::BOOL> {
    windows_core::link!("webauthn.dll" "system" fn WebAuthNIsUserVerifyingPlatformAuthenticatorAvailable(pbisuserverifyingplatformauthenticatoravailable : *mut windows_core::BOOL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WebAuthNIsUserVerifyingPlatformAuthenticatorAvailable(&mut result__).map(|| result__)
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AUTHENTICATOR_STATE(pub i32);
pub const AuthenticatorState_Disabled: AUTHENTICATOR_STATE = AUTHENTICATOR_STATE(0i32);
pub const AuthenticatorState_Enabled: AUTHENTICATOR_STATE = AUTHENTICATOR_STATE(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
    pub pwszAuthenticatorName: windows_core::PCWSTR,
    pub wEncodedTunnelServerDomain: u16,
}
impl Default for CTAPCBOR_HYBRID_STORAGE_LINKED_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CTAPCBOR_HYBRID_STORAGE_LINKED_DATA_CURRENT_VERSION: u32 = 1u32;
pub const CTAPCBOR_HYBRID_STORAGE_LINKED_DATA_VERSION_1: u32 = 1u32;
windows_core::imp::define_interface!(EXPERIMENTAL_IPluginAuthenticator, EXPERIMENTAL_IPluginAuthenticator_Vtbl, 0xe6466e9a_b2f3_47c5_b88d_89bc14a8d998);
windows_core::imp::interface_hierarchy!(EXPERIMENTAL_IPluginAuthenticator, windows_core::IUnknown);
impl EXPERIMENTAL_IPluginAuthenticator {
    pub unsafe fn EXPERIMENTAL_PluginMakeCredential(&self, request: *const EXPERIMENTAL_WEBAUTHN_PLUGIN_OPERATION_REQUEST) -> windows_core::Result<*mut EXPERIMENTAL_WEBAUTHN_PLUGIN_OPERATION_RESPONSE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EXPERIMENTAL_PluginMakeCredential)(windows_core::Interface::as_raw(self), request, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EXPERIMENTAL_PluginGetAssertion(&self, request: *const EXPERIMENTAL_WEBAUTHN_PLUGIN_OPERATION_REQUEST) -> windows_core::Result<*mut EXPERIMENTAL_WEBAUTHN_PLUGIN_OPERATION_RESPONSE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EXPERIMENTAL_PluginGetAssertion)(windows_core::Interface::as_raw(self), request, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EXPERIMENTAL_PluginCancelOperation(&self, request: *const EXPERIMENTAL_WEBAUTHN_PLUGIN_CANCEL_OPERATION_REQUEST) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EXPERIMENTAL_PluginCancelOperation)(windows_core::Interface::as_raw(self), request).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct EXPERIMENTAL_IPluginAuthenticator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub EXPERIMENTAL_PluginMakeCredential: unsafe extern "system" fn(*mut core::ffi::c_void, *const EXPERIMENTAL_WEBAUTHN_PLUGIN_OPERATION_REQUEST, *mut *mut EXPERIMENTAL_WEBAUTHN_PLUGIN_OPERATION_RESPONSE) -> windows_core::HRESULT,
    pub EXPERIMENTAL_PluginGetAssertion: unsafe extern "system" fn(*mut core::ffi::c_void, *const EXPERIMENTAL_WEBAUTHN_PLUGIN_OPERATION_REQUEST, *mut *mut EXPERIMENTAL_WEBAUTHN_PLUGIN_OPERATION_RESPONSE) -> windows_core::HRESULT,
    pub EXPERIMENTAL_PluginCancelOperation: unsafe extern "system" fn(*mut core::ffi::c_void, *const EXPERIMENTAL_WEBAUTHN_PLUGIN_CANCEL_OPERATION_REQUEST) -> windows_core::HRESULT,
}
pub trait EXPERIMENTAL_IPluginAuthenticator_Impl: windows_core::IUnknownImpl {
    fn EXPERIMENTAL_PluginMakeCredential(&self, request: *const EXPERIMENTAL_WEBAUTHN_PLUGIN_OPERATION_REQUEST) -> windows_core::Result<*mut EXPERIMENTAL_WEBAUTHN_PLUGIN_OPERATION_RESPONSE>;
    fn EXPERIMENTAL_PluginGetAssertion(&self, request: *const EXPERIMENTAL_WEBAUTHN_PLUGIN_OPERATION_REQUEST) -> windows_core::Result<*mut EXPERIMENTAL_WEBAUTHN_PLUGIN_OPERATION_RESPONSE>;
    fn EXPERIMENTAL_PluginCancelOperation(&self, request: *const EXPERIMENTAL_WEBAUTHN_PLUGIN_CANCEL_OPERATION_REQUEST) -> windows_core::Result<()>;
}
impl EXPERIMENTAL_IPluginAuthenticator_Vtbl {
    pub const fn new<Identity: EXPERIMENTAL_IPluginAuthenticator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EXPERIMENTAL_PluginMakeCredential<Identity: EXPERIMENTAL_IPluginAuthenticator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, request: *const EXPERIMENTAL_WEBAUTHN_PLUGIN_OPERATION_REQUEST, response: *mut *mut EXPERIMENTAL_WEBAUTHN_PLUGIN_OPERATION_RESPONSE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match EXPERIMENTAL_IPluginAuthenticator_Impl::EXPERIMENTAL_PluginMakeCredential(this, core::mem::transmute_copy(&request)) {
                    Ok(ok__) => {
                        response.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EXPERIMENTAL_PluginGetAssertion<Identity: EXPERIMENTAL_IPluginAuthenticator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, request: *const EXPERIMENTAL_WEBAUTHN_PLUGIN_OPERATION_REQUEST, response: *mut *mut EXPERIMENTAL_WEBAUTHN_PLUGIN_OPERATION_RESPONSE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match EXPERIMENTAL_IPluginAuthenticator_Impl::EXPERIMENTAL_PluginGetAssertion(this, core::mem::transmute_copy(&request)) {
                    Ok(ok__) => {
                        response.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EXPERIMENTAL_PluginCancelOperation<Identity: EXPERIMENTAL_IPluginAuthenticator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, request: *const EXPERIMENTAL_WEBAUTHN_PLUGIN_CANCEL_OPERATION_REQUEST) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                EXPERIMENTAL_IPluginAuthenticator_Impl::EXPERIMENTAL_PluginCancelOperation(this, core::mem::transmute_copy(&request)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EXPERIMENTAL_PluginMakeCredential: EXPERIMENTAL_PluginMakeCredential::<Identity, OFFSET>,
            EXPERIMENTAL_PluginGetAssertion: EXPERIMENTAL_PluginGetAssertion::<Identity, OFFSET>,
            EXPERIMENTAL_PluginCancelOperation: EXPERIMENTAL_PluginCancelOperation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<EXPERIMENTAL_IPluginAuthenticator as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for EXPERIMENTAL_IPluginAuthenticator {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EXPERIMENTAL_PLUGIN_AUTHENTICATOR_STATE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EXPERIMENTAL_WEBAUTHN_CTAPCBOR_AUTHENTICATOR_OPTIONS {
    pub dwVersion: u32,
    pub lUp: i32,
    pub lUv: i32,
    pub lRequireResidentKey: i32,
}
pub const EXPERIMENTAL_WEBAUTHN_CTAPCBOR_AUTHENTICATOR_OPTIONS_CURRENT_VERSION: u32 = 1u32;
pub const EXPERIMENTAL_WEBAUTHN_CTAPCBOR_AUTHENTICATOR_OPTIONS_VERSION_1: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EXPERIMENTAL_WEBAUTHN_CTAPCBOR_GET_ASSERTION_REQUEST {
    pub dwVersion: u32,
    pub pwszRpId: windows_core::PCWSTR,
    pub cbRpId: u32,
    pub pbRpId: *mut u8,
    pub cbClientDataHash: u32,
    pub pbClientDataHash: *mut u8,
    pub CredentialList: WEBAUTHN_CREDENTIAL_LIST,
    pub cbCborExtensionsMap: u32,
    pub pbCborExtensionsMap: *mut u8,
    pub pAuthenticatorOptions: *mut EXPERIMENTAL_WEBAUTHN_CTAPCBOR_AUTHENTICATOR_OPTIONS,
    pub fEmptyPinAuth: windows_core::BOOL,
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
    pub fEmptyPinAuth: windows_core::BOOL,
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EXPERIMENTAL_WEBAUTHN_PLUGIN_ADD_AUTHENTICATOR_OPTIONS {
    pub pwszAuthenticatorName: windows_core::PCWSTR,
    pub pwszPluginClsId: windows_core::PCWSTR,
    pub pwszPluginRpId: windows_core::PCWSTR,
    pub pwszLightThemeLogo: windows_core::PCWSTR,
    pub pwszDarkThemeLogo: windows_core::PCWSTR,
    pub cbAuthenticatorInfo: u32,
    pub pbAuthenticatorInfo: *mut u8,
}
impl Default for EXPERIMENTAL_WEBAUTHN_PLUGIN_ADD_AUTHENTICATOR_OPTIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EXPERIMENTAL_WEBAUTHN_PLUGIN_ADD_AUTHENTICATOR_OPTIONS_2 {
    pub pwszAuthenticatorName: windows_core::PCWSTR,
    pub pClsid: *const windows_core::GUID,
    pub pwszPluginRpId: windows_core::PCWSTR,
    pub pwszLightThemeLogoSvg: windows_core::PCWSTR,
    pub pwszDarkThemeLogoSvg: windows_core::PCWSTR,
    pub cbAuthenticatorInfo: u32,
    pub pbAuthenticatorInfo: *const u8,
    pub cSupportedRpIds: u32,
    pub ppwszSupportedRpIds: *const windows_core::PCWSTR,
    pub pwszUserVerificationKeyName: windows_core::PCWSTR,
}
impl Default for EXPERIMENTAL_WEBAUTHN_PLUGIN_ADD_AUTHENTICATOR_OPTIONS_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EXPERIMENTAL_WEBAUTHN_PLUGIN_CANCEL_OPERATION_REQUEST {
    pub transactionId: windows_core::GUID,
    pub cbRequestSignature: u32,
    pub pbRequestSignature: *mut u8,
}
impl Default for EXPERIMENTAL_WEBAUTHN_PLUGIN_CANCEL_OPERATION_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EXPERIMENTAL_WEBAUTHN_PLUGIN_CREDENTIAL_DETAILS {
    pub cbCredentialId: u32,
    pub pbCredentialId: *mut u8,
    pub pwszRpId: windows_core::PWSTR,
    pub pwszRpName: windows_core::PWSTR,
    pub cbUserId: u32,
    pub pbUserId: *mut u8,
    pub pwszUserName: windows_core::PWSTR,
    pub pwszUserDisplayName: windows_core::PWSTR,
}
impl Default for EXPERIMENTAL_WEBAUTHN_PLUGIN_CREDENTIAL_DETAILS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EXPERIMENTAL_WEBAUTHN_PLUGIN_CREDENTIAL_DETAILS_LIST {
    pub pwszPluginClsId: windows_core::PWSTR,
    pub cCredentialDetails: u32,
    pub pCredentialDetails: *mut *mut EXPERIMENTAL_WEBAUTHN_PLUGIN_CREDENTIAL_DETAILS,
}
impl Default for EXPERIMENTAL_WEBAUTHN_PLUGIN_CREDENTIAL_DETAILS_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EXPERIMENTAL_WEBAUTHN_PLUGIN_OPERATION_REQUEST {
    pub hWnd: super::super::super::Foundation::HWND,
    pub transactionId: windows_core::GUID,
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EXPERIMENTAL_WEBAUTHN_PLUGIN_PERFORM_UV {
    pub hwnd: super::super::super::Foundation::HWND,
    pub transactionId: *mut windows_core::GUID,
    pub r#type: EXPERIMENTAL_WEBAUTHN_PLUGIN_PERFORM_UV_OPERATION_TYPE,
    pub pwszUsername: windows_core::PCWSTR,
    pub pwszContext: windows_core::PCWSTR,
}
impl Default for EXPERIMENTAL_WEBAUTHN_PLUGIN_PERFORM_UV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EXPERIMENTAL_WEBAUTHN_PLUGIN_PERFORM_UV_OPERATION_TYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EXPERIMENTAL_WEBAUTHN_PLUGIN_UPDATE_AUTHENTICATOR_DETAILS {
    pub pwszAuthenticatorName: windows_core::PCWSTR,
    pub pwszPluginClsId: windows_core::PCWSTR,
    pub pwszNewPluginClsId: windows_core::PCWSTR,
    pub pwszLightThemeLogo: windows_core::PCWSTR,
    pub pwszDarkThemeLogo: windows_core::PCWSTR,
    pub cbAuthenticatorInfo: u32,
    pub pbAuthenticatorInfo: *mut u8,
}
impl Default for EXPERIMENTAL_WEBAUTHN_PLUGIN_UPDATE_AUTHENTICATOR_DETAILS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EXPERIMENTAL_WEBAUTHN_PLUGIN_UPDATE_AUTHENTICATOR_DETAILS_2 {
    pub pwszAuthenticatorName: windows_core::PCWSTR,
    pub pClsid: *const windows_core::GUID,
    pub pClsidNew: *const windows_core::GUID,
    pub pwszLightThemeLogoSvg: windows_core::PCWSTR,
    pub pwszDarkThemeLogoSvg: windows_core::PCWSTR,
    pub cbAuthenticatorInfo: u32,
    pub pbAuthenticatorInfo: *const u8,
    pub cSupportedRpIds: u32,
    pub ppwszSupportedRpIds: *const windows_core::PCWSTR,
    pub pwszUserVerificationKeyName: windows_core::PCWSTR,
}
impl Default for EXPERIMENTAL_WEBAUTHN_PLUGIN_UPDATE_AUTHENTICATOR_DETAILS_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EXPERIMENTAL_WEBAUTHN_PLUGIN_USER_VERIFICATION_REQUEST_2 {
    pub hwnd: super::super::super::Foundation::HWND,
    pub pGuidTransactionId: *const windows_core::GUID,
    pub pwszUsername: windows_core::PCWSTR,
    pub pwszDisplayHint: windows_core::PCWSTR,
    pub cbBufferToSign: u32,
    pub pbBufferToSign: *mut u8,
}
impl Default for EXPERIMENTAL_WEBAUTHN_PLUGIN_USER_VERIFICATION_REQUEST_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const GetPubKey: EXPERIMENTAL_WEBAUTHN_PLUGIN_PERFORM_UV_OPERATION_TYPE = EXPERIMENTAL_WEBAUTHN_PLUGIN_PERFORM_UV_OPERATION_TYPE(3i32);
pub const GetPublicKey: WEBAUTHN_PLUGIN_PERFORM_UV_OPERATION_TYPE = WEBAUTHN_PLUGIN_PERFORM_UV_OPERATION_TYPE(3i32);
pub const GetUserVerificationCount: WEBAUTHN_PLUGIN_PERFORM_UV_OPERATION_TYPE = WEBAUTHN_PLUGIN_PERFORM_UV_OPERATION_TYPE(2i32);
pub const GetUvCount: EXPERIMENTAL_WEBAUTHN_PLUGIN_PERFORM_UV_OPERATION_TYPE = EXPERIMENTAL_WEBAUTHN_PLUGIN_PERFORM_UV_OPERATION_TYPE(2i32);
windows_core::imp::define_interface!(IPluginAuthenticator, IPluginAuthenticator_Vtbl, 0xd26bcf6f_b54c_43ff_9f06_d5bf148625f7);
windows_core::imp::interface_hierarchy!(IPluginAuthenticator, windows_core::IUnknown);
impl IPluginAuthenticator {
    pub unsafe fn MakeCredential(&self, request: *const WEBAUTHN_PLUGIN_OPERATION_REQUEST) -> windows_core::Result<WEBAUTHN_PLUGIN_OPERATION_RESPONSE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MakeCredential)(windows_core::Interface::as_raw(self), request, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetAssertion(&self, request: *const WEBAUTHN_PLUGIN_OPERATION_REQUEST) -> windows_core::Result<WEBAUTHN_PLUGIN_OPERATION_RESPONSE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAssertion)(windows_core::Interface::as_raw(self), request, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CancelOperation(&self, request: *const WEBAUTHN_PLUGIN_CANCEL_OPERATION_REQUEST) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CancelOperation)(windows_core::Interface::as_raw(self), request).ok() }
    }
    pub unsafe fn GetLockStatus(&self) -> windows_core::Result<PLUGIN_LOCK_STATUS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLockStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPluginAuthenticator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub MakeCredential: unsafe extern "system" fn(*mut core::ffi::c_void, *const WEBAUTHN_PLUGIN_OPERATION_REQUEST, *mut WEBAUTHN_PLUGIN_OPERATION_RESPONSE) -> windows_core::HRESULT,
    pub GetAssertion: unsafe extern "system" fn(*mut core::ffi::c_void, *const WEBAUTHN_PLUGIN_OPERATION_REQUEST, *mut WEBAUTHN_PLUGIN_OPERATION_RESPONSE) -> windows_core::HRESULT,
    pub CancelOperation: unsafe extern "system" fn(*mut core::ffi::c_void, *const WEBAUTHN_PLUGIN_CANCEL_OPERATION_REQUEST) -> windows_core::HRESULT,
    pub GetLockStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PLUGIN_LOCK_STATUS) -> windows_core::HRESULT,
}
pub trait IPluginAuthenticator_Impl: windows_core::IUnknownImpl {
    fn MakeCredential(&self, request: *const WEBAUTHN_PLUGIN_OPERATION_REQUEST) -> windows_core::Result<WEBAUTHN_PLUGIN_OPERATION_RESPONSE>;
    fn GetAssertion(&self, request: *const WEBAUTHN_PLUGIN_OPERATION_REQUEST) -> windows_core::Result<WEBAUTHN_PLUGIN_OPERATION_RESPONSE>;
    fn CancelOperation(&self, request: *const WEBAUTHN_PLUGIN_CANCEL_OPERATION_REQUEST) -> windows_core::Result<()>;
    fn GetLockStatus(&self) -> windows_core::Result<PLUGIN_LOCK_STATUS>;
}
impl IPluginAuthenticator_Vtbl {
    pub const fn new<Identity: IPluginAuthenticator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn MakeCredential<Identity: IPluginAuthenticator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, request: *const WEBAUTHN_PLUGIN_OPERATION_REQUEST, response: *mut WEBAUTHN_PLUGIN_OPERATION_RESPONSE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPluginAuthenticator_Impl::MakeCredential(this, core::mem::transmute_copy(&request)) {
                    Ok(ok__) => {
                        response.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAssertion<Identity: IPluginAuthenticator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, request: *const WEBAUTHN_PLUGIN_OPERATION_REQUEST, response: *mut WEBAUTHN_PLUGIN_OPERATION_RESPONSE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPluginAuthenticator_Impl::GetAssertion(this, core::mem::transmute_copy(&request)) {
                    Ok(ok__) => {
                        response.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CancelOperation<Identity: IPluginAuthenticator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, request: *const WEBAUTHN_PLUGIN_CANCEL_OPERATION_REQUEST) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPluginAuthenticator_Impl::CancelOperation(this, core::mem::transmute_copy(&request)).into()
            }
        }
        unsafe extern "system" fn GetLockStatus<Identity: IPluginAuthenticator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lockstatus: *mut PLUGIN_LOCK_STATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPluginAuthenticator_Impl::GetLockStatus(this) {
                    Ok(ok__) => {
                        lockstatus.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            MakeCredential: MakeCredential::<Identity, OFFSET>,
            GetAssertion: GetAssertion::<Identity, OFFSET>,
            CancelOperation: CancelOperation::<Identity, OFFSET>,
            GetLockStatus: GetLockStatus::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPluginAuthenticator as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPluginAuthenticator {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PLUGIN_LOCK_STATUS(pub i32);
pub const PerformUserVerification: WEBAUTHN_PLUGIN_PERFORM_UV_OPERATION_TYPE = WEBAUTHN_PLUGIN_PERFORM_UV_OPERATION_TYPE(1i32);
pub const PerformUv: EXPERIMENTAL_WEBAUTHN_PLUGIN_PERFORM_UV_OPERATION_TYPE = EXPERIMENTAL_WEBAUTHN_PLUGIN_PERFORM_UV_OPERATION_TYPE(1i32);
pub const PluginAuthenticatorState_Disabled: EXPERIMENTAL_PLUGIN_AUTHENTICATOR_STATE = EXPERIMENTAL_PLUGIN_AUTHENTICATOR_STATE(1i32);
pub const PluginAuthenticatorState_Enabled: EXPERIMENTAL_PLUGIN_AUTHENTICATOR_STATE = EXPERIMENTAL_PLUGIN_AUTHENTICATOR_STATE(2i32);
pub const PluginAuthenticatorState_Unknown: EXPERIMENTAL_PLUGIN_AUTHENTICATOR_STATE = EXPERIMENTAL_PLUGIN_AUTHENTICATOR_STATE(0i32);
pub const PluginLocked: PLUGIN_LOCK_STATUS = PLUGIN_LOCK_STATUS(0i32);
pub const PluginUnlocked: PLUGIN_LOCK_STATUS = PLUGIN_LOCK_STATUS(1i32);
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub const WEBAUTHN_ATTESTATION_TYPE_NONE: windows_core::PCWSTR = windows_core::w!("none");
pub const WEBAUTHN_ATTESTATION_TYPE_PACKED: windows_core::PCWSTR = windows_core::w!("packed");
pub const WEBAUTHN_ATTESTATION_TYPE_TPM: windows_core::PCWSTR = windows_core::w!("tpm");
pub const WEBAUTHN_ATTESTATION_TYPE_U2F: windows_core::PCWSTR = windows_core::w!("fido-u2f");
pub const WEBAUTHN_ATTESTATION_VER_TPM_2_0: windows_core::PCWSTR = windows_core::w!("2.0");
pub const WEBAUTHN_AUTHENTICATOR_ATTACHMENT_ANY: u32 = 0u32;
pub const WEBAUTHN_AUTHENTICATOR_ATTACHMENT_CROSS_PLATFORM: u32 = 2u32;
pub const WEBAUTHN_AUTHENTICATOR_ATTACHMENT_CROSS_PLATFORM_U2F_V2: u32 = 3u32;
pub const WEBAUTHN_AUTHENTICATOR_ATTACHMENT_PLATFORM: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WEBAUTHN_AUTHENTICATOR_DETAILS {
    pub dwVersion: u32,
    pub cbAuthenticatorId: u32,
    pub pbAuthenticatorId: *mut u8,
    pub pwszAuthenticatorName: windows_core::PCWSTR,
    pub cbAuthenticatorLogo: u32,
    pub pbAuthenticatorLogo: *mut u8,
    pub bLocked: windows_core::BOOL,
}
impl Default for WEBAUTHN_AUTHENTICATOR_DETAILS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WEBAUTHN_AUTHENTICATOR_DETAILS_CURRENT_VERSION: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WEBAUTHN_AUTHENTICATOR_DETAILS_OPTIONS {
    pub dwVersion: u32,
}
pub const WEBAUTHN_AUTHENTICATOR_DETAILS_OPTIONS_CURRENT_VERSION: u32 = 1u32;
pub const WEBAUTHN_AUTHENTICATOR_DETAILS_OPTIONS_VERSION_1: u32 = 1u32;
pub const WEBAUTHN_AUTHENTICATOR_DETAILS_VERSION_1: u32 = 1u32;
#[repr(C)]
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
    pub pAllowCredentialList: *mut WEBAUTHN_CREDENTIAL_LIST,
    pub dwCredLargeBlobOperation: u32,
    pub cbCredLargeBlob: u32,
    pub pbCredLargeBlob: *mut u8,
    pub pHmacSecretSaltValues: *mut WEBAUTHN_HMAC_SECRET_SALT_VALUES,
    pub bBrowserInPrivateMode: windows_core::BOOL,
    pub pLinkedDevice: *mut CTAPCBOR_HYBRID_STORAGE_LINKED_DATA,
    pub bAutoFill: windows_core::BOOL,
    pub cbJsonExt: u32,
    pub pbJsonExt: *mut u8,
    pub cCredentialHints: u32,
    pub ppwszCredentialHints: *const windows_core::PCWSTR,
    pub pwszRemoteWebOrigin: windows_core::PCWSTR,
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
    pub pExcludeCredentialList: *mut WEBAUTHN_CREDENTIAL_LIST,
    pub dwEnterpriseAttestation: u32,
    pub dwLargeBlobSupport: u32,
    pub bPreferResidentKey: windows_core::BOOL,
    pub bBrowserInPrivateMode: windows_core::BOOL,
    pub bEnablePrf: windows_core::BOOL,
    pub pLinkedDevice: *mut CTAPCBOR_HYBRID_STORAGE_LINKED_DATA,
    pub cbJsonExt: u32,
    pub pbJsonExt: *mut u8,
    pub pPRFGlobalEval: *mut WEBAUTHN_HMAC_SECRET_SALT,
    pub cCredentialHints: u32,
    pub ppwszCredentialHints: *const windows_core::PCWSTR,
    pub bThirdPartyPayment: windows_core::BOOL,
    pub pwszRemoteWebOrigin: windows_core::PCWSTR,
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WEBAUTHN_CLIENT_DATA {
    pub dwVersion: u32,
    pub cbClientDataJSON: u32,
    pub pbClientDataJSON: *mut u8,
    pub pwszHashAlgId: windows_core::PCWSTR,
}
impl Default for WEBAUTHN_CLIENT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WEBAUTHN_CLIENT_DATA_CURRENT_VERSION: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WEBAUTHN_COMMON_ATTESTATION {
    pub dwVersion: u32,
    pub pwszAlg: windows_core::PCWSTR,
    pub lAlg: i32,
    pub cbSignature: u32,
    pub pbSignature: *mut u8,
    pub cX5c: u32,
    pub pX5c: *mut WEBAUTHN_X5C,
    pub pwszVer: windows_core::PCWSTR,
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WEBAUTHN_COSE_CREDENTIAL_PARAMETER {
    pub dwVersion: u32,
    pub pwszCredentialType: windows_core::PCWSTR,
    pub lAlg: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WEBAUTHN_CREDENTIAL {
    pub dwVersion: u32,
    pub cbId: u32,
    pub pbId: *mut u8,
    pub pwszCredentialType: windows_core::PCWSTR,
}
impl Default for WEBAUTHN_CREDENTIAL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WEBAUTHN_CREDENTIAL_ATTESTATION {
    pub dwVersion: u32,
    pub pwszFormatType: windows_core::PCWSTR,
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
    pub bEpAtt: windows_core::BOOL,
    pub bLargeBlobSupported: windows_core::BOOL,
    pub bResidentKey: windows_core::BOOL,
    pub bPrfEnabled: windows_core::BOOL,
    pub cbUnsignedExtensionOutputs: u32,
    pub pbUnsignedExtensionOutputs: *mut u8,
    pub pHmacSecret: *mut WEBAUTHN_HMAC_SECRET_SALT,
    pub bThirdPartyPayment: windows_core::BOOL,
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WEBAUTHN_CREDENTIAL_DETAILS {
    pub dwVersion: u32,
    pub cbCredentialID: u32,
    pub pbCredentialID: *mut u8,
    pub pRpInformation: *mut WEBAUTHN_RP_ENTITY_INFORMATION,
    pub pUserInformation: *mut WEBAUTHN_USER_ENTITY_INFORMATION,
    pub bRemovable: windows_core::BOOL,
    pub bBackedUp: windows_core::BOOL,
    pub pwszAuthenticatorName: windows_core::PCWSTR,
    pub cbAuthenticatorLogo: u32,
    pub pbAuthenticatorLogo: *mut u8,
    pub bThirdPartyPayment: windows_core::BOOL,
    pub dwTransports: u32,
}
impl Default for WEBAUTHN_CREDENTIAL_DETAILS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WEBAUTHN_CREDENTIAL_DETAILS_CURRENT_VERSION: u32 = 4u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WEBAUTHN_CREDENTIAL_EX {
    pub dwVersion: u32,
    pub cbId: u32,
    pub pbId: *mut u8,
    pub pwszCredentialType: windows_core::PCWSTR,
    pub dwTransports: u32,
}
impl Default for WEBAUTHN_CREDENTIAL_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WEBAUTHN_CREDENTIAL_EX_CURRENT_VERSION: u32 = 1u32;
pub const WEBAUTHN_CREDENTIAL_HINT_CLIENT_DEVICE: windows_core::PCWSTR = windows_core::w!("client-device");
pub const WEBAUTHN_CREDENTIAL_HINT_HYBRID: windows_core::PCWSTR = windows_core::w!("hybrid");
pub const WEBAUTHN_CREDENTIAL_HINT_SECURITY_KEY: windows_core::PCWSTR = windows_core::w!("security-key");
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WEBAUTHN_CREDENTIAL_LIST {
    pub cCredentials: u32,
    pub ppCredentials: *mut *mut WEBAUTHN_CREDENTIAL_EX,
}
impl Default for WEBAUTHN_CREDENTIAL_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WEBAUTHN_CREDENTIAL_TYPE_PUBLIC_KEY: windows_core::PCWSTR = windows_core::w!("public-key");
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WEBAUTHN_CRED_PROTECT_EXTENSION_IN {
    pub dwCredProtect: u32,
    pub bRequireCredProtect: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WEBAUTHN_CTAPCBOR_AUTHENTICATOR_OPTIONS {
    pub dwVersion: u32,
    pub lUp: i32,
    pub lUv: i32,
    pub lRequireResidentKey: i32,
}
pub const WEBAUTHN_CTAPCBOR_AUTHENTICATOR_OPTIONS_CURRENT_VERSION: u32 = 1u32;
pub const WEBAUTHN_CTAPCBOR_AUTHENTICATOR_OPTIONS_VERSION_1: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WEBAUTHN_CTAPCBOR_GET_ASSERTION_REQUEST {
    pub dwVersion: u32,
    pub pwszRpId: windows_core::PCWSTR,
    pub cbRpId: u32,
    pub pbRpId: *mut u8,
    pub cbClientDataHash: u32,
    pub pbClientDataHash: *mut u8,
    pub CredentialList: WEBAUTHN_CREDENTIAL_LIST,
    pub cbCborExtensionsMap: u32,
    pub pbCborExtensionsMap: *mut u8,
    pub pAuthenticatorOptions: *mut WEBAUTHN_CTAPCBOR_AUTHENTICATOR_OPTIONS,
    pub fEmptyPinAuth: windows_core::BOOL,
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
    pub fEmptyPinAuth: windows_core::BOOL,
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
pub const WEBAUTHN_CTAP_TRANSPORT_BLE_STRING: windows_core::PCSTR = windows_core::s!("ble");
pub const WEBAUTHN_CTAP_TRANSPORT_FLAGS_MASK: u32 = 127u32;
pub const WEBAUTHN_CTAP_TRANSPORT_HYBRID: u32 = 32u32;
pub const WEBAUTHN_CTAP_TRANSPORT_HYBRID_STRING: windows_core::PCSTR = windows_core::s!("hybrid");
pub const WEBAUTHN_CTAP_TRANSPORT_INTERNAL: u32 = 16u32;
pub const WEBAUTHN_CTAP_TRANSPORT_INTERNAL_STRING: windows_core::PCSTR = windows_core::s!("internal");
pub const WEBAUTHN_CTAP_TRANSPORT_NFC: u32 = 2u32;
pub const WEBAUTHN_CTAP_TRANSPORT_NFC_STRING: windows_core::PCSTR = windows_core::s!("nfc");
pub const WEBAUTHN_CTAP_TRANSPORT_SMART_CARD: u32 = 64u32;
pub const WEBAUTHN_CTAP_TRANSPORT_SMART_CARD_STRING: windows_core::PCSTR = windows_core::s!("smart-card");
pub const WEBAUTHN_CTAP_TRANSPORT_TEST: u32 = 8u32;
pub const WEBAUTHN_CTAP_TRANSPORT_USB: u32 = 1u32;
pub const WEBAUTHN_CTAP_TRANSPORT_USB_STRING: windows_core::PCSTR = windows_core::s!("usb");
pub const WEBAUTHN_ENTERPRISE_ATTESTATION_NONE: u32 = 0u32;
pub const WEBAUTHN_ENTERPRISE_ATTESTATION_PLATFORM_MANAGED: u32 = 2u32;
pub const WEBAUTHN_ENTERPRISE_ATTESTATION_VENDOR_FACILITATED: u32 = 1u32;
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WEBAUTHN_EXTENSIONS {
    pub cExtensions: u32,
    pub pExtensions: *mut WEBAUTHN_EXTENSION,
}
impl Default for WEBAUTHN_EXTENSIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
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
pub const WEBAUTHN_GET_CREDENTIALS_OPTIONS_CURRENT_VERSION: u32 = 1u32;
pub const WEBAUTHN_GET_CREDENTIALS_OPTIONS_VERSION_1: u32 = 1u32;
pub const WEBAUTHN_HASH_ALGORITHM_SHA_256: windows_core::PCWSTR = windows_core::w!("SHA-256");
pub const WEBAUTHN_HASH_ALGORITHM_SHA_384: windows_core::PCWSTR = windows_core::w!("SHA-384");
pub const WEBAUTHN_HASH_ALGORITHM_SHA_512: windows_core::PCWSTR = windows_core::w!("SHA-512");
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WEBAUTHN_PLUGIN_ADD_AUTHENTICATOR_OPTIONS {
    pub pwszAuthenticatorName: windows_core::PCWSTR,
    pub rclsid: *const windows_core::GUID,
    pub pwszPluginRpId: windows_core::PCWSTR,
    pub pwszLightThemeLogoSvg: windows_core::PCWSTR,
    pub pwszDarkThemeLogoSvg: windows_core::PCWSTR,
    pub cbAuthenticatorInfo: u32,
    pub pbAuthenticatorInfo: *const u8,
    pub cSupportedRpIds: u32,
    pub ppwszSupportedRpIds: *const windows_core::PCWSTR,
}
impl Default for WEBAUTHN_PLUGIN_ADD_AUTHENTICATOR_OPTIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WEBAUTHN_PLUGIN_CANCEL_OPERATION_REQUEST {
    pub transactionId: windows_core::GUID,
    pub cbRequestSignature: u32,
    pub pbRequestSignature: *mut u8,
}
impl Default for WEBAUTHN_PLUGIN_CANCEL_OPERATION_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WEBAUTHN_PLUGIN_CREDENTIAL_DETAILS {
    pub cbCredentialId: u32,
    pub pbCredentialId: *const u8,
    pub pwszRpId: windows_core::PCWSTR,
    pub pwszRpName: windows_core::PCWSTR,
    pub cbUserId: u32,
    pub pbUserId: *const u8,
    pub pwszUserName: windows_core::PCWSTR,
    pub pwszUserDisplayName: windows_core::PCWSTR,
}
impl Default for WEBAUTHN_PLUGIN_CREDENTIAL_DETAILS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WEBAUTHN_PLUGIN_OPERATION_REQUEST {
    pub hWnd: super::super::super::Foundation::HWND,
    pub transactionId: windows_core::GUID,
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WEBAUTHN_PLUGIN_OPERATION_RESPONSE {
    pub cbEncodedResponse: u32,
    pub pbEncodedResponse: *mut u8,
}
impl Default for WEBAUTHN_PLUGIN_OPERATION_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WEBAUTHN_PLUGIN_PERFORM_UV_OPERATION_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WEBAUTHN_PLUGIN_REQUEST_TYPE(pub i32);
pub const WEBAUTHN_PLUGIN_REQUEST_TYPE_CTAP2_CBOR: WEBAUTHN_PLUGIN_REQUEST_TYPE = WEBAUTHN_PLUGIN_REQUEST_TYPE(1i32);
pub type WEBAUTHN_PLUGIN_STATUS_CHANGE_CALLBACK = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void)>;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WEBAUTHN_PLUGIN_UPDATE_AUTHENTICATOR_DETAILS {
    pub pwszAuthenticatorName: windows_core::PCWSTR,
    pub rclsid: *const windows_core::GUID,
    pub rclsidNew: *const windows_core::GUID,
    pub pwszLightThemeLogoSvg: windows_core::PCWSTR,
    pub pwszDarkThemeLogoSvg: windows_core::PCWSTR,
    pub cbAuthenticatorInfo: u32,
    pub pbAuthenticatorInfo: *const u8,
    pub cSupportedRpIds: u32,
    pub ppwszSupportedRpIds: *const windows_core::PCWSTR,
}
impl Default for WEBAUTHN_PLUGIN_UPDATE_AUTHENTICATOR_DETAILS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WEBAUTHN_PLUGIN_USER_VERIFICATION_REQUEST {
    pub hwnd: super::super::super::Foundation::HWND,
    pub rguidTransactionId: *const windows_core::GUID,
    pub pwszUsername: windows_core::PCWSTR,
    pub pwszDisplayHint: windows_core::PCWSTR,
}
impl Default for WEBAUTHN_PLUGIN_USER_VERIFICATION_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WEBAUTHN_RP_ENTITY_INFORMATION {
    pub dwVersion: u32,
    pub pwszId: windows_core::PCWSTR,
    pub pwszName: windows_core::PCWSTR,
    pub pwszIcon: windows_core::PCWSTR,
}
pub const WEBAUTHN_RP_ENTITY_INFORMATION_CURRENT_VERSION: u32 = 1u32;
pub const WEBAUTHN_RP_ENTITY_INFORMATION_VERSION_1: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WEBAUTHN_USER_ENTITY_INFORMATION {
    pub dwVersion: u32,
    pub cbId: u32,
    pub pbId: *mut u8,
    pub pwszName: windows_core::PCWSTR,
    pub pwszIcon: windows_core::PCWSTR,
    pub pwszDisplayName: windows_core::PCWSTR,
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WEBAUTHN_X5C {
    pub cbData: u32,
    pub pbData: *mut u8,
}
impl Default for WEBAUTHN_X5C {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
