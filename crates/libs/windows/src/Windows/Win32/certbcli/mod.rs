#[inline]
pub unsafe fn CertSrvBackupClose(hbc: HCSBC) -> windows_core::HRESULT {
    windows_core::link!("certadm.dll" "system" fn CertSrvBackupClose(hbc : HCSBC) -> windows_core::HRESULT);
    unsafe { CertSrvBackupClose(hbc as _) }
}
#[inline]
pub unsafe fn CertSrvBackupEnd(hbc: HCSBC) -> windows_core::HRESULT {
    windows_core::link!("certadm.dll" "system" fn CertSrvBackupEnd(hbc : HCSBC) -> windows_core::HRESULT);
    unsafe { CertSrvBackupEnd(hbc as _) }
}
#[inline]
pub unsafe fn CertSrvBackupFree(pv: *mut core::ffi::c_void) {
    windows_core::link!("certadm.dll" "system" fn CertSrvBackupFree(pv : *mut core::ffi::c_void));
    unsafe { CertSrvBackupFree(pv as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CertSrvBackupGetBackupLogs(hbc: HCSBC, ppwszzbackuplogfiles: *mut windows_core::PWSTR, pcbsize: *mut super::DWORD) -> windows_core::HRESULT {
    windows_core::link!("certadm.dll" "system" "CertSrvBackupGetBackupLogsW" fn CertSrvBackupGetBackupLogs(hbc : HCSBC, ppwszzbackuplogfiles : *mut windows_core::PWSTR, pcbsize : *mut super::DWORD) -> windows_core::HRESULT);
    unsafe { CertSrvBackupGetBackupLogs(hbc, ppwszzbackuplogfiles as _, pcbsize as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CertSrvBackupGetBackupLogsW(hbc: HCSBC, ppwszzbackuplogfiles: *mut windows_core::PWSTR, pcbsize: *mut super::DWORD) -> windows_core::HRESULT {
    windows_core::link!("certadm.dll" "system" fn CertSrvBackupGetBackupLogsW(hbc : HCSBC, ppwszzbackuplogfiles : *mut windows_core::PWSTR, pcbsize : *mut super::DWORD) -> windows_core::HRESULT);
    unsafe { CertSrvBackupGetBackupLogsW(hbc, ppwszzbackuplogfiles as _, pcbsize as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CertSrvBackupGetDatabaseNames(hbc: HCSBC, ppwszzattachmentinformation: *mut windows_core::PWSTR, pcbsize: *mut super::DWORD) -> windows_core::HRESULT {
    windows_core::link!("certadm.dll" "system" "CertSrvBackupGetDatabaseNamesW" fn CertSrvBackupGetDatabaseNames(hbc : HCSBC, ppwszzattachmentinformation : *mut windows_core::PWSTR, pcbsize : *mut super::DWORD) -> windows_core::HRESULT);
    unsafe { CertSrvBackupGetDatabaseNames(hbc, ppwszzattachmentinformation as _, pcbsize as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CertSrvBackupGetDatabaseNamesW(hbc: HCSBC, ppwszzattachmentinformation: *mut windows_core::PWSTR, pcbsize: *mut super::DWORD) -> windows_core::HRESULT {
    windows_core::link!("certadm.dll" "system" fn CertSrvBackupGetDatabaseNamesW(hbc : HCSBC, ppwszzattachmentinformation : *mut windows_core::PWSTR, pcbsize : *mut super::DWORD) -> windows_core::HRESULT);
    unsafe { CertSrvBackupGetDatabaseNamesW(hbc, ppwszzattachmentinformation as _, pcbsize as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CertSrvBackupGetDynamicFileList(hbc: HCSBC, ppwszzfilelist: *mut windows_core::PWSTR, pcbsize: *mut super::DWORD) -> windows_core::HRESULT {
    windows_core::link!("certadm.dll" "system" "CertSrvBackupGetDynamicFileListW" fn CertSrvBackupGetDynamicFileList(hbc : HCSBC, ppwszzfilelist : *mut windows_core::PWSTR, pcbsize : *mut super::DWORD) -> windows_core::HRESULT);
    unsafe { CertSrvBackupGetDynamicFileList(hbc, ppwszzfilelist as _, pcbsize as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CertSrvBackupGetDynamicFileListW(hbc: HCSBC, ppwszzfilelist: *mut windows_core::PWSTR, pcbsize: *mut super::DWORD) -> windows_core::HRESULT {
    windows_core::link!("certadm.dll" "system" fn CertSrvBackupGetDynamicFileListW(hbc : HCSBC, ppwszzfilelist : *mut windows_core::PWSTR, pcbsize : *mut super::DWORD) -> windows_core::HRESULT);
    unsafe { CertSrvBackupGetDynamicFileListW(hbc, ppwszzfilelist as _, pcbsize as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CertSrvBackupOpenFile(hbc: HCSBC, pwszattachmentname: *const super::WCHAR, cbreadhintsize: super::DWORD, plifilesize: *mut super::LARGE_INTEGER) -> windows_core::HRESULT {
    windows_core::link!("certadm.dll" "system" "CertSrvBackupOpenFileW" fn CertSrvBackupOpenFile(hbc : HCSBC, pwszattachmentname : *const super::WCHAR, cbreadhintsize : super::DWORD, plifilesize : *mut super::LARGE_INTEGER) -> windows_core::HRESULT);
    unsafe { CertSrvBackupOpenFile(hbc as _, pwszattachmentname, cbreadhintsize, plifilesize as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CertSrvBackupOpenFileW(hbc: HCSBC, pwszattachmentname: *const super::WCHAR, cbreadhintsize: super::DWORD, plifilesize: *mut super::LARGE_INTEGER) -> windows_core::HRESULT {
    windows_core::link!("certadm.dll" "system" fn CertSrvBackupOpenFileW(hbc : HCSBC, pwszattachmentname : *const super::WCHAR, cbreadhintsize : super::DWORD, plifilesize : *mut super::LARGE_INTEGER) -> windows_core::HRESULT);
    unsafe { CertSrvBackupOpenFileW(hbc as _, pwszattachmentname, cbreadhintsize, plifilesize as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CertSrvBackupPrepare(pwszservername: *const super::WCHAR, grbitjet: super::ULONG, dwbackupflags: super::ULONG, phbc: *mut HCSBC) -> windows_core::HRESULT {
    windows_core::link!("certadm.dll" "system" "CertSrvBackupPrepareW" fn CertSrvBackupPrepare(pwszservername : *const super::WCHAR, grbitjet : super::ULONG, dwbackupflags : super::ULONG, phbc : *mut HCSBC) -> windows_core::HRESULT);
    unsafe { CertSrvBackupPrepare(pwszservername, grbitjet, dwbackupflags, phbc as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CertSrvBackupPrepareW(pwszservername: *const super::WCHAR, grbitjet: super::ULONG, dwbackupflags: super::ULONG, phbc: *mut HCSBC) -> windows_core::HRESULT {
    windows_core::link!("certadm.dll" "system" fn CertSrvBackupPrepareW(pwszservername : *const super::WCHAR, grbitjet : super::ULONG, dwbackupflags : super::ULONG, phbc : *mut HCSBC) -> windows_core::HRESULT);
    unsafe { CertSrvBackupPrepareW(pwszservername, grbitjet, dwbackupflags, phbc as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CertSrvBackupRead(hbc: HCSBC, pvbuffer: *mut core::ffi::c_void, cbbuffer: super::DWORD, pcbread: *mut super::DWORD) -> windows_core::HRESULT {
    windows_core::link!("certadm.dll" "system" fn CertSrvBackupRead(hbc : HCSBC, pvbuffer : *mut core::ffi::c_void, cbbuffer : super::DWORD, pcbread : *mut super::DWORD) -> windows_core::HRESULT);
    unsafe { CertSrvBackupRead(hbc as _, pvbuffer as _, cbbuffer, pcbread as _) }
}
#[inline]
pub unsafe fn CertSrvBackupTruncateLogs(hbc: HCSBC) -> windows_core::HRESULT {
    windows_core::link!("certadm.dll" "system" fn CertSrvBackupTruncateLogs(hbc : HCSBC) -> windows_core::HRESULT);
    unsafe { CertSrvBackupTruncateLogs(hbc as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CertSrvIsServerOnline(pwszservername: *const super::WCHAR, pfserveronline: *mut windows_core::BOOL) -> windows_core::HRESULT {
    windows_core::link!("certadm.dll" "system" "CertSrvIsServerOnlineW" fn CertSrvIsServerOnline(pwszservername : *const super::WCHAR, pfserveronline : *mut windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { CertSrvIsServerOnline(pwszservername, pfserveronline as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CertSrvIsServerOnlineW(pwszservername: *const super::WCHAR, pfserveronline: *mut windows_core::BOOL) -> windows_core::HRESULT {
    windows_core::link!("certadm.dll" "system" fn CertSrvIsServerOnlineW(pwszservername : *const super::WCHAR, pfserveronline : *mut windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { CertSrvIsServerOnlineW(pwszservername, pfserveronline as _) }
}
#[inline]
pub unsafe fn CertSrvRestoreEnd(hbc: HCSBC) -> windows_core::HRESULT {
    windows_core::link!("certadm.dll" "system" fn CertSrvRestoreEnd(hbc : HCSBC) -> windows_core::HRESULT);
    unsafe { CertSrvRestoreEnd(hbc as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CertSrvRestoreGetDatabaseLocations(hbc: HCSBC, ppwszzdatabaselocationlist: *mut windows_core::PWSTR, pcbsize: *mut super::DWORD) -> windows_core::HRESULT {
    windows_core::link!("certadm.dll" "system" "CertSrvRestoreGetDatabaseLocationsW" fn CertSrvRestoreGetDatabaseLocations(hbc : HCSBC, ppwszzdatabaselocationlist : *mut windows_core::PWSTR, pcbsize : *mut super::DWORD) -> windows_core::HRESULT);
    unsafe { CertSrvRestoreGetDatabaseLocations(hbc, ppwszzdatabaselocationlist as _, pcbsize as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CertSrvRestoreGetDatabaseLocationsW(hbc: HCSBC, ppwszzdatabaselocationlist: *mut windows_core::PWSTR, pcbsize: *mut super::DWORD) -> windows_core::HRESULT {
    windows_core::link!("certadm.dll" "system" fn CertSrvRestoreGetDatabaseLocationsW(hbc : HCSBC, ppwszzdatabaselocationlist : *mut windows_core::PWSTR, pcbsize : *mut super::DWORD) -> windows_core::HRESULT);
    unsafe { CertSrvRestoreGetDatabaseLocationsW(hbc, ppwszzdatabaselocationlist as _, pcbsize as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CertSrvRestorePrepare(pwszservername: *const super::WCHAR, dwrestoreflags: super::ULONG, phbc: *mut HCSBC) -> windows_core::HRESULT {
    windows_core::link!("certadm.dll" "system" "CertSrvRestorePrepareW" fn CertSrvRestorePrepare(pwszservername : *const super::WCHAR, dwrestoreflags : super::ULONG, phbc : *mut HCSBC) -> windows_core::HRESULT);
    unsafe { CertSrvRestorePrepare(pwszservername, dwrestoreflags, phbc as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CertSrvRestorePrepareW(pwszservername: *const super::WCHAR, dwrestoreflags: super::ULONG, phbc: *mut HCSBC) -> windows_core::HRESULT {
    windows_core::link!("certadm.dll" "system" fn CertSrvRestorePrepareW(pwszservername : *const super::WCHAR, dwrestoreflags : super::ULONG, phbc : *mut HCSBC) -> windows_core::HRESULT);
    unsafe { CertSrvRestorePrepareW(pwszservername, dwrestoreflags, phbc as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CertSrvRestoreRegister(hbc: HCSBC, pwszcheckpointfilepath: *const super::WCHAR, pwszlogpath: *const super::WCHAR, rgrstmap: *mut CSEDB_RSTMAPW, crstmap: super::LONG, pwszbackuplogpath: *const super::WCHAR, genlow: super::ULONG, genhigh: super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("certadm.dll" "system" "CertSrvRestoreRegisterW" fn CertSrvRestoreRegister(hbc : HCSBC, pwszcheckpointfilepath : *const super::WCHAR, pwszlogpath : *const super::WCHAR, rgrstmap : *mut CSEDB_RSTMAPW, crstmap : super::LONG, pwszbackuplogpath : *const super::WCHAR, genlow : super::ULONG, genhigh : super::ULONG) -> windows_core::HRESULT);
    unsafe { CertSrvRestoreRegister(hbc as _, pwszcheckpointfilepath, pwszlogpath, rgrstmap as _, crstmap, pwszbackuplogpath, genlow, genhigh) }
}
#[inline]
pub unsafe fn CertSrvRestoreRegisterComplete(hbc: HCSBC, hrrestorestate: windows_core::HRESULT) -> windows_core::HRESULT {
    windows_core::link!("certadm.dll" "system" fn CertSrvRestoreRegisterComplete(hbc : HCSBC, hrrestorestate : windows_core::HRESULT) -> windows_core::HRESULT);
    unsafe { CertSrvRestoreRegisterComplete(hbc as _, hrrestorestate) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CertSrvRestoreRegisterThroughFile(hbc: HCSBC, pwszcheckpointfilepath: *const super::WCHAR, pwszlogpath: *const super::WCHAR, rgrstmap: *mut CSEDB_RSTMAPW, crstmap: super::LONG, pwszbackuplogpath: *const super::WCHAR, genlow: super::ULONG, genhigh: super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("certadm.dll" "system" fn CertSrvRestoreRegisterThroughFile(hbc : HCSBC, pwszcheckpointfilepath : *const super::WCHAR, pwszlogpath : *const super::WCHAR, rgrstmap : *mut CSEDB_RSTMAPW, crstmap : super::LONG, pwszbackuplogpath : *const super::WCHAR, genlow : super::ULONG, genhigh : super::ULONG) -> windows_core::HRESULT);
    unsafe { CertSrvRestoreRegisterThroughFile(hbc as _, pwszcheckpointfilepath, pwszlogpath, rgrstmap as _, crstmap, pwszbackuplogpath, genlow, genhigh) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CertSrvRestoreRegisterW(hbc: HCSBC, pwszcheckpointfilepath: *const super::WCHAR, pwszlogpath: *const super::WCHAR, rgrstmap: *mut CSEDB_RSTMAPW, crstmap: super::LONG, pwszbackuplogpath: *const super::WCHAR, genlow: super::ULONG, genhigh: super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("certadm.dll" "system" fn CertSrvRestoreRegisterW(hbc : HCSBC, pwszcheckpointfilepath : *const super::WCHAR, pwszlogpath : *const super::WCHAR, rgrstmap : *mut CSEDB_RSTMAPW, crstmap : super::LONG, pwszbackuplogpath : *const super::WCHAR, genlow : super::ULONG, genhigh : super::ULONG) -> windows_core::HRESULT);
    unsafe { CertSrvRestoreRegisterW(hbc as _, pwszcheckpointfilepath, pwszlogpath, rgrstmap as _, crstmap, pwszbackuplogpath, genlow, genhigh) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CertSrvServerControl(pwszservername: *const super::WCHAR, dwcontrolflags: super::DWORD, pcbout: *mut super::DWORD, ppbout: *mut *mut super::BYTE) -> windows_core::HRESULT {
    windows_core::link!("certadm.dll" "system" "CertSrvServerControlW" fn CertSrvServerControl(pwszservername : *const super::WCHAR, dwcontrolflags : super::DWORD, pcbout : *mut super::DWORD, ppbout : *mut *mut super::BYTE) -> windows_core::HRESULT);
    unsafe { CertSrvServerControl(pwszservername, dwcontrolflags, pcbout as _, ppbout as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CertSrvServerControlW(pwszservername: *const super::WCHAR, dwcontrolflags: super::DWORD, pcbout: *mut super::DWORD, ppbout: *mut *mut super::BYTE) -> windows_core::HRESULT {
    windows_core::link!("certadm.dll" "system" fn CertSrvServerControlW(pwszservername : *const super::WCHAR, dwcontrolflags : super::DWORD, pcbout : *mut super::DWORD, ppbout : *mut *mut super::BYTE) -> windows_core::HRESULT);
    unsafe { CertSrvServerControlW(pwszservername, dwcontrolflags, pcbout as _, ppbout as _) }
}
pub const CSBACKUP_DISABLE_INCREMENTAL: u32 = 4294967295;
pub const CSBACKUP_TYPE_FULL: i32 = 1;
pub const CSBACKUP_TYPE_LOGS_ONLY: i32 = 2;
pub const CSBACKUP_TYPE_MASK: i32 = 3;
pub type CSBFT = u16;
pub const CSBFT_CERTSERVER_DATABASE: CSBFT = 68;
pub const CSBFT_CHECKPOINT_DIR: CSBFT = 131;
pub const CSBFT_DATABASE_DIRECTORY: i32 = 64;
pub const CSBFT_DIRECTORY: i32 = 128;
pub const CSBFT_LOG: CSBFT = 33;
pub const CSBFT_LOG_DIR: CSBFT = 130;
pub const CSBFT_LOG_DIRECTORY: i32 = 32;
pub const CSBFT_PATCH_FILE: CSBFT = 37;
pub const CSBFT_UNKNOWN: CSBFT = 15;
pub const CSCONTROL_RESTART: i32 = 3;
pub const CSCONTROL_SHUTDOWN: i32 = 1;
pub const CSCONTROL_SUSPEND: i32 = 2;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CSEDB_RSTMAPW {
    pub pwszDatabaseName: *mut super::WCHAR,
    pub pwszNewDatabaseName: *mut super::WCHAR,
}
pub const CSRESTORE_TYPE_CATCHUP: i32 = 4;
pub const CSRESTORE_TYPE_FULL: i32 = 1;
pub const CSRESTORE_TYPE_MASK: i32 = 5;
pub const CSRESTORE_TYPE_ONLINE: i32 = 2;
pub type FNCERTSRVBACKUPCLOSE = Option<unsafe extern "system" fn(hbc: HCSBC) -> windows_core::HRESULT>;
pub type FNCERTSRVBACKUPEND = Option<unsafe extern "system" fn(hbc: HCSBC) -> windows_core::HRESULT>;
pub type FNCERTSRVBACKUPFREE = Option<unsafe extern "system" fn(pv: *mut core::ffi::c_void)>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type FNCERTSRVBACKUPGETBACKUPLOGSW = Option<unsafe extern "system" fn(hbc: HCSBC, ppwszzbackuplogfiles: *mut *mut super::WCHAR, pcbsize: *mut super::DWORD) -> windows_core::HRESULT>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type FNCERTSRVBACKUPGETDATABASENAMESW = Option<unsafe extern "system" fn(hbc: HCSBC, ppwszzattachmentinformation: *mut *mut super::WCHAR, pcbsize: *mut super::DWORD) -> windows_core::HRESULT>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type FNCERTSRVBACKUPGETDYNAMICFILELISTW = Option<unsafe extern "system" fn(hbc: HCSBC, ppwszzfilelist: *mut *mut super::WCHAR, pcbsize: *mut super::DWORD) -> windows_core::HRESULT>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type FNCERTSRVBACKUPOPENFILEW = Option<unsafe extern "system" fn(hbc: HCSBC, pwszattachmentname: *const super::WCHAR, cbreadhintsize: super::DWORD, plifilesize: *mut super::LARGE_INTEGER) -> windows_core::HRESULT>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type FNCERTSRVBACKUPPREPAREW = Option<unsafe extern "system" fn(pwszservername: *const super::WCHAR, grbitjet: super::ULONG, dwbackupflags: super::ULONG, phbc: *mut HCSBC) -> windows_core::HRESULT>;
#[cfg(feature = "shared")]
pub type FNCERTSRVBACKUPREAD = Option<unsafe extern "system" fn(hbc: HCSBC, pvbuffer: *mut core::ffi::c_void, cbbuffer: super::DWORD, pcbread: *mut super::DWORD) -> windows_core::HRESULT>;
pub type FNCERTSRVBACKUPTRUNCATELOGS = Option<unsafe extern "system" fn(hbc: HCSBC) -> windows_core::HRESULT>;
#[cfg(feature = "winnt")]
pub type FNCERTSRVISSERVERONLINEW = Option<unsafe extern "system" fn(pwszservername: *const super::WCHAR, pfserveronline: *mut windows_core::BOOL) -> windows_core::HRESULT>;
pub type FNCERTSRVRESTOREEND = Option<unsafe extern "system" fn(hbc: HCSBC) -> windows_core::HRESULT>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type FNCERTSRVRESTOREGETDATABASELOCATIONSW = Option<unsafe extern "system" fn(hbc: HCSBC, ppwszzdatabaselocationlist: *mut *mut super::WCHAR, pcbsize: *mut super::DWORD) -> windows_core::HRESULT>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type FNCERTSRVRESTOREPREPAREW = Option<unsafe extern "system" fn(pwszservername: *const super::WCHAR, dwrestoreflags: super::ULONG, phbc: *mut HCSBC) -> windows_core::HRESULT>;
pub type FNCERTSRVRESTOREREGISTERCOMPLETE = Option<unsafe extern "system" fn(hbc: HCSBC, hrrestorestate: windows_core::HRESULT) -> windows_core::HRESULT>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type FNCERTSRVRESTOREREGISTERW = Option<unsafe extern "system" fn(hbc: HCSBC, pwszcheckpointfilepath: *const super::WCHAR, pwszlogpath: *const super::WCHAR, rgrstmap: *mut CSEDB_RSTMAPW, crstmap: super::LONG, pwszbackuplogpath: *const super::WCHAR, genlow: super::ULONG, genhigh: super::ULONG) -> windows_core::HRESULT>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type FNCERTSRVSERVERCONTROLW = Option<unsafe extern "system" fn(pwszservername: *const super::WCHAR, dwcontrolflags: super::DWORD, pcbout: *mut super::DWORD, ppbout: *mut *mut super::BYTE) -> windows_core::HRESULT>;
pub type HCSBC = *mut core::ffi::c_void;
pub const szBACKUPANNOTATION: windows_core::PCSTR = windows_core::s!("Cert Server Backup Interface");
pub const szRESTOREANNOTATION: windows_core::PCSTR = windows_core::s!("Cert Server Restore Interface");
pub const wszBACKUPANNOTATION: windows_core::PCSTR = windows_core::s!("Cert Server Backup Interface");
pub const wszRESTOREANNOTATION: windows_core::PCSTR = windows_core::s!("Cert Server Restore Interface");
