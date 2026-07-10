#[inline]
pub unsafe fn CryptSIPAddProvider(psnewprov: *mut SIP_ADD_NEWPROVIDER) -> windows_core::BOOL {
    windows_core::link!("crypt32.dll" "system" fn CryptSIPAddProvider(psnewprov : *mut SIP_ADD_NEWPROVIDER) -> windows_core::BOOL);
    unsafe { CryptSIPAddProvider(psnewprov as _) }
}
#[cfg(all(feature = "mscat", feature = "wincrypt", feature = "winnt"))]
#[inline]
pub unsafe fn CryptSIPCreateIndirectData(psubjectinfo: *mut SIP_SUBJECTINFO, pcbindirectdata: *mut u32, pindirectdata: *mut SIP_INDIRECT_DATA) -> windows_core::BOOL {
    windows_core::link!("crypt32.dll" "system" fn CryptSIPCreateIndirectData(psubjectinfo : *mut SIP_SUBJECTINFO, pcbindirectdata : *mut u32, pindirectdata : *mut SIP_INDIRECT_DATA) -> windows_core::BOOL);
    unsafe { CryptSIPCreateIndirectData(psubjectinfo as _, pcbindirectdata as _, pindirectdata as _) }
}
#[cfg(all(feature = "mscat", feature = "wincrypt", feature = "winnt"))]
#[inline]
pub unsafe fn CryptSIPGetCaps(psubjinfo: *const SIP_SUBJECTINFO, pcaps: *mut SIP_CAP_SET) -> windows_core::BOOL {
    windows_core::link!("crypt32.dll" "system" fn CryptSIPGetCaps(psubjinfo : *const SIP_SUBJECTINFO, pcaps : *mut SIP_CAP_SET) -> windows_core::BOOL);
    unsafe { CryptSIPGetCaps(psubjinfo, pcaps as _) }
}
#[cfg(all(feature = "mscat", feature = "wincrypt", feature = "winnt"))]
#[inline]
pub unsafe fn CryptSIPGetSealedDigest(psubjectinfo: *const SIP_SUBJECTINFO, psig: *const u8, dwsig: u32, pbdigest: Option<*mut u8>, pcbdigest: *mut u32) -> windows_core::BOOL {
    windows_core::link!("crypt32.dll" "system" fn CryptSIPGetSealedDigest(psubjectinfo : *const SIP_SUBJECTINFO, psig : *const u8, dwsig : u32, pbdigest : *mut u8, pcbdigest : *mut u32) -> windows_core::BOOL);
    unsafe { CryptSIPGetSealedDigest(psubjectinfo, psig, dwsig, pbdigest.unwrap_or(core::mem::zeroed()) as _, pcbdigest as _) }
}
#[cfg(all(feature = "mscat", feature = "wincrypt", feature = "winnt"))]
#[inline]
pub unsafe fn CryptSIPGetSignedDataMsg(psubjectinfo: *mut SIP_SUBJECTINFO, pdwencodingtype: *mut u32, dwindex: u32, pcbsigneddatamsg: *mut u32, pbsigneddatamsg: *mut u8) -> windows_core::BOOL {
    windows_core::link!("crypt32.dll" "system" fn CryptSIPGetSignedDataMsg(psubjectinfo : *mut SIP_SUBJECTINFO, pdwencodingtype : *mut u32, dwindex : u32, pcbsigneddatamsg : *mut u32, pbsigneddatamsg : *mut u8) -> windows_core::BOOL);
    unsafe { CryptSIPGetSignedDataMsg(psubjectinfo as _, pdwencodingtype as _, dwindex, pcbsigneddatamsg as _, pbsigneddatamsg as _) }
}
#[cfg(all(feature = "mscat", feature = "wincrypt", feature = "winnt"))]
#[inline]
pub unsafe fn CryptSIPLoad(pgsubject: *const windows_core::GUID, dwflags: u32, psipdispatch: *mut SIP_DISPATCH_INFO) -> windows_core::BOOL {
    windows_core::link!("crypt32.dll" "system" fn CryptSIPLoad(pgsubject : *const windows_core::GUID, dwflags : u32, psipdispatch : *mut SIP_DISPATCH_INFO) -> windows_core::BOOL);
    unsafe { CryptSIPLoad(pgsubject, dwflags, psipdispatch as _) }
}
#[cfg(all(feature = "mscat", feature = "wincrypt", feature = "winnt"))]
#[inline]
pub unsafe fn CryptSIPPutSignedDataMsg(psubjectinfo: *mut SIP_SUBJECTINFO, dwencodingtype: u32, pdwindex: *mut u32, cbsigneddatamsg: u32, pbsigneddatamsg: *mut u8) -> windows_core::BOOL {
    windows_core::link!("crypt32.dll" "system" fn CryptSIPPutSignedDataMsg(psubjectinfo : *mut SIP_SUBJECTINFO, dwencodingtype : u32, pdwindex : *mut u32, cbsigneddatamsg : u32, pbsigneddatamsg : *mut u8) -> windows_core::BOOL);
    unsafe { CryptSIPPutSignedDataMsg(psubjectinfo as _, dwencodingtype, pdwindex as _, cbsigneddatamsg, pbsigneddatamsg as _) }
}
#[inline]
pub unsafe fn CryptSIPRemoveProvider(pgprov: *mut windows_core::GUID) -> windows_core::BOOL {
    windows_core::link!("crypt32.dll" "system" fn CryptSIPRemoveProvider(pgprov : *mut windows_core::GUID) -> windows_core::BOOL);
    unsafe { CryptSIPRemoveProvider(pgprov as _) }
}
#[cfg(all(feature = "mscat", feature = "wincrypt", feature = "winnt"))]
#[inline]
pub unsafe fn CryptSIPRemoveSignedDataMsg(psubjectinfo: *mut SIP_SUBJECTINFO, dwindex: u32) -> windows_core::BOOL {
    windows_core::link!("crypt32.dll" "system" fn CryptSIPRemoveSignedDataMsg(psubjectinfo : *mut SIP_SUBJECTINFO, dwindex : u32) -> windows_core::BOOL);
    unsafe { CryptSIPRemoveSignedDataMsg(psubjectinfo as _, dwindex) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CryptSIPRetrieveSubjectGuid<P0>(filename: P0, hfilein: super::winnt::HANDLE, pgsubject: *mut windows_core::GUID) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("crypt32.dll" "system" fn CryptSIPRetrieveSubjectGuid(filename : windows_core::PCWSTR, hfilein : super::winnt::HANDLE, pgsubject : *mut windows_core::GUID) -> windows_core::BOOL);
    unsafe { CryptSIPRetrieveSubjectGuid(filename.param().abi(), hfilein, pgsubject as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CryptSIPRetrieveSubjectGuidForCatalogFile<P0>(filename: P0, hfilein: super::winnt::HANDLE, pgsubject: *mut windows_core::GUID) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("crypt32.dll" "system" fn CryptSIPRetrieveSubjectGuidForCatalogFile(filename : windows_core::PCWSTR, hfilein : super::winnt::HANDLE, pgsubject : *mut windows_core::GUID) -> windows_core::BOOL);
    unsafe { CryptSIPRetrieveSubjectGuidForCatalogFile(filename.param().abi(), hfilein, pgsubject as _) }
}
#[cfg(all(feature = "mscat", feature = "wincrypt", feature = "winnt"))]
#[inline]
pub unsafe fn CryptSIPVerifyIndirectData(psubjectinfo: *mut SIP_SUBJECTINFO, pindirectdata: *mut SIP_INDIRECT_DATA) -> windows_core::BOOL {
    windows_core::link!("crypt32.dll" "system" fn CryptSIPVerifyIndirectData(psubjectinfo : *mut SIP_SUBJECTINFO, pindirectdata : *mut SIP_INDIRECT_DATA) -> windows_core::BOOL);
    unsafe { CryptSIPVerifyIndirectData(psubjectinfo as _, pindirectdata as _) }
}
#[cfg(feature = "wincrypt")]
pub type CRYPT_DIGEST_DATA = super::wincrypt::CRYPT_HASH_BLOB;
#[cfg(all(feature = "mscat", feature = "wincrypt", feature = "winnt"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSIP_DISPATCH_INFO(pub *mut SIP_DISPATCH_INFO);
#[cfg(all(feature = "mscat", feature = "wincrypt", feature = "winnt"))]
impl LPSIP_DISPATCH_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "mscat", feature = "wincrypt", feature = "winnt"))]
impl Default for LPSIP_DISPATCH_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "mscat", feature = "wincrypt", feature = "winnt"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSIP_SUBJECTINFO(pub *mut SIP_SUBJECTINFO);
#[cfg(all(feature = "mscat", feature = "wincrypt", feature = "winnt"))]
impl LPSIP_SUBJECTINFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "mscat", feature = "wincrypt", feature = "winnt"))]
impl Default for LPSIP_SUBJECTINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MS_ADDINFO_CATALOGMEMBER {
    pub cbStruct: u32,
    pub pStore: *mut super::mscat::CRYPTCATSTORE,
    pub pMember: *mut super::mscat::CRYPTCATMEMBER,
}
#[cfg(all(feature = "mscat", feature = "wincrypt", feature = "winnt"))]
impl Default for MS_ADDINFO_CATALOGMEMBER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MS_ADDINFO_DETACHEDSIG {
    pub cbStruct: u32,
    pub hSignatureFile: super::winnt::HANDLE,
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMS_ADDINFO_BLOB(pub *mut MS_ADDINFO_BLOB);
impl PMS_ADDINFO_BLOB {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PMS_ADDINFO_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "mscat", feature = "wincrypt", feature = "winnt"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMS_ADDINFO_CATALOGMEMBER(pub *mut MS_ADDINFO_CATALOGMEMBER);
#[cfg(all(feature = "mscat", feature = "wincrypt", feature = "winnt"))]
impl PMS_ADDINFO_CATALOGMEMBER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "mscat", feature = "wincrypt", feature = "winnt"))]
impl Default for PMS_ADDINFO_CATALOGMEMBER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMS_ADDINFO_DETACHEDSIG(pub *mut MS_ADDINFO_DETACHEDSIG);
#[cfg(feature = "winnt")]
impl PMS_ADDINFO_DETACHEDSIG {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "winnt")]
impl Default for PMS_ADDINFO_DETACHEDSIG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "wincrypt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMS_ADDINFO_FLAT(pub *mut MS_ADDINFO_FLAT);
#[cfg(feature = "wincrypt")]
impl PMS_ADDINFO_FLAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "wincrypt")]
impl Default for PMS_ADDINFO_FLAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSIP_ADD_NEWPROVIDER(pub *mut SIP_ADD_NEWPROVIDER);
impl PSIP_ADD_NEWPROVIDER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSIP_ADD_NEWPROVIDER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PSIP_CAP_SET(pub PSIP_CAP_SET_V3);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSIP_CAP_SET_V2(pub *mut SIP_CAP_SET_V2);
impl PSIP_CAP_SET_V2 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSIP_CAP_SET_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSIP_CAP_SET_V3(pub *mut SIP_CAP_SET_V3);
impl PSIP_CAP_SET_V3 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSIP_CAP_SET_V3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "wincrypt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSIP_INDIRECT_DATA(pub *mut SIP_INDIRECT_DATA);
#[cfg(feature = "wincrypt")]
impl PSIP_INDIRECT_DATA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "wincrypt")]
impl Default for PSIP_INDIRECT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SIP_ADD_NEWPROVIDER {
    pub cbStruct: u32,
    pub pgSubject: *mut windows_core::GUID,
    pub pwszDLLFileName: *mut u16,
    pub pwszMagicNumber: *mut u16,
    pub pwszIsFunctionName: *mut u16,
    pub pwszGetFuncName: *mut u16,
    pub pwszPutFuncName: *mut u16,
    pub pwszCreateFuncName: *mut u16,
    pub pwszVerifyFuncName: *mut u16,
    pub pwszRemoveFuncName: *mut u16,
    pub pwszIsFunctionNameFmt2: *mut u16,
    pub pwszGetCapFuncName: windows_core::PWSTR,
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SIP_CAP_SET_V2 {
    pub cbSize: u32,
    pub dwVersion: u32,
    pub isMultiSign: windows_core::BOOL,
    pub dwReserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SIP_CAP_SET_V3 {
    pub cbSize: u32,
    pub dwVersion: u32,
    pub isMultiSign: windows_core::BOOL,
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
#[derive(Clone, Copy, Debug, Default)]
pub struct SIP_DISPATCH_INFO {
    pub cbSize: u32,
    pub hSIP: super::winnt::HANDLE,
    pub pfGet: pCryptSIPGetSignedDataMsg,
    pub pfPut: pCryptSIPPutSignedDataMsg,
    pub pfCreate: pCryptSIPCreateIndirectData,
    pub pfVerify: pCryptSIPVerifyIndirectData,
    pub pfRemove: pCryptSIPRemoveSignedDataMsg,
}
#[repr(C)]
#[cfg(feature = "wincrypt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SIP_INDIRECT_DATA {
    pub Data: super::wincrypt::CRYPT_ATTRIBUTE_TYPE_VALUE,
    pub DigestAlgorithm: super::wincrypt::CRYPT_ALGORITHM_IDENTIFIER,
    pub Digest: super::wincrypt::CRYPT_HASH_BLOB,
}
pub const SIP_MAX_MAGIC_NUMBER: u32 = 4;
#[repr(C)]
#[cfg(all(feature = "mscat", feature = "wincrypt", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct SIP_SUBJECTINFO {
    pub cbSize: u32,
    pub pgSubjectType: *mut windows_core::GUID,
    pub hFile: super::winnt::HANDLE,
    pub pwsFileName: windows_core::PCWSTR,
    pub pwsDisplayName: windows_core::PCWSTR,
    pub dwReserved1: u32,
    pub dwIntVersion: u32,
    pub hProv: super::wincrypt::HCRYPTPROV,
    pub DigestAlgorithm: super::wincrypt::CRYPT_ALGORITHM_IDENTIFIER,
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
pub type pCryptSIPCreateIndirectData = Option<unsafe extern "system" fn(psubjectinfo: *mut SIP_SUBJECTINFO, pcbindirectdata: *mut u32, pindirectdata: *mut SIP_INDIRECT_DATA) -> windows_core::BOOL>;
#[cfg(all(feature = "mscat", feature = "wincrypt", feature = "winnt"))]
pub type pCryptSIPGetCaps = Option<unsafe extern "system" fn(psubjinfo: *const SIP_SUBJECTINFO, pcaps: *mut SIP_CAP_SET) -> windows_core::BOOL>;
#[cfg(all(feature = "mscat", feature = "wincrypt", feature = "winnt"))]
pub type pCryptSIPGetSealedDigest = Option<unsafe extern "system" fn(psubjectinfo: *const SIP_SUBJECTINFO, psig: *const u8, dwsig: u32, pbdigest: *mut u8, pcbdigest: *mut u32) -> windows_core::BOOL>;
#[cfg(all(feature = "mscat", feature = "wincrypt", feature = "winnt"))]
pub type pCryptSIPGetSignedDataMsg = Option<unsafe extern "system" fn(psubjectinfo: *mut SIP_SUBJECTINFO, pdwencodingtype: *mut u32, dwindex: u32, pcbsigneddatamsg: *mut u32, pbsigneddatamsg: *mut u8) -> windows_core::BOOL>;
#[cfg(all(feature = "mscat", feature = "wincrypt", feature = "winnt"))]
pub type pCryptSIPPutSignedDataMsg = Option<unsafe extern "system" fn(psubjectinfo: *mut SIP_SUBJECTINFO, dwencodingtype: u32, pdwindex: *mut u32, cbsigneddatamsg: u32, pbsigneddatamsg: *mut u8) -> windows_core::BOOL>;
#[cfg(all(feature = "mscat", feature = "wincrypt", feature = "winnt"))]
pub type pCryptSIPRemoveSignedDataMsg = Option<unsafe extern "system" fn(psubjectinfo: *mut SIP_SUBJECTINFO, dwindex: u32) -> windows_core::BOOL>;
#[cfg(all(feature = "mscat", feature = "wincrypt", feature = "winnt"))]
pub type pCryptSIPVerifyIndirectData = Option<unsafe extern "system" fn(psubjectinfo: *mut SIP_SUBJECTINFO, pindirectdata: *mut SIP_INDIRECT_DATA) -> windows_core::BOOL>;
#[cfg(feature = "winnt")]
pub type pfnIsFileSupported = Option<unsafe extern "system" fn(hfile: super::winnt::HANDLE, pgsubject: *mut windows_core::GUID) -> windows_core::BOOL>;
pub type pfnIsFileSupportedName = Option<unsafe extern "system" fn(pwszfilename: *mut u16, pgsubject: *mut windows_core::GUID) -> windows_core::BOOL>;
