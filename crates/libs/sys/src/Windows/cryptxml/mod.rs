#[cfg(feature = "wincrypt")]
windows_link::link!("cryptxml.dll" "system" fn CryptXmlAddObject(hsignatureorobject : HCRYPTXML, dwflags : u32, rgproperty : *const CRYPT_XML_PROPERTY, cproperty : u32, pencoded : *const CRYPT_XML_BLOB, ppobject : *mut *mut CRYPT_XML_OBJECT) -> windows_sys::core::HRESULT);
windows_link::link!("cryptxml.dll" "system" fn CryptXmlClose(hcryptxml : HCRYPTXML) -> windows_sys::core::HRESULT);
windows_link::link!("cryptxml.dll" "system" fn CryptXmlCreateReference(hcryptxml : HCRYPTXML, dwflags : u32, wszid : windows_sys::core::PCWSTR, wszuri : windows_sys::core::PCWSTR, wsztype : windows_sys::core::PCWSTR, pdigestmethod : *const CRYPT_XML_ALGORITHM, ctransform : u32, rgtransform : *const CRYPT_XML_ALGORITHM, phreference : *mut HCRYPTXML) -> windows_sys::core::HRESULT);
windows_link::link!("cryptxml.dll" "system" fn CryptXmlDigestReference(hreference : HCRYPTXML, dwflags : u32, pdataproviderin : *const CRYPT_XML_DATA_PROVIDER) -> windows_sys::core::HRESULT);
windows_link::link!("cryptxml.dll" "system" fn CryptXmlEncode(hcryptxml : HCRYPTXML, dwcharset : CRYPT_XML_CHARSET, rgproperty : *const CRYPT_XML_PROPERTY, cproperty : u32, pvcallbackstate : *mut core::ffi::c_void, pfnwrite : PFN_CRYPT_XML_WRITE_CALLBACK) -> windows_sys::core::HRESULT);
windows_link::link!("cryptxml.dll" "system" fn CryptXmlEnumAlgorithmInfo(dwgroupid : u32, dwflags : u32, pvarg : *mut core::ffi::c_void, pfnenumalginfo : PFN_CRYPT_XML_ENUM_ALG_INFO) -> windows_sys::core::HRESULT);
windows_link::link!("cryptxml.dll" "system" fn CryptXmlFindAlgorithmInfo(dwfindbytype : u32, pvfindby : *const core::ffi::c_void, dwgroupid : u32, dwflags : u32) -> *const CRYPT_XML_ALGORITHM_INFO);
windows_link::link!("cryptxml.dll" "system" fn CryptXmlGetAlgorithmInfo(pxmlalgorithm : *const CRYPT_XML_ALGORITHM, dwflags : u32, ppalginfo : *mut *mut CRYPT_XML_ALGORITHM_INFO) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "bcrypt", feature = "wincrypt"))]
windows_link::link!("cryptxml.dll" "system" fn CryptXmlGetDocContext(hcryptxml : HCRYPTXML, ppstruct : *mut *mut CRYPT_XML_DOC_CTXT) -> windows_sys::core::HRESULT);
#[cfg(feature = "wincrypt")]
windows_link::link!("cryptxml.dll" "system" fn CryptXmlGetReference(hcryptxml : HCRYPTXML, ppstruct : *mut *mut CRYPT_XML_REFERENCE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "bcrypt", feature = "wincrypt"))]
windows_link::link!("cryptxml.dll" "system" fn CryptXmlGetSignature(hcryptxml : HCRYPTXML, ppstruct : *mut *mut CRYPT_XML_SIGNATURE) -> windows_sys::core::HRESULT);
windows_link::link!("cryptxml.dll" "system" fn CryptXmlGetStatus(hcryptxml : HCRYPTXML, pstatus : *mut CRYPT_XML_STATUS) -> windows_sys::core::HRESULT);
windows_link::link!("cryptxml.dll" "system" fn CryptXmlGetTransforms(ppconfig : *mut *mut CRYPT_XML_TRANSFORM_CHAIN_CONFIG) -> windows_sys::core::HRESULT);
#[cfg(feature = "bcrypt")]
windows_link::link!("cryptxml.dll" "system" fn CryptXmlImportPublicKey(dwflags : u32, pkeyvalue : *const CRYPT_XML_KEY_VALUE, phkey : *mut super::bcrypt::BCRYPT_KEY_HANDLE) -> windows_sys::core::HRESULT);
windows_link::link!("cryptxml.dll" "system" fn CryptXmlOpenToDecode(pconfig : *const CRYPT_XML_TRANSFORM_CHAIN_CONFIG, dwflags : u32, rgproperty : *const CRYPT_XML_PROPERTY, cproperty : u32, pencoded : *const CRYPT_XML_BLOB, phcryptxml : *mut HCRYPTXML) -> windows_sys::core::HRESULT);
windows_link::link!("cryptxml.dll" "system" fn CryptXmlOpenToEncode(pconfig : *const CRYPT_XML_TRANSFORM_CHAIN_CONFIG, dwflags : u32, wszid : windows_sys::core::PCWSTR, rgproperty : *const CRYPT_XML_PROPERTY, cproperty : u32, pencoded : *const CRYPT_XML_BLOB, phsignature : *mut HCRYPTXML) -> windows_sys::core::HRESULT);
windows_link::link!("cryptxml.dll" "system" fn CryptXmlSetHMACSecret(hsignature : HCRYPTXML, pbsecret : *const u8, cbsecret : u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "wincrypt")]
windows_link::link!("cryptxml.dll" "system" fn CryptXmlSign(hsignature : HCRYPTXML, hkey : super::wincrypt::HCRYPTPROV_OR_NCRYPT_KEY_HANDLE, dwkeyspec : u32, dwflags : u32, dwkeyinfospec : CRYPT_XML_KEYINFO_SPEC, pvkeyinfospec : *const core::ffi::c_void, psignaturemethod : *const CRYPT_XML_ALGORITHM, pcanonicalization : *const CRYPT_XML_ALGORITHM) -> windows_sys::core::HRESULT);
#[cfg(feature = "bcrypt")]
windows_link::link!("cryptxml.dll" "system" fn CryptXmlVerifySignature(hsignature : HCRYPTXML, hkey : super::bcrypt::BCRYPT_KEY_HANDLE, dwflags : u32) -> windows_sys::core::HRESULT);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPT_XML_ALGORITHM {
    pub cbSize: u32,
    pub wszAlgorithm: windows_sys::core::PCWSTR,
    pub Encoded: CRYPT_XML_BLOB,
}
impl Default for CRYPT_XML_ALGORITHM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPT_XML_ALGORITHM_INFO {
    pub cbSize: u32,
    pub wszAlgorithmURI: *mut u16,
    pub wszName: *mut u16,
    pub dwGroupId: u32,
    pub wszCNGAlgid: *mut u16,
    pub wszCNGExtraAlgid: *mut u16,
    pub dwSignFlags: u32,
    pub dwVerifyFlags: u32,
    pub pvPaddingInfo: *mut core::ffi::c_void,
    pub pvExtraInfo: *mut core::ffi::c_void,
}
impl Default for CRYPT_XML_ALGORITHM_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_XML_ALGORITHM_INFO_FIND_BY_CNG_ALGID: u32 = 3;
pub const CRYPT_XML_ALGORITHM_INFO_FIND_BY_CNG_SIGN_ALGID: u32 = 4;
pub const CRYPT_XML_ALGORITHM_INFO_FIND_BY_NAME: u32 = 2;
pub const CRYPT_XML_ALGORITHM_INFO_FIND_BY_URI: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPT_XML_BLOB {
    pub dwCharset: CRYPT_XML_CHARSET,
    pub cbData: u32,
    pub pbData: *mut u8,
}
impl Default for CRYPT_XML_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_XML_BLOB_MAX: u32 = 2147483640;
pub type CRYPT_XML_CHARSET = i32;
pub const CRYPT_XML_CHARSET_AUTO: CRYPT_XML_CHARSET = 0;
pub const CRYPT_XML_CHARSET_UTF16BE: CRYPT_XML_CHARSET = 3;
pub const CRYPT_XML_CHARSET_UTF16LE: CRYPT_XML_CHARSET = 2;
pub const CRYPT_XML_CHARSET_UTF8: CRYPT_XML_CHARSET = 1;
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "wincrypt"))]
#[derive(Clone, Copy, Default)]
pub struct CRYPT_XML_CRYPTOGRAPHIC_INTERFACE {
    pub cbSize: u32,
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
#[derive(Clone, Copy)]
pub struct CRYPT_XML_DATA_BLOB {
    pub cbData: u32,
    pub pbData: *mut u8,
}
impl Default for CRYPT_XML_DATA_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPT_XML_DATA_PROVIDER {
    pub pvCallbackState: *mut core::ffi::c_void,
    pub cbBufferSize: u32,
    pub pfnRead: PFN_CRYPT_XML_DATA_PROVIDER_READ,
    pub pfnClose: PFN_CRYPT_XML_DATA_PROVIDER_CLOSE,
}
impl Default for CRYPT_XML_DATA_PROVIDER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type CRYPT_XML_DIGEST = *mut core::ffi::c_void;
pub const CRYPT_XML_DIGEST_REFERENCE_DATA_TRANSFORMED: u32 = 1;
pub const CRYPT_XML_DIGEST_VALUE_MAX: u32 = 128;
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "wincrypt"))]
#[derive(Clone, Copy)]
pub struct CRYPT_XML_DOC_CTXT {
    pub cbSize: u32,
    pub hDocCtxt: HCRYPTXML,
    pub pTransformsConfig: *mut CRYPT_XML_TRANSFORM_CHAIN_CONFIG,
    pub cSignature: u32,
    pub rgpSignature: *mut PCRYPT_XML_SIGNATURE,
}
#[cfg(all(feature = "bcrypt", feature = "wincrypt"))]
impl Default for CRYPT_XML_DOC_CTXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_XML_E_ALGORITHM: windows_sys::core::HRESULT = 0x80092104_u32 as _;
pub const CRYPT_XML_E_BASE: windows_sys::core::HRESULT = 0x80092100_u32 as _;
pub const CRYPT_XML_E_ENCODING: windows_sys::core::HRESULT = 0x80092103_u32 as _;
pub const CRYPT_XML_E_HANDLE: windows_sys::core::HRESULT = 0x80092106_u32 as _;
pub const CRYPT_XML_E_HASH_FAILED: windows_sys::core::HRESULT = 0x8009210B_u32 as _;
pub const CRYPT_XML_E_INVALID_DIGEST: windows_sys::core::HRESULT = 0x80092109_u32 as _;
pub const CRYPT_XML_E_INVALID_KEYVALUE: windows_sys::core::HRESULT = 0x8009210F_u32 as _;
pub const CRYPT_XML_E_INVALID_SIGNATURE: windows_sys::core::HRESULT = 0x8009210A_u32 as _;
pub const CRYPT_XML_E_LARGE: windows_sys::core::HRESULT = 0x80092101_u32 as _;
pub const CRYPT_XML_E_LAST: windows_sys::core::HRESULT = 0x80092112_u32 as _;
pub const CRYPT_XML_E_NON_UNIQUE_ID: windows_sys::core::HRESULT = 0x80092112_u32 as _;
pub const CRYPT_XML_E_OPERATION: windows_sys::core::HRESULT = 0x80092107_u32 as _;
pub const CRYPT_XML_E_SIGNER: windows_sys::core::HRESULT = 0x80092111_u32 as _;
pub const CRYPT_XML_E_SIGN_FAILED: windows_sys::core::HRESULT = 0x8009210C_u32 as _;
pub const CRYPT_XML_E_TOO_MANY_SIGNATURES: windows_sys::core::HRESULT = 0x8009210E_u32 as _;
pub const CRYPT_XML_E_TOO_MANY_TRANSFORMS: windows_sys::core::HRESULT = 0x80092102_u32 as _;
pub const CRYPT_XML_E_TRANSFORM: windows_sys::core::HRESULT = 0x80092105_u32 as _;
pub const CRYPT_XML_E_UNEXPECTED_XML: windows_sys::core::HRESULT = 0x80092110_u32 as _;
pub const CRYPT_XML_E_UNRESOLVED_REFERENCE: windows_sys::core::HRESULT = 0x80092108_u32 as _;
pub const CRYPT_XML_E_VERIFY_FAILED: windows_sys::core::HRESULT = 0x8009210D_u32 as _;
pub const CRYPT_XML_FLAG_ADD_OBJECT_CREATE_COPY: u32 = 1;
pub const CRYPT_XML_FLAG_ALWAYS_RETURN_ENCODED_OBJECT: u32 = 1073741824;
pub const CRYPT_XML_FLAG_CREATE_REFERENCE_AS_OBJECT: u32 = 1;
pub const CRYPT_XML_FLAG_DISABLE_EXTENSIONS: u32 = 268435456;
pub const CRYPT_XML_FLAG_ECDSA_DSIG11: u32 = 67108864;
pub const CRYPT_XML_FLAG_ENFORCE_ID_NAME_FORMAT: u32 = 134217728;
pub const CRYPT_XML_FLAG_ENFORCE_ID_NCNAME_FORMAT: u32 = 536870912;
pub const CRYPT_XML_FLAG_NO_SERIALIZE: u32 = 2147483648;
pub const CRYPT_XML_GROUP_ID_HASH: u32 = 1;
pub const CRYPT_XML_GROUP_ID_SIGN: u32 = 2;
pub const CRYPT_XML_ID_MAX: u32 = 256;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPT_XML_ISSUER_SERIAL {
    pub wszIssuer: windows_sys::core::PCWSTR,
    pub wszSerial: windows_sys::core::PCWSTR,
}
impl Default for CRYPT_XML_ISSUER_SERIAL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "wincrypt")]
#[derive(Clone, Copy)]
pub struct CRYPT_XML_KEYINFO_PARAM {
    pub wszId: windows_sys::core::PCWSTR,
    pub wszKeyName: windows_sys::core::PCWSTR,
    pub SKI: super::wincrypt::CERT_BLOB,
    pub wszSubjectName: windows_sys::core::PCWSTR,
    pub cCertificate: u32,
    pub rgCertificate: *mut super::wincrypt::CERT_BLOB,
    pub cCRL: u32,
    pub rgCRL: *mut super::wincrypt::CERT_BLOB,
}
#[cfg(feature = "wincrypt")]
impl Default for CRYPT_XML_KEYINFO_PARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type CRYPT_XML_KEYINFO_SPEC = i32;
pub const CRYPT_XML_KEYINFO_SPEC_ENCODED: CRYPT_XML_KEYINFO_SPEC = 1;
pub const CRYPT_XML_KEYINFO_SPEC_NONE: CRYPT_XML_KEYINFO_SPEC = 0;
pub const CRYPT_XML_KEYINFO_SPEC_PARAM: CRYPT_XML_KEYINFO_SPEC = 2;
pub const CRYPT_XML_KEYINFO_TYPE_CUSTOM: u32 = 5;
pub const CRYPT_XML_KEYINFO_TYPE_KEYNAME: u32 = 1;
pub const CRYPT_XML_KEYINFO_TYPE_KEYVALUE: u32 = 2;
pub const CRYPT_XML_KEYINFO_TYPE_RETRIEVAL: u32 = 3;
pub const CRYPT_XML_KEYINFO_TYPE_X509DATA: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
#[derive(Clone, Copy)]
pub struct CRYPT_XML_KEY_ECDSA_KEY_VALUE {
    pub wszNamedCurve: windows_sys::core::PCWSTR,
    pub X: CRYPT_XML_DATA_BLOB,
    pub Y: CRYPT_XML_DATA_BLOB,
    pub ExplicitPara: CRYPT_XML_BLOB,
}
impl Default for CRYPT_XML_KEY_ECDSA_KEY_VALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "bcrypt")]
#[derive(Clone, Copy)]
pub struct CRYPT_XML_KEY_INFO {
    pub cbSize: u32,
    pub wszId: windows_sys::core::PCWSTR,
    pub cKeyInfo: u32,
    pub rgKeyInfo: *mut CRYPT_XML_KEY_INFO_ITEM,
    pub hVerifyKey: super::bcrypt::BCRYPT_KEY_HANDLE,
}
#[cfg(feature = "bcrypt")]
impl Default for CRYPT_XML_KEY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPT_XML_KEY_INFO_ITEM {
    pub dwType: u32,
    pub Anonymous: CRYPT_XML_KEY_INFO_ITEM_0,
}
impl Default for CRYPT_XML_KEY_INFO_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CRYPT_XML_KEY_INFO_ITEM_0 {
    pub wszKeyName: windows_sys::core::PCWSTR,
    pub KeyValue: CRYPT_XML_KEY_VALUE,
    pub RetrievalMethod: CRYPT_XML_BLOB,
    pub X509Data: CRYPT_XML_X509DATA,
    pub Custom: CRYPT_XML_BLOB,
}
impl Default for CRYPT_XML_KEY_INFO_ITEM_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CRYPT_XML_KEY_RSA_KEY_VALUE {
    pub Modulus: CRYPT_XML_DATA_BLOB,
    pub Exponent: CRYPT_XML_DATA_BLOB,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPT_XML_KEY_VALUE {
    pub dwType: u32,
    pub Anonymous: CRYPT_XML_KEY_VALUE_0,
}
impl Default for CRYPT_XML_KEY_VALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CRYPT_XML_KEY_VALUE_0 {
    pub DSAKeyValue: CRYPT_XML_KEY_DSA_KEY_VALUE,
    pub RSAKeyValue: CRYPT_XML_KEY_RSA_KEY_VALUE,
    pub ECDSAKeyValue: CRYPT_XML_KEY_ECDSA_KEY_VALUE,
    pub Custom: CRYPT_XML_BLOB,
}
impl Default for CRYPT_XML_KEY_VALUE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_XML_KEY_VALUE_TYPE_CUSTOM: u32 = 4;
pub const CRYPT_XML_KEY_VALUE_TYPE_DSA: u32 = 1;
pub const CRYPT_XML_KEY_VALUE_TYPE_ECDSA: u32 = 3;
pub const CRYPT_XML_KEY_VALUE_TYPE_RSA: u32 = 2;
#[repr(C)]
#[cfg(feature = "wincrypt")]
#[derive(Clone, Copy)]
pub struct CRYPT_XML_OBJECT {
    pub cbSize: u32,
    pub hObject: HCRYPTXML,
    pub wszId: windows_sys::core::PCWSTR,
    pub wszMimeType: windows_sys::core::PCWSTR,
    pub wszEncoding: windows_sys::core::PCWSTR,
    pub Manifest: CRYPT_XML_REFERENCES,
    pub Encoded: CRYPT_XML_BLOB,
}
#[cfg(feature = "wincrypt")]
impl Default for CRYPT_XML_OBJECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_XML_OBJECTS_MAX: u32 = 256;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPT_XML_PROPERTY {
    pub dwPropId: CRYPT_XML_PROPERTY_ID,
    pub pvValue: *const core::ffi::c_void,
    pub cbValue: u32,
}
impl Default for CRYPT_XML_PROPERTY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_XML_PROPERTY_DOC_DECLARATION: CRYPT_XML_PROPERTY_ID = 4;
pub type CRYPT_XML_PROPERTY_ID = i32;
pub const CRYPT_XML_PROPERTY_MAX_HEAP_SIZE: CRYPT_XML_PROPERTY_ID = 1;
pub const CRYPT_XML_PROPERTY_MAX_SIGNATURES: CRYPT_XML_PROPERTY_ID = 3;
pub const CRYPT_XML_PROPERTY_SIGNATURE_LOCATION: CRYPT_XML_PROPERTY_ID = 2;
pub const CRYPT_XML_PROPERTY_XML_OUTPUT_CHARSET: CRYPT_XML_PROPERTY_ID = 5;
#[repr(C)]
#[cfg(feature = "wincrypt")]
#[derive(Clone, Copy)]
pub struct CRYPT_XML_REFERENCE {
    pub cbSize: u32,
    pub hReference: HCRYPTXML,
    pub wszId: windows_sys::core::PCWSTR,
    pub wszUri: windows_sys::core::PCWSTR,
    pub wszType: windows_sys::core::PCWSTR,
    pub DigestMethod: CRYPT_XML_ALGORITHM,
    pub DigestValue: super::wincrypt::CRYPT_DATA_BLOB,
    pub cTransform: u32,
    pub rgTransform: *mut CRYPT_XML_ALGORITHM,
}
#[cfg(feature = "wincrypt")]
impl Default for CRYPT_XML_REFERENCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "wincrypt")]
#[derive(Clone, Copy)]
pub struct CRYPT_XML_REFERENCES {
    pub cReference: u32,
    pub rgpReference: *mut PCRYPT_XML_REFERENCE,
}
#[cfg(feature = "wincrypt")]
impl Default for CRYPT_XML_REFERENCES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_XML_REFERENCES_MAX: u32 = 32760;
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "wincrypt"))]
#[derive(Clone, Copy)]
pub struct CRYPT_XML_SIGNATURE {
    pub cbSize: u32,
    pub hSignature: HCRYPTXML,
    pub wszId: windows_sys::core::PCWSTR,
    pub SignedInfo: CRYPT_XML_SIGNED_INFO,
    pub SignatureValue: super::wincrypt::CRYPT_DATA_BLOB,
    pub pKeyInfo: *mut CRYPT_XML_KEY_INFO,
    pub cObject: u32,
    pub rgpObject: *mut PCRYPT_XML_OBJECT,
}
#[cfg(all(feature = "bcrypt", feature = "wincrypt"))]
impl Default for CRYPT_XML_SIGNATURE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_XML_SIGNATURES_MAX: u32 = 16;
pub const CRYPT_XML_SIGNATURE_VALUE_MAX: u32 = 2048;
#[repr(C)]
#[cfg(feature = "wincrypt")]
#[derive(Clone, Copy)]
pub struct CRYPT_XML_SIGNED_INFO {
    pub cbSize: u32,
    pub wszId: windows_sys::core::PCWSTR,
    pub Canonicalization: CRYPT_XML_ALGORITHM,
    pub SignatureMethod: CRYPT_XML_ALGORITHM,
    pub cReference: u32,
    pub rgpReference: *mut PCRYPT_XML_REFERENCE,
    pub Encoded: CRYPT_XML_BLOB,
}
#[cfg(feature = "wincrypt")]
impl Default for CRYPT_XML_SIGNED_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_XML_SIGN_ADD_KEYVALUE: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CRYPT_XML_STATUS {
    pub cbSize: u32,
    pub dwErrorStatus: u32,
    pub dwInfoStatus: u32,
}
pub const CRYPT_XML_STATUS_DIGESTING: u32 = 4;
pub const CRYPT_XML_STATUS_DIGEST_VALID: u32 = 8;
pub const CRYPT_XML_STATUS_ERROR_DIGEST_INVALID: u32 = 2;
pub const CRYPT_XML_STATUS_ERROR_KEYINFO_NOT_PARSED: u32 = 131072;
pub const CRYPT_XML_STATUS_ERROR_NOT_RESOLVED: u32 = 1;
pub const CRYPT_XML_STATUS_ERROR_NOT_SUPPORTED_ALGORITHM: u32 = 4;
pub const CRYPT_XML_STATUS_ERROR_NOT_SUPPORTED_TRANSFORM: u32 = 8;
pub const CRYPT_XML_STATUS_ERROR_SIGNATURE_INVALID: u32 = 65536;
pub const CRYPT_XML_STATUS_INTERNAL_REFERENCE: u32 = 1;
pub const CRYPT_XML_STATUS_KEY_AVAILABLE: u32 = 2;
pub const CRYPT_XML_STATUS_NO_ERROR: u32 = 0;
pub const CRYPT_XML_STATUS_OPENED_TO_ENCODE: u32 = 2147483648;
pub const CRYPT_XML_STATUS_SIGNATURE_VALID: u32 = 65536;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPT_XML_TRANSFORM_CHAIN_CONFIG {
    pub cbSize: u32,
    pub cTransformInfo: u32,
    pub rgpTransformInfo: *mut PCRYPT_XML_TRANSFORM_INFO,
}
impl Default for CRYPT_XML_TRANSFORM_CHAIN_CONFIG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPT_XML_TRANSFORM_INFO {
    pub cbSize: u32,
    pub wszAlgorithm: windows_sys::core::PCWSTR,
    pub cbBufferSize: u32,
    pub dwFlags: u32,
    pub pfnCreateTransform: PFN_CRYPT_XML_CREATE_TRANSFORM,
}
impl Default for CRYPT_XML_TRANSFORM_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_XML_TRANSFORM_MAX: u32 = 16;
pub const CRYPT_XML_TRANSFORM_ON_NODESET: u32 = 2;
pub const CRYPT_XML_TRANSFORM_ON_STREAM: u32 = 1;
pub const CRYPT_XML_TRANSFORM_URI_QUERY_STRING: u32 = 4;
pub const CRYPT_XML_URI_MAX: u32 = 8192;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPT_XML_X509DATA {
    pub cX509Data: u32,
    pub rgX509Data: *mut CRYPT_XML_X509DATA_ITEM,
}
impl Default for CRYPT_XML_X509DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRYPT_XML_X509DATA_ITEM {
    pub dwType: u32,
    pub Anonymous: CRYPT_XML_X509DATA_ITEM_0,
}
impl Default for CRYPT_XML_X509DATA_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CRYPT_XML_X509DATA_ITEM_0 {
    pub IssuerSerial: CRYPT_XML_ISSUER_SERIAL,
    pub SKI: CRYPT_XML_DATA_BLOB,
    pub wszSubjectName: windows_sys::core::PCWSTR,
    pub Certificate: CRYPT_XML_DATA_BLOB,
    pub CRL: CRYPT_XML_DATA_BLOB,
    pub Custom: CRYPT_XML_BLOB,
}
impl Default for CRYPT_XML_X509DATA_ITEM_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CRYPT_XML_X509DATA_TYPE_CERTIFICATE: u32 = 4;
pub const CRYPT_XML_X509DATA_TYPE_CRL: u32 = 5;
pub const CRYPT_XML_X509DATA_TYPE_CUSTOM: u32 = 6;
pub const CRYPT_XML_X509DATA_TYPE_ISSUER_SERIAL: u32 = 1;
pub const CRYPT_XML_X509DATA_TYPE_SKI: u32 = 2;
pub const CRYPT_XML_X509DATA_TYPE_SUBJECT_NAME: u32 = 3;
pub type CryptXmlDllCloseDigest = Option<unsafe extern "system" fn(hdigest: CRYPT_XML_DIGEST) -> windows_sys::core::HRESULT>;
pub type CryptXmlDllCreateDigest = Option<unsafe extern "system" fn(pdigestmethod: *const CRYPT_XML_ALGORITHM, pcbsize: *mut u32, phdigest: *mut CRYPT_XML_DIGEST) -> windows_sys::core::HRESULT>;
#[cfg(feature = "bcrypt")]
pub type CryptXmlDllCreateKey = Option<unsafe extern "system" fn(pencoded: *const CRYPT_XML_BLOB, phkey: *mut super::bcrypt::BCRYPT_KEY_HANDLE) -> windows_sys::core::HRESULT>;
pub type CryptXmlDllDigestData = Option<unsafe extern "system" fn(hdigest: CRYPT_XML_DIGEST, pbdata: *const u8, cbdata: u32) -> windows_sys::core::HRESULT>;
pub type CryptXmlDllEncodeAlgorithm = Option<unsafe extern "system" fn(palginfo: *const CRYPT_XML_ALGORITHM_INFO, dwcharset: CRYPT_XML_CHARSET, pvcallbackstate: *mut core::ffi::c_void, pfnwrite: PFN_CRYPT_XML_WRITE_CALLBACK) -> windows_sys::core::HRESULT>;
#[cfg(feature = "ncrypt")]
pub type CryptXmlDllEncodeKeyValue = Option<unsafe extern "system" fn(hkey: super::ncrypt::NCRYPT_KEY_HANDLE, dwcharset: CRYPT_XML_CHARSET, pvcallbackstate: *mut core::ffi::c_void, pfnwrite: PFN_CRYPT_XML_WRITE_CALLBACK) -> windows_sys::core::HRESULT>;
pub type CryptXmlDllFinalizeDigest = Option<unsafe extern "system" fn(hdigest: CRYPT_XML_DIGEST, pbdigest: *mut u8, cbdigest: u32) -> windows_sys::core::HRESULT>;
pub type CryptXmlDllGetAlgorithmInfo = Option<unsafe extern "system" fn(pxmlalgorithm: *const CRYPT_XML_ALGORITHM, ppalginfo: *mut *mut CRYPT_XML_ALGORITHM_INFO) -> windows_sys::core::HRESULT>;
#[cfg(all(feature = "bcrypt", feature = "wincrypt"))]
pub type CryptXmlDllGetInterface = Option<unsafe extern "system" fn(dwflags: u32, pmethod: *const CRYPT_XML_ALGORITHM_INFO, pinterface: *mut CRYPT_XML_CRYPTOGRAPHIC_INTERFACE) -> windows_sys::core::HRESULT>;
#[cfg(feature = "wincrypt")]
pub type CryptXmlDllSignData = Option<unsafe extern "system" fn(psignaturemethod: *const CRYPT_XML_ALGORITHM, hcryptprovorncryptkey: super::wincrypt::HCRYPTPROV_OR_NCRYPT_KEY_HANDLE, dwkeyspec: u32, pbinput: *const u8, cbinput: u32, pboutput: *mut u8, cboutput: u32, pcbresult: *mut u32) -> windows_sys::core::HRESULT>;
#[cfg(feature = "bcrypt")]
pub type CryptXmlDllVerifySignature = Option<unsafe extern "system" fn(psignaturemethod: *const CRYPT_XML_ALGORITHM, hkey: super::bcrypt::BCRYPT_KEY_HANDLE, pbinput: *const u8, cbinput: u32, pbsignature: *const u8, cbsignature: u32) -> windows_sys::core::HRESULT>;
pub type HCRYPTXML = *mut core::ffi::c_void;
pub type PCRYPT_XML_ALGORITHM = *mut CRYPT_XML_ALGORITHM;
pub type PCRYPT_XML_ALGORITHM_INFO = *mut CRYPT_XML_ALGORITHM_INFO;
pub type PCRYPT_XML_BLOB = *mut CRYPT_XML_BLOB;
#[cfg(all(feature = "bcrypt", feature = "wincrypt"))]
pub type PCRYPT_XML_CRYPTOGRAPHIC_INTERFACE = *mut CRYPT_XML_CRYPTOGRAPHIC_INTERFACE;
#[cfg(all(feature = "bcrypt", feature = "wincrypt"))]
pub type PCRYPT_XML_CRYPTO_PROVIDER = *mut CRYPT_XML_CRYPTOGRAPHIC_INTERFACE;
pub type PCRYPT_XML_DATA_BLOB = *mut CRYPT_XML_DATA_BLOB;
pub type PCRYPT_XML_DATA_PROVIDER = *mut CRYPT_XML_DATA_PROVIDER;
#[cfg(all(feature = "bcrypt", feature = "wincrypt"))]
pub type PCRYPT_XML_DOC_CTXT = *mut CRYPT_XML_DOC_CTXT;
#[cfg(feature = "bcrypt")]
pub type PCRYPT_XML_KEY_INFO = *mut CRYPT_XML_KEY_INFO;
#[cfg(feature = "wincrypt")]
pub type PCRYPT_XML_OBJECT = *mut CRYPT_XML_OBJECT;
pub type PCRYPT_XML_PROPERTY = *mut CRYPT_XML_PROPERTY;
#[cfg(feature = "wincrypt")]
pub type PCRYPT_XML_REFERENCE = *mut CRYPT_XML_REFERENCE;
#[cfg(feature = "wincrypt")]
pub type PCRYPT_XML_REFERENCES = *mut CRYPT_XML_REFERENCES;
#[cfg(all(feature = "bcrypt", feature = "wincrypt"))]
pub type PCRYPT_XML_SIGNATURE = *mut CRYPT_XML_SIGNATURE;
#[cfg(feature = "wincrypt")]
pub type PCRYPT_XML_SIGNED_INFO = *mut CRYPT_XML_SIGNED_INFO;
pub type PCRYPT_XML_STATUS = *mut CRYPT_XML_STATUS;
pub type PCRYPT_XML_TRANSFORM_CHAIN_CONFIG = *mut CRYPT_XML_TRANSFORM_CHAIN_CONFIG;
pub type PCRYPT_XML_TRANSFORM_INFO = *mut CRYPT_XML_TRANSFORM_INFO;
pub type PFN_CRYPT_XML_CREATE_TRANSFORM = Option<unsafe extern "system" fn(ptransform: *const CRYPT_XML_ALGORITHM, pproviderin: *const CRYPT_XML_DATA_PROVIDER, pproviderout: *mut CRYPT_XML_DATA_PROVIDER) -> windows_sys::core::HRESULT>;
pub type PFN_CRYPT_XML_DATA_PROVIDER_CLOSE = Option<unsafe extern "system" fn(pvcallbackstate: *mut core::ffi::c_void) -> windows_sys::core::HRESULT>;
pub type PFN_CRYPT_XML_DATA_PROVIDER_READ = Option<unsafe extern "system" fn(pvcallbackstate: *mut core::ffi::c_void, pbdata: *mut u8, cbdata: u32, pcbread: *mut u32) -> windows_sys::core::HRESULT>;
pub type PFN_CRYPT_XML_ENUM_ALG_INFO = Option<unsafe extern "system" fn(pinfo: *const CRYPT_XML_ALGORITHM_INFO, pvarg: *mut core::ffi::c_void) -> windows_sys::core::BOOL>;
pub type PFN_CRYPT_XML_WRITE_CALLBACK = Option<unsafe extern "system" fn(pvcallbackstate: *mut core::ffi::c_void, pbdata: *const u8, cbdata: u32) -> windows_sys::core::HRESULT>;
pub const wszURI_CANONICALIZATION_C14N: windows_sys::core::PCWSTR = windows_sys::core::w!("http://www.w3.org/TR/2001/REC-xml-c14n-20010315");
pub const wszURI_CANONICALIZATION_C14NC: windows_sys::core::PCWSTR = windows_sys::core::w!("http://www.w3.org/TR/2001/REC-xml-c14n-20010315#WithComments");
pub const wszURI_CANONICALIZATION_EXSLUSIVE_C14N: windows_sys::core::PCWSTR = windows_sys::core::w!("http://www.w3.org/2001/10/xml-exc-c14n#");
pub const wszURI_CANONICALIZATION_EXSLUSIVE_C14NC: windows_sys::core::PCWSTR = windows_sys::core::w!("http://www.w3.org/2001/10/xml-exc-c14n#WithComments");
pub const wszURI_TRANSFORM_XPATH: windows_sys::core::PCWSTR = windows_sys::core::w!("http://www.w3.org/TR/1999/REC-xpath-19991116");
pub const wszURI_XMLNS_DIGSIG_BASE64: windows_sys::core::PCWSTR = windows_sys::core::w!("http://www.w3.org/2000/09/xmldsig#base64");
pub const wszURI_XMLNS_DIGSIG_DSA_SHA1: windows_sys::core::PCWSTR = windows_sys::core::w!("http://www.w3.org/2000/09/xmldsig#dsa-sha1");
pub const wszURI_XMLNS_DIGSIG_ECDSA_SHA1: windows_sys::core::PCWSTR = windows_sys::core::w!("http://www.w3.org/2001/04/xmldsig-more#ecdsa-sha1");
pub const wszURI_XMLNS_DIGSIG_ECDSA_SHA256: windows_sys::core::PCWSTR = windows_sys::core::w!("http://www.w3.org/2001/04/xmldsig-more#ecdsa-sha256");
pub const wszURI_XMLNS_DIGSIG_ECDSA_SHA384: windows_sys::core::PCWSTR = windows_sys::core::w!("http://www.w3.org/2001/04/xmldsig-more#ecdsa-sha384");
pub const wszURI_XMLNS_DIGSIG_ECDSA_SHA512: windows_sys::core::PCWSTR = windows_sys::core::w!("http://www.w3.org/2001/04/xmldsig-more#ecdsa-sha512");
pub const wszURI_XMLNS_DIGSIG_HMAC_SHA1: windows_sys::core::PCWSTR = windows_sys::core::w!("http://www.w3.org/2000/09/xmldsig#hmac-sha1");
pub const wszURI_XMLNS_DIGSIG_HMAC_SHA256: windows_sys::core::PCWSTR = windows_sys::core::w!("http://www.w3.org/2001/04/xmldsig-more#hmac-sha256");
pub const wszURI_XMLNS_DIGSIG_HMAC_SHA384: windows_sys::core::PCWSTR = windows_sys::core::w!("http://www.w3.org/2001/04/xmldsig-more#hmac-sha384");
pub const wszURI_XMLNS_DIGSIG_HMAC_SHA512: windows_sys::core::PCWSTR = windows_sys::core::w!("http://www.w3.org/2001/04/xmldsig-more#hmac-sha512");
pub const wszURI_XMLNS_DIGSIG_RSA_SHA1: windows_sys::core::PCWSTR = windows_sys::core::w!("http://www.w3.org/2000/09/xmldsig#rsa-sha1");
pub const wszURI_XMLNS_DIGSIG_RSA_SHA256: windows_sys::core::PCWSTR = windows_sys::core::w!("http://www.w3.org/2001/04/xmldsig-more#rsa-sha256");
pub const wszURI_XMLNS_DIGSIG_RSA_SHA384: windows_sys::core::PCWSTR = windows_sys::core::w!("http://www.w3.org/2001/04/xmldsig-more#rsa-sha384");
pub const wszURI_XMLNS_DIGSIG_RSA_SHA512: windows_sys::core::PCWSTR = windows_sys::core::w!("http://www.w3.org/2001/04/xmldsig-more#rsa-sha512");
pub const wszURI_XMLNS_DIGSIG_SHA1: windows_sys::core::PCWSTR = windows_sys::core::w!("http://www.w3.org/2000/09/xmldsig#sha1");
pub const wszURI_XMLNS_DIGSIG_SHA256: windows_sys::core::PCWSTR = windows_sys::core::w!("http://www.w3.org/2001/04/xmlenc#sha256");
pub const wszURI_XMLNS_DIGSIG_SHA384: windows_sys::core::PCWSTR = windows_sys::core::w!("http://www.w3.org/2001/04/xmldsig-more#sha384");
pub const wszURI_XMLNS_DIGSIG_SHA512: windows_sys::core::PCWSTR = windows_sys::core::w!("http://www.w3.org/2001/04/xmlenc#sha512");
pub const wszURI_XMLNS_TRANSFORM_BASE64: windows_sys::core::PCWSTR = windows_sys::core::w!("http://www.w3.org/2000/09/xmldsig#base64");
pub const wszURI_XMLNS_TRANSFORM_ENVELOPED: windows_sys::core::PCWSTR = windows_sys::core::w!("http://www.w3.org/2000/09/xmldsig#enveloped-signature");
pub const wszXMLNS_DIGSIG: windows_sys::core::PCWSTR = windows_sys::core::w!("http://www.w3.org/2000/09/xmldsig#");
pub const wszXMLNS_DIGSIG_Id: windows_sys::core::PCWSTR = windows_sys::core::w!("Id");
pub const wszXMLNS_DIGSIG_SignatureProperties: windows_sys::core::PCWSTR = windows_sys::core::w!("http://www.w3.org/2000/09/xmldsig#SignatureProperties");
