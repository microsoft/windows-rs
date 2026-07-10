#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn AddUsersToEncryptedFile<P0>(lpfilename: P0, pencryptioncertificates: *const ENCRYPTION_CERTIFICATE_LIST) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn AddUsersToEncryptedFile(lpfilename : windows_core::PCWSTR, pencryptioncertificates : *const ENCRYPTION_CERTIFICATE_LIST) -> u32);
    unsafe { AddUsersToEncryptedFile(lpfilename.param().abi(), pencryptioncertificates) }
}
#[cfg(feature = "minwinbase")]
#[inline]
pub unsafe fn DuplicateEncryptionInfoFile<P0, P1>(srcfilename: P0, dstfilename: P1, dwcreationdistribution: u32, dwattributes: u32, lpsecurityattributes: Option<*const super::minwinbase::SECURITY_ATTRIBUTES>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn DuplicateEncryptionInfoFile(srcfilename : windows_core::PCWSTR, dstfilename : windows_core::PCWSTR, dwcreationdistribution : u32, dwattributes : u32, lpsecurityattributes : *const super::minwinbase::SECURITY_ATTRIBUTES) -> u32);
    unsafe { DuplicateEncryptionInfoFile(srcfilename.param().abi(), dstfilename.param().abi(), dwcreationdistribution, dwattributes, lpsecurityattributes.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn EncryptionDisable<P0>(dirpath: P0, disable: bool) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn EncryptionDisable(dirpath : windows_core::PCWSTR, disable : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { EncryptionDisable(dirpath.param().abi(), disable.into()) }
}
#[inline]
pub unsafe fn FreeEncryptedFileMetadata(pbmetadata: *const u8) {
    windows_core::link!("advapi32.dll" "system" fn FreeEncryptedFileMetadata(pbmetadata : *const u8));
    unsafe { FreeEncryptedFileMetadata(pbmetadata) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn FreeEncryptionCertificateHashList(pusers: *const ENCRYPTION_CERTIFICATE_HASH_LIST) {
    windows_core::link!("advapi32.dll" "system" fn FreeEncryptionCertificateHashList(pusers : *const ENCRYPTION_CERTIFICATE_HASH_LIST));
    unsafe { FreeEncryptionCertificateHashList(pusers) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn GetEncryptedFileMetadata<P0>(lpfilename: P0, pcbmetadata: *mut u32, ppbmetadata: *mut super::minwindef::PBYTE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn GetEncryptedFileMetadata(lpfilename : windows_core::PCWSTR, pcbmetadata : *mut u32, ppbmetadata : *mut super::minwindef::PBYTE) -> u32);
    unsafe { GetEncryptedFileMetadata(lpfilename.param().abi(), pcbmetadata as _, ppbmetadata as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn QueryRecoveryAgentsOnEncryptedFile<P0>(lpfilename: P0, precoveryagents: *mut PENCRYPTION_CERTIFICATE_HASH_LIST) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn QueryRecoveryAgentsOnEncryptedFile(lpfilename : windows_core::PCWSTR, precoveryagents : *mut PENCRYPTION_CERTIFICATE_HASH_LIST) -> u32);
    unsafe { QueryRecoveryAgentsOnEncryptedFile(lpfilename.param().abi(), precoveryagents as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn QueryUsersOnEncryptedFile<P0>(lpfilename: P0, pusers: *mut PENCRYPTION_CERTIFICATE_HASH_LIST) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn QueryUsersOnEncryptedFile(lpfilename : windows_core::PCWSTR, pusers : *mut PENCRYPTION_CERTIFICATE_HASH_LIST) -> u32);
    unsafe { QueryUsersOnEncryptedFile(lpfilename.param().abi(), pusers as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn RemoveUsersFromEncryptedFile<P0>(lpfilename: P0, phashes: *const ENCRYPTION_CERTIFICATE_HASH_LIST) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RemoveUsersFromEncryptedFile(lpfilename : windows_core::PCWSTR, phashes : *const ENCRYPTION_CERTIFICATE_HASH_LIST) -> u32);
    unsafe { RemoveUsersFromEncryptedFile(lpfilename.param().abi(), phashes) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn SetEncryptedFileMetadata<P0>(lpfilename: P0, pboldmetadata: Option<*const u8>, pbnewmetadata: *const u8, pownerhash: *const ENCRYPTION_CERTIFICATE_HASH, dwoperation: u32, pcertificatesadded: Option<*const ENCRYPTION_CERTIFICATE_HASH_LIST>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn SetEncryptedFileMetadata(lpfilename : windows_core::PCWSTR, pboldmetadata : *const u8, pbnewmetadata : *const u8, pownerhash : *const ENCRYPTION_CERTIFICATE_HASH, dwoperation : u32, pcertificatesadded : *const ENCRYPTION_CERTIFICATE_HASH_LIST) -> u32);
    unsafe { SetEncryptedFileMetadata(lpfilename.param().abi(), pboldmetadata.unwrap_or(core::mem::zeroed()) as _, pbnewmetadata, pownerhash, dwoperation, pcertificatesadded.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn SetUserFileEncryptionKey(pencryptioncertificate: Option<*const ENCRYPTION_CERTIFICATE>) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn SetUserFileEncryptionKey(pencryptioncertificate : *const ENCRYPTION_CERTIFICATE) -> u32);
    unsafe { SetUserFileEncryptionKey(pencryptioncertificate.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn SetUserFileEncryptionKeyEx(pencryptioncertificate: Option<*const ENCRYPTION_CERTIFICATE>, dwcapabilities: u32, dwflags: u32, pvreserved: Option<*const core::ffi::c_void>) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn SetUserFileEncryptionKeyEx(pencryptioncertificate : *const ENCRYPTION_CERTIFICATE, dwcapabilities : u32, dwflags : u32, pvreserved : *const core::ffi::c_void) -> u32);
    unsafe { SetUserFileEncryptionKeyEx(pencryptioncertificate.unwrap_or(core::mem::zeroed()) as _, dwcapabilities, dwflags, pvreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EFS_CERTIFICATE_BLOB {
    pub dwCertEncodingType: u32,
    pub cbData: u32,
    pub pbData: super::minwindef::PBYTE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EFS_COMPATIBILITY_INFO {
    pub EfsVersion: u32,
}
pub const EFS_COMPATIBILITY_VERSION_NCRYPT_PROTECTOR: u32 = 5;
pub const EFS_COMPATIBILITY_VERSION_PFILE_PROTECTOR: u32 = 6;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EFS_DECRYPTION_STATUS_INFO {
    pub dwDecryptionError: u32,
    pub dwHashOffset: u32,
    pub cbHash: u32,
}
pub const EFS_EFS_SUBVER_EFS_CERT: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EFS_ENCRYPTION_STATUS_INFO {
    pub bHasCurrentKey: windows_core::BOOL,
    pub dwEncryptionError: u32,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EFS_HASH_BLOB {
    pub cbData: u32,
    pub pbData: super::minwindef::PBYTE,
}
#[repr(C)]
#[cfg(feature = "wincrypt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EFS_KEY_INFO {
    pub dwVersion: u32,
    pub Entropy: u32,
    pub Algorithm: super::wincrypt::ALG_ID,
    pub KeyLength: u32,
}
pub const EFS_METADATA_ADD_USER: u32 = 1;
pub const EFS_METADATA_GENERAL_OP: u32 = 8;
pub const EFS_METADATA_REMOVE_USER: u32 = 2;
pub const EFS_METADATA_REPLACE_USER: u32 = 4;
pub const EFS_PFILE_SUBVER_APPX: u32 = 3;
pub const EFS_PFILE_SUBVER_RMS: u32 = 2;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EFS_PIN_BLOB {
    pub cbPadding: u32,
    pub cbData: u32,
    pub pbData: super::minwindef::PBYTE,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EFS_RPC_BLOB {
    pub cbData: u32,
    pub pbData: super::minwindef::PBYTE,
}
pub const EFS_SUBVER_UNKNOWN: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EFS_VERSION_INFO {
    pub EfsVersion: u32,
    pub SubVersion: u32,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ENCRYPTED_FILE_METADATA_SIGNATURE {
    pub dwEfsAccessType: u32,
    pub pCertificatesAdded: PENCRYPTION_CERTIFICATE_HASH_LIST,
    pub pEncryptionCertificate: PENCRYPTION_CERTIFICATE,
    pub pEfsStreamSignature: PEFS_RPC_BLOB,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ENCRYPTION_CERTIFICATE {
    pub cbTotalLength: u32,
    pub pUserSid: *mut super::winnt::SID,
    pub pCertBlob: PEFS_CERTIFICATE_BLOB,
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for ENCRYPTION_CERTIFICATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ENCRYPTION_CERTIFICATE_HASH {
    pub cbTotalLength: u32,
    pub pUserSid: *mut super::winnt::SID,
    pub pHash: PEFS_HASH_BLOB,
    pub lpDisplayInformation: windows_core::PWSTR,
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for ENCRYPTION_CERTIFICATE_HASH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ENCRYPTION_CERTIFICATE_HASH_LIST {
    pub nCert_Hash: u32,
    pub pUsers: *mut PENCRYPTION_CERTIFICATE_HASH,
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for ENCRYPTION_CERTIFICATE_HASH_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ENCRYPTION_CERTIFICATE_LIST {
    pub nUsers: u32,
    pub pUsers: *mut PENCRYPTION_CERTIFICATE,
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for ENCRYPTION_CERTIFICATE_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ENCRYPTION_PROTECTOR {
    pub cbTotalLength: u32,
    pub pUserSid: *mut super::winnt::SID,
    pub lpProtectorDescriptor: windows_core::PWSTR,
}
#[cfg(feature = "winnt")]
impl Default for ENCRYPTION_PROTECTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ENCRYPTION_PROTECTOR_LIST {
    pub nProtectors: u32,
    pub pProtectors: *mut PENCRYPTION_PROTECTOR,
}
#[cfg(feature = "winnt")]
impl Default for ENCRYPTION_PROTECTOR_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MAX_SID_SIZE: u32 = 256;
#[cfg(feature = "minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PEFS_CERTIFICATE_BLOB(pub *mut EFS_CERTIFICATE_BLOB);
#[cfg(feature = "minwindef")]
impl PEFS_CERTIFICATE_BLOB {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "minwindef")]
impl Default for PEFS_CERTIFICATE_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PEFS_COMPATIBILITY_INFO(pub *mut EFS_COMPATIBILITY_INFO);
impl PEFS_COMPATIBILITY_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PEFS_COMPATIBILITY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PEFS_DECRYPTION_STATUS_INFO(pub *mut EFS_DECRYPTION_STATUS_INFO);
impl PEFS_DECRYPTION_STATUS_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PEFS_DECRYPTION_STATUS_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PEFS_ENCRYPTION_STATUS_INFO(pub *mut EFS_ENCRYPTION_STATUS_INFO);
impl PEFS_ENCRYPTION_STATUS_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PEFS_ENCRYPTION_STATUS_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PEFS_HASH_BLOB(pub *mut EFS_HASH_BLOB);
#[cfg(feature = "minwindef")]
impl PEFS_HASH_BLOB {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "minwindef")]
impl Default for PEFS_HASH_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "wincrypt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PEFS_KEY_INFO(pub *mut EFS_KEY_INFO);
#[cfg(feature = "wincrypt")]
impl PEFS_KEY_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "wincrypt")]
impl Default for PEFS_KEY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PEFS_PIN_BLOB(pub *mut EFS_PIN_BLOB);
#[cfg(feature = "minwindef")]
impl PEFS_PIN_BLOB {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "minwindef")]
impl Default for PEFS_PIN_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PEFS_RPC_BLOB(pub *mut EFS_RPC_BLOB);
#[cfg(feature = "minwindef")]
impl PEFS_RPC_BLOB {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "minwindef")]
impl Default for PEFS_RPC_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PEFS_VERSION_INFO(pub *mut EFS_VERSION_INFO);
impl PEFS_VERSION_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PEFS_VERSION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PENCRYPTED_FILE_METADATA_SIGNATURE(pub *mut ENCRYPTED_FILE_METADATA_SIGNATURE);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl PENCRYPTED_FILE_METADATA_SIGNATURE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for PENCRYPTED_FILE_METADATA_SIGNATURE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PENCRYPTION_CERTIFICATE(pub *mut ENCRYPTION_CERTIFICATE);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl PENCRYPTION_CERTIFICATE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for PENCRYPTION_CERTIFICATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PENCRYPTION_CERTIFICATE_HASH(pub *mut ENCRYPTION_CERTIFICATE_HASH);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl PENCRYPTION_CERTIFICATE_HASH {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for PENCRYPTION_CERTIFICATE_HASH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PENCRYPTION_CERTIFICATE_HASH_LIST(pub *mut ENCRYPTION_CERTIFICATE_HASH_LIST);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl PENCRYPTION_CERTIFICATE_HASH_LIST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for PENCRYPTION_CERTIFICATE_HASH_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PENCRYPTION_CERTIFICATE_LIST(pub *mut ENCRYPTION_CERTIFICATE_LIST);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl PENCRYPTION_CERTIFICATE_LIST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for PENCRYPTION_CERTIFICATE_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PENCRYPTION_PROTECTOR(pub *mut ENCRYPTION_PROTECTOR);
#[cfg(feature = "winnt")]
impl PENCRYPTION_PROTECTOR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "winnt")]
impl Default for PENCRYPTION_PROTECTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PENCRYPTION_PROTECTOR_LIST(pub *mut ENCRYPTION_PROTECTOR_LIST);
#[cfg(feature = "winnt")]
impl PENCRYPTION_PROTECTOR_LIST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "winnt")]
impl Default for PENCRYPTION_PROTECTOR_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WINEFS_SETUSERKEY_SET_CAPABILITIES: u32 = 1;
