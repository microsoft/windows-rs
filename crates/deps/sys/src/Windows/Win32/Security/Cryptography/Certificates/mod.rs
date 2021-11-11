#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn CertSrvBackupClose();
    fn CertSrvBackupEnd();
    fn CertSrvBackupFree();
    fn CertSrvBackupGetBackupLogsW();
    fn CertSrvBackupGetDatabaseNamesW();
    fn CertSrvBackupGetDynamicFileListW();
    fn CertSrvBackupOpenFileW();
    fn CertSrvBackupPrepareW();
    fn CertSrvBackupRead();
    fn CertSrvBackupTruncateLogs();
    fn CertSrvIsServerOnlineW();
    fn CertSrvRestoreEnd();
    fn CertSrvRestoreGetDatabaseLocationsW();
    fn CertSrvRestorePrepareW();
    fn CertSrvRestoreRegisterComplete();
    fn CertSrvRestoreRegisterThroughFile();
    fn CertSrvRestoreRegisterW();
    fn CertSrvServerControlW();
    fn PstAcquirePrivateKey();
    fn PstGetCertificateChain();
    fn PstGetCertificates();
    fn PstGetTrustAnchors();
    fn PstGetTrustAnchorsEx();
    fn PstGetUserNameForCertificate();
    fn PstMapCertificate();
    fn PstValidate();
}
