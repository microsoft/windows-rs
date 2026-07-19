windows_link::link!("crypt32.dll" "system" fn CryptSIPAddProvider(psnewprov : *mut SIP_ADD_NEWPROVIDER) -> windows_sys::core::BOOL);
#[cfg(all(feature = "mscat", feature = "wincrypt", feature = "winnt"))]
windows_link::link!("crypt32.dll" "system" fn CryptSIPCreateIndirectData(psubjectinfo : *mut SIP_SUBJECTINFO, pcbindirectdata : *mut u32, pindirectdata : *mut SIP_INDIRECT_DATA) -> windows_sys::core::BOOL);
#[cfg(all(feature = "mscat", feature = "wincrypt", feature = "winnt"))]
windows_link::link!("crypt32.dll" "system" fn CryptSIPGetCaps(psubjinfo : *const SIP_SUBJECTINFO, pcaps : *mut SIP_CAP_SET) -> windows_sys::core::BOOL);
#[cfg(all(feature = "mscat", feature = "wincrypt", feature = "winnt"))]
windows_link::link!("crypt32.dll" "system" fn CryptSIPGetSealedDigest(psubjectinfo : *const SIP_SUBJECTINFO, psig : *const u8, dwsig : u32, pbdigest : *mut u8, pcbdigest : *mut u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "mscat", feature = "wincrypt", feature = "winnt"))]
windows_link::link!("crypt32.dll" "system" fn CryptSIPGetSignedDataMsg(psubjectinfo : *mut SIP_SUBJECTINFO, pdwencodingtype : *mut u32, dwindex : u32, pcbsigneddatamsg : *mut u32, pbsigneddatamsg : *mut u8) -> windows_sys::core::BOOL);
#[cfg(all(feature = "mscat", feature = "wincrypt", feature = "winnt"))]
windows_link::link!("crypt32.dll" "system" fn CryptSIPLoad(pgsubject : *const windows_sys::core::GUID, dwflags : u32, psipdispatch : *mut SIP_DISPATCH_INFO) -> windows_sys::core::BOOL);
#[cfg(all(feature = "mscat", feature = "wincrypt", feature = "winnt"))]
windows_link::link!("crypt32.dll" "system" fn CryptSIPPutSignedDataMsg(psubjectinfo : *mut SIP_SUBJECTINFO, dwencodingtype : u32, pdwindex : *mut u32, cbsigneddatamsg : u32, pbsigneddatamsg : *mut u8) -> windows_sys::core::BOOL);
windows_link::link!("crypt32.dll" "system" fn CryptSIPRemoveProvider(pgprov : *mut windows_sys::core::GUID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "mscat", feature = "wincrypt", feature = "winnt"))]
windows_link::link!("crypt32.dll" "system" fn CryptSIPRemoveSignedDataMsg(psubjectinfo : *mut SIP_SUBJECTINFO, dwindex : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("crypt32.dll" "system" fn CryptSIPRetrieveSubjectGuid(filename : windows_sys::core::PCWSTR, hfilein : super::HANDLE, pgsubject : *mut windows_sys::core::GUID) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("crypt32.dll" "system" fn CryptSIPRetrieveSubjectGuidForCatalogFile(filename : windows_sys::core::PCWSTR, hfilein : super::HANDLE, pgsubject : *mut windows_sys::core::GUID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "mscat", feature = "wincrypt", feature = "winnt"))]
windows_link::link!("crypt32.dll" "system" fn CryptSIPVerifyIndirectData(psubjectinfo : *mut SIP_SUBJECTINFO, pindirectdata : *mut SIP_INDIRECT_DATA) -> windows_sys::core::BOOL);
#[cfg(feature = "wincrypt")]
pub type CRYPT_DIGEST_DATA = super::CRYPT_HASH_BLOB;
#[cfg(all(feature = "mscat", feature = "wincrypt", feature = "winnt"))]
pub type LPSIP_DISPATCH_INFO = *mut SIP_DISPATCH_INFO;
#[cfg(all(feature = "mscat", feature = "wincrypt", feature = "winnt"))]
pub type LPSIP_SUBJECTINFO = *mut SIP_SUBJECTINFO;
pub const MSSIP_ADDINFO_BLOB: u32 = 3;
pub const MSSIP_ADDINFO_CATMEMBER: u32 = 2;
pub const MSSIP_ADDINFO_DETACHEDSIG: u32 = 4;
pub const MSSIP_ADDINFO_FLAT: u32 = 1;
pub const MSSIP_ADDINFO_NONE: u32 = 0;
pub const MSSIP_ADDINFO_NONMSSIP: u32 = 500;
pub const MSSIP_FLAGS_MULTI_HASH: u32 = 262144;
pub const MSSIP_FLAGS_PROHIBIT_RESIZE_ON_CREATE: u32 = 65536;
pub const MSSIP_FLAGS_USE_CATALOG: u32 = 131072;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MS_ADDINFO_BLOB {
    pub cbStruct: u32,
    pub cbMemObject: u32,
    pub pbMemObject: *mut u8,
    pub cbMemSignedMsg: u32,
    pub pbMemSignedMsg: *mut u8,
}
impl Default for MS_ADDINFO_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "mscat", feature = "wincrypt", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct MS_ADDINFO_CATALOGMEMBER {
    pub cbStruct: u32,
    pub pStore: *mut super::CRYPTCATSTORE,
    pub pMember: *mut super::CRYPTCATMEMBER,
}
#[cfg(all(feature = "mscat", feature = "wincrypt", feature = "winnt"))]
impl Default for MS_ADDINFO_CATALOGMEMBER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct MS_ADDINFO_DETACHEDSIG {
    pub cbStruct: u32,
    pub hSignatureFile: super::HANDLE,
    pub cbSignatureObject: u32,
    pub pbSignatureObject: *mut u8,
}
#[cfg(feature = "winnt")]
impl Default for MS_ADDINFO_DETACHEDSIG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "wincrypt")]
#[derive(Clone, Copy)]
pub struct MS_ADDINFO_FLAT {
    pub cbStruct: u32,
    pub pIndirectData: *mut SIP_INDIRECT_DATA,
}
#[cfg(feature = "wincrypt")]
impl Default for MS_ADDINFO_FLAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PMS_ADDINFO_BLOB = *mut MS_ADDINFO_BLOB;
#[cfg(all(feature = "mscat", feature = "wincrypt", feature = "winnt"))]
pub type PMS_ADDINFO_CATALOGMEMBER = *mut MS_ADDINFO_CATALOGMEMBER;
#[cfg(feature = "winnt")]
pub type PMS_ADDINFO_DETACHEDSIG = *mut MS_ADDINFO_DETACHEDSIG;
#[cfg(feature = "wincrypt")]
pub type PMS_ADDINFO_FLAT = *mut MS_ADDINFO_FLAT;
pub type PSIP_ADD_NEWPROVIDER = *mut SIP_ADD_NEWPROVIDER;
pub type PSIP_CAP_SET = PSIP_CAP_SET_V3;
pub type PSIP_CAP_SET_V2 = *mut SIP_CAP_SET_V2;
pub type PSIP_CAP_SET_V3 = *mut SIP_CAP_SET_V3;
#[cfg(feature = "wincrypt")]
pub type PSIP_INDIRECT_DATA = *mut SIP_INDIRECT_DATA;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SIP_ADD_NEWPROVIDER {
    pub cbStruct: u32,
    pub pgSubject: *mut windows_sys::core::GUID,
    pub pwszDLLFileName: *mut u16,
    pub pwszMagicNumber: *mut u16,
    pub pwszIsFunctionName: *mut u16,
    pub pwszGetFuncName: *mut u16,
    pub pwszPutFuncName: *mut u16,
    pub pwszCreateFuncName: *mut u16,
    pub pwszVerifyFuncName: *mut u16,
    pub pwszRemoveFuncName: *mut u16,
    pub pwszIsFunctionNameFmt2: *mut u16,
    pub pwszGetCapFuncName: windows_sys::core::PWSTR,
}
impl Default for SIP_ADD_NEWPROVIDER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SIP_CAP_FLAG_SEALING: u32 = 1;
pub type SIP_CAP_SET = SIP_CAP_SET_V3;
pub const SIP_CAP_SET_CUR_VER: u32 = 3;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SIP_CAP_SET_V2 {
    pub cbSize: u32,
    pub dwVersion: u32,
    pub isMultiSign: windows_sys::core::BOOL,
    pub dwReserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SIP_CAP_SET_V3 {
    pub cbSize: u32,
    pub dwVersion: u32,
    pub isMultiSign: windows_sys::core::BOOL,
    pub Anonymous: SIP_CAP_SET_V3_0,
}
impl Default for SIP_CAP_SET_V3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union SIP_CAP_SET_V3_0 {
    pub dwFlags: u32,
    pub dwReserved: u32,
}
impl Default for SIP_CAP_SET_V3_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SIP_CAP_SET_VERSION_2: u32 = 2;
pub const SIP_CAP_SET_VERSION_3: u32 = 3;
#[repr(C)]
#[cfg(all(feature = "mscat", feature = "wincrypt", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct SIP_DISPATCH_INFO {
    pub cbSize: u32,
    pub hSIP: super::HANDLE,
    pub pfGet: pCryptSIPGetSignedDataMsg,
    pub pfPut: pCryptSIPPutSignedDataMsg,
    pub pfCreate: pCryptSIPCreateIndirectData,
    pub pfVerify: pCryptSIPVerifyIndirectData,
    pub pfRemove: pCryptSIPRemoveSignedDataMsg,
}
#[cfg(all(feature = "mscat", feature = "wincrypt", feature = "winnt"))]
impl Default for SIP_DISPATCH_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "wincrypt")]
#[derive(Clone, Copy, Default)]
pub struct SIP_INDIRECT_DATA {
    pub Data: super::CRYPT_ATTRIBUTE_TYPE_VALUE,
    pub DigestAlgorithm: super::CRYPT_ALGORITHM_IDENTIFIER,
    pub Digest: super::CRYPT_HASH_BLOB,
}
pub const SIP_MAX_MAGIC_NUMBER: u32 = 4;
#[repr(C)]
#[cfg(all(feature = "mscat", feature = "wincrypt", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct SIP_SUBJECTINFO {
    pub cbSize: u32,
    pub pgSubjectType: *mut windows_sys::core::GUID,
    pub hFile: super::HANDLE,
    pub pwsFileName: windows_sys::core::PCWSTR,
    pub pwsDisplayName: windows_sys::core::PCWSTR,
    pub dwReserved1: u32,
    pub dwIntVersion: u32,
    pub hProv: super::HCRYPTPROV,
    pub DigestAlgorithm: super::CRYPT_ALGORITHM_IDENTIFIER,
    pub dwFlags: u32,
    pub dwEncodingType: u32,
    pub dwReserved2: u32,
    pub fdwCAPISettings: u32,
    pub fdwSecuritySettings: u32,
    pub dwIndex: u32,
    pub dwUnionChoice: u32,
    pub Anonymous: SIP_SUBJECTINFO_0,
    pub pClientData: *mut core::ffi::c_void,
}
#[cfg(all(feature = "mscat", feature = "wincrypt", feature = "winnt"))]
impl Default for SIP_SUBJECTINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "mscat", feature = "wincrypt", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union SIP_SUBJECTINFO_0 {
    pub psFlat: *mut MS_ADDINFO_FLAT,
    pub psCatMember: *mut MS_ADDINFO_CATALOGMEMBER,
    pub psBlob: *mut MS_ADDINFO_BLOB,
    pub psDetachedSig: *mut MS_ADDINFO_DETACHEDSIG,
}
#[cfg(all(feature = "mscat", feature = "wincrypt", feature = "winnt"))]
impl Default for SIP_SUBJECTINFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SPC_DIGEST_GENERATE_FLAG: u32 = 512;
pub const SPC_DIGEST_SIGN_EX_FLAG: u32 = 16384;
pub const SPC_DIGEST_SIGN_FLAG: u32 = 1024;
pub const SPC_EXC_PE_PAGE_HASHES_FLAG: u32 = 16;
pub const SPC_INC_PE_DEBUG_INFO_FLAG: u32 = 64;
pub const SPC_INC_PE_IMPORT_ADDR_TABLE_FLAG: u32 = 32;
pub const SPC_INC_PE_PAGE_HASHES_FLAG: u32 = 256;
pub const SPC_INC_PE_RESOURCES_FLAG: u32 = 128;
pub const SPC_MARKER_CHECK_CURRENTLY_SUPPORTED_FLAGS: u32 = 1;
pub const SPC_MARKER_CHECK_SKIP_SIP_INDIRECT_DATA_FLAG: u32 = 1;
pub const SPC_RELAXED_PE_MARKER_CHECK: u32 = 2048;
#[cfg(all(feature = "mscat", feature = "wincrypt", feature = "winnt"))]
pub type pCryptSIPCreateIndirectData = Option<unsafe extern "system" fn(psubjectinfo: *mut SIP_SUBJECTINFO, pcbindirectdata: *mut u32, pindirectdata: *mut SIP_INDIRECT_DATA) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "mscat", feature = "wincrypt", feature = "winnt"))]
pub type pCryptSIPGetCaps = Option<unsafe extern "system" fn(psubjinfo: *const SIP_SUBJECTINFO, pcaps: *mut SIP_CAP_SET) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "mscat", feature = "wincrypt", feature = "winnt"))]
pub type pCryptSIPGetSealedDigest = Option<unsafe extern "system" fn(psubjectinfo: *const SIP_SUBJECTINFO, psig: *const u8, dwsig: u32, pbdigest: *mut u8, pcbdigest: *mut u32) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "mscat", feature = "wincrypt", feature = "winnt"))]
pub type pCryptSIPGetSignedDataMsg = Option<unsafe extern "system" fn(psubjectinfo: *mut SIP_SUBJECTINFO, pdwencodingtype: *mut u32, dwindex: u32, pcbsigneddatamsg: *mut u32, pbsigneddatamsg: *mut u8) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "mscat", feature = "wincrypt", feature = "winnt"))]
pub type pCryptSIPPutSignedDataMsg = Option<unsafe extern "system" fn(psubjectinfo: *mut SIP_SUBJECTINFO, dwencodingtype: u32, pdwindex: *mut u32, cbsigneddatamsg: u32, pbsigneddatamsg: *mut u8) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "mscat", feature = "wincrypt", feature = "winnt"))]
pub type pCryptSIPRemoveSignedDataMsg = Option<unsafe extern "system" fn(psubjectinfo: *mut SIP_SUBJECTINFO, dwindex: u32) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "mscat", feature = "wincrypt", feature = "winnt"))]
pub type pCryptSIPVerifyIndirectData = Option<unsafe extern "system" fn(psubjectinfo: *mut SIP_SUBJECTINFO, pindirectdata: *mut SIP_INDIRECT_DATA) -> windows_sys::core::BOOL>;
#[cfg(feature = "winnt")]
pub type pfnIsFileSupported = Option<unsafe extern "system" fn(hfile: super::HANDLE, pgsubject: *mut windows_sys::core::GUID) -> windows_sys::core::BOOL>;
pub type pfnIsFileSupportedName = Option<unsafe extern "system" fn(pwszfilename: *mut u16, pgsubject: *mut windows_sys::core::GUID) -> windows_sys::core::BOOL>;
