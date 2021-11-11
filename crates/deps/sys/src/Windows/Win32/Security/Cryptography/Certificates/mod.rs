#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
    pub fn CertSrvBackupClose();
    #[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
    pub fn CertSrvBackupEnd();
    #[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
    pub fn CertSrvBackupFree();
    #[doc = "*Required features: `Win32_Security_Cryptography_Certificates`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSrvBackupGetBackupLogsW();
    #[doc = "*Required features: `Win32_Security_Cryptography_Certificates`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSrvBackupGetDatabaseNamesW();
    #[doc = "*Required features: `Win32_Security_Cryptography_Certificates`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSrvBackupGetDynamicFileListW();
    #[doc = "*Required features: `Win32_Security_Cryptography_Certificates`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSrvBackupOpenFileW();
    #[doc = "*Required features: `Win32_Security_Cryptography_Certificates`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSrvBackupPrepareW();
    #[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
    pub fn CertSrvBackupRead();
    #[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
    pub fn CertSrvBackupTruncateLogs();
    #[doc = "*Required features: `Win32_Security_Cryptography_Certificates`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSrvIsServerOnlineW();
    #[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
    pub fn CertSrvRestoreEnd();
    #[doc = "*Required features: `Win32_Security_Cryptography_Certificates`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSrvRestoreGetDatabaseLocationsW();
    #[doc = "*Required features: `Win32_Security_Cryptography_Certificates`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSrvRestorePrepareW();
    #[doc = "*Required features: `Win32_Security_Cryptography_Certificates`*"]
    pub fn CertSrvRestoreRegisterComplete();
    #[doc = "*Required features: `Win32_Security_Cryptography_Certificates`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSrvRestoreRegisterThroughFile();
    #[doc = "*Required features: `Win32_Security_Cryptography_Certificates`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSrvRestoreRegisterW();
    #[doc = "*Required features: `Win32_Security_Cryptography_Certificates`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSrvServerControlW();
    #[doc = "*Required features: `Win32_Security_Cryptography_Certificates`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PstAcquirePrivateKey();
    #[doc = "*Required features: `Win32_Security_Cryptography_Certificates`, `Win32_Foundation`, `Win32_Security_Authentication_Identity`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity"))]
    pub fn PstGetCertificateChain();
    #[doc = "*Required features: `Win32_Security_Cryptography_Certificates`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PstGetCertificates();
    #[doc = "*Required features: `Win32_Security_Cryptography_Certificates`, `Win32_Foundation`, `Win32_Security_Authentication_Identity`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity"))]
    pub fn PstGetTrustAnchors();
    #[doc = "*Required features: `Win32_Security_Cryptography_Certificates`, `Win32_Foundation`, `Win32_Security_Authentication_Identity`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity"))]
    pub fn PstGetTrustAnchorsEx();
    #[doc = "*Required features: `Win32_Security_Cryptography_Certificates`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PstGetUserNameForCertificate();
    #[doc = "*Required features: `Win32_Security_Cryptography_Certificates`, `Win32_Foundation`, `Win32_Security_Authentication_Identity`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity"))]
    pub fn PstMapCertificate();
    #[doc = "*Required features: `Win32_Security_Cryptography_Certificates`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PstValidate();
}
