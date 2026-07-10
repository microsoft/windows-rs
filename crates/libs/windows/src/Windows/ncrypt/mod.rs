#[cfg(feature = "bcrypt")]
#[inline]
pub unsafe fn NCryptCreateClaim(hsubjectkey: Option<NCRYPT_KEY_HANDLE>, hauthoritykey: Option<NCRYPT_KEY_HANDLE>, dwclaimtype: u32, pparameterlist: Option<*const NCryptBufferDesc>, pbclaimblob: Option<&mut [u8]>, pcbresult: *mut u32, dwflags: u32) -> SECURITY_STATUS {
    windows_core::link!("ncrypt.dll" "system" fn NCryptCreateClaim(hsubjectkey : NCRYPT_KEY_HANDLE, hauthoritykey : NCRYPT_KEY_HANDLE, dwclaimtype : u32, pparameterlist : *const NCryptBufferDesc, pbclaimblob : *mut u8, cbclaimblob : u32, pcbresult : *mut u32, dwflags : u32) -> SECURITY_STATUS);
    unsafe { NCryptCreateClaim(hsubjectkey.unwrap_or(core::mem::zeroed()) as _, hauthoritykey.unwrap_or(core::mem::zeroed()) as _, dwclaimtype, pparameterlist.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pbclaimblob.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pbclaimblob.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbresult as _, dwflags) }
}
#[inline]
pub unsafe fn NCryptCreatePersistedKey<P2, P3>(hprovider: NCRYPT_PROV_HANDLE, phkey: *mut NCRYPT_KEY_HANDLE, pszalgid: P2, pszkeyname: P3, dwlegacykeyspec: u32, dwflags: u32) -> SECURITY_STATUS
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ncrypt.dll" "system" fn NCryptCreatePersistedKey(hprovider : NCRYPT_PROV_HANDLE, phkey : *mut NCRYPT_KEY_HANDLE, pszalgid : windows_core::PCWSTR, pszkeyname : windows_core::PCWSTR, dwlegacykeyspec : u32, dwflags : u32) -> SECURITY_STATUS);
    unsafe { NCryptCreatePersistedKey(hprovider, phkey as _, pszalgid.param().abi(), pszkeyname.param().abi(), dwlegacykeyspec, dwflags) }
}
#[inline]
pub unsafe fn NCryptDecapsulate(hkey: NCRYPT_KEY_HANDLE, pbciphertext: &[u8], pbsecretkey: Option<&mut [u8]>, pcbsecretkey: *mut u32, dwflags: u32) -> SECURITY_STATUS {
    windows_core::link!("ncrypt.dll" "system" fn NCryptDecapsulate(hkey : NCRYPT_KEY_HANDLE, pbciphertext : *const u8, cbciphertext : u32, pbsecretkey : *mut u8, cbsecretkey : u32, pcbsecretkey : *mut u32, dwflags : u32) -> SECURITY_STATUS);
    unsafe { NCryptDecapsulate(hkey, core::mem::transmute(pbciphertext.as_ptr()), pbciphertext.len().try_into().unwrap(), core::mem::transmute(pbsecretkey.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pbsecretkey.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbsecretkey as _, dwflags) }
}
#[inline]
pub unsafe fn NCryptDecrypt(hkey: NCRYPT_KEY_HANDLE, pbinput: Option<&[u8]>, ppaddinginfo: Option<*const core::ffi::c_void>, pboutput: Option<&mut [u8]>, pcbresult: *mut u32, dwflags: u32) -> SECURITY_STATUS {
    windows_core::link!("ncrypt.dll" "system" fn NCryptDecrypt(hkey : NCRYPT_KEY_HANDLE, pbinput : *const u8, cbinput : u32, ppaddinginfo : *const core::ffi::c_void, pboutput : *mut u8, cboutput : u32, pcbresult : *mut u32, dwflags : u32) -> SECURITY_STATUS);
    unsafe { NCryptDecrypt(hkey, core::mem::transmute(pbinput.map_or(core::ptr::null(), |slice| slice.as_ptr())), pbinput.map_or(0, |slice| slice.len().try_into().unwrap()), ppaddinginfo.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pboutput.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pboutput.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbresult as _, dwflags) }
}
#[inline]
pub unsafe fn NCryptDeleteKey(hkey: NCRYPT_KEY_HANDLE, dwflags: u32) -> SECURITY_STATUS {
    windows_core::link!("ncrypt.dll" "system" fn NCryptDeleteKey(hkey : NCRYPT_KEY_HANDLE, dwflags : u32) -> SECURITY_STATUS);
    unsafe { NCryptDeleteKey(hkey, dwflags) }
}
#[cfg(feature = "bcrypt")]
#[inline]
pub unsafe fn NCryptDeriveKey<P1>(hsharedsecret: NCRYPT_SECRET_HANDLE, pwszkdf: P1, pparameterlist: Option<*const NCryptBufferDesc>, pbderivedkey: Option<&mut [u8]>, pcbresult: *mut u32, dwflags: u32) -> SECURITY_STATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ncrypt.dll" "system" fn NCryptDeriveKey(hsharedsecret : NCRYPT_SECRET_HANDLE, pwszkdf : windows_core::PCWSTR, pparameterlist : *const NCryptBufferDesc, pbderivedkey : *mut u8, cbderivedkey : u32, pcbresult : *mut u32, dwflags : u32) -> SECURITY_STATUS);
    unsafe { NCryptDeriveKey(hsharedsecret, pwszkdf.param().abi(), pparameterlist.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pbderivedkey.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pbderivedkey.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbresult as _, dwflags) }
}
#[inline]
pub unsafe fn NCryptEncapsulate(hkey: NCRYPT_KEY_HANDLE, pbsecretkey: Option<&mut [u8]>, pcbsecretkey: *mut u32, pbciphertext: Option<&mut [u8]>, pcbciphertext: *mut u32, dwflags: u32) -> SECURITY_STATUS {
    windows_core::link!("ncrypt.dll" "system" fn NCryptEncapsulate(hkey : NCRYPT_KEY_HANDLE, pbsecretkey : *mut u8, cbsecretkey : u32, pcbsecretkey : *mut u32, pbciphertext : *mut u8, cbciphertext : u32, pcbciphertext : *mut u32, dwflags : u32) -> SECURITY_STATUS);
    unsafe { NCryptEncapsulate(hkey, core::mem::transmute(pbsecretkey.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pbsecretkey.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbsecretkey as _, core::mem::transmute(pbciphertext.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pbciphertext.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbciphertext as _, dwflags) }
}
#[inline]
pub unsafe fn NCryptEncrypt(hkey: NCRYPT_KEY_HANDLE, pbinput: Option<&[u8]>, ppaddinginfo: Option<*const core::ffi::c_void>, pboutput: Option<&mut [u8]>, pcbresult: *mut u32, dwflags: u32) -> SECURITY_STATUS {
    windows_core::link!("ncrypt.dll" "system" fn NCryptEncrypt(hkey : NCRYPT_KEY_HANDLE, pbinput : *const u8, cbinput : u32, ppaddinginfo : *const core::ffi::c_void, pboutput : *mut u8, cboutput : u32, pcbresult : *mut u32, dwflags : u32) -> SECURITY_STATUS);
    unsafe { NCryptEncrypt(hkey, core::mem::transmute(pbinput.map_or(core::ptr::null(), |slice| slice.as_ptr())), pbinput.map_or(0, |slice| slice.len().try_into().unwrap()), ppaddinginfo.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pboutput.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pboutput.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbresult as _, dwflags) }
}
#[inline]
pub unsafe fn NCryptEnumAlgorithms(hprovider: NCRYPT_PROV_HANDLE, dwalgoperations: u32, pdwalgcount: *mut u32, ppalglist: *mut *mut NCryptAlgorithmName, dwflags: u32) -> SECURITY_STATUS {
    windows_core::link!("ncrypt.dll" "system" fn NCryptEnumAlgorithms(hprovider : NCRYPT_PROV_HANDLE, dwalgoperations : u32, pdwalgcount : *mut u32, ppalglist : *mut *mut NCryptAlgorithmName, dwflags : u32) -> SECURITY_STATUS);
    unsafe { NCryptEnumAlgorithms(hprovider, dwalgoperations, pdwalgcount as _, ppalglist as _, dwflags) }
}
#[inline]
pub unsafe fn NCryptEnumKeys<P1>(hprovider: NCRYPT_PROV_HANDLE, pszscope: P1, ppkeyname: *mut *mut NCryptKeyName, ppenumstate: *mut *mut core::ffi::c_void, dwflags: u32) -> SECURITY_STATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ncrypt.dll" "system" fn NCryptEnumKeys(hprovider : NCRYPT_PROV_HANDLE, pszscope : windows_core::PCWSTR, ppkeyname : *mut *mut NCryptKeyName, ppenumstate : *mut *mut core::ffi::c_void, dwflags : u32) -> SECURITY_STATUS);
    unsafe { NCryptEnumKeys(hprovider, pszscope.param().abi(), ppkeyname as _, ppenumstate as _, dwflags) }
}
#[inline]
pub unsafe fn NCryptEnumStorageProviders(pdwprovidercount: *mut u32, ppproviderlist: *mut *mut NCryptProviderName, dwflags: u32) -> SECURITY_STATUS {
    windows_core::link!("ncrypt.dll" "system" fn NCryptEnumStorageProviders(pdwprovidercount : *mut u32, ppproviderlist : *mut *mut NCryptProviderName, dwflags : u32) -> SECURITY_STATUS);
    unsafe { NCryptEnumStorageProviders(pdwprovidercount as _, ppproviderlist as _, dwflags) }
}
#[cfg(feature = "bcrypt")]
#[inline]
pub unsafe fn NCryptExportKey<P2>(hkey: NCRYPT_KEY_HANDLE, hexportkey: Option<NCRYPT_KEY_HANDLE>, pszblobtype: P2, pparameterlist: Option<*const NCryptBufferDesc>, pboutput: Option<&mut [u8]>, pcbresult: *mut u32, dwflags: u32) -> SECURITY_STATUS
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ncrypt.dll" "system" fn NCryptExportKey(hkey : NCRYPT_KEY_HANDLE, hexportkey : NCRYPT_KEY_HANDLE, pszblobtype : windows_core::PCWSTR, pparameterlist : *const NCryptBufferDesc, pboutput : *mut u8, cboutput : u32, pcbresult : *mut u32, dwflags : u32) -> SECURITY_STATUS);
    unsafe { NCryptExportKey(hkey, hexportkey.unwrap_or(core::mem::zeroed()) as _, pszblobtype.param().abi(), pparameterlist.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pboutput.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pboutput.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbresult as _, dwflags) }
}
#[inline]
pub unsafe fn NCryptFinalizeKey(hkey: NCRYPT_KEY_HANDLE, dwflags: u32) -> SECURITY_STATUS {
    windows_core::link!("ncrypt.dll" "system" fn NCryptFinalizeKey(hkey : NCRYPT_KEY_HANDLE, dwflags : u32) -> SECURITY_STATUS);
    unsafe { NCryptFinalizeKey(hkey, dwflags) }
}
#[inline]
pub unsafe fn NCryptFreeBuffer(pvinput: *mut core::ffi::c_void) -> SECURITY_STATUS {
    windows_core::link!("ncrypt.dll" "system" fn NCryptFreeBuffer(pvinput : *mut core::ffi::c_void) -> SECURITY_STATUS);
    unsafe { NCryptFreeBuffer(pvinput as _) }
}
#[inline]
pub unsafe fn NCryptFreeObject(hobject: NCRYPT_HANDLE) -> SECURITY_STATUS {
    windows_core::link!("ncrypt.dll" "system" fn NCryptFreeObject(hobject : NCRYPT_HANDLE) -> SECURITY_STATUS);
    unsafe { NCryptFreeObject(hobject) }
}
#[inline]
pub unsafe fn NCryptGetProperty<P1>(hobject: NCRYPT_HANDLE, pszproperty: P1, pboutput: Option<&mut [u8]>, pcbresult: *mut u32, dwflags: u32) -> SECURITY_STATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ncrypt.dll" "system" fn NCryptGetProperty(hobject : NCRYPT_HANDLE, pszproperty : windows_core::PCWSTR, pboutput : *mut u8, cboutput : u32, pcbresult : *mut u32, dwflags : u32) -> SECURITY_STATUS);
    unsafe { NCryptGetProperty(hobject, pszproperty.param().abi(), core::mem::transmute(pboutput.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pboutput.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbresult as _, dwflags) }
}
#[cfg(feature = "bcrypt")]
#[inline]
pub unsafe fn NCryptImportKey<P2>(hprovider: NCRYPT_PROV_HANDLE, himportkey: Option<NCRYPT_KEY_HANDLE>, pszblobtype: P2, pparameterlist: Option<*const NCryptBufferDesc>, phkey: *mut NCRYPT_KEY_HANDLE, pbdata: &[u8], dwflags: u32) -> SECURITY_STATUS
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ncrypt.dll" "system" fn NCryptImportKey(hprovider : NCRYPT_PROV_HANDLE, himportkey : NCRYPT_KEY_HANDLE, pszblobtype : windows_core::PCWSTR, pparameterlist : *const NCryptBufferDesc, phkey : *mut NCRYPT_KEY_HANDLE, pbdata : *const u8, cbdata : u32, dwflags : u32) -> SECURITY_STATUS);
    unsafe { NCryptImportKey(hprovider, himportkey.unwrap_or(core::mem::zeroed()) as _, pszblobtype.param().abi(), pparameterlist.unwrap_or(core::mem::zeroed()) as _, phkey as _, core::mem::transmute(pbdata.as_ptr()), pbdata.len().try_into().unwrap(), dwflags) }
}
#[inline]
pub unsafe fn NCryptIsAlgSupported<P1>(hprovider: NCRYPT_PROV_HANDLE, pszalgid: P1, dwflags: u32) -> SECURITY_STATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ncrypt.dll" "system" fn NCryptIsAlgSupported(hprovider : NCRYPT_PROV_HANDLE, pszalgid : windows_core::PCWSTR, dwflags : u32) -> SECURITY_STATUS);
    unsafe { NCryptIsAlgSupported(hprovider, pszalgid.param().abi(), dwflags) }
}
#[inline]
pub unsafe fn NCryptIsKeyHandle(hkey: NCRYPT_KEY_HANDLE) -> windows_core::BOOL {
    windows_core::link!("ncrypt.dll" "system" fn NCryptIsKeyHandle(hkey : NCRYPT_KEY_HANDLE) -> windows_core::BOOL);
    unsafe { NCryptIsKeyHandle(hkey) }
}
#[cfg(feature = "bcrypt")]
#[inline]
pub unsafe fn NCryptKeyDerivation(hkey: NCRYPT_KEY_HANDLE, pparameterlist: Option<*const NCryptBufferDesc>, pbderivedkey: &mut [u8], pcbresult: *mut u32, dwflags: u32) -> SECURITY_STATUS {
    windows_core::link!("ncrypt.dll" "system" fn NCryptKeyDerivation(hkey : NCRYPT_KEY_HANDLE, pparameterlist : *const NCryptBufferDesc, pbderivedkey : *mut u8, cbderivedkey : u32, pcbresult : *mut u32, dwflags : u32) -> SECURITY_STATUS);
    unsafe { NCryptKeyDerivation(hkey, pparameterlist.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pbderivedkey.as_ptr()), pbderivedkey.len().try_into().unwrap(), pcbresult as _, dwflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn NCryptNotifyChangeKey(hprovider: NCRYPT_PROV_HANDLE, phevent: *mut super::winnt::HANDLE, dwflags: u32) -> SECURITY_STATUS {
    windows_core::link!("ncrypt.dll" "system" fn NCryptNotifyChangeKey(hprovider : NCRYPT_PROV_HANDLE, phevent : *mut super::winnt::HANDLE, dwflags : u32) -> SECURITY_STATUS);
    unsafe { NCryptNotifyChangeKey(hprovider, phevent as _, dwflags) }
}
#[inline]
pub unsafe fn NCryptOpenKey<P2>(hprovider: NCRYPT_PROV_HANDLE, phkey: *mut NCRYPT_KEY_HANDLE, pszkeyname: P2, dwlegacykeyspec: Option<u32>, dwflags: u32) -> SECURITY_STATUS
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ncrypt.dll" "system" fn NCryptOpenKey(hprovider : NCRYPT_PROV_HANDLE, phkey : *mut NCRYPT_KEY_HANDLE, pszkeyname : windows_core::PCWSTR, dwlegacykeyspec : u32, dwflags : u32) -> SECURITY_STATUS);
    unsafe { NCryptOpenKey(hprovider, phkey as _, pszkeyname.param().abi(), dwlegacykeyspec.unwrap_or(core::mem::zeroed()) as _, dwflags) }
}
#[inline]
pub unsafe fn NCryptOpenStorageProvider<P1>(phprovider: *mut NCRYPT_PROV_HANDLE, pszprovidername: P1, dwflags: u32) -> SECURITY_STATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ncrypt.dll" "system" fn NCryptOpenStorageProvider(phprovider : *mut NCRYPT_PROV_HANDLE, pszprovidername : windows_core::PCWSTR, dwflags : u32) -> SECURITY_STATUS);
    unsafe { NCryptOpenStorageProvider(phprovider as _, pszprovidername.param().abi(), dwflags) }
}
#[inline]
pub unsafe fn NCryptSecretAgreement(hprivkey: NCRYPT_KEY_HANDLE, hpubkey: NCRYPT_KEY_HANDLE, phagreedsecret: *mut NCRYPT_SECRET_HANDLE, dwflags: u32) -> SECURITY_STATUS {
    windows_core::link!("ncrypt.dll" "system" fn NCryptSecretAgreement(hprivkey : NCRYPT_KEY_HANDLE, hpubkey : NCRYPT_KEY_HANDLE, phagreedsecret : *mut NCRYPT_SECRET_HANDLE, dwflags : u32) -> SECURITY_STATUS);
    unsafe { NCryptSecretAgreement(hprivkey, hpubkey, phagreedsecret as _, dwflags) }
}
#[inline]
pub unsafe fn NCryptSetProperty<P1>(hobject: NCRYPT_HANDLE, pszproperty: P1, pbinput: &[u8], dwflags: u32) -> SECURITY_STATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ncrypt.dll" "system" fn NCryptSetProperty(hobject : NCRYPT_HANDLE, pszproperty : windows_core::PCWSTR, pbinput : *const u8, cbinput : u32, dwflags : u32) -> SECURITY_STATUS);
    unsafe { NCryptSetProperty(hobject, pszproperty.param().abi(), core::mem::transmute(pbinput.as_ptr()), pbinput.len().try_into().unwrap(), dwflags) }
}
#[inline]
pub unsafe fn NCryptSignHash(hkey: NCRYPT_KEY_HANDLE, ppaddinginfo: Option<*const core::ffi::c_void>, pbhashvalue: &[u8], pbsignature: Option<&mut [u8]>, pcbresult: *mut u32, dwflags: u32) -> SECURITY_STATUS {
    windows_core::link!("ncrypt.dll" "system" fn NCryptSignHash(hkey : NCRYPT_KEY_HANDLE, ppaddinginfo : *const core::ffi::c_void, pbhashvalue : *const u8, cbhashvalue : u32, pbsignature : *mut u8, cbsignature : u32, pcbresult : *mut u32, dwflags : u32) -> SECURITY_STATUS);
    unsafe { NCryptSignHash(hkey, ppaddinginfo.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pbhashvalue.as_ptr()), pbhashvalue.len().try_into().unwrap(), core::mem::transmute(pbsignature.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pbsignature.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbresult as _, dwflags) }
}
#[cfg(feature = "wincrypt")]
#[inline]
pub unsafe fn NCryptTranslateHandle(phprovider: Option<*mut NCRYPT_PROV_HANDLE>, phkey: *mut NCRYPT_KEY_HANDLE, hlegacyprov: super::wincrypt::HCRYPTPROV, hlegacykey: Option<super::wincrypt::HCRYPTKEY>, dwlegacykeyspec: Option<u32>, dwflags: u32) -> SECURITY_STATUS {
    windows_core::link!("ncrypt.dll" "system" fn NCryptTranslateHandle(phprovider : *mut NCRYPT_PROV_HANDLE, phkey : *mut NCRYPT_KEY_HANDLE, hlegacyprov : super::wincrypt::HCRYPTPROV, hlegacykey : super::wincrypt::HCRYPTKEY, dwlegacykeyspec : u32, dwflags : u32) -> SECURITY_STATUS);
    unsafe { NCryptTranslateHandle(phprovider.unwrap_or(core::mem::zeroed()) as _, phkey as _, hlegacyprov, hlegacykey.unwrap_or(core::mem::zeroed()) as _, dwlegacykeyspec.unwrap_or(core::mem::zeroed()) as _, dwflags) }
}
#[cfg(feature = "bcrypt")]
#[inline]
pub unsafe fn NCryptVerifyClaim(hsubjectkey: NCRYPT_KEY_HANDLE, hauthoritykey: Option<NCRYPT_KEY_HANDLE>, dwclaimtype: u32, pparameterlist: Option<*const NCryptBufferDesc>, pbclaimblob: &[u8], poutput: *mut NCryptBufferDesc, dwflags: u32) -> SECURITY_STATUS {
    windows_core::link!("ncrypt.dll" "system" fn NCryptVerifyClaim(hsubjectkey : NCRYPT_KEY_HANDLE, hauthoritykey : NCRYPT_KEY_HANDLE, dwclaimtype : u32, pparameterlist : *const NCryptBufferDesc, pbclaimblob : *const u8, cbclaimblob : u32, poutput : *mut NCryptBufferDesc, dwflags : u32) -> SECURITY_STATUS);
    unsafe { NCryptVerifyClaim(hsubjectkey, hauthoritykey.unwrap_or(core::mem::zeroed()) as _, dwclaimtype, pparameterlist.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pbclaimblob.as_ptr()), pbclaimblob.len().try_into().unwrap(), poutput as _, dwflags) }
}
#[inline]
pub unsafe fn NCryptVerifySignature(hkey: NCRYPT_KEY_HANDLE, ppaddinginfo: Option<*const core::ffi::c_void>, pbhashvalue: &[u8], pbsignature: &[u8], dwflags: u32) -> SECURITY_STATUS {
    windows_core::link!("ncrypt.dll" "system" fn NCryptVerifySignature(hkey : NCRYPT_KEY_HANDLE, ppaddinginfo : *const core::ffi::c_void, pbhashvalue : *const u8, cbhashvalue : u32, pbsignature : *const u8, cbsignature : u32, dwflags : u32) -> SECURITY_STATUS);
    unsafe { NCryptVerifySignature(hkey, ppaddinginfo.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pbhashvalue.as_ptr()), pbhashvalue.len().try_into().unwrap(), core::mem::transmute(pbsignature.as_ptr()), pbsignature.len().try_into().unwrap(), dwflags) }
}
pub const IFX_RSA_KEYGEN_VUL_AFFECTED_LEVEL_1: u32 = 1;
pub const IFX_RSA_KEYGEN_VUL_AFFECTED_LEVEL_2: u32 = 2;
pub const IFX_RSA_KEYGEN_VUL_NOT_AFFECTED: u32 = 0;
pub const MS_KEY_STORAGE_PROVIDER: windows_core::PCWSTR = windows_core::w!("Microsoft Software Key Storage Provider");
pub const MS_NGC_KEY_STORAGE_PROVIDER: windows_core::PCWSTR = windows_core::w!("Microsoft Passport Key Storage Provider");
pub const MS_PLATFORM_KEY_STORAGE_PROVIDER: windows_core::PCWSTR = windows_core::w!("Microsoft Platform Crypto Provider");
pub const MS_PLUTON_CRYPTO_PROVIDER: windows_core::PCWSTR = windows_core::w!("Microsoft Pluton Cryptographic Provider");
pub const MS_SMART_CARD_KEY_STORAGE_PROVIDER: windows_core::PCWSTR = windows_core::w!("Microsoft Smart Card Key Storage Provider");
pub const NCRYPTBUFFER_ATTESTATIONSTATEMENT_BLOB: u32 = 51;
pub const NCRYPTBUFFER_ATTESTATION_CLAIM_CHALLENGE_REQUIRED: u32 = 53;
pub const NCRYPTBUFFER_ATTESTATION_CLAIM_TYPE: u32 = 52;
pub const NCRYPTBUFFER_ATTESTATION_STATEMENT_NONCE: u32 = 49;
pub const NCRYPTBUFFER_ATTESTATION_STATEMENT_SIGNATURE_HASH: u32 = 90;
pub const NCRYPTBUFFER_ATTESTATION_STATEMENT_SIGNATURE_PADDING_ALGO: u32 = 92;
pub const NCRYPTBUFFER_ATTESTATION_STATEMENT_SIGNATURE_PADDING_SALT: u32 = 93;
pub const NCRYPTBUFFER_ATTESTATION_STATEMENT_SIGNATURE_PADDING_SALT_SIZE: u32 = 93;
pub const NCRYPTBUFFER_ATTESTATION_STATEMENT_SIGNATURE_PADDING_SCHEME: u32 = 91;
pub const NCRYPTBUFFER_CERT_BLOB: u32 = 47;
pub const NCRYPTBUFFER_CLAIM_IDBINDING_NONCE: u32 = 48;
pub const NCRYPTBUFFER_CLAIM_KEYATTESTATION_NONCE: u32 = 49;
pub const NCRYPTBUFFER_DATA: u32 = 1;
pub const NCRYPTBUFFER_ECC_CURVE_NAME: u32 = 60;
pub const NCRYPTBUFFER_ECC_PARAMETERS: u32 = 61;
pub const NCRYPTBUFFER_EMPTY: u32 = 0;
pub const NCRYPTBUFFER_KEY_PROPERTY_FLAGS: u32 = 50;
pub const NCRYPTBUFFER_PKCS_AES_KEY_BITS: u32 = 96;
pub const NCRYPTBUFFER_PKCS_ALG_ID: u32 = 43;
pub const NCRYPTBUFFER_PKCS_ALG_OID: u32 = 41;
pub const NCRYPTBUFFER_PKCS_ALG_PARAM: u32 = 42;
pub const NCRYPTBUFFER_PKCS_ATTRS: u32 = 44;
pub const NCRYPTBUFFER_PKCS_KEY_NAME: u32 = 45;
pub const NCRYPTBUFFER_PKCS_OID: u32 = 40;
pub const NCRYPTBUFFER_PKCS_PADDING_ALGO: u32 = 97;
pub const NCRYPTBUFFER_PKCS_PADDING_LABEL: u32 = 98;
pub const NCRYPTBUFFER_PKCS_SECRET: u32 = 46;
pub const NCRYPTBUFFER_PROTECTION_DESCRIPTOR_STRING: u32 = 3;
pub const NCRYPTBUFFER_PROTECTION_FLAGS: u32 = 4;
pub const NCRYPTBUFFER_SSL_CLEAR_KEY: u32 = 23;
pub const NCRYPTBUFFER_SSL_CLIENT_RANDOM: u32 = 20;
pub const NCRYPTBUFFER_SSL_HIGHEST_VERSION: u32 = 22;
pub const NCRYPTBUFFER_SSL_KEY_ARG_DATA: u32 = 24;
pub const NCRYPTBUFFER_SSL_SERVER_RANDOM: u32 = 21;
pub const NCRYPTBUFFER_SSL_SESSION_HASH: u32 = 25;
pub const NCRYPTBUFFER_TPM_PLATFORM_CLAIM_NONCE: u32 = 81;
pub const NCRYPTBUFFER_TPM_PLATFORM_CLAIM_PCR_MASK: u32 = 80;
pub const NCRYPTBUFFER_TPM_PLATFORM_CLAIM_STATIC_CREATE: u32 = 82;
pub const NCRYPTBUFFER_TPM_SEAL_NO_DA_PROTECTION: u32 = 73;
pub const NCRYPTBUFFER_TPM_SEAL_PASSWORD: u32 = 70;
pub const NCRYPTBUFFER_TPM_SEAL_POLICYINFO: u32 = 71;
pub const NCRYPTBUFFER_TPM_SEAL_TICKET: u32 = 72;
pub const NCRYPTBUFFER_VBS_ATTESTATION_STATEMENT_IDENTITY_DETAILS: u32 = 95;
pub const NCRYPTBUFFER_VBS_ATTESTATION_STATEMENT_ROOT_DETAILS: u32 = 94;
pub const NCRYPTBUFFER_VERSION: u32 = 0;
pub const NCRYPT_ALGORITHM_GROUP_PROPERTY: windows_core::PCWSTR = windows_core::w!("Algorithm Group");
pub const NCRYPT_ALGORITHM_PROPERTY: windows_core::PCWSTR = windows_core::w!("Algorithm Name");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct NCRYPT_ALLOC_PARA {
    pub cbSize: u32,
    pub pfnAlloc: PFN_NCRYPT_ALLOC,
    pub pfnFree: PFN_NCRYPT_FREE,
}
pub const NCRYPT_ALLOW_ALL_USAGES: u32 = 16777215;
pub const NCRYPT_ALLOW_ARCHIVING_FLAG: u32 = 4;
pub const NCRYPT_ALLOW_DECRYPT_FLAG: u32 = 1;
pub const NCRYPT_ALLOW_EXPORT_FLAG: u32 = 1;
pub const NCRYPT_ALLOW_KEY_AGREEMENT_FLAG: u32 = 4;
pub const NCRYPT_ALLOW_KEY_ATTESTATION_FLAG: u32 = 16;
pub const NCRYPT_ALLOW_KEY_ENVELOPE_FLAG: u32 = 8;
pub const NCRYPT_ALLOW_KEY_IMPORT_EPHEMERAL_FLAG: u32 = 32;
pub const NCRYPT_ALLOW_KEY_IMPORT_FLAG: u32 = 8;
pub const NCRYPT_ALLOW_PKCS11_RSA_AES_EXPORT_FLAG: u32 = 16;
pub const NCRYPT_ALLOW_PLAINTEXT_ARCHIVING_FLAG: u32 = 8;
pub const NCRYPT_ALLOW_PLAINTEXT_EXPORT_FLAG: u32 = 2;
pub const NCRYPT_ALLOW_SIGNING_FLAG: u32 = 2;
pub const NCRYPT_ALLOW_SILENT_KEY_ACCESS: u32 = 1;
pub const NCRYPT_ASSOCIATED_ECDH_KEY: windows_core::PCWSTR = windows_core::w!("SmartCardAssociatedECDHKey");
pub const NCRYPT_ASYMMETRIC_ENCRYPTION_INTERFACE: u32 = 3;
pub const NCRYPT_ASYMMETRIC_ENCRYPTION_OPERATION: u32 = 4;
pub const NCRYPT_ATTESTATION_FLAG: u32 = 32;
pub const NCRYPT_AUTHORITY_KEY_FLAG: u32 = 256;
pub const NCRYPT_AUTH_TAG_LENGTH: windows_core::PCWSTR = windows_core::w!("AuthTagLength");
pub const NCRYPT_BLOCK_LENGTH_PROPERTY: windows_core::PCWSTR = windows_core::w!("Block Length");
pub const NCRYPT_CERTIFICATE_FROM_NVRAM_PROPERTY: windows_core::PCWSTR = windows_core::w!("KeyCertificateFromTpmNvram");
pub const NCRYPT_CERTIFICATE_PROPERTY: windows_core::PCWSTR = windows_core::w!("SmartCardKeyCertificate");
pub const NCRYPT_CHAINING_MODE_PROPERTY: windows_core::PCWSTR = windows_core::w!("Chaining Mode");
pub const NCRYPT_CIPHER_BLOCK_PADDING_FLAG: u32 = 1;
pub const NCRYPT_CIPHER_INTERFACE: u32 = 1;
pub const NCRYPT_CIPHER_KEY_BLOB: windows_core::PCWSTR = windows_core::w!("CipherKeyBlob");
pub const NCRYPT_CIPHER_KEY_BLOB_MAGIC: u32 = 1380470851;
pub const NCRYPT_CIPHER_NO_PADDING_FLAG: u32 = 0;
pub const NCRYPT_CIPHER_OPERATION: u32 = 1;
pub const NCRYPT_CIPHER_OTHER_PADDING_FLAG: u32 = 2;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NCRYPT_CIPHER_PADDING_INFO {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub pbIV: super::minwindef::PUCHAR,
    pub cbIV: u32,
    pub pbOtherInfo: super::minwindef::PUCHAR,
    pub cbOtherInfo: u32,
}
pub const NCRYPT_CLAIM_AUTHORITY_AND_SUBJECT: u32 = 3;
pub const NCRYPT_CLAIM_AUTHORITY_ONLY: u32 = 1;
pub const NCRYPT_CLAIM_PLATFORM: u32 = 65536;
pub const NCRYPT_CLAIM_SUBJECT_ONLY: u32 = 2;
pub const NCRYPT_CLAIM_UNKNOWN: u32 = 4096;
pub const NCRYPT_CLAIM_VBS_IDENTITY: u32 = 6;
pub const NCRYPT_CLAIM_VBS_KEY_ATTESTATION_STATEMENT: u32 = 4;
pub const NCRYPT_CLAIM_VBS_ROOT: u32 = 5;
pub const NCRYPT_CLAIM_WEB_AUTH_SUBJECT_ONLY: u32 = 258;
pub const NCRYPT_CLAIM_WEB_AUTH_SUBJECT_ONLY_V2: u32 = 259;
pub const NCRYPT_DES_ALGORITHM_GROUP: windows_core::PCWSTR = windows_core::w!("DES");
pub const NCRYPT_DISMISS_UI_TIMEOUT_SEC_PROPERTY: windows_core::PCWSTR = windows_core::w!("SmartCardDismissUITimeoutSeconds");
pub const NCRYPT_DO_NOT_FINALIZE_FLAG: u32 = 1024;
pub const NCRYPT_ECDH_ALGORITHM_GROUP: windows_core::PCWSTR = windows_core::w!("ECDH");
pub const NCRYPT_ECDSA_ALGORITHM_GROUP: windows_core::PCWSTR = windows_core::w!("ECDSA");
pub const NCRYPT_EPHEMERAL_NAME_PROPERTY: windows_core::PCWSTR = windows_core::w!("Ephemeral Name");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NCRYPT_EXPORTED_ISOLATED_KEY_ENVELOPE {
    pub Header: NCRYPT_EXPORTED_ISOLATED_KEY_HEADER,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NCRYPT_EXPORTED_ISOLATED_KEY_HEADER {
    pub Version: u32,
    pub KeyUsage: u32,
    pub _bitfield: u32,
    pub cbAlgName: u32,
    pub cbNonce: u32,
    pub cbAuthTag: u32,
    pub cbWrappingKey: u32,
    pub cbIsolatedKey: u32,
}
pub const NCRYPT_EXPORTED_ISOLATED_KEY_HEADER_CURRENT_VERSION: u32 = 0;
pub const NCRYPT_EXPORTED_ISOLATED_KEY_HEADER_V0: u32 = 0;
pub const NCRYPT_EXPORT_LEGACY_FLAG: u32 = 2048;
pub const NCRYPT_EXPORT_POLICY_PROPERTY: windows_core::PCWSTR = windows_core::w!("Export Policy");
pub const NCRYPT_EXTENDED_ERRORS_FLAG: u32 = 268435456;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct NCRYPT_HANDLE(pub usize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct NCRYPT_HASH_HANDLE(pub usize);
pub const NCRYPT_HASH_INTERFACE: u32 = 2;
pub const NCRYPT_HASH_OPERATION: u32 = 2;
pub const NCRYPT_HMAC_SHA256_ALGORITHM: windows_core::PCWSTR = windows_core::w!("HMAC-SHA256");
pub const NCRYPT_IGNORE_DEVICE_STATE_FLAG: u32 = 4096;
pub const NCRYPT_IMPL_HARDWARE_FLAG: u32 = 1;
pub const NCRYPT_IMPL_HARDWARE_RNG_FLAG: u32 = 16;
pub const NCRYPT_IMPL_REMOVABLE_FLAG: u32 = 8;
pub const NCRYPT_IMPL_SOFTWARE_FLAG: u32 = 2;
pub const NCRYPT_IMPL_TYPE_PROPERTY: windows_core::PCWSTR = windows_core::w!("Impl Type");
pub const NCRYPT_IMPL_VIRTUAL_ISOLATION_FLAG: u32 = 32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NCRYPT_ISOLATED_KEY_ATTESTED_ATTRIBUTES {
    pub Version: u32,
    pub Flags: u32,
    pub cbPublicKeyBlob: u32,
}
pub const NCRYPT_ISOLATED_KEY_ATTESTED_ATTRIBUTES_CURRENT_VERSION: u32 = 0;
pub const NCRYPT_ISOLATED_KEY_ATTESTED_ATTRIBUTES_V0: u32 = 0;
pub const NCRYPT_ISOLATED_KEY_ENVELOPE_BLOB: windows_core::PCWSTR = windows_core::w!("ISOLATED_KEY_ENVELOPE");
pub const NCRYPT_ISOLATED_KEY_FLAG_CREATED_IN_ISOLATION: u32 = 1;
pub const NCRYPT_ISOLATED_KEY_FLAG_IMPORT_ONLY: u32 = 2;
pub const NCRYPT_ISOLATED_KEY_FLAG_PER_BOOT_KEY: u32 = 4;
pub const NCRYPT_KDF_KEY_BLOB: windows_core::PCWSTR = windows_core::w!("KDFKeyBlob");
pub const NCRYPT_KDF_KEY_BLOB_MAGIC: u32 = 826688587;
pub const NCRYPT_KDF_SECRET_VALUE: windows_core::PCWSTR = windows_core::w!("KDFKeySecret");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NCRYPT_KEY_ACCESS_POLICY_BLOB {
    pub dwVersion: u32,
    pub dwPolicyFlags: u32,
    pub cbUserSid: u32,
    pub cbApplicationSid: u32,
}
pub const NCRYPT_KEY_ACCESS_POLICY_PROPERTY: windows_core::PCWSTR = windows_core::w!("Key Access Policy");
pub const NCRYPT_KEY_ACCESS_POLICY_VERSION: u32 = 1;
pub const NCRYPT_KEY_ATTEST_MAGIC: u32 = 1146110283;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NCRYPT_KEY_ATTEST_PADDING_INFO {
    pub magic: u32,
    pub pbKeyBlob: super::minwindef::PUCHAR,
    pub cbKeyBlob: u32,
    pub pbKeyAuth: super::minwindef::PUCHAR,
    pub cbKeyAuth: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NCRYPT_KEY_BLOB_HEADER {
    pub cbSize: u32,
    pub dwMagic: u32,
    pub cbAlgName: u32,
    pub cbKeyData: u32,
}
pub const NCRYPT_KEY_DERIVATION_GROUP: windows_core::PCWSTR = windows_core::w!("KEY_DERIVATION");
pub const NCRYPT_KEY_DERIVATION_INTERFACE: u32 = 7;
pub const NCRYPT_KEY_DERIVATION_OPERATION: u32 = 64;
pub const NCRYPT_KEY_ENCAPSULATION_INTERFACE: u32 = 8;
pub const NCRYPT_KEY_ENCAPSULATION_OPERATION: u32 = 128;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct NCRYPT_KEY_HANDLE(pub usize);
pub const NCRYPT_KEY_PROTECTION_INTERFACE: u32 = 65540;
pub const NCRYPT_KEY_STORAGE_ALGORITHM: windows_core::PCWSTR = windows_core::w!("KEY_STORAGE");
pub const NCRYPT_KEY_STORAGE_INTERFACE: u32 = 65537;
pub const NCRYPT_KEY_TYPE_PROPERTY: windows_core::PCWSTR = windows_core::w!("Key Type");
pub const NCRYPT_KEY_USAGE_PROPERTY: windows_core::PCWSTR = windows_core::w!("Key Usage");
pub const NCRYPT_LAST_MODIFIED_PROPERTY: windows_core::PCWSTR = windows_core::w!("Modified");
pub const NCRYPT_LENGTHS_PROPERTY: windows_core::PCWSTR = windows_core::w!("Lengths");
pub const NCRYPT_LENGTH_PROPERTY: windows_core::PCWSTR = windows_core::w!("Length");
pub const NCRYPT_MACHINE_KEY_FLAG: u32 = 32;
pub const NCRYPT_MAX_ALG_ID_LENGTH: u32 = 512;
pub const NCRYPT_MAX_KEY_NAME_LENGTH: u32 = 512;
pub const NCRYPT_MAX_NAME_LENGTH_PROPERTY: windows_core::PCWSTR = windows_core::w!("Max Name Length");
pub const NCRYPT_MAX_PROPERTY_DATA: u32 = 1048576;
pub const NCRYPT_MAX_PROPERTY_NAME: u32 = 64;
pub const NCRYPT_MLDSA_ALGORITHM_GROUP: windows_core::PCWSTR = windows_core::w!("MLDSA");
pub const NCRYPT_MLKEM_ALGORITHM_GROUP: windows_core::PCWSTR = windows_core::w!("MLKEM");
pub const NCRYPT_NAME_PROPERTY: windows_core::PCWSTR = windows_core::w!("Name");
pub const NCRYPT_NO_CACHED_PASSWORD: u32 = 16384;
pub const NCRYPT_NO_KEY_VALIDATION: u32 = 8;
pub const NCRYPT_NO_PADDING_FLAG: u32 = 1;
pub const NCRYPT_OPAQUETRANSPORT_BLOB: windows_core::PCWSTR = windows_core::w!("OpaqueTransport");
pub const NCRYPT_OVERWRITE_KEY_FLAG: u32 = 128;
pub const NCRYPT_PAD_CIPHER_FLAG: u32 = 16;
pub const NCRYPT_PAD_OAEP_FLAG: u32 = 4;
pub const NCRYPT_PAD_PKCS1_FLAG: u32 = 2;
pub const NCRYPT_PAD_PQDSA_FLAG: u32 = 32;
pub const NCRYPT_PAD_PSS_FLAG: u32 = 8;
pub const NCRYPT_PCP_AIKSTORE_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_AIKSTORE");
pub const NCRYPT_PCP_ALTERNATE_KEY_STORAGE_LOCATION_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_ALTERNATE_KEY_STORAGE_LOCATION");
pub const NCRYPT_PCP_CHANGEPASSWORD_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_CHANGEPASSWORD");
pub const NCRYPT_PCP_ECC_EKCERT_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_ECC_EKCERT");
pub const NCRYPT_PCP_ECC_EKNVCERT_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_ECC_EKNVCERT");
pub const NCRYPT_PCP_ECC_EKPUB_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_ECC_EKPUB");
pub const NCRYPT_PCP_EKCERT_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_EKCERT");
pub const NCRYPT_PCP_EKNVCERT_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_EKNVCERT");
pub const NCRYPT_PCP_EKPUB_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_EKPUB");
pub const NCRYPT_PCP_EKSTORE_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_EKSTORE");
pub const NCRYPT_PCP_ENCRYPTION_KEY: u32 = 2;
pub const NCRYPT_PCP_EXPORT_ALLOWED_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_EXPORT_ALLOWED");
pub const NCRYPT_PCP_GENERIC_KEY: u32 = 3;
pub const NCRYPT_PCP_HMACVERIFICATION_KEY: u32 = 16;
pub const NCRYPT_PCP_HMAC_AUTH_NONCE: windows_core::PCWSTR = windows_core::w!("PCP_HMAC_AUTH_NONCE");
pub const NCRYPT_PCP_HMAC_AUTH_POLICYINFO: windows_core::PCWSTR = windows_core::w!("PCP_HMAC_AUTH_POLICYINFO");
pub const NCRYPT_PCP_HMAC_AUTH_POLICYREF: windows_core::PCWSTR = windows_core::w!("PCP_HMAC_AUTH_POLICYREF");
pub const NCRYPT_PCP_HMAC_AUTH_SIGNATURE: windows_core::PCWSTR = windows_core::w!("PCP_HMAC_AUTH_SIGNATURE");
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NCRYPT_PCP_HMAC_AUTH_SIGNATURE_INFO {
    pub dwVersion: u32,
    pub iExpiration: i32,
    pub pabNonce: [u8; 32],
    pub pabPolicyRef: [u8; 32],
    pub pabHMAC: [u8; 32],
}
impl Default for NCRYPT_PCP_HMAC_AUTH_SIGNATURE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NCRYPT_PCP_HMAC_AUTH_TICKET: windows_core::PCWSTR = windows_core::w!("PCP_HMAC_AUTH_TICKET");
pub const NCRYPT_PCP_IDENTITY_KEY: u32 = 8;
pub const NCRYPT_PCP_INTERMEDIATE_CA_EKCERT_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_INTERMEDIATE_CA_EKCERT");
pub const NCRYPT_PCP_KEYATTESTATION_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_TPM12_KEYATTESTATION");
pub const NCRYPT_PCP_KEY_CREATIONHASH_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_KEY_CREATIONHASH");
pub const NCRYPT_PCP_KEY_CREATIONTICKET_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_KEY_CREATIONTICKET");
pub const NCRYPT_PCP_KEY_USAGE_POLICY_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_KEY_USAGE_POLICY");
pub const NCRYPT_PCP_MIGRATIONPASSWORD_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_MIGRATIONPASSWORD");
pub const NCRYPT_PCP_NO_DA_PROTECTION_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_NO_DA_PROTECTION");
pub const NCRYPT_PCP_PASSWORD_REQUIRED_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_PASSWORD_REQUIRED");
pub const NCRYPT_PCP_PCRTABLE_ALGORITHM_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_PCRTABLE_ALGORITHM");
pub const NCRYPT_PCP_PCRTABLE_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_PCRTABLE");
pub const NCRYPT_PCP_PLATFORMHANDLE_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_PLATFORMHANDLE");
pub const NCRYPT_PCP_PLATFORM_BINDING_PCRALGID_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_PLATFORM_BINDING_PCRALGID");
pub const NCRYPT_PCP_PLATFORM_BINDING_PCRDIGESTLIST_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_PLATFORM_BINDING_PCRDIGESTLIST");
pub const NCRYPT_PCP_PLATFORM_BINDING_PCRDIGEST_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_PLATFORM_BINDING_PCRDIGEST");
pub const NCRYPT_PCP_PLATFORM_BINDING_PCRMASK_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_PLATFORM_BINDING_PCRMASK");
pub const NCRYPT_PCP_PLATFORM_TYPE_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_PLATFORM_TYPE");
pub const NCRYPT_PCP_PROVIDERHANDLE_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_PROVIDERMHANDLE");
pub const NCRYPT_PCP_PROVIDER_VERSION_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_PROVIDER_VERSION");
pub const NCRYPT_PCP_PSS_SALT_SIZE_PROPERTY: windows_core::PCWSTR = windows_core::w!("PSS Salt Size");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NCRYPT_PCP_RAW_POLICYDIGEST_INFO {
    pub dwVersion: u32,
    pub cbDigest: u32,
}
pub const NCRYPT_PCP_RAW_POLICYDIGEST_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_RAW_POLICYDIGEST");
pub const NCRYPT_PCP_RSA_EKCERT_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_RSA_EKCERT");
pub const NCRYPT_PCP_RSA_EKNVCERT_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_RSA_EKNVCERT");
pub const NCRYPT_PCP_RSA_EKPUB_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_RSA_EKPUB");
pub const NCRYPT_PCP_RSA_SCHEME_HASH_ALG_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_RSA_SCHEME_HASH_ALG");
pub const NCRYPT_PCP_RSA_SCHEME_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_RSA_SCHEME");
pub const NCRYPT_PCP_SDDIDK_CONTEXT_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_SDDIDK_CONTEXT");
pub const NCRYPT_PCP_SDDIDK_KEY: u32 = 32;
pub const NCRYPT_PCP_SDDIDK_OPERATION: u32 = 268435456;
pub const NCRYPT_PCP_SESSIONID_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_SESSIONID");
pub const NCRYPT_PCP_SIGNATURE_KEY: u32 = 1;
pub const NCRYPT_PCP_SRKPUB_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_SRKPUB");
pub const NCRYPT_PCP_STORAGEPARENT_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_STORAGEPARENT");
pub const NCRYPT_PCP_STORAGE_KEY: u32 = 4;
pub const NCRYPT_PCP_SYMMETRIC_KEYBITS_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_SYMMETRIC_KEYBITS");
pub const NCRYPT_PCP_TPM12_IDACTIVATION_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_TPM12_IDACTIVATION");
pub const NCRYPT_PCP_TPM12_IDBINDING_DYNAMIC_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_TPM12_IDBINDING_DYNAMIC");
pub const NCRYPT_PCP_TPM12_IDBINDING_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_TPM12_IDBINDING");
pub const NCRYPT_PCP_TPM2BNAME_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_TPM2BNAME");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NCRYPT_PCP_TPM_FW_VERSION_INFO {
    pub major1: u16,
    pub major2: u16,
    pub minor1: u16,
    pub minor2: u16,
}
pub const NCRYPT_PCP_TPM_FW_VERSION_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_TPM_FW_VERSION");
pub const NCRYPT_PCP_TPM_IFX_RSA_KEYGEN_PROHIBITED_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_TPM_IFX_RSA_KEYGEN_PROHIBITED");
pub const NCRYPT_PCP_TPM_IFX_RSA_KEYGEN_VULNERABILITY_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_TPM_IFX_RSA_KEYGEN_VULNERABILITY");
pub const NCRYPT_PCP_TPM_MANUFACTURER_ID_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_TPM_MANUFACTURER_ID");
pub const NCRYPT_PCP_TPM_VERSION_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_TPM_VERSION");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NCRYPT_PCP_TPM_WEB_AUTHN_ATTESTATION_STATEMENT {
    pub Magic: u32,
    pub Version: u32,
    pub HeaderSize: u32,
    pub cbCertifyInfo: u32,
    pub cbSignature: u32,
    pub cbTpmPublic: u32,
}
pub const NCRYPT_PCP_USAGEAUTH_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_USAGEAUTH");
pub const NCRYPT_PERSIST_FLAG: u32 = 2147483648;
pub const NCRYPT_PERSIST_ONLY_FLAG: u32 = 1073741824;
pub const NCRYPT_PIN_CACHE_APPLICATION_IMAGE_PROPERTY: windows_core::PCWSTR = windows_core::w!("PinCacheApplicationImage");
pub const NCRYPT_PIN_CACHE_APPLICATION_STATUS_PROPERTY: windows_core::PCWSTR = windows_core::w!("PinCacheApplicationStatus");
pub const NCRYPT_PIN_CACHE_APPLICATION_TICKET_BYTE_LENGTH: u32 = 90;
pub const NCRYPT_PIN_CACHE_APPLICATION_TICKET_PROPERTY: windows_core::PCWSTR = windows_core::w!("PinCacheApplicationTicket");
pub const NCRYPT_PIN_CACHE_CLEAR_FOR_CALLING_PROCESS_OPTION: u32 = 1;
pub const NCRYPT_PIN_CACHE_CLEAR_PROPERTY: windows_core::PCWSTR = windows_core::w!("PinCacheClear");
pub const NCRYPT_PIN_CACHE_DISABLE_DPL_FLAG: u32 = 1;
pub const NCRYPT_PIN_CACHE_FLAGS_PROPERTY: windows_core::PCWSTR = windows_core::w!("PinCacheFlags");
pub const NCRYPT_PIN_CACHE_FREE_APPLICATION_TICKET_PROPERTY: windows_core::PCWSTR = windows_core::w!("PinCacheFreeApplicationTicket");
pub const NCRYPT_PIN_CACHE_IS_GESTURE_REQUIRED_PROPERTY: windows_core::PCWSTR = windows_core::w!("PinCacheIsGestureRequired");
pub const NCRYPT_PIN_CACHE_PIN_BYTE_LENGTH: u32 = 90;
pub const NCRYPT_PIN_CACHE_PIN_PROPERTY: windows_core::PCWSTR = windows_core::w!("PinCachePin");
pub const NCRYPT_PIN_CACHE_REQUIRE_GESTURE_FLAG: u32 = 1;
pub const NCRYPT_PIN_PROMPT_PROPERTY: windows_core::PCWSTR = windows_core::w!("SmartCardPinPrompt");
pub const NCRYPT_PIN_PROPERTY: windows_core::PCWSTR = windows_core::w!("SmartCardPin");
pub const NCRYPT_PKCS7_ENVELOPE_BLOB: windows_core::PCWSTR = windows_core::w!("PKCS7_ENVELOPE");
pub const NCRYPT_PKCS8_PRIVATE_KEY_BLOB: windows_core::PCWSTR = windows_core::w!("PKCS8_PRIVATEKEY");
pub const NCRYPT_PLATFORM_ATTEST_MAGIC: u32 = 1146110288;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NCRYPT_PLATFORM_ATTEST_PADDING_INFO {
    pub magic: u32,
    pub pcrMask: u32,
}
pub const NCRYPT_PLUTON_ECC_EKCERT_PROPERTY: windows_core::PCWSTR = windows_core::w!("PLUTON_ECC_EKCERT");
pub const NCRYPT_PLUTON_ECC_EKPUB_PROPERTY: windows_core::PCWSTR = windows_core::w!("PLUTON_ECC_EKPUB");
pub const NCRYPT_PLUTON_EKCERT_PROPERTY: windows_core::PCWSTR = windows_core::w!("PLUTON_EKCERT");
pub const NCRYPT_PLUTON_EKPUB_PROPERTY: windows_core::PCWSTR = windows_core::w!("PLUTON_EKPUB");
pub const NCRYPT_PLUTON_KDF_PARAMS_BUFFER_DESC_PROPERTY: windows_core::PCWSTR = windows_core::w!("PlutonKdfParamsBufferDesc");
pub const NCRYPT_PLUTON_KDF_PARENT_KEY_UNIQUE_NAME_PROPERTY: windows_core::PCWSTR = windows_core::w!("PlutonKdfParentKeyUniqueName");
pub const NCRYPT_PLUTON_RSA_EKCERT_PROPERTY: windows_core::PCWSTR = windows_core::w!("PLUTON_RSA_EKCERT");
pub const NCRYPT_PLUTON_RSA_EKPUB_PROPERTY: windows_core::PCWSTR = windows_core::w!("PLUTON_RSA_EKPUB");
pub const NCRYPT_PLUTON_SESSION_ID_PROPERTY: windows_core::PCWSTR = windows_core::w!("PLUTON_SESSION_ID");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NCRYPT_PQ_BLOB {
    pub dwMagic: u32,
    pub cbBCryptType: u32,
    pub cbBCryptBlob: u32,
}
pub const NCRYPT_PQ_PRIVATE_BLOB_MAGIC: u32 = 1380995408;
pub const NCRYPT_PQ_PRIVATE_KEY_BLOB: windows_core::PCWSTR = windows_core::w!("PQPrivateKeyBlob");
pub const NCRYPT_PREFER_VBS_FLAG: u32 = 65536;
pub const NCRYPT_PREFER_VIRTUAL_ISOLATION_FLAG: u32 = 65536;
pub const NCRYPT_PROTECTED_KEY_BLOB: windows_core::PCWSTR = windows_core::w!("ProtectedKeyBlob");
pub const NCRYPT_PROTECTED_KEY_BLOB_MAGIC: u32 = 1263817296;
pub const NCRYPT_PROTECT_TO_LOCAL_SYSTEM: u32 = 32768;
pub const NCRYPT_PROVIDER_HANDLE_PROPERTY: windows_core::PCWSTR = windows_core::w!("Provider Handle");
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct NCRYPT_PROV_HANDLE(pub usize);
pub const NCRYPT_READER_ICON_PROPERTY: windows_core::PCWSTR = windows_core::w!("SmartCardReaderIcon");
pub const NCRYPT_READER_PROPERTY: windows_core::PCWSTR = windows_core::w!("SmartCardReader");
pub const NCRYPT_REGISTER_NOTIFY_FLAG: u32 = 1;
pub const NCRYPT_REQUIRE_KDS_LRPC_BIND_FLAG: u32 = 536870912;
pub const NCRYPT_REQUIRE_VBS_FLAG: u32 = 131072;
pub const NCRYPT_RNG_OPERATION: u32 = 32;
pub const NCRYPT_ROOT_CERTSTORE_PROPERTY: windows_core::PCWSTR = windows_core::w!("SmartcardRootCertStore");
pub const NCRYPT_SCARD_NGC_KEY_NAME: windows_core::PCWSTR = windows_core::w!("SmartCardNgcKeyName");
pub const NCRYPT_SCARD_PIN_ID: windows_core::PCWSTR = windows_core::w!("SmartCardPinId");
pub const NCRYPT_SCARD_PIN_INFO: windows_core::PCWSTR = windows_core::w!("SmartCardPinInfo");
pub const NCRYPT_SCHANNEL_INTERFACE: u32 = 65538;
pub const NCRYPT_SCHANNEL_SIGNATURE_INTERFACE: u32 = 65539;
pub const NCRYPT_SEALING_FLAG: u32 = 256;
pub const NCRYPT_SECRET_AGREEMENT_INTERFACE: u32 = 4;
pub const NCRYPT_SECRET_AGREEMENT_OPERATION: u32 = 8;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct NCRYPT_SECRET_HANDLE(pub usize);
pub const NCRYPT_SECURE_PIN_PROPERTY: windows_core::PCWSTR = windows_core::w!("SmartCardSecurePin");
pub const NCRYPT_SECURITY_DESCR_PROPERTY: windows_core::PCWSTR = windows_core::w!("Security Descr");
pub const NCRYPT_SECURITY_DESCR_SUPPORT_PROPERTY: windows_core::PCWSTR = windows_core::w!("Security Descr Support");
pub const NCRYPT_SIGNATURE_INTERFACE: u32 = 5;
pub const NCRYPT_SIGNATURE_OPERATION: u32 = 16;
pub const NCRYPT_SILENT_FLAG: u32 = 64;
pub const NCRYPT_SLHDSA_ALGORITHM_GROUP: windows_core::PCWSTR = windows_core::w!("SLHDSA");
pub const NCRYPT_SMARTCARD_GUID_PROPERTY: windows_core::PCWSTR = windows_core::w!("SmartCardGuid");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NCRYPT_SUPPORTED_LENGTHS {
    pub dwMinLength: u32,
    pub dwMaxLength: u32,
    pub dwIncrement: u32,
    pub dwDefaultLength: u32,
}
pub const NCRYPT_TPM12_PROVIDER: u32 = 65536;
pub const NCRYPT_TPM_LOADABLE_KEY_BLOB: windows_core::PCWSTR = windows_core::w!("PcpTpmProtectedKeyBlob");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NCRYPT_TPM_LOADABLE_KEY_BLOB_HEADER {
    pub magic: u32,
    pub cbHeader: u32,
    pub cbPublic: u32,
    pub cbPrivate: u32,
    pub cbName: u32,
}
pub const NCRYPT_TPM_LOADABLE_KEY_BLOB_MAGIC: u32 = 1297371211;
pub const NCRYPT_TPM_PAD_PSS_IGNORE_SALT: u32 = 32;
pub const NCRYPT_TPM_PERSISTENT_KEY_BLOB: windows_core::PCWSTR = windows_core::w!("PcpTpmPersistentKeyBlob");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NCRYPT_TPM_PERSISTENT_KEY_BLOB_HEADER {
    pub magic: u32,
    pub cbHeader: u32,
    pub tpmHandle: u32,
}
pub const NCRYPT_TPM_PERSISTENT_KEY_BLOB_MAGIC: u32 = 1297371211;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NCRYPT_TPM_PLATFORM_ATTESTATION_STATEMENT {
    pub Magic: u32,
    pub Version: u32,
    pub pcrAlg: u32,
    pub cbSignature: u32,
    pub cbQuote: u32,
    pub cbPcrs: u32,
}
pub const NCRYPT_TPM_PLATFORM_ATTESTATION_STATEMENT_CURRENT_VERSION: u32 = 0;
pub const NCRYPT_TPM_PLATFORM_ATTESTATION_STATEMENT_V0: u32 = 0;
pub const NCRYPT_TPM_PSS_SALT_SIZE_HASHSIZE: u32 = 2;
pub const NCRYPT_TPM_PSS_SALT_SIZE_MAXIMUM: u32 = 1;
pub const NCRYPT_TPM_PSS_SALT_SIZE_UNKNOWN: u32 = 0;
pub const NCRYPT_TREAT_NIST_AS_GENERIC_ECC_FLAG: u32 = 8192;
pub const NCRYPT_UI_APPCONTAINER_ACCESS_MEDIUM_FLAG: u32 = 8;
pub const NCRYPT_UI_FINGERPRINT_PROTECTION_FLAG: u32 = 4;
pub const NCRYPT_UI_FORCE_HIGH_PROTECTION_FLAG: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NCRYPT_UI_POLICY {
    pub dwVersion: u32,
    pub dwFlags: u32,
    pub pszCreationTitle: windows_core::PCWSTR,
    pub pszFriendlyName: windows_core::PCWSTR,
    pub pszDescription: windows_core::PCWSTR,
}
pub const NCRYPT_UI_POLICY_PROPERTY: windows_core::PCWSTR = windows_core::w!("UI Policy");
pub const NCRYPT_UI_PROTECT_KEY_FLAG: u32 = 1;
pub const NCRYPT_UNIQUE_NAME_PROPERTY: windows_core::PCWSTR = windows_core::w!("Unique Name");
pub const NCRYPT_UNREGISTER_NOTIFY_FLAG: u32 = 2;
pub const NCRYPT_USER_CERTSTORE_PROPERTY: windows_core::PCWSTR = windows_core::w!("SmartCardUserCertStore");
pub const NCRYPT_USE_CONTEXT_PROPERTY: windows_core::PCWSTR = windows_core::w!("Use Context");
pub const NCRYPT_USE_COUNT_ENABLED_PROPERTY: windows_core::PCWSTR = windows_core::w!("Enabled Use Count");
pub const NCRYPT_USE_COUNT_PROPERTY: windows_core::PCWSTR = windows_core::w!("Use Count");
pub const NCRYPT_USE_PER_BOOT_KEY_FLAG: u32 = 262144;
pub const NCRYPT_USE_PER_BOOT_KEY_PROPERTY: windows_core::PCWSTR = windows_core::w!("Per Boot Key");
pub const NCRYPT_USE_RECOVERY_LDAP_PORT: u32 = 65536;
pub const NCRYPT_USE_VBS_PER_BOOT_KEY_FLAG: u32 = 262144;
pub const NCRYPT_USE_VIRTUAL_ISOLATION_FLAG: u32 = 131072;
pub const NCRYPT_USE_VIRTUAL_ISOLATION_PROPERTY: windows_core::PCWSTR = windows_core::w!("Virtual Iso");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NCRYPT_VBS_IDENTITY_ATTESTATION_HEADER {
    pub Magic: u32,
    pub Version: u32,
    pub cbAttributes: u32,
    pub cbNonce: u32,
    pub cbHashAlg: u32,
    pub cbPadding: u32,
    pub cbSignatureAlg: u32,
    pub cbSignature: u32,
}
pub const NCRYPT_VBS_IDENTITY_ATTESTATION_HEADER_CURRENT_VERSION: u32 = 0;
pub const NCRYPT_VBS_IDENTITY_ATTESTATION_HEADER_V0: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NCRYPT_VBS_IDENTITY_ATTESTATION_PADDING {
    pub Version: u32,
    pub ulPaddingScheme: u32,
    pub cbHashAlg: u32,
    pub ulSalt: u32,
}
pub const NCRYPT_VBS_IDENTITY_ATTESTATION_PADDING_CURRENT_VERSION: u32 = 0;
pub const NCRYPT_VBS_IDENTITY_ATTESTATION_PADDING_V0: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NCRYPT_VBS_IDENTITY_KEY_ATTESTATION_CLAIM_DETAILS {
    pub ulKeyFlags: u32,
    pub pszSignatureHashAlg: windows_core::PCWSTR,
    pub ulPaddingScheme: u32,
    pub pszPaddingHashAlg: windows_core::PCWSTR,
    pub ulPaddingSalt: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NCRYPT_VBS_KEY_ATTESTATION_STATEMENT {
    pub Magic: u32,
    pub Version: u32,
    pub ClaimType: u32,
}
pub const NCRYPT_VBS_KEY_ATTESTATION_STATEMENT_CURRENT_VERSION: u32 = 2;
pub const NCRYPT_VBS_KEY_ATTESTATION_STATEMENT_V1: u32 = 1;
pub const NCRYPT_VBS_KEY_ATTESTATION_STATEMENT_V2: u32 = 2;
pub const NCRYPT_VBS_KEY_ATTESTED_ATTRIBUTES_CURRENT_VERSION: u32 = 0;
pub const NCRYPT_VBS_KEY_FLAG_CREATED_IN_ISOLATION: u32 = 1;
pub const NCRYPT_VBS_KEY_FLAG_IMPORT_EPHEMERAL_ONLY: u32 = 8;
pub const NCRYPT_VBS_KEY_FLAG_IMPORT_ONLY: u32 = 2;
pub const NCRYPT_VBS_KEY_FLAG_PER_BOOT_KEY: u32 = 4;
pub const NCRYPT_VBS_RETURN_CLAIM_DETAILS_FLAG: u32 = 1048576;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NCRYPT_VBS_ROOT_ATTESTATION_HEADER {
    pub Magic: u32,
    pub Version: u32,
    pub cbAttributes: u32,
    pub cbNonce: u32,
    pub cbReport: u32,
    pub cbSignature: u32,
}
pub const NCRYPT_VBS_ROOT_ATTESTATION_HEADER_CURRENT_VERSION: u32 = 0;
pub const NCRYPT_VBS_ROOT_ATTESTATION_HEADER_V0: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NCRYPT_VBS_ROOT_KEY_ATTESTATION_CLAIM_DETAILS {
    pub ulKeyFlags: u32,
    pub ullTrustletId: u64,
    pub ulTrustletSecurityVersion: u32,
    pub ulTrustletDebuggable: u32,
}
pub const NCRYPT_VBS_ROOT_PUB_PROPERTY: windows_core::PCWSTR = windows_core::w!("VBS_ROOT_PUB");
pub const NCRYPT_VERSION_PROPERTY: windows_core::PCWSTR = windows_core::w!("Version");
pub const NCRYPT_WINDOW_HANDLE_PROPERTY: windows_core::PCWSTR = windows_core::w!("HWND Handle");
pub const NCRYPT_WRITE_KEY_TO_LEGACY_STORE_FLAG: u32 = 512;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NCryptAlgorithmName {
    pub pszName: windows_core::PWSTR,
    pub dwClass: u32,
    pub dwAlgOperations: u32,
    pub dwFlags: u32,
}
#[cfg(feature = "bcrypt")]
pub type NCryptBuffer = super::bcrypt::BCryptBuffer;
#[cfg(feature = "bcrypt")]
pub type NCryptBufferDesc = super::bcrypt::BCryptBufferDesc;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NCryptKeyName {
    pub pszName: windows_core::PWSTR,
    pub pszAlgid: windows_core::PWSTR,
    pub dwLegacyKeySpec: u32,
    pub dwFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NCryptProviderName {
    pub pszName: windows_core::PWSTR,
    pub pszComment: windows_core::PWSTR,
}
pub type PFN_NCRYPT_ALLOC = Option<unsafe extern "system" fn(cbsize: usize) -> *mut core::ffi::c_void>;
pub type PFN_NCRYPT_FREE = Option<unsafe extern "system" fn(pv: *const core::ffi::c_void)>;
#[cfg(feature = "minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PNCRYPT_CIPHER_PADDING_INFO(pub *mut NCRYPT_CIPHER_PADDING_INFO);
#[cfg(feature = "minwindef")]
impl PNCRYPT_CIPHER_PADDING_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "minwindef")]
impl Default for PNCRYPT_CIPHER_PADDING_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PNCRYPT_EXPORTED_ISOLATED_KEY_ENVELOPE(pub *mut NCRYPT_EXPORTED_ISOLATED_KEY_ENVELOPE);
impl PNCRYPT_EXPORTED_ISOLATED_KEY_ENVELOPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PNCRYPT_EXPORTED_ISOLATED_KEY_ENVELOPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PNCRYPT_EXPORTED_ISOLATED_KEY_HEADER(pub *mut NCRYPT_EXPORTED_ISOLATED_KEY_HEADER);
impl PNCRYPT_EXPORTED_ISOLATED_KEY_HEADER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PNCRYPT_EXPORTED_ISOLATED_KEY_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PNCRYPT_ISOLATED_KEY_ATTESTED_ATTRIBUTES(pub *mut NCRYPT_ISOLATED_KEY_ATTESTED_ATTRIBUTES);
impl PNCRYPT_ISOLATED_KEY_ATTESTED_ATTRIBUTES {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PNCRYPT_ISOLATED_KEY_ATTESTED_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PNCRYPT_KEY_BLOB_HEADER(pub *mut NCRYPT_KEY_BLOB_HEADER);
impl PNCRYPT_KEY_BLOB_HEADER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PNCRYPT_KEY_BLOB_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PNCRYPT_PCP_TPM_WEB_AUTHN_ATTESTATION_STATEMENT(pub *mut NCRYPT_PCP_TPM_WEB_AUTHN_ATTESTATION_STATEMENT);
impl PNCRYPT_PCP_TPM_WEB_AUTHN_ATTESTATION_STATEMENT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PNCRYPT_PCP_TPM_WEB_AUTHN_ATTESTATION_STATEMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PNCRYPT_PQ_BLOB(pub *mut NCRYPT_PQ_BLOB);
impl PNCRYPT_PQ_BLOB {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PNCRYPT_PQ_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PNCRYPT_TPM_LOADABLE_KEY_BLOB_HEADER(pub *mut NCRYPT_TPM_LOADABLE_KEY_BLOB_HEADER);
impl PNCRYPT_TPM_LOADABLE_KEY_BLOB_HEADER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PNCRYPT_TPM_LOADABLE_KEY_BLOB_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PNCRYPT_TPM_PERSISTENT_KEY_BLOB_HEADER(pub *mut NCRYPT_TPM_PERSISTENT_KEY_BLOB_HEADER);
impl PNCRYPT_TPM_PERSISTENT_KEY_BLOB_HEADER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PNCRYPT_TPM_PERSISTENT_KEY_BLOB_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PNCRYPT_TPM_PLATFORM_ATTESTATION_STATEMENT(pub *mut NCRYPT_TPM_PLATFORM_ATTESTATION_STATEMENT);
impl PNCRYPT_TPM_PLATFORM_ATTESTATION_STATEMENT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PNCRYPT_TPM_PLATFORM_ATTESTATION_STATEMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PNCRYPT_VBS_IDENTITY_ATTESTATION_HEADER(pub *mut NCRYPT_VBS_IDENTITY_ATTESTATION_HEADER);
impl PNCRYPT_VBS_IDENTITY_ATTESTATION_HEADER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PNCRYPT_VBS_IDENTITY_ATTESTATION_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PNCRYPT_VBS_IDENTITY_ATTESTATION_PADDING(pub *mut NCRYPT_VBS_IDENTITY_ATTESTATION_PADDING);
impl PNCRYPT_VBS_IDENTITY_ATTESTATION_PADDING {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PNCRYPT_VBS_IDENTITY_ATTESTATION_PADDING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PNCRYPT_VBS_IDENTITY_KEY_ATTESTATION_CLAIM_DETAILS(pub *mut NCRYPT_VBS_IDENTITY_KEY_ATTESTATION_CLAIM_DETAILS);
impl PNCRYPT_VBS_IDENTITY_KEY_ATTESTATION_CLAIM_DETAILS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PNCRYPT_VBS_IDENTITY_KEY_ATTESTATION_CLAIM_DETAILS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PNCRYPT_VBS_KEY_ATTESTATION_STATEMENT(pub *mut NCRYPT_VBS_KEY_ATTESTATION_STATEMENT);
impl PNCRYPT_VBS_KEY_ATTESTATION_STATEMENT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PNCRYPT_VBS_KEY_ATTESTATION_STATEMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PNCRYPT_VBS_ROOT_ATTESTATION_HEADER(pub *mut NCRYPT_VBS_ROOT_ATTESTATION_HEADER);
impl PNCRYPT_VBS_ROOT_ATTESTATION_HEADER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PNCRYPT_VBS_ROOT_ATTESTATION_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PNCRYPT_VBS_ROOT_KEY_ATTESTATION_CLAIM_DETAILS(pub *mut NCRYPT_VBS_ROOT_KEY_ATTESTATION_CLAIM_DETAILS);
impl PNCRYPT_VBS_ROOT_KEY_ATTESTATION_CLAIM_DETAILS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PNCRYPT_VBS_ROOT_KEY_ATTESTATION_CLAIM_DETAILS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "bcrypt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PNCryptBuffer(pub *mut super::bcrypt::BCryptBuffer);
#[cfg(feature = "bcrypt")]
impl PNCryptBuffer {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "bcrypt")]
impl Default for PNCryptBuffer {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "bcrypt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PNCryptBufferDesc(pub *mut super::bcrypt::BCryptBufferDesc);
#[cfg(feature = "bcrypt")]
impl PNCryptBufferDesc {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "bcrypt")]
impl Default for PNCryptBufferDesc {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct SECURITY_STATUS(pub i32);
pub const TPM_RSA_SRK_SEAL_KEY: windows_core::PCWSTR = windows_core::w!("MICROSOFT_PCP_KSP_RSA_SEAL_KEY_3BD1C4BF-004E-4E2F-8A4D-0BF633DCB074");
pub const VBS_IDENTITY_ATTESTATION_HEADER_MAGIC: u32 = 1212369238;
pub const VBS_KEY_ATTESTATION_STATEMENT_MAGIC: u32 = 1396788054;
pub const VBS_ROOT_ATTESTATION_HEADER_MAGIC: u32 = 1212371542;
