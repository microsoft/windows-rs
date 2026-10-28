#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn AddUsersToEncryptedFile<P0>(lpfilename: P0, pencryptioncertificates: PENCRYPTION_CERTIFICATE_LIST) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn AddUsersToEncryptedFile(lpfilename : windows_core::PCWSTR, pencryptioncertificates : PENCRYPTION_CERTIFICATE_LIST) -> super::DWORD);
    unsafe { AddUsersToEncryptedFile(lpfilename.param().abi(), pencryptioncertificates) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn DuplicateEncryptionInfoFile<P0, P1>(srcfilename: P0, dstfilename: P1, dwcreationdistribution: super::DWORD, dwattributes: super::DWORD, lpsecurityattributes: Option<super::LPSECURITY_ATTRIBUTES>) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn DuplicateEncryptionInfoFile(srcfilename : windows_core::PCWSTR, dstfilename : windows_core::PCWSTR, dwcreationdistribution : super::DWORD, dwattributes : super::DWORD, lpsecurityattributes : super::LPSECURITY_ATTRIBUTES) -> super::DWORD);
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
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn FreeEncryptedFileMetadata(pbmetadata: super::PBYTE) {
    windows_core::link!("advapi32.dll" "system" fn FreeEncryptedFileMetadata(pbmetadata : super::PBYTE));
    unsafe { FreeEncryptedFileMetadata(pbmetadata) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FreeEncryptionCertificateHashList(pusers: PENCRYPTION_CERTIFICATE_HASH_LIST) {
    windows_core::link!("advapi32.dll" "system" fn FreeEncryptionCertificateHashList(pusers : PENCRYPTION_CERTIFICATE_HASH_LIST));
    unsafe { FreeEncryptionCertificateHashList(pusers) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn GetEncryptedFileMetadata<P0>(lpfilename: P0, pcbmetadata: super::PDWORD, ppbmetadata: *mut super::PBYTE) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn GetEncryptedFileMetadata(lpfilename : windows_core::PCWSTR, pcbmetadata : super::PDWORD, ppbmetadata : *mut super::PBYTE) -> super::DWORD);
    unsafe { GetEncryptedFileMetadata(lpfilename.param().abi(), pcbmetadata as _, ppbmetadata as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn QueryRecoveryAgentsOnEncryptedFile<P0>(lpfilename: P0, precoveryagents: *mut PENCRYPTION_CERTIFICATE_HASH_LIST) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn QueryRecoveryAgentsOnEncryptedFile(lpfilename : windows_core::PCWSTR, precoveryagents : *mut PENCRYPTION_CERTIFICATE_HASH_LIST) -> super::DWORD);
    unsafe { QueryRecoveryAgentsOnEncryptedFile(lpfilename.param().abi(), precoveryagents as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn QueryUsersOnEncryptedFile<P0>(lpfilename: P0, pusers: *mut PENCRYPTION_CERTIFICATE_HASH_LIST) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn QueryUsersOnEncryptedFile(lpfilename : windows_core::PCWSTR, pusers : *mut PENCRYPTION_CERTIFICATE_HASH_LIST) -> super::DWORD);
    unsafe { QueryUsersOnEncryptedFile(lpfilename.param().abi(), pusers as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn RemoveUsersFromEncryptedFile<P0>(lpfilename: P0, phashes: PENCRYPTION_CERTIFICATE_HASH_LIST) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RemoveUsersFromEncryptedFile(lpfilename : windows_core::PCWSTR, phashes : PENCRYPTION_CERTIFICATE_HASH_LIST) -> super::DWORD);
    unsafe { RemoveUsersFromEncryptedFile(lpfilename.param().abi(), phashes) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetEncryptedFileMetadata<P0>(lpfilename: P0, pboldmetadata: Option<super::PBYTE>, pbnewmetadata: super::PBYTE, pownerhash: PENCRYPTION_CERTIFICATE_HASH, dwoperation: super::DWORD, pcertificatesadded: Option<PENCRYPTION_CERTIFICATE_HASH_LIST>) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn SetEncryptedFileMetadata(lpfilename : windows_core::PCWSTR, pboldmetadata : super::PBYTE, pbnewmetadata : super::PBYTE, pownerhash : PENCRYPTION_CERTIFICATE_HASH, dwoperation : super::DWORD, pcertificatesadded : PENCRYPTION_CERTIFICATE_HASH_LIST) -> super::DWORD);
    unsafe { SetEncryptedFileMetadata(lpfilename.param().abi(), pboldmetadata.unwrap_or(core::mem::zeroed()) as _, pbnewmetadata, pownerhash, dwoperation, pcertificatesadded.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetUserFileEncryptionKey(pencryptioncertificate: Option<PENCRYPTION_CERTIFICATE>) -> super::DWORD {
    windows_core::link!("advapi32.dll" "system" fn SetUserFileEncryptionKey(pencryptioncertificate : PENCRYPTION_CERTIFICATE) -> super::DWORD);
    unsafe { SetUserFileEncryptionKey(pencryptioncertificate.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetUserFileEncryptionKeyEx(pencryptioncertificate: Option<PENCRYPTION_CERTIFICATE>, dwcapabilities: super::DWORD, dwflags: super::DWORD, pvreserved: Option<super::LPVOID>) -> super::DWORD {
    windows_core::link!("advapi32.dll" "system" fn SetUserFileEncryptionKeyEx(pencryptioncertificate : PENCRYPTION_CERTIFICATE, dwcapabilities : super::DWORD, dwflags : super::DWORD, pvreserved : super::LPVOID) -> super::DWORD);
    unsafe { SetUserFileEncryptionKeyEx(pencryptioncertificate.unwrap_or(core::mem::zeroed()) as _, dwcapabilities, dwflags, pvreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EFS_CERTIFICATE_BLOB {
    pub dwCertEncodingType: super::DWORD,
    pub cbData: super::DWORD,
    pub pbData: super::PBYTE,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EFS_COMPATIBILITY_INFO {
    pub EfsVersion: super::DWORD,
}
pub const EFS_COMPATIBILITY_VERSION_NCRYPT_PROTECTOR: i32 = 5;
pub const EFS_COMPATIBILITY_VERSION_PFILE_PROTECTOR: i32 = 6;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EFS_DECRYPTION_STATUS_INFO {
    pub dwDecryptionError: super::DWORD,
    pub dwHashOffset: super::DWORD,
    pub cbHash: super::DWORD,
}
pub const EFS_EFS_SUBVER_EFS_CERT: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EFS_ENCRYPTION_STATUS_INFO {
    pub bHasCurrentKey: windows_core::BOOL,
    pub dwEncryptionError: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EFS_HASH_BLOB {
    pub cbData: super::DWORD,
    pub pbData: super::PBYTE,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EFS_KEY_INFO {
    pub dwVersion: super::DWORD,
    pub Entropy: super::ULONG,
    pub Algorithm: super::ALG_ID,
    pub KeyLength: super::ULONG,
}
pub const EFS_METADATA_ADD_USER: i32 = 1;
pub const EFS_METADATA_GENERAL_OP: i32 = 8;
pub const EFS_METADATA_REMOVE_USER: i32 = 2;
pub const EFS_METADATA_REPLACE_USER: i32 = 4;
pub const EFS_PFILE_SUBVER_APPX: i32 = 3;
pub const EFS_PFILE_SUBVER_RMS: i32 = 2;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EFS_PIN_BLOB {
    pub cbPadding: super::DWORD,
    pub cbData: super::DWORD,
    pub pbData: super::PBYTE,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EFS_RPC_BLOB {
    pub cbData: super::DWORD,
    pub pbData: super::PBYTE,
}
pub const EFS_SUBVER_UNKNOWN: i32 = 0;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EFS_VERSION_INFO {
    pub EfsVersion: super::DWORD,
    pub SubVersion: super::DWORD,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ENCRYPTED_FILE_METADATA_SIGNATURE {
    pub dwEfsAccessType: super::DWORD,
    pub pCertificatesAdded: PENCRYPTION_CERTIFICATE_HASH_LIST,
    pub pEncryptionCertificate: PENCRYPTION_CERTIFICATE,
    pub pEfsStreamSignature: PEFS_RPC_BLOB,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ENCRYPTION_CERTIFICATE {
    pub cbTotalLength: super::DWORD,
    pub pUserSid: *mut super::SID,
    pub pCertBlob: PEFS_CERTIFICATE_BLOB,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ENCRYPTION_CERTIFICATE_HASH {
    pub cbTotalLength: super::DWORD,
    pub pUserSid: *mut super::SID,
    pub pHash: PEFS_HASH_BLOB,
    pub lpDisplayInformation: windows_core::PWSTR,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ENCRYPTION_CERTIFICATE_HASH_LIST {
    pub nCert_Hash: super::DWORD,
    pub pUsers: *mut PENCRYPTION_CERTIFICATE_HASH,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ENCRYPTION_CERTIFICATE_LIST {
    pub nUsers: super::DWORD,
    pub pUsers: *mut PENCRYPTION_CERTIFICATE,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ENCRYPTION_PROTECTOR {
    pub cbTotalLength: super::DWORD,
    pub pUserSid: *mut super::SID,
    pub lpProtectorDescriptor: windows_core::PWSTR,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ENCRYPTION_PROTECTOR_LIST {
    pub nProtectors: super::DWORD,
    pub pProtectors: *mut PENCRYPTION_PROTECTOR,
}
pub const MAX_SID_SIZE: i32 = 256;
#[cfg(feature = "shared")]
pub type PEFS_CERTIFICATE_BLOB = *mut EFS_CERTIFICATE_BLOB;
#[cfg(feature = "shared")]
pub type PEFS_COMPATIBILITY_INFO = *mut EFS_COMPATIBILITY_INFO;
#[cfg(feature = "shared")]
pub type PEFS_DECRYPTION_STATUS_INFO = *mut EFS_DECRYPTION_STATUS_INFO;
#[cfg(feature = "shared")]
pub type PEFS_ENCRYPTION_STATUS_INFO = *mut EFS_ENCRYPTION_STATUS_INFO;
#[cfg(feature = "shared")]
pub type PEFS_HASH_BLOB = *mut EFS_HASH_BLOB;
#[cfg(feature = "shared")]
pub type PEFS_KEY_INFO = *mut EFS_KEY_INFO;
#[cfg(feature = "shared")]
pub type PEFS_PIN_BLOB = *mut EFS_PIN_BLOB;
#[cfg(feature = "shared")]
pub type PEFS_RPC_BLOB = *mut EFS_RPC_BLOB;
#[cfg(feature = "shared")]
pub type PEFS_VERSION_INFO = *mut EFS_VERSION_INFO;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PENCRYPTED_FILE_METADATA_SIGNATURE = *mut ENCRYPTED_FILE_METADATA_SIGNATURE;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PENCRYPTION_CERTIFICATE = *mut ENCRYPTION_CERTIFICATE;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PENCRYPTION_CERTIFICATE_HASH = *mut ENCRYPTION_CERTIFICATE_HASH;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PENCRYPTION_CERTIFICATE_HASH_LIST = *mut ENCRYPTION_CERTIFICATE_HASH_LIST;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PENCRYPTION_CERTIFICATE_LIST = *mut ENCRYPTION_CERTIFICATE_LIST;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PENCRYPTION_PROTECTOR = *mut ENCRYPTION_PROTECTOR;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PENCRYPTION_PROTECTOR_LIST = *mut ENCRYPTION_PROTECTOR_LIST;
pub const WINEFS_SETUSERKEY_SET_CAPABILITIES: i32 = 1;
