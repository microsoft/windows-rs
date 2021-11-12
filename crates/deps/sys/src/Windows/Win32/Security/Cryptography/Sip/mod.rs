#![allow(non_snake_case, non_camel_case_types)]
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
pub struct MS_ADDINFO_BLOB(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
#[repr(C)]
pub struct MS_ADDINFO_CATALOGMEMBER(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct MS_ADDINFO_FLAT(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SIP_ADD_NEWPROVIDER(i32);
pub const SIP_CAP_FLAG_SEALING: u32 = 1u32;
pub const SIP_CAP_SET_CUR_VER: u32 = 3u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SIP_CAP_SET_V2(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SIP_CAP_SET_V3(i32);
pub const SIP_CAP_SET_VERSION_2: u32 = 2u32;
pub const SIP_CAP_SET_VERSION_3: u32 = 3u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
#[repr(C)]
pub struct SIP_DISPATCH_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SIP_INDIRECT_DATA(i32);
pub const SIP_MAX_MAGIC_NUMBER: u32 = 4u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography_Catalog"))]
#[repr(C)]
pub struct SIP_SUBJECTINFO(i32);
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
#[repr(C)]
pub struct pCryptSIPCreateIndirectData(i32);
#[repr(C)]
pub struct pCryptSIPGetCaps(i32);
#[repr(C)]
pub struct pCryptSIPGetSealedDigest(i32);
#[repr(C)]
pub struct pCryptSIPGetSignedDataMsg(i32);
#[repr(C)]
pub struct pCryptSIPPutSignedDataMsg(i32);
#[repr(C)]
pub struct pCryptSIPRemoveSignedDataMsg(i32);
#[repr(C)]
pub struct pCryptSIPVerifyIndirectData(i32);
#[repr(C)]
pub struct pfnIsFileSupported(i32);
#[repr(C)]
pub struct pfnIsFileSupportedName(i32);
