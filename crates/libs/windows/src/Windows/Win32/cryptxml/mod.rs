#[cfg(all(feature = "shared", feature = "wincrypt"))]
#[inline]
pub unsafe fn CryptXmlAddObject(hsignatureorobject: HCRYPTXML, dwflags: super::DWORD, rgproperty: Option<&[CRYPT_XML_PROPERTY]>, pencoded: *const CRYPT_XML_BLOB, ppobject: *mut *const CRYPT_XML_OBJECT) -> windows_core::HRESULT {
    windows_core::link!("cryptxml.dll" "system" fn CryptXmlAddObject(hsignatureorobject : HCRYPTXML, dwflags : super::DWORD, rgproperty : *const CRYPT_XML_PROPERTY, cproperty : super::ULONG, pencoded : *const CRYPT_XML_BLOB, ppobject : *mut *const CRYPT_XML_OBJECT) -> windows_core::HRESULT);
    unsafe { CryptXmlAddObject(hsignatureorobject, dwflags, rgproperty.map_or(core::ptr::null(), |slice| slice.as_ptr()), rgproperty.map_or(0, |slice| slice.len().try_into().unwrap()), pencoded, ppobject as _) }
}
#[inline]
pub unsafe fn CryptXmlClose(hcryptxml: HCRYPTXML) -> windows_core::HRESULT {
    windows_core::link!("cryptxml.dll" "system" fn CryptXmlClose(hcryptxml : HCRYPTXML) -> windows_core::HRESULT);
    unsafe { CryptXmlClose(hcryptxml) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CryptXmlCreateReference<P2, P3, P4>(hcryptxml: HCRYPTXML, dwflags: super::DWORD, wszid: P2, wszuri: P3, wsztype: P4, pdigestmethod: *const CRYPT_XML_ALGORITHM, rgtransform: Option<&[CRYPT_XML_ALGORITHM]>, phreference: *mut HCRYPTXML) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cryptxml.dll" "system" fn CryptXmlCreateReference(hcryptxml : HCRYPTXML, dwflags : super::DWORD, wszid : windows_core::PCWSTR, wszuri : windows_core::PCWSTR, wsztype : windows_core::PCWSTR, pdigestmethod : *const CRYPT_XML_ALGORITHM, ctransform : super::ULONG, rgtransform : *const CRYPT_XML_ALGORITHM, phreference : *mut HCRYPTXML) -> windows_core::HRESULT);
    unsafe { CryptXmlCreateReference(hcryptxml, dwflags, wszid.param().abi(), wszuri.param().abi(), wsztype.param().abi(), pdigestmethod, rgtransform.map_or(0, |slice| slice.len().try_into().unwrap()), rgtransform.map_or(core::ptr::null(), |slice| slice.as_ptr()), phreference as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CryptXmlDigestReference(hreference: HCRYPTXML, dwflags: super::DWORD, pdataproviderin: *const CRYPT_XML_DATA_PROVIDER) -> windows_core::HRESULT {
    windows_core::link!("cryptxml.dll" "system" fn CryptXmlDigestReference(hreference : HCRYPTXML, dwflags : super::DWORD, pdataproviderin : *const CRYPT_XML_DATA_PROVIDER) -> windows_core::HRESULT);
    unsafe { CryptXmlDigestReference(hreference, dwflags, pdataproviderin) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CryptXmlEncode(hcryptxml: HCRYPTXML, dwcharset: CRYPT_XML_CHARSET, rgproperty: Option<&[CRYPT_XML_PROPERTY]>, pvcallbackstate: *mut core::ffi::c_void, pfnwrite: PFN_CRYPT_XML_WRITE_CALLBACK) -> windows_core::HRESULT {
    windows_core::link!("cryptxml.dll" "system" fn CryptXmlEncode(hcryptxml : HCRYPTXML, dwcharset : CRYPT_XML_CHARSET, rgproperty : *const CRYPT_XML_PROPERTY, cproperty : super::ULONG, pvcallbackstate : *mut core::ffi::c_void, pfnwrite : PFN_CRYPT_XML_WRITE_CALLBACK) -> windows_core::HRESULT);
    unsafe { CryptXmlEncode(hcryptxml, dwcharset, rgproperty.map_or(core::ptr::null(), |slice| slice.as_ptr()), rgproperty.map_or(0, |slice| slice.len().try_into().unwrap()), pvcallbackstate as _, pfnwrite) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CryptXmlEnumAlgorithmInfo(dwgroupid: super::DWORD, dwflags: super::DWORD, pvarg: Option<*mut core::ffi::c_void>, pfnenumalginfo: PFN_CRYPT_XML_ENUM_ALG_INFO) -> windows_core::HRESULT {
    windows_core::link!("cryptxml.dll" "system" fn CryptXmlEnumAlgorithmInfo(dwgroupid : super::DWORD, dwflags : super::DWORD, pvarg : *mut core::ffi::c_void, pfnenumalginfo : PFN_CRYPT_XML_ENUM_ALG_INFO) -> windows_core::HRESULT);
    unsafe { CryptXmlEnumAlgorithmInfo(dwgroupid, dwflags, pvarg.unwrap_or(core::mem::zeroed()) as _, pfnenumalginfo) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CryptXmlFindAlgorithmInfo(dwfindbytype: super::DWORD, pvfindby: *const core::ffi::c_void, dwgroupid: super::DWORD, dwflags: super::DWORD) -> *const CRYPT_XML_ALGORITHM_INFO {
    windows_core::link!("cryptxml.dll" "system" fn CryptXmlFindAlgorithmInfo(dwfindbytype : super::DWORD, pvfindby : *const core::ffi::c_void, dwgroupid : super::DWORD, dwflags : super::DWORD) -> *const CRYPT_XML_ALGORITHM_INFO);
    unsafe { CryptXmlFindAlgorithmInfo(dwfindbytype, pvfindby, dwgroupid, dwflags) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CryptXmlGetAlgorithmInfo(pxmlalgorithm: *const CRYPT_XML_ALGORITHM, dwflags: super::DWORD, ppalginfo: *mut *mut CRYPT_XML_ALGORITHM_INFO) -> windows_core::HRESULT {
    windows_core::link!("cryptxml.dll" "system" fn CryptXmlGetAlgorithmInfo(pxmlalgorithm : *const CRYPT_XML_ALGORITHM, dwflags : super::DWORD, ppalginfo : *mut *mut CRYPT_XML_ALGORITHM_INFO) -> windows_core::HRESULT);
    unsafe { CryptXmlGetAlgorithmInfo(pxmlalgorithm, dwflags, ppalginfo as _) }
}
#[cfg(all(feature = "bcrypt", feature = "shared", feature = "wincrypt"))]
#[inline]
pub unsafe fn CryptXmlGetDocContext(hcryptxml: HCRYPTXML, ppstruct: *mut *const CRYPT_XML_DOC_CTXT) -> windows_core::HRESULT {
    windows_core::link!("cryptxml.dll" "system" fn CryptXmlGetDocContext(hcryptxml : HCRYPTXML, ppstruct : *mut *const CRYPT_XML_DOC_CTXT) -> windows_core::HRESULT);
    unsafe { CryptXmlGetDocContext(hcryptxml, ppstruct as _) }
}
#[cfg(all(feature = "shared", feature = "wincrypt"))]
#[inline]
pub unsafe fn CryptXmlGetReference(hcryptxml: HCRYPTXML, ppstruct: *mut *const CRYPT_XML_REFERENCE) -> windows_core::HRESULT {
    windows_core::link!("cryptxml.dll" "system" fn CryptXmlGetReference(hcryptxml : HCRYPTXML, ppstruct : *mut *const CRYPT_XML_REFERENCE) -> windows_core::HRESULT);
    unsafe { CryptXmlGetReference(hcryptxml, ppstruct as _) }
}
#[cfg(all(feature = "bcrypt", feature = "shared", feature = "wincrypt"))]
#[inline]
pub unsafe fn CryptXmlGetSignature(hcryptxml: HCRYPTXML, ppstruct: *mut *const CRYPT_XML_SIGNATURE) -> windows_core::HRESULT {
    windows_core::link!("cryptxml.dll" "system" fn CryptXmlGetSignature(hcryptxml : HCRYPTXML, ppstruct : *mut *const CRYPT_XML_SIGNATURE) -> windows_core::HRESULT);
    unsafe { CryptXmlGetSignature(hcryptxml, ppstruct as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CryptXmlGetStatus(hcryptxml: HCRYPTXML, pstatus: *mut CRYPT_XML_STATUS) -> windows_core::HRESULT {
    windows_core::link!("cryptxml.dll" "system" fn CryptXmlGetStatus(hcryptxml : HCRYPTXML, pstatus : *mut CRYPT_XML_STATUS) -> windows_core::HRESULT);
    unsafe { CryptXmlGetStatus(hcryptxml, pstatus as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CryptXmlGetTransforms(ppconfig: *mut *const CRYPT_XML_TRANSFORM_CHAIN_CONFIG) -> windows_core::HRESULT {
    windows_core::link!("cryptxml.dll" "system" fn CryptXmlGetTransforms(ppconfig : *mut *const CRYPT_XML_TRANSFORM_CHAIN_CONFIG) -> windows_core::HRESULT);
    unsafe { CryptXmlGetTransforms(ppconfig as _) }
}
#[cfg(all(feature = "bcrypt", feature = "shared"))]
#[inline]
pub unsafe fn CryptXmlImportPublicKey(dwflags: super::DWORD, pkeyvalue: *const CRYPT_XML_KEY_VALUE, phkey: *mut super::BCRYPT_KEY_HANDLE) -> windows_core::HRESULT {
    windows_core::link!("cryptxml.dll" "system" fn CryptXmlImportPublicKey(dwflags : super::DWORD, pkeyvalue : *const CRYPT_XML_KEY_VALUE, phkey : *mut super::BCRYPT_KEY_HANDLE) -> windows_core::HRESULT);
    unsafe { CryptXmlImportPublicKey(dwflags, pkeyvalue, phkey as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CryptXmlOpenToDecode(pconfig: Option<*const CRYPT_XML_TRANSFORM_CHAIN_CONFIG>, dwflags: super::DWORD, rgproperty: Option<&[CRYPT_XML_PROPERTY]>, pencoded: *const CRYPT_XML_BLOB, phcryptxml: *mut HCRYPTXML) -> windows_core::HRESULT {
    windows_core::link!("cryptxml.dll" "system" fn CryptXmlOpenToDecode(pconfig : *const CRYPT_XML_TRANSFORM_CHAIN_CONFIG, dwflags : super::DWORD, rgproperty : *const CRYPT_XML_PROPERTY, cproperty : super::ULONG, pencoded : *const CRYPT_XML_BLOB, phcryptxml : *mut HCRYPTXML) -> windows_core::HRESULT);
    unsafe { CryptXmlOpenToDecode(pconfig.unwrap_or(core::mem::zeroed()) as _, dwflags, rgproperty.map_or(core::ptr::null(), |slice| slice.as_ptr()), rgproperty.map_or(0, |slice| slice.len().try_into().unwrap()), pencoded, phcryptxml as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CryptXmlOpenToEncode<P2>(pconfig: Option<*const CRYPT_XML_TRANSFORM_CHAIN_CONFIG>, dwflags: super::DWORD, wszid: P2, rgproperty: Option<&[CRYPT_XML_PROPERTY]>, pencoded: Option<*const CRYPT_XML_BLOB>, phsignature: *mut HCRYPTXML) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cryptxml.dll" "system" fn CryptXmlOpenToEncode(pconfig : *const CRYPT_XML_TRANSFORM_CHAIN_CONFIG, dwflags : super::DWORD, wszid : windows_core::PCWSTR, rgproperty : *const CRYPT_XML_PROPERTY, cproperty : super::ULONG, pencoded : *const CRYPT_XML_BLOB, phsignature : *mut HCRYPTXML) -> windows_core::HRESULT);
    unsafe { CryptXmlOpenToEncode(pconfig.unwrap_or(core::mem::zeroed()) as _, dwflags, wszid.param().abi(), rgproperty.map_or(core::ptr::null(), |slice| slice.as_ptr()), rgproperty.map_or(0, |slice| slice.len().try_into().unwrap()), pencoded.unwrap_or(core::mem::zeroed()) as _, phsignature as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CryptXmlSetHMACSecret(hsignature: HCRYPTXML, pbsecret: &[u8]) -> windows_core::HRESULT {
    windows_core::link!("cryptxml.dll" "system" fn CryptXmlSetHMACSecret(hsignature : HCRYPTXML, pbsecret : *const super::BYTE, cbsecret : super::ULONG) -> windows_core::HRESULT);
    unsafe { CryptXmlSetHMACSecret(hsignature, core::mem::transmute(pbsecret.as_ptr()), pbsecret.len().try_into().unwrap()) }
}
#[cfg(all(feature = "shared", feature = "wincrypt"))]
#[inline]
pub unsafe fn CryptXmlSign(hsignature: HCRYPTXML, hkey: Option<super::HCRYPTPROV_OR_NCRYPT_KEY_HANDLE>, dwkeyspec: super::DWORD, dwflags: super::DWORD, dwkeyinfospec: CRYPT_XML_KEYINFO_SPEC, pvkeyinfospec: Option<*const core::ffi::c_void>, psignaturemethod: *const CRYPT_XML_ALGORITHM, pcanonicalization: *const CRYPT_XML_ALGORITHM) -> windows_core::HRESULT {
    windows_core::link!("cryptxml.dll" "system" fn CryptXmlSign(hsignature : HCRYPTXML, hkey : super::HCRYPTPROV_OR_NCRYPT_KEY_HANDLE, dwkeyspec : super::DWORD, dwflags : super::DWORD, dwkeyinfospec : CRYPT_XML_KEYINFO_SPEC, pvkeyinfospec : *const core::ffi::c_void, psignaturemethod : *const CRYPT_XML_ALGORITHM, pcanonicalization : *const CRYPT_XML_ALGORITHM) -> windows_core::HRESULT);
    unsafe { CryptXmlSign(hsignature, hkey.unwrap_or(core::mem::zeroed()) as _, dwkeyspec, dwflags, dwkeyinfospec, pvkeyinfospec.unwrap_or(core::mem::zeroed()) as _, psignaturemethod, pcanonicalization) }
}
#[cfg(all(feature = "bcrypt", feature = "shared"))]
#[inline]
pub unsafe fn CryptXmlVerifySignature(hsignature: HCRYPTXML, hkey: Option<super::BCRYPT_KEY_HANDLE>, dwflags: super::DWORD) -> windows_core::HRESULT {
    windows_core::link!("cryptxml.dll" "system" fn CryptXmlVerifySignature(hsignature : HCRYPTXML, hkey : super::BCRYPT_KEY_HANDLE, dwflags : super::DWORD) -> windows_core::HRESULT);
    unsafe { CryptXmlVerifySignature(hsignature, hkey.unwrap_or(core::mem::zeroed()) as _, dwflags) }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_XML_ALGORITHM {
    pub cbSize: super::ULONG,
    pub wszAlgorithm: windows_core::PCWSTR,
    pub Encoded: CRYPT_XML_BLOB,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_XML_ALGORITHM_INFO {
    pub cbSize: super::DWORD,
    pub wszAlgorithmURI: *mut super::WCHAR,
    pub wszName: *mut super::WCHAR,
    pub dwGroupId: super::DWORD,
    pub wszCNGAlgid: *mut super::WCHAR,
    pub wszCNGExtraAlgid: *mut super::WCHAR,
    pub dwSignFlags: super::DWORD,
    pub dwVerifyFlags: super::DWORD,
    pub pvPaddingInfo: *mut core::ffi::c_void,
    pub pvExtraInfo: *mut core::ffi::c_void,
}
pub const CRYPT_XML_ALGORITHM_INFO_FIND_BY_CNG_ALGID: i32 = 3;
pub const CRYPT_XML_ALGORITHM_INFO_FIND_BY_CNG_SIGN_ALGID: i32 = 4;
pub const CRYPT_XML_ALGORITHM_INFO_FIND_BY_NAME: i32 = 2;
pub const CRYPT_XML_ALGORITHM_INFO_FIND_BY_URI: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_XML_BLOB {
    pub dwCharset: CRYPT_XML_CHARSET,
    pub cbData: super::ULONG,
    pub pbData: *mut super::BYTE,
}
pub const CRYPT_XML_BLOB_MAX: i32 = 2147483640;
pub type CRYPT_XML_CHARSET = i32;
pub const CRYPT_XML_CHARSET_AUTO: CRYPT_XML_CHARSET = 0;
pub const CRYPT_XML_CHARSET_UTF16BE: CRYPT_XML_CHARSET = 3;
pub const CRYPT_XML_CHARSET_UTF16LE: CRYPT_XML_CHARSET = 2;
pub const CRYPT_XML_CHARSET_UTF8: CRYPT_XML_CHARSET = 1;
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "shared", feature = "wincrypt", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct CRYPT_XML_CRYPTOGRAPHIC_INTERFACE {
    pub cbSize: super::ULONG,
    pub fpCryptXmlEncodeAlgorithm: CryptXmlDllEncodeAlgorithm,
    pub fpCryptXmlCreateDigest: CryptXmlDllCreateDigest,
    pub fpCryptXmlDigestData: CryptXmlDllDigestData,
    pub fpCryptXmlFinalizeDigest: CryptXmlDllFinalizeDigest,
    pub fpCryptXmlCloseDigest: CryptXmlDllCloseDigest,
    pub fpCryptXmlSignData: CryptXmlDllSignData,
    pub fpCryptXmlVerifySignature: CryptXmlDllVerifySignature,
    pub fpCryptXmlGetAlgorithmInfo: CryptXmlDllGetAlgorithmInfo,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_XML_DATA_BLOB {
    pub cbData: super::ULONG,
    pub pbData: *mut super::BYTE,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default)]
pub struct CRYPT_XML_DATA_PROVIDER {
    pub pvCallbackState: *mut core::ffi::c_void,
    pub cbBufferSize: super::ULONG,
    pub pfnRead: PFN_CRYPT_XML_DATA_PROVIDER_READ,
    pub pfnClose: PFN_CRYPT_XML_DATA_PROVIDER_CLOSE,
}
pub type CRYPT_XML_DIGEST = *mut core::ffi::c_void;
pub const CRYPT_XML_DIGEST_REFERENCE_DATA_TRANSFORMED: i32 = 1;
pub const CRYPT_XML_DIGEST_VALUE_MAX: i32 = 128;
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "shared", feature = "wincrypt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_XML_DOC_CTXT {
    pub cbSize: super::ULONG,
    pub hDocCtxt: HCRYPTXML,
    pub pTransformsConfig: *mut CRYPT_XML_TRANSFORM_CHAIN_CONFIG,
    pub cSignature: super::ULONG,
    pub rgpSignature: *mut PCRYPT_XML_SIGNATURE,
}
pub const CRYPT_XML_E_ALGORITHM: windows_core::HRESULT = windows_core::HRESULT(0x80092104_u32 as _);
pub const CRYPT_XML_E_BASE: windows_core::HRESULT = windows_core::HRESULT(0x80092100_u32 as _);
pub const CRYPT_XML_E_ENCODING: windows_core::HRESULT = windows_core::HRESULT(0x80092103_u32 as _);
pub const CRYPT_XML_E_HANDLE: windows_core::HRESULT = windows_core::HRESULT(0x80092106_u32 as _);
pub const CRYPT_XML_E_HASH_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x8009210B_u32 as _);
pub const CRYPT_XML_E_INVALID_DIGEST: windows_core::HRESULT = windows_core::HRESULT(0x80092109_u32 as _);
pub const CRYPT_XML_E_INVALID_KEYVALUE: windows_core::HRESULT = windows_core::HRESULT(0x8009210F_u32 as _);
pub const CRYPT_XML_E_INVALID_SIGNATURE: windows_core::HRESULT = windows_core::HRESULT(0x8009210A_u32 as _);
pub const CRYPT_XML_E_LARGE: windows_core::HRESULT = windows_core::HRESULT(0x80092101_u32 as _);
pub const CRYPT_XML_E_LAST: windows_core::HRESULT = windows_core::HRESULT(0x80092112_u32 as _);
pub const CRYPT_XML_E_NON_UNIQUE_ID: windows_core::HRESULT = windows_core::HRESULT(0x80092112_u32 as _);
pub const CRYPT_XML_E_OPERATION: windows_core::HRESULT = windows_core::HRESULT(0x80092107_u32 as _);
pub const CRYPT_XML_E_SIGNER: windows_core::HRESULT = windows_core::HRESULT(0x80092111_u32 as _);
pub const CRYPT_XML_E_SIGN_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x8009210C_u32 as _);
pub const CRYPT_XML_E_TOO_MANY_SIGNATURES: windows_core::HRESULT = windows_core::HRESULT(0x8009210E_u32 as _);
pub const CRYPT_XML_E_TOO_MANY_TRANSFORMS: windows_core::HRESULT = windows_core::HRESULT(0x80092102_u32 as _);
pub const CRYPT_XML_E_TRANSFORM: windows_core::HRESULT = windows_core::HRESULT(0x80092105_u32 as _);
pub const CRYPT_XML_E_UNEXPECTED_XML: windows_core::HRESULT = windows_core::HRESULT(0x80092110_u32 as _);
pub const CRYPT_XML_E_UNRESOLVED_REFERENCE: windows_core::HRESULT = windows_core::HRESULT(0x80092108_u32 as _);
pub const CRYPT_XML_E_VERIFY_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x8009210D_u32 as _);
pub const CRYPT_XML_FLAG_ADD_OBJECT_CREATE_COPY: i32 = 1;
pub const CRYPT_XML_FLAG_ALWAYS_RETURN_ENCODED_OBJECT: i32 = 1073741824;
pub const CRYPT_XML_FLAG_CREATE_REFERENCE_AS_OBJECT: i32 = 1;
pub const CRYPT_XML_FLAG_DISABLE_EXTENSIONS: i32 = 268435456;
pub const CRYPT_XML_FLAG_ECDSA_DSIG11: i32 = 67108864;
pub const CRYPT_XML_FLAG_ENFORCE_ID_NAME_FORMAT: i32 = 134217728;
pub const CRYPT_XML_FLAG_ENFORCE_ID_NCNAME_FORMAT: i32 = 536870912;
pub const CRYPT_XML_FLAG_NO_SERIALIZE: u32 = 2147483648;
pub const CRYPT_XML_GROUP_ID_HASH: i32 = 1;
pub const CRYPT_XML_GROUP_ID_SIGN: i32 = 2;
pub const CRYPT_XML_ID_MAX: i32 = 256;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_XML_ISSUER_SERIAL {
    pub wszIssuer: windows_core::PCWSTR,
    pub wszSerial: windows_core::PCWSTR,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wincrypt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_XML_KEYINFO_PARAM {
    pub wszId: windows_core::PCWSTR,
    pub wszKeyName: windows_core::PCWSTR,
    pub SKI: super::CERT_BLOB,
    pub wszSubjectName: windows_core::PCWSTR,
    pub cCertificate: super::ULONG,
    pub rgCertificate: *mut super::CERT_BLOB,
    pub cCRL: super::ULONG,
    pub rgCRL: *mut super::CERT_BLOB,
}
pub type CRYPT_XML_KEYINFO_SPEC = i32;
pub const CRYPT_XML_KEYINFO_SPEC_ENCODED: CRYPT_XML_KEYINFO_SPEC = 1;
pub const CRYPT_XML_KEYINFO_SPEC_NONE: CRYPT_XML_KEYINFO_SPEC = 0;
pub const CRYPT_XML_KEYINFO_SPEC_PARAM: CRYPT_XML_KEYINFO_SPEC = 2;
pub const CRYPT_XML_KEYINFO_TYPE_CUSTOM: i32 = 5;
pub const CRYPT_XML_KEYINFO_TYPE_KEYNAME: i32 = 1;
pub const CRYPT_XML_KEYINFO_TYPE_KEYVALUE: i32 = 2;
pub const CRYPT_XML_KEYINFO_TYPE_RETRIEVAL: i32 = 3;
pub const CRYPT_XML_KEYINFO_TYPE_X509DATA: i32 = 4;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_XML_KEY_DSA_KEY_VALUE {
    pub P: CRYPT_XML_DATA_BLOB,
    pub Q: CRYPT_XML_DATA_BLOB,
    pub G: CRYPT_XML_DATA_BLOB,
    pub Y: CRYPT_XML_DATA_BLOB,
    pub J: CRYPT_XML_DATA_BLOB,
    pub Seed: CRYPT_XML_DATA_BLOB,
    pub Counter: CRYPT_XML_DATA_BLOB,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_XML_KEY_ECDSA_KEY_VALUE {
    pub wszNamedCurve: windows_core::PCWSTR,
    pub X: CRYPT_XML_DATA_BLOB,
    pub Y: CRYPT_XML_DATA_BLOB,
    pub ExplicitPara: CRYPT_XML_BLOB,
}
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_XML_KEY_INFO {
    pub cbSize: super::ULONG,
    pub wszId: windows_core::PCWSTR,
    pub cKeyInfo: super::UINT,
    pub rgKeyInfo: *mut CRYPT_XML_KEY_INFO_ITEM,
    pub hVerifyKey: super::BCRYPT_KEY_HANDLE,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct CRYPT_XML_KEY_INFO_ITEM {
    pub dwType: super::DWORD,
    pub Anonymous: CRYPT_XML_KEY_INFO_ITEM_0,
}
#[cfg(feature = "shared")]
impl Default for CRYPT_XML_KEY_INFO_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union CRYPT_XML_KEY_INFO_ITEM_0 {
    pub wszKeyName: windows_core::PCWSTR,
    pub KeyValue: CRYPT_XML_KEY_VALUE,
    pub RetrievalMethod: CRYPT_XML_BLOB,
    pub X509Data: CRYPT_XML_X509DATA,
    pub Custom: CRYPT_XML_BLOB,
}
#[cfg(feature = "shared")]
impl Default for CRYPT_XML_KEY_INFO_ITEM_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_XML_KEY_RSA_KEY_VALUE {
    pub Modulus: CRYPT_XML_DATA_BLOB,
    pub Exponent: CRYPT_XML_DATA_BLOB,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct CRYPT_XML_KEY_VALUE {
    pub dwType: super::DWORD,
    pub Anonymous: CRYPT_XML_KEY_VALUE_0,
}
#[cfg(feature = "shared")]
impl Default for CRYPT_XML_KEY_VALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union CRYPT_XML_KEY_VALUE_0 {
    pub DSAKeyValue: CRYPT_XML_KEY_DSA_KEY_VALUE,
    pub RSAKeyValue: CRYPT_XML_KEY_RSA_KEY_VALUE,
    pub ECDSAKeyValue: CRYPT_XML_KEY_ECDSA_KEY_VALUE,
    pub Custom: CRYPT_XML_BLOB,
}
#[cfg(feature = "shared")]
impl Default for CRYPT_XML_KEY_VALUE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_XML_KEY_VALUE_TYPE_CUSTOM: i32 = 4;
pub const CRYPT_XML_KEY_VALUE_TYPE_DSA: i32 = 1;
pub const CRYPT_XML_KEY_VALUE_TYPE_ECDSA: i32 = 3;
pub const CRYPT_XML_KEY_VALUE_TYPE_RSA: i32 = 2;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wincrypt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_XML_OBJECT {
    pub cbSize: super::ULONG,
    pub hObject: HCRYPTXML,
    pub wszId: windows_core::PCWSTR,
    pub wszMimeType: windows_core::PCWSTR,
    pub wszEncoding: windows_core::PCWSTR,
    pub Manifest: CRYPT_XML_REFERENCES,
    pub Encoded: CRYPT_XML_BLOB,
}
pub const CRYPT_XML_OBJECTS_MAX: i32 = 256;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_XML_PROPERTY {
    pub dwPropId: CRYPT_XML_PROPERTY_ID,
    pub pvValue: *const core::ffi::c_void,
    pub cbValue: super::ULONG,
}
pub const CRYPT_XML_PROPERTY_DOC_DECLARATION: CRYPT_XML_PROPERTY_ID = 4;
pub type CRYPT_XML_PROPERTY_ID = i32;
pub const CRYPT_XML_PROPERTY_MAX_HEAP_SIZE: CRYPT_XML_PROPERTY_ID = 1;
pub const CRYPT_XML_PROPERTY_MAX_SIGNATURES: CRYPT_XML_PROPERTY_ID = 3;
pub const CRYPT_XML_PROPERTY_SIGNATURE_LOCATION: CRYPT_XML_PROPERTY_ID = 2;
pub const CRYPT_XML_PROPERTY_XML_OUTPUT_CHARSET: CRYPT_XML_PROPERTY_ID = 5;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wincrypt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_XML_REFERENCE {
    pub cbSize: super::ULONG,
    pub hReference: HCRYPTXML,
    pub wszId: windows_core::PCWSTR,
    pub wszUri: windows_core::PCWSTR,
    pub wszType: windows_core::PCWSTR,
    pub DigestMethod: CRYPT_XML_ALGORITHM,
    pub DigestValue: super::CRYPT_DATA_BLOB,
    pub cTransform: super::ULONG,
    pub rgTransform: *mut CRYPT_XML_ALGORITHM,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wincrypt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_XML_REFERENCES {
    pub cReference: super::ULONG,
    pub rgpReference: *mut PCRYPT_XML_REFERENCE,
}
pub const CRYPT_XML_REFERENCES_MAX: i32 = 32760;
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "shared", feature = "wincrypt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_XML_SIGNATURE {
    pub cbSize: super::ULONG,
    pub hSignature: HCRYPTXML,
    pub wszId: windows_core::PCWSTR,
    pub SignedInfo: CRYPT_XML_SIGNED_INFO,
    pub SignatureValue: super::CRYPT_DATA_BLOB,
    pub pKeyInfo: *mut CRYPT_XML_KEY_INFO,
    pub cObject: super::ULONG,
    pub rgpObject: *mut PCRYPT_XML_OBJECT,
}
pub const CRYPT_XML_SIGNATURES_MAX: i32 = 16;
pub const CRYPT_XML_SIGNATURE_VALUE_MAX: i32 = 2048;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wincrypt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_XML_SIGNED_INFO {
    pub cbSize: super::ULONG,
    pub wszId: windows_core::PCWSTR,
    pub Canonicalization: CRYPT_XML_ALGORITHM,
    pub SignatureMethod: CRYPT_XML_ALGORITHM,
    pub cReference: super::ULONG,
    pub rgpReference: *mut PCRYPT_XML_REFERENCE,
    pub Encoded: CRYPT_XML_BLOB,
}
pub const CRYPT_XML_SIGN_ADD_KEYVALUE: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_XML_STATUS {
    pub cbSize: super::ULONG,
    pub dwErrorStatus: super::DWORD,
    pub dwInfoStatus: super::DWORD,
}
pub const CRYPT_XML_STATUS_DIGESTING: i32 = 4;
pub const CRYPT_XML_STATUS_DIGEST_VALID: i32 = 8;
pub const CRYPT_XML_STATUS_ERROR_DIGEST_INVALID: i32 = 2;
pub const CRYPT_XML_STATUS_ERROR_KEYINFO_NOT_PARSED: i32 = 131072;
pub const CRYPT_XML_STATUS_ERROR_NOT_RESOLVED: i32 = 1;
pub const CRYPT_XML_STATUS_ERROR_NOT_SUPPORTED_ALGORITHM: i32 = 4;
pub const CRYPT_XML_STATUS_ERROR_NOT_SUPPORTED_TRANSFORM: i32 = 8;
pub const CRYPT_XML_STATUS_ERROR_SIGNATURE_INVALID: i32 = 65536;
pub const CRYPT_XML_STATUS_INTERNAL_REFERENCE: i32 = 1;
pub const CRYPT_XML_STATUS_KEY_AVAILABLE: i32 = 2;
pub const CRYPT_XML_STATUS_NO_ERROR: i32 = 0;
pub const CRYPT_XML_STATUS_OPENED_TO_ENCODE: u32 = 2147483648;
pub const CRYPT_XML_STATUS_SIGNATURE_VALID: i32 = 65536;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_XML_TRANSFORM_CHAIN_CONFIG {
    pub cbSize: super::ULONG,
    pub cTransformInfo: super::ULONG,
    pub rgpTransformInfo: *mut PCRYPT_XML_TRANSFORM_INFO,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default)]
pub struct CRYPT_XML_TRANSFORM_INFO {
    pub cbSize: super::ULONG,
    pub wszAlgorithm: windows_core::PCWSTR,
    pub cbBufferSize: super::ULONG,
    pub dwFlags: super::DWORD,
    pub pfnCreateTransform: PFN_CRYPT_XML_CREATE_TRANSFORM,
}
pub const CRYPT_XML_TRANSFORM_MAX: i32 = 16;
pub const CRYPT_XML_TRANSFORM_ON_NODESET: i32 = 2;
pub const CRYPT_XML_TRANSFORM_ON_STREAM: i32 = 1;
pub const CRYPT_XML_TRANSFORM_URI_QUERY_STRING: i32 = 4;
pub const CRYPT_XML_URI_MAX: i32 = 8192;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_XML_X509DATA {
    pub cX509Data: super::UINT,
    pub rgX509Data: *mut CRYPT_XML_X509DATA_ITEM,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct CRYPT_XML_X509DATA_ITEM {
    pub dwType: super::DWORD,
    pub Anonymous: CRYPT_XML_X509DATA_ITEM_0,
}
#[cfg(feature = "shared")]
impl Default for CRYPT_XML_X509DATA_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union CRYPT_XML_X509DATA_ITEM_0 {
    pub IssuerSerial: CRYPT_XML_ISSUER_SERIAL,
    pub SKI: CRYPT_XML_DATA_BLOB,
    pub wszSubjectName: windows_core::PCWSTR,
    pub Certificate: CRYPT_XML_DATA_BLOB,
    pub CRL: CRYPT_XML_DATA_BLOB,
    pub Custom: CRYPT_XML_BLOB,
}
#[cfg(feature = "shared")]
impl Default for CRYPT_XML_X509DATA_ITEM_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_XML_X509DATA_TYPE_CERTIFICATE: i32 = 4;
pub const CRYPT_XML_X509DATA_TYPE_CRL: i32 = 5;
pub const CRYPT_XML_X509DATA_TYPE_CUSTOM: i32 = 6;
pub const CRYPT_XML_X509DATA_TYPE_ISSUER_SERIAL: i32 = 1;
pub const CRYPT_XML_X509DATA_TYPE_SKI: i32 = 2;
pub const CRYPT_XML_X509DATA_TYPE_SUBJECT_NAME: i32 = 3;
pub type CryptXmlDllCloseDigest = Option<unsafe extern "system" fn(hdigest: CRYPT_XML_DIGEST) -> windows_core::HRESULT>;
#[cfg(feature = "shared")]
pub type CryptXmlDllCreateDigest = Option<unsafe extern "system" fn(pdigestmethod: *const CRYPT_XML_ALGORITHM, pcbsize: *mut super::ULONG, phdigest: *mut CRYPT_XML_DIGEST) -> windows_core::HRESULT>;
#[cfg(all(feature = "bcrypt", feature = "shared"))]
pub type CryptXmlDllCreateKey = Option<unsafe extern "system" fn(pencoded: *const CRYPT_XML_BLOB, phkey: *mut super::BCRYPT_KEY_HANDLE) -> windows_core::HRESULT>;
#[cfg(feature = "shared")]
pub type CryptXmlDllDigestData = Option<unsafe extern "system" fn(hdigest: CRYPT_XML_DIGEST, pbdata: *const super::BYTE, cbdata: super::ULONG) -> windows_core::HRESULT>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type CryptXmlDllEncodeAlgorithm = Option<unsafe extern "system" fn(palginfo: *const CRYPT_XML_ALGORITHM_INFO, dwcharset: CRYPT_XML_CHARSET, pvcallbackstate: *mut core::ffi::c_void, pfnwrite: PFN_CRYPT_XML_WRITE_CALLBACK) -> windows_core::HRESULT>;
#[cfg(all(feature = "ncrypt", feature = "shared"))]
pub type CryptXmlDllEncodeKeyValue = Option<unsafe extern "system" fn(hkey: super::NCRYPT_KEY_HANDLE, dwcharset: CRYPT_XML_CHARSET, pvcallbackstate: *mut core::ffi::c_void, pfnwrite: PFN_CRYPT_XML_WRITE_CALLBACK) -> windows_core::HRESULT>;
#[cfg(feature = "shared")]
pub type CryptXmlDllFinalizeDigest = Option<unsafe extern "system" fn(hdigest: CRYPT_XML_DIGEST, pbdigest: *mut super::BYTE, cbdigest: super::ULONG) -> windows_core::HRESULT>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type CryptXmlDllGetAlgorithmInfo = Option<unsafe extern "system" fn(pxmlalgorithm: *const CRYPT_XML_ALGORITHM, ppalginfo: *mut *mut CRYPT_XML_ALGORITHM_INFO) -> windows_core::HRESULT>;
#[cfg(all(feature = "bcrypt", feature = "shared", feature = "wincrypt", feature = "winnt"))]
pub type CryptXmlDllGetInterface = Option<unsafe extern "system" fn(dwflags: super::DWORD, pmethod: *const CRYPT_XML_ALGORITHM_INFO, pinterface: *mut CRYPT_XML_CRYPTOGRAPHIC_INTERFACE) -> windows_core::HRESULT>;
#[cfg(all(feature = "shared", feature = "wincrypt"))]
pub type CryptXmlDllSignData = Option<unsafe extern "system" fn(psignaturemethod: *const CRYPT_XML_ALGORITHM, hcryptprovorncryptkey: super::HCRYPTPROV_OR_NCRYPT_KEY_HANDLE, dwkeyspec: super::DWORD, pbinput: *const super::BYTE, cbinput: super::ULONG, pboutput: *mut super::BYTE, cboutput: super::ULONG, pcbresult: *mut super::ULONG) -> windows_core::HRESULT>;
#[cfg(all(feature = "bcrypt", feature = "shared"))]
pub type CryptXmlDllVerifySignature = Option<unsafe extern "system" fn(psignaturemethod: *const CRYPT_XML_ALGORITHM, hkey: super::BCRYPT_KEY_HANDLE, pbinput: *const super::BYTE, cbinput: super::ULONG, pbsignature: *const super::BYTE, cbsignature: super::ULONG) -> windows_core::HRESULT>;
pub type HCRYPTXML = *mut core::ffi::c_void;
#[cfg(feature = "shared")]
pub type PCRYPT_XML_ALGORITHM = *mut CRYPT_XML_ALGORITHM;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PCRYPT_XML_ALGORITHM_INFO = *mut CRYPT_XML_ALGORITHM_INFO;
#[cfg(feature = "shared")]
pub type PCRYPT_XML_BLOB = *mut CRYPT_XML_BLOB;
#[cfg(all(feature = "bcrypt", feature = "shared", feature = "wincrypt", feature = "winnt"))]
pub type PCRYPT_XML_CRYPTOGRAPHIC_INTERFACE = *mut CRYPT_XML_CRYPTOGRAPHIC_INTERFACE;
#[cfg(all(feature = "bcrypt", feature = "shared", feature = "wincrypt", feature = "winnt"))]
pub type PCRYPT_XML_CRYPTO_PROVIDER = *mut CRYPT_XML_CRYPTOGRAPHIC_INTERFACE;
#[cfg(feature = "shared")]
pub type PCRYPT_XML_DATA_BLOB = *mut CRYPT_XML_DATA_BLOB;
#[cfg(feature = "shared")]
pub type PCRYPT_XML_DATA_PROVIDER = *mut CRYPT_XML_DATA_PROVIDER;
#[cfg(all(feature = "bcrypt", feature = "shared", feature = "wincrypt"))]
pub type PCRYPT_XML_DOC_CTXT = *mut CRYPT_XML_DOC_CTXT;
#[cfg(all(feature = "bcrypt", feature = "shared"))]
pub type PCRYPT_XML_KEY_INFO = *mut CRYPT_XML_KEY_INFO;
#[cfg(all(feature = "shared", feature = "wincrypt"))]
pub type PCRYPT_XML_OBJECT = *mut CRYPT_XML_OBJECT;
#[cfg(feature = "shared")]
pub type PCRYPT_XML_PROPERTY = *mut CRYPT_XML_PROPERTY;
#[cfg(all(feature = "shared", feature = "wincrypt"))]
pub type PCRYPT_XML_REFERENCE = *mut CRYPT_XML_REFERENCE;
#[cfg(all(feature = "shared", feature = "wincrypt"))]
pub type PCRYPT_XML_REFERENCES = *mut CRYPT_XML_REFERENCES;
#[cfg(all(feature = "bcrypt", feature = "shared", feature = "wincrypt"))]
pub type PCRYPT_XML_SIGNATURE = *mut CRYPT_XML_SIGNATURE;
#[cfg(all(feature = "shared", feature = "wincrypt"))]
pub type PCRYPT_XML_SIGNED_INFO = *mut CRYPT_XML_SIGNED_INFO;
#[cfg(feature = "shared")]
pub type PCRYPT_XML_STATUS = *mut CRYPT_XML_STATUS;
#[cfg(feature = "shared")]
pub type PCRYPT_XML_TRANSFORM_CHAIN_CONFIG = *mut CRYPT_XML_TRANSFORM_CHAIN_CONFIG;
#[cfg(feature = "shared")]
pub type PCRYPT_XML_TRANSFORM_INFO = *mut CRYPT_XML_TRANSFORM_INFO;
#[cfg(feature = "shared")]
pub type PFN_CRYPT_XML_CREATE_TRANSFORM = Option<unsafe extern "system" fn(ptransform: *const CRYPT_XML_ALGORITHM, pproviderin: *const CRYPT_XML_DATA_PROVIDER, pproviderout: *mut CRYPT_XML_DATA_PROVIDER) -> windows_core::HRESULT>;
pub type PFN_CRYPT_XML_DATA_PROVIDER_CLOSE = Option<unsafe extern "system" fn(pvcallbackstate: *mut core::ffi::c_void) -> windows_core::HRESULT>;
#[cfg(feature = "shared")]
pub type PFN_CRYPT_XML_DATA_PROVIDER_READ = Option<unsafe extern "system" fn(pvcallbackstate: *mut core::ffi::c_void, pbdata: *mut super::BYTE, cbdata: super::ULONG, pcbread: *mut super::ULONG) -> windows_core::HRESULT>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PFN_CRYPT_XML_ENUM_ALG_INFO = Option<unsafe extern "system" fn(pinfo: *const CRYPT_XML_ALGORITHM_INFO, pvarg: *mut core::ffi::c_void) -> windows_core::BOOL>;
#[cfg(feature = "shared")]
pub type PFN_CRYPT_XML_WRITE_CALLBACK = Option<unsafe extern "system" fn(pvcallbackstate: *mut core::ffi::c_void, pbdata: *const super::BYTE, cbdata: super::ULONG) -> windows_core::HRESULT>;
pub const wszURI_CANONICALIZATION_C14N: windows_core::PCWSTR = windows_core::w!("http://www.w3.org/TR/2001/REC-xml-c14n-20010315");
pub const wszURI_CANONICALIZATION_C14NC: windows_core::PCWSTR = windows_core::w!("http://www.w3.org/TR/2001/REC-xml-c14n-20010315#WithComments");
pub const wszURI_CANONICALIZATION_EXSLUSIVE_C14N: windows_core::PCWSTR = windows_core::w!("http://www.w3.org/2001/10/xml-exc-c14n#");
pub const wszURI_CANONICALIZATION_EXSLUSIVE_C14NC: windows_core::PCWSTR = windows_core::w!("http://www.w3.org/2001/10/xml-exc-c14n#WithComments");
pub const wszURI_TRANSFORM_XPATH: windows_core::PCWSTR = windows_core::w!("http://www.w3.org/TR/1999/REC-xpath-19991116");
pub const wszURI_XMLNS_DIGSIG_BASE64: windows_core::PCWSTR = windows_core::w!("http://www.w3.org/2000/09/xmldsig#base64");
pub const wszURI_XMLNS_DIGSIG_DSA_SHA1: windows_core::PCWSTR = windows_core::w!("http://www.w3.org/2000/09/xmldsig#dsa-sha1");
pub const wszURI_XMLNS_DIGSIG_ECDSA_SHA1: windows_core::PCWSTR = windows_core::w!("http://www.w3.org/2001/04/xmldsig-more#ecdsa-sha1");
pub const wszURI_XMLNS_DIGSIG_ECDSA_SHA256: windows_core::PCWSTR = windows_core::w!("http://www.w3.org/2001/04/xmldsig-more#ecdsa-sha256");
pub const wszURI_XMLNS_DIGSIG_ECDSA_SHA384: windows_core::PCWSTR = windows_core::w!("http://www.w3.org/2001/04/xmldsig-more#ecdsa-sha384");
pub const wszURI_XMLNS_DIGSIG_ECDSA_SHA512: windows_core::PCWSTR = windows_core::w!("http://www.w3.org/2001/04/xmldsig-more#ecdsa-sha512");
pub const wszURI_XMLNS_DIGSIG_HMAC_SHA1: windows_core::PCWSTR = windows_core::w!("http://www.w3.org/2000/09/xmldsig#hmac-sha1");
pub const wszURI_XMLNS_DIGSIG_HMAC_SHA256: windows_core::PCWSTR = windows_core::w!("http://www.w3.org/2001/04/xmldsig-more#hmac-sha256");
pub const wszURI_XMLNS_DIGSIG_HMAC_SHA384: windows_core::PCWSTR = windows_core::w!("http://www.w3.org/2001/04/xmldsig-more#hmac-sha384");
pub const wszURI_XMLNS_DIGSIG_HMAC_SHA512: windows_core::PCWSTR = windows_core::w!("http://www.w3.org/2001/04/xmldsig-more#hmac-sha512");
pub const wszURI_XMLNS_DIGSIG_RSA_SHA1: windows_core::PCWSTR = windows_core::w!("http://www.w3.org/2000/09/xmldsig#rsa-sha1");
pub const wszURI_XMLNS_DIGSIG_RSA_SHA256: windows_core::PCWSTR = windows_core::w!("http://www.w3.org/2001/04/xmldsig-more#rsa-sha256");
pub const wszURI_XMLNS_DIGSIG_RSA_SHA384: windows_core::PCWSTR = windows_core::w!("http://www.w3.org/2001/04/xmldsig-more#rsa-sha384");
pub const wszURI_XMLNS_DIGSIG_RSA_SHA512: windows_core::PCWSTR = windows_core::w!("http://www.w3.org/2001/04/xmldsig-more#rsa-sha512");
pub const wszURI_XMLNS_DIGSIG_SHA1: windows_core::PCWSTR = windows_core::w!("http://www.w3.org/2000/09/xmldsig#sha1");
pub const wszURI_XMLNS_DIGSIG_SHA256: windows_core::PCWSTR = windows_core::w!("http://www.w3.org/2001/04/xmlenc#sha256");
pub const wszURI_XMLNS_DIGSIG_SHA384: windows_core::PCWSTR = windows_core::w!("http://www.w3.org/2001/04/xmldsig-more#sha384");
pub const wszURI_XMLNS_DIGSIG_SHA512: windows_core::PCWSTR = windows_core::w!("http://www.w3.org/2001/04/xmlenc#sha512");
pub const wszURI_XMLNS_TRANSFORM_BASE64: windows_core::PCWSTR = windows_core::w!("http://www.w3.org/2000/09/xmldsig#base64");
pub const wszURI_XMLNS_TRANSFORM_ENVELOPED: windows_core::PCWSTR = windows_core::w!("http://www.w3.org/2000/09/xmldsig#enveloped-signature");
pub const wszXMLNS_DIGSIG: windows_core::PCWSTR = windows_core::w!("http://www.w3.org/2000/09/xmldsig#");
pub const wszXMLNS_DIGSIG_Id: windows_core::PCWSTR = windows_core::w!("Id");
pub const wszXMLNS_DIGSIG_SignatureProperties: windows_core::PCWSTR = windows_core::w!("http://www.w3.org/2000/09/xmldsig#SignatureProperties");
