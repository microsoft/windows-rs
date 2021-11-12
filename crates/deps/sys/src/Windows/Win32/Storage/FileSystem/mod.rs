#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
#[repr(C)]
pub struct BY_HANDLE_FILE_INFORMATION(i32);
#[repr(C)]
pub struct CACHE_ACCESS_CHECK(i32);
#[repr(C)]
pub struct CACHE_DESTROY_CALLBACK(i32);
#[repr(C)]
pub struct CACHE_KEY_COMPARE(i32);
#[repr(C)]
pub struct CACHE_KEY_HASH(i32);
#[repr(C)]
pub struct CACHE_READ_CALLBACK(i32);
#[repr(C)]
pub struct CLAIMMEDIALABEL(i32);
#[repr(C)]
pub struct CLAIMMEDIALABELEX(i32);
#[repr(C)]
pub struct CLFS_BLOCK_ALLOCATION(i32);
#[repr(C)]
pub struct CLFS_BLOCK_DEALLOCATION(i32);
#[repr(transparent)]
pub struct CLFS_CONTEXT_MODE(pub i32);
pub const ClfsContextNone: CLFS_CONTEXT_MODE = CLFS_CONTEXT_MODE(0i32);
pub const ClfsContextUndoNext: CLFS_CONTEXT_MODE = CLFS_CONTEXT_MODE(1i32);
pub const ClfsContextPrevious: CLFS_CONTEXT_MODE = CLFS_CONTEXT_MODE(2i32);
pub const ClfsContextForward: CLFS_CONTEXT_MODE = CLFS_CONTEXT_MODE(3i32);
#[repr(transparent)]
pub struct CLFS_FLAG(pub u32);
pub const CLFS_FLAG_FORCE_APPEND: CLFS_FLAG = CLFS_FLAG(1u32);
pub const CLFS_FLAG_FORCE_FLUSH: CLFS_FLAG = CLFS_FLAG(2u32);
pub const CLFS_FLAG_NO_FLAGS: CLFS_FLAG = CLFS_FLAG(0u32);
pub const CLFS_FLAG_USE_RESERVATION: CLFS_FLAG = CLFS_FLAG(4u32);
pub const CLFS_FLAG_FILTER_INTERMEDIATE_LEVEL: u32 = 16u32;
pub const CLFS_FLAG_FILTER_TOP_LEVEL: u32 = 32u32;
pub const CLFS_FLAG_HIDDEN_SYSTEM_LOG: u32 = 512u32;
pub const CLFS_FLAG_IGNORE_SHARE_ACCESS: u32 = 64u32;
pub const CLFS_FLAG_MINIFILTER_LEVEL: u32 = 256u32;
pub const CLFS_FLAG_NON_REENTRANT_FILTER: u32 = 16u32;
pub const CLFS_FLAG_READ_IN_PROGRESS: u32 = 128u32;
pub const CLFS_FLAG_REENTRANT_FILE_SYSTEM: u32 = 8u32;
pub const CLFS_FLAG_REENTRANT_FILTER: u32 = 32u32;
#[repr(transparent)]
pub struct CLFS_IOSTATS_CLASS(pub i32);
pub const ClfsIoStatsDefault: CLFS_IOSTATS_CLASS = CLFS_IOSTATS_CLASS(0i32);
pub const ClfsIoStatsMax: CLFS_IOSTATS_CLASS = CLFS_IOSTATS_CLASS(65535i32);
#[repr(transparent)]
pub struct CLFS_LOG_ARCHIVE_MODE(pub i32);
pub const ClfsLogArchiveEnabled: CLFS_LOG_ARCHIVE_MODE = CLFS_LOG_ARCHIVE_MODE(1i32);
pub const ClfsLogArchiveDisabled: CLFS_LOG_ARCHIVE_MODE = CLFS_LOG_ARCHIVE_MODE(2i32);
#[repr(C)]
pub struct CLFS_LOG_NAME_INFORMATION(i32);
pub const CLFS_MARSHALLING_FLAG_DISABLE_BUFF_INIT: u32 = 1u32;
pub const CLFS_MARSHALLING_FLAG_NONE: u32 = 0u32;
pub const CLFS_MAX_CONTAINER_INFO: u32 = 256u32;
pub const CLFS_MGMT_CLIENT_REGISTRATION_VERSION: u32 = 1u32;
#[repr(C)]
pub struct CLFS_MGMT_NOTIFICATION(i32);
#[repr(transparent)]
pub struct CLFS_MGMT_NOTIFICATION_TYPE(pub i32);
pub const ClfsMgmtAdvanceTailNotification: CLFS_MGMT_NOTIFICATION_TYPE = CLFS_MGMT_NOTIFICATION_TYPE(0i32);
pub const ClfsMgmtLogFullHandlerNotification: CLFS_MGMT_NOTIFICATION_TYPE = CLFS_MGMT_NOTIFICATION_TYPE(1i32);
pub const ClfsMgmtLogUnpinnedNotification: CLFS_MGMT_NOTIFICATION_TYPE = CLFS_MGMT_NOTIFICATION_TYPE(2i32);
pub const ClfsMgmtLogWriteNotification: CLFS_MGMT_NOTIFICATION_TYPE = CLFS_MGMT_NOTIFICATION_TYPE(3i32);
#[repr(C)]
pub struct CLFS_MGMT_POLICY(i32);
#[repr(transparent)]
pub struct CLFS_MGMT_POLICY_TYPE(pub i32);
pub const ClfsMgmtPolicyMaximumSize: CLFS_MGMT_POLICY_TYPE = CLFS_MGMT_POLICY_TYPE(0i32);
pub const ClfsMgmtPolicyMinimumSize: CLFS_MGMT_POLICY_TYPE = CLFS_MGMT_POLICY_TYPE(1i32);
pub const ClfsMgmtPolicyNewContainerSize: CLFS_MGMT_POLICY_TYPE = CLFS_MGMT_POLICY_TYPE(2i32);
pub const ClfsMgmtPolicyGrowthRate: CLFS_MGMT_POLICY_TYPE = CLFS_MGMT_POLICY_TYPE(3i32);
pub const ClfsMgmtPolicyLogTail: CLFS_MGMT_POLICY_TYPE = CLFS_MGMT_POLICY_TYPE(4i32);
pub const ClfsMgmtPolicyAutoShrink: CLFS_MGMT_POLICY_TYPE = CLFS_MGMT_POLICY_TYPE(5i32);
pub const ClfsMgmtPolicyAutoGrow: CLFS_MGMT_POLICY_TYPE = CLFS_MGMT_POLICY_TYPE(6i32);
pub const ClfsMgmtPolicyNewContainerPrefix: CLFS_MGMT_POLICY_TYPE = CLFS_MGMT_POLICY_TYPE(7i32);
pub const ClfsMgmtPolicyNewContainerSuffix: CLFS_MGMT_POLICY_TYPE = CLFS_MGMT_POLICY_TYPE(8i32);
pub const ClfsMgmtPolicyNewContainerExtension: CLFS_MGMT_POLICY_TYPE = CLFS_MGMT_POLICY_TYPE(9i32);
pub const ClfsMgmtPolicyInvalid: CLFS_MGMT_POLICY_TYPE = CLFS_MGMT_POLICY_TYPE(10i32);
pub const CLFS_MGMT_POLICY_VERSION: u32 = 1u32;
#[repr(C)]
pub struct CLFS_NODE_ID(i32);
#[repr(C)]
pub struct CLFS_PHYSICAL_LSN_INFORMATION(i32);
pub const CLFS_SCAN_BACKWARD: u8 = 4u8;
pub const CLFS_SCAN_BUFFERED: u8 = 32u8;
pub const CLFS_SCAN_CLOSE: u8 = 8u8;
pub const CLFS_SCAN_FORWARD: u8 = 2u8;
pub const CLFS_SCAN_INIT: u8 = 1u8;
pub const CLFS_SCAN_INITIALIZED: u8 = 16u8;
#[repr(C)]
pub struct CLFS_STREAM_ID_INFORMATION(i32);
pub const CLSID_DiskQuotaControl: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2039002481, data2: 60553, data3: 4559, data4: [156, 0, 0, 170, 0, 161, 79, 86] };
#[repr(C)]
pub struct CLS_ARCHIVE_DESCRIPTOR(i32);
#[repr(C)]
pub struct CLS_CONTAINER_INFORMATION(i32);
#[repr(transparent)]
pub struct CLS_CONTEXT_MODE(pub i32);
pub const ClsContextNone: CLS_CONTEXT_MODE = CLS_CONTEXT_MODE(0i32);
pub const ClsContextUndoNext: CLS_CONTEXT_MODE = CLS_CONTEXT_MODE(1i32);
pub const ClsContextPrevious: CLS_CONTEXT_MODE = CLS_CONTEXT_MODE(2i32);
pub const ClsContextForward: CLS_CONTEXT_MODE = CLS_CONTEXT_MODE(3i32);
#[repr(C)]
pub struct CLS_INFORMATION(i32);
#[repr(transparent)]
pub struct CLS_IOSTATS_CLASS(pub i32);
pub const ClsIoStatsDefault: CLS_IOSTATS_CLASS = CLS_IOSTATS_CLASS(0i32);
pub const ClsIoStatsMax: CLS_IOSTATS_CLASS = CLS_IOSTATS_CLASS(65535i32);
#[repr(C)]
pub struct CLS_IO_STATISTICS(i32);
#[repr(C)]
pub struct CLS_IO_STATISTICS_HEADER(i32);
#[repr(transparent)]
pub struct CLS_LOG_INFORMATION_CLASS(pub i32);
pub const ClfsLogBasicInformation: CLS_LOG_INFORMATION_CLASS = CLS_LOG_INFORMATION_CLASS(0i32);
pub const ClfsLogBasicInformationPhysical: CLS_LOG_INFORMATION_CLASS = CLS_LOG_INFORMATION_CLASS(1i32);
pub const ClfsLogPhysicalNameInformation: CLS_LOG_INFORMATION_CLASS = CLS_LOG_INFORMATION_CLASS(2i32);
pub const ClfsLogStreamIdentifierInformation: CLS_LOG_INFORMATION_CLASS = CLS_LOG_INFORMATION_CLASS(3i32);
pub const ClfsLogSystemMarkingInformation: CLS_LOG_INFORMATION_CLASS = CLS_LOG_INFORMATION_CLASS(4i32);
pub const ClfsLogPhysicalLsnInformation: CLS_LOG_INFORMATION_CLASS = CLS_LOG_INFORMATION_CLASS(5i32);
#[repr(C)]
pub struct CLS_LSN(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct CLS_SCAN_CONTEXT(i32);
#[repr(C)]
pub struct CLS_WRITE_ENTRY(i32);
#[repr(C)]
pub struct CONNECTION_INFO_0(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct CONNECTION_INFO_1(i32);
#[repr(transparent)]
pub struct COPYFILE2_COPY_PHASE(pub i32);
pub const COPYFILE2_PHASE_NONE: COPYFILE2_COPY_PHASE = COPYFILE2_COPY_PHASE(0i32);
pub const COPYFILE2_PHASE_PREPARE_SOURCE: COPYFILE2_COPY_PHASE = COPYFILE2_COPY_PHASE(1i32);
pub const COPYFILE2_PHASE_PREPARE_DEST: COPYFILE2_COPY_PHASE = COPYFILE2_COPY_PHASE(2i32);
pub const COPYFILE2_PHASE_READ_SOURCE: COPYFILE2_COPY_PHASE = COPYFILE2_COPY_PHASE(3i32);
pub const COPYFILE2_PHASE_WRITE_DESTINATION: COPYFILE2_COPY_PHASE = COPYFILE2_COPY_PHASE(4i32);
pub const COPYFILE2_PHASE_SERVER_COPY: COPYFILE2_COPY_PHASE = COPYFILE2_COPY_PHASE(5i32);
pub const COPYFILE2_PHASE_NAMEGRAFT_COPY: COPYFILE2_COPY_PHASE = COPYFILE2_COPY_PHASE(6i32);
pub const COPYFILE2_PHASE_MAX: COPYFILE2_COPY_PHASE = COPYFILE2_COPY_PHASE(7i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct COPYFILE2_EXTENDED_PARAMETERS(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct COPYFILE2_EXTENDED_PARAMETERS_V2(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct COPYFILE2_MESSAGE(i32);
#[repr(transparent)]
pub struct COPYFILE2_MESSAGE_ACTION(pub i32);
pub const COPYFILE2_PROGRESS_CONTINUE: COPYFILE2_MESSAGE_ACTION = COPYFILE2_MESSAGE_ACTION(0i32);
pub const COPYFILE2_PROGRESS_CANCEL: COPYFILE2_MESSAGE_ACTION = COPYFILE2_MESSAGE_ACTION(1i32);
pub const COPYFILE2_PROGRESS_STOP: COPYFILE2_MESSAGE_ACTION = COPYFILE2_MESSAGE_ACTION(2i32);
pub const COPYFILE2_PROGRESS_QUIET: COPYFILE2_MESSAGE_ACTION = COPYFILE2_MESSAGE_ACTION(3i32);
pub const COPYFILE2_PROGRESS_PAUSE: COPYFILE2_MESSAGE_ACTION = COPYFILE2_MESSAGE_ACTION(4i32);
#[repr(transparent)]
pub struct COPYFILE2_MESSAGE_TYPE(pub i32);
pub const COPYFILE2_CALLBACK_NONE: COPYFILE2_MESSAGE_TYPE = COPYFILE2_MESSAGE_TYPE(0i32);
pub const COPYFILE2_CALLBACK_CHUNK_STARTED: COPYFILE2_MESSAGE_TYPE = COPYFILE2_MESSAGE_TYPE(1i32);
pub const COPYFILE2_CALLBACK_CHUNK_FINISHED: COPYFILE2_MESSAGE_TYPE = COPYFILE2_MESSAGE_TYPE(2i32);
pub const COPYFILE2_CALLBACK_STREAM_STARTED: COPYFILE2_MESSAGE_TYPE = COPYFILE2_MESSAGE_TYPE(3i32);
pub const COPYFILE2_CALLBACK_STREAM_FINISHED: COPYFILE2_MESSAGE_TYPE = COPYFILE2_MESSAGE_TYPE(4i32);
pub const COPYFILE2_CALLBACK_POLL_CONTINUE: COPYFILE2_MESSAGE_TYPE = COPYFILE2_MESSAGE_TYPE(5i32);
pub const COPYFILE2_CALLBACK_ERROR: COPYFILE2_MESSAGE_TYPE = COPYFILE2_MESSAGE_TYPE(6i32);
pub const COPYFILE2_CALLBACK_MAX: COPYFILE2_MESSAGE_TYPE = COPYFILE2_MESSAGE_TYPE(7i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[repr(C)]
pub struct CREATEFILE2_EXTENDED_PARAMETERS(i32);
#[repr(transparent)]
pub struct CREATE_TAPE_PARTITION_METHOD(pub i32);
pub const TAPE_FIXED_PARTITIONS: CREATE_TAPE_PARTITION_METHOD = CREATE_TAPE_PARTITION_METHOD(0i32);
pub const TAPE_INITIATOR_PARTITIONS: CREATE_TAPE_PARTITION_METHOD = CREATE_TAPE_PARTITION_METHOD(2i32);
pub const TAPE_SELECT_PARTITIONS: CREATE_TAPE_PARTITION_METHOD = CREATE_TAPE_PARTITION_METHOD(1i32);
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
#[repr(transparent)]
pub struct DEFINE_DOS_DEVICE_FLAGS(pub u32);
pub const DDD_RAW_TARGET_PATH: DEFINE_DOS_DEVICE_FLAGS = DEFINE_DOS_DEVICE_FLAGS(1u32);
pub const DDD_REMOVE_DEFINITION: DEFINE_DOS_DEVICE_FLAGS = DEFINE_DOS_DEVICE_FLAGS(2u32);
pub const DDD_EXACT_MATCH_ON_REMOVE: DEFINE_DOS_DEVICE_FLAGS = DEFINE_DOS_DEVICE_FLAGS(4u32);
pub const DDD_NO_BROADCAST_SYSTEM: DEFINE_DOS_DEVICE_FLAGS = DEFINE_DOS_DEVICE_FLAGS(8u32);
pub const DDD_LUID_BROADCAST_DRIVE: DEFINE_DOS_DEVICE_FLAGS = DEFINE_DOS_DEVICE_FLAGS(16u32);
pub const DISKQUOTA_FILESTATE_INCOMPLETE: u32 = 256u32;
pub const DISKQUOTA_FILESTATE_MASK: u32 = 768u32;
pub const DISKQUOTA_FILESTATE_REBUILDING: u32 = 512u32;
pub const DISKQUOTA_LOGFLAG_USER_LIMIT: u32 = 2u32;
pub const DISKQUOTA_LOGFLAG_USER_THRESHOLD: u32 = 1u32;
pub const DISKQUOTA_STATE_DISABLED: u32 = 0u32;
pub const DISKQUOTA_STATE_ENFORCE: u32 = 2u32;
pub const DISKQUOTA_STATE_MASK: u32 = 3u32;
pub const DISKQUOTA_STATE_TRACK: u32 = 1u32;
#[repr(transparent)]
pub struct DISKQUOTA_USERNAME_RESOLVE(pub u32);
pub const DISKQUOTA_USERNAME_RESOLVE_ASYNC: DISKQUOTA_USERNAME_RESOLVE = DISKQUOTA_USERNAME_RESOLVE(2u32);
pub const DISKQUOTA_USERNAME_RESOLVE_NONE: DISKQUOTA_USERNAME_RESOLVE = DISKQUOTA_USERNAME_RESOLVE(0u32);
pub const DISKQUOTA_USERNAME_RESOLVE_SYNC: DISKQUOTA_USERNAME_RESOLVE = DISKQUOTA_USERNAME_RESOLVE(1u32);
pub const DISKQUOTA_USER_ACCOUNT_DELETED: u32 = 2u32;
pub const DISKQUOTA_USER_ACCOUNT_INVALID: u32 = 3u32;
pub const DISKQUOTA_USER_ACCOUNT_RESOLVED: u32 = 0u32;
pub const DISKQUOTA_USER_ACCOUNT_UNAVAILABLE: u32 = 1u32;
pub const DISKQUOTA_USER_ACCOUNT_UNKNOWN: u32 = 4u32;
pub const DISKQUOTA_USER_ACCOUNT_UNRESOLVED: u32 = 5u32;
#[repr(C)]
pub struct DISKQUOTA_USER_INFORMATION(i32);
#[repr(C)]
pub struct DISK_SPACE_INFORMATION(i32);
#[repr(C)]
pub struct EFS_CERTIFICATE_BLOB(i32);
#[repr(C)]
pub struct EFS_COMPATIBILITY_INFO(i32);
pub const EFS_COMPATIBILITY_VERSION_NCRYPT_PROTECTOR: u32 = 5u32;
pub const EFS_COMPATIBILITY_VERSION_PFILE_PROTECTOR: u32 = 6u32;
#[repr(C)]
pub struct EFS_DECRYPTION_STATUS_INFO(i32);
pub const EFS_EFS_SUBVER_EFS_CERT: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct EFS_ENCRYPTION_STATUS_INFO(i32);
#[repr(C)]
pub struct EFS_HASH_BLOB(i32);
#[repr(C)]
pub struct EFS_KEY_INFO(i32);
pub const EFS_METADATA_ADD_USER: u32 = 1u32;
pub const EFS_METADATA_GENERAL_OP: u32 = 8u32;
pub const EFS_METADATA_REMOVE_USER: u32 = 2u32;
pub const EFS_METADATA_REPLACE_USER: u32 = 4u32;
pub const EFS_PFILE_SUBVER_APPX: u32 = 3u32;
pub const EFS_PFILE_SUBVER_RMS: u32 = 2u32;
#[repr(C)]
pub struct EFS_PIN_BLOB(i32);
#[repr(C)]
pub struct EFS_RPC_BLOB(i32);
pub const EFS_SUBVER_UNKNOWN: u32 = 0u32;
#[repr(C)]
pub struct EFS_VERSION_INFO(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[repr(C)]
pub struct ENCRYPTED_FILE_METADATA_SIGNATURE(i32);
#[cfg(feature = "Win32_Security")]
#[repr(C)]
pub struct ENCRYPTION_CERTIFICATE(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[repr(C)]
pub struct ENCRYPTION_CERTIFICATE_HASH(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[repr(C)]
pub struct ENCRYPTION_CERTIFICATE_HASH_LIST(i32);
#[cfg(feature = "Win32_Security")]
#[repr(C)]
pub struct ENCRYPTION_CERTIFICATE_LIST(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[repr(C)]
pub struct ENCRYPTION_PROTECTOR(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[repr(C)]
pub struct ENCRYPTION_PROTECTOR_LIST(i32);
pub const ENLISTMENT_MAXIMUM_OPTION: u32 = 1u32;
pub const ENLISTMENT_SUPERIOR: u32 = 1u32;
#[repr(transparent)]
pub struct ERASE_TAPE_TYPE(pub i32);
pub const TAPE_ERASE_LONG: ERASE_TAPE_TYPE = ERASE_TAPE_TYPE(1i32);
pub const TAPE_ERASE_SHORT: ERASE_TAPE_TYPE = ERASE_TAPE_TYPE(0i32);
#[repr(C)]
pub struct FCACHE_CREATE_CALLBACK(i32);
#[repr(C)]
pub struct FCACHE_RICHCREATE_CALLBACK(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct FH_OVERLAPPED(i32);
#[repr(transparent)]
pub struct FILE_ACCESS_FLAGS(pub u32);
pub const FILE_READ_DATA: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(1u32);
pub const FILE_LIST_DIRECTORY: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(1u32);
pub const FILE_WRITE_DATA: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(2u32);
pub const FILE_ADD_FILE: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(2u32);
pub const FILE_APPEND_DATA: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(4u32);
pub const FILE_ADD_SUBDIRECTORY: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(4u32);
pub const FILE_CREATE_PIPE_INSTANCE: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(4u32);
pub const FILE_READ_EA: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(8u32);
pub const FILE_WRITE_EA: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(16u32);
pub const FILE_EXECUTE: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(32u32);
pub const FILE_TRAVERSE: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(32u32);
pub const FILE_DELETE_CHILD: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(64u32);
pub const FILE_READ_ATTRIBUTES: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(128u32);
pub const FILE_WRITE_ATTRIBUTES: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(256u32);
pub const READ_CONTROL: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(131072u32);
pub const SYNCHRONIZE: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(1048576u32);
pub const STANDARD_RIGHTS_REQUIRED: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(983040u32);
pub const STANDARD_RIGHTS_READ: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(131072u32);
pub const STANDARD_RIGHTS_WRITE: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(131072u32);
pub const STANDARD_RIGHTS_EXECUTE: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(131072u32);
pub const STANDARD_RIGHTS_ALL: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(2031616u32);
pub const SPECIFIC_RIGHTS_ALL: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(65535u32);
pub const FILE_ALL_ACCESS: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(2032127u32);
pub const FILE_GENERIC_READ: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(1179785u32);
pub const FILE_GENERIC_WRITE: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(1179926u32);
pub const FILE_GENERIC_EXECUTE: FILE_ACCESS_FLAGS = FILE_ACCESS_FLAGS(1179808u32);
#[repr(transparent)]
pub struct FILE_ACTION(pub u32);
pub const FILE_ACTION_ADDED: FILE_ACTION = FILE_ACTION(1u32);
pub const FILE_ACTION_REMOVED: FILE_ACTION = FILE_ACTION(2u32);
pub const FILE_ACTION_MODIFIED: FILE_ACTION = FILE_ACTION(3u32);
pub const FILE_ACTION_RENAMED_OLD_NAME: FILE_ACTION = FILE_ACTION(4u32);
pub const FILE_ACTION_RENAMED_NEW_NAME: FILE_ACTION = FILE_ACTION(5u32);
#[repr(C)]
pub struct FILE_ALIGNMENT_INFO(i32);
#[repr(C)]
pub struct FILE_ALLOCATION_INFO(i32);
#[repr(C)]
pub struct FILE_ATTRIBUTE_TAG_INFO(i32);
#[repr(C)]
pub struct FILE_BASIC_INFO(i32);
#[repr(C)]
pub struct FILE_COMPRESSION_INFO(i32);
#[repr(transparent)]
pub struct FILE_CREATION_DISPOSITION(pub u32);
pub const CREATE_NEW: FILE_CREATION_DISPOSITION = FILE_CREATION_DISPOSITION(1u32);
pub const CREATE_ALWAYS: FILE_CREATION_DISPOSITION = FILE_CREATION_DISPOSITION(2u32);
pub const OPEN_EXISTING: FILE_CREATION_DISPOSITION = FILE_CREATION_DISPOSITION(3u32);
pub const OPEN_ALWAYS: FILE_CREATION_DISPOSITION = FILE_CREATION_DISPOSITION(4u32);
pub const TRUNCATE_EXISTING: FILE_CREATION_DISPOSITION = FILE_CREATION_DISPOSITION(5u32);
#[repr(transparent)]
pub struct FILE_DEVICE_TYPE(pub u32);
pub const FILE_DEVICE_CD_ROM: FILE_DEVICE_TYPE = FILE_DEVICE_TYPE(2u32);
pub const FILE_DEVICE_DISK: FILE_DEVICE_TYPE = FILE_DEVICE_TYPE(7u32);
pub const FILE_DEVICE_TAPE: FILE_DEVICE_TYPE = FILE_DEVICE_TYPE(31u32);
pub const FILE_DEVICE_DVD: FILE_DEVICE_TYPE = FILE_DEVICE_TYPE(51u32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct FILE_DISPOSITION_INFO(i32);
#[repr(C)]
pub struct FILE_END_OF_FILE_INFO(i32);
#[repr(C)]
pub struct FILE_EXTENT(i32);
#[repr(transparent)]
pub struct FILE_FLAGS_AND_ATTRIBUTES(pub u32);
pub const FILE_ATTRIBUTE_READONLY: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(1u32);
pub const FILE_ATTRIBUTE_HIDDEN: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(2u32);
pub const FILE_ATTRIBUTE_SYSTEM: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(4u32);
pub const FILE_ATTRIBUTE_DIRECTORY: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(16u32);
pub const FILE_ATTRIBUTE_ARCHIVE: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(32u32);
pub const FILE_ATTRIBUTE_DEVICE: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(64u32);
pub const FILE_ATTRIBUTE_NORMAL: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(128u32);
pub const FILE_ATTRIBUTE_TEMPORARY: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(256u32);
pub const FILE_ATTRIBUTE_SPARSE_FILE: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(512u32);
pub const FILE_ATTRIBUTE_REPARSE_POINT: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(1024u32);
pub const FILE_ATTRIBUTE_COMPRESSED: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(2048u32);
pub const FILE_ATTRIBUTE_OFFLINE: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(4096u32);
pub const FILE_ATTRIBUTE_NOT_CONTENT_INDEXED: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(8192u32);
pub const FILE_ATTRIBUTE_ENCRYPTED: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(16384u32);
pub const FILE_ATTRIBUTE_INTEGRITY_STREAM: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(32768u32);
pub const FILE_ATTRIBUTE_VIRTUAL: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(65536u32);
pub const FILE_ATTRIBUTE_NO_SCRUB_DATA: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(131072u32);
pub const FILE_ATTRIBUTE_EA: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(262144u32);
pub const FILE_ATTRIBUTE_PINNED: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(524288u32);
pub const FILE_ATTRIBUTE_UNPINNED: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(1048576u32);
pub const FILE_ATTRIBUTE_RECALL_ON_OPEN: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(262144u32);
pub const FILE_ATTRIBUTE_RECALL_ON_DATA_ACCESS: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(4194304u32);
pub const FILE_FLAG_WRITE_THROUGH: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(2147483648u32);
pub const FILE_FLAG_OVERLAPPED: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(1073741824u32);
pub const FILE_FLAG_NO_BUFFERING: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(536870912u32);
pub const FILE_FLAG_RANDOM_ACCESS: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(268435456u32);
pub const FILE_FLAG_SEQUENTIAL_SCAN: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(134217728u32);
pub const FILE_FLAG_DELETE_ON_CLOSE: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(67108864u32);
pub const FILE_FLAG_BACKUP_SEMANTICS: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(33554432u32);
pub const FILE_FLAG_POSIX_SEMANTICS: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(16777216u32);
pub const FILE_FLAG_SESSION_AWARE: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(8388608u32);
pub const FILE_FLAG_OPEN_REPARSE_POINT: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(2097152u32);
pub const FILE_FLAG_OPEN_NO_RECALL: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(1048576u32);
pub const FILE_FLAG_FIRST_PIPE_INSTANCE: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(524288u32);
pub const PIPE_ACCESS_DUPLEX: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(3u32);
pub const PIPE_ACCESS_INBOUND: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(1u32);
pub const PIPE_ACCESS_OUTBOUND: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(2u32);
pub const SECURITY_ANONYMOUS: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(0u32);
pub const SECURITY_IDENTIFICATION: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(65536u32);
pub const SECURITY_IMPERSONATION: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(131072u32);
pub const SECURITY_DELEGATION: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(196608u32);
pub const SECURITY_CONTEXT_TRACKING: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(262144u32);
pub const SECURITY_EFFECTIVE_ONLY: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(524288u32);
pub const SECURITY_SQOS_PRESENT: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(1048576u32);
pub const SECURITY_VALID_SQOS_FLAGS: FILE_FLAGS_AND_ATTRIBUTES = FILE_FLAGS_AND_ATTRIBUTES(2031616u32);
#[repr(C)]
pub struct FILE_FULL_DIR_INFO(i32);
#[repr(C)]
pub struct FILE_ID_128(i32);
#[repr(C)]
pub struct FILE_ID_BOTH_DIR_INFO(i32);
#[repr(C)]
pub struct FILE_ID_DESCRIPTOR(i32);
#[repr(C)]
pub struct FILE_ID_EXTD_DIR_INFO(i32);
#[repr(C)]
pub struct FILE_ID_INFO(i32);
#[repr(transparent)]
pub struct FILE_ID_TYPE(pub i32);
pub const FileIdType: FILE_ID_TYPE = FILE_ID_TYPE(0i32);
pub const ObjectIdType: FILE_ID_TYPE = FILE_ID_TYPE(1i32);
pub const ExtendedFileIdType: FILE_ID_TYPE = FILE_ID_TYPE(2i32);
pub const MaximumFileIdType: FILE_ID_TYPE = FILE_ID_TYPE(3i32);
#[repr(C)]
pub struct FILE_INFO_2(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct FILE_INFO_3(i32);
#[repr(transparent)]
pub struct FILE_INFO_BY_HANDLE_CLASS(pub i32);
pub const FileBasicInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(0i32);
pub const FileStandardInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(1i32);
pub const FileNameInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(2i32);
pub const FileRenameInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(3i32);
pub const FileDispositionInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(4i32);
pub const FileAllocationInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(5i32);
pub const FileEndOfFileInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(6i32);
pub const FileStreamInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(7i32);
pub const FileCompressionInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(8i32);
pub const FileAttributeTagInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(9i32);
pub const FileIdBothDirectoryInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(10i32);
pub const FileIdBothDirectoryRestartInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(11i32);
pub const FileIoPriorityHintInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(12i32);
pub const FileRemoteProtocolInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(13i32);
pub const FileFullDirectoryInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(14i32);
pub const FileFullDirectoryRestartInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(15i32);
pub const FileStorageInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(16i32);
pub const FileAlignmentInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(17i32);
pub const FileIdInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(18i32);
pub const FileIdExtdDirectoryInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(19i32);
pub const FileIdExtdDirectoryRestartInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(20i32);
pub const FileDispositionInfoEx: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(21i32);
pub const FileRenameInfoEx: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(22i32);
pub const FileCaseSensitiveInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(23i32);
pub const FileNormalizedNameInfo: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(24i32);
pub const MaximumFileInfoByHandleClass: FILE_INFO_BY_HANDLE_CLASS = FILE_INFO_BY_HANDLE_CLASS(25i32);
#[repr(transparent)]
pub struct FILE_INFO_FLAGS_PERMISSIONS(pub u32);
pub const PERM_FILE_READ: FILE_INFO_FLAGS_PERMISSIONS = FILE_INFO_FLAGS_PERMISSIONS(1u32);
pub const PERM_FILE_WRITE: FILE_INFO_FLAGS_PERMISSIONS = FILE_INFO_FLAGS_PERMISSIONS(2u32);
pub const PERM_FILE_CREATE: FILE_INFO_FLAGS_PERMISSIONS = FILE_INFO_FLAGS_PERMISSIONS(4u32);
#[repr(C)]
pub struct FILE_IO_PRIORITY_HINT_INFO(i32);
#[repr(transparent)]
pub struct FILE_NAME(pub u32);
pub const FILE_NAME_NORMALIZED: FILE_NAME = FILE_NAME(0u32);
pub const FILE_NAME_OPENED: FILE_NAME = FILE_NAME(8u32);
#[repr(C)]
pub struct FILE_NAME_INFO(i32);
#[repr(transparent)]
pub struct FILE_NOTIFY_CHANGE(pub u32);
pub const FILE_NOTIFY_CHANGE_FILE_NAME: FILE_NOTIFY_CHANGE = FILE_NOTIFY_CHANGE(1u32);
pub const FILE_NOTIFY_CHANGE_DIR_NAME: FILE_NOTIFY_CHANGE = FILE_NOTIFY_CHANGE(2u32);
pub const FILE_NOTIFY_CHANGE_ATTRIBUTES: FILE_NOTIFY_CHANGE = FILE_NOTIFY_CHANGE(4u32);
pub const FILE_NOTIFY_CHANGE_SIZE: FILE_NOTIFY_CHANGE = FILE_NOTIFY_CHANGE(8u32);
pub const FILE_NOTIFY_CHANGE_LAST_WRITE: FILE_NOTIFY_CHANGE = FILE_NOTIFY_CHANGE(16u32);
pub const FILE_NOTIFY_CHANGE_LAST_ACCESS: FILE_NOTIFY_CHANGE = FILE_NOTIFY_CHANGE(32u32);
pub const FILE_NOTIFY_CHANGE_CREATION: FILE_NOTIFY_CHANGE = FILE_NOTIFY_CHANGE(64u32);
pub const FILE_NOTIFY_CHANGE_SECURITY: FILE_NOTIFY_CHANGE = FILE_NOTIFY_CHANGE(256u32);
#[repr(C)]
pub struct FILE_NOTIFY_EXTENDED_INFORMATION(i32);
#[repr(C)]
pub struct FILE_NOTIFY_INFORMATION(i32);
pub const FILE_PROVIDER_COMPRESSION_LZX: u32 = 1u32;
pub const FILE_PROVIDER_COMPRESSION_XPRESS16K: u32 = 3u32;
pub const FILE_PROVIDER_COMPRESSION_XPRESS4K: u32 = 0u32;
pub const FILE_PROVIDER_COMPRESSION_XPRESS8K: u32 = 2u32;
#[repr(C)]
pub struct FILE_REMOTE_PROTOCOL_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct FILE_RENAME_INFO(i32);
#[repr(C)]
pub struct FILE_SEGMENT_ELEMENT(i32);
#[repr(transparent)]
pub struct FILE_SHARE_MODE(pub u32);
pub const FILE_SHARE_NONE: FILE_SHARE_MODE = FILE_SHARE_MODE(0u32);
pub const FILE_SHARE_DELETE: FILE_SHARE_MODE = FILE_SHARE_MODE(4u32);
pub const FILE_SHARE_READ: FILE_SHARE_MODE = FILE_SHARE_MODE(1u32);
pub const FILE_SHARE_WRITE: FILE_SHARE_MODE = FILE_SHARE_MODE(2u32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct FILE_STANDARD_INFO(i32);
#[repr(C)]
pub struct FILE_STORAGE_INFO(i32);
#[repr(C)]
pub struct FILE_STREAM_INFO(i32);
#[repr(transparent)]
pub struct FINDEX_INFO_LEVELS(pub i32);
pub const FindExInfoStandard: FINDEX_INFO_LEVELS = FINDEX_INFO_LEVELS(0i32);
pub const FindExInfoBasic: FINDEX_INFO_LEVELS = FINDEX_INFO_LEVELS(1i32);
pub const FindExInfoMaxInfoLevel: FINDEX_INFO_LEVELS = FINDEX_INFO_LEVELS(2i32);
#[repr(transparent)]
pub struct FINDEX_SEARCH_OPS(pub i32);
pub const FindExSearchNameMatch: FINDEX_SEARCH_OPS = FINDEX_SEARCH_OPS(0i32);
pub const FindExSearchLimitToDirectories: FINDEX_SEARCH_OPS = FINDEX_SEARCH_OPS(1i32);
pub const FindExSearchLimitToDevices: FINDEX_SEARCH_OPS = FINDEX_SEARCH_OPS(2i32);
pub const FindExSearchMaxSearchOp: FINDEX_SEARCH_OPS = FINDEX_SEARCH_OPS(3i32);
#[repr(transparent)]
pub struct FIND_FIRST_EX_FLAGS(pub u32);
pub const FIND_FIRST_EX_CASE_SENSITIVE: FIND_FIRST_EX_FLAGS = FIND_FIRST_EX_FLAGS(1u32);
pub const FIND_FIRST_EX_LARGE_FETCH: FIND_FIRST_EX_FLAGS = FIND_FIRST_EX_FLAGS(2u32);
pub const FIND_FIRST_EX_ON_DISK_ENTRIES_ONLY: FIND_FIRST_EX_FLAGS = FIND_FIRST_EX_FLAGS(4u32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct FIO_CONTEXT(i32);
#[repr(C)]
pub struct FindChangeNotificationHandle(i32);
#[repr(C)]
pub struct FindFileHandle(i32);
#[repr(C)]
pub struct FindFileNameHandle(i32);
#[repr(C)]
pub struct FindStreamHandle(i32);
#[repr(C)]
pub struct FindVolumeHandle(i32);
#[repr(C)]
pub struct FindVolumeMointPointHandle(i32);
#[repr(transparent)]
pub struct GET_FILEEX_INFO_LEVELS(pub i32);
pub const GetFileExInfoStandard: GET_FILEEX_INFO_LEVELS = GET_FILEEX_INFO_LEVELS(0i32);
pub const GetFileExMaxInfoLevel: GET_FILEEX_INFO_LEVELS = GET_FILEEX_INFO_LEVELS(1i32);
#[repr(transparent)]
pub struct GET_FILE_VERSION_INFO_FLAGS(pub u32);
pub const FILE_VER_GET_LOCALISED: GET_FILE_VERSION_INFO_FLAGS = GET_FILE_VERSION_INFO_FLAGS(1u32);
pub const FILE_VER_GET_NEUTRAL: GET_FILE_VERSION_INFO_FLAGS = GET_FILE_VERSION_INFO_FLAGS(2u32);
pub const FILE_VER_GET_PREFETCHED: GET_FILE_VERSION_INFO_FLAGS = GET_FILE_VERSION_INFO_FLAGS(4u32);
#[repr(transparent)]
pub struct GET_TAPE_DRIVE_PARAMETERS_OPERATION(pub u32);
pub const GET_TAPE_DRIVE_INFORMATION: GET_TAPE_DRIVE_PARAMETERS_OPERATION = GET_TAPE_DRIVE_PARAMETERS_OPERATION(1u32);
pub const GET_TAPE_MEDIA_INFORMATION: GET_TAPE_DRIVE_PARAMETERS_OPERATION = GET_TAPE_DRIVE_PARAMETERS_OPERATION(0u32);
#[repr(C)]
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
#[repr(C)]
pub struct IORING_BUFFER_INFO(i32);
#[repr(C)]
pub struct IORING_BUFFER_REF(i32);
#[repr(C)]
pub struct IORING_CAPABILITIES(i32);
#[repr(C)]
pub struct IORING_CQE(i32);
#[repr(transparent)]
pub struct IORING_CREATE_ADVISORY_FLAGS(pub i32);
pub const IORING_CREATE_ADVISORY_FLAGS_NONE: IORING_CREATE_ADVISORY_FLAGS = IORING_CREATE_ADVISORY_FLAGS(0i32);
#[repr(C)]
pub struct IORING_CREATE_FLAGS(i32);
#[repr(transparent)]
pub struct IORING_CREATE_REQUIRED_FLAGS(pub i32);
pub const IORING_CREATE_REQUIRED_FLAGS_NONE: IORING_CREATE_REQUIRED_FLAGS = IORING_CREATE_REQUIRED_FLAGS(0i32);
#[repr(transparent)]
pub struct IORING_FEATURE_FLAGS(pub i32);
pub const IORING_FEATURE_FLAGS_NONE: IORING_FEATURE_FLAGS = IORING_FEATURE_FLAGS(0i32);
pub const IORING_FEATURE_UM_EMULATION: IORING_FEATURE_FLAGS = IORING_FEATURE_FLAGS(1i32);
pub const IORING_FEATURE_SET_COMPLETION_EVENT: IORING_FEATURE_FLAGS = IORING_FEATURE_FLAGS(2i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct IORING_HANDLE_REF(i32);
#[repr(C)]
pub struct IORING_INFO(i32);
#[repr(transparent)]
pub struct IORING_OP_CODE(pub i32);
pub const IORING_OP_NOP: IORING_OP_CODE = IORING_OP_CODE(0i32);
pub const IORING_OP_READ: IORING_OP_CODE = IORING_OP_CODE(1i32);
pub const IORING_OP_REGISTER_FILES: IORING_OP_CODE = IORING_OP_CODE(2i32);
pub const IORING_OP_REGISTER_BUFFERS: IORING_OP_CODE = IORING_OP_CODE(3i32);
pub const IORING_OP_CANCEL: IORING_OP_CODE = IORING_OP_CODE(4i32);
#[repr(transparent)]
pub struct IORING_REF_KIND(pub i32);
pub const IORING_REF_RAW: IORING_REF_KIND = IORING_REF_KIND(0i32);
pub const IORING_REF_REGISTERED: IORING_REF_KIND = IORING_REF_KIND(1i32);
#[repr(C)]
pub struct IORING_REGISTERED_BUFFER(i32);
#[repr(transparent)]
pub struct IORING_SQE_FLAGS(pub i32);
pub const IOSQE_FLAGS_NONE: IORING_SQE_FLAGS = IORING_SQE_FLAGS(0i32);
#[repr(transparent)]
pub struct IORING_VERSION(pub i32);
pub const IORING_VERSION_INVALID: IORING_VERSION = IORING_VERSION(0i32);
pub const IORING_VERSION_1: IORING_VERSION = IORING_VERSION(1i32);
#[repr(C)]
pub struct KCRM_MARSHAL_HEADER(i32);
#[repr(C)]
pub struct KCRM_PROTOCOL_BLOB(i32);
#[repr(C)]
pub struct KCRM_TRANSACTION_BLOB(i32);
pub const KTM_MARSHAL_BLOB_VERSION_MAJOR: u32 = 1u32;
pub const KTM_MARSHAL_BLOB_VERSION_MINOR: u32 = 1u32;
#[repr(transparent)]
pub struct LOCK_FILE_FLAGS(pub u32);
pub const LOCKFILE_EXCLUSIVE_LOCK: LOCK_FILE_FLAGS = LOCK_FILE_FLAGS(2u32);
pub const LOCKFILE_FAIL_IMMEDIATELY: LOCK_FILE_FLAGS = LOCK_FILE_FLAGS(1u32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct LOG_MANAGEMENT_CALLBACKS(i32);
pub const LOG_POLICY_OVERWRITE: u32 = 1u32;
pub const LOG_POLICY_PERSIST: u32 = 2u32;
#[repr(C)]
pub struct LPPROGRESS_ROUTINE(i32);
#[repr(transparent)]
pub struct LPPROGRESS_ROUTINE_CALLBACK_REASON(pub u32);
pub const CALLBACK_CHUNK_FINISHED: LPPROGRESS_ROUTINE_CALLBACK_REASON = LPPROGRESS_ROUTINE_CALLBACK_REASON(0u32);
pub const CALLBACK_STREAM_SWITCH: LPPROGRESS_ROUTINE_CALLBACK_REASON = LPPROGRESS_ROUTINE_CALLBACK_REASON(1u32);
pub const LZERROR_BADINHANDLE: i32 = -1i32;
pub const LZERROR_BADOUTHANDLE: i32 = -2i32;
pub const LZERROR_BADVALUE: i32 = -7i32;
pub const LZERROR_GLOBALLOC: i32 = -5i32;
pub const LZERROR_GLOBLOCK: i32 = -6i32;
pub const LZERROR_READ: i32 = -3i32;
pub const LZERROR_UNKNOWNALG: i32 = -8i32;
pub const LZERROR_WRITE: i32 = -4i32;
#[repr(transparent)]
pub struct LZOPENFILE_STYLE(pub u32);
pub const OF_CANCEL: LZOPENFILE_STYLE = LZOPENFILE_STYLE(2048u32);
pub const OF_CREATE: LZOPENFILE_STYLE = LZOPENFILE_STYLE(4096u32);
pub const OF_DELETE: LZOPENFILE_STYLE = LZOPENFILE_STYLE(512u32);
pub const OF_EXIST: LZOPENFILE_STYLE = LZOPENFILE_STYLE(16384u32);
pub const OF_PARSE: LZOPENFILE_STYLE = LZOPENFILE_STYLE(256u32);
pub const OF_PROMPT: LZOPENFILE_STYLE = LZOPENFILE_STYLE(8192u32);
pub const OF_READ: LZOPENFILE_STYLE = LZOPENFILE_STYLE(0u32);
pub const OF_READWRITE: LZOPENFILE_STYLE = LZOPENFILE_STYLE(2u32);
pub const OF_REOPEN: LZOPENFILE_STYLE = LZOPENFILE_STYLE(32768u32);
pub const OF_SHARE_DENY_NONE: LZOPENFILE_STYLE = LZOPENFILE_STYLE(64u32);
pub const OF_SHARE_DENY_READ: LZOPENFILE_STYLE = LZOPENFILE_STYLE(48u32);
pub const OF_SHARE_DENY_WRITE: LZOPENFILE_STYLE = LZOPENFILE_STYLE(32u32);
pub const OF_SHARE_EXCLUSIVE: LZOPENFILE_STYLE = LZOPENFILE_STYLE(16u32);
pub const OF_WRITE: LZOPENFILE_STYLE = LZOPENFILE_STYLE(1u32);
pub const OF_SHARE_COMPAT: LZOPENFILE_STYLE = LZOPENFILE_STYLE(0u32);
pub const OF_VERIFY: LZOPENFILE_STYLE = LZOPENFILE_STYLE(1024u32);
#[repr(C)]
pub struct MAXMEDIALABEL(i32);
pub const MAX_RESOURCEMANAGER_DESCRIPTION_LENGTH: u32 = 64u32;
pub const MAX_SID_SIZE: u32 = 256u32;
pub const MAX_TRANSACTION_DESCRIPTION_LENGTH: u32 = 64u32;
#[repr(transparent)]
pub struct MOVE_FILE_FLAGS(pub u32);
pub const MOVEFILE_COPY_ALLOWED: MOVE_FILE_FLAGS = MOVE_FILE_FLAGS(2u32);
pub const MOVEFILE_CREATE_HARDLINK: MOVE_FILE_FLAGS = MOVE_FILE_FLAGS(16u32);
pub const MOVEFILE_DELAY_UNTIL_REBOOT: MOVE_FILE_FLAGS = MOVE_FILE_FLAGS(4u32);
pub const MOVEFILE_REPLACE_EXISTING: MOVE_FILE_FLAGS = MOVE_FILE_FLAGS(1u32);
pub const MOVEFILE_WRITE_THROUGH: MOVE_FILE_FLAGS = MOVE_FILE_FLAGS(8u32);
pub const MOVEFILE_FAIL_IF_NOT_TRACKABLE: MOVE_FILE_FLAGS = MOVE_FILE_FLAGS(32u32);
#[repr(C)]
pub struct MediaLabelInfo(i32);
#[repr(C)]
pub struct NAME_CACHE_CONTEXT(i32);
pub const NTMSMLI_MAXAPPDESCR: u32 = 256u32;
pub const NTMSMLI_MAXIDSIZE: u32 = 256u32;
pub const NTMSMLI_MAXTYPE: u32 = 64u32;
#[repr(C)]
pub struct NTMS_ALLOCATION_INFORMATION(i32);
pub const NTMS_APPLICATIONNAME_LENGTH: u32 = 64u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NTMS_ASYNC_IO(i32);
pub const NTMS_BARCODE_LENGTH: u32 = 64u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NTMS_CHANGERINFORMATIONA(i32);
#[repr(C)]
pub struct NTMS_CHANGERINFORMATIONW(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NTMS_CHANGERTYPEINFORMATIONA(i32);
#[repr(C)]
pub struct NTMS_CHANGERTYPEINFORMATIONW(i32);
#[repr(C)]
pub struct NTMS_COMPUTERINFORMATION(i32);
pub const NTMS_COMPUTERNAME_LENGTH: u32 = 64u32;
pub const NTMS_DESCRIPTION_LENGTH: u32 = 127u32;
pub const NTMS_DEVICENAME_LENGTH: u32 = 64u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NTMS_DRIVEINFORMATIONA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NTMS_DRIVEINFORMATIONW(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NTMS_DRIVETYPEINFORMATIONA(i32);
#[repr(C)]
pub struct NTMS_DRIVETYPEINFORMATIONW(i32);
#[repr(C)]
pub struct NTMS_FILESYSTEM_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NTMS_I1_LIBRARYINFORMATION(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NTMS_I1_LIBREQUESTINFORMATIONA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NTMS_I1_LIBREQUESTINFORMATIONW(i32);
pub const NTMS_I1_MESSAGE_LENGTH: u32 = 127u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NTMS_I1_OBJECTINFORMATIONA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NTMS_I1_OBJECTINFORMATIONW(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NTMS_I1_OPREQUESTINFORMATIONA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NTMS_I1_OPREQUESTINFORMATIONW(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NTMS_I1_PARTITIONINFORMATIONA(i32);
#[repr(C)]
pub struct NTMS_I1_PARTITIONINFORMATIONW(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NTMS_I1_PMIDINFORMATIONA(i32);
#[repr(C)]
pub struct NTMS_I1_PMIDINFORMATIONW(i32);
#[repr(C)]
pub struct NTMS_IEDOORINFORMATION(i32);
#[repr(C)]
pub struct NTMS_IEPORTINFORMATION(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NTMS_LIBRARYINFORMATION(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NTMS_LIBREQUESTINFORMATIONA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NTMS_LIBREQUESTINFORMATIONW(i32);
#[repr(C)]
pub struct NTMS_LMIDINFORMATION(i32);
pub const NTMS_MAXATTR_LENGTH: u32 = 65536u32;
pub const NTMS_MAXATTR_NAMELEN: u32 = 32u32;
#[repr(C)]
pub struct NTMS_MEDIAPOOLINFORMATION(i32);
#[repr(C)]
pub struct NTMS_MEDIATYPEINFORMATION(i32);
pub const NTMS_MESSAGE_LENGTH: u32 = 256u32;
#[repr(C)]
pub struct NTMS_MOUNT_INFORMATION(i32);
#[repr(C)]
pub struct NTMS_NOTIFICATIONINFORMATION(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NTMS_OBJECTINFORMATIONA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NTMS_OBJECTINFORMATIONW(i32);
pub const NTMS_OBJECTNAME_LENGTH: u32 = 64u32;
pub const NTMS_OMIDLABELID_LENGTH: u32 = 255u32;
pub const NTMS_OMIDLABELINFO_LENGTH: u32 = 256u32;
pub const NTMS_OMIDLABELTYPE_LENGTH: u32 = 64u32;
#[repr(transparent)]
pub struct NTMS_OMID_TYPE(pub u32);
pub const NTMS_OMID_TYPE_FILESYSTEM_INFO: NTMS_OMID_TYPE = NTMS_OMID_TYPE(2u32);
pub const NTMS_OMID_TYPE_RAW_LABEL: NTMS_OMID_TYPE = NTMS_OMID_TYPE(1u32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NTMS_OPREQUESTINFORMATIONA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NTMS_OPREQUESTINFORMATIONW(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NTMS_PARTITIONINFORMATIONA(i32);
#[repr(C)]
pub struct NTMS_PARTITIONINFORMATIONW(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NTMS_PMIDINFORMATIONA(i32);
#[repr(C)]
pub struct NTMS_PMIDINFORMATIONW(i32);
pub const NTMS_POOLHIERARCHY_LENGTH: u32 = 512u32;
pub const NTMS_PRODUCTNAME_LENGTH: u32 = 128u32;
pub const NTMS_REVISION_LENGTH: u32 = 32u32;
pub const NTMS_SEQUENCE_LENGTH: u32 = 32u32;
pub const NTMS_SERIALNUMBER_LENGTH: u32 = 32u32;
#[repr(C)]
pub struct NTMS_STORAGESLOTINFORMATION(i32);
pub const NTMS_USERNAME_LENGTH: u32 = 64u32;
pub const NTMS_VENDORNAME_LENGTH: u32 = 128u32;
#[repr(transparent)]
pub struct NT_CREATE_FILE_DISPOSITION(pub u32);
pub const FILE_SUPERSEDE: NT_CREATE_FILE_DISPOSITION = NT_CREATE_FILE_DISPOSITION(0u32);
pub const FILE_CREATE: NT_CREATE_FILE_DISPOSITION = NT_CREATE_FILE_DISPOSITION(2u32);
pub const FILE_OPEN: NT_CREATE_FILE_DISPOSITION = NT_CREATE_FILE_DISPOSITION(1u32);
pub const FILE_OPEN_IF: NT_CREATE_FILE_DISPOSITION = NT_CREATE_FILE_DISPOSITION(3u32);
pub const FILE_OVERWRITE: NT_CREATE_FILE_DISPOSITION = NT_CREATE_FILE_DISPOSITION(4u32);
pub const FILE_OVERWRITE_IF: NT_CREATE_FILE_DISPOSITION = NT_CREATE_FILE_DISPOSITION(5u32);
#[repr(transparent)]
pub struct NtmsAccessMask(pub i32);
pub const NTMS_USE_ACCESS: NtmsAccessMask = NtmsAccessMask(1i32);
pub const NTMS_MODIFY_ACCESS: NtmsAccessMask = NtmsAccessMask(2i32);
pub const NTMS_CONTROL_ACCESS: NtmsAccessMask = NtmsAccessMask(4i32);
#[repr(transparent)]
pub struct NtmsAllocateOptions(pub i32);
pub const NTMS_ALLOCATE_NEW: NtmsAllocateOptions = NtmsAllocateOptions(1i32);
pub const NTMS_ALLOCATE_NEXT: NtmsAllocateOptions = NtmsAllocateOptions(2i32);
pub const NTMS_ALLOCATE_ERROR_IF_UNAVAILABLE: NtmsAllocateOptions = NtmsAllocateOptions(4i32);
#[repr(transparent)]
pub struct NtmsAllocationPolicy(pub i32);
pub const NTMS_ALLOCATE_FROMSCRATCH: NtmsAllocationPolicy = NtmsAllocationPolicy(1i32);
#[repr(transparent)]
pub struct NtmsAsyncOperations(pub i32);
pub const NTMS_ASYNCOP_MOUNT: NtmsAsyncOperations = NtmsAsyncOperations(1i32);
#[repr(transparent)]
pub struct NtmsAsyncStatus(pub i32);
pub const NTMS_ASYNCSTATE_QUEUED: NtmsAsyncStatus = NtmsAsyncStatus(0i32);
pub const NTMS_ASYNCSTATE_WAIT_RESOURCE: NtmsAsyncStatus = NtmsAsyncStatus(1i32);
pub const NTMS_ASYNCSTATE_WAIT_OPERATOR: NtmsAsyncStatus = NtmsAsyncStatus(2i32);
pub const NTMS_ASYNCSTATE_INPROCESS: NtmsAsyncStatus = NtmsAsyncStatus(3i32);
pub const NTMS_ASYNCSTATE_COMPLETE: NtmsAsyncStatus = NtmsAsyncStatus(4i32);
#[repr(transparent)]
pub struct NtmsBarCodeState(pub i32);
pub const NTMS_BARCODESTATE_OK: NtmsBarCodeState = NtmsBarCodeState(1i32);
pub const NTMS_BARCODESTATE_UNREADABLE: NtmsBarCodeState = NtmsBarCodeState(2i32);
#[repr(transparent)]
pub struct NtmsCreateNtmsMediaOptions(pub i32);
pub const NTMS_ERROR_ON_DUPLICATE: NtmsCreateNtmsMediaOptions = NtmsCreateNtmsMediaOptions(1i32);
#[repr(transparent)]
pub struct NtmsCreateOptions(pub i32);
pub const NTMS_OPEN_EXISTING: NtmsCreateOptions = NtmsCreateOptions(1i32);
pub const NTMS_CREATE_NEW: NtmsCreateOptions = NtmsCreateOptions(2i32);
pub const NTMS_OPEN_ALWAYS: NtmsCreateOptions = NtmsCreateOptions(3i32);
#[repr(transparent)]
pub struct NtmsDeallocationPolicy(pub i32);
pub const NTMS_DEALLOCATE_TOSCRATCH: NtmsDeallocationPolicy = NtmsDeallocationPolicy(1i32);
#[repr(transparent)]
pub struct NtmsDismountOptions(pub i32);
pub const NTMS_DISMOUNT_DEFERRED: NtmsDismountOptions = NtmsDismountOptions(1i32);
pub const NTMS_DISMOUNT_IMMEDIATE: NtmsDismountOptions = NtmsDismountOptions(2i32);
#[repr(transparent)]
pub struct NtmsDoorState(pub i32);
pub const NTMS_DOORSTATE_UNKNOWN: NtmsDoorState = NtmsDoorState(0i32);
pub const NTMS_DOORSTATE_CLOSED: NtmsDoorState = NtmsDoorState(1i32);
pub const NTMS_DOORSTATE_OPEN: NtmsDoorState = NtmsDoorState(2i32);
#[repr(transparent)]
pub struct NtmsDriveState(pub i32);
pub const NTMS_DRIVESTATE_DISMOUNTED: NtmsDriveState = NtmsDriveState(0i32);
pub const NTMS_DRIVESTATE_MOUNTED: NtmsDriveState = NtmsDriveState(1i32);
pub const NTMS_DRIVESTATE_LOADED: NtmsDriveState = NtmsDriveState(2i32);
pub const NTMS_DRIVESTATE_UNLOADED: NtmsDriveState = NtmsDriveState(5i32);
pub const NTMS_DRIVESTATE_BEING_CLEANED: NtmsDriveState = NtmsDriveState(6i32);
pub const NTMS_DRIVESTATE_DISMOUNTABLE: NtmsDriveState = NtmsDriveState(7i32);
#[repr(transparent)]
pub struct NtmsDriveType(pub i32);
pub const NTMS_UNKNOWN_DRIVE: NtmsDriveType = NtmsDriveType(0i32);
#[repr(transparent)]
pub struct NtmsEjectOperation(pub i32);
pub const NTMS_EJECT_START: NtmsEjectOperation = NtmsEjectOperation(0i32);
pub const NTMS_EJECT_STOP: NtmsEjectOperation = NtmsEjectOperation(1i32);
pub const NTMS_EJECT_QUEUE: NtmsEjectOperation = NtmsEjectOperation(2i32);
pub const NTMS_EJECT_FORCE: NtmsEjectOperation = NtmsEjectOperation(3i32);
pub const NTMS_EJECT_IMMEDIATE: NtmsEjectOperation = NtmsEjectOperation(4i32);
pub const NTMS_EJECT_ASK_USER: NtmsEjectOperation = NtmsEjectOperation(5i32);
#[repr(transparent)]
pub struct NtmsEnumerateOption(pub i32);
pub const NTMS_ENUM_DEFAULT: NtmsEnumerateOption = NtmsEnumerateOption(0i32);
pub const NTMS_ENUM_ROOTPOOL: NtmsEnumerateOption = NtmsEnumerateOption(1i32);
#[repr(transparent)]
pub struct NtmsInjectOperation(pub i32);
pub const NTMS_INJECT_START: NtmsInjectOperation = NtmsInjectOperation(0i32);
pub const NTMS_INJECT_STOP: NtmsInjectOperation = NtmsInjectOperation(1i32);
pub const NTMS_INJECT_RETRACT: NtmsInjectOperation = NtmsInjectOperation(2i32);
pub const NTMS_INJECT_STARTMANY: NtmsInjectOperation = NtmsInjectOperation(3i32);
#[repr(transparent)]
pub struct NtmsInventoryMethod(pub i32);
pub const NTMS_INVENTORY_NONE: NtmsInventoryMethod = NtmsInventoryMethod(0i32);
pub const NTMS_INVENTORY_FAST: NtmsInventoryMethod = NtmsInventoryMethod(1i32);
pub const NTMS_INVENTORY_OMID: NtmsInventoryMethod = NtmsInventoryMethod(2i32);
pub const NTMS_INVENTORY_DEFAULT: NtmsInventoryMethod = NtmsInventoryMethod(3i32);
pub const NTMS_INVENTORY_SLOT: NtmsInventoryMethod = NtmsInventoryMethod(4i32);
pub const NTMS_INVENTORY_STOP: NtmsInventoryMethod = NtmsInventoryMethod(5i32);
pub const NTMS_INVENTORY_MAX: NtmsInventoryMethod = NtmsInventoryMethod(6i32);
#[repr(transparent)]
pub struct NtmsLibRequestFlags(pub i32);
pub const NTMS_LIBREQFLAGS_NOAUTOPURGE: NtmsLibRequestFlags = NtmsLibRequestFlags(1i32);
pub const NTMS_LIBREQFLAGS_NOFAILEDPURGE: NtmsLibRequestFlags = NtmsLibRequestFlags(2i32);
#[repr(transparent)]
pub struct NtmsLibraryFlags(pub i32);
pub const NTMS_LIBRARYFLAG_FIXEDOFFLINE: NtmsLibraryFlags = NtmsLibraryFlags(1i32);
pub const NTMS_LIBRARYFLAG_CLEANERPRESENT: NtmsLibraryFlags = NtmsLibraryFlags(2i32);
pub const NTMS_LIBRARYFLAG_AUTODETECTCHANGE: NtmsLibraryFlags = NtmsLibraryFlags(4i32);
pub const NTMS_LIBRARYFLAG_IGNORECLEANERUSESREMAINING: NtmsLibraryFlags = NtmsLibraryFlags(8i32);
pub const NTMS_LIBRARYFLAG_RECOGNIZECLEANERBARCODE: NtmsLibraryFlags = NtmsLibraryFlags(16i32);
#[repr(transparent)]
pub struct NtmsLibraryType(pub i32);
pub const NTMS_LIBRARYTYPE_UNKNOWN: NtmsLibraryType = NtmsLibraryType(0i32);
pub const NTMS_LIBRARYTYPE_OFFLINE: NtmsLibraryType = NtmsLibraryType(1i32);
pub const NTMS_LIBRARYTYPE_ONLINE: NtmsLibraryType = NtmsLibraryType(2i32);
pub const NTMS_LIBRARYTYPE_STANDALONE: NtmsLibraryType = NtmsLibraryType(3i32);
#[repr(transparent)]
pub struct NtmsLmOperation(pub i32);
pub const NTMS_LM_REMOVE: NtmsLmOperation = NtmsLmOperation(0i32);
pub const NTMS_LM_DISABLECHANGER: NtmsLmOperation = NtmsLmOperation(1i32);
pub const NTMS_LM_DISABLELIBRARY: NtmsLmOperation = NtmsLmOperation(1i32);
pub const NTMS_LM_ENABLECHANGER: NtmsLmOperation = NtmsLmOperation(2i32);
pub const NTMS_LM_ENABLELIBRARY: NtmsLmOperation = NtmsLmOperation(2i32);
pub const NTMS_LM_DISABLEDRIVE: NtmsLmOperation = NtmsLmOperation(3i32);
pub const NTMS_LM_ENABLEDRIVE: NtmsLmOperation = NtmsLmOperation(4i32);
pub const NTMS_LM_DISABLEMEDIA: NtmsLmOperation = NtmsLmOperation(5i32);
pub const NTMS_LM_ENABLEMEDIA: NtmsLmOperation = NtmsLmOperation(6i32);
pub const NTMS_LM_UPDATEOMID: NtmsLmOperation = NtmsLmOperation(7i32);
pub const NTMS_LM_INVENTORY: NtmsLmOperation = NtmsLmOperation(8i32);
pub const NTMS_LM_DOORACCESS: NtmsLmOperation = NtmsLmOperation(9i32);
pub const NTMS_LM_EJECT: NtmsLmOperation = NtmsLmOperation(10i32);
pub const NTMS_LM_EJECTCLEANER: NtmsLmOperation = NtmsLmOperation(11i32);
pub const NTMS_LM_INJECT: NtmsLmOperation = NtmsLmOperation(12i32);
pub const NTMS_LM_INJECTCLEANER: NtmsLmOperation = NtmsLmOperation(13i32);
pub const NTMS_LM_PROCESSOMID: NtmsLmOperation = NtmsLmOperation(14i32);
pub const NTMS_LM_CLEANDRIVE: NtmsLmOperation = NtmsLmOperation(15i32);
pub const NTMS_LM_DISMOUNT: NtmsLmOperation = NtmsLmOperation(16i32);
pub const NTMS_LM_MOUNT: NtmsLmOperation = NtmsLmOperation(17i32);
pub const NTMS_LM_WRITESCRATCH: NtmsLmOperation = NtmsLmOperation(18i32);
pub const NTMS_LM_CLASSIFY: NtmsLmOperation = NtmsLmOperation(19i32);
pub const NTMS_LM_RESERVECLEANER: NtmsLmOperation = NtmsLmOperation(20i32);
pub const NTMS_LM_RELEASECLEANER: NtmsLmOperation = NtmsLmOperation(21i32);
pub const NTMS_LM_MAXWORKITEM: NtmsLmOperation = NtmsLmOperation(22i32);
#[repr(transparent)]
pub struct NtmsLmState(pub i32);
pub const NTMS_LM_QUEUED: NtmsLmState = NtmsLmState(0i32);
pub const NTMS_LM_INPROCESS: NtmsLmState = NtmsLmState(1i32);
pub const NTMS_LM_PASSED: NtmsLmState = NtmsLmState(2i32);
pub const NTMS_LM_FAILED: NtmsLmState = NtmsLmState(3i32);
pub const NTMS_LM_INVALID: NtmsLmState = NtmsLmState(4i32);
pub const NTMS_LM_WAITING: NtmsLmState = NtmsLmState(5i32);
pub const NTMS_LM_DEFERRED: NtmsLmState = NtmsLmState(6i32);
pub const NTMS_LM_DEFFERED: NtmsLmState = NtmsLmState(6i32);
pub const NTMS_LM_CANCELLED: NtmsLmState = NtmsLmState(7i32);
pub const NTMS_LM_STOPPED: NtmsLmState = NtmsLmState(8i32);
#[repr(transparent)]
pub struct NtmsMediaPoolPolicy(pub i32);
pub const NTMS_POOLPOLICY_PURGEOFFLINESCRATCH: NtmsMediaPoolPolicy = NtmsMediaPoolPolicy(1i32);
pub const NTMS_POOLPOLICY_KEEPOFFLINEIMPORT: NtmsMediaPoolPolicy = NtmsMediaPoolPolicy(2i32);
#[repr(transparent)]
pub struct NtmsMediaState(pub i32);
pub const NTMS_MEDIASTATE_IDLE: NtmsMediaState = NtmsMediaState(0i32);
pub const NTMS_MEDIASTATE_INUSE: NtmsMediaState = NtmsMediaState(1i32);
pub const NTMS_MEDIASTATE_MOUNTED: NtmsMediaState = NtmsMediaState(2i32);
pub const NTMS_MEDIASTATE_LOADED: NtmsMediaState = NtmsMediaState(3i32);
pub const NTMS_MEDIASTATE_UNLOADED: NtmsMediaState = NtmsMediaState(4i32);
pub const NTMS_MEDIASTATE_OPERROR: NtmsMediaState = NtmsMediaState(5i32);
pub const NTMS_MEDIASTATE_OPREQ: NtmsMediaState = NtmsMediaState(6i32);
#[repr(transparent)]
pub struct NtmsMountOptions(pub i32);
pub const NTMS_MOUNT_READ: NtmsMountOptions = NtmsMountOptions(1i32);
pub const NTMS_MOUNT_WRITE: NtmsMountOptions = NtmsMountOptions(2i32);
pub const NTMS_MOUNT_ERROR_NOT_AVAILABLE: NtmsMountOptions = NtmsMountOptions(4i32);
pub const NTMS_MOUNT_ERROR_IF_UNAVAILABLE: NtmsMountOptions = NtmsMountOptions(4i32);
pub const NTMS_MOUNT_ERROR_OFFLINE: NtmsMountOptions = NtmsMountOptions(8i32);
pub const NTMS_MOUNT_ERROR_IF_OFFLINE: NtmsMountOptions = NtmsMountOptions(8i32);
pub const NTMS_MOUNT_SPECIFIC_DRIVE: NtmsMountOptions = NtmsMountOptions(16i32);
pub const NTMS_MOUNT_NOWAIT: NtmsMountOptions = NtmsMountOptions(32i32);
#[repr(transparent)]
pub struct NtmsMountPriority(pub i32);
pub const NTMS_PRIORITY_DEFAULT: NtmsMountPriority = NtmsMountPriority(0i32);
pub const NTMS_PRIORITY_HIGHEST: NtmsMountPriority = NtmsMountPriority(15i32);
pub const NTMS_PRIORITY_HIGH: NtmsMountPriority = NtmsMountPriority(7i32);
pub const NTMS_PRIORITY_NORMAL: NtmsMountPriority = NtmsMountPriority(0i32);
pub const NTMS_PRIORITY_LOW: NtmsMountPriority = NtmsMountPriority(-7i32);
pub const NTMS_PRIORITY_LOWEST: NtmsMountPriority = NtmsMountPriority(-15i32);
#[repr(transparent)]
pub struct NtmsNotificationOperations(pub i32);
pub const NTMS_OBJ_UPDATE: NtmsNotificationOperations = NtmsNotificationOperations(1i32);
pub const NTMS_OBJ_INSERT: NtmsNotificationOperations = NtmsNotificationOperations(2i32);
pub const NTMS_OBJ_DELETE: NtmsNotificationOperations = NtmsNotificationOperations(3i32);
pub const NTMS_EVENT_SIGNAL: NtmsNotificationOperations = NtmsNotificationOperations(4i32);
pub const NTMS_EVENT_COMPLETE: NtmsNotificationOperations = NtmsNotificationOperations(5i32);
#[repr(transparent)]
pub struct NtmsObjectsTypes(pub i32);
pub const NTMS_UNKNOWN: NtmsObjectsTypes = NtmsObjectsTypes(0i32);
pub const NTMS_OBJECT: NtmsObjectsTypes = NtmsObjectsTypes(1i32);
pub const NTMS_CHANGER: NtmsObjectsTypes = NtmsObjectsTypes(2i32);
pub const NTMS_CHANGER_TYPE: NtmsObjectsTypes = NtmsObjectsTypes(3i32);
pub const NTMS_COMPUTER: NtmsObjectsTypes = NtmsObjectsTypes(4i32);
pub const NTMS_DRIVE: NtmsObjectsTypes = NtmsObjectsTypes(5i32);
pub const NTMS_DRIVE_TYPE: NtmsObjectsTypes = NtmsObjectsTypes(6i32);
pub const NTMS_IEDOOR: NtmsObjectsTypes = NtmsObjectsTypes(7i32);
pub const NTMS_IEPORT: NtmsObjectsTypes = NtmsObjectsTypes(8i32);
pub const NTMS_LIBRARY: NtmsObjectsTypes = NtmsObjectsTypes(9i32);
pub const NTMS_LIBREQUEST: NtmsObjectsTypes = NtmsObjectsTypes(10i32);
pub const NTMS_LOGICAL_MEDIA: NtmsObjectsTypes = NtmsObjectsTypes(11i32);
pub const NTMS_MEDIA_POOL: NtmsObjectsTypes = NtmsObjectsTypes(12i32);
pub const NTMS_MEDIA_TYPE: NtmsObjectsTypes = NtmsObjectsTypes(13i32);
pub const NTMS_PARTITION: NtmsObjectsTypes = NtmsObjectsTypes(14i32);
pub const NTMS_PHYSICAL_MEDIA: NtmsObjectsTypes = NtmsObjectsTypes(15i32);
pub const NTMS_STORAGESLOT: NtmsObjectsTypes = NtmsObjectsTypes(16i32);
pub const NTMS_OPREQUEST: NtmsObjectsTypes = NtmsObjectsTypes(17i32);
pub const NTMS_UI_DESTINATION: NtmsObjectsTypes = NtmsObjectsTypes(18i32);
pub const NTMS_NUMBER_OF_OBJECT_TYPES: NtmsObjectsTypes = NtmsObjectsTypes(19i32);
#[repr(transparent)]
pub struct NtmsOpRequestFlags(pub i32);
pub const NTMS_OPREQFLAGS_NOAUTOPURGE: NtmsOpRequestFlags = NtmsOpRequestFlags(1i32);
pub const NTMS_OPREQFLAGS_NOFAILEDPURGE: NtmsOpRequestFlags = NtmsOpRequestFlags(2i32);
pub const NTMS_OPREQFLAGS_NOALERTS: NtmsOpRequestFlags = NtmsOpRequestFlags(16i32);
pub const NTMS_OPREQFLAGS_NOTRAYICON: NtmsOpRequestFlags = NtmsOpRequestFlags(32i32);
#[repr(transparent)]
pub struct NtmsOperationalState(pub i32);
pub const NTMS_READY: NtmsOperationalState = NtmsOperationalState(0i32);
pub const NTMS_INITIALIZING: NtmsOperationalState = NtmsOperationalState(10i32);
pub const NTMS_NEEDS_SERVICE: NtmsOperationalState = NtmsOperationalState(20i32);
pub const NTMS_NOT_PRESENT: NtmsOperationalState = NtmsOperationalState(21i32);
#[repr(transparent)]
pub struct NtmsOpreqCommand(pub i32);
pub const NTMS_OPREQ_UNKNOWN: NtmsOpreqCommand = NtmsOpreqCommand(0i32);
pub const NTMS_OPREQ_NEWMEDIA: NtmsOpreqCommand = NtmsOpreqCommand(1i32);
pub const NTMS_OPREQ_CLEANER: NtmsOpreqCommand = NtmsOpreqCommand(2i32);
pub const NTMS_OPREQ_DEVICESERVICE: NtmsOpreqCommand = NtmsOpreqCommand(3i32);
pub const NTMS_OPREQ_MOVEMEDIA: NtmsOpreqCommand = NtmsOpreqCommand(4i32);
pub const NTMS_OPREQ_MESSAGE: NtmsOpreqCommand = NtmsOpreqCommand(5i32);
#[repr(transparent)]
pub struct NtmsOpreqState(pub i32);
pub const NTMS_OPSTATE_UNKNOWN: NtmsOpreqState = NtmsOpreqState(0i32);
pub const NTMS_OPSTATE_SUBMITTED: NtmsOpreqState = NtmsOpreqState(1i32);
pub const NTMS_OPSTATE_ACTIVE: NtmsOpreqState = NtmsOpreqState(2i32);
pub const NTMS_OPSTATE_INPROGRESS: NtmsOpreqState = NtmsOpreqState(3i32);
pub const NTMS_OPSTATE_REFUSED: NtmsOpreqState = NtmsOpreqState(4i32);
pub const NTMS_OPSTATE_COMPLETE: NtmsOpreqState = NtmsOpreqState(5i32);
#[repr(transparent)]
pub struct NtmsPartitionState(pub i32);
pub const NTMS_PARTSTATE_UNKNOWN: NtmsPartitionState = NtmsPartitionState(0i32);
pub const NTMS_PARTSTATE_UNPREPARED: NtmsPartitionState = NtmsPartitionState(1i32);
pub const NTMS_PARTSTATE_INCOMPATIBLE: NtmsPartitionState = NtmsPartitionState(2i32);
pub const NTMS_PARTSTATE_DECOMMISSIONED: NtmsPartitionState = NtmsPartitionState(3i32);
pub const NTMS_PARTSTATE_AVAILABLE: NtmsPartitionState = NtmsPartitionState(4i32);
pub const NTMS_PARTSTATE_ALLOCATED: NtmsPartitionState = NtmsPartitionState(5i32);
pub const NTMS_PARTSTATE_COMPLETE: NtmsPartitionState = NtmsPartitionState(6i32);
pub const NTMS_PARTSTATE_FOREIGN: NtmsPartitionState = NtmsPartitionState(7i32);
pub const NTMS_PARTSTATE_IMPORT: NtmsPartitionState = NtmsPartitionState(8i32);
pub const NTMS_PARTSTATE_RESERVED: NtmsPartitionState = NtmsPartitionState(9i32);
#[repr(transparent)]
pub struct NtmsPoolType(pub i32);
pub const NTMS_POOLTYPE_UNKNOWN: NtmsPoolType = NtmsPoolType(0i32);
pub const NTMS_POOLTYPE_SCRATCH: NtmsPoolType = NtmsPoolType(1i32);
pub const NTMS_POOLTYPE_FOREIGN: NtmsPoolType = NtmsPoolType(2i32);
pub const NTMS_POOLTYPE_IMPORT: NtmsPoolType = NtmsPoolType(3i32);
pub const NTMS_POOLTYPE_APPLICATION: NtmsPoolType = NtmsPoolType(1000i32);
#[repr(transparent)]
pub struct NtmsPortContent(pub i32);
pub const NTMS_PORTCONTENT_UNKNOWN: NtmsPortContent = NtmsPortContent(0i32);
pub const NTMS_PORTCONTENT_FULL: NtmsPortContent = NtmsPortContent(1i32);
pub const NTMS_PORTCONTENT_EMPTY: NtmsPortContent = NtmsPortContent(2i32);
#[repr(transparent)]
pub struct NtmsPortPosition(pub i32);
pub const NTMS_PORTPOSITION_UNKNOWN: NtmsPortPosition = NtmsPortPosition(0i32);
pub const NTMS_PORTPOSITION_EXTENDED: NtmsPortPosition = NtmsPortPosition(1i32);
pub const NTMS_PORTPOSITION_RETRACTED: NtmsPortPosition = NtmsPortPosition(2i32);
#[repr(transparent)]
pub struct NtmsReadWriteCharacteristics(pub i32);
pub const NTMS_MEDIARW_UNKNOWN: NtmsReadWriteCharacteristics = NtmsReadWriteCharacteristics(0i32);
pub const NTMS_MEDIARW_REWRITABLE: NtmsReadWriteCharacteristics = NtmsReadWriteCharacteristics(1i32);
pub const NTMS_MEDIARW_WRITEONCE: NtmsReadWriteCharacteristics = NtmsReadWriteCharacteristics(2i32);
pub const NTMS_MEDIARW_READONLY: NtmsReadWriteCharacteristics = NtmsReadWriteCharacteristics(3i32);
#[repr(transparent)]
pub struct NtmsSessionOptions(pub i32);
pub const NTMS_SESSION_QUERYEXPEDITE: NtmsSessionOptions = NtmsSessionOptions(1i32);
#[repr(transparent)]
pub struct NtmsSlotState(pub i32);
pub const NTMS_SLOTSTATE_UNKNOWN: NtmsSlotState = NtmsSlotState(0i32);
pub const NTMS_SLOTSTATE_FULL: NtmsSlotState = NtmsSlotState(1i32);
pub const NTMS_SLOTSTATE_EMPTY: NtmsSlotState = NtmsSlotState(2i32);
pub const NTMS_SLOTSTATE_NOTPRESENT: NtmsSlotState = NtmsSlotState(3i32);
pub const NTMS_SLOTSTATE_NEEDSINVENTORY: NtmsSlotState = NtmsSlotState(4i32);
#[repr(transparent)]
pub struct NtmsUIOperations(pub i32);
pub const NTMS_UIDEST_ADD: NtmsUIOperations = NtmsUIOperations(1i32);
pub const NTMS_UIDEST_DELETE: NtmsUIOperations = NtmsUIOperations(2i32);
pub const NTMS_UIDEST_DELETEALL: NtmsUIOperations = NtmsUIOperations(3i32);
pub const NTMS_UIOPERATION_MAX: NtmsUIOperations = NtmsUIOperations(4i32);
#[repr(transparent)]
pub struct NtmsUITypes(pub i32);
pub const NTMS_UITYPE_INVALID: NtmsUITypes = NtmsUITypes(0i32);
pub const NTMS_UITYPE_INFO: NtmsUITypes = NtmsUITypes(1i32);
pub const NTMS_UITYPE_REQ: NtmsUITypes = NtmsUITypes(2i32);
pub const NTMS_UITYPE_ERR: NtmsUITypes = NtmsUITypes(3i32);
pub const NTMS_UITYPE_MAX: NtmsUITypes = NtmsUITypes(4i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
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
#[repr(C)]
pub struct PCLFS_COMPLETION_ROUTINE(i32);
#[repr(C)]
pub struct PCOPYFILE2_PROGRESS_ROUTINE(i32);
#[repr(C)]
pub struct PFE_EXPORT_FUNC(i32);
#[repr(C)]
pub struct PFE_IMPORT_FUNC(i32);
#[repr(C)]
pub struct PFN_IO_COMPLETION(i32);
#[repr(C)]
pub struct PLOG_FULL_HANDLER_CALLBACK(i32);
#[repr(C)]
pub struct PLOG_TAIL_ADVANCE_CALLBACK(i32);
#[repr(C)]
pub struct PLOG_UNPINNED_CALLBACK(i32);
#[repr(transparent)]
pub struct PREPARE_TAPE_OPERATION(pub i32);
pub const TAPE_FORMAT: PREPARE_TAPE_OPERATION = PREPARE_TAPE_OPERATION(5i32);
pub const TAPE_LOAD: PREPARE_TAPE_OPERATION = PREPARE_TAPE_OPERATION(0i32);
pub const TAPE_LOCK: PREPARE_TAPE_OPERATION = PREPARE_TAPE_OPERATION(3i32);
pub const TAPE_TENSION: PREPARE_TAPE_OPERATION = PREPARE_TAPE_OPERATION(2i32);
pub const TAPE_UNLOAD: PREPARE_TAPE_OPERATION = PREPARE_TAPE_OPERATION(1i32);
pub const TAPE_UNLOCK: PREPARE_TAPE_OPERATION = PREPARE_TAPE_OPERATION(4i32);
#[repr(transparent)]
pub struct PRIORITY_HINT(pub i32);
pub const IoPriorityHintVeryLow: PRIORITY_HINT = PRIORITY_HINT(0i32);
pub const IoPriorityHintLow: PRIORITY_HINT = PRIORITY_HINT(1i32);
pub const IoPriorityHintNormal: PRIORITY_HINT = PRIORITY_HINT(2i32);
pub const MaximumIoPriorityHintType: PRIORITY_HINT = PRIORITY_HINT(3i32);
#[repr(transparent)]
pub struct READ_DIRECTORY_NOTIFY_INFORMATION_CLASS(pub i32);
pub const ReadDirectoryNotifyInformation: READ_DIRECTORY_NOTIFY_INFORMATION_CLASS = READ_DIRECTORY_NOTIFY_INFORMATION_CLASS(1i32);
pub const ReadDirectoryNotifyExtendedInformation: READ_DIRECTORY_NOTIFY_INFORMATION_CLASS = READ_DIRECTORY_NOTIFY_INFORMATION_CLASS(2i32);
#[repr(C)]
pub struct REPARSE_GUID_DATA_BUFFER(i32);
#[repr(transparent)]
pub struct REPLACE_FILE_FLAGS(pub u32);
pub const REPLACEFILE_WRITE_THROUGH: REPLACE_FILE_FLAGS = REPLACE_FILE_FLAGS(1u32);
pub const REPLACEFILE_IGNORE_MERGE_ERRORS: REPLACE_FILE_FLAGS = REPLACE_FILE_FLAGS(2u32);
pub const REPLACEFILE_IGNORE_ACL_ERRORS: REPLACE_FILE_FLAGS = REPLACE_FILE_FLAGS(4u32);
pub const RESOURCE_MANAGER_COMMUNICATION: u32 = 2u32;
pub const RESOURCE_MANAGER_MAXIMUM_OPTION: u32 = 3u32;
pub const RESOURCE_MANAGER_VOLATILE: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SERVER_ALIAS_INFO_0(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SERVER_CERTIFICATE_INFO_0(i32);
#[repr(transparent)]
pub struct SERVER_CERTIFICATE_TYPE(pub i32);
pub const QUIC: SERVER_CERTIFICATE_TYPE = SERVER_CERTIFICATE_TYPE(0i32);
pub const SESI1_NUM_ELEMENTS: u32 = 8u32;
pub const SESI2_NUM_ELEMENTS: u32 = 9u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SESSION_INFO_0(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SESSION_INFO_1(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SESSION_INFO_10(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SESSION_INFO_2(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SESSION_INFO_502(i32);
#[repr(transparent)]
pub struct SESSION_INFO_USER_FLAGS(pub u32);
pub const SESS_GUEST: SESSION_INFO_USER_FLAGS = SESSION_INFO_USER_FLAGS(1u32);
pub const SESS_NOENCRYPTION: SESSION_INFO_USER_FLAGS = SESSION_INFO_USER_FLAGS(2u32);
#[repr(transparent)]
pub struct SET_FILE_POINTER_MOVE_METHOD(pub u32);
pub const FILE_BEGIN: SET_FILE_POINTER_MOVE_METHOD = SET_FILE_POINTER_MOVE_METHOD(0u32);
pub const FILE_CURRENT: SET_FILE_POINTER_MOVE_METHOD = SET_FILE_POINTER_MOVE_METHOD(1u32);
pub const FILE_END: SET_FILE_POINTER_MOVE_METHOD = SET_FILE_POINTER_MOVE_METHOD(2u32);
pub const SHARE_CURRENT_USES_PARMNUM: u32 = 7u32;
pub const SHARE_FILE_SD_PARMNUM: u32 = 501u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SHARE_INFO_0(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SHARE_INFO_1(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SHARE_INFO_1004(i32);
#[repr(C)]
pub struct SHARE_INFO_1005(i32);
#[repr(C)]
pub struct SHARE_INFO_1006(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[repr(C)]
pub struct SHARE_INFO_1501(i32);
#[repr(C)]
pub struct SHARE_INFO_1503(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SHARE_INFO_2(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SHARE_INFO_501(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[repr(C)]
pub struct SHARE_INFO_502(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[repr(C)]
pub struct SHARE_INFO_503(i32);
#[repr(transparent)]
pub struct SHARE_INFO_PERMISSIONS(pub u32);
pub const ACCESS_READ: SHARE_INFO_PERMISSIONS = SHARE_INFO_PERMISSIONS(1u32);
pub const ACCESS_WRITE: SHARE_INFO_PERMISSIONS = SHARE_INFO_PERMISSIONS(2u32);
pub const ACCESS_CREATE: SHARE_INFO_PERMISSIONS = SHARE_INFO_PERMISSIONS(4u32);
pub const ACCESS_EXEC: SHARE_INFO_PERMISSIONS = SHARE_INFO_PERMISSIONS(8u32);
pub const ACCESS_DELETE: SHARE_INFO_PERMISSIONS = SHARE_INFO_PERMISSIONS(16u32);
pub const ACCESS_ATRIB: SHARE_INFO_PERMISSIONS = SHARE_INFO_PERMISSIONS(32u32);
pub const ACCESS_PERM: SHARE_INFO_PERMISSIONS = SHARE_INFO_PERMISSIONS(64u32);
pub const ACCESS_ALL: SHARE_INFO_PERMISSIONS = SHARE_INFO_PERMISSIONS(32768u32);
pub const SHARE_MAX_USES_PARMNUM: u32 = 6u32;
pub const SHARE_NETNAME_PARMNUM: u32 = 1u32;
pub const SHARE_PASSWD_PARMNUM: u32 = 9u32;
pub const SHARE_PATH_PARMNUM: u32 = 8u32;
pub const SHARE_PERMISSIONS_PARMNUM: u32 = 5u32;
pub const SHARE_REMARK_PARMNUM: u32 = 4u32;
pub const SHARE_SERVER_PARMNUM: u32 = 503u32;
#[repr(transparent)]
pub struct SHARE_TYPE(pub u32);
pub const STYPE_DISKTREE: SHARE_TYPE = SHARE_TYPE(0u32);
pub const STYPE_PRINTQ: SHARE_TYPE = SHARE_TYPE(1u32);
pub const STYPE_DEVICE: SHARE_TYPE = SHARE_TYPE(2u32);
pub const STYPE_IPC: SHARE_TYPE = SHARE_TYPE(3u32);
pub const STYPE_SPECIAL: SHARE_TYPE = SHARE_TYPE(2147483648u32);
pub const STYPE_TEMPORARY: SHARE_TYPE = SHARE_TYPE(1073741824u32);
pub const STYPE_MASK: SHARE_TYPE = SHARE_TYPE(255u32);
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
#[repr(C)]
pub struct STAT_SERVER_0(i32);
#[repr(C)]
pub struct STAT_WORKSTATION_0(i32);
#[repr(transparent)]
pub struct STORAGE_BUS_TYPE(pub i32);
pub const BusTypeUnknown: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(0i32);
pub const BusTypeScsi: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(1i32);
pub const BusTypeAtapi: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(2i32);
pub const BusTypeAta: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(3i32);
pub const BusType1394: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(4i32);
pub const BusTypeSsa: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(5i32);
pub const BusTypeFibre: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(6i32);
pub const BusTypeUsb: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(7i32);
pub const BusTypeRAID: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(8i32);
pub const BusTypeiScsi: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(9i32);
pub const BusTypeSas: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(10i32);
pub const BusTypeSata: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(11i32);
pub const BusTypeSd: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(12i32);
pub const BusTypeMmc: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(13i32);
pub const BusTypeVirtual: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(14i32);
pub const BusTypeFileBackedVirtual: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(15i32);
pub const BusTypeSpaces: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(16i32);
pub const BusTypeNvme: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(17i32);
pub const BusTypeSCM: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(18i32);
pub const BusTypeUfs: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(19i32);
pub const BusTypeMax: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(20i32);
pub const BusTypeMaxReserved: STORAGE_BUS_TYPE = STORAGE_BUS_TYPE(127i32);
#[repr(transparent)]
pub struct STREAM_INFO_LEVELS(pub i32);
pub const FindStreamInfoStandard: STREAM_INFO_LEVELS = STREAM_INFO_LEVELS(0i32);
pub const FindStreamInfoMaxInfoLevel: STREAM_INFO_LEVELS = STREAM_INFO_LEVELS(1i32);
pub const STYPE_RESERVED1: u32 = 16777216u32;
pub const STYPE_RESERVED2: u32 = 33554432u32;
pub const STYPE_RESERVED3: u32 = 67108864u32;
pub const STYPE_RESERVED4: u32 = 134217728u32;
pub const STYPE_RESERVED5: u32 = 1048576u32;
pub const STYPE_RESERVED_ALL: u32 = 1073741568u32;
#[repr(transparent)]
pub struct SYMBOLIC_LINK_FLAGS(pub u32);
pub const SYMBOLIC_LINK_FLAG_DIRECTORY: SYMBOLIC_LINK_FLAGS = SYMBOLIC_LINK_FLAGS(1u32);
pub const SYMBOLIC_LINK_FLAG_ALLOW_UNPRIVILEGED_CREATE: SYMBOLIC_LINK_FLAGS = SYMBOLIC_LINK_FLAGS(2u32);
#[repr(transparent)]
pub struct TAPEMARK_TYPE(pub i32);
pub const TAPE_FILEMARKS: TAPEMARK_TYPE = TAPEMARK_TYPE(1i32);
pub const TAPE_LONG_FILEMARKS: TAPEMARK_TYPE = TAPEMARK_TYPE(3i32);
pub const TAPE_SETMARKS: TAPEMARK_TYPE = TAPEMARK_TYPE(0i32);
pub const TAPE_SHORT_FILEMARKS: TAPEMARK_TYPE = TAPEMARK_TYPE(2i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct TAPE_ERASE(i32);
#[repr(C)]
pub struct TAPE_GET_POSITION(i32);
#[repr(transparent)]
pub struct TAPE_INFORMATION_TYPE(pub u32);
pub const SET_TAPE_DRIVE_INFORMATION: TAPE_INFORMATION_TYPE = TAPE_INFORMATION_TYPE(1u32);
pub const SET_TAPE_MEDIA_INFORMATION: TAPE_INFORMATION_TYPE = TAPE_INFORMATION_TYPE(0u32);
#[repr(transparent)]
pub struct TAPE_POSITION_METHOD(pub i32);
pub const TAPE_ABSOLUTE_BLOCK: TAPE_POSITION_METHOD = TAPE_POSITION_METHOD(1i32);
pub const TAPE_LOGICAL_BLOCK: TAPE_POSITION_METHOD = TAPE_POSITION_METHOD(2i32);
pub const TAPE_REWIND: TAPE_POSITION_METHOD = TAPE_POSITION_METHOD(0i32);
pub const TAPE_SPACE_END_OF_DATA: TAPE_POSITION_METHOD = TAPE_POSITION_METHOD(4i32);
pub const TAPE_SPACE_FILEMARKS: TAPE_POSITION_METHOD = TAPE_POSITION_METHOD(6i32);
pub const TAPE_SPACE_RELATIVE_BLOCKS: TAPE_POSITION_METHOD = TAPE_POSITION_METHOD(5i32);
pub const TAPE_SPACE_SEQUENTIAL_FMKS: TAPE_POSITION_METHOD = TAPE_POSITION_METHOD(7i32);
pub const TAPE_SPACE_SEQUENTIAL_SMKS: TAPE_POSITION_METHOD = TAPE_POSITION_METHOD(9i32);
pub const TAPE_SPACE_SETMARKS: TAPE_POSITION_METHOD = TAPE_POSITION_METHOD(8i32);
#[repr(transparent)]
pub struct TAPE_POSITION_TYPE(pub i32);
pub const TAPE_ABSOLUTE_POSITION: TAPE_POSITION_TYPE = TAPE_POSITION_TYPE(0i32);
pub const TAPE_LOGICAL_POSITION: TAPE_POSITION_TYPE = TAPE_POSITION_TYPE(1i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct TAPE_PREPARE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct TAPE_SET_POSITION(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
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
#[repr(C)]
pub struct TRANSACTION_NOTIFICATION(i32);
#[repr(C)]
pub struct TRANSACTION_NOTIFICATION_MARSHAL_ARGUMENT(i32);
#[repr(C)]
pub struct TRANSACTION_NOTIFICATION_PROPAGATE_ARGUMENT(i32);
#[repr(C)]
pub struct TRANSACTION_NOTIFICATION_RECOVERY_ARGUMENT(i32);
#[repr(C)]
pub struct TRANSACTION_NOTIFICATION_SAVEPOINT_ARGUMENT(i32);
#[repr(C)]
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
#[repr(transparent)]
pub struct TRANSACTION_OUTCOME(pub i32);
pub const TransactionOutcomeUndetermined: TRANSACTION_OUTCOME = TRANSACTION_OUTCOME(1i32);
pub const TransactionOutcomeCommitted: TRANSACTION_OUTCOME = TRANSACTION_OUTCOME(2i32);
pub const TransactionOutcomeAborted: TRANSACTION_OUTCOME = TRANSACTION_OUTCOME(3i32);
#[repr(transparent)]
pub struct TXFS_MINIVERSION(pub u32);
pub const TXFS_MINIVERSION_COMMITTED_VIEW: TXFS_MINIVERSION = TXFS_MINIVERSION(0u32);
pub const TXFS_MINIVERSION_DIRTY_VIEW: TXFS_MINIVERSION = TXFS_MINIVERSION(65535u32);
pub const TXFS_MINIVERSION_DEFAULT_VIEW: TXFS_MINIVERSION = TXFS_MINIVERSION(65534u32);
#[repr(C)]
pub struct TXF_ID(i32);
#[repr(C)]
pub struct TXF_LOG_RECORD_AFFECTED_FILE(i32);
#[repr(C)]
pub struct TXF_LOG_RECORD_BASE(i32);
pub const TXF_LOG_RECORD_GENERIC_TYPE_ABORT: u32 = 2u32;
pub const TXF_LOG_RECORD_GENERIC_TYPE_COMMIT: u32 = 1u32;
pub const TXF_LOG_RECORD_GENERIC_TYPE_DATA: u32 = 8u32;
pub const TXF_LOG_RECORD_GENERIC_TYPE_PREPARE: u32 = 4u32;
#[repr(C)]
pub struct TXF_LOG_RECORD_TRUNCATE(i32);
#[repr(transparent)]
pub struct TXF_LOG_RECORD_TYPE(pub u16);
pub const TXF_LOG_RECORD_TYPE_AFFECTED_FILE: TXF_LOG_RECORD_TYPE = TXF_LOG_RECORD_TYPE(4u16);
pub const TXF_LOG_RECORD_TYPE_TRUNCATE: TXF_LOG_RECORD_TYPE = TXF_LOG_RECORD_TYPE(2u16);
pub const TXF_LOG_RECORD_TYPE_WRITE: TXF_LOG_RECORD_TYPE = TXF_LOG_RECORD_TYPE(1u16);
#[repr(C)]
pub struct TXF_LOG_RECORD_WRITE(i32);
#[repr(transparent)]
pub struct VER_FIND_FILE_FLAGS(pub u32);
pub const VFFF_ISSHAREDFILE: VER_FIND_FILE_FLAGS = VER_FIND_FILE_FLAGS(1u32);
#[repr(transparent)]
pub struct VER_FIND_FILE_STATUS(pub u32);
pub const VFF_CURNEDEST: VER_FIND_FILE_STATUS = VER_FIND_FILE_STATUS(1u32);
pub const VFF_FILEINUSE: VER_FIND_FILE_STATUS = VER_FIND_FILE_STATUS(2u32);
pub const VFF_BUFFTOOSMALL: VER_FIND_FILE_STATUS = VER_FIND_FILE_STATUS(4u32);
#[repr(transparent)]
pub struct VER_INSTALL_FILE_FLAGS(pub u32);
pub const VIFF_FORCEINSTALL: VER_INSTALL_FILE_FLAGS = VER_INSTALL_FILE_FLAGS(1u32);
pub const VIFF_DONTDELETEOLD: VER_INSTALL_FILE_FLAGS = VER_INSTALL_FILE_FLAGS(2u32);
#[repr(transparent)]
pub struct VER_INSTALL_FILE_STATUS(pub u32);
pub const VIF_TEMPFILE: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(1u32);
pub const VIF_MISMATCH: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(2u32);
pub const VIF_SRCOLD: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(4u32);
pub const VIF_DIFFLANG: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(8u32);
pub const VIF_DIFFCODEPG: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(16u32);
pub const VIF_DIFFTYPE: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(32u32);
pub const VIF_WRITEPROT: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(64u32);
pub const VIF_FILEINUSE: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(128u32);
pub const VIF_OUTOFSPACE: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(256u32);
pub const VIF_ACCESSVIOLATION: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(512u32);
pub const VIF_SHARINGVIOLATION: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(1024u32);
pub const VIF_CANNOTCREATE: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(2048u32);
pub const VIF_CANNOTDELETE: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(4096u32);
pub const VIF_CANNOTRENAME: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(8192u32);
pub const VIF_CANNOTDELETECUR: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(16384u32);
pub const VIF_OUTOFMEMORY: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(32768u32);
pub const VIF_CANNOTREADSRC: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(65536u32);
pub const VIF_CANNOTREADDST: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(131072u32);
pub const VIF_BUFFTOOSMALL: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(262144u32);
pub const VIF_CANNOTLOADLZ32: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(524288u32);
pub const VIF_CANNOTLOADCABINET: VER_INSTALL_FILE_STATUS = VER_INSTALL_FILE_STATUS(1048576u32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct VOLUME_ALLOCATE_BC_STREAM_INPUT(i32);
#[repr(C)]
pub struct VOLUME_ALLOCATE_BC_STREAM_OUTPUT(i32);
#[repr(C)]
pub struct VOLUME_ALLOCATION_HINT_INPUT(i32);
#[repr(C)]
pub struct VOLUME_ALLOCATION_HINT_OUTPUT(i32);
#[repr(C)]
pub struct VOLUME_CRITICAL_IO(i32);
#[repr(C)]
pub struct VOLUME_FAILOVER_SET(i32);
#[repr(C)]
pub struct VOLUME_GET_BC_PROPERTIES_INPUT(i32);
#[repr(C)]
pub struct VOLUME_GET_BC_PROPERTIES_OUTPUT(i32);
#[repr(C)]
pub struct VOLUME_LOGICAL_OFFSET(i32);
#[repr(C)]
pub struct VOLUME_NUMBER(i32);
#[repr(C)]
pub struct VOLUME_PHYSICAL_OFFSET(i32);
#[repr(C)]
pub struct VOLUME_PHYSICAL_OFFSETS(i32);
#[repr(C)]
pub struct VOLUME_READ_PLEX_INPUT(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct VOLUME_SET_GPT_ATTRIBUTES_INFORMATION(i32);
#[repr(C)]
pub struct VOLUME_SHRINK_INFO(i32);
pub const VS_FFI_FILEFLAGSMASK: i32 = 63i32;
pub const VS_FFI_SIGNATURE: i32 = -17890115i32;
pub const VS_FFI_STRUCVERSION: i32 = 65536i32;
#[repr(C)]
pub struct VS_FIXEDFILEINFO(i32);
#[repr(transparent)]
pub struct VS_FIXEDFILEINFO_FILE_FLAGS(pub u32);
pub const VS_FF_DEBUG: VS_FIXEDFILEINFO_FILE_FLAGS = VS_FIXEDFILEINFO_FILE_FLAGS(1u32);
pub const VS_FF_PRERELEASE: VS_FIXEDFILEINFO_FILE_FLAGS = VS_FIXEDFILEINFO_FILE_FLAGS(2u32);
pub const VS_FF_PATCHED: VS_FIXEDFILEINFO_FILE_FLAGS = VS_FIXEDFILEINFO_FILE_FLAGS(4u32);
pub const VS_FF_PRIVATEBUILD: VS_FIXEDFILEINFO_FILE_FLAGS = VS_FIXEDFILEINFO_FILE_FLAGS(8u32);
pub const VS_FF_INFOINFERRED: VS_FIXEDFILEINFO_FILE_FLAGS = VS_FIXEDFILEINFO_FILE_FLAGS(16u32);
pub const VS_FF_SPECIALBUILD: VS_FIXEDFILEINFO_FILE_FLAGS = VS_FIXEDFILEINFO_FILE_FLAGS(32u32);
#[repr(transparent)]
pub struct VS_FIXEDFILEINFO_FILE_OS(pub i32);
pub const VOS_UNKNOWN: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(0i32);
pub const VOS_DOS: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(65536i32);
pub const VOS_OS216: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(131072i32);
pub const VOS_OS232: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(196608i32);
pub const VOS_NT: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(262144i32);
pub const VOS_WINCE: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(327680i32);
pub const VOS__BASE: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(0i32);
pub const VOS__WINDOWS16: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(1i32);
pub const VOS__PM16: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(2i32);
pub const VOS__PM32: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(3i32);
pub const VOS__WINDOWS32: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(4i32);
pub const VOS_DOS_WINDOWS16: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(65537i32);
pub const VOS_DOS_WINDOWS32: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(65540i32);
pub const VOS_OS216_PM16: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(131074i32);
pub const VOS_OS232_PM32: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(196611i32);
pub const VOS_NT_WINDOWS32: VS_FIXEDFILEINFO_FILE_OS = VS_FIXEDFILEINFO_FILE_OS(262148i32);
#[repr(transparent)]
pub struct VS_FIXEDFILEINFO_FILE_SUBTYPE(pub i32);
pub const VFT2_UNKNOWN: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(0i32);
pub const VFT2_DRV_PRINTER: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(1i32);
pub const VFT2_DRV_KEYBOARD: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(2i32);
pub const VFT2_DRV_LANGUAGE: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(3i32);
pub const VFT2_DRV_DISPLAY: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(4i32);
pub const VFT2_DRV_MOUSE: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(5i32);
pub const VFT2_DRV_NETWORK: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(6i32);
pub const VFT2_DRV_SYSTEM: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(7i32);
pub const VFT2_DRV_INSTALLABLE: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(8i32);
pub const VFT2_DRV_SOUND: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(9i32);
pub const VFT2_DRV_COMM: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(10i32);
pub const VFT2_DRV_INPUTMETHOD: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(11i32);
pub const VFT2_DRV_VERSIONED_PRINTER: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(12i32);
pub const VFT2_FONT_RASTER: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(1i32);
pub const VFT2_FONT_VECTOR: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(2i32);
pub const VFT2_FONT_TRUETYPE: VS_FIXEDFILEINFO_FILE_SUBTYPE = VS_FIXEDFILEINFO_FILE_SUBTYPE(3i32);
#[repr(transparent)]
pub struct VS_FIXEDFILEINFO_FILE_TYPE(pub i32);
pub const VFT_UNKNOWN: VS_FIXEDFILEINFO_FILE_TYPE = VS_FIXEDFILEINFO_FILE_TYPE(0i32);
pub const VFT_APP: VS_FIXEDFILEINFO_FILE_TYPE = VS_FIXEDFILEINFO_FILE_TYPE(1i32);
pub const VFT_DLL: VS_FIXEDFILEINFO_FILE_TYPE = VS_FIXEDFILEINFO_FILE_TYPE(2i32);
pub const VFT_DRV: VS_FIXEDFILEINFO_FILE_TYPE = VS_FIXEDFILEINFO_FILE_TYPE(3i32);
pub const VFT_FONT: VS_FIXEDFILEINFO_FILE_TYPE = VS_FIXEDFILEINFO_FILE_TYPE(4i32);
pub const VFT_VXD: VS_FIXEDFILEINFO_FILE_TYPE = VS_FIXEDFILEINFO_FILE_TYPE(5i32);
pub const VFT_STATIC_LIB: VS_FIXEDFILEINFO_FILE_TYPE = VS_FIXEDFILEINFO_FILE_TYPE(7i32);
pub const VS_USER_DEFINED: u32 = 100u32;
pub const VS_VERSION_INFO: u32 = 1u32;
pub const WIM_BOOT_NOT_OS_WIM: u32 = 0u32;
pub const WIM_BOOT_OS_WIM: u32 = 1u32;
pub const WIM_ENTRY_FLAG_NOT_ACTIVE: u32 = 1u32;
pub const WIM_ENTRY_FLAG_SUSPENDED: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WIM_ENTRY_INFO(i32);
#[repr(C)]
pub struct WIM_EXTERNAL_FILE_INFO(i32);
pub const WIM_EXTERNAL_FILE_INFO_FLAG_NOT_ACTIVE: u32 = 1u32;
pub const WIM_EXTERNAL_FILE_INFO_FLAG_SUSPENDED: u32 = 2u32;
pub const WIM_PROVIDER_HASH_SIZE: u32 = 20u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WIN32_FILE_ATTRIBUTE_DATA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WIN32_FIND_DATAA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WIN32_FIND_DATAW(i32);
#[repr(C)]
pub struct WIN32_FIND_STREAM_DATA(i32);
#[repr(C)]
pub struct WIN32_STREAM_ID(i32);
pub const WINEFS_SETUSERKEY_SET_CAPABILITIES: u32 = 1u32;
#[repr(transparent)]
pub struct WIN_STREAM_ID(pub u32);
pub const BACKUP_ALTERNATE_DATA: WIN_STREAM_ID = WIN_STREAM_ID(4u32);
pub const BACKUP_DATA: WIN_STREAM_ID = WIN_STREAM_ID(1u32);
pub const BACKUP_EA_DATA: WIN_STREAM_ID = WIN_STREAM_ID(2u32);
pub const BACKUP_LINK: WIN_STREAM_ID = WIN_STREAM_ID(5u32);
pub const BACKUP_OBJECT_ID: WIN_STREAM_ID = WIN_STREAM_ID(7u32);
pub const BACKUP_PROPERTY_DATA: WIN_STREAM_ID = WIN_STREAM_ID(6u32);
pub const BACKUP_REPARSE_DATA: WIN_STREAM_ID = WIN_STREAM_ID(8u32);
pub const BACKUP_SECURITY_DATA: WIN_STREAM_ID = WIN_STREAM_ID(3u32);
pub const BACKUP_SPARSE_BLOCK: WIN_STREAM_ID = WIN_STREAM_ID(9u32);
pub const BACKUP_TXFS_DATA: WIN_STREAM_ID = WIN_STREAM_ID(10u32);
#[repr(C)]
pub struct WOF_FILE_COMPRESSION_INFO_V0(i32);
#[repr(C)]
pub struct WOF_FILE_COMPRESSION_INFO_V1(i32);
pub const WOF_PROVIDER_FILE: u32 = 2u32;
pub const WOF_PROVIDER_WIM: u32 = 1u32;
#[repr(C)]
pub struct WofEnumEntryProc(i32);
#[repr(C)]
pub struct WofEnumFilesProc(i32);
pub const _FT_TYPES_DEFINITION_: u32 = 1u32;
