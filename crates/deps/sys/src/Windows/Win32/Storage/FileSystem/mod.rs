#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddLogContainer(hlog: super::super::Foundation::HANDLE, pcbcontainer: *const u64, pwszcontainerpath: super::super::Foundation::PWSTR, preserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddLogContainerSet(hlog: super::super::Foundation::HANDLE, ccontainer: u16, pcbcontainer: *const u64, rgwszcontainerpath: *const super::super::Foundation::PWSTR, preserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn AddUsersToEncryptedFile(lpfilename: super::super::Foundation::PWSTR, pencryptioncertificates: *const ENCRYPTION_CERTIFICATE_LIST) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn AdvanceLogBase(pvmarshal: *mut ::core::ffi::c_void, plsnbase: *mut CLS_LSN, fflags: u32, poverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AlignReservedLog(pvmarshal: *mut ::core::ffi::c_void, creservedrecords: u32, rgcbreservation: *mut i64, pcbalignreservation: *mut i64) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AllocReservedLog(pvmarshal: *mut ::core::ffi::c_void, creservedrecords: u32, pcbadjustment: *mut i64) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AreFileApisANSI() -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AreShortNamesEnabled(handle: super::super::Foundation::HANDLE, enabled: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BackupRead(hfile: super::super::Foundation::HANDLE, lpbuffer: *mut u8, nnumberofbytestoread: u32, lpnumberofbytesread: *mut u32, babort: super::super::Foundation::BOOL, bprocesssecurity: super::super::Foundation::BOOL, lpcontext: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BackupSeek(hfile: super::super::Foundation::HANDLE, dwlowbytestoseek: u32, dwhighbytestoseek: u32, lpdwlowbyteseeked: *mut u32, lpdwhighbyteseeked: *mut u32, lpcontext: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BackupWrite(hfile: super::super::Foundation::HANDLE, lpbuffer: *const u8, nnumberofbytestowrite: u32, lpnumberofbyteswritten: *mut u32, babort: super::super::Foundation::BOOL, bprocesssecurity: super::super::Foundation::BOOL, lpcontext: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BuildIoRingCancelRequest(ioring: *const HIORING__, file: IORING_HANDLE_REF, optocancel: usize, userdata: usize) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BuildIoRingReadFile(ioring: *const HIORING__, fileref: IORING_HANDLE_REF, dataref: IORING_BUFFER_REF, numberofbytestoread: u32, fileoffset: u64, userdata: usize, flags: IORING_SQE_FLAGS) -> ::windows_sys::core::HRESULT;
    pub fn BuildIoRingRegisterBuffers(ioring: *const HIORING__, count: u32, buffers: *const IORING_BUFFER_INFO, userdata: usize) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BuildIoRingRegisterFileHandles(ioring: *const HIORING__, count: u32, handles: *const super::super::Foundation::HANDLE, userdata: usize) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CheckNameLegalDOS8Dot3A(lpname: super::super::Foundation::PSTR, lpoemname: super::super::Foundation::PSTR, oemnamesize: u32, pbnamecontainsspaces: *mut super::super::Foundation::BOOL, pbnamelegal: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CheckNameLegalDOS8Dot3W(lpname: super::super::Foundation::PWSTR, lpoemname: super::super::Foundation::PSTR, oemnamesize: u32, pbnamecontainsspaces: *mut super::super::Foundation::BOOL, pbnamelegal: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CloseAndResetLogFile(hlog: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    pub fn CloseEncryptedFileRaw(pvcontext: *const ::core::ffi::c_void);
    pub fn CloseIoRing(ioring: *const HIORING__) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CommitComplete(enlistmenthandle: super::super::Foundation::HANDLE, tmvirtualclock: *mut i64) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CommitEnlistment(enlistmenthandle: super::super::Foundation::HANDLE, tmvirtualclock: *mut i64) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CommitTransaction(transactionhandle: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CommitTransactionAsync(transactionhandle: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CompareFileTime(lpfiletime1: *const super::super::Foundation::FILETIME, lpfiletime2: *const super::super::Foundation::FILETIME) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CopyFile2(pwszexistingfilename: super::super::Foundation::PWSTR, pwsznewfilename: super::super::Foundation::PWSTR, pextendedparameters: *const COPYFILE2_EXTENDED_PARAMETERS) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CopyFileA(lpexistingfilename: super::super::Foundation::PSTR, lpnewfilename: super::super::Foundation::PSTR, bfailifexists: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CopyFileExA(lpexistingfilename: super::super::Foundation::PSTR, lpnewfilename: super::super::Foundation::PSTR, lpprogressroutine: LPPROGRESS_ROUTINE, lpdata: *const ::core::ffi::c_void, pbcancel: *mut i32, dwcopyflags: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CopyFileExW(lpexistingfilename: super::super::Foundation::PWSTR, lpnewfilename: super::super::Foundation::PWSTR, lpprogressroutine: LPPROGRESS_ROUTINE, lpdata: *const ::core::ffi::c_void, pbcancel: *mut i32, dwcopyflags: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CopyFileFromAppW(lpexistingfilename: super::super::Foundation::PWSTR, lpnewfilename: super::super::Foundation::PWSTR, bfailifexists: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CopyFileTransactedA(lpexistingfilename: super::super::Foundation::PSTR, lpnewfilename: super::super::Foundation::PSTR, lpprogressroutine: LPPROGRESS_ROUTINE, lpdata: *const ::core::ffi::c_void, pbcancel: *const i32, dwcopyflags: u32, htransaction: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CopyFileTransactedW(lpexistingfilename: super::super::Foundation::PWSTR, lpnewfilename: super::super::Foundation::PWSTR, lpprogressroutine: LPPROGRESS_ROUTINE, lpdata: *const ::core::ffi::c_void, pbcancel: *const i32, dwcopyflags: u32, htransaction: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CopyFileW(lpexistingfilename: super::super::Foundation::PWSTR, lpnewfilename: super::super::Foundation::PWSTR, bfailifexists: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    pub fn CopyLZFile(hfsource: i32, hfdest: i32) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateDirectoryA(lppathname: super::super::Foundation::PSTR, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateDirectoryExA(lptemplatedirectory: super::super::Foundation::PSTR, lpnewdirectory: super::super::Foundation::PSTR, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateDirectoryExW(lptemplatedirectory: super::super::Foundation::PWSTR, lpnewdirectory: super::super::Foundation::PWSTR, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateDirectoryFromAppW(lppathname: super::super::Foundation::PWSTR, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateDirectoryTransactedA(lptemplatedirectory: super::super::Foundation::PSTR, lpnewdirectory: super::super::Foundation::PSTR, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, htransaction: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateDirectoryTransactedW(lptemplatedirectory: super::super::Foundation::PWSTR, lpnewdirectory: super::super::Foundation::PWSTR, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, htransaction: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateDirectoryW(lppathname: super::super::Foundation::PWSTR, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateEnlistment(lpenlistmentattributes: *mut super::super::Security::SECURITY_ATTRIBUTES, resourcemanagerhandle: super::super::Foundation::HANDLE, transactionhandle: super::super::Foundation::HANDLE, notificationmask: u32, createoptions: u32, enlistmentkey: *mut ::core::ffi::c_void) -> super::super::Foundation::HANDLE;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateFile2(lpfilename: super::super::Foundation::PWSTR, dwdesiredaccess: FILE_ACCESS_FLAGS, dwsharemode: FILE_SHARE_MODE, dwcreationdisposition: FILE_CREATION_DISPOSITION, pcreateexparams: *const CREATEFILE2_EXTENDED_PARAMETERS) -> super::super::Foundation::HANDLE;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateFile2FromAppW(lpfilename: super::super::Foundation::PWSTR, dwdesiredaccess: u32, dwsharemode: u32, dwcreationdisposition: u32, pcreateexparams: *const CREATEFILE2_EXTENDED_PARAMETERS) -> super::super::Foundation::HANDLE;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateFileA(lpfilename: super::super::Foundation::PSTR, dwdesiredaccess: FILE_ACCESS_FLAGS, dwsharemode: FILE_SHARE_MODE, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, dwcreationdisposition: FILE_CREATION_DISPOSITION, dwflagsandattributes: FILE_FLAGS_AND_ATTRIBUTES, htemplatefile: super::super::Foundation::HANDLE) -> super::super::Foundation::HANDLE;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateFileFromAppW(lpfilename: super::super::Foundation::PWSTR, dwdesiredaccess: u32, dwsharemode: u32, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, dwcreationdisposition: u32, dwflagsandattributes: u32, htemplatefile: super::super::Foundation::HANDLE) -> super::super::Foundation::HANDLE;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateFileTransactedA(
        lpfilename: super::super::Foundation::PSTR,
        dwdesiredaccess: u32,
        dwsharemode: FILE_SHARE_MODE,
        lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
        dwcreationdisposition: FILE_CREATION_DISPOSITION,
        dwflagsandattributes: FILE_FLAGS_AND_ATTRIBUTES,
        htemplatefile: super::super::Foundation::HANDLE,
        htransaction: super::super::Foundation::HANDLE,
        pusminiversion: *const TXFS_MINIVERSION,
        lpextendedparameter: *mut ::core::ffi::c_void,
    ) -> super::super::Foundation::HANDLE;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateFileTransactedW(
        lpfilename: super::super::Foundation::PWSTR,
        dwdesiredaccess: u32,
        dwsharemode: FILE_SHARE_MODE,
        lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
        dwcreationdisposition: FILE_CREATION_DISPOSITION,
        dwflagsandattributes: FILE_FLAGS_AND_ATTRIBUTES,
        htemplatefile: super::super::Foundation::HANDLE,
        htransaction: super::super::Foundation::HANDLE,
        pusminiversion: *const TXFS_MINIVERSION,
        lpextendedparameter: *mut ::core::ffi::c_void,
    ) -> super::super::Foundation::HANDLE;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateFileW(lpfilename: super::super::Foundation::PWSTR, dwdesiredaccess: FILE_ACCESS_FLAGS, dwsharemode: FILE_SHARE_MODE, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, dwcreationdisposition: FILE_CREATION_DISPOSITION, dwflagsandattributes: FILE_FLAGS_AND_ATTRIBUTES, htemplatefile: super::super::Foundation::HANDLE) -> super::super::Foundation::HANDLE;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateHardLinkA(lpfilename: super::super::Foundation::PSTR, lpexistingfilename: super::super::Foundation::PSTR, lpsecurityattributes: *mut super::super::Security::SECURITY_ATTRIBUTES) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateHardLinkTransactedA(lpfilename: super::super::Foundation::PSTR, lpexistingfilename: super::super::Foundation::PSTR, lpsecurityattributes: *mut super::super::Security::SECURITY_ATTRIBUTES, htransaction: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateHardLinkTransactedW(lpfilename: super::super::Foundation::PWSTR, lpexistingfilename: super::super::Foundation::PWSTR, lpsecurityattributes: *mut super::super::Security::SECURITY_ATTRIBUTES, htransaction: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateHardLinkW(lpfilename: super::super::Foundation::PWSTR, lpexistingfilename: super::super::Foundation::PWSTR, lpsecurityattributes: *mut super::super::Security::SECURITY_ATTRIBUTES) -> super::super::Foundation::BOOL;
    pub fn CreateIoRing(ioringversion: IORING_VERSION, flags: IORING_CREATE_FLAGS, submissionqueuesize: u32, completionqueuesize: u32, h: *mut *mut HIORING__) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn CreateLogContainerScanContext(hlog: super::super::Foundation::HANDLE, cfromcontainer: u32, ccontainers: u32, escanmode: u8, pcxscan: *mut CLS_SCAN_CONTEXT, poverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateLogFile(pszlogfilename: super::super::Foundation::PWSTR, fdesiredaccess: FILE_ACCESS_FLAGS, dwsharemode: FILE_SHARE_MODE, psalogfile: *mut super::super::Security::SECURITY_ATTRIBUTES, fcreatedisposition: FILE_CREATION_DISPOSITION, fflagsandattributes: FILE_FLAGS_AND_ATTRIBUTES) -> super::super::Foundation::HANDLE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateLogMarshallingArea(hlog: super::super::Foundation::HANDLE, pfnallocbuffer: CLFS_BLOCK_ALLOCATION, pfnfreebuffer: CLFS_BLOCK_DEALLOCATION, pvblockalloccontext: *mut ::core::ffi::c_void, cbmarshallingbuffer: u32, cmaxwritebuffers: u32, cmaxreadbuffers: u32, ppvmarshal: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateResourceManager(lpresourcemanagerattributes: *mut super::super::Security::SECURITY_ATTRIBUTES, resourcemanagerid: *mut ::windows_sys::core::GUID, createoptions: u32, tmhandle: super::super::Foundation::HANDLE, description: super::super::Foundation::PWSTR) -> super::super::Foundation::HANDLE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateSymbolicLinkA(lpsymlinkfilename: super::super::Foundation::PSTR, lptargetfilename: super::super::Foundation::PSTR, dwflags: SYMBOLIC_LINK_FLAGS) -> super::super::Foundation::BOOLEAN;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateSymbolicLinkTransactedA(lpsymlinkfilename: super::super::Foundation::PSTR, lptargetfilename: super::super::Foundation::PSTR, dwflags: SYMBOLIC_LINK_FLAGS, htransaction: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOLEAN;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateSymbolicLinkTransactedW(lpsymlinkfilename: super::super::Foundation::PWSTR, lptargetfilename: super::super::Foundation::PWSTR, dwflags: SYMBOLIC_LINK_FLAGS, htransaction: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOLEAN;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateSymbolicLinkW(lpsymlinkfilename: super::super::Foundation::PWSTR, lptargetfilename: super::super::Foundation::PWSTR, dwflags: SYMBOLIC_LINK_FLAGS) -> super::super::Foundation::BOOLEAN;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateTapePartition(hdevice: super::super::Foundation::HANDLE, dwpartitionmethod: CREATE_TAPE_PARTITION_METHOD, dwcount: u32, dwsize: u32) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateTransaction(lptransactionattributes: *mut super::super::Security::SECURITY_ATTRIBUTES, uow: *mut ::windows_sys::core::GUID, createoptions: u32, isolationlevel: u32, isolationflags: u32, timeout: u32, description: super::super::Foundation::PWSTR) -> super::super::Foundation::HANDLE;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateTransactionManager(lptransactionattributes: *mut super::super::Security::SECURITY_ATTRIBUTES, logfilename: super::super::Foundation::PWSTR, createoptions: u32, commitstrength: u32) -> super::super::Foundation::HANDLE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DecryptFileA(lpfilename: super::super::Foundation::PSTR, dwreserved: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DecryptFileW(lpfilename: super::super::Foundation::PWSTR, dwreserved: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DefineDosDeviceA(dwflags: DEFINE_DOS_DEVICE_FLAGS, lpdevicename: super::super::Foundation::PSTR, lptargetpath: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DefineDosDeviceW(dwflags: DEFINE_DOS_DEVICE_FLAGS, lpdevicename: super::super::Foundation::PWSTR, lptargetpath: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteFileA(lpfilename: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteFileFromAppW(lpfilename: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteFileTransactedA(lpfilename: super::super::Foundation::PSTR, htransaction: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteFileTransactedW(lpfilename: super::super::Foundation::PWSTR, htransaction: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteFileW(lpfilename: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteLogByHandle(hlog: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteLogFile(pszlogfilename: super::super::Foundation::PWSTR, pvreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteLogMarshallingArea(pvmarshal: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteVolumeMountPointA(lpszvolumemountpoint: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteVolumeMountPointW(lpszvolumemountpoint: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeregisterManageableLogClient(hlog: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn DuplicateEncryptionInfoFile(srcfilename: super::super::Foundation::PWSTR, dstfilename: super::super::Foundation::PWSTR, dwcreationdistribution: u32, dwattributes: u32, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EncryptFileA(lpfilename: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EncryptFileW(lpfilename: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EncryptionDisable(dirpath: super::super::Foundation::PWSTR, disable: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EraseTape(hdevice: super::super::Foundation::HANDLE, dwerasetype: ERASE_TAPE_TYPE, bimmediate: super::super::Foundation::BOOL) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FileEncryptionStatusA(lpfilename: super::super::Foundation::PSTR, lpstatus: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FileEncryptionStatusW(lpfilename: super::super::Foundation::PWSTR, lpstatus: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FileTimeToLocalFileTime(lpfiletime: *const super::super::Foundation::FILETIME, lplocalfiletime: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindClose(hfindfile: FindFileHandle) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindCloseChangeNotification(hchangehandle: FindChangeNotificationHandle) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindFirstChangeNotificationA(lppathname: super::super::Foundation::PSTR, bwatchsubtree: super::super::Foundation::BOOL, dwnotifyfilter: FILE_NOTIFY_CHANGE) -> FindChangeNotificationHandle;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindFirstChangeNotificationW(lppathname: super::super::Foundation::PWSTR, bwatchsubtree: super::super::Foundation::BOOL, dwnotifyfilter: FILE_NOTIFY_CHANGE) -> FindChangeNotificationHandle;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindFirstFileA(lpfilename: super::super::Foundation::PSTR, lpfindfiledata: *mut WIN32_FIND_DATAA) -> FindFileHandle;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindFirstFileExA(lpfilename: super::super::Foundation::PSTR, finfolevelid: FINDEX_INFO_LEVELS, lpfindfiledata: *mut ::core::ffi::c_void, fsearchop: FINDEX_SEARCH_OPS, lpsearchfilter: *mut ::core::ffi::c_void, dwadditionalflags: FIND_FIRST_EX_FLAGS) -> FindFileHandle;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindFirstFileExFromAppW(lpfilename: super::super::Foundation::PWSTR, finfolevelid: FINDEX_INFO_LEVELS, lpfindfiledata: *mut ::core::ffi::c_void, fsearchop: FINDEX_SEARCH_OPS, lpsearchfilter: *mut ::core::ffi::c_void, dwadditionalflags: u32) -> super::super::Foundation::HANDLE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindFirstFileExW(lpfilename: super::super::Foundation::PWSTR, finfolevelid: FINDEX_INFO_LEVELS, lpfindfiledata: *mut ::core::ffi::c_void, fsearchop: FINDEX_SEARCH_OPS, lpsearchfilter: *mut ::core::ffi::c_void, dwadditionalflags: FIND_FIRST_EX_FLAGS) -> FindFileHandle;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindFirstFileNameTransactedW(lpfilename: super::super::Foundation::PWSTR, dwflags: u32, stringlength: *mut u32, linkname: super::super::Foundation::PWSTR, htransaction: super::super::Foundation::HANDLE) -> FindFileNameHandle;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindFirstFileNameW(lpfilename: super::super::Foundation::PWSTR, dwflags: u32, stringlength: *mut u32, linkname: super::super::Foundation::PWSTR) -> FindFileNameHandle;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindFirstFileTransactedA(lpfilename: super::super::Foundation::PSTR, finfolevelid: FINDEX_INFO_LEVELS, lpfindfiledata: *mut ::core::ffi::c_void, fsearchop: FINDEX_SEARCH_OPS, lpsearchfilter: *mut ::core::ffi::c_void, dwadditionalflags: u32, htransaction: super::super::Foundation::HANDLE) -> FindFileHandle;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindFirstFileTransactedW(lpfilename: super::super::Foundation::PWSTR, finfolevelid: FINDEX_INFO_LEVELS, lpfindfiledata: *mut ::core::ffi::c_void, fsearchop: FINDEX_SEARCH_OPS, lpsearchfilter: *mut ::core::ffi::c_void, dwadditionalflags: u32, htransaction: super::super::Foundation::HANDLE) -> FindFileHandle;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindFirstFileW(lpfilename: super::super::Foundation::PWSTR, lpfindfiledata: *mut WIN32_FIND_DATAW) -> FindFileHandle;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindFirstStreamTransactedW(lpfilename: super::super::Foundation::PWSTR, infolevel: STREAM_INFO_LEVELS, lpfindstreamdata: *mut ::core::ffi::c_void, dwflags: u32, htransaction: super::super::Foundation::HANDLE) -> FindStreamHandle;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindFirstStreamW(lpfilename: super::super::Foundation::PWSTR, infolevel: STREAM_INFO_LEVELS, lpfindstreamdata: *mut ::core::ffi::c_void, dwflags: u32) -> FindStreamHandle;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindFirstVolumeA(lpszvolumename: super::super::Foundation::PSTR, cchbufferlength: u32) -> FindVolumeHandle;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindFirstVolumeMountPointA(lpszrootpathname: super::super::Foundation::PSTR, lpszvolumemountpoint: super::super::Foundation::PSTR, cchbufferlength: u32) -> FindVolumeMointPointHandle;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindFirstVolumeMountPointW(lpszrootpathname: super::super::Foundation::PWSTR, lpszvolumemountpoint: super::super::Foundation::PWSTR, cchbufferlength: u32) -> FindVolumeMointPointHandle;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindFirstVolumeW(lpszvolumename: super::super::Foundation::PWSTR, cchbufferlength: u32) -> FindVolumeHandle;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindNextChangeNotification(hchangehandle: FindChangeNotificationHandle) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindNextFileA(hfindfile: FindFileHandle, lpfindfiledata: *mut WIN32_FIND_DATAA) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindNextFileNameW(hfindstream: FindFileNameHandle, stringlength: *mut u32, linkname: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindNextFileW(hfindfile: super::super::Foundation::HANDLE, lpfindfiledata: *mut WIN32_FIND_DATAW) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindNextStreamW(hfindstream: FindStreamHandle, lpfindstreamdata: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindNextVolumeA(hfindvolume: FindVolumeHandle, lpszvolumename: super::super::Foundation::PSTR, cchbufferlength: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindNextVolumeMountPointA(hfindvolumemountpoint: FindVolumeMointPointHandle, lpszvolumemountpoint: super::super::Foundation::PSTR, cchbufferlength: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindNextVolumeMountPointW(hfindvolumemountpoint: FindVolumeMointPointHandle, lpszvolumemountpoint: super::super::Foundation::PWSTR, cchbufferlength: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindNextVolumeW(hfindvolume: FindVolumeHandle, lpszvolumename: super::super::Foundation::PWSTR, cchbufferlength: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindVolumeClose(hfindvolume: FindVolumeHandle) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindVolumeMountPointClose(hfindvolumemountpoint: FindVolumeMointPointHandle) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FlushFileBuffers(hfile: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn FlushLogBuffers(pvmarshal: *mut ::core::ffi::c_void, poverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn FlushLogToLsn(pvmarshalcontext: *mut ::core::ffi::c_void, plsnflush: *mut CLS_LSN, plsnlastflushed: *mut CLS_LSN, poverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    pub fn FreeEncryptedFileMetadata(pbmetadata: *const u8);
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FreeEncryptionCertificateHashList(pusers: *const ENCRYPTION_CERTIFICATE_HASH_LIST);
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeReservedLog(pvmarshal: *mut ::core::ffi::c_void, creservedrecords: u32, pcbadjustment: *mut i64) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetBinaryTypeA(lpapplicationname: super::super::Foundation::PSTR, lpbinarytype: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetBinaryTypeW(lpapplicationname: super::super::Foundation::PWSTR, lpbinarytype: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCompressedFileSizeA(lpfilename: super::super::Foundation::PSTR, lpfilesizehigh: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCompressedFileSizeTransactedA(lpfilename: super::super::Foundation::PSTR, lpfilesizehigh: *mut u32, htransaction: super::super::Foundation::HANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCompressedFileSizeTransactedW(lpfilename: super::super::Foundation::PWSTR, lpfilesizehigh: *mut u32, htransaction: super::super::Foundation::HANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCompressedFileSizeW(lpfilename: super::super::Foundation::PWSTR, lpfilesizehigh: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrentClockTransactionManager(transactionmanagerhandle: super::super::Foundation::HANDLE, tmvirtualclock: *mut i64) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDiskFreeSpaceA(lprootpathname: super::super::Foundation::PSTR, lpsectorspercluster: *mut u32, lpbytespersector: *mut u32, lpnumberoffreeclusters: *mut u32, lptotalnumberofclusters: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDiskFreeSpaceExA(lpdirectoryname: super::super::Foundation::PSTR, lpfreebytesavailabletocaller: *mut u64, lptotalnumberofbytes: *mut u64, lptotalnumberoffreebytes: *mut u64) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDiskFreeSpaceExW(lpdirectoryname: super::super::Foundation::PWSTR, lpfreebytesavailabletocaller: *mut u64, lptotalnumberofbytes: *mut u64, lptotalnumberoffreebytes: *mut u64) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDiskFreeSpaceW(lprootpathname: super::super::Foundation::PWSTR, lpsectorspercluster: *mut u32, lpbytespersector: *mut u32, lpnumberoffreeclusters: *mut u32, lptotalnumberofclusters: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDiskSpaceInformationA(rootpath: super::super::Foundation::PSTR, diskspaceinfo: *mut DISK_SPACE_INFORMATION) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDiskSpaceInformationW(rootpath: super::super::Foundation::PWSTR, diskspaceinfo: *mut DISK_SPACE_INFORMATION) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDriveTypeA(lprootpathname: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDriveTypeW(lprootpathname: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetEncryptedFileMetadata(lpfilename: super::super::Foundation::PWSTR, pcbmetadata: *mut u32, ppbmetadata: *mut *mut u8) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetEnlistmentId(enlistmenthandle: super::super::Foundation::HANDLE, enlistmentid: *mut ::windows_sys::core::GUID) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetEnlistmentRecoveryInformation(enlistmenthandle: super::super::Foundation::HANDLE, buffersize: u32, buffer: *mut ::core::ffi::c_void, bufferused: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetExpandedNameA(lpszsource: super::super::Foundation::PSTR, lpszbuffer: super::super::Foundation::PSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetExpandedNameW(lpszsource: super::super::Foundation::PWSTR, lpszbuffer: super::super::Foundation::PWSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileAttributesA(lpfilename: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileAttributesExA(lpfilename: super::super::Foundation::PSTR, finfolevelid: GET_FILEEX_INFO_LEVELS, lpfileinformation: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileAttributesExFromAppW(lpfilename: super::super::Foundation::PWSTR, finfolevelid: GET_FILEEX_INFO_LEVELS, lpfileinformation: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileAttributesExW(lpfilename: super::super::Foundation::PWSTR, finfolevelid: GET_FILEEX_INFO_LEVELS, lpfileinformation: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileAttributesTransactedA(lpfilename: super::super::Foundation::PSTR, finfolevelid: GET_FILEEX_INFO_LEVELS, lpfileinformation: *mut ::core::ffi::c_void, htransaction: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileAttributesTransactedW(lpfilename: super::super::Foundation::PWSTR, finfolevelid: GET_FILEEX_INFO_LEVELS, lpfileinformation: *mut ::core::ffi::c_void, htransaction: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileAttributesW(lpfilename: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileBandwidthReservation(hfile: super::super::Foundation::HANDLE, lpperiodmilliseconds: *mut u32, lpbytesperperiod: *mut u32, pdiscardable: *mut i32, lptransfersize: *mut u32, lpnumoutstandingrequests: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileInformationByHandle(hfile: super::super::Foundation::HANDLE, lpfileinformation: *mut BY_HANDLE_FILE_INFORMATION) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileInformationByHandleEx(hfile: super::super::Foundation::HANDLE, fileinformationclass: FILE_INFO_BY_HANDLE_CLASS, lpfileinformation: *mut ::core::ffi::c_void, dwbuffersize: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileSize(hfile: super::super::Foundation::HANDLE, lpfilesizehigh: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileSizeEx(hfile: super::super::Foundation::HANDLE, lpfilesize: *mut i64) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileTime(hfile: super::super::Foundation::HANDLE, lpcreationtime: *mut super::super::Foundation::FILETIME, lplastaccesstime: *mut super::super::Foundation::FILETIME, lplastwritetime: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileType(hfile: super::super::Foundation::HANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileVersionInfoA(lptstrfilename: super::super::Foundation::PSTR, dwhandle: u32, dwlen: u32, lpdata: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileVersionInfoExA(dwflags: GET_FILE_VERSION_INFO_FLAGS, lpwstrfilename: super::super::Foundation::PSTR, dwhandle: u32, dwlen: u32, lpdata: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileVersionInfoExW(dwflags: GET_FILE_VERSION_INFO_FLAGS, lpwstrfilename: super::super::Foundation::PWSTR, dwhandle: u32, dwlen: u32, lpdata: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileVersionInfoSizeA(lptstrfilename: super::super::Foundation::PSTR, lpdwhandle: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileVersionInfoSizeExA(dwflags: GET_FILE_VERSION_INFO_FLAGS, lpwstrfilename: super::super::Foundation::PSTR, lpdwhandle: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileVersionInfoSizeExW(dwflags: GET_FILE_VERSION_INFO_FLAGS, lpwstrfilename: super::super::Foundation::PWSTR, lpdwhandle: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileVersionInfoSizeW(lptstrfilename: super::super::Foundation::PWSTR, lpdwhandle: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileVersionInfoW(lptstrfilename: super::super::Foundation::PWSTR, dwhandle: u32, dwlen: u32, lpdata: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFinalPathNameByHandleA(hfile: super::super::Foundation::HANDLE, lpszfilepath: super::super::Foundation::PSTR, cchfilepath: u32, dwflags: FILE_NAME) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFinalPathNameByHandleW(hfile: super::super::Foundation::HANDLE, lpszfilepath: super::super::Foundation::PWSTR, cchfilepath: u32, dwflags: FILE_NAME) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFullPathNameA(lpfilename: super::super::Foundation::PSTR, nbufferlength: u32, lpbuffer: super::super::Foundation::PSTR, lpfilepart: *mut super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFullPathNameTransactedA(lpfilename: super::super::Foundation::PSTR, nbufferlength: u32, lpbuffer: super::super::Foundation::PSTR, lpfilepart: *mut super::super::Foundation::PSTR, htransaction: super::super::Foundation::HANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFullPathNameTransactedW(lpfilename: super::super::Foundation::PWSTR, nbufferlength: u32, lpbuffer: super::super::Foundation::PWSTR, lpfilepart: *mut super::super::Foundation::PWSTR, htransaction: super::super::Foundation::HANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFullPathNameW(lpfilename: super::super::Foundation::PWSTR, nbufferlength: u32, lpbuffer: super::super::Foundation::PWSTR, lpfilepart: *mut super::super::Foundation::PWSTR) -> u32;
    pub fn GetIoRingInfo(ioring: *const HIORING__, info: *mut IORING_INFO) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLogContainerName(hlog: super::super::Foundation::HANDLE, cidlogicalcontainer: u32, pwstrcontainername: super::super::Foundation::PWSTR, clencontainername: u32, pcactuallencontainername: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLogFileInformation(hlog: super::super::Foundation::HANDLE, pinfobuffer: *mut CLS_INFORMATION, cbbuffer: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLogIoStatistics(hlog: super::super::Foundation::HANDLE, pvstatsbuffer: *mut ::core::ffi::c_void, cbstatsbuffer: u32, estatsclass: CLFS_IOSTATS_CLASS, pcbstatswritten: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLogReservationInfo(pvmarshal: *const ::core::ffi::c_void, pcbrecordnumber: *mut u32, pcbuserreservation: *mut i64, pcbcommitreservation: *mut i64) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLogicalDriveStringsA(nbufferlength: u32, lpbuffer: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLogicalDriveStringsW(nbufferlength: u32, lpbuffer: super::super::Foundation::PWSTR) -> u32;
    pub fn GetLogicalDrives() -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLongPathNameA(lpszshortpath: super::super::Foundation::PSTR, lpszlongpath: super::super::Foundation::PSTR, cchbuffer: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLongPathNameTransactedA(lpszshortpath: super::super::Foundation::PSTR, lpszlongpath: super::super::Foundation::PSTR, cchbuffer: u32, htransaction: super::super::Foundation::HANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLongPathNameTransactedW(lpszshortpath: super::super::Foundation::PWSTR, lpszlongpath: super::super::Foundation::PWSTR, cchbuffer: u32, htransaction: super::super::Foundation::HANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLongPathNameW(lpszshortpath: super::super::Foundation::PWSTR, lpszlongpath: super::super::Foundation::PWSTR, cchbuffer: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNextLogArchiveExtent(pvarchivecontext: *mut ::core::ffi::c_void, rgadextent: *mut CLS_ARCHIVE_DESCRIPTOR, cdescriptors: u32, pcdescriptorsreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNotificationResourceManager(resourcemanagerhandle: super::super::Foundation::HANDLE, transactionnotification: *mut TRANSACTION_NOTIFICATION, notificationlength: u32, dwmilliseconds: u32, returnlength: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn GetNotificationResourceManagerAsync(resourcemanagerhandle: super::super::Foundation::HANDLE, transactionnotification: *mut TRANSACTION_NOTIFICATION, transactionnotificationlength: u32, returnlength: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetShortPathNameA(lpszlongpath: super::super::Foundation::PSTR, lpszshortpath: super::super::Foundation::PSTR, cchbuffer: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetShortPathNameW(lpszlongpath: super::super::Foundation::PWSTR, lpszshortpath: super::super::Foundation::PWSTR, cchbuffer: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTapeParameters(hdevice: super::super::Foundation::HANDLE, dwoperation: GET_TAPE_DRIVE_PARAMETERS_OPERATION, lpdwsize: *mut u32, lptapeinformation: *mut ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTapePosition(hdevice: super::super::Foundation::HANDLE, dwpositiontype: TAPE_POSITION_TYPE, lpdwpartition: *mut u32, lpdwoffsetlow: *mut u32, lpdwoffsethigh: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTapeStatus(hdevice: super::super::Foundation::HANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTempFileNameA(lppathname: super::super::Foundation::PSTR, lpprefixstring: super::super::Foundation::PSTR, uunique: u32, lptempfilename: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTempFileNameW(lppathname: super::super::Foundation::PWSTR, lpprefixstring: super::super::Foundation::PWSTR, uunique: u32, lptempfilename: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTempPath2A(bufferlength: u32, buffer: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTempPath2W(bufferlength: u32, buffer: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTempPathA(nbufferlength: u32, lpbuffer: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTempPathW(nbufferlength: u32, lpbuffer: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTransactionId(transactionhandle: super::super::Foundation::HANDLE, transactionid: *mut ::windows_sys::core::GUID) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTransactionInformation(transactionhandle: super::super::Foundation::HANDLE, outcome: *mut u32, isolationlevel: *mut u32, isolationflags: *mut u32, timeout: *mut u32, bufferlength: u32, description: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTransactionManagerId(transactionmanagerhandle: super::super::Foundation::HANDLE, transactionmanagerid: *mut ::windows_sys::core::GUID) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetVolumeInformationA(lprootpathname: super::super::Foundation::PSTR, lpvolumenamebuffer: super::super::Foundation::PSTR, nvolumenamesize: u32, lpvolumeserialnumber: *mut u32, lpmaximumcomponentlength: *mut u32, lpfilesystemflags: *mut u32, lpfilesystemnamebuffer: super::super::Foundation::PSTR, nfilesystemnamesize: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetVolumeInformationByHandleW(hfile: super::super::Foundation::HANDLE, lpvolumenamebuffer: super::super::Foundation::PWSTR, nvolumenamesize: u32, lpvolumeserialnumber: *mut u32, lpmaximumcomponentlength: *mut u32, lpfilesystemflags: *mut u32, lpfilesystemnamebuffer: super::super::Foundation::PWSTR, nfilesystemnamesize: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetVolumeInformationW(lprootpathname: super::super::Foundation::PWSTR, lpvolumenamebuffer: super::super::Foundation::PWSTR, nvolumenamesize: u32, lpvolumeserialnumber: *mut u32, lpmaximumcomponentlength: *mut u32, lpfilesystemflags: *mut u32, lpfilesystemnamebuffer: super::super::Foundation::PWSTR, nfilesystemnamesize: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetVolumeNameForVolumeMountPointA(lpszvolumemountpoint: super::super::Foundation::PSTR, lpszvolumename: super::super::Foundation::PSTR, cchbufferlength: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetVolumeNameForVolumeMountPointW(lpszvolumemountpoint: super::super::Foundation::PWSTR, lpszvolumename: super::super::Foundation::PWSTR, cchbufferlength: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetVolumePathNameA(lpszfilename: super::super::Foundation::PSTR, lpszvolumepathname: super::super::Foundation::PSTR, cchbufferlength: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetVolumePathNameW(lpszfilename: super::super::Foundation::PWSTR, lpszvolumepathname: super::super::Foundation::PWSTR, cchbufferlength: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetVolumePathNamesForVolumeNameA(lpszvolumename: super::super::Foundation::PSTR, lpszvolumepathnames: super::super::Foundation::PSTR, cchbufferlength: u32, lpcchreturnlength: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetVolumePathNamesForVolumeNameW(lpszvolumename: super::super::Foundation::PWSTR, lpszvolumepathnames: super::super::Foundation::PWSTR, cchbufferlength: u32, lpcchreturnlength: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn HandleLogFull(hlog: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn InstallLogPolicy(hlog: super::super::Foundation::HANDLE, ppolicy: *mut CLFS_MGMT_POLICY) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsIoRingOpSupported(ioring: *const HIORING__, op: IORING_OP_CODE) -> super::super::Foundation::BOOL;
    pub fn LZClose(hfile: i32);
    pub fn LZCopy(hfsource: i32, hfdest: i32) -> i32;
    pub fn LZDone();
    pub fn LZInit(hfsource: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn LZOpenFileA(lpfilename: super::super::Foundation::PSTR, lpreopenbuf: *mut OFSTRUCT, wstyle: LZOPENFILE_STYLE) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn LZOpenFileW(lpfilename: super::super::Foundation::PWSTR, lpreopenbuf: *mut OFSTRUCT, wstyle: LZOPENFILE_STYLE) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn LZRead(hfile: i32, lpbuffer: super::super::Foundation::PSTR, cbread: i32) -> i32;
    pub fn LZSeek(hfile: i32, loffset: i32, iorigin: i32) -> i32;
    pub fn LZStart() -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn LocalFileTimeToFileTime(lplocalfiletime: *const super::super::Foundation::FILETIME, lpfiletime: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn LockFile(hfile: super::super::Foundation::HANDLE, dwfileoffsetlow: u32, dwfileoffsethigh: u32, nnumberofbytestolocklow: u32, nnumberofbytestolockhigh: u32) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn LockFileEx(hfile: super::super::Foundation::HANDLE, dwflags: LOCK_FILE_FLAGS, dwreserved: u32, nnumberofbytestolocklow: u32, nnumberofbytestolockhigh: u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn LogTailAdvanceFailure(hlog: super::super::Foundation::HANDLE, dwreason: u32) -> super::super::Foundation::BOOL;
    pub fn LsnBlockOffset(plsn: *const CLS_LSN) -> u32;
    pub fn LsnContainer(plsn: *const CLS_LSN) -> u32;
    pub fn LsnCreate(cidcontainer: u32, offblock: u32, crecord: u32) -> CLS_LSN;
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsnEqual(plsn1: *const CLS_LSN, plsn2: *const CLS_LSN) -> super::super::Foundation::BOOLEAN;
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsnGreater(plsn1: *const CLS_LSN, plsn2: *const CLS_LSN) -> super::super::Foundation::BOOLEAN;
    pub fn LsnIncrement(plsn: *const CLS_LSN) -> CLS_LSN;
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsnInvalid(plsn: *const CLS_LSN) -> super::super::Foundation::BOOLEAN;
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsnLess(plsn1: *const CLS_LSN, plsn2: *const CLS_LSN) -> super::super::Foundation::BOOLEAN;
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsnNull(plsn: *const CLS_LSN) -> super::super::Foundation::BOOLEAN;
    pub fn LsnRecordSequence(plsn: *const CLS_LSN) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MoveFileA(lpexistingfilename: super::super::Foundation::PSTR, lpnewfilename: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MoveFileExA(lpexistingfilename: super::super::Foundation::PSTR, lpnewfilename: super::super::Foundation::PSTR, dwflags: MOVE_FILE_FLAGS) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MoveFileExW(lpexistingfilename: super::super::Foundation::PWSTR, lpnewfilename: super::super::Foundation::PWSTR, dwflags: MOVE_FILE_FLAGS) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MoveFileFromAppW(lpexistingfilename: super::super::Foundation::PWSTR, lpnewfilename: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MoveFileTransactedA(lpexistingfilename: super::super::Foundation::PSTR, lpnewfilename: super::super::Foundation::PSTR, lpprogressroutine: LPPROGRESS_ROUTINE, lpdata: *const ::core::ffi::c_void, dwflags: MOVE_FILE_FLAGS, htransaction: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MoveFileTransactedW(lpexistingfilename: super::super::Foundation::PWSTR, lpnewfilename: super::super::Foundation::PWSTR, lpprogressroutine: LPPROGRESS_ROUTINE, lpdata: *const ::core::ffi::c_void, dwflags: MOVE_FILE_FLAGS, htransaction: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MoveFileW(lpexistingfilename: super::super::Foundation::PWSTR, lpnewfilename: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MoveFileWithProgressA(lpexistingfilename: super::super::Foundation::PSTR, lpnewfilename: super::super::Foundation::PSTR, lpprogressroutine: LPPROGRESS_ROUTINE, lpdata: *const ::core::ffi::c_void, dwflags: MOVE_FILE_FLAGS) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MoveFileWithProgressW(lpexistingfilename: super::super::Foundation::PWSTR, lpnewfilename: super::super::Foundation::PWSTR, lpprogressroutine: LPPROGRESS_ROUTINE, lpdata: *const ::core::ffi::c_void, dwflags: MOVE_FILE_FLAGS) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetConnectionEnum(servername: super::super::Foundation::PWSTR, qualifier: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetFileClose(servername: super::super::Foundation::PWSTR, fileid: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetFileEnum(servername: super::super::Foundation::PWSTR, basepath: super::super::Foundation::PWSTR, username: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: *mut usize) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetFileGetInfo(servername: super::super::Foundation::PWSTR, fileid: u32, level: u32, bufptr: *mut *mut u8) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetServerAliasAdd(servername: super::super::Foundation::PWSTR, level: u32, buf: *const u8) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetServerAliasDel(servername: super::super::Foundation::PWSTR, level: u32, buf: *const u8) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetServerAliasEnum(servername: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resumehandle: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetSessionDel(servername: super::super::Foundation::PWSTR, uncclientname: super::super::Foundation::PWSTR, username: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetSessionEnum(servername: super::super::Foundation::PWSTR, uncclientname: super::super::Foundation::PWSTR, username: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetSessionGetInfo(servername: super::super::Foundation::PWSTR, uncclientname: super::super::Foundation::PWSTR, username: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetShareAdd(servername: super::super::Foundation::PWSTR, level: u32, buf: *const u8, parm_err: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetShareCheck(servername: super::super::Foundation::PWSTR, device: super::super::Foundation::PWSTR, r#type: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetShareDel(servername: super::super::Foundation::PWSTR, netname: super::super::Foundation::PWSTR, reserved: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetShareDelEx(servername: super::super::Foundation::PWSTR, level: u32, buf: *const u8) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetShareDelSticky(servername: super::super::Foundation::PWSTR, netname: super::super::Foundation::PWSTR, reserved: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetShareEnum(servername: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetShareEnumSticky(servername: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetShareGetInfo(servername: super::super::Foundation::PWSTR, netname: super::super::Foundation::PWSTR, level: u32, bufptr: *mut *mut u8) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetShareSetInfo(servername: super::super::Foundation::PWSTR, netname: super::super::Foundation::PWSTR, level: u32, buf: *const u8, parm_err: *mut u32) -> u32;
    pub fn NetStatisticsGet(servername: *const i8, service: *const i8, level: u32, options: u32, buffer: *mut *mut u8) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
    pub fn NtCreateFile(filehandle: *mut super::super::Foundation::HANDLE, desiredaccess: u32, objectattributes: *mut super::super::System::WindowsProgramming::OBJECT_ATTRIBUTES, iostatusblock: *mut super::super::System::WindowsProgramming::IO_STATUS_BLOCK, allocationsize: *mut i64, fileattributes: u32, shareaccess: FILE_SHARE_MODE, createdisposition: NT_CREATE_FILE_DISPOSITION, createoptions: u32, eabuffer: *mut ::core::ffi::c_void, ealength: u32) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenEncryptedFileRawA(lpfilename: super::super::Foundation::PSTR, ulflags: u32, pvcontext: *mut *mut ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenEncryptedFileRawW(lpfilename: super::super::Foundation::PWSTR, ulflags: u32, pvcontext: *mut *mut ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenEnlistment(dwdesiredaccess: u32, resourcemanagerhandle: super::super::Foundation::HANDLE, enlistmentid: *mut ::windows_sys::core::GUID) -> super::super::Foundation::HANDLE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenFile(lpfilename: super::super::Foundation::PSTR, lpreopenbuff: *mut OFSTRUCT, ustyle: LZOPENFILE_STYLE) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn OpenFileById(hvolumehint: super::super::Foundation::HANDLE, lpfileid: *const FILE_ID_DESCRIPTOR, dwdesiredaccess: FILE_ACCESS_FLAGS, dwsharemode: FILE_SHARE_MODE, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, dwflagsandattributes: FILE_FLAGS_AND_ATTRIBUTES) -> super::super::Foundation::HANDLE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenResourceManager(dwdesiredaccess: u32, tmhandle: super::super::Foundation::HANDLE, resourcemanagerid: *mut ::windows_sys::core::GUID) -> super::super::Foundation::HANDLE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenTransaction(dwdesiredaccess: u32, transactionid: *mut ::windows_sys::core::GUID) -> super::super::Foundation::HANDLE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenTransactionManager(logfilename: super::super::Foundation::PWSTR, desiredaccess: u32, openoptions: u32) -> super::super::Foundation::HANDLE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenTransactionManagerById(transactionmanagerid: *const ::windows_sys::core::GUID, desiredaccess: u32, openoptions: u32) -> super::super::Foundation::HANDLE;
    pub fn PopIoRingCompletion(ioring: *const HIORING__, cqe: *mut IORING_CQE) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrePrepareComplete(enlistmenthandle: super::super::Foundation::HANDLE, tmvirtualclock: *mut i64) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrePrepareEnlistment(enlistmenthandle: super::super::Foundation::HANDLE, tmvirtualclock: *mut i64) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrepareComplete(enlistmenthandle: super::super::Foundation::HANDLE, tmvirtualclock: *mut i64) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrepareEnlistment(enlistmenthandle: super::super::Foundation::HANDLE, tmvirtualclock: *mut i64) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrepareLogArchive(hlog: super::super::Foundation::HANDLE, pszbaselogfilename: super::super::Foundation::PWSTR, clen: u32, plsnlow: *const CLS_LSN, plsnhigh: *const CLS_LSN, pcactuallength: *mut u32, poffbaselogfiledata: *mut u64, pcbbaselogfilelength: *mut u64, plsnbase: *mut CLS_LSN, plsnlast: *mut CLS_LSN, plsncurrentarchivetail: *mut CLS_LSN, ppvarchivecontext: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrepareTape(hdevice: super::super::Foundation::HANDLE, dwoperation: PREPARE_TAPE_OPERATION, bimmediate: super::super::Foundation::BOOL) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryDosDeviceA(lpdevicename: super::super::Foundation::PSTR, lptargetpath: super::super::Foundation::PSTR, ucchmax: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryDosDeviceW(lpdevicename: super::super::Foundation::PWSTR, lptargetpath: super::super::Foundation::PWSTR, ucchmax: u32) -> u32;
    pub fn QueryIoRingCapabilities(capabilities: *mut IORING_CAPABILITIES) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryLogPolicy(hlog: super::super::Foundation::HANDLE, epolicytype: CLFS_MGMT_POLICY_TYPE, ppolicybuffer: *mut CLFS_MGMT_POLICY, pcbpolicybuffer: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn QueryRecoveryAgentsOnEncryptedFile(lpfilename: super::super::Foundation::PWSTR, precoveryagents: *mut *mut ENCRYPTION_CERTIFICATE_HASH_LIST) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn QueryUsersOnEncryptedFile(lpfilename: super::super::Foundation::PWSTR, pusers: *mut *mut ENCRYPTION_CERTIFICATE_HASH_LIST) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReOpenFile(horiginalfile: super::super::Foundation::HANDLE, dwdesiredaccess: FILE_ACCESS_FLAGS, dwsharemode: FILE_SHARE_MODE, dwflagsandattributes: FILE_FLAGS_AND_ATTRIBUTES) -> super::super::Foundation::HANDLE;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn ReadDirectoryChangesExW(hdirectory: super::super::Foundation::HANDLE, lpbuffer: *mut ::core::ffi::c_void, nbufferlength: u32, bwatchsubtree: super::super::Foundation::BOOL, dwnotifyfilter: FILE_NOTIFY_CHANGE, lpbytesreturned: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, lpcompletionroutine: super::super::System::IO::LPOVERLAPPED_COMPLETION_ROUTINE, readdirectorynotifyinformationclass: READ_DIRECTORY_NOTIFY_INFORMATION_CLASS) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn ReadDirectoryChangesW(hdirectory: super::super::Foundation::HANDLE, lpbuffer: *mut ::core::ffi::c_void, nbufferlength: u32, bwatchsubtree: super::super::Foundation::BOOL, dwnotifyfilter: FILE_NOTIFY_CHANGE, lpbytesreturned: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, lpcompletionroutine: super::super::System::IO::LPOVERLAPPED_COMPLETION_ROUTINE) -> super::super::Foundation::BOOL;
    pub fn ReadEncryptedFileRaw(pfexportcallback: PFE_EXPORT_FUNC, pvcallbackcontext: *const ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn ReadFile(hfile: super::super::Foundation::HANDLE, lpbuffer: *mut ::core::ffi::c_void, nnumberofbytestoread: u32, lpnumberofbytesread: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn ReadFileEx(hfile: super::super::Foundation::HANDLE, lpbuffer: *mut ::core::ffi::c_void, nnumberofbytestoread: u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, lpcompletionroutine: super::super::System::IO::LPOVERLAPPED_COMPLETION_ROUTINE) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn ReadFileScatter(hfile: super::super::Foundation::HANDLE, asegmentarray: *const FILE_SEGMENT_ELEMENT, nnumberofbytestoread: u32, lpreserved: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReadLogArchiveMetadata(pvarchivecontext: *mut ::core::ffi::c_void, cboffset: u32, cbbytestoread: u32, pbreadbuffer: *mut u8, pcbbytesread: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn ReadLogNotification(hlog: super::super::Foundation::HANDLE, pnotification: *mut CLFS_MGMT_NOTIFICATION, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn ReadLogRecord(pvmarshal: *mut ::core::ffi::c_void, plsnfirst: *mut CLS_LSN, econtextmode: CLFS_CONTEXT_MODE, ppvreadbuffer: *mut *mut ::core::ffi::c_void, pcbreadbuffer: *mut u32, perecordtype: *mut u8, plsnundonext: *mut CLS_LSN, plsnprevious: *mut CLS_LSN, ppvreadcontext: *mut *mut ::core::ffi::c_void, poverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn ReadLogRestartArea(pvmarshal: *mut ::core::ffi::c_void, ppvrestartbuffer: *mut *mut ::core::ffi::c_void, pcbrestartbuffer: *mut u32, plsn: *mut CLS_LSN, ppvcontext: *mut *mut ::core::ffi::c_void, poverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn ReadNextLogRecord(pvreadcontext: *mut ::core::ffi::c_void, ppvbuffer: *mut *mut ::core::ffi::c_void, pcbbuffer: *mut u32, perecordtype: *mut u8, plsnuser: *mut CLS_LSN, plsnundonext: *mut CLS_LSN, plsnprevious: *mut CLS_LSN, plsnrecord: *mut CLS_LSN, poverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReadOnlyEnlistment(enlistmenthandle: super::super::Foundation::HANDLE, tmvirtualclock: *mut i64) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn ReadPreviousLogRestartArea(pvreadcontext: *mut ::core::ffi::c_void, ppvrestartbuffer: *mut *mut ::core::ffi::c_void, pcbrestartbuffer: *mut u32, plsnrestart: *mut CLS_LSN, poverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RecoverEnlistment(enlistmenthandle: super::super::Foundation::HANDLE, enlistmentkey: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RecoverResourceManager(resourcemanagerhandle: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RecoverTransactionManager(transactionmanagerhandle: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterForLogWriteNotification(hlog: super::super::Foundation::HANDLE, cbthreshold: u32, fenable: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterManageableLogClient(hlog: super::super::Foundation::HANDLE, pcallbacks: *mut LOG_MANAGEMENT_CALLBACKS) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveDirectoryA(lppathname: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveDirectoryFromAppW(lppathname: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveDirectoryTransactedA(lppathname: super::super::Foundation::PSTR, htransaction: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveDirectoryTransactedW(lppathname: super::super::Foundation::PWSTR, htransaction: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveDirectoryW(lppathname: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveLogContainer(hlog: super::super::Foundation::HANDLE, pwszcontainerpath: super::super::Foundation::PWSTR, fforce: super::super::Foundation::BOOL, preserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveLogContainerSet(hlog: super::super::Foundation::HANDLE, ccontainer: u16, rgwszcontainerpath: *const super::super::Foundation::PWSTR, fforce: super::super::Foundation::BOOL, preserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveLogPolicy(hlog: super::super::Foundation::HANDLE, epolicytype: CLFS_MGMT_POLICY_TYPE) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn RemoveUsersFromEncryptedFile(lpfilename: super::super::Foundation::PWSTR, phashes: *const ENCRYPTION_CERTIFICATE_HASH_LIST) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RenameTransactionManager(logfilename: super::super::Foundation::PWSTR, existingtransactionmanagerguid: *mut ::windows_sys::core::GUID) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReplaceFileA(lpreplacedfilename: super::super::Foundation::PSTR, lpreplacementfilename: super::super::Foundation::PSTR, lpbackupfilename: super::super::Foundation::PSTR, dwreplaceflags: REPLACE_FILE_FLAGS, lpexclude: *mut ::core::ffi::c_void, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReplaceFileFromAppW(lpreplacedfilename: super::super::Foundation::PWSTR, lpreplacementfilename: super::super::Foundation::PWSTR, lpbackupfilename: super::super::Foundation::PWSTR, dwreplaceflags: u32, lpexclude: *mut ::core::ffi::c_void, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReplaceFileW(lpreplacedfilename: super::super::Foundation::PWSTR, lpreplacementfilename: super::super::Foundation::PWSTR, lpbackupfilename: super::super::Foundation::PWSTR, dwreplaceflags: REPLACE_FILE_FLAGS, lpexclude: *mut ::core::ffi::c_void, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn ReserveAndAppendLog(pvmarshal: *mut ::core::ffi::c_void, rgwriteentries: *mut CLS_WRITE_ENTRY, cwriteentries: u32, plsnundonext: *mut CLS_LSN, plsnprevious: *mut CLS_LSN, creserverecords: u32, rgcbreservation: *mut i64, fflags: CLFS_FLAG, plsn: *mut CLS_LSN, poverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn ReserveAndAppendLogAligned(pvmarshal: *mut ::core::ffi::c_void, rgwriteentries: *mut CLS_WRITE_ENTRY, cwriteentries: u32, cbentryalignment: u32, plsnundonext: *mut CLS_LSN, plsnprevious: *mut CLS_LSN, creserverecords: u32, rgcbreservation: *mut i64, fflags: CLFS_FLAG, plsn: *mut CLS_LSN, poverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RollbackComplete(enlistmenthandle: super::super::Foundation::HANDLE, tmvirtualclock: *mut i64) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RollbackEnlistment(enlistmenthandle: super::super::Foundation::HANDLE, tmvirtualclock: *mut i64) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RollbackTransaction(transactionhandle: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RollbackTransactionAsync(transactionhandle: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RollforwardTransactionManager(transactionmanagerhandle: super::super::Foundation::HANDLE, tmvirtualclock: *mut i64) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ScanLogContainers(pcxscan: *mut CLS_SCAN_CONTEXT, escanmode: u8, preserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SearchPathA(lppath: super::super::Foundation::PSTR, lpfilename: super::super::Foundation::PSTR, lpextension: super::super::Foundation::PSTR, nbufferlength: u32, lpbuffer: super::super::Foundation::PSTR, lpfilepart: *mut super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SearchPathW(lppath: super::super::Foundation::PWSTR, lpfilename: super::super::Foundation::PWSTR, lpextension: super::super::Foundation::PWSTR, nbufferlength: u32, lpbuffer: super::super::Foundation::PWSTR, lpfilepart: *mut super::super::Foundation::PWSTR) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn SetEncryptedFileMetadata(lpfilename: super::super::Foundation::PWSTR, pboldmetadata: *const u8, pbnewmetadata: *const u8, pownerhash: *const ENCRYPTION_CERTIFICATE_HASH, dwoperation: u32, pcertificatesadded: *const ENCRYPTION_CERTIFICATE_HASH_LIST) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetEndOfFile(hfile: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn SetEndOfLog(hlog: super::super::Foundation::HANDLE, plsnend: *mut CLS_LSN, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetEnlistmentRecoveryInformation(enlistmenthandle: super::super::Foundation::HANDLE, buffersize: u32, buffer: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    pub fn SetFileApisToANSI();
    pub fn SetFileApisToOEM();
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFileAttributesA(lpfilename: super::super::Foundation::PSTR, dwfileattributes: FILE_FLAGS_AND_ATTRIBUTES) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFileAttributesFromAppW(lpfilename: super::super::Foundation::PWSTR, dwfileattributes: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFileAttributesTransactedA(lpfilename: super::super::Foundation::PSTR, dwfileattributes: u32, htransaction: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFileAttributesTransactedW(lpfilename: super::super::Foundation::PWSTR, dwfileattributes: u32, htransaction: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFileAttributesW(lpfilename: super::super::Foundation::PWSTR, dwfileattributes: FILE_FLAGS_AND_ATTRIBUTES) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFileBandwidthReservation(hfile: super::super::Foundation::HANDLE, nperiodmilliseconds: u32, nbytesperperiod: u32, bdiscardable: super::super::Foundation::BOOL, lptransfersize: *mut u32, lpnumoutstandingrequests: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFileCompletionNotificationModes(filehandle: super::super::Foundation::HANDLE, flags: u8) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFileInformationByHandle(hfile: super::super::Foundation::HANDLE, fileinformationclass: FILE_INFO_BY_HANDLE_CLASS, lpfileinformation: *const ::core::ffi::c_void, dwbuffersize: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFileIoOverlappedRange(filehandle: super::super::Foundation::HANDLE, overlappedrangestart: *const u8, length: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFilePointer(hfile: super::super::Foundation::HANDLE, ldistancetomove: i32, lpdistancetomovehigh: *mut i32, dwmovemethod: SET_FILE_POINTER_MOVE_METHOD) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFilePointerEx(hfile: super::super::Foundation::HANDLE, lidistancetomove: i64, lpnewfilepointer: *mut i64, dwmovemethod: SET_FILE_POINTER_MOVE_METHOD) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFileShortNameA(hfile: super::super::Foundation::HANDLE, lpshortname: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFileShortNameW(hfile: super::super::Foundation::HANDLE, lpshortname: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFileTime(hfile: super::super::Foundation::HANDLE, lpcreationtime: *const super::super::Foundation::FILETIME, lplastaccesstime: *const super::super::Foundation::FILETIME, lplastwritetime: *const super::super::Foundation::FILETIME) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFileValidData(hfile: super::super::Foundation::HANDLE, validdatalength: i64) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetIoRingCompletionEvent(ioring: *const HIORING__, hevent: super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetLogArchiveMode(hlog: super::super::Foundation::HANDLE, emode: CLFS_LOG_ARCHIVE_MODE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetLogArchiveTail(hlog: super::super::Foundation::HANDLE, plsnarchivetail: *mut CLS_LSN, preserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetLogFileSizeWithPolicy(hlog: super::super::Foundation::HANDLE, pdesiredsize: *mut u64, presultingsize: *mut u64) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetResourceManagerCompletionPort(resourcemanagerhandle: super::super::Foundation::HANDLE, iocompletionporthandle: super::super::Foundation::HANDLE, completionkey: usize) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetSearchPathMode(flags: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetTapeParameters(hdevice: super::super::Foundation::HANDLE, dwoperation: TAPE_INFORMATION_TYPE, lptapeinformation: *const ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetTapePosition(hdevice: super::super::Foundation::HANDLE, dwpositionmethod: TAPE_POSITION_METHOD, dwpartition: u32, dwoffsetlow: u32, dwoffsethigh: u32, bimmediate: super::super::Foundation::BOOL) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetTransactionInformation(transactionhandle: super::super::Foundation::HANDLE, isolationlevel: u32, isolationflags: u32, timeout: u32, description: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Security")]
    pub fn SetUserFileEncryptionKey(pencryptioncertificate: *const ENCRYPTION_CERTIFICATE) -> u32;
    #[cfg(feature = "Win32_Security")]
    pub fn SetUserFileEncryptionKeyEx(pencryptioncertificate: *const ENCRYPTION_CERTIFICATE, dwcapabilities: u32, dwflags: u32, pvreserved: *mut ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetVolumeLabelA(lprootpathname: super::super::Foundation::PSTR, lpvolumename: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetVolumeLabelW(lprootpathname: super::super::Foundation::PWSTR, lpvolumename: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetVolumeMountPointA(lpszvolumemountpoint: super::super::Foundation::PSTR, lpszvolumename: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetVolumeMountPointW(lpszvolumemountpoint: super::super::Foundation::PWSTR, lpszvolumename: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SinglePhaseReject(enlistmenthandle: super::super::Foundation::HANDLE, tmvirtualclock: *mut i64) -> super::super::Foundation::BOOL;
    pub fn SubmitIoRing(ioring: *const HIORING__, waitoperations: u32, milliseconds: u32, submittedentries: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn TerminateLogArchive(pvarchivecontext: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn TerminateReadLog(pvcursorcontext: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn TruncateLog(pvmarshal: *const ::core::ffi::c_void, plsnend: *const CLS_LSN, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    pub fn TxfGetThreadMiniVersionForCreate(miniversion: *mut u16);
    #[cfg(feature = "Win32_Foundation")]
    pub fn TxfLogCreateFileReadContext(logpath: super::super::Foundation::PWSTR, beginninglsn: CLS_LSN, endinglsn: CLS_LSN, txffileid: *const TXF_ID, txflogcontext: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn TxfLogCreateRangeReadContext(logpath: super::super::Foundation::PWSTR, beginninglsn: CLS_LSN, endinglsn: CLS_LSN, beginningvirtualclock: *const i64, endingvirtualclock: *const i64, recordtypemask: u32, txflogcontext: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn TxfLogDestroyReadContext(txflogcontext: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn TxfLogReadRecords(txflogcontext: *const ::core::ffi::c_void, bufferlength: u32, buffer: *mut ::core::ffi::c_void, bytesused: *mut u32, recordcount: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn TxfLogRecordGetFileName(recordbuffer: *const ::core::ffi::c_void, recordbufferlengthinbytes: u32, namebuffer: super::super::Foundation::PWSTR, namebufferlengthinbytes: *mut u32, txfid: *mut TXF_ID) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn TxfLogRecordGetGenericType(recordbuffer: *const ::core::ffi::c_void, recordbufferlengthinbytes: u32, generictype: *mut u32, virtualclock: *mut i64) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn TxfReadMetadataInfo(filehandle: super::super::Foundation::HANDLE, txffileid: *mut TXF_ID, lastlsn: *mut CLS_LSN, transactionstate: *mut u32, lockingtransaction: *mut ::windows_sys::core::GUID) -> super::super::Foundation::BOOL;
    pub fn TxfSetThreadMiniVersionForCreate(miniversion: u16);
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnlockFile(hfile: super::super::Foundation::HANDLE, dwfileoffsetlow: u32, dwfileoffsethigh: u32, nnumberofbytestounlocklow: u32, nnumberofbytestounlockhigh: u32) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn UnlockFileEx(hfile: super::super::Foundation::HANDLE, dwreserved: u32, nnumberofbytestounlocklow: u32, nnumberofbytestounlockhigh: u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn ValidateLog(pszlogfilename: super::super::Foundation::PWSTR, psalogfile: *mut super::super::Security::SECURITY_ATTRIBUTES, pinfobuffer: *mut CLS_INFORMATION, pcbbuffer: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VerFindFileA(uflags: VER_FIND_FILE_FLAGS, szfilename: super::super::Foundation::PSTR, szwindir: super::super::Foundation::PSTR, szappdir: super::super::Foundation::PSTR, szcurdir: super::super::Foundation::PSTR, pucurdirlen: *mut u32, szdestdir: super::super::Foundation::PSTR, pudestdirlen: *mut u32) -> VER_FIND_FILE_STATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VerFindFileW(uflags: VER_FIND_FILE_FLAGS, szfilename: super::super::Foundation::PWSTR, szwindir: super::super::Foundation::PWSTR, szappdir: super::super::Foundation::PWSTR, szcurdir: super::super::Foundation::PWSTR, pucurdirlen: *mut u32, szdestdir: super::super::Foundation::PWSTR, pudestdirlen: *mut u32) -> VER_FIND_FILE_STATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VerInstallFileA(uflags: VER_INSTALL_FILE_FLAGS, szsrcfilename: super::super::Foundation::PSTR, szdestfilename: super::super::Foundation::PSTR, szsrcdir: super::super::Foundation::PSTR, szdestdir: super::super::Foundation::PSTR, szcurdir: super::super::Foundation::PSTR, sztmpfile: super::super::Foundation::PSTR, putmpfilelen: *mut u32) -> VER_INSTALL_FILE_STATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VerInstallFileW(uflags: VER_INSTALL_FILE_FLAGS, szsrcfilename: super::super::Foundation::PWSTR, szdestfilename: super::super::Foundation::PWSTR, szsrcdir: super::super::Foundation::PWSTR, szdestdir: super::super::Foundation::PWSTR, szcurdir: super::super::Foundation::PWSTR, sztmpfile: super::super::Foundation::PWSTR, putmpfilelen: *mut u32) -> VER_INSTALL_FILE_STATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VerLanguageNameA(wlang: u32, szlang: super::super::Foundation::PSTR, cchlang: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VerLanguageNameW(wlang: u32, szlang: super::super::Foundation::PWSTR, cchlang: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VerQueryValueA(pblock: *const ::core::ffi::c_void, lpsubblock: super::super::Foundation::PSTR, lplpbuffer: *mut *mut ::core::ffi::c_void, pulen: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VerQueryValueW(pblock: *const ::core::ffi::c_void, lpsubblock: super::super::Foundation::PWSTR, lplpbuffer: *mut *mut ::core::ffi::c_void, pulen: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WofEnumEntries(volumename: super::super::Foundation::PWSTR, provider: u32, enumproc: WofEnumEntryProc, userdata: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WofFileEnumFiles(volumename: super::super::Foundation::PWSTR, algorithm: u32, enumproc: WofEnumFilesProc, userdata: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WofGetDriverVersion(fileorvolumehandle: super::super::Foundation::HANDLE, provider: u32, wofversion: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WofIsExternalFile(filepath: super::super::Foundation::PWSTR, isexternalfile: *mut super::super::Foundation::BOOL, provider: *mut u32, externalfileinfo: *mut ::core::ffi::c_void, bufferlength: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WofSetFileDataLocation(filehandle: super::super::Foundation::HANDLE, provider: u32, externalfileinfo: *const ::core::ffi::c_void, length: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WofShouldCompressBinaries(volume: super::super::Foundation::PWSTR, algorithm: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WofWimAddEntry(volumename: super::super::Foundation::PWSTR, wimpath: super::super::Foundation::PWSTR, wimtype: u32, wimindex: u32, datasourceid: *mut i64) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WofWimEnumFiles(volumename: super::super::Foundation::PWSTR, datasourceid: i64, enumproc: WofEnumFilesProc, userdata: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WofWimRemoveEntry(volumename: super::super::Foundation::PWSTR, datasourceid: i64) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WofWimSuspendEntry(volumename: super::super::Foundation::PWSTR, datasourceid: i64) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WofWimUpdateEntry(volumename: super::super::Foundation::PWSTR, datasourceid: i64, newwimpath: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn Wow64DisableWow64FsRedirection(oldvalue: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn Wow64EnableWow64FsRedirection(wow64fsenableredirection: super::super::Foundation::BOOLEAN) -> super::super::Foundation::BOOLEAN;
    #[cfg(feature = "Win32_Foundation")]
    pub fn Wow64RevertWow64FsRedirection(olvalue: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    pub fn WriteEncryptedFileRaw(pfimportcallback: PFE_IMPORT_FUNC, pvcallbackcontext: *const ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WriteFile(hfile: super::super::Foundation::HANDLE, lpbuffer: *const ::core::ffi::c_void, nnumberofbytestowrite: u32, lpnumberofbyteswritten: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WriteFileEx(hfile: super::super::Foundation::HANDLE, lpbuffer: *const ::core::ffi::c_void, nnumberofbytestowrite: u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, lpcompletionroutine: super::super::System::IO::LPOVERLAPPED_COMPLETION_ROUTINE) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WriteFileGather(hfile: super::super::Foundation::HANDLE, asegmentarray: *const FILE_SEGMENT_ELEMENT, nnumberofbytestowrite: u32, lpreserved: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WriteLogRestartArea(pvmarshal: *mut ::core::ffi::c_void, pvrestartbuffer: *mut ::core::ffi::c_void, cbrestartbuffer: u32, plsnbase: *mut CLS_LSN, fflags: CLFS_FLAG, pcbwritten: *mut u32, plsnnext: *mut CLS_LSN, poverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WriteTapemark(hdevice: super::super::Foundation::HANDLE, dwtapemarktype: TAPEMARK_TYPE, dwtapemarkcount: u32, bimmediate: super::super::Foundation::BOOL) -> u32;
}
#[cfg(feature = "Win32_Foundation")]
pub struct BY_HANDLE_FILE_INFORMATION(i32);
pub struct CACHE_ACCESS_CHECK(i32);
pub struct CACHE_DESTROY_CALLBACK(i32);
pub struct CACHE_KEY_COMPARE(i32);
pub struct CACHE_KEY_HASH(i32);
pub struct CACHE_READ_CALLBACK(i32);
pub struct CLAIMMEDIALABEL(i32);
pub struct CLAIMMEDIALABELEX(i32);
pub struct CLFS_BLOCK_ALLOCATION(i32);
pub struct CLFS_BLOCK_DEALLOCATION(i32);
pub struct CLFS_CONTEXT_MODE(i32);
pub struct CLFS_FLAG(i32);
pub const CLFS_FLAG_FILTER_INTERMEDIATE_LEVEL: u32 = 16u32;
pub const CLFS_FLAG_FILTER_TOP_LEVEL: u32 = 32u32;
pub const CLFS_FLAG_HIDDEN_SYSTEM_LOG: u32 = 512u32;
pub const CLFS_FLAG_IGNORE_SHARE_ACCESS: u32 = 64u32;
pub const CLFS_FLAG_MINIFILTER_LEVEL: u32 = 256u32;
pub const CLFS_FLAG_NON_REENTRANT_FILTER: u32 = 16u32;
pub const CLFS_FLAG_READ_IN_PROGRESS: u32 = 128u32;
pub const CLFS_FLAG_REENTRANT_FILE_SYSTEM: u32 = 8u32;
pub const CLFS_FLAG_REENTRANT_FILTER: u32 = 32u32;
pub struct CLFS_IOSTATS_CLASS(i32);
pub struct CLFS_LOG_ARCHIVE_MODE(i32);
pub struct CLFS_LOG_NAME_INFORMATION(i32);
pub const CLFS_MARSHALLING_FLAG_DISABLE_BUFF_INIT: u32 = 1u32;
pub const CLFS_MARSHALLING_FLAG_NONE: u32 = 0u32;
pub const CLFS_MAX_CONTAINER_INFO: u32 = 256u32;
pub const CLFS_MGMT_CLIENT_REGISTRATION_VERSION: u32 = 1u32;
pub struct CLFS_MGMT_NOTIFICATION(i32);
pub struct CLFS_MGMT_NOTIFICATION_TYPE(i32);
pub struct CLFS_MGMT_POLICY(i32);
pub struct CLFS_MGMT_POLICY_TYPE(i32);
pub const CLFS_MGMT_POLICY_VERSION: u32 = 1u32;
pub struct CLFS_NODE_ID(i32);
pub struct CLFS_PHYSICAL_LSN_INFORMATION(i32);
pub const CLFS_SCAN_BACKWARD: u8 = 4u8;
pub const CLFS_SCAN_BUFFERED: u8 = 32u8;
pub const CLFS_SCAN_CLOSE: u8 = 8u8;
pub const CLFS_SCAN_FORWARD: u8 = 2u8;
pub const CLFS_SCAN_INIT: u8 = 1u8;
pub const CLFS_SCAN_INITIALIZED: u8 = 16u8;
pub struct CLFS_STREAM_ID_INFORMATION(i32);
pub const CLSID_DiskQuotaControl: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2039002481, data2: 60553, data3: 4559, data4: [156, 0, 0, 170, 0, 161, 79, 86] };
pub struct CLS_ARCHIVE_DESCRIPTOR(i32);
pub struct CLS_CONTAINER_INFORMATION(i32);
pub struct CLS_CONTEXT_MODE(i32);
pub struct CLS_INFORMATION(i32);
pub struct CLS_IOSTATS_CLASS(i32);
pub struct CLS_IO_STATISTICS(i32);
pub struct CLS_IO_STATISTICS_HEADER(i32);
pub struct CLS_LOG_INFORMATION_CLASS(i32);
pub struct CLS_LSN(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CLS_SCAN_CONTEXT(i32);
pub struct CLS_WRITE_ENTRY(i32);
pub struct CONNECTION_INFO_0(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CONNECTION_INFO_1(i32);
pub struct COPYFILE2_COPY_PHASE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct COPYFILE2_EXTENDED_PARAMETERS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct COPYFILE2_EXTENDED_PARAMETERS_V2(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct COPYFILE2_MESSAGE(i32);
pub struct COPYFILE2_MESSAGE_ACTION(i32);
pub struct COPYFILE2_MESSAGE_TYPE(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct CREATEFILE2_EXTENDED_PARAMETERS(i32);
pub struct CREATE_TAPE_PARTITION_METHOD(i32);
pub const CRM_PROTOCOL_DYNAMIC_MARSHAL_INFO: u32 = 2u32;
pub const CRM_PROTOCOL_EXPLICIT_MARSHAL_ONLY: u32 = 1u32;
pub const CRM_PROTOCOL_MAXIMUM_OPTION: u32 = 3u32;
pub const CSC_CACHE_AUTO_REINT: u32 = 16u32;
pub const CSC_CACHE_MANUAL_REINT: u32 = 0u32;
pub const CSC_CACHE_NONE: u32 = 48u32;
pub const CSC_CACHE_VDO: u32 = 32u32;
pub const CSC_MASK: u32 = 48u32;
pub const CSC_MASK_EXT: u32 = 8240u32;
pub const CSV_BLOCK_AND_FILE_CACHE_CALLBACK_VERSION: u32 = 2u32;
pub const CSV_BLOCK_CACHE_CALLBACK_VERSION: u32 = 1u32;
pub const ClfsClientRecord: u8 = 3u8;
pub const ClfsContainerActive: u32 = 4u32;
pub const ClfsContainerActivePendingDelete: u32 = 8u32;
pub const ClfsContainerInactive: u32 = 2u32;
pub const ClfsContainerInitializing: u32 = 1u32;
pub const ClfsContainerPendingArchive: u32 = 16u32;
pub const ClfsContainerPendingArchiveAndDelete: u32 = 32u32;
pub const ClfsDataRecord: u8 = 1u8;
pub const ClfsNullRecord: u8 = 0u8;
pub const ClfsRestartRecord: u8 = 2u8;
pub const ClsContainerActive: u32 = 4u32;
pub const ClsContainerActivePendingDelete: u32 = 8u32;
pub const ClsContainerInactive: u32 = 2u32;
pub const ClsContainerInitializing: u32 = 1u32;
pub const ClsContainerPendingArchive: u32 = 16u32;
pub const ClsContainerPendingArchiveAndDelete: u32 = 32u32;
pub struct DEFINE_DOS_DEVICE_FLAGS(i32);
pub const DISKQUOTA_FILESTATE_INCOMPLETE: u32 = 256u32;
pub const DISKQUOTA_FILESTATE_MASK: u32 = 768u32;
pub const DISKQUOTA_FILESTATE_REBUILDING: u32 = 512u32;
pub const DISKQUOTA_LOGFLAG_USER_LIMIT: u32 = 2u32;
pub const DISKQUOTA_LOGFLAG_USER_THRESHOLD: u32 = 1u32;
pub const DISKQUOTA_STATE_DISABLED: u32 = 0u32;
pub const DISKQUOTA_STATE_ENFORCE: u32 = 2u32;
pub const DISKQUOTA_STATE_MASK: u32 = 3u32;
pub const DISKQUOTA_STATE_TRACK: u32 = 1u32;
pub struct DISKQUOTA_USERNAME_RESOLVE(i32);
pub const DISKQUOTA_USER_ACCOUNT_DELETED: u32 = 2u32;
pub const DISKQUOTA_USER_ACCOUNT_INVALID: u32 = 3u32;
pub const DISKQUOTA_USER_ACCOUNT_RESOLVED: u32 = 0u32;
pub const DISKQUOTA_USER_ACCOUNT_UNAVAILABLE: u32 = 1u32;
pub const DISKQUOTA_USER_ACCOUNT_UNKNOWN: u32 = 4u32;
pub const DISKQUOTA_USER_ACCOUNT_UNRESOLVED: u32 = 5u32;
pub struct DISKQUOTA_USER_INFORMATION(i32);
pub struct DISK_SPACE_INFORMATION(i32);
pub struct EFS_CERTIFICATE_BLOB(i32);
pub struct EFS_COMPATIBILITY_INFO(i32);
pub const EFS_COMPATIBILITY_VERSION_NCRYPT_PROTECTOR: u32 = 5u32;
pub const EFS_COMPATIBILITY_VERSION_PFILE_PROTECTOR: u32 = 6u32;
pub struct EFS_DECRYPTION_STATUS_INFO(i32);
pub const EFS_EFS_SUBVER_EFS_CERT: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
pub struct EFS_ENCRYPTION_STATUS_INFO(i32);
pub struct EFS_HASH_BLOB(i32);
pub struct EFS_KEY_INFO(i32);
pub const EFS_METADATA_ADD_USER: u32 = 1u32;
pub const EFS_METADATA_GENERAL_OP: u32 = 8u32;
pub const EFS_METADATA_REMOVE_USER: u32 = 2u32;
pub const EFS_METADATA_REPLACE_USER: u32 = 4u32;
pub const EFS_PFILE_SUBVER_APPX: u32 = 3u32;
pub const EFS_PFILE_SUBVER_RMS: u32 = 2u32;
pub struct EFS_PIN_BLOB(i32);
pub struct EFS_RPC_BLOB(i32);
pub const EFS_SUBVER_UNKNOWN: u32 = 0u32;
pub struct EFS_VERSION_INFO(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct ENCRYPTED_FILE_METADATA_SIGNATURE(i32);
#[cfg(feature = "Win32_Security")]
pub struct ENCRYPTION_CERTIFICATE(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct ENCRYPTION_CERTIFICATE_HASH(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct ENCRYPTION_CERTIFICATE_HASH_LIST(i32);
#[cfg(feature = "Win32_Security")]
pub struct ENCRYPTION_CERTIFICATE_LIST(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct ENCRYPTION_PROTECTOR(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct ENCRYPTION_PROTECTOR_LIST(i32);
pub const ENLISTMENT_MAXIMUM_OPTION: u32 = 1u32;
pub const ENLISTMENT_SUPERIOR: u32 = 1u32;
pub struct ERASE_TAPE_TYPE(i32);
pub struct FCACHE_CREATE_CALLBACK(i32);
pub struct FCACHE_RICHCREATE_CALLBACK(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct FH_OVERLAPPED(i32);
pub struct FILE_ACCESS_FLAGS(i32);
pub struct FILE_ACTION(i32);
pub struct FILE_ALIGNMENT_INFO(i32);
pub struct FILE_ALLOCATION_INFO(i32);
pub struct FILE_ATTRIBUTE_TAG_INFO(i32);
pub struct FILE_BASIC_INFO(i32);
pub struct FILE_COMPRESSION_INFO(i32);
pub struct FILE_CREATION_DISPOSITION(i32);
pub struct FILE_DEVICE_TYPE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct FILE_DISPOSITION_INFO(i32);
pub struct FILE_END_OF_FILE_INFO(i32);
pub struct FILE_EXTENT(i32);
pub struct FILE_FLAGS_AND_ATTRIBUTES(i32);
pub struct FILE_FULL_DIR_INFO(i32);
pub struct FILE_ID_128(i32);
pub struct FILE_ID_BOTH_DIR_INFO(i32);
pub struct FILE_ID_DESCRIPTOR(i32);
pub struct FILE_ID_EXTD_DIR_INFO(i32);
pub struct FILE_ID_INFO(i32);
pub struct FILE_ID_TYPE(i32);
pub struct FILE_INFO_2(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct FILE_INFO_3(i32);
pub struct FILE_INFO_BY_HANDLE_CLASS(i32);
pub struct FILE_INFO_FLAGS_PERMISSIONS(i32);
pub struct FILE_IO_PRIORITY_HINT_INFO(i32);
pub struct FILE_NAME(i32);
pub struct FILE_NAME_INFO(i32);
pub struct FILE_NOTIFY_CHANGE(i32);
pub struct FILE_NOTIFY_EXTENDED_INFORMATION(i32);
pub struct FILE_NOTIFY_INFORMATION(i32);
pub const FILE_PROVIDER_COMPRESSION_LZX: u32 = 1u32;
pub const FILE_PROVIDER_COMPRESSION_XPRESS16K: u32 = 3u32;
pub const FILE_PROVIDER_COMPRESSION_XPRESS4K: u32 = 0u32;
pub const FILE_PROVIDER_COMPRESSION_XPRESS8K: u32 = 2u32;
pub struct FILE_REMOTE_PROTOCOL_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct FILE_RENAME_INFO(i32);
pub struct FILE_SEGMENT_ELEMENT(i32);
pub struct FILE_SHARE_MODE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct FILE_STANDARD_INFO(i32);
pub struct FILE_STORAGE_INFO(i32);
pub struct FILE_STREAM_INFO(i32);
pub struct FINDEX_INFO_LEVELS(i32);
pub struct FINDEX_SEARCH_OPS(i32);
pub struct FIND_FIRST_EX_FLAGS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct FIO_CONTEXT(i32);
pub struct FindChangeNotificationHandle(i32);
pub struct FindFileHandle(i32);
pub struct FindFileNameHandle(i32);
pub struct FindStreamHandle(i32);
pub struct FindVolumeHandle(i32);
pub struct FindVolumeMointPointHandle(i32);
pub struct GET_FILEEX_INFO_LEVELS(i32);
pub struct GET_FILE_VERSION_INFO_FLAGS(i32);
pub struct GET_TAPE_DRIVE_PARAMETERS_OPERATION(i32);
pub struct HIORING__(i32);
#[repr(transparent)]
pub struct IDiskQuotaControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDiskQuotaEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDiskQuotaUser(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDiskQuotaUserBatch(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumDiskQuotaUsers(pub *mut ::core::ffi::c_void);
pub const INVALID_FILE_ATTRIBUTES: u32 = 4294967295u32;
pub const INVALID_SET_FILE_POINTER: u32 = 4294967295u32;
pub const IOCTL_VOLUME_ALLOCATE_BC_STREAM: u32 = 5685312u32;
pub const IOCTL_VOLUME_BASE: u32 = 86u32;
pub const IOCTL_VOLUME_BC_VERSION: u32 = 1u32;
pub const IOCTL_VOLUME_FREE_BC_STREAM: u32 = 5685316u32;
pub const IOCTL_VOLUME_GET_BC_PROPERTIES: u32 = 5652540u32;
pub const IOCTL_VOLUME_GET_CSVBLOCKCACHE_CALLBACK: u32 = 5685352u32;
pub const IOCTL_VOLUME_GET_GPT_ATTRIBUTES: u32 = 5636152u32;
pub const IOCTL_VOLUME_GET_VOLUME_DISK_EXTENTS: u32 = 5636096u32;
pub const IOCTL_VOLUME_IS_CLUSTERED: u32 = 5636144u32;
pub const IOCTL_VOLUME_IS_CSV: u32 = 5636192u32;
pub const IOCTL_VOLUME_IS_DYNAMIC: u32 = 5636168u32;
pub const IOCTL_VOLUME_IS_IO_CAPABLE: u32 = 5636116u32;
pub const IOCTL_VOLUME_IS_OFFLINE: u32 = 5636112u32;
pub const IOCTL_VOLUME_IS_PARTITION: u32 = 5636136u32;
pub const IOCTL_VOLUME_LOGICAL_TO_PHYSICAL: u32 = 5636128u32;
pub const IOCTL_VOLUME_OFFLINE: u32 = 5685260u32;
pub const IOCTL_VOLUME_ONLINE: u32 = 5685256u32;
pub const IOCTL_VOLUME_PHYSICAL_TO_LOGICAL: u32 = 5636132u32;
pub const IOCTL_VOLUME_POST_ONLINE: u32 = 5685348u32;
pub const IOCTL_VOLUME_PREPARE_FOR_CRITICAL_IO: u32 = 5685324u32;
pub const IOCTL_VOLUME_PREPARE_FOR_SHRINK: u32 = 5685340u32;
pub const IOCTL_VOLUME_QUERY_ALLOCATION_HINT: u32 = 5652562u32;
pub const IOCTL_VOLUME_QUERY_FAILOVER_SET: u32 = 5636120u32;
pub const IOCTL_VOLUME_QUERY_MINIMUM_SHRINK_SIZE: u32 = 5652568u32;
pub const IOCTL_VOLUME_QUERY_VOLUME_NUMBER: u32 = 5636124u32;
pub const IOCTL_VOLUME_READ_PLEX: u32 = 5652526u32;
pub const IOCTL_VOLUME_SET_GPT_ATTRIBUTES: u32 = 5636148u32;
pub const IOCTL_VOLUME_SUPPORTS_ONLINE_OFFLINE: u32 = 5636100u32;
pub const IOCTL_VOLUME_UPDATE_PROPERTIES: u32 = 5636180u32;
pub struct IORING_BUFFER_INFO(i32);
pub struct IORING_BUFFER_REF(i32);
pub struct IORING_CAPABILITIES(i32);
pub struct IORING_CQE(i32);
pub struct IORING_CREATE_ADVISORY_FLAGS(i32);
pub struct IORING_CREATE_FLAGS(i32);
pub struct IORING_CREATE_REQUIRED_FLAGS(i32);
pub struct IORING_FEATURE_FLAGS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct IORING_HANDLE_REF(i32);
pub struct IORING_INFO(i32);
pub struct IORING_OP_CODE(i32);
pub struct IORING_REF_KIND(i32);
pub struct IORING_REGISTERED_BUFFER(i32);
pub struct IORING_SQE_FLAGS(i32);
pub struct IORING_VERSION(i32);
pub struct KCRM_MARSHAL_HEADER(i32);
pub struct KCRM_PROTOCOL_BLOB(i32);
pub struct KCRM_TRANSACTION_BLOB(i32);
pub const KTM_MARSHAL_BLOB_VERSION_MAJOR: u32 = 1u32;
pub const KTM_MARSHAL_BLOB_VERSION_MINOR: u32 = 1u32;
pub struct LOCK_FILE_FLAGS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct LOG_MANAGEMENT_CALLBACKS(i32);
pub const LOG_POLICY_OVERWRITE: u32 = 1u32;
pub const LOG_POLICY_PERSIST: u32 = 2u32;
pub struct LPPROGRESS_ROUTINE(i32);
pub struct LPPROGRESS_ROUTINE_CALLBACK_REASON(i32);
pub const LZERROR_BADINHANDLE: i32 = -1i32;
pub const LZERROR_BADOUTHANDLE: i32 = -2i32;
pub const LZERROR_BADVALUE: i32 = -7i32;
pub const LZERROR_GLOBALLOC: i32 = -5i32;
pub const LZERROR_GLOBLOCK: i32 = -6i32;
pub const LZERROR_READ: i32 = -3i32;
pub const LZERROR_UNKNOWNALG: i32 = -8i32;
pub const LZERROR_WRITE: i32 = -4i32;
pub struct LZOPENFILE_STYLE(i32);
pub struct MAXMEDIALABEL(i32);
pub const MAX_RESOURCEMANAGER_DESCRIPTION_LENGTH: u32 = 64u32;
pub const MAX_SID_SIZE: u32 = 256u32;
pub const MAX_TRANSACTION_DESCRIPTION_LENGTH: u32 = 64u32;
pub struct MOVE_FILE_FLAGS(i32);
pub struct MediaLabelInfo(i32);
pub struct NAME_CACHE_CONTEXT(i32);
pub const NTMSMLI_MAXAPPDESCR: u32 = 256u32;
pub const NTMSMLI_MAXIDSIZE: u32 = 256u32;
pub const NTMSMLI_MAXTYPE: u32 = 64u32;
pub struct NTMS_ALLOCATION_INFORMATION(i32);
pub const NTMS_APPLICATIONNAME_LENGTH: u32 = 64u32;
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_ASYNC_IO(i32);
pub const NTMS_BARCODE_LENGTH: u32 = 64u32;
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_CHANGERINFORMATIONA(i32);
pub struct NTMS_CHANGERINFORMATIONW(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_CHANGERTYPEINFORMATIONA(i32);
pub struct NTMS_CHANGERTYPEINFORMATIONW(i32);
pub struct NTMS_COMPUTERINFORMATION(i32);
pub const NTMS_COMPUTERNAME_LENGTH: u32 = 64u32;
pub const NTMS_DESCRIPTION_LENGTH: u32 = 127u32;
pub const NTMS_DEVICENAME_LENGTH: u32 = 64u32;
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_DRIVEINFORMATIONA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_DRIVEINFORMATIONW(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_DRIVETYPEINFORMATIONA(i32);
pub struct NTMS_DRIVETYPEINFORMATIONW(i32);
pub struct NTMS_FILESYSTEM_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_I1_LIBRARYINFORMATION(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_I1_LIBREQUESTINFORMATIONA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_I1_LIBREQUESTINFORMATIONW(i32);
pub const NTMS_I1_MESSAGE_LENGTH: u32 = 127u32;
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_I1_OBJECTINFORMATIONA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_I1_OBJECTINFORMATIONW(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_I1_OPREQUESTINFORMATIONA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_I1_OPREQUESTINFORMATIONW(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_I1_PARTITIONINFORMATIONA(i32);
pub struct NTMS_I1_PARTITIONINFORMATIONW(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_I1_PMIDINFORMATIONA(i32);
pub struct NTMS_I1_PMIDINFORMATIONW(i32);
pub struct NTMS_IEDOORINFORMATION(i32);
pub struct NTMS_IEPORTINFORMATION(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_LIBRARYINFORMATION(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_LIBREQUESTINFORMATIONA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_LIBREQUESTINFORMATIONW(i32);
pub struct NTMS_LMIDINFORMATION(i32);
pub const NTMS_MAXATTR_LENGTH: u32 = 65536u32;
pub const NTMS_MAXATTR_NAMELEN: u32 = 32u32;
pub struct NTMS_MEDIAPOOLINFORMATION(i32);
pub struct NTMS_MEDIATYPEINFORMATION(i32);
pub const NTMS_MESSAGE_LENGTH: u32 = 256u32;
pub struct NTMS_MOUNT_INFORMATION(i32);
pub struct NTMS_NOTIFICATIONINFORMATION(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_OBJECTINFORMATIONA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_OBJECTINFORMATIONW(i32);
pub const NTMS_OBJECTNAME_LENGTH: u32 = 64u32;
pub const NTMS_OMIDLABELID_LENGTH: u32 = 255u32;
pub const NTMS_OMIDLABELINFO_LENGTH: u32 = 256u32;
pub const NTMS_OMIDLABELTYPE_LENGTH: u32 = 64u32;
pub struct NTMS_OMID_TYPE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_OPREQUESTINFORMATIONA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_OPREQUESTINFORMATIONW(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_PARTITIONINFORMATIONA(i32);
pub struct NTMS_PARTITIONINFORMATIONW(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_PMIDINFORMATIONA(i32);
pub struct NTMS_PMIDINFORMATIONW(i32);
pub const NTMS_POOLHIERARCHY_LENGTH: u32 = 512u32;
pub const NTMS_PRODUCTNAME_LENGTH: u32 = 128u32;
pub const NTMS_REVISION_LENGTH: u32 = 32u32;
pub const NTMS_SEQUENCE_LENGTH: u32 = 32u32;
pub const NTMS_SERIALNUMBER_LENGTH: u32 = 32u32;
pub struct NTMS_STORAGESLOTINFORMATION(i32);
pub const NTMS_USERNAME_LENGTH: u32 = 64u32;
pub const NTMS_VENDORNAME_LENGTH: u32 = 128u32;
pub struct NT_CREATE_FILE_DISPOSITION(i32);
pub struct NtmsAccessMask(i32);
pub struct NtmsAllocateOptions(i32);
pub struct NtmsAllocationPolicy(i32);
pub struct NtmsAsyncOperations(i32);
pub struct NtmsAsyncStatus(i32);
pub struct NtmsBarCodeState(i32);
pub struct NtmsCreateNtmsMediaOptions(i32);
pub struct NtmsCreateOptions(i32);
pub struct NtmsDeallocationPolicy(i32);
pub struct NtmsDismountOptions(i32);
pub struct NtmsDoorState(i32);
pub struct NtmsDriveState(i32);
pub struct NtmsDriveType(i32);
pub struct NtmsEjectOperation(i32);
pub struct NtmsEnumerateOption(i32);
pub struct NtmsInjectOperation(i32);
pub struct NtmsInventoryMethod(i32);
pub struct NtmsLibRequestFlags(i32);
pub struct NtmsLibraryFlags(i32);
pub struct NtmsLibraryType(i32);
pub struct NtmsLmOperation(i32);
pub struct NtmsLmState(i32);
pub struct NtmsMediaPoolPolicy(i32);
pub struct NtmsMediaState(i32);
pub struct NtmsMountOptions(i32);
pub struct NtmsMountPriority(i32);
pub struct NtmsNotificationOperations(i32);
pub struct NtmsObjectsTypes(i32);
pub struct NtmsOpRequestFlags(i32);
pub struct NtmsOperationalState(i32);
pub struct NtmsOpreqCommand(i32);
pub struct NtmsOpreqState(i32);
pub struct NtmsPartitionState(i32);
pub struct NtmsPoolType(i32);
pub struct NtmsPortContent(i32);
pub struct NtmsPortPosition(i32);
pub struct NtmsReadWriteCharacteristics(i32);
pub struct NtmsSessionOptions(i32);
pub struct NtmsSlotState(i32);
pub struct NtmsUIOperations(i32);
pub struct NtmsUITypes(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct OFSTRUCT(i32);
pub const PARTITION_BASIC_DATA_GUID: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3956318370,
    data2: 47589,
    data3: 17459,
    data4: [135, 192, 104, 182, 183, 38, 153, 199],
};
pub const PARTITION_BSP_GUID: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1464029011, data2: 19961, data3: 17849, data4: [142, 158, 35, 112, 240, 6, 69, 124] };
pub const PARTITION_CLUSTER_GUID: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3684162473,
    data2: 2112,
    data3: 19374,
    data4: [151, 240, 255, 185, 163, 39, 199, 225],
};
pub const PARTITION_DPP_GUID: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1464029011,
    data2: 38091,
    data3: 17392,
    data4: [165, 51, 215, 60, 16, 207, 165, 125],
};
pub const PARTITION_ENTRY_UNUSED_GUID: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 0, data2: 0, data3: 0, data4: [0, 0, 0, 0, 0, 0, 0, 0] };
pub const PARTITION_LDM_DATA_GUID: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2946195616, data2: 5169, data3: 20322, data4: [188, 104, 51, 17, 113, 74, 105, 173] };
pub const PARTITION_LDM_METADATA_GUID: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1476970666,
    data2: 32399,
    data3: 17120,
    data4: [133, 210, 225, 233, 4, 52, 207, 179],
};
pub const PARTITION_LEGACY_BL_GUID: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1112318178,
    data2: 31922,
    data3: 20409,
    data4: [129, 67, 197, 42, 153, 57, 139, 198],
};
pub const PARTITION_LEGACY_BL_GUID_BACKUP: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1112292972,
    data2: 55199,
    data3: 18891,
    data4: [147, 93, 54, 215, 20, 103, 162, 136],
};
pub const PARTITION_MAIN_OS_GUID: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1464029011, data2: 36677, data3: 16478, data4: [138, 35, 24, 109, 138, 67, 48, 211] };
pub const PARTITION_MSFT_RECOVERY_GUID: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3734289316,
    data2: 1745,
    data3: 19776,
    data4: [161, 106, 191, 213, 1, 121, 214, 172],
};
pub const PARTITION_MSFT_RESERVED_GUID: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3821658902, data2: 2908, data3: 19896, data4: [129, 125, 249, 45, 240, 2, 21, 174] };
pub const PARTITION_MSFT_SNAPSHOT_GUID: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3403541489, data2: 17408, data3: 19944, data4: [177, 3, 18, 17, 125, 207, 60, 207] };
pub const PARTITION_OS_DATA_GUID: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1464029011, data2: 9202, data3: 17621, data4: [168, 48, 103, 187, 218, 166, 9, 249] };
pub const PARTITION_PATCH_GUID: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2305271430, data2: 38570, data3: 27304, data4: [149, 137, 168, 66, 86, 84, 16, 144] };
pub const PARTITION_PRE_INSTALLED_GUID: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1464029011, data2: 32736, data3: 16790, data4: [155, 66, 66, 123, 81, 100, 52, 132] };
pub const PARTITION_SERVICING_FILES_GUID: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1464029011,
    data2: 17198,
    data3: 16404,
    data4: [174, 76, 141, 234, 169, 192, 0, 106],
};
pub const PARTITION_SERVICING_METADATA_GUID: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1464029011,
    data2: 50833,
    data3: 18949,
    data4: [187, 78, 112, 61, 175, 210, 41, 206],
};
pub const PARTITION_SERVICING_RESERVE_GUID: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1464029011,
    data2: 19329,
    data3: 17931,
    data4: [163, 25, 255, 182, 254, 19, 109, 20],
};
pub const PARTITION_SERVICING_STAGING_ROOT_GUID: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1464029011,
    data2: 59469,
    data3: 20100,
    data4: [170, 243, 236, 187, 189, 4, 185, 223],
};
pub const PARTITION_SPACES_DATA_GUID: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3886931124,
    data2: 56372,
    data3: 17721,
    data4: [154, 118, 235, 189, 7, 190, 111, 126],
};
pub const PARTITION_SPACES_GUID: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3881611151,
    data2: 63104,
    data3: 19694,
    data4: [175, 163, 176, 1, 229, 110, 252, 45],
};
pub const PARTITION_SYSTEM_GUID: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3240784680, data2: 63519, data3: 4562, data4: [186, 75, 0, 160, 201, 62, 201, 59] };
pub const PARTITION_WINDOWS_SYSTEM_GUID: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1464029011,
    data2: 58339,
    data3: 17969,
    data4: [165, 197, 38, 210, 36, 56, 115, 170],
};
pub struct PCLFS_COMPLETION_ROUTINE(i32);
pub struct PCOPYFILE2_PROGRESS_ROUTINE(i32);
pub struct PFE_EXPORT_FUNC(i32);
pub struct PFE_IMPORT_FUNC(i32);
pub struct PFN_IO_COMPLETION(i32);
pub struct PLOG_FULL_HANDLER_CALLBACK(i32);
pub struct PLOG_TAIL_ADVANCE_CALLBACK(i32);
pub struct PLOG_UNPINNED_CALLBACK(i32);
pub struct PREPARE_TAPE_OPERATION(i32);
pub struct PRIORITY_HINT(i32);
pub struct READ_DIRECTORY_NOTIFY_INFORMATION_CLASS(i32);
pub struct REPARSE_GUID_DATA_BUFFER(i32);
pub struct REPLACE_FILE_FLAGS(i32);
pub const RESOURCE_MANAGER_COMMUNICATION: u32 = 2u32;
pub const RESOURCE_MANAGER_MAXIMUM_OPTION: u32 = 3u32;
pub const RESOURCE_MANAGER_VOLATILE: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
pub struct SERVER_ALIAS_INFO_0(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct SERVER_CERTIFICATE_INFO_0(i32);
pub struct SERVER_CERTIFICATE_TYPE(i32);
pub const SESI1_NUM_ELEMENTS: u32 = 8u32;
pub const SESI2_NUM_ELEMENTS: u32 = 9u32;
#[cfg(feature = "Win32_Foundation")]
pub struct SESSION_INFO_0(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct SESSION_INFO_1(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct SESSION_INFO_10(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct SESSION_INFO_2(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct SESSION_INFO_502(i32);
pub struct SESSION_INFO_USER_FLAGS(i32);
pub struct SET_FILE_POINTER_MOVE_METHOD(i32);
pub const SHARE_CURRENT_USES_PARMNUM: u32 = 7u32;
pub const SHARE_FILE_SD_PARMNUM: u32 = 501u32;
#[cfg(feature = "Win32_Foundation")]
pub struct SHARE_INFO_0(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct SHARE_INFO_1(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct SHARE_INFO_1004(i32);
pub struct SHARE_INFO_1005(i32);
pub struct SHARE_INFO_1006(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct SHARE_INFO_1501(i32);
pub struct SHARE_INFO_1503(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct SHARE_INFO_2(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct SHARE_INFO_501(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct SHARE_INFO_502(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct SHARE_INFO_503(i32);
pub struct SHARE_INFO_PERMISSIONS(i32);
pub const SHARE_MAX_USES_PARMNUM: u32 = 6u32;
pub const SHARE_NETNAME_PARMNUM: u32 = 1u32;
pub const SHARE_PASSWD_PARMNUM: u32 = 9u32;
pub const SHARE_PATH_PARMNUM: u32 = 8u32;
pub const SHARE_PERMISSIONS_PARMNUM: u32 = 5u32;
pub const SHARE_REMARK_PARMNUM: u32 = 4u32;
pub const SHARE_SERVER_PARMNUM: u32 = 503u32;
pub struct SHARE_TYPE(i32);
pub const SHARE_TYPE_PARMNUM: u32 = 3u32;
pub const SHI1005_FLAGS_ACCESS_BASED_DIRECTORY_ENUM: u32 = 2048u32;
pub const SHI1005_FLAGS_ALLOW_NAMESPACE_CACHING: u32 = 1024u32;
pub const SHI1005_FLAGS_CLUSTER_MANAGED: u32 = 524288u32;
pub const SHI1005_FLAGS_COMPRESS_DATA: u32 = 1048576u32;
pub const SHI1005_FLAGS_DFS: u32 = 1u32;
pub const SHI1005_FLAGS_DFS_ROOT: u32 = 2u32;
pub const SHI1005_FLAGS_DISABLE_CLIENT_BUFFERING: u32 = 131072u32;
pub const SHI1005_FLAGS_ENABLE_CA: u32 = 16384u32;
pub const SHI1005_FLAGS_ENABLE_HASH: u32 = 8192u32;
pub const SHI1005_FLAGS_ENCRYPT_DATA: u32 = 32768u32;
pub const SHI1005_FLAGS_FORCE_LEVELII_OPLOCK: u32 = 4096u32;
pub const SHI1005_FLAGS_FORCE_SHARED_DELETE: u32 = 512u32;
pub const SHI1005_FLAGS_IDENTITY_REMOTING: u32 = 262144u32;
pub const SHI1005_FLAGS_RESERVED: u32 = 65536u32;
pub const SHI1005_FLAGS_RESTRICT_EXCLUSIVE_OPENS: u32 = 256u32;
pub const SHI1_NUM_ELEMENTS: u32 = 4u32;
pub const SHI2_NUM_ELEMENTS: u32 = 10u32;
pub const SHI_USES_UNLIMITED: u32 = 4294967295u32;
pub const STATSOPT_CLR: u32 = 1u32;
pub struct STAT_SERVER_0(i32);
pub struct STAT_WORKSTATION_0(i32);
pub struct STORAGE_BUS_TYPE(i32);
pub struct STREAM_INFO_LEVELS(i32);
pub const STYPE_RESERVED1: u32 = 16777216u32;
pub const STYPE_RESERVED2: u32 = 33554432u32;
pub const STYPE_RESERVED3: u32 = 67108864u32;
pub const STYPE_RESERVED4: u32 = 134217728u32;
pub const STYPE_RESERVED5: u32 = 1048576u32;
pub const STYPE_RESERVED_ALL: u32 = 1073741568u32;
pub struct SYMBOLIC_LINK_FLAGS(i32);
pub struct TAPEMARK_TYPE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct TAPE_ERASE(i32);
pub struct TAPE_GET_POSITION(i32);
pub struct TAPE_INFORMATION_TYPE(i32);
pub struct TAPE_POSITION_METHOD(i32);
pub struct TAPE_POSITION_TYPE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct TAPE_PREPARE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct TAPE_SET_POSITION(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct TAPE_WRITE_MARKS(i32);
pub const TRANSACTION_DO_NOT_PROMOTE: u32 = 1u32;
pub const TRANSACTION_MANAGER_COMMIT_DEFAULT: u32 = 0u32;
pub const TRANSACTION_MANAGER_COMMIT_LOWEST: u32 = 8u32;
pub const TRANSACTION_MANAGER_COMMIT_SYSTEM_HIVES: u32 = 4u32;
pub const TRANSACTION_MANAGER_COMMIT_SYSTEM_VOLUME: u32 = 2u32;
pub const TRANSACTION_MANAGER_CORRUPT_FOR_PROGRESS: u32 = 32u32;
pub const TRANSACTION_MANAGER_CORRUPT_FOR_RECOVERY: u32 = 16u32;
pub const TRANSACTION_MANAGER_MAXIMUM_OPTION: u32 = 63u32;
pub const TRANSACTION_MANAGER_VOLATILE: u32 = 1u32;
pub const TRANSACTION_MAXIMUM_OPTION: u32 = 1u32;
pub struct TRANSACTION_NOTIFICATION(i32);
pub struct TRANSACTION_NOTIFICATION_MARSHAL_ARGUMENT(i32);
pub struct TRANSACTION_NOTIFICATION_PROPAGATE_ARGUMENT(i32);
pub struct TRANSACTION_NOTIFICATION_RECOVERY_ARGUMENT(i32);
pub struct TRANSACTION_NOTIFICATION_SAVEPOINT_ARGUMENT(i32);
pub struct TRANSACTION_NOTIFICATION_TM_ONLINE_ARGUMENT(i32);
pub const TRANSACTION_NOTIFICATION_TM_ONLINE_FLAG_IS_CLUSTERED: u32 = 1u32;
pub const TRANSACTION_NOTIFY_COMMIT: u32 = 4u32;
pub const TRANSACTION_NOTIFY_COMMIT_COMPLETE: u32 = 64u32;
pub const TRANSACTION_NOTIFY_COMMIT_FINALIZE: u32 = 1073741824u32;
pub const TRANSACTION_NOTIFY_COMMIT_REQUEST: u32 = 67108864u32;
pub const TRANSACTION_NOTIFY_DELEGATE_COMMIT: u32 = 1024u32;
pub const TRANSACTION_NOTIFY_ENLIST_MASK: u32 = 262144u32;
pub const TRANSACTION_NOTIFY_ENLIST_PREPREPARE: u32 = 4096u32;
pub const TRANSACTION_NOTIFY_INDOUBT: u32 = 16384u32;
pub const TRANSACTION_NOTIFY_LAST_RECOVER: u32 = 8192u32;
pub const TRANSACTION_NOTIFY_MARSHAL: u32 = 131072u32;
pub const TRANSACTION_NOTIFY_MASK: u32 = 1073741823u32;
pub const TRANSACTION_NOTIFY_PREPARE: u32 = 2u32;
pub const TRANSACTION_NOTIFY_PREPARE_COMPLETE: u32 = 32u32;
pub const TRANSACTION_NOTIFY_PREPREPARE: u32 = 1u32;
pub const TRANSACTION_NOTIFY_PREPREPARE_COMPLETE: u32 = 16u32;
pub const TRANSACTION_NOTIFY_PROMOTE: u32 = 134217728u32;
pub const TRANSACTION_NOTIFY_PROMOTE_NEW: u32 = 268435456u32;
pub const TRANSACTION_NOTIFY_PROPAGATE_PULL: u32 = 32768u32;
pub const TRANSACTION_NOTIFY_PROPAGATE_PUSH: u32 = 65536u32;
pub const TRANSACTION_NOTIFY_RECOVER: u32 = 256u32;
pub const TRANSACTION_NOTIFY_RECOVER_QUERY: u32 = 2048u32;
pub const TRANSACTION_NOTIFY_REQUEST_OUTCOME: u32 = 536870912u32;
pub const TRANSACTION_NOTIFY_RM_DISCONNECTED: u32 = 16777216u32;
pub const TRANSACTION_NOTIFY_ROLLBACK: u32 = 8u32;
pub const TRANSACTION_NOTIFY_ROLLBACK_COMPLETE: u32 = 128u32;
pub const TRANSACTION_NOTIFY_SINGLE_PHASE_COMMIT: u32 = 512u32;
pub const TRANSACTION_NOTIFY_TM_ONLINE: u32 = 33554432u32;
pub struct TRANSACTION_OUTCOME(i32);
pub struct TXFS_MINIVERSION(i32);
pub struct TXF_ID(i32);
pub struct TXF_LOG_RECORD_AFFECTED_FILE(i32);
pub struct TXF_LOG_RECORD_BASE(i32);
pub const TXF_LOG_RECORD_GENERIC_TYPE_ABORT: u32 = 2u32;
pub const TXF_LOG_RECORD_GENERIC_TYPE_COMMIT: u32 = 1u32;
pub const TXF_LOG_RECORD_GENERIC_TYPE_DATA: u32 = 8u32;
pub const TXF_LOG_RECORD_GENERIC_TYPE_PREPARE: u32 = 4u32;
pub struct TXF_LOG_RECORD_TRUNCATE(i32);
pub struct TXF_LOG_RECORD_TYPE(i32);
pub struct TXF_LOG_RECORD_WRITE(i32);
pub struct VER_FIND_FILE_FLAGS(i32);
pub struct VER_FIND_FILE_STATUS(i32);
pub struct VER_INSTALL_FILE_FLAGS(i32);
pub struct VER_INSTALL_FILE_STATUS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct VOLUME_ALLOCATE_BC_STREAM_INPUT(i32);
pub struct VOLUME_ALLOCATE_BC_STREAM_OUTPUT(i32);
pub struct VOLUME_ALLOCATION_HINT_INPUT(i32);
pub struct VOLUME_ALLOCATION_HINT_OUTPUT(i32);
pub struct VOLUME_CRITICAL_IO(i32);
pub struct VOLUME_FAILOVER_SET(i32);
pub struct VOLUME_GET_BC_PROPERTIES_INPUT(i32);
pub struct VOLUME_GET_BC_PROPERTIES_OUTPUT(i32);
pub struct VOLUME_LOGICAL_OFFSET(i32);
pub struct VOLUME_NUMBER(i32);
pub struct VOLUME_PHYSICAL_OFFSET(i32);
pub struct VOLUME_PHYSICAL_OFFSETS(i32);
pub struct VOLUME_READ_PLEX_INPUT(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct VOLUME_SET_GPT_ATTRIBUTES_INFORMATION(i32);
pub struct VOLUME_SHRINK_INFO(i32);
pub const VS_FFI_FILEFLAGSMASK: i32 = 63i32;
pub const VS_FFI_SIGNATURE: i32 = -17890115i32;
pub const VS_FFI_STRUCVERSION: i32 = 65536i32;
pub struct VS_FIXEDFILEINFO(i32);
pub struct VS_FIXEDFILEINFO_FILE_FLAGS(i32);
pub struct VS_FIXEDFILEINFO_FILE_OS(i32);
pub struct VS_FIXEDFILEINFO_FILE_SUBTYPE(i32);
pub struct VS_FIXEDFILEINFO_FILE_TYPE(i32);
pub const VS_USER_DEFINED: u32 = 100u32;
pub const VS_VERSION_INFO: u32 = 1u32;
pub const WIM_BOOT_NOT_OS_WIM: u32 = 0u32;
pub const WIM_BOOT_OS_WIM: u32 = 1u32;
pub const WIM_ENTRY_FLAG_NOT_ACTIVE: u32 = 1u32;
pub const WIM_ENTRY_FLAG_SUSPENDED: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
pub struct WIM_ENTRY_INFO(i32);
pub struct WIM_EXTERNAL_FILE_INFO(i32);
pub const WIM_EXTERNAL_FILE_INFO_FLAG_NOT_ACTIVE: u32 = 1u32;
pub const WIM_EXTERNAL_FILE_INFO_FLAG_SUSPENDED: u32 = 2u32;
pub const WIM_PROVIDER_HASH_SIZE: u32 = 20u32;
#[cfg(feature = "Win32_Foundation")]
pub struct WIN32_FILE_ATTRIBUTE_DATA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct WIN32_FIND_DATAA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct WIN32_FIND_DATAW(i32);
pub struct WIN32_FIND_STREAM_DATA(i32);
pub struct WIN32_STREAM_ID(i32);
pub const WINEFS_SETUSERKEY_SET_CAPABILITIES: u32 = 1u32;
pub struct WIN_STREAM_ID(i32);
pub struct WOF_FILE_COMPRESSION_INFO_V0(i32);
pub struct WOF_FILE_COMPRESSION_INFO_V1(i32);
pub const WOF_PROVIDER_FILE: u32 = 2u32;
pub const WOF_PROVIDER_WIM: u32 = 1u32;
pub struct WofEnumEntryProc(i32);
pub struct WofEnumFilesProc(i32);
pub const _FT_TYPES_DEFINITION_: u32 = 1u32;
