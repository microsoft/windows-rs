#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn AddUsersToEncryptedFile(lpfilename : windows_sys::core::PCWSTR, pencryptioncertificates : PENCRYPTION_CERTIFICATE_LIST) -> super::DWORD);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared"))]
windows_link::link!("advapi32.dll" "system" fn DuplicateEncryptionInfoFile(srcfilename : windows_sys::core::PCWSTR, dstfilename : windows_sys::core::PCWSTR, dwcreationdistribution : super::DWORD, dwattributes : super::DWORD, lpsecurityattributes : super::LPSECURITY_ATTRIBUTES) -> super::DWORD);
windows_link::link!("advapi32.dll" "system" fn EncryptionDisable(dirpath : windows_sys::core::PCWSTR, disable : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("advapi32.dll" "system" fn FreeEncryptedFileMetadata(pbmetadata : super::PBYTE));
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn FreeEncryptionCertificateHashList(pusers : PENCRYPTION_CERTIFICATE_HASH_LIST));
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("advapi32.dll" "system" fn GetEncryptedFileMetadata(lpfilename : windows_sys::core::PCWSTR, pcbmetadata : super::PDWORD, ppbmetadata : *mut super::PBYTE) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn QueryRecoveryAgentsOnEncryptedFile(lpfilename : windows_sys::core::PCWSTR, precoveryagents : *mut PENCRYPTION_CERTIFICATE_HASH_LIST) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn QueryUsersOnEncryptedFile(lpfilename : windows_sys::core::PCWSTR, pusers : *mut PENCRYPTION_CERTIFICATE_HASH_LIST) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn RemoveUsersFromEncryptedFile(lpfilename : windows_sys::core::PCWSTR, phashes : PENCRYPTION_CERTIFICATE_HASH_LIST) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn SetEncryptedFileMetadata(lpfilename : windows_sys::core::PCWSTR, pboldmetadata : super::PBYTE, pbnewmetadata : super::PBYTE, pownerhash : PENCRYPTION_CERTIFICATE_HASH, dwoperation : super::DWORD, pcertificatesadded : PENCRYPTION_CERTIFICATE_HASH_LIST) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn SetUserFileEncryptionKey(pencryptioncertificate : PENCRYPTION_CERTIFICATE) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn SetUserFileEncryptionKeyEx(pencryptioncertificate : PENCRYPTION_CERTIFICATE, dwcapabilities : super::DWORD, dwflags : super::DWORD, pvreserved : super::LPVOID) -> super::DWORD);
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct EFS_CERTIFICATE_BLOB {
    pub dwCertEncodingType: super::DWORD,
    pub cbData: super::DWORD,
    pub pbData: super::PBYTE,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct EFS_COMPATIBILITY_INFO {
    pub EfsVersion: super::DWORD,
}
pub const EFS_COMPATIBILITY_VERSION_NCRYPT_PROTECTOR: i32 = 5;
pub const EFS_COMPATIBILITY_VERSION_PFILE_PROTECTOR: i32 = 6;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct EFS_DECRYPTION_STATUS_INFO {
    pub dwDecryptionError: super::DWORD,
    pub dwHashOffset: super::DWORD,
    pub cbHash: super::DWORD,
}
pub const EFS_EFS_SUBVER_EFS_CERT: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct EFS_ENCRYPTION_STATUS_INFO {
    pub bHasCurrentKey: windows_sys::core::BOOL,
    pub dwEncryptionError: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct EFS_HASH_BLOB {
    pub cbData: super::DWORD,
    pub pbData: super::PBYTE,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
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
#[derive(Clone, Copy, Default)]
pub struct EFS_PIN_BLOB {
    pub cbPadding: super::DWORD,
    pub cbData: super::DWORD,
    pub pbData: super::PBYTE,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct EFS_RPC_BLOB {
    pub cbData: super::DWORD,
    pub pbData: super::PBYTE,
}
pub const EFS_SUBVER_UNKNOWN: i32 = 0;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct EFS_VERSION_INFO {
    pub EfsVersion: super::DWORD,
    pub SubVersion: super::DWORD,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct ENCRYPTED_FILE_METADATA_SIGNATURE {
    pub dwEfsAccessType: super::DWORD,
    pub pCertificatesAdded: PENCRYPTION_CERTIFICATE_HASH_LIST,
    pub pEncryptionCertificate: PENCRYPTION_CERTIFICATE,
    pub pEfsStreamSignature: PEFS_RPC_BLOB,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct ENCRYPTION_CERTIFICATE {
    pub cbTotalLength: super::DWORD,
    pub pUserSid: *mut super::SID,
    pub pCertBlob: PEFS_CERTIFICATE_BLOB,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct ENCRYPTION_CERTIFICATE_HASH {
    pub cbTotalLength: super::DWORD,
    pub pUserSid: *mut super::SID,
    pub pHash: PEFS_HASH_BLOB,
    pub lpDisplayInformation: windows_sys::core::PWSTR,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct ENCRYPTION_CERTIFICATE_HASH_LIST {
    pub nCert_Hash: super::DWORD,
    pub pUsers: *mut PENCRYPTION_CERTIFICATE_HASH,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct ENCRYPTION_CERTIFICATE_LIST {
    pub nUsers: super::DWORD,
    pub pUsers: *mut PENCRYPTION_CERTIFICATE,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct ENCRYPTION_PROTECTOR {
    pub cbTotalLength: super::DWORD,
    pub pUserSid: *mut super::SID,
    pub lpProtectorDescriptor: windows_sys::core::PWSTR,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
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
