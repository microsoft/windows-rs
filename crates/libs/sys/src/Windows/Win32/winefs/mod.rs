#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
windows_link::link!("advapi32.dll" "system" fn AddUsersToEncryptedFile(lpfilename : windows_sys::core::PCWSTR, pencryptioncertificates : *const ENCRYPTION_CERTIFICATE_LIST) -> u32);
#[cfg(feature = "Win32_minwinbase")]
windows_link::link!("advapi32.dll" "system" fn DuplicateEncryptionInfoFile(srcfilename : windows_sys::core::PCWSTR, dstfilename : windows_sys::core::PCWSTR, dwcreationdistribution : u32, dwattributes : u32, lpsecurityattributes : *const super::minwinbase::SECURITY_ATTRIBUTES) -> u32);
windows_link::link!("advapi32.dll" "system" fn EncryptionDisable(dirpath : windows_sys::core::PCWSTR, disable : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn FreeEncryptedFileMetadata(pbmetadata : *const u8));
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
windows_link::link!("advapi32.dll" "system" fn FreeEncryptionCertificateHashList(pusers : *const ENCRYPTION_CERTIFICATE_HASH_LIST));
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("advapi32.dll" "system" fn GetEncryptedFileMetadata(lpfilename : windows_sys::core::PCWSTR, pcbmetadata : *mut u32, ppbmetadata : *mut super::minwindef::PBYTE) -> u32);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
windows_link::link!("advapi32.dll" "system" fn QueryRecoveryAgentsOnEncryptedFile(lpfilename : windows_sys::core::PCWSTR, precoveryagents : *mut PENCRYPTION_CERTIFICATE_HASH_LIST) -> u32);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
windows_link::link!("advapi32.dll" "system" fn QueryUsersOnEncryptedFile(lpfilename : windows_sys::core::PCWSTR, pusers : *mut PENCRYPTION_CERTIFICATE_HASH_LIST) -> u32);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
windows_link::link!("advapi32.dll" "system" fn RemoveUsersFromEncryptedFile(lpfilename : windows_sys::core::PCWSTR, phashes : *const ENCRYPTION_CERTIFICATE_HASH_LIST) -> u32);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
windows_link::link!("advapi32.dll" "system" fn SetEncryptedFileMetadata(lpfilename : windows_sys::core::PCWSTR, pboldmetadata : *const u8, pbnewmetadata : *const u8, pownerhash : *const ENCRYPTION_CERTIFICATE_HASH, dwoperation : u32, pcertificatesadded : *const ENCRYPTION_CERTIFICATE_HASH_LIST) -> u32);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
windows_link::link!("advapi32.dll" "system" fn SetUserFileEncryptionKey(pencryptioncertificate : *const ENCRYPTION_CERTIFICATE) -> u32);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
windows_link::link!("advapi32.dll" "system" fn SetUserFileEncryptionKeyEx(pencryptioncertificate : *const ENCRYPTION_CERTIFICATE, dwcapabilities : u32, dwflags : u32, pvreserved : *const core::ffi::c_void) -> u32);
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy)]
pub struct EFS_CERTIFICATE_BLOB {
    pub dwCertEncodingType: u32,
    pub cbData: u32,
    pub pbData: super::minwindef::PBYTE,
}
#[cfg(feature = "Win32_minwindef")]
impl Default for EFS_CERTIFICATE_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct EFS_COMPATIBILITY_INFO {
    pub EfsVersion: u32,
}
pub const EFS_COMPATIBILITY_VERSION_NCRYPT_PROTECTOR: u32 = 5;
pub const EFS_COMPATIBILITY_VERSION_PFILE_PROTECTOR: u32 = 6;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct EFS_DECRYPTION_STATUS_INFO {
    pub dwDecryptionError: u32,
    pub dwHashOffset: u32,
    pub cbHash: u32,
}
pub const EFS_EFS_SUBVER_EFS_CERT: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct EFS_ENCRYPTION_STATUS_INFO {
    pub bHasCurrentKey: windows_sys::core::BOOL,
    pub dwEncryptionError: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy)]
