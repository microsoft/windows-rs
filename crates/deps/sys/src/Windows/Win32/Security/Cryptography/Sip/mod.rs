#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSIPAddProvider(psnewprov: *mut SIP_ADD_NEWPROVIDER) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
    pub fn CryptSIPCreateIndirectData(psubjectinfo: *mut SIP_SUBJECTINFO, pcbindirectdata: *mut u32, pindirectdata: *mut SIP_INDIRECT_DATA) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
    pub fn CryptSIPGetCaps(psubjinfo: *const SIP_SUBJECTINFO, pcaps: *mut SIP_CAP_SET_V3) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
    pub fn CryptSIPGetSealedDigest(psubjectinfo: *const SIP_SUBJECTINFO, psig: *const u8, dwsig: u32, pbdigest: *mut u8, pcbdigest: *mut u32) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
    pub fn CryptSIPGetSignedDataMsg(psubjectinfo: *mut SIP_SUBJECTINFO, pdwencodingtype: *mut super::CERT_QUERY_ENCODING_TYPE, dwindex: u32, pcbsigneddatamsg: *mut u32, pbsigneddatamsg: *mut u8) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
    pub fn CryptSIPLoad(pgsubject: *const ::windows_sys::core::GUID, dwflags: u32, psipdispatch: *mut SIP_DISPATCH_INFO) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
    pub fn CryptSIPPutSignedDataMsg(psubjectinfo: *mut SIP_SUBJECTINFO, dwencodingtype: super::CERT_QUERY_ENCODING_TYPE, pdwindex: *mut u32, cbsigneddatamsg: u32, pbsigneddatamsg: *mut u8) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSIPRemoveProvider(pgprov: *mut ::windows_sys::core::GUID) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
    pub fn CryptSIPRemoveSignedDataMsg(psubjectinfo: *mut SIP_SUBJECTINFO, dwindex: u32) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSIPRetrieveSubjectGuid(filename: super::super::super::Foundation::PWSTR, hfilein: super::super::super::Foundation::HANDLE, pgsubject: *mut ::windows_sys::core::GUID) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSIPRetrieveSubjectGuidForCatalogFile(filename: super::super::super::Foundation::PWSTR, hfilein: super::super::super::Foundation::HANDLE, pgsubject: *mut ::windows_sys::core::GUID) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
    pub fn CryptSIPVerifyIndirectData(psubjectinfo: *mut SIP_SUBJECTINFO, pindirectdata: *mut SIP_INDIRECT_DATA) -> super::super::super::Foundation::BOOL;
}
pub const MSSIP_FLAGS_MULTI_HASH: u32 = 262144u32;
pub const MSSIP_FLAGS_PROHIBIT_RESIZE_ON_CREATE: u32 = 65536u32;
pub const MSSIP_FLAGS_USE_CATALOG: u32 = 131072u32;
#[repr(C)]
pub struct MS_ADDINFO_BLOB {
    pub cbStruct: u32,
    pub cbMemObject: u32,
    pub pbMemObject: *mut u8,
    pub cbMemSignedMsg: u32,
    pub pbMemSignedMsg: *mut u8,
}
impl ::core::marker::Copy for MS_ADDINFO_BLOB {}
impl ::core::clone::Clone for MS_ADDINFO_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
pub struct MS_ADDINFO_CATALOGMEMBER {
    pub cbStruct: u32,
    pub pStore: *mut super::Catalog::CRYPTCATSTORE,
    pub pMember: *mut super::Catalog::CRYPTCATMEMBER,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
impl ::core::marker::Copy for MS_ADDINFO_CATALOGMEMBER {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
impl ::core::clone::Clone for MS_ADDINFO_CATALOGMEMBER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct MS_ADDINFO_FLAT {
    pub cbStruct: u32,
    pub pIndirectData: *mut SIP_INDIRECT_DATA,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MS_ADDINFO_FLAT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MS_ADDINFO_FLAT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SIP_ADD_NEWPROVIDER {
    pub cbStruct: u32,
    pub pgSubject: *mut ::windows_sys::core::GUID,
    pub pwszDLLFileName: super::super::super::Foundation::PWSTR,
    pub pwszMagicNumber: super::super::super::Foundation::PWSTR,
    pub pwszIsFunctionName: super::super::super::Foundation::PWSTR,
    pub pwszGetFuncName: super::super::super::Foundation::PWSTR,
    pub pwszPutFuncName: super::super::super::Foundation::PWSTR,
    pub pwszCreateFuncName: super::super::super::Foundation::PWSTR,
    pub pwszVerifyFuncName: super::super::super::Foundation::PWSTR,
    pub pwszRemoveFuncName: super::super::super::Foundation::PWSTR,
    pub pwszIsFunctionNameFmt2: super::super::super::Foundation::PWSTR,
    pub pwszGetCapFuncName: super::super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SIP_ADD_NEWPROVIDER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SIP_ADD_NEWPROVIDER {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SIP_CAP_FLAG_SEALING: u32 = 1u32;
pub const SIP_CAP_SET_CUR_VER: u32 = 3u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SIP_CAP_SET_V2 {
    pub cbSize: u32,
    pub dwVersion: u32,
    pub isMultiSign: super::super::super::Foundation::BOOL,
    pub dwReserved: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SIP_CAP_SET_V2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SIP_CAP_SET_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SIP_CAP_SET_V3 {
    pub cbSize: u32,
    pub dwVersion: u32,
    pub isMultiSign: super::super::super::Foundation::BOOL,
    pub Anonymous: SIP_CAP_SET_V3_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SIP_CAP_SET_V3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SIP_CAP_SET_V3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union SIP_CAP_SET_V3_0 {
    pub dwFlags: u32,
    pub dwReserved: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SIP_CAP_SET_V3_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SIP_CAP_SET_V3_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SIP_CAP_SET_VERSION_2: u32 = 2u32;
pub const SIP_CAP_SET_VERSION_3: u32 = 3u32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
pub struct SIP_DISPATCH_INFO {
    pub cbSize: u32,
    pub hSIP: super::super::super::Foundation::HANDLE,
    pub pfGet: pCryptSIPGetSignedDataMsg,
    pub pfPut: pCryptSIPPutSignedDataMsg,
    pub pfCreate: pCryptSIPCreateIndirectData,
    pub pfVerify: pCryptSIPVerifyIndirectData,
    pub pfRemove: pCryptSIPRemoveSignedDataMsg,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
impl ::core::marker::Copy for SIP_DISPATCH_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
impl ::core::clone::Clone for SIP_DISPATCH_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SIP_INDIRECT_DATA {
    pub Data: super::CRYPT_ATTRIBUTE_TYPE_VALUE,
    pub DigestAlgorithm: super::CRYPT_ALGORITHM_IDENTIFIER,
    pub Digest: super::CRYPTOAPI_BLOB,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SIP_INDIRECT_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SIP_INDIRECT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SIP_MAX_MAGIC_NUMBER: u32 = 4u32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
pub struct SIP_SUBJECTINFO {
    pub cbSize: u32,
    pub pgSubjectType: *mut ::windows_sys::core::GUID,
    pub hFile: super::super::super::Foundation::HANDLE,
    pub pwsFileName: super::super::super::Foundation::PWSTR,
    pub pwsDisplayName: super::super::super::Foundation::PWSTR,
    pub dwReserved1: u32,
    pub dwIntVersion: u32,
    pub hProv: usize,
    pub DigestAlgorithm: super::CRYPT_ALGORITHM_IDENTIFIER,
    pub dwFlags: u32,
    pub dwEncodingType: u32,
    pub dwReserved2: u32,
    pub fdwCAPISettings: u32,
    pub fdwSecuritySettings: u32,
    pub dwIndex: u32,
    pub dwUnionChoice: u32,
    pub Anonymous: SIP_SUBJECTINFO_0,
    pub pClientData: *mut ::core::ffi::c_void,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
impl ::core::marker::Copy for SIP_SUBJECTINFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
impl ::core::clone::Clone for SIP_SUBJECTINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
pub union SIP_SUBJECTINFO_0 {
    pub psFlat: *mut MS_ADDINFO_FLAT,
    pub psCatMember: *mut MS_ADDINFO_CATALOGMEMBER,
    pub psBlob: *mut MS_ADDINFO_BLOB,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
impl ::core::marker::Copy for SIP_SUBJECTINFO_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
impl ::core::clone::Clone for SIP_SUBJECTINFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SPC_DIGEST_GENERATE_FLAG: u32 = 512u32;
pub const SPC_DIGEST_SIGN_EX_FLAG: u32 = 16384u32;
pub const SPC_DIGEST_SIGN_FLAG: u32 = 1024u32;
pub const SPC_EXC_PE_PAGE_HASHES_FLAG: u32 = 16u32;
pub const SPC_INC_PE_DEBUG_INFO_FLAG: u32 = 64u32;
pub const SPC_INC_PE_IMPORT_ADDR_TABLE_FLAG: u32 = 32u32;
pub const SPC_INC_PE_PAGE_HASHES_FLAG: u32 = 256u32;
pub const SPC_INC_PE_RESOURCES_FLAG: u32 = 128u32;
pub const SPC_MARKER_CHECK_CURRENTLY_SUPPORTED_FLAGS: u32 = 1u32;
pub const SPC_MARKER_CHECK_SKIP_SIP_INDIRECT_DATA_FLAG: u32 = 1u32;
pub const SPC_RELAXED_PE_MARKER_CHECK: u32 = 2048u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
pub type pCryptSIPCreateIndirectData = unsafe extern "system" fn(psubjectinfo: *mut SIP_SUBJECTINFO, pcbindirectdata: *mut u32, pindirectdata: *mut SIP_INDIRECT_DATA) -> super::super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
pub type pCryptSIPGetCaps = unsafe extern "system" fn(psubjinfo: *const SIP_SUBJECTINFO, pcaps: *mut SIP_CAP_SET_V3) -> super::super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
pub type pCryptSIPGetSealedDigest = unsafe extern "system" fn(psubjectinfo: *const SIP_SUBJECTINFO, psig: *const u8, dwsig: u32, pbdigest: *mut u8, pcbdigest: *mut u32) -> super::super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
pub type pCryptSIPGetSignedDataMsg = unsafe extern "system" fn(psubjectinfo: *mut SIP_SUBJECTINFO, pdwencodingtype: *mut u32, dwindex: u32, pcbsigneddatamsg: *mut u32, pbsigneddatamsg: *mut u8) -> super::super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
pub type pCryptSIPPutSignedDataMsg = unsafe extern "system" fn(psubjectinfo: *mut SIP_SUBJECTINFO, dwencodingtype: u32, pdwindex: *mut u32, cbsigneddatamsg: u32, pbsigneddatamsg: *mut u8) -> super::super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
pub type pCryptSIPRemoveSignedDataMsg = unsafe extern "system" fn(psubjectinfo: *mut SIP_SUBJECTINFO, dwindex: u32) -> super::super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
pub type pCryptSIPVerifyIndirectData = unsafe extern "system" fn(psubjectinfo: *mut SIP_SUBJECTINFO, pindirectdata: *mut SIP_INDIRECT_DATA) -> super::super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type pfnIsFileSupported = unsafe extern "system" fn(hfile: super::super::super::Foundation::HANDLE, pgsubject: *mut ::windows_sys::core::GUID) -> super::super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type pfnIsFileSupportedName = unsafe extern "system" fn(pwszfilename: super::super::super::Foundation::PWSTR, pgsubject: *mut ::windows_sys::core::GUID) -> super::super::super::Foundation::BOOL;
