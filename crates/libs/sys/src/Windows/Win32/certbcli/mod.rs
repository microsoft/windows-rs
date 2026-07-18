windows_link::link!("certadm.dll" "system" fn CertSrvBackupClose(hbc : HCSBC) -> windows_sys::core::HRESULT);
windows_link::link!("certadm.dll" "system" fn CertSrvBackupEnd(hbc : HCSBC) -> windows_sys::core::HRESULT);
windows_link::link!("certadm.dll" "system" fn CertSrvBackupFree(pv : *mut core::ffi::c_void));
windows_link::link!("certadm.dll" "system" fn CertSrvBackupGetBackupLogsW(hbc : HCSBC, ppwszzbackuplogfiles : *mut windows_sys::core::PWSTR, pcbsize : *mut u32) -> windows_sys::core::HRESULT);
windows_link::link!("certadm.dll" "system" fn CertSrvBackupGetDatabaseNamesW(hbc : HCSBC, ppwszzattachmentinformation : *mut windows_sys::core::PWSTR, pcbsize : *mut u32) -> windows_sys::core::HRESULT);
windows_link::link!("certadm.dll" "system" fn CertSrvBackupGetDynamicFileListW(hbc : HCSBC, ppwszzfilelist : *mut windows_sys::core::PWSTR, pcbsize : *mut u32) -> windows_sys::core::HRESULT);
windows_link::link!("certadm.dll" "system" fn CertSrvBackupOpenFileW(hbc : HCSBC, pwszattachmentname : *const u16, cbreadhintsize : u32, plifilesize : *mut i64) -> windows_sys::core::HRESULT);
windows_link::link!("certadm.dll" "system" fn CertSrvBackupPrepareW(pwszservername : *const u16, grbitjet : u32, dwbackupflags : u32, phbc : *mut HCSBC) -> windows_sys::core::HRESULT);
windows_link::link!("certadm.dll" "system" fn CertSrvBackupRead(hbc : HCSBC, pvbuffer : *mut core::ffi::c_void, cbbuffer : u32, pcbread : *mut u32) -> windows_sys::core::HRESULT);
windows_link::link!("certadm.dll" "system" fn CertSrvBackupTruncateLogs(hbc : HCSBC) -> windows_sys::core::HRESULT);
windows_link::link!("certadm.dll" "system" fn CertSrvIsServerOnlineW(pwszservername : *const u16, pfserveronline : *mut windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
windows_link::link!("certadm.dll" "system" fn CertSrvRestoreEnd(hbc : HCSBC) -> windows_sys::core::HRESULT);
windows_link::link!("certadm.dll" "system" fn CertSrvRestoreGetDatabaseLocationsW(hbc : HCSBC, ppwszzdatabaselocationlist : *mut windows_sys::core::PWSTR, pcbsize : *mut u32) -> windows_sys::core::HRESULT);
windows_link::link!("certadm.dll" "system" fn CertSrvRestorePrepareW(pwszservername : *const u16, dwrestoreflags : u32, phbc : *mut HCSBC) -> windows_sys::core::HRESULT);
windows_link::link!("certadm.dll" "system" fn CertSrvRestoreRegisterComplete(hbc : HCSBC, hrrestorestate : windows_sys::core::HRESULT) -> windows_sys::core::HRESULT);
windows_link::link!("certadm.dll" "system" fn CertSrvRestoreRegisterThroughFile(hbc : HCSBC, pwszcheckpointfilepath : *const u16, pwszlogpath : *const u16, rgrstmap : *mut CSEDB_RSTMAPW, crstmap : i32, pwszbackuplogpath : *const u16, genlow : u32, genhigh : u32) -> windows_sys::core::HRESULT);
windows_link::link!("certadm.dll" "system" fn CertSrvRestoreRegisterW(hbc : HCSBC, pwszcheckpointfilepath : *const u16, pwszlogpath : *const u16, rgrstmap : *mut CSEDB_RSTMAPW, crstmap : i32, pwszbackuplogpath : *const u16, genlow : u32, genhigh : u32) -> windows_sys::core::HRESULT);
windows_link::link!("certadm.dll" "system" fn CertSrvServerControlW(pwszservername : *const u16, dwcontrolflags : u32, pcbout : *mut u32, ppbout : *mut *mut u8) -> windows_sys::core::HRESULT);
pub const CSBACKUP_DISABLE_INCREMENTAL: u32 = 4294967295;
pub const CSBACKUP_TYPE_FULL: u32 = 1;
pub const CSBACKUP_TYPE_LOGS_ONLY: u32 = 2;
pub const CSBACKUP_TYPE_MASK: u32 = 3;
pub type CSBFT = u16;
pub const CSBFT_CERTSERVER_DATABASE: u32 = 68;
pub const CSBFT_CHECKPOINT_DIR: u32 = 131;
pub const CSBFT_DATABASE_DIRECTORY: u32 = 64;
pub const CSBFT_DIRECTORY: u32 = 128;
pub const CSBFT_LOG: u32 = 33;
pub const CSBFT_LOG_DIR: u32 = 130;
pub const CSBFT_LOG_DIRECTORY: u32 = 32;
pub const CSBFT_PATCH_FILE: u32 = 37;
pub const CSBFT_UNKNOWN: u32 = 15;
pub const CSCONTROL_RESTART: u32 = 3;
pub const CSCONTROL_SHUTDOWN: u32 = 1;
pub const CSCONTROL_SUSPEND: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CSEDB_RSTMAPW {
    pub pwszDatabaseName: *mut u16,
    pub pwszNewDatabaseName: *mut u16,
}
impl Default for CSEDB_RSTMAPW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CSRESTORE_TYPE_CATCHUP: u32 = 4;
pub const CSRESTORE_TYPE_FULL: u32 = 1;
pub const CSRESTORE_TYPE_MASK: u32 = 5;
pub const CSRESTORE_TYPE_ONLINE: u32 = 2;
pub type FNCERTSRVBACKUPCLOSE = Option<unsafe extern "system" fn(hbc: HCSBC) -> windows_sys::core::HRESULT>;
pub type FNCERTSRVBACKUPEND = Option<unsafe extern "system" fn(hbc: HCSBC) -> windows_sys::core::HRESULT>;
pub type FNCERTSRVBACKUPFREE = Option<unsafe extern "system" fn(pv: *mut core::ffi::c_void)>;
pub type FNCERTSRVBACKUPGETBACKUPLOGSW = Option<unsafe extern "system" fn(hbc: HCSBC, ppwszzbackuplogfiles: *mut *mut u16, pcbsize: *mut u32) -> windows_sys::core::HRESULT>;
pub type FNCERTSRVBACKUPGETDATABASENAMESW = Option<unsafe extern "system" fn(hbc: HCSBC, ppwszzattachmentinformation: *mut *mut u16, pcbsize: *mut u32) -> windows_sys::core::HRESULT>;
pub type FNCERTSRVBACKUPGETDYNAMICFILELISTW = Option<unsafe extern "system" fn(hbc: HCSBC, ppwszzfilelist: *mut *mut u16, pcbsize: *mut u32) -> windows_sys::core::HRESULT>;
pub type FNCERTSRVBACKUPOPENFILEW = Option<unsafe extern "system" fn(hbc: HCSBC, pwszattachmentname: *const u16, cbreadhintsize: u32, plifilesize: *mut i64) -> windows_sys::core::HRESULT>;
pub type FNCERTSRVBACKUPPREPAREW = Option<unsafe extern "system" fn(pwszservername: *const u16, grbitjet: u32, dwbackupflags: u32, phbc: *mut HCSBC) -> windows_sys::core::HRESULT>;
pub type FNCERTSRVBACKUPREAD = Option<unsafe extern "system" fn(hbc: HCSBC, pvbuffer: *mut core::ffi::c_void, cbbuffer: u32, pcbread: *mut u32) -> windows_sys::core::HRESULT>;
pub type FNCERTSRVBACKUPTRUNCATELOGS = Option<unsafe extern "system" fn(hbc: HCSBC) -> windows_sys::core::HRESULT>;
pub type FNCERTSRVISSERVERONLINEW = Option<unsafe extern "system" fn(pwszservername: *const u16, pfserveronline: *mut windows_sys::core::BOOL) -> windows_sys::core::HRESULT>;
pub type FNCERTSRVRESTOREEND = Option<unsafe extern "system" fn(hbc: HCSBC) -> windows_sys::core::HRESULT>;
pub type FNCERTSRVRESTOREGETDATABASELOCATIONSW = Option<unsafe extern "system" fn(hbc: HCSBC, ppwszzdatabaselocationlist: *mut *mut u16, pcbsize: *mut u32) -> windows_sys::core::HRESULT>;
pub type FNCERTSRVRESTOREPREPAREW = Option<unsafe extern "system" fn(pwszservername: *const u16, dwrestoreflags: u32, phbc: *mut HCSBC) -> windows_sys::core::HRESULT>;
pub type FNCERTSRVRESTOREREGISTERCOMPLETE = Option<unsafe extern "system" fn(hbc: HCSBC, hrrestorestate: windows_sys::core::HRESULT) -> windows_sys::core::HRESULT>;
pub type FNCERTSRVRESTOREREGISTERW = Option<unsafe extern "system" fn(hbc: HCSBC, pwszcheckpointfilepath: *const u16, pwszlogpath: *const u16, rgrstmap: *mut CSEDB_RSTMAPW, crstmap: i32, pwszbackuplogpath: *const u16, genlow: u32, genhigh: u32) -> windows_sys::core::HRESULT>;
pub type FNCERTSRVSERVERCONTROLW = Option<unsafe extern "system" fn(pwszservername: *const u16, dwcontrolflags: u32, pcbout: *mut u32, ppbout: *mut *mut u8) -> windows_sys::core::HRESULT>;
pub type HCSBC = *mut core::ffi::c_void;
pub const szBACKUPANNOTATION: windows_sys::core::PCSTR = windows_sys::core::s!("Cert Server Backup Interface");
pub const szRESTOREANNOTATION: windows_sys::core::PCSTR = windows_sys::core::s!("Cert Server Restore Interface");
