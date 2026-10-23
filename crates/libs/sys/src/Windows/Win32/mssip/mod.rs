#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("crypt32.dll" "system" fn CryptSIPAddProvider(psnewprov : *mut SIP_ADD_NEWPROVIDER) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "mscat", feature = "shared", feature = "wincrypt", feature = "winnt"))]
windows_link::link!("crypt32.dll" "system" fn CryptSIPCreateIndirectData(psubjectinfo : *mut SIP_SUBJECTINFO, pcbindirectdata : *mut super::DWORD, pindirectdata : *mut SIP_INDIRECT_DATA) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "mscat", feature = "shared", feature = "wincrypt", feature = "winnt"))]
windows_link::link!("crypt32.dll" "system" fn CryptSIPGetCaps(psubjinfo : *const SIP_SUBJECTINFO, pcaps : *mut SIP_CAP_SET) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "mscat", feature = "shared", feature = "wincrypt", feature = "winnt"))]
windows_link::link!("crypt32.dll" "system" fn CryptSIPGetSealedDigest(psubjectinfo : *const SIP_SUBJECTINFO, psig : *const super::BYTE, dwsig : super::DWORD, pbdigest : *mut super::BYTE, pcbdigest : *mut super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "mscat", feature = "shared", feature = "wincrypt", feature = "winnt"))]
windows_link::link!("crypt32.dll" "system" fn CryptSIPGetSignedDataMsg(psubjectinfo : *mut SIP_SUBJECTINFO, pdwencodingtype : *mut super::DWORD, dwindex : super::DWORD, pcbsigneddatamsg : *mut super::DWORD, pbsigneddatamsg : *mut super::BYTE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "mscat", feature = "shared", feature = "wincrypt", feature = "winnt"))]
windows_link::link!("crypt32.dll" "system" fn CryptSIPLoad(pgsubject : *const windows_sys::core::GUID, dwflags : super::DWORD, psipdispatch : *mut SIP_DISPATCH_INFO) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "mscat", feature = "shared", feature = "wincrypt", feature = "winnt"))]
windows_link::link!("crypt32.dll" "system" fn CryptSIPPutSignedDataMsg(psubjectinfo : *mut SIP_SUBJECTINFO, dwencodingtype : super::DWORD, pdwindex : *mut super::DWORD, cbsigneddatamsg : super::DWORD, pbsigneddatamsg : *mut super::BYTE) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CryptSIPRemoveProvider(pgprov : *mut windows_sys::core::GUID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "mscat", feature = "shared", feature = "wincrypt", feature = "winnt"))]
windows_link::link!("crypt32.dll" "system" fn CryptSIPRemoveSignedDataMsg(psubjectinfo : *mut SIP_SUBJECTINFO, dwindex : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("crypt32.dll" "system" fn CryptSIPRetrieveSubjectGuid(filename : windows_sys::core::PCWSTR, hfilein : super::HANDLE, pgsubject : *mut windows_sys::core::GUID) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("crypt32.dll" "system" fn CryptSIPRetrieveSubjectGuidForCatalogFile(filename : windows_sys::core::PCWSTR, hfilein : super::HANDLE, pgsubject : *mut windows_sys::core::GUID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "mscat", feature = "shared", feature = "wincrypt", feature = "winnt"))]
windows_link::link!("crypt32.dll" "system" fn CryptSIPVerifyIndirectData(psubjectinfo : *mut SIP_SUBJECTINFO, pindirectdata : *mut SIP_INDIRECT_DATA) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "wincrypt"))]
pub type CRYPT_DIGEST_DATA = super::_CRYPTOAPI_BLOB;
#[cfg(all(feature = "minwindef", feature = "mscat", feature = "shared", feature = "wincrypt", feature = "winnt"))]
pub type LPSIP_DISPATCH_INFO = *mut SIP_DISPATCH_INFO;
#[cfg(all(feature = "minwindef", feature = "mscat", feature = "shared", feature = "wincrypt", feature = "winnt"))]
pub type LPSIP_SUBJECTINFO = *mut SIP_SUBJECTINFO;
pub const MSSIP_ADDINFO_BLOB: i32 = 3;
pub const MSSIP_ADDINFO_CATMEMBER: i32 = 2;
pub const MSSIP_ADDINFO_DETACHEDSIG: i32 = 4;
pub const MSSIP_ADDINFO_FLAT: i32 = 1;
pub const MSSIP_ADDINFO_NONE: i32 = 0;
pub const MSSIP_ADDINFO_NONMSSIP: i32 = 500;
pub const MSSIP_FLAGS_MULTI_HASH: i32 = 262144;
pub const MSSIP_FLAGS_PROHIBIT_RESIZE_ON_CREATE: i32 = 65536;
pub const MSSIP_FLAGS_USE_CATALOG: i32 = 131072;
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MS_ADDINFO_BLOB {
    pub cbStruct: super::DWORD,
    pub cbMemObject: super::DWORD,
    pub pbMemObject: *mut super::BYTE,
    pub cbMemSignedMsg: super::DWORD,
    pub pbMemSignedMsg: *mut super::BYTE,
}
#[repr(C, packed(8))]
#[cfg(all(feature = "mscat", feature = "shared", feature = "wincrypt", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct MS_ADDINFO_CATALOGMEMBER {
    pub cbStruct: super::DWORD,
    pub pStore: *mut super::CRYPTCATSTORE,
    pub pMember: *mut super::CRYPTCATMEMBER,
}
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct MS_ADDINFO_DETACHEDSIG {
    pub cbStruct: super::DWORD,
    pub hSignatureFile: super::HANDLE,
    pub cbSignatureObject: super::DWORD,
    pub pbSignatureObject: *mut super::BYTE,
}
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "wincrypt"))]
#[derive(Clone, Copy, Default)]
pub struct MS_ADDINFO_FLAT {
    pub cbStruct: super::DWORD,
    pub pIndirectData: *mut SIP_INDIRECT_DATA,
}
#[cfg(feature = "shared")]
pub type PMS_ADDINFO_BLOB = *mut MS_ADDINFO_BLOB;
#[cfg(all(feature = "mscat", feature = "shared", feature = "wincrypt", feature = "winnt"))]
pub type PMS_ADDINFO_CATALOGMEMBER = *mut MS_ADDINFO_CATALOGMEMBER;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PMS_ADDINFO_DETACHEDSIG = *mut MS_ADDINFO_DETACHEDSIG;
#[cfg(all(feature = "shared", feature = "wincrypt"))]
pub type PMS_ADDINFO_FLAT = *mut MS_ADDINFO_FLAT;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSIP_ADD_NEWPROVIDER = *mut SIP_ADD_NEWPROVIDER;
#[cfg(feature = "shared")]
pub type PSIP_CAP_SET = *mut SIP_CAP_SET_V3;
#[cfg(feature = "shared")]
pub type PSIP_CAP_SET_V2 = *mut SIP_CAP_SET_V2;
#[cfg(feature = "shared")]
pub type PSIP_CAP_SET_V3 = *mut SIP_CAP_SET_V3;
#[cfg(all(feature = "shared", feature = "wincrypt"))]
pub type PSIP_INDIRECT_DATA = *mut SIP_INDIRECT_DATA;
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct SIP_ADD_NEWPROVIDER {
    pub cbStruct: super::DWORD,
    pub pgSubject: *mut windows_sys::core::GUID,
    pub pwszDLLFileName: *mut super::WCHAR,
    pub pwszMagicNumber: *mut super::WCHAR,
    pub pwszIsFunctionName: *mut super::WCHAR,
    pub pwszGetFuncName: *mut super::WCHAR,
    pub pwszPutFuncName: *mut super::WCHAR,
    pub pwszCreateFuncName: *mut super::WCHAR,
    pub pwszVerifyFuncName: *mut super::WCHAR,
    pub pwszRemoveFuncName: *mut super::WCHAR,
    pub pwszIsFunctionNameFmt2: *mut super::WCHAR,
    pub pwszGetCapFuncName: windows_sys::core::PWSTR,
}
pub const SIP_CAP_FLAG_SEALING: i32 = 1;
#[cfg(feature = "shared")]
pub type SIP_CAP_SET = SIP_CAP_SET_V3;
pub const SIP_CAP_SET_CUR_VER: i32 = 3;
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct SIP_CAP_SET_V2 {
    pub cbSize: super::DWORD,
    pub dwVersion: super::DWORD,
    pub isMultiSign: windows_sys::core::BOOL,
    pub dwReserved: super::DWORD,
}
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct SIP_CAP_SET_V3 {
    pub cbSize: super::DWORD,
    pub dwVersion: super::DWORD,
    pub isMultiSign: windows_sys::core::BOOL,
    pub Anonymous: SIP_CAP_SET_V3_0,
}
#[cfg(feature = "shared")]
impl Default for SIP_CAP_SET_V3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union SIP_CAP_SET_V3_0 {
    pub dwFlags: super::DWORD,
    pub dwReserved: super::DWORD,
}
#[cfg(feature = "shared")]
impl Default for SIP_CAP_SET_V3_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SIP_CAP_SET_VERSION_2: i32 = 2;
pub const SIP_CAP_SET_VERSION_3: i32 = 3;
#[repr(C, packed(8))]
#[cfg(all(feature = "minwindef", feature = "mscat", feature = "shared", feature = "wincrypt", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct SIP_DISPATCH_INFO {
    pub cbSize: super::DWORD,
    pub hSIP: super::HANDLE,
    pub pfGet: pCryptSIPGetSignedDataMsg,
    pub pfPut: pCryptSIPPutSignedDataMsg,
    pub pfCreate: pCryptSIPCreateIndirectData,
    pub pfVerify: pCryptSIPVerifyIndirectData,
    pub pfRemove: pCryptSIPRemoveSignedDataMsg,
}
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "wincrypt"))]
#[derive(Clone, Copy, Default)]
pub struct SIP_INDIRECT_DATA {
    pub Data: super::CRYPT_ATTRIBUTE_TYPE_VALUE,
    pub DigestAlgorithm: super::CRYPT_ALGORITHM_IDENTIFIER,
    pub Digest: super::CRYPT_HASH_BLOB,
}
pub const SIP_MAX_MAGIC_NUMBER: i32 = 4;
#[repr(C, packed(8))]
#[cfg(all(feature = "minwindef", feature = "mscat", feature = "shared", feature = "wincrypt", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct SIP_SUBJECTINFO {
    pub cbSize: super::DWORD,
    pub pgSubjectType: *mut windows_sys::core::GUID,
    pub hFile: super::HANDLE,
    pub pwsFileName: windows_sys::core::PCWSTR,
    pub pwsDisplayName: windows_sys::core::PCWSTR,
    pub dwReserved1: super::DWORD,
    pub dwIntVersion: super::DWORD,
    pub hProv: super::HCRYPTPROV,
    pub DigestAlgorithm: super::CRYPT_ALGORITHM_IDENTIFIER,
    pub dwFlags: super::DWORD,
    pub dwEncodingType: super::DWORD,
    pub dwReserved2: super::DWORD,
    pub fdwCAPISettings: super::DWORD,
    pub fdwSecuritySettings: super::DWORD,
    pub dwIndex: super::DWORD,
    pub dwUnionChoice: super::DWORD,
    pub Anonymous: SIP_SUBJECTINFO_0,
    pub pClientData: super::LPVOID,
}
#[cfg(all(feature = "minwindef", feature = "mscat", feature = "shared", feature = "wincrypt", feature = "winnt"))]
impl Default for SIP_SUBJECTINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(all(feature = "minwindef", feature = "mscat", feature = "shared", feature = "wincrypt", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union SIP_SUBJECTINFO_0 {
    pub psFlat: *mut MS_ADDINFO_FLAT,
    pub psCatMember: *mut MS_ADDINFO_CATALOGMEMBER,
    pub psBlob: *mut MS_ADDINFO_BLOB,
    pub psDetachedSig: *mut MS_ADDINFO_DETACHEDSIG,
}
#[cfg(all(feature = "minwindef", feature = "mscat", feature = "shared", feature = "wincrypt", feature = "winnt"))]
impl Default for SIP_SUBJECTINFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SPC_DIGEST_GENERATE_FLAG: i32 = 512;
pub const SPC_DIGEST_SIGN_EX_FLAG: i32 = 16384;
pub const SPC_DIGEST_SIGN_FLAG: i32 = 1024;
pub const SPC_EXC_PE_PAGE_HASHES_FLAG: i32 = 16;
pub const SPC_INC_PE_DEBUG_INFO_FLAG: i32 = 64;
pub const SPC_INC_PE_IMPORT_ADDR_TABLE_FLAG: i32 = 32;
pub const SPC_INC_PE_PAGE_HASHES_FLAG: i32 = 256;
pub const SPC_INC_PE_RESOURCES_FLAG: i32 = 128;
pub const SPC_MARKER_CHECK_CURRENTLY_SUPPORTED_FLAGS: i32 = 1;
pub const SPC_MARKER_CHECK_SKIP_SIP_INDIRECT_DATA_FLAG: i32 = 1;
pub const SPC_RELAXED_PE_MARKER_CHECK: i32 = 2048;
#[cfg(all(feature = "minwindef", feature = "mscat", feature = "shared", feature = "wincrypt", feature = "winnt"))]
pub type pCryptSIPCreateIndirectData = Option<unsafe extern "system" fn(psubjectinfo: *mut SIP_SUBJECTINFO, pcbindirectdata: *mut super::DWORD, pindirectdata: *mut SIP_INDIRECT_DATA) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "minwindef", feature = "mscat", feature = "shared", feature = "wincrypt", feature = "winnt"))]
pub type pCryptSIPGetCaps = Option<unsafe extern "system" fn(psubjinfo: *const SIP_SUBJECTINFO, pcaps: *mut SIP_CAP_SET) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "minwindef", feature = "mscat", feature = "shared", feature = "wincrypt", feature = "winnt"))]
pub type pCryptSIPGetSealedDigest = Option<unsafe extern "system" fn(psubjectinfo: *const SIP_SUBJECTINFO, psig: *const super::BYTE, dwsig: super::DWORD, pbdigest: *mut super::BYTE, pcbdigest: *mut super::DWORD) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "minwindef", feature = "mscat", feature = "shared", feature = "wincrypt", feature = "winnt"))]
pub type pCryptSIPGetSignedDataMsg = Option<unsafe extern "system" fn(psubjectinfo: *mut SIP_SUBJECTINFO, pdwencodingtype: *mut super::DWORD, dwindex: super::DWORD, pcbsigneddatamsg: *mut super::DWORD, pbsigneddatamsg: *mut super::BYTE) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "minwindef", feature = "mscat", feature = "shared", feature = "wincrypt", feature = "winnt"))]
pub type pCryptSIPPutSignedDataMsg = Option<unsafe extern "system" fn(psubjectinfo: *mut SIP_SUBJECTINFO, dwencodingtype: super::DWORD, pdwindex: *mut super::DWORD, cbsigneddatamsg: super::DWORD, pbsigneddatamsg: *mut super::BYTE) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "minwindef", feature = "mscat", feature = "shared", feature = "wincrypt", feature = "winnt"))]
pub type pCryptSIPRemoveSignedDataMsg = Option<unsafe extern "system" fn(psubjectinfo: *mut SIP_SUBJECTINFO, dwindex: super::DWORD) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "minwindef", feature = "mscat", feature = "shared", feature = "wincrypt", feature = "winnt"))]
pub type pCryptSIPVerifyIndirectData = Option<unsafe extern "system" fn(psubjectinfo: *mut SIP_SUBJECTINFO, pindirectdata: *mut SIP_INDIRECT_DATA) -> windows_sys::core::BOOL>;
#[cfg(feature = "winnt")]
pub type pfnIsFileSupported = Option<unsafe extern "system" fn(hfile: super::HANDLE, pgsubject: *mut windows_sys::core::GUID) -> windows_sys::core::BOOL>;
#[cfg(feature = "winnt")]
pub type pfnIsFileSupportedName = Option<unsafe extern "system" fn(pwszfilename: *mut super::WCHAR, pgsubject: *mut windows_sys::core::GUID) -> windows_sys::core::BOOL>;