pub struct EFS_HASH_BLOB {
    pub cbData: u32,
    pub pbData: super::minwindef::PBYTE,
}
#[cfg(feature = "Win32_minwindef")]
impl Default for EFS_HASH_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_wincrypt")]
#[derive(Clone, Copy, Default)]
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
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy)]
pub struct EFS_PIN_BLOB {
    pub cbPadding: u32,
    pub cbData: u32,
    pub pbData: super::minwindef::PBYTE,
}
#[cfg(feature = "Win32_minwindef")]
impl Default for EFS_PIN_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy)]
pub struct EFS_RPC_BLOB {
    pub cbData: u32,
    pub pbData: super::minwindef::PBYTE,
}
#[cfg(feature = "Win32_minwindef")]
impl Default for EFS_RPC_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const EFS_SUBVER_UNKNOWN: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct EFS_VERSION_INFO {
    pub EfsVersion: u32,
    pub SubVersion: u32,
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct ENCRYPTED_FILE_METADATA_SIGNATURE {
    pub dwEfsAccessType: u32,
    pub pCertificatesAdded: PENCRYPTION_CERTIFICATE_HASH_LIST,
    pub pEncryptionCertificate: PENCRYPTION_CERTIFICATE,
    pub pEfsStreamSignature: PEFS_RPC_BLOB,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
impl Default for ENCRYPTED_FILE_METADATA_SIGNATURE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct ENCRYPTION_CERTIFICATE {
    pub cbTotalLength: u32,
    pub pUserSid: *mut super::winnt::SID,
    pub pCertBlob: PEFS_CERTIFICATE_BLOB,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
impl Default for ENCRYPTION_CERTIFICATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct ENCRYPTION_CERTIFICATE_HASH {
    pub cbTotalLength: u32,
    pub pUserSid: *mut super::winnt::SID,
    pub pHash: PEFS_HASH_BLOB,
    pub lpDisplayInformation: windows_sys::core::PWSTR,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
impl Default for ENCRYPTION_CERTIFICATE_HASH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct ENCRYPTION_CERTIFICATE_HASH_LIST {
    pub nCert_Hash: u32,
    pub pUsers: *mut PENCRYPTION_CERTIFICATE_HASH,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
impl Default for ENCRYPTION_CERTIFICATE_HASH_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct ENCRYPTION_CERTIFICATE_LIST {
    pub nUsers: u32,
    pub pUsers: *mut PENCRYPTION_CERTIFICATE,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
impl Default for ENCRYPTION_CERTIFICATE_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct ENCRYPTION_PROTECTOR {
    pub cbTotalLength: u32,
    pub pUserSid: *mut super::winnt::SID,
    pub lpProtectorDescriptor: windows_sys::core::PWSTR,
}
#[cfg(feature = "Win32_winnt")]
impl Default for ENCRYPTION_PROTECTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct ENCRYPTION_PROTECTOR_LIST {
    pub nProtectors: u32,
    pub pProtectors: *mut PENCRYPTION_PROTECTOR,
}
#[cfg(feature = "Win32_winnt")]
impl Default for ENCRYPTION_PROTECTOR_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MAX_SID_SIZE: u32 = 256;
#[cfg(feature = "Win32_minwindef")]
pub type PEFS_CERTIFICATE_BLOB = *mut EFS_CERTIFICATE_BLOB;
pub type PEFS_COMPATIBILITY_INFO = *mut EFS_COMPATIBILITY_INFO;
pub type PEFS_DECRYPTION_STATUS_INFO = *mut EFS_DECRYPTION_STATUS_INFO;
pub type PEFS_ENCRYPTION_STATUS_INFO = *mut EFS_ENCRYPTION_STATUS_INFO;
#[cfg(feature = "Win32_minwindef")]
pub type PEFS_HASH_BLOB = *mut EFS_HASH_BLOB;
#[cfg(feature = "Win32_wincrypt")]
pub type PEFS_KEY_INFO = *mut EFS_KEY_INFO;
#[cfg(feature = "Win32_minwindef")]
pub type PEFS_PIN_BLOB = *mut EFS_PIN_BLOB;
#[cfg(feature = "Win32_minwindef")]
pub type PEFS_RPC_BLOB = *mut EFS_RPC_BLOB;
pub type PEFS_VERSION_INFO = *mut EFS_VERSION_INFO;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
pub type PENCRYPTED_FILE_METADATA_SIGNATURE = *mut ENCRYPTED_FILE_METADATA_SIGNATURE;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
pub type PENCRYPTION_CERTIFICATE = *mut ENCRYPTION_CERTIFICATE;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
pub type PENCRYPTION_CERTIFICATE_HASH = *mut ENCRYPTION_CERTIFICATE_HASH;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
pub type PENCRYPTION_CERTIFICATE_HASH_LIST = *mut ENCRYPTION_CERTIFICATE_HASH_LIST;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
pub type PENCRYPTION_CERTIFICATE_LIST = *mut ENCRYPTION_CERTIFICATE_LIST;
#[cfg(feature = "Win32_winnt")]
pub type PENCRYPTION_PROTECTOR = *mut ENCRYPTION_PROTECTOR;
#[cfg(feature = "Win32_winnt")]
pub type PENCRYPTION_PROTECTOR_LIST = *mut ENCRYPTION_PROTECTOR_LIST;
pub const WINEFS_SETUSERKEY_SET_CAPABILITIES: u32 = 1;
