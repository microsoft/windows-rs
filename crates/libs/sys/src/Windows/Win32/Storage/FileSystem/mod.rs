#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddLogContainer(hlog: super::super::Foundation::HANDLE, pcbcontainer: *const u64, pwszcontainerpath: ::windows_sys::core::PCWSTR, preserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddLogContainerSet(hlog: super::super::Foundation::HANDLE, ccontainer: u16, pcbcontainer: *const u64, rgwszcontainerpath: *const ::windows_sys::core::PWSTR, preserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Security\"`*"]
    #[cfg(feature = "Win32_Security")]
    pub fn AddUsersToEncryptedFile(lpfilename: ::windows_sys::core::PCWSTR, pencryptioncertificates: *const ENCRYPTION_CERTIFICATE_LIST) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn AdvanceLogBase(pvmarshal: *mut ::core::ffi::c_void, plsnbase: *mut CLS_LSN, fflags: u32, poverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AlignReservedLog(pvmarshal: *mut ::core::ffi::c_void, creservedrecords: u32, rgcbreservation: *mut i64, pcbalignreservation: *mut i64) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AllocReservedLog(pvmarshal: *mut ::core::ffi::c_void, creservedrecords: u32, pcbadjustment: *mut i64) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AreFileApisANSI() -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AreShortNamesEnabled(handle: super::super::Foundation::HANDLE, enabled: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BackupRead(hfile: super::super::Foundation::HANDLE, lpbuffer: *mut u8, nnumberofbytestoread: u32, lpnumberofbytesread: *mut u32, babort: super::super::Foundation::BOOL, bprocesssecurity: super::super::Foundation::BOOL, lpcontext: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BackupSeek(hfile: super::super::Foundation::HANDLE, dwlowbytestoseek: u32, dwhighbytestoseek: u32, lpdwlowbyteseeked: *mut u32, lpdwhighbyteseeked: *mut u32, lpcontext: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BackupWrite(hfile: super::super::Foundation::HANDLE, lpbuffer: *const u8, nnumberofbytestowrite: u32, lpnumberofbyteswritten: *mut u32, babort: super::super::Foundation::BOOL, bprocesssecurity: super::super::Foundation::BOOL, lpcontext: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BuildIoRingCancelRequest(ioring: *const HIORING__, file: IORING_HANDLE_REF, optocancel: usize, userdata: usize) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BuildIoRingReadFile(ioring: *const HIORING__, fileref: IORING_HANDLE_REF, dataref: IORING_BUFFER_REF, numberofbytestoread: u32, fileoffset: u64, userdata: usize, flags: IORING_SQE_FLAGS) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn BuildIoRingRegisterBuffers(ioring: *const HIORING__, count: u32, buffers: *const IORING_BUFFER_INFO, userdata: usize) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BuildIoRingRegisterFileHandles(ioring: *const HIORING__, count: u32, handles: *const super::super::Foundation::HANDLE, userdata: usize) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CheckNameLegalDOS8Dot3A(lpname: ::windows_sys::core::PCSTR, lpoemname: ::windows_sys::core::PSTR, oemnamesize: u32, pbnamecontainsspaces: *mut super::super::Foundation::BOOL, pbnamelegal: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CheckNameLegalDOS8Dot3W(lpname: ::windows_sys::core::PCWSTR, lpoemname: ::windows_sys::core::PSTR, oemnamesize: u32, pbnamecontainsspaces: *mut super::super::Foundation::BOOL, pbnamelegal: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CloseAndResetLogFile(hlog: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn CloseEncryptedFileRaw(pvcontext: *const ::core::ffi::c_void);
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn CloseIoRing(ioring: *const HIORING__) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CommitComplete(enlistmenthandle: super::super::Foundation::HANDLE, tmvirtualclock: *mut i64) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CommitEnlistment(enlistmenthandle: super::super::Foundation::HANDLE, tmvirtualclock: *mut i64) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CommitTransaction(transactionhandle: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CommitTransactionAsync(transactionhandle: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CompareFileTime(lpfiletime1: *const super::super::Foundation::FILETIME, lpfiletime2: *const super::super::Foundation::FILETIME) -> i32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CopyFile2(pwszexistingfilename: ::windows_sys::core::PCWSTR, pwsznewfilename: ::windows_sys::core::PCWSTR, pextendedparameters: *const COPYFILE2_EXTENDED_PARAMETERS) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CopyFileA(lpexistingfilename: ::windows_sys::core::PCSTR, lpnewfilename: ::windows_sys::core::PCSTR, bfailifexists: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CopyFileExA(lpexistingfilename: ::windows_sys::core::PCSTR, lpnewfilename: ::windows_sys::core::PCSTR, lpprogressroutine: LPPROGRESS_ROUTINE, lpdata: *const ::core::ffi::c_void, pbcancel: *mut i32, dwcopyflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CopyFileExW(lpexistingfilename: ::windows_sys::core::PCWSTR, lpnewfilename: ::windows_sys::core::PCWSTR, lpprogressroutine: LPPROGRESS_ROUTINE, lpdata: *const ::core::ffi::c_void, pbcancel: *mut i32, dwcopyflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CopyFileFromAppW(lpexistingfilename: ::windows_sys::core::PCWSTR, lpnewfilename: ::windows_sys::core::PCWSTR, bfailifexists: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CopyFileTransactedA(lpexistingfilename: ::windows_sys::core::PCSTR, lpnewfilename: ::windows_sys::core::PCSTR, lpprogressroutine: LPPROGRESS_ROUTINE, lpdata: *const ::core::ffi::c_void, pbcancel: *const i32, dwcopyflags: u32, htransaction: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CopyFileTransactedW(lpexistingfilename: ::windows_sys::core::PCWSTR, lpnewfilename: ::windows_sys::core::PCWSTR, lpprogressroutine: LPPROGRESS_ROUTINE, lpdata: *const ::core::ffi::c_void, pbcancel: *const i32, dwcopyflags: u32, htransaction: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CopyFileW(lpexistingfilename: ::windows_sys::core::PCWSTR, lpnewfilename: ::windows_sys::core::PCWSTR, bfailifexists: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn CopyLZFile(hfsource: i32, hfdest: i32) -> i32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateDirectoryA(lppathname: ::windows_sys::core::PCSTR, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateDirectoryExA(lptemplatedirectory: ::windows_sys::core::PCSTR, lpnewdirectory: ::windows_sys::core::PCSTR, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateDirectoryExW(lptemplatedirectory: ::windows_sys::core::PCWSTR, lpnewdirectory: ::windows_sys::core::PCWSTR, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateDirectoryFromAppW(lppathname: ::windows_sys::core::PCWSTR, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateDirectoryTransactedA(lptemplatedirectory: ::windows_sys::core::PCSTR, lpnewdirectory: ::windows_sys::core::PCSTR, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, htransaction: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateDirectoryTransactedW(lptemplatedirectory: ::windows_sys::core::PCWSTR, lpnewdirectory: ::windows_sys::core::PCWSTR, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, htransaction: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateDirectoryW(lppathname: ::windows_sys::core::PCWSTR, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateEnlistment(lpenlistmentattributes: *mut super::super::Security::SECURITY_ATTRIBUTES, resourcemanagerhandle: super::super::Foundation::HANDLE, transactionhandle: super::super::Foundation::HANDLE, notificationmask: u32, createoptions: u32, enlistmentkey: *mut ::core::ffi::c_void) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateFile2(lpfilename: ::windows_sys::core::PCWSTR, dwdesiredaccess: FILE_ACCESS_FLAGS, dwsharemode: FILE_SHARE_MODE, dwcreationdisposition: FILE_CREATION_DISPOSITION, pcreateexparams: *const CREATEFILE2_EXTENDED_PARAMETERS) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateFile2FromAppW(lpfilename: ::windows_sys::core::PCWSTR, dwdesiredaccess: u32, dwsharemode: u32, dwcreationdisposition: u32, pcreateexparams: *const CREATEFILE2_EXTENDED_PARAMETERS) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateFileA(lpfilename: ::windows_sys::core::PCSTR, dwdesiredaccess: FILE_ACCESS_FLAGS, dwsharemode: FILE_SHARE_MODE, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, dwcreationdisposition: FILE_CREATION_DISPOSITION, dwflagsandattributes: FILE_FLAGS_AND_ATTRIBUTES, htemplatefile: super::super::Foundation::HANDLE) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateFileFromAppW(lpfilename: ::windows_sys::core::PCWSTR, dwdesiredaccess: u32, dwsharemode: u32, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, dwcreationdisposition: u32, dwflagsandattributes: u32, htemplatefile: super::super::Foundation::HANDLE) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateFileTransactedA(lpfilename: ::windows_sys::core::PCSTR, dwdesiredaccess: u32, dwsharemode: FILE_SHARE_MODE, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, dwcreationdisposition: FILE_CREATION_DISPOSITION, dwflagsandattributes: FILE_FLAGS_AND_ATTRIBUTES, htemplatefile: super::super::Foundation::HANDLE, htransaction: super::super::Foundation::HANDLE, pusminiversion: *const TXFS_MINIVERSION, lpextendedparameter: *mut ::core::ffi::c_void) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateFileTransactedW(lpfilename: ::windows_sys::core::PCWSTR, dwdesiredaccess: u32, dwsharemode: FILE_SHARE_MODE, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, dwcreationdisposition: FILE_CREATION_DISPOSITION, dwflagsandattributes: FILE_FLAGS_AND_ATTRIBUTES, htemplatefile: super::super::Foundation::HANDLE, htransaction: super::super::Foundation::HANDLE, pusminiversion: *const TXFS_MINIVERSION, lpextendedparameter: *mut ::core::ffi::c_void) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateFileW(lpfilename: ::windows_sys::core::PCWSTR, dwdesiredaccess: FILE_ACCESS_FLAGS, dwsharemode: FILE_SHARE_MODE, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, dwcreationdisposition: FILE_CREATION_DISPOSITION, dwflagsandattributes: FILE_FLAGS_AND_ATTRIBUTES, htemplatefile: super::super::Foundation::HANDLE) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateHardLinkA(lpfilename: ::windows_sys::core::PCSTR, lpexistingfilename: ::windows_sys::core::PCSTR, lpsecurityattributes: *mut super::super::Security::SECURITY_ATTRIBUTES) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateHardLinkTransactedA(lpfilename: ::windows_sys::core::PCSTR, lpexistingfilename: ::windows_sys::core::PCSTR, lpsecurityattributes: *mut super::super::Security::SECURITY_ATTRIBUTES, htransaction: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateHardLinkTransactedW(lpfilename: ::windows_sys::core::PCWSTR, lpexistingfilename: ::windows_sys::core::PCWSTR, lpsecurityattributes: *mut super::super::Security::SECURITY_ATTRIBUTES, htransaction: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateHardLinkW(lpfilename: ::windows_sys::core::PCWSTR, lpexistingfilename: ::windows_sys::core::PCWSTR, lpsecurityattributes: *mut super::super::Security::SECURITY_ATTRIBUTES) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn CreateIoRing(ioringversion: IORING_VERSION, flags: IORING_CREATE_FLAGS, submissionqueuesize: u32, completionqueuesize: u32, h: *mut *mut HIORING__) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn CreateLogContainerScanContext(hlog: super::super::Foundation::HANDLE, cfromcontainer: u32, ccontainers: u32, escanmode: u8, pcxscan: *mut CLS_SCAN_CONTEXT, poverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateLogFile(pszlogfilename: ::windows_sys::core::PCWSTR, fdesiredaccess: FILE_ACCESS_FLAGS, dwsharemode: FILE_SHARE_MODE, psalogfile: *mut super::super::Security::SECURITY_ATTRIBUTES, fcreatedisposition: FILE_CREATION_DISPOSITION, fflagsandattributes: FILE_FLAGS_AND_ATTRIBUTES) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateLogMarshallingArea(hlog: super::super::Foundation::HANDLE, pfnallocbuffer: CLFS_BLOCK_ALLOCATION, pfnfreebuffer: CLFS_BLOCK_DEALLOCATION, pvblockalloccontext: *mut ::core::ffi::c_void, cbmarshallingbuffer: u32, cmaxwritebuffers: u32, cmaxreadbuffers: u32, ppvmarshal: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateResourceManager(lpresourcemanagerattributes: *mut super::super::Security::SECURITY_ATTRIBUTES, resourcemanagerid: *mut ::windows_sys::core::GUID, createoptions: u32, tmhandle: super::super::Foundation::HANDLE, description: ::windows_sys::core::PCWSTR) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateSymbolicLinkA(lpsymlinkfilename: ::windows_sys::core::PCSTR, lptargetfilename: ::windows_sys::core::PCSTR, dwflags: SYMBOLIC_LINK_FLAGS) -> super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateSymbolicLinkTransactedA(lpsymlinkfilename: ::windows_sys::core::PCSTR, lptargetfilename: ::windows_sys::core::PCSTR, dwflags: SYMBOLIC_LINK_FLAGS, htransaction: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateSymbolicLinkTransactedW(lpsymlinkfilename: ::windows_sys::core::PCWSTR, lptargetfilename: ::windows_sys::core::PCWSTR, dwflags: SYMBOLIC_LINK_FLAGS, htransaction: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateSymbolicLinkW(lpsymlinkfilename: ::windows_sys::core::PCWSTR, lptargetfilename: ::windows_sys::core::PCWSTR, dwflags: SYMBOLIC_LINK_FLAGS) -> super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateTapePartition(hdevice: super::super::Foundation::HANDLE, dwpartitionmethod: CREATE_TAPE_PARTITION_METHOD, dwcount: u32, dwsize: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateTransaction(lptransactionattributes: *mut super::super::Security::SECURITY_ATTRIBUTES, uow: *mut ::windows_sys::core::GUID, createoptions: u32, isolationlevel: u32, isolationflags: u32, timeout: u32, description: ::windows_sys::core::PCWSTR) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateTransactionManager(lptransactionattributes: *mut super::super::Security::SECURITY_ATTRIBUTES, logfilename: ::windows_sys::core::PCWSTR, createoptions: u32, commitstrength: u32) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DecryptFileA(lpfilename: ::windows_sys::core::PCSTR, dwreserved: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DecryptFileW(lpfilename: ::windows_sys::core::PCWSTR, dwreserved: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DefineDosDeviceA(dwflags: DEFINE_DOS_DEVICE_FLAGS, lpdevicename: ::windows_sys::core::PCSTR, lptargetpath: ::windows_sys::core::PCSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DefineDosDeviceW(dwflags: DEFINE_DOS_DEVICE_FLAGS, lpdevicename: ::windows_sys::core::PCWSTR, lptargetpath: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteFileA(lpfilename: ::windows_sys::core::PCSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteFileFromAppW(lpfilename: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteFileTransactedA(lpfilename: ::windows_sys::core::PCSTR, htransaction: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteFileTransactedW(lpfilename: ::windows_sys::core::PCWSTR, htransaction: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteFileW(lpfilename: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteLogByHandle(hlog: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteLogFile(pszlogfilename: ::windows_sys::core::PCWSTR, pvreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteLogMarshallingArea(pvmarshal: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteVolumeMountPointA(lpszvolumemountpoint: ::windows_sys::core::PCSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteVolumeMountPointW(lpszvolumemountpoint: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeregisterManageableLogClient(hlog: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn DuplicateEncryptionInfoFile(srcfilename: ::windows_sys::core::PCWSTR, dstfilename: ::windows_sys::core::PCWSTR, dwcreationdistribution: u32, dwattributes: u32, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EncryptFileA(lpfilename: ::windows_sys::core::PCSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EncryptFileW(lpfilename: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EncryptionDisable(dirpath: ::windows_sys::core::PCWSTR, disable: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EraseTape(hdevice: super::super::Foundation::HANDLE, dwerasetype: ERASE_TAPE_TYPE, bimmediate: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FileEncryptionStatusA(lpfilename: ::windows_sys::core::PCSTR, lpstatus: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FileEncryptionStatusW(lpfilename: ::windows_sys::core::PCWSTR, lpstatus: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FileTimeToLocalFileTime(lpfiletime: *const super::super::Foundation::FILETIME, lplocalfiletime: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindClose(hfindfile: FindFileHandle) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindCloseChangeNotification(hchangehandle: FindChangeNotificationHandle) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindFirstChangeNotificationA(lppathname: ::windows_sys::core::PCSTR, bwatchsubtree: super::super::Foundation::BOOL, dwnotifyfilter: FILE_NOTIFY_CHANGE) -> FindChangeNotificationHandle;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindFirstChangeNotificationW(lppathname: ::windows_sys::core::PCWSTR, bwatchsubtree: super::super::Foundation::BOOL, dwnotifyfilter: FILE_NOTIFY_CHANGE) -> FindChangeNotificationHandle;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindFirstFileA(lpfilename: ::windows_sys::core::PCSTR, lpfindfiledata: *mut WIN32_FIND_DATAA) -> FindFileHandle;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn FindFirstFileExA(lpfilename: ::windows_sys::core::PCSTR, finfolevelid: FINDEX_INFO_LEVELS, lpfindfiledata: *mut ::core::ffi::c_void, fsearchop: FINDEX_SEARCH_OPS, lpsearchfilter: *mut ::core::ffi::c_void, dwadditionalflags: FIND_FIRST_EX_FLAGS) -> FindFileHandle;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindFirstFileExFromAppW(lpfilename: ::windows_sys::core::PCWSTR, finfolevelid: FINDEX_INFO_LEVELS, lpfindfiledata: *mut ::core::ffi::c_void, fsearchop: FINDEX_SEARCH_OPS, lpsearchfilter: *mut ::core::ffi::c_void, dwadditionalflags: u32) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn FindFirstFileExW(lpfilename: ::windows_sys::core::PCWSTR, finfolevelid: FINDEX_INFO_LEVELS, lpfindfiledata: *mut ::core::ffi::c_void, fsearchop: FINDEX_SEARCH_OPS, lpsearchfilter: *mut ::core::ffi::c_void, dwadditionalflags: FIND_FIRST_EX_FLAGS) -> FindFileHandle;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindFirstFileNameTransactedW(lpfilename: ::windows_sys::core::PCWSTR, dwflags: u32, stringlength: *mut u32, linkname: ::windows_sys::core::PWSTR, htransaction: super::super::Foundation::HANDLE) -> FindFileNameHandle;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn FindFirstFileNameW(lpfilename: ::windows_sys::core::PCWSTR, dwflags: u32, stringlength: *mut u32, linkname: ::windows_sys::core::PWSTR) -> FindFileNameHandle;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindFirstFileTransactedA(lpfilename: ::windows_sys::core::PCSTR, finfolevelid: FINDEX_INFO_LEVELS, lpfindfiledata: *mut ::core::ffi::c_void, fsearchop: FINDEX_SEARCH_OPS, lpsearchfilter: *mut ::core::ffi::c_void, dwadditionalflags: u32, htransaction: super::super::Foundation::HANDLE) -> FindFileHandle;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindFirstFileTransactedW(lpfilename: ::windows_sys::core::PCWSTR, finfolevelid: FINDEX_INFO_LEVELS, lpfindfiledata: *mut ::core::ffi::c_void, fsearchop: FINDEX_SEARCH_OPS, lpsearchfilter: *mut ::core::ffi::c_void, dwadditionalflags: u32, htransaction: super::super::Foundation::HANDLE) -> FindFileHandle;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindFirstFileW(lpfilename: ::windows_sys::core::PCWSTR, lpfindfiledata: *mut WIN32_FIND_DATAW) -> FindFileHandle;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindFirstStreamTransactedW(lpfilename: ::windows_sys::core::PCWSTR, infolevel: STREAM_INFO_LEVELS, lpfindstreamdata: *mut ::core::ffi::c_void, dwflags: u32, htransaction: super::super::Foundation::HANDLE) -> FindStreamHandle;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn FindFirstStreamW(lpfilename: ::windows_sys::core::PCWSTR, infolevel: STREAM_INFO_LEVELS, lpfindstreamdata: *mut ::core::ffi::c_void, dwflags: u32) -> FindStreamHandle;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn FindFirstVolumeA(lpszvolumename: ::windows_sys::core::PSTR, cchbufferlength: u32) -> FindVolumeHandle;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn FindFirstVolumeMountPointA(lpszrootpathname: ::windows_sys::core::PCSTR, lpszvolumemountpoint: ::windows_sys::core::PSTR, cchbufferlength: u32) -> FindVolumeMointPointHandle;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn FindFirstVolumeMountPointW(lpszrootpathname: ::windows_sys::core::PCWSTR, lpszvolumemountpoint: ::windows_sys::core::PWSTR, cchbufferlength: u32) -> FindVolumeMointPointHandle;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn FindFirstVolumeW(lpszvolumename: ::windows_sys::core::PWSTR, cchbufferlength: u32) -> FindVolumeHandle;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindNextChangeNotification(hchangehandle: FindChangeNotificationHandle) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindNextFileA(hfindfile: FindFileHandle, lpfindfiledata: *mut WIN32_FIND_DATAA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindNextFileNameW(hfindstream: FindFileNameHandle, stringlength: *mut u32, linkname: ::windows_sys::core::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindNextFileW(hfindfile: super::super::Foundation::HANDLE, lpfindfiledata: *mut WIN32_FIND_DATAW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindNextStreamW(hfindstream: FindStreamHandle, lpfindstreamdata: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindNextVolumeA(hfindvolume: FindVolumeHandle, lpszvolumename: ::windows_sys::core::PSTR, cchbufferlength: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindNextVolumeMountPointA(hfindvolumemountpoint: FindVolumeMointPointHandle, lpszvolumemountpoint: ::windows_sys::core::PSTR, cchbufferlength: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindNextVolumeMountPointW(hfindvolumemountpoint: FindVolumeMointPointHandle, lpszvolumemountpoint: ::windows_sys::core::PWSTR, cchbufferlength: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindNextVolumeW(hfindvolume: FindVolumeHandle, lpszvolumename: ::windows_sys::core::PWSTR, cchbufferlength: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindVolumeClose(hfindvolume: FindVolumeHandle) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindVolumeMountPointClose(hfindvolumemountpoint: FindVolumeMointPointHandle) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FlushFileBuffers(hfile: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn FlushLogBuffers(pvmarshal: *mut ::core::ffi::c_void, poverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn FlushLogToLsn(pvmarshalcontext: *mut ::core::ffi::c_void, plsnflush: *mut CLS_LSN, plsnlastflushed: *mut CLS_LSN, poverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn FreeEncryptedFileMetadata(pbmetadata: *const u8);
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Security\"`*"]
    #[cfg(feature = "Win32_Security")]
    pub fn FreeEncryptionCertificateHashList(pusers: *const ENCRYPTION_CERTIFICATE_HASH_LIST);
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeReservedLog(pvmarshal: *mut ::core::ffi::c_void, creservedrecords: u32, pcbadjustment: *mut i64) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetBinaryTypeA(lpapplicationname: ::windows_sys::core::PCSTR, lpbinarytype: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetBinaryTypeW(lpapplicationname: ::windows_sys::core::PCWSTR, lpbinarytype: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn GetCompressedFileSizeA(lpfilename: ::windows_sys::core::PCSTR, lpfilesizehigh: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCompressedFileSizeTransactedA(lpfilename: ::windows_sys::core::PCSTR, lpfilesizehigh: *mut u32, htransaction: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCompressedFileSizeTransactedW(lpfilename: ::windows_sys::core::PCWSTR, lpfilesizehigh: *mut u32, htransaction: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn GetCompressedFileSizeW(lpfilename: ::windows_sys::core::PCWSTR, lpfilesizehigh: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrentClockTransactionManager(transactionmanagerhandle: super::super::Foundation::HANDLE, tmvirtualclock: *mut i64) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDiskFreeSpaceA(lprootpathname: ::windows_sys::core::PCSTR, lpsectorspercluster: *mut u32, lpbytespersector: *mut u32, lpnumberoffreeclusters: *mut u32, lptotalnumberofclusters: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDiskFreeSpaceExA(lpdirectoryname: ::windows_sys::core::PCSTR, lpfreebytesavailabletocaller: *mut u64, lptotalnumberofbytes: *mut u64, lptotalnumberoffreebytes: *mut u64) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDiskFreeSpaceExW(lpdirectoryname: ::windows_sys::core::PCWSTR, lpfreebytesavailabletocaller: *mut u64, lptotalnumberofbytes: *mut u64, lptotalnumberoffreebytes: *mut u64) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDiskFreeSpaceW(lprootpathname: ::windows_sys::core::PCWSTR, lpsectorspercluster: *mut u32, lpbytespersector: *mut u32, lpnumberoffreeclusters: *mut u32, lptotalnumberofclusters: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn GetDiskSpaceInformationA(rootpath: ::windows_sys::core::PCSTR, diskspaceinfo: *mut DISK_SPACE_INFORMATION) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn GetDiskSpaceInformationW(rootpath: ::windows_sys::core::PCWSTR, diskspaceinfo: *mut DISK_SPACE_INFORMATION) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn GetDriveTypeA(lprootpathname: ::windows_sys::core::PCSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn GetDriveTypeW(lprootpathname: ::windows_sys::core::PCWSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn GetEncryptedFileMetadata(lpfilename: ::windows_sys::core::PCWSTR, pcbmetadata: *mut u32, ppbmetadata: *mut *mut u8) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetEnlistmentId(enlistmenthandle: super::super::Foundation::HANDLE, enlistmentid: *mut ::windows_sys::core::GUID) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetEnlistmentRecoveryInformation(enlistmenthandle: super::super::Foundation::HANDLE, buffersize: u32, buffer: *mut ::core::ffi::c_void, bufferused: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn GetExpandedNameA(lpszsource: ::windows_sys::core::PCSTR, lpszbuffer: ::windows_sys::core::PSTR) -> i32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn GetExpandedNameW(lpszsource: ::windows_sys::core::PCWSTR, lpszbuffer: ::windows_sys::core::PWSTR) -> i32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn GetFileAttributesA(lpfilename: ::windows_sys::core::PCSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileAttributesExA(lpfilename: ::windows_sys::core::PCSTR, finfolevelid: GET_FILEEX_INFO_LEVELS, lpfileinformation: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileAttributesExFromAppW(lpfilename: ::windows_sys::core::PCWSTR, finfolevelid: GET_FILEEX_INFO_LEVELS, lpfileinformation: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileAttributesExW(lpfilename: ::windows_sys::core::PCWSTR, finfolevelid: GET_FILEEX_INFO_LEVELS, lpfileinformation: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileAttributesTransactedA(lpfilename: ::windows_sys::core::PCSTR, finfolevelid: GET_FILEEX_INFO_LEVELS, lpfileinformation: *mut ::core::ffi::c_void, htransaction: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileAttributesTransactedW(lpfilename: ::windows_sys::core::PCWSTR, finfolevelid: GET_FILEEX_INFO_LEVELS, lpfileinformation: *mut ::core::ffi::c_void, htransaction: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn GetFileAttributesW(lpfilename: ::windows_sys::core::PCWSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileBandwidthReservation(hfile: super::super::Foundation::HANDLE, lpperiodmilliseconds: *mut u32, lpbytesperperiod: *mut u32, pdiscardable: *mut i32, lptransfersize: *mut u32, lpnumoutstandingrequests: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileInformationByHandle(hfile: super::super::Foundation::HANDLE, lpfileinformation: *mut BY_HANDLE_FILE_INFORMATION) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileInformationByHandleEx(hfile: super::super::Foundation::HANDLE, fileinformationclass: FILE_INFO_BY_HANDLE_CLASS, lpfileinformation: *mut ::core::ffi::c_void, dwbuffersize: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileSize(hfile: super::super::Foundation::HANDLE, lpfilesizehigh: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileSizeEx(hfile: super::super::Foundation::HANDLE, lpfilesize: *mut i64) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileTime(hfile: super::super::Foundation::HANDLE, lpcreationtime: *mut super::super::Foundation::FILETIME, lplastaccesstime: *mut super::super::Foundation::FILETIME, lplastwritetime: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileType(hfile: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileVersionInfoA(lptstrfilename: ::windows_sys::core::PCSTR, dwhandle: u32, dwlen: u32, lpdata: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileVersionInfoExA(dwflags: GET_FILE_VERSION_INFO_FLAGS, lpwstrfilename: ::windows_sys::core::PCSTR, dwhandle: u32, dwlen: u32, lpdata: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileVersionInfoExW(dwflags: GET_FILE_VERSION_INFO_FLAGS, lpwstrfilename: ::windows_sys::core::PCWSTR, dwhandle: u32, dwlen: u32, lpdata: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn GetFileVersionInfoSizeA(lptstrfilename: ::windows_sys::core::PCSTR, lpdwhandle: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn GetFileVersionInfoSizeExA(dwflags: GET_FILE_VERSION_INFO_FLAGS, lpwstrfilename: ::windows_sys::core::PCSTR, lpdwhandle: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn GetFileVersionInfoSizeExW(dwflags: GET_FILE_VERSION_INFO_FLAGS, lpwstrfilename: ::windows_sys::core::PCWSTR, lpdwhandle: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn GetFileVersionInfoSizeW(lptstrfilename: ::windows_sys::core::PCWSTR, lpdwhandle: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileVersionInfoW(lptstrfilename: ::windows_sys::core::PCWSTR, dwhandle: u32, dwlen: u32, lpdata: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFinalPathNameByHandleA(hfile: super::super::Foundation::HANDLE, lpszfilepath: ::windows_sys::core::PSTR, cchfilepath: u32, dwflags: FILE_NAME) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFinalPathNameByHandleW(hfile: super::super::Foundation::HANDLE, lpszfilepath: ::windows_sys::core::PWSTR, cchfilepath: u32, dwflags: FILE_NAME) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn GetFullPathNameA(lpfilename: ::windows_sys::core::PCSTR, nbufferlength: u32, lpbuffer: ::windows_sys::core::PSTR, lpfilepart: *mut ::windows_sys::core::PSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFullPathNameTransactedA(lpfilename: ::windows_sys::core::PCSTR, nbufferlength: u32, lpbuffer: ::windows_sys::core::PSTR, lpfilepart: *mut ::windows_sys::core::PSTR, htransaction: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFullPathNameTransactedW(lpfilename: ::windows_sys::core::PCWSTR, nbufferlength: u32, lpbuffer: ::windows_sys::core::PWSTR, lpfilepart: *mut ::windows_sys::core::PWSTR, htransaction: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn GetFullPathNameW(lpfilename: ::windows_sys::core::PCWSTR, nbufferlength: u32, lpbuffer: ::windows_sys::core::PWSTR, lpfilepart: *mut ::windows_sys::core::PWSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn GetIoRingInfo(ioring: *const HIORING__, info: *mut IORING_INFO) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLogContainerName(hlog: super::super::Foundation::HANDLE, cidlogicalcontainer: u32, pwstrcontainername: ::windows_sys::core::PCWSTR, clencontainername: u32, pcactuallencontainername: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLogFileInformation(hlog: super::super::Foundation::HANDLE, pinfobuffer: *mut CLS_INFORMATION, cbbuffer: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLogIoStatistics(hlog: super::super::Foundation::HANDLE, pvstatsbuffer: *mut ::core::ffi::c_void, cbstatsbuffer: u32, estatsclass: CLFS_IOSTATS_CLASS, pcbstatswritten: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLogReservationInfo(pvmarshal: *const ::core::ffi::c_void, pcbrecordnumber: *mut u32, pcbuserreservation: *mut i64, pcbcommitreservation: *mut i64) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn GetLogicalDriveStringsA(nbufferlength: u32, lpbuffer: ::windows_sys::core::PSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn GetLogicalDriveStringsW(nbufferlength: u32, lpbuffer: ::windows_sys::core::PWSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn GetLogicalDrives() -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn GetLongPathNameA(lpszshortpath: ::windows_sys::core::PCSTR, lpszlongpath: ::windows_sys::core::PSTR, cchbuffer: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLongPathNameTransactedA(lpszshortpath: ::windows_sys::core::PCSTR, lpszlongpath: ::windows_sys::core::PSTR, cchbuffer: u32, htransaction: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLongPathNameTransactedW(lpszshortpath: ::windows_sys::core::PCWSTR, lpszlongpath: ::windows_sys::core::PWSTR, cchbuffer: u32, htransaction: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn GetLongPathNameW(lpszshortpath: ::windows_sys::core::PCWSTR, lpszlongpath: ::windows_sys::core::PWSTR, cchbuffer: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNextLogArchiveExtent(pvarchivecontext: *mut ::core::ffi::c_void, rgadextent: *mut CLS_ARCHIVE_DESCRIPTOR, cdescriptors: u32, pcdescriptorsreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNotificationResourceManager(resourcemanagerhandle: super::super::Foundation::HANDLE, transactionnotification: *mut TRANSACTION_NOTIFICATION, notificationlength: u32, dwmilliseconds: u32, returnlength: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn GetNotificationResourceManagerAsync(resourcemanagerhandle: super::super::Foundation::HANDLE, transactionnotification: *mut TRANSACTION_NOTIFICATION, transactionnotificationlength: u32, returnlength: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn GetShortPathNameA(lpszlongpath: ::windows_sys::core::PCSTR, lpszshortpath: ::windows_sys::core::PSTR, cchbuffer: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn GetShortPathNameW(lpszlongpath: ::windows_sys::core::PCWSTR, lpszshortpath: ::windows_sys::core::PWSTR, cchbuffer: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTapeParameters(hdevice: super::super::Foundation::HANDLE, dwoperation: GET_TAPE_DRIVE_PARAMETERS_OPERATION, lpdwsize: *mut u32, lptapeinformation: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTapePosition(hdevice: super::super::Foundation::HANDLE, dwpositiontype: TAPE_POSITION_TYPE, lpdwpartition: *mut u32, lpdwoffsetlow: *mut u32, lpdwoffsethigh: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTapeStatus(hdevice: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn GetTempFileNameA(lppathname: ::windows_sys::core::PCSTR, lpprefixstring: ::windows_sys::core::PCSTR, uunique: u32, lptempfilename: ::windows_sys::core::PSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn GetTempFileNameW(lppathname: ::windows_sys::core::PCWSTR, lpprefixstring: ::windows_sys::core::PCWSTR, uunique: u32, lptempfilename: ::windows_sys::core::PWSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn GetTempPath2A(bufferlength: u32, buffer: ::windows_sys::core::PSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn GetTempPath2W(bufferlength: u32, buffer: ::windows_sys::core::PWSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn GetTempPathA(nbufferlength: u32, lpbuffer: ::windows_sys::core::PSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn GetTempPathW(nbufferlength: u32, lpbuffer: ::windows_sys::core::PWSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTransactionId(transactionhandle: super::super::Foundation::HANDLE, transactionid: *mut ::windows_sys::core::GUID) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTransactionInformation(transactionhandle: super::super::Foundation::HANDLE, outcome: *mut u32, isolationlevel: *mut u32, isolationflags: *mut u32, timeout: *mut u32, bufferlength: u32, description: ::windows_sys::core::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTransactionManagerId(transactionmanagerhandle: super::super::Foundation::HANDLE, transactionmanagerid: *mut ::windows_sys::core::GUID) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetVolumeInformationA(lprootpathname: ::windows_sys::core::PCSTR, lpvolumenamebuffer: ::windows_sys::core::PSTR, nvolumenamesize: u32, lpvolumeserialnumber: *mut u32, lpmaximumcomponentlength: *mut u32, lpfilesystemflags: *mut u32, lpfilesystemnamebuffer: ::windows_sys::core::PSTR, nfilesystemnamesize: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetVolumeInformationByHandleW(hfile: super::super::Foundation::HANDLE, lpvolumenamebuffer: ::windows_sys::core::PWSTR, nvolumenamesize: u32, lpvolumeserialnumber: *mut u32, lpmaximumcomponentlength: *mut u32, lpfilesystemflags: *mut u32, lpfilesystemnamebuffer: ::windows_sys::core::PWSTR, nfilesystemnamesize: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetVolumeInformationW(lprootpathname: ::windows_sys::core::PCWSTR, lpvolumenamebuffer: ::windows_sys::core::PWSTR, nvolumenamesize: u32, lpvolumeserialnumber: *mut u32, lpmaximumcomponentlength: *mut u32, lpfilesystemflags: *mut u32, lpfilesystemnamebuffer: ::windows_sys::core::PWSTR, nfilesystemnamesize: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetVolumeNameForVolumeMountPointA(lpszvolumemountpoint: ::windows_sys::core::PCSTR, lpszvolumename: ::windows_sys::core::PSTR, cchbufferlength: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetVolumeNameForVolumeMountPointW(lpszvolumemountpoint: ::windows_sys::core::PCWSTR, lpszvolumename: ::windows_sys::core::PWSTR, cchbufferlength: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetVolumePathNameA(lpszfilename: ::windows_sys::core::PCSTR, lpszvolumepathname: ::windows_sys::core::PSTR, cchbufferlength: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetVolumePathNameW(lpszfilename: ::windows_sys::core::PCWSTR, lpszvolumepathname: ::windows_sys::core::PWSTR, cchbufferlength: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetVolumePathNamesForVolumeNameA(lpszvolumename: ::windows_sys::core::PCSTR, lpszvolumepathnames: ::windows_sys::core::PSTR, cchbufferlength: u32, lpcchreturnlength: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetVolumePathNamesForVolumeNameW(lpszvolumename: ::windows_sys::core::PCWSTR, lpszvolumepathnames: ::windows_sys::core::PWSTR, cchbufferlength: u32, lpcchreturnlength: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HandleLogFull(hlog: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InstallLogPolicy(hlog: super::super::Foundation::HANDLE, ppolicy: *mut CLFS_MGMT_POLICY) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsIoRingOpSupported(ioring: *const HIORING__, op: IORING_OP_CODE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn LZClose(hfile: i32);
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn LZCopy(hfsource: i32, hfdest: i32) -> i32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn LZDone();
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn LZInit(hfsource: i32) -> i32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LZOpenFileA(lpfilename: ::windows_sys::core::PCSTR, lpreopenbuf: *mut OFSTRUCT, wstyle: LZOPENFILE_STYLE) -> i32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LZOpenFileW(lpfilename: ::windows_sys::core::PCWSTR, lpreopenbuf: *mut OFSTRUCT, wstyle: LZOPENFILE_STYLE) -> i32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn LZRead(hfile: i32, lpbuffer: ::windows_sys::core::PSTR, cbread: i32) -> i32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn LZSeek(hfile: i32, loffset: i32, iorigin: i32) -> i32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn LZStart() -> i32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LocalFileTimeToFileTime(lplocalfiletime: *const super::super::Foundation::FILETIME, lpfiletime: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LockFile(hfile: super::super::Foundation::HANDLE, dwfileoffsetlow: u32, dwfileoffsethigh: u32, nnumberofbytestolocklow: u32, nnumberofbytestolockhigh: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn LockFileEx(hfile: super::super::Foundation::HANDLE, dwflags: LOCK_FILE_FLAGS, dwreserved: u32, nnumberofbytestolocklow: u32, nnumberofbytestolockhigh: u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LogTailAdvanceFailure(hlog: super::super::Foundation::HANDLE, dwreason: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn LsnBlockOffset(plsn: *const CLS_LSN) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn LsnContainer(plsn: *const CLS_LSN) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn LsnCreate(cidcontainer: u32, offblock: u32, crecord: u32) -> CLS_LSN;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsnEqual(plsn1: *const CLS_LSN, plsn2: *const CLS_LSN) -> super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsnGreater(plsn1: *const CLS_LSN, plsn2: *const CLS_LSN) -> super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn LsnIncrement(plsn: *const CLS_LSN) -> CLS_LSN;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsnInvalid(plsn: *const CLS_LSN) -> super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsnLess(plsn1: *const CLS_LSN, plsn2: *const CLS_LSN) -> super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LsnNull(plsn: *const CLS_LSN) -> super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn LsnRecordSequence(plsn: *const CLS_LSN) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MoveFileA(lpexistingfilename: ::windows_sys::core::PCSTR, lpnewfilename: ::windows_sys::core::PCSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MoveFileExA(lpexistingfilename: ::windows_sys::core::PCSTR, lpnewfilename: ::windows_sys::core::PCSTR, dwflags: MOVE_FILE_FLAGS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MoveFileExW(lpexistingfilename: ::windows_sys::core::PCWSTR, lpnewfilename: ::windows_sys::core::PCWSTR, dwflags: MOVE_FILE_FLAGS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MoveFileFromAppW(lpexistingfilename: ::windows_sys::core::PCWSTR, lpnewfilename: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MoveFileTransactedA(lpexistingfilename: ::windows_sys::core::PCSTR, lpnewfilename: ::windows_sys::core::PCSTR, lpprogressroutine: LPPROGRESS_ROUTINE, lpdata: *const ::core::ffi::c_void, dwflags: MOVE_FILE_FLAGS, htransaction: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MoveFileTransactedW(lpexistingfilename: ::windows_sys::core::PCWSTR, lpnewfilename: ::windows_sys::core::PCWSTR, lpprogressroutine: LPPROGRESS_ROUTINE, lpdata: *const ::core::ffi::c_void, dwflags: MOVE_FILE_FLAGS, htransaction: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MoveFileW(lpexistingfilename: ::windows_sys::core::PCWSTR, lpnewfilename: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MoveFileWithProgressA(lpexistingfilename: ::windows_sys::core::PCSTR, lpnewfilename: ::windows_sys::core::PCSTR, lpprogressroutine: LPPROGRESS_ROUTINE, lpdata: *const ::core::ffi::c_void, dwflags: MOVE_FILE_FLAGS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MoveFileWithProgressW(lpexistingfilename: ::windows_sys::core::PCWSTR, lpnewfilename: ::windows_sys::core::PCWSTR, lpprogressroutine: LPPROGRESS_ROUTINE, lpdata: *const ::core::ffi::c_void, dwflags: MOVE_FILE_FLAGS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn NetConnectionEnum(servername: ::windows_sys::core::PCWSTR, qualifier: ::windows_sys::core::PCWSTR, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn NetFileClose(servername: ::windows_sys::core::PCWSTR, fileid: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn NetFileEnum(servername: ::windows_sys::core::PCWSTR, basepath: ::windows_sys::core::PCWSTR, username: ::windows_sys::core::PCWSTR, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: *mut usize) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn NetFileGetInfo(servername: ::windows_sys::core::PCWSTR, fileid: u32, level: u32, bufptr: *mut *mut u8) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn NetServerAliasAdd(servername: ::windows_sys::core::PCWSTR, level: u32, buf: *const u8) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn NetServerAliasDel(servername: ::windows_sys::core::PCWSTR, level: u32, buf: *const u8) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn NetServerAliasEnum(servername: ::windows_sys::core::PCWSTR, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resumehandle: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn NetSessionDel(servername: ::windows_sys::core::PCWSTR, uncclientname: ::windows_sys::core::PCWSTR, username: ::windows_sys::core::PCWSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn NetSessionEnum(servername: ::windows_sys::core::PCWSTR, uncclientname: ::windows_sys::core::PCWSTR, username: ::windows_sys::core::PCWSTR, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn NetSessionGetInfo(servername: ::windows_sys::core::PCWSTR, uncclientname: ::windows_sys::core::PCWSTR, username: ::windows_sys::core::PCWSTR, level: u32, bufptr: *mut *mut u8) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn NetShareAdd(servername: ::windows_sys::core::PCWSTR, level: u32, buf: *const u8, parm_err: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn NetShareCheck(servername: ::windows_sys::core::PCWSTR, device: ::windows_sys::core::PCWSTR, r#type: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn NetShareDel(servername: ::windows_sys::core::PCWSTR, netname: ::windows_sys::core::PCWSTR, reserved: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn NetShareDelEx(servername: ::windows_sys::core::PCWSTR, level: u32, buf: *const u8) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn NetShareDelSticky(servername: ::windows_sys::core::PCWSTR, netname: ::windows_sys::core::PCWSTR, reserved: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn NetShareEnum(servername: ::windows_sys::core::PCWSTR, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn NetShareEnumSticky(servername: ::windows_sys::core::PCWSTR, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn NetShareGetInfo(servername: ::windows_sys::core::PCWSTR, netname: ::windows_sys::core::PCWSTR, level: u32, bufptr: *mut *mut u8) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn NetShareSetInfo(servername: ::windows_sys::core::PCWSTR, netname: ::windows_sys::core::PCWSTR, level: u32, buf: *const u8, parm_err: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn NetStatisticsGet(servername: *const i8, service: *const i8, level: u32, options: u32, buffer: *mut *mut u8) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_WindowsProgramming\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
    pub fn NtCreateFile(filehandle: *mut super::super::Foundation::HANDLE, desiredaccess: u32, objectattributes: *mut super::super::System::WindowsProgramming::OBJECT_ATTRIBUTES, iostatusblock: *mut super::super::System::WindowsProgramming::IO_STATUS_BLOCK, allocationsize: *mut i64, fileattributes: u32, shareaccess: FILE_SHARE_MODE, createdisposition: NT_CREATE_FILE_DISPOSITION, createoptions: u32, eabuffer: *mut ::core::ffi::c_void, ealength: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn OpenEncryptedFileRawA(lpfilename: ::windows_sys::core::PCSTR, ulflags: u32, pvcontext: *mut *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn OpenEncryptedFileRawW(lpfilename: ::windows_sys::core::PCWSTR, ulflags: u32, pvcontext: *mut *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenEnlistment(dwdesiredaccess: u32, resourcemanagerhandle: super::super::Foundation::HANDLE, enlistmentid: *mut ::windows_sys::core::GUID) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenFile(lpfilename: ::windows_sys::core::PCSTR, lpreopenbuff: *mut OFSTRUCT, ustyle: LZOPENFILE_STYLE) -> i32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn OpenFileById(hvolumehint: super::super::Foundation::HANDLE, lpfileid: *const FILE_ID_DESCRIPTOR, dwdesiredaccess: FILE_ACCESS_FLAGS, dwsharemode: FILE_SHARE_MODE, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, dwflagsandattributes: FILE_FLAGS_AND_ATTRIBUTES) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenResourceManager(dwdesiredaccess: u32, tmhandle: super::super::Foundation::HANDLE, resourcemanagerid: *mut ::windows_sys::core::GUID) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenTransaction(dwdesiredaccess: u32, transactionid: *mut ::windows_sys::core::GUID) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenTransactionManager(logfilename: ::windows_sys::core::PCWSTR, desiredaccess: u32, openoptions: u32) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenTransactionManagerById(transactionmanagerid: *const ::windows_sys::core::GUID, desiredaccess: u32, openoptions: u32) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn PopIoRingCompletion(ioring: *const HIORING__, cqe: *mut IORING_CQE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrePrepareComplete(enlistmenthandle: super::super::Foundation::HANDLE, tmvirtualclock: *mut i64) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrePrepareEnlistment(enlistmenthandle: super::super::Foundation::HANDLE, tmvirtualclock: *mut i64) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrepareComplete(enlistmenthandle: super::super::Foundation::HANDLE, tmvirtualclock: *mut i64) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrepareEnlistment(enlistmenthandle: super::super::Foundation::HANDLE, tmvirtualclock: *mut i64) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrepareLogArchive(hlog: super::super::Foundation::HANDLE, pszbaselogfilename: ::windows_sys::core::PWSTR, clen: u32, plsnlow: *const CLS_LSN, plsnhigh: *const CLS_LSN, pcactuallength: *mut u32, poffbaselogfiledata: *mut u64, pcbbaselogfilelength: *mut u64, plsnbase: *mut CLS_LSN, plsnlast: *mut CLS_LSN, plsncurrentarchivetail: *mut CLS_LSN, ppvarchivecontext: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrepareTape(hdevice: super::super::Foundation::HANDLE, dwoperation: PREPARE_TAPE_OPERATION, bimmediate: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn QueryDosDeviceA(lpdevicename: ::windows_sys::core::PCSTR, lptargetpath: ::windows_sys::core::PSTR, ucchmax: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn QueryDosDeviceW(lpdevicename: ::windows_sys::core::PCWSTR, lptargetpath: ::windows_sys::core::PWSTR, ucchmax: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn QueryIoRingCapabilities(capabilities: *mut IORING_CAPABILITIES) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryLogPolicy(hlog: super::super::Foundation::HANDLE, epolicytype: CLFS_MGMT_POLICY_TYPE, ppolicybuffer: *mut CLFS_MGMT_POLICY, pcbpolicybuffer: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Security\"`*"]
    #[cfg(feature = "Win32_Security")]
    pub fn QueryRecoveryAgentsOnEncryptedFile(lpfilename: ::windows_sys::core::PCWSTR, precoveryagents: *mut *mut ENCRYPTION_CERTIFICATE_HASH_LIST) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Security\"`*"]
    #[cfg(feature = "Win32_Security")]
    pub fn QueryUsersOnEncryptedFile(lpfilename: ::windows_sys::core::PCWSTR, pusers: *mut *mut ENCRYPTION_CERTIFICATE_HASH_LIST) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReOpenFile(horiginalfile: super::super::Foundation::HANDLE, dwdesiredaccess: FILE_ACCESS_FLAGS, dwsharemode: FILE_SHARE_MODE, dwflagsandattributes: FILE_FLAGS_AND_ATTRIBUTES) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn ReadDirectoryChangesExW(hdirectory: super::super::Foundation::HANDLE, lpbuffer: *mut ::core::ffi::c_void, nbufferlength: u32, bwatchsubtree: super::super::Foundation::BOOL, dwnotifyfilter: FILE_NOTIFY_CHANGE, lpbytesreturned: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, lpcompletionroutine: super::super::System::IO::LPOVERLAPPED_COMPLETION_ROUTINE, readdirectorynotifyinformationclass: READ_DIRECTORY_NOTIFY_INFORMATION_CLASS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn ReadDirectoryChangesW(hdirectory: super::super::Foundation::HANDLE, lpbuffer: *mut ::core::ffi::c_void, nbufferlength: u32, bwatchsubtree: super::super::Foundation::BOOL, dwnotifyfilter: FILE_NOTIFY_CHANGE, lpbytesreturned: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, lpcompletionroutine: super::super::System::IO::LPOVERLAPPED_COMPLETION_ROUTINE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn ReadEncryptedFileRaw(pfexportcallback: PFE_EXPORT_FUNC, pvcallbackcontext: *const ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn ReadFile(hfile: super::super::Foundation::HANDLE, lpbuffer: *mut ::core::ffi::c_void, nnumberofbytestoread: u32, lpnumberofbytesread: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn ReadFileEx(hfile: super::super::Foundation::HANDLE, lpbuffer: *mut ::core::ffi::c_void, nnumberofbytestoread: u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, lpcompletionroutine: super::super::System::IO::LPOVERLAPPED_COMPLETION_ROUTINE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn ReadFileScatter(hfile: super::super::Foundation::HANDLE, asegmentarray: *const FILE_SEGMENT_ELEMENT, nnumberofbytestoread: u32, lpreserved: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReadLogArchiveMetadata(pvarchivecontext: *mut ::core::ffi::c_void, cboffset: u32, cbbytestoread: u32, pbreadbuffer: *mut u8, pcbbytesread: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn ReadLogNotification(hlog: super::super::Foundation::HANDLE, pnotification: *mut CLFS_MGMT_NOTIFICATION, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn ReadLogRecord(pvmarshal: *mut ::core::ffi::c_void, plsnfirst: *mut CLS_LSN, econtextmode: CLFS_CONTEXT_MODE, ppvreadbuffer: *mut *mut ::core::ffi::c_void, pcbreadbuffer: *mut u32, perecordtype: *mut u8, plsnundonext: *mut CLS_LSN, plsnprevious: *mut CLS_LSN, ppvreadcontext: *mut *mut ::core::ffi::c_void, poverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn ReadLogRestartArea(pvmarshal: *mut ::core::ffi::c_void, ppvrestartbuffer: *mut *mut ::core::ffi::c_void, pcbrestartbuffer: *mut u32, plsn: *mut CLS_LSN, ppvcontext: *mut *mut ::core::ffi::c_void, poverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn ReadNextLogRecord(pvreadcontext: *mut ::core::ffi::c_void, ppvbuffer: *mut *mut ::core::ffi::c_void, pcbbuffer: *mut u32, perecordtype: *mut u8, plsnuser: *mut CLS_LSN, plsnundonext: *mut CLS_LSN, plsnprevious: *mut CLS_LSN, plsnrecord: *mut CLS_LSN, poverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReadOnlyEnlistment(enlistmenthandle: super::super::Foundation::HANDLE, tmvirtualclock: *mut i64) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn ReadPreviousLogRestartArea(pvreadcontext: *mut ::core::ffi::c_void, ppvrestartbuffer: *mut *mut ::core::ffi::c_void, pcbrestartbuffer: *mut u32, plsnrestart: *mut CLS_LSN, poverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RecoverEnlistment(enlistmenthandle: super::super::Foundation::HANDLE, enlistmentkey: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RecoverResourceManager(resourcemanagerhandle: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RecoverTransactionManager(transactionmanagerhandle: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterForLogWriteNotification(hlog: super::super::Foundation::HANDLE, cbthreshold: u32, fenable: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterManageableLogClient(hlog: super::super::Foundation::HANDLE, pcallbacks: *mut LOG_MANAGEMENT_CALLBACKS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveDirectoryA(lppathname: ::windows_sys::core::PCSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveDirectoryFromAppW(lppathname: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveDirectoryTransactedA(lppathname: ::windows_sys::core::PCSTR, htransaction: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveDirectoryTransactedW(lppathname: ::windows_sys::core::PCWSTR, htransaction: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveDirectoryW(lppathname: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveLogContainer(hlog: super::super::Foundation::HANDLE, pwszcontainerpath: ::windows_sys::core::PCWSTR, fforce: super::super::Foundation::BOOL, preserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveLogContainerSet(hlog: super::super::Foundation::HANDLE, ccontainer: u16, rgwszcontainerpath: *const ::windows_sys::core::PWSTR, fforce: super::super::Foundation::BOOL, preserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveLogPolicy(hlog: super::super::Foundation::HANDLE, epolicytype: CLFS_MGMT_POLICY_TYPE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Security\"`*"]
    #[cfg(feature = "Win32_Security")]
    pub fn RemoveUsersFromEncryptedFile(lpfilename: ::windows_sys::core::PCWSTR, phashes: *const ENCRYPTION_CERTIFICATE_HASH_LIST) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RenameTransactionManager(logfilename: ::windows_sys::core::PCWSTR, existingtransactionmanagerguid: *mut ::windows_sys::core::GUID) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReplaceFileA(lpreplacedfilename: ::windows_sys::core::PCSTR, lpreplacementfilename: ::windows_sys::core::PCSTR, lpbackupfilename: ::windows_sys::core::PCSTR, dwreplaceflags: REPLACE_FILE_FLAGS, lpexclude: *mut ::core::ffi::c_void, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReplaceFileFromAppW(lpreplacedfilename: ::windows_sys::core::PCWSTR, lpreplacementfilename: ::windows_sys::core::PCWSTR, lpbackupfilename: ::windows_sys::core::PCWSTR, dwreplaceflags: u32, lpexclude: *mut ::core::ffi::c_void, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReplaceFileW(lpreplacedfilename: ::windows_sys::core::PCWSTR, lpreplacementfilename: ::windows_sys::core::PCWSTR, lpbackupfilename: ::windows_sys::core::PCWSTR, dwreplaceflags: REPLACE_FILE_FLAGS, lpexclude: *mut ::core::ffi::c_void, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn ReserveAndAppendLog(pvmarshal: *mut ::core::ffi::c_void, rgwriteentries: *mut CLS_WRITE_ENTRY, cwriteentries: u32, plsnundonext: *mut CLS_LSN, plsnprevious: *mut CLS_LSN, creserverecords: u32, rgcbreservation: *mut i64, fflags: CLFS_FLAG, plsn: *mut CLS_LSN, poverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn ReserveAndAppendLogAligned(pvmarshal: *mut ::core::ffi::c_void, rgwriteentries: *mut CLS_WRITE_ENTRY, cwriteentries: u32, cbentryalignment: u32, plsnundonext: *mut CLS_LSN, plsnprevious: *mut CLS_LSN, creserverecords: u32, rgcbreservation: *mut i64, fflags: CLFS_FLAG, plsn: *mut CLS_LSN, poverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RollbackComplete(enlistmenthandle: super::super::Foundation::HANDLE, tmvirtualclock: *mut i64) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RollbackEnlistment(enlistmenthandle: super::super::Foundation::HANDLE, tmvirtualclock: *mut i64) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RollbackTransaction(transactionhandle: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RollbackTransactionAsync(transactionhandle: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RollforwardTransactionManager(transactionmanagerhandle: super::super::Foundation::HANDLE, tmvirtualclock: *mut i64) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ScanLogContainers(pcxscan: *mut CLS_SCAN_CONTEXT, escanmode: u8, preserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn SearchPathA(lppath: ::windows_sys::core::PCSTR, lpfilename: ::windows_sys::core::PCSTR, lpextension: ::windows_sys::core::PCSTR, nbufferlength: u32, lpbuffer: ::windows_sys::core::PSTR, lpfilepart: *mut ::windows_sys::core::PSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn SearchPathW(lppath: ::windows_sys::core::PCWSTR, lpfilename: ::windows_sys::core::PCWSTR, lpextension: ::windows_sys::core::PCWSTR, nbufferlength: u32, lpbuffer: ::windows_sys::core::PWSTR, lpfilepart: *mut ::windows_sys::core::PWSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Security\"`*"]
    #[cfg(feature = "Win32_Security")]
    pub fn SetEncryptedFileMetadata(lpfilename: ::windows_sys::core::PCWSTR, pboldmetadata: *const u8, pbnewmetadata: *const u8, pownerhash: *const ENCRYPTION_CERTIFICATE_HASH, dwoperation: u32, pcertificatesadded: *const ENCRYPTION_CERTIFICATE_HASH_LIST) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetEndOfFile(hfile: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn SetEndOfLog(hlog: super::super::Foundation::HANDLE, plsnend: *mut CLS_LSN, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetEnlistmentRecoveryInformation(enlistmenthandle: super::super::Foundation::HANDLE, buffersize: u32, buffer: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn SetFileApisToANSI();
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn SetFileApisToOEM();
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFileAttributesA(lpfilename: ::windows_sys::core::PCSTR, dwfileattributes: FILE_FLAGS_AND_ATTRIBUTES) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFileAttributesFromAppW(lpfilename: ::windows_sys::core::PCWSTR, dwfileattributes: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFileAttributesTransactedA(lpfilename: ::windows_sys::core::PCSTR, dwfileattributes: u32, htransaction: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFileAttributesTransactedW(lpfilename: ::windows_sys::core::PCWSTR, dwfileattributes: u32, htransaction: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFileAttributesW(lpfilename: ::windows_sys::core::PCWSTR, dwfileattributes: FILE_FLAGS_AND_ATTRIBUTES) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFileBandwidthReservation(hfile: super::super::Foundation::HANDLE, nperiodmilliseconds: u32, nbytesperperiod: u32, bdiscardable: super::super::Foundation::BOOL, lptransfersize: *mut u32, lpnumoutstandingrequests: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFileCompletionNotificationModes(filehandle: super::super::Foundation::HANDLE, flags: u8) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFileInformationByHandle(hfile: super::super::Foundation::HANDLE, fileinformationclass: FILE_INFO_BY_HANDLE_CLASS, lpfileinformation: *const ::core::ffi::c_void, dwbuffersize: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFileIoOverlappedRange(filehandle: super::super::Foundation::HANDLE, overlappedrangestart: *const u8, length: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFilePointer(hfile: super::super::Foundation::HANDLE, ldistancetomove: i32, lpdistancetomovehigh: *mut i32, dwmovemethod: SET_FILE_POINTER_MOVE_METHOD) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFilePointerEx(hfile: super::super::Foundation::HANDLE, lidistancetomove: i64, lpnewfilepointer: *mut i64, dwmovemethod: SET_FILE_POINTER_MOVE_METHOD) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFileShortNameA(hfile: super::super::Foundation::HANDLE, lpshortname: ::windows_sys::core::PCSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFileShortNameW(hfile: super::super::Foundation::HANDLE, lpshortname: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFileTime(hfile: super::super::Foundation::HANDLE, lpcreationtime: *const super::super::Foundation::FILETIME, lplastaccesstime: *const super::super::Foundation::FILETIME, lplastwritetime: *const super::super::Foundation::FILETIME) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFileValidData(hfile: super::super::Foundation::HANDLE, validdatalength: i64) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetIoRingCompletionEvent(ioring: *const HIORING__, hevent: super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetLogArchiveMode(hlog: super::super::Foundation::HANDLE, emode: CLFS_LOG_ARCHIVE_MODE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetLogArchiveTail(hlog: super::super::Foundation::HANDLE, plsnarchivetail: *mut CLS_LSN, preserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetLogFileSizeWithPolicy(hlog: super::super::Foundation::HANDLE, pdesiredsize: *mut u64, presultingsize: *mut u64) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetResourceManagerCompletionPort(resourcemanagerhandle: super::super::Foundation::HANDLE, iocompletionporthandle: super::super::Foundation::HANDLE, completionkey: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetSearchPathMode(flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetTapeParameters(hdevice: super::super::Foundation::HANDLE, dwoperation: TAPE_INFORMATION_TYPE, lptapeinformation: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetTapePosition(hdevice: super::super::Foundation::HANDLE, dwpositionmethod: TAPE_POSITION_METHOD, dwpartition: u32, dwoffsetlow: u32, dwoffsethigh: u32, bimmediate: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetTransactionInformation(transactionhandle: super::super::Foundation::HANDLE, isolationlevel: u32, isolationflags: u32, timeout: u32, description: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Security\"`*"]
    #[cfg(feature = "Win32_Security")]
    pub fn SetUserFileEncryptionKey(pencryptioncertificate: *const ENCRYPTION_CERTIFICATE) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Security\"`*"]
    #[cfg(feature = "Win32_Security")]
    pub fn SetUserFileEncryptionKeyEx(pencryptioncertificate: *const ENCRYPTION_CERTIFICATE, dwcapabilities: u32, dwflags: u32, pvreserved: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetVolumeLabelA(lprootpathname: ::windows_sys::core::PCSTR, lpvolumename: ::windows_sys::core::PCSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetVolumeLabelW(lprootpathname: ::windows_sys::core::PCWSTR, lpvolumename: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetVolumeMountPointA(lpszvolumemountpoint: ::windows_sys::core::PCSTR, lpszvolumename: ::windows_sys::core::PCSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetVolumeMountPointW(lpszvolumemountpoint: ::windows_sys::core::PCWSTR, lpszvolumename: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SinglePhaseReject(enlistmenthandle: super::super::Foundation::HANDLE, tmvirtualclock: *mut i64) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn SubmitIoRing(ioring: *const HIORING__, waitoperations: u32, milliseconds: u32, submittedentries: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TerminateLogArchive(pvarchivecontext: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TerminateReadLog(pvcursorcontext: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn TruncateLog(pvmarshal: *const ::core::ffi::c_void, plsnend: *const CLS_LSN, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn TxfGetThreadMiniVersionForCreate(miniversion: *mut u16);
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TxfLogCreateFileReadContext(logpath: ::windows_sys::core::PCWSTR, beginninglsn: CLS_LSN, endinglsn: CLS_LSN, txffileid: *const TXF_ID, txflogcontext: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TxfLogCreateRangeReadContext(logpath: ::windows_sys::core::PCWSTR, beginninglsn: CLS_LSN, endinglsn: CLS_LSN, beginningvirtualclock: *const i64, endingvirtualclock: *const i64, recordtypemask: u32, txflogcontext: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TxfLogDestroyReadContext(txflogcontext: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TxfLogReadRecords(txflogcontext: *const ::core::ffi::c_void, bufferlength: u32, buffer: *mut ::core::ffi::c_void, bytesused: *mut u32, recordcount: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TxfLogRecordGetFileName(recordbuffer: *const ::core::ffi::c_void, recordbufferlengthinbytes: u32, namebuffer: ::windows_sys::core::PWSTR, namebufferlengthinbytes: *mut u32, txfid: *mut TXF_ID) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TxfLogRecordGetGenericType(recordbuffer: *const ::core::ffi::c_void, recordbufferlengthinbytes: u32, generictype: *mut u32, virtualclock: *mut i64) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TxfReadMetadataInfo(filehandle: super::super::Foundation::HANDLE, txffileid: *mut TXF_ID, lastlsn: *mut CLS_LSN, transactionstate: *mut u32, lockingtransaction: *mut ::windows_sys::core::GUID) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn TxfSetThreadMiniVersionForCreate(miniversion: u16);
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnlockFile(hfile: super::super::Foundation::HANDLE, dwfileoffsetlow: u32, dwfileoffsethigh: u32, nnumberofbytestounlocklow: u32, nnumberofbytestounlockhigh: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn UnlockFileEx(hfile: super::super::Foundation::HANDLE, dwreserved: u32, nnumberofbytestounlocklow: u32, nnumberofbytestounlockhigh: u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn ValidateLog(pszlogfilename: ::windows_sys::core::PCWSTR, psalogfile: *mut super::super::Security::SECURITY_ATTRIBUTES, pinfobuffer: *mut CLS_INFORMATION, pcbbuffer: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn VerFindFileA(uflags: VER_FIND_FILE_FLAGS, szfilename: ::windows_sys::core::PCSTR, szwindir: ::windows_sys::core::PCSTR, szappdir: ::windows_sys::core::PCSTR, szcurdir: ::windows_sys::core::PSTR, pucurdirlen: *mut u32, szdestdir: ::windows_sys::core::PSTR, pudestdirlen: *mut u32) -> VER_FIND_FILE_STATUS;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn VerFindFileW(uflags: VER_FIND_FILE_FLAGS, szfilename: ::windows_sys::core::PCWSTR, szwindir: ::windows_sys::core::PCWSTR, szappdir: ::windows_sys::core::PCWSTR, szcurdir: ::windows_sys::core::PWSTR, pucurdirlen: *mut u32, szdestdir: ::windows_sys::core::PWSTR, pudestdirlen: *mut u32) -> VER_FIND_FILE_STATUS;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn VerInstallFileA(uflags: VER_INSTALL_FILE_FLAGS, szsrcfilename: ::windows_sys::core::PCSTR, szdestfilename: ::windows_sys::core::PCSTR, szsrcdir: ::windows_sys::core::PCSTR, szdestdir: ::windows_sys::core::PCSTR, szcurdir: ::windows_sys::core::PCSTR, sztmpfile: ::windows_sys::core::PSTR, putmpfilelen: *mut u32) -> VER_INSTALL_FILE_STATUS;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn VerInstallFileW(uflags: VER_INSTALL_FILE_FLAGS, szsrcfilename: ::windows_sys::core::PCWSTR, szdestfilename: ::windows_sys::core::PCWSTR, szsrcdir: ::windows_sys::core::PCWSTR, szdestdir: ::windows_sys::core::PCWSTR, szcurdir: ::windows_sys::core::PCWSTR, sztmpfile: ::windows_sys::core::PWSTR, putmpfilelen: *mut u32) -> VER_INSTALL_FILE_STATUS;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn VerLanguageNameA(wlang: u32, szlang: ::windows_sys::core::PSTR, cchlang: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn VerLanguageNameW(wlang: u32, szlang: ::windows_sys::core::PWSTR, cchlang: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VerQueryValueA(pblock: *const ::core::ffi::c_void, lpsubblock: ::windows_sys::core::PCSTR, lplpbuffer: *mut *mut ::core::ffi::c_void, pulen: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VerQueryValueW(pblock: *const ::core::ffi::c_void, lpsubblock: ::windows_sys::core::PCWSTR, lplpbuffer: *mut *mut ::core::ffi::c_void, pulen: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WofEnumEntries(volumename: ::windows_sys::core::PCWSTR, provider: u32, enumproc: WofEnumEntryProc, userdata: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WofFileEnumFiles(volumename: ::windows_sys::core::PCWSTR, algorithm: u32, enumproc: WofEnumFilesProc, userdata: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WofGetDriverVersion(fileorvolumehandle: super::super::Foundation::HANDLE, provider: u32, wofversion: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WofIsExternalFile(filepath: ::windows_sys::core::PCWSTR, isexternalfile: *mut super::super::Foundation::BOOL, provider: *mut u32, externalfileinfo: *mut ::core::ffi::c_void, bufferlength: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WofSetFileDataLocation(filehandle: super::super::Foundation::HANDLE, provider: u32, externalfileinfo: *const ::core::ffi::c_void, length: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WofShouldCompressBinaries(volume: ::windows_sys::core::PCWSTR, algorithm: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn WofWimAddEntry(volumename: ::windows_sys::core::PCWSTR, wimpath: ::windows_sys::core::PCWSTR, wimtype: u32, wimindex: u32, datasourceid: *mut i64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WofWimEnumFiles(volumename: ::windows_sys::core::PCWSTR, datasourceid: i64, enumproc: WofEnumFilesProc, userdata: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn WofWimRemoveEntry(volumename: ::windows_sys::core::PCWSTR, datasourceid: i64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn WofWimSuspendEntry(volumename: ::windows_sys::core::PCWSTR, datasourceid: i64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn WofWimUpdateEntry(volumename: ::windows_sys::core::PCWSTR, datasourceid: i64, newwimpath: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Wow64DisableWow64FsRedirection(oldvalue: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Wow64EnableWow64FsRedirection(wow64fsenableredirection: super::super::Foundation::BOOLEAN) -> super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Wow64RevertWow64FsRedirection(olvalue: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
    pub fn WriteEncryptedFileRaw(pfimportcallback: PFE_IMPORT_FUNC, pvcallbackcontext: *const ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WriteFile(hfile: super::super::Foundation::HANDLE, lpbuffer: *const ::core::ffi::c_void, nnumberofbytestowrite: u32, lpnumberofbyteswritten: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WriteFileEx(hfile: super::super::Foundation::HANDLE, lpbuffer: *const ::core::ffi::c_void, nnumberofbytestowrite: u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, lpcompletionroutine: super::super::System::IO::LPOVERLAPPED_COMPLETION_ROUTINE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WriteFileGather(hfile: super::super::Foundation::HANDLE, asegmentarray: *const FILE_SEGMENT_ELEMENT, nnumberofbytestowrite: u32, lpreserved: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WriteLogRestartArea(pvmarshal: *mut ::core::ffi::c_void, pvrestartbuffer: *mut ::core::ffi::c_void, cbrestartbuffer: u32, plsnbase: *mut CLS_LSN, fflags: CLFS_FLAG, pcbwritten: *mut u32, plsnnext: *mut CLS_LSN, poverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WriteTapemark(hdevice: super::super::Foundation::HANDLE, dwtapemarktype: TAPEMARK_TYPE, dwtapemarkcount: u32, bimmediate: super::super::Foundation::BOOL) -> u32;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct BY_HANDLE_FILE_INFORMATION {
    pub dwFileAttributes: u32,
    pub ftCreationTime: super::super::Foundation::FILETIME,
    pub ftLastAccessTime: super::super::Foundation::FILETIME,
    pub ftLastWriteTime: super::super::Foundation::FILETIME,
    pub dwVolumeSerialNumber: u32,
    pub nFileSizeHigh: u32,
    pub nFileSizeLow: u32,
    pub nNumberOfLinks: u32,
    pub nFileIndexHigh: u32,
    pub nFileIndexLow: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for BY_HANDLE_FILE_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for BY_HANDLE_FILE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub type CACHE_ACCESS_CHECK = ::core::option::Option<unsafe extern "system" fn(psecuritydescriptor: super::super::Security::PSECURITY_DESCRIPTOR, hclienttoken: super::super::Foundation::HANDLE, dwdesiredaccess: u32, genericmapping: *mut super::super::Security::GENERIC_MAPPING, privilegeset: *mut super::super::Security::PRIVILEGE_SET, privilegesetlength: *mut u32, grantedaccess: *mut u32, accessstatus: *mut i32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type CACHE_DESTROY_CALLBACK = ::core::option::Option<unsafe extern "system" fn(cb: u32, lpb: *mut u8)>;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type CACHE_KEY_COMPARE = ::core::option::Option<unsafe extern "system" fn(cbkey1: u32, lpbkey1: *mut u8, cbkey2: u32, lpbkey2: *mut u8) -> i32>;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type CACHE_KEY_HASH = ::core::option::Option<unsafe extern "system" fn(lpbkey: *mut u8, cbkey: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type CACHE_READ_CALLBACK = ::core::option::Option<unsafe extern "system" fn(cb: u32, lpb: *mut u8, lpvcontext: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type CLAIMMEDIALABEL = ::core::option::Option<unsafe extern "system" fn(pbuffer: *const u8, nbuffersize: u32, plabelinfo: *mut MediaLabelInfo) -> u32>;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type CLAIMMEDIALABELEX = ::core::option::Option<unsafe extern "system" fn(pbuffer: *const u8, nbuffersize: u32, plabelinfo: *mut MediaLabelInfo, labelguid: *mut ::windows_sys::core::GUID) -> u32>;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CLFS_BASELOG_EXTENSION: &str = ".blf";
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type CLFS_BLOCK_ALLOCATION = ::core::option::Option<unsafe extern "system" fn(cbbufferlength: u32, pvusercontext: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void>;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type CLFS_BLOCK_DEALLOCATION = ::core::option::Option<unsafe extern "system" fn(pvbuffer: *mut ::core::ffi::c_void, pvusercontext: *mut ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CLFS_CONTAINER_RELATIVE_PREFIX: &str = "%BLF%\\";
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CLFS_CONTAINER_STREAM_PREFIX: &str = "%BLF%:";
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type CLFS_CONTEXT_MODE = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsContextNone: CLFS_CONTEXT_MODE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsContextUndoNext: CLFS_CONTEXT_MODE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsContextPrevious: CLFS_CONTEXT_MODE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsContextForward: CLFS_CONTEXT_MODE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type CLFS_FLAG = u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CLFS_FLAG_FORCE_APPEND: CLFS_FLAG = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CLFS_FLAG_FORCE_FLUSH: CLFS_FLAG = 2u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CLFS_FLAG_NO_FLAGS: CLFS_FLAG = 0u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CLFS_FLAG_USE_RESERVATION: CLFS_FLAG = 4u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CLFS_FLAG_FILTER_INTERMEDIATE_LEVEL: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CLFS_FLAG_FILTER_TOP_LEVEL: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CLFS_FLAG_HIDDEN_SYSTEM_LOG: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CLFS_FLAG_IGNORE_SHARE_ACCESS: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CLFS_FLAG_MINIFILTER_LEVEL: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CLFS_FLAG_NON_REENTRANT_FILTER: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CLFS_FLAG_READ_IN_PROGRESS: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CLFS_FLAG_REENTRANT_FILE_SYSTEM: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CLFS_FLAG_REENTRANT_FILTER: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type CLFS_IOSTATS_CLASS = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsIoStatsDefault: CLFS_IOSTATS_CLASS = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsIoStatsMax: CLFS_IOSTATS_CLASS = 65535i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type CLFS_LOG_ARCHIVE_MODE = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsLogArchiveEnabled: CLFS_LOG_ARCHIVE_MODE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsLogArchiveDisabled: CLFS_LOG_ARCHIVE_MODE = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct CLFS_LOG_NAME_INFORMATION {
    pub NameLengthInBytes: u16,
    pub Name: [u16; 1],
}
impl ::core::marker::Copy for CLFS_LOG_NAME_INFORMATION {}
impl ::core::clone::Clone for CLFS_LOG_NAME_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CLFS_MARSHALLING_FLAG_DISABLE_BUFF_INIT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CLFS_MARSHALLING_FLAG_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CLFS_MAX_CONTAINER_INFO: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CLFS_MGMT_CLIENT_REGISTRATION_VERSION: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct CLFS_MGMT_NOTIFICATION {
    pub Notification: CLFS_MGMT_NOTIFICATION_TYPE,
    pub Lsn: CLS_LSN,
    pub LogIsPinned: u16,
}
impl ::core::marker::Copy for CLFS_MGMT_NOTIFICATION {}
impl ::core::clone::Clone for CLFS_MGMT_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type CLFS_MGMT_NOTIFICATION_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsMgmtAdvanceTailNotification: CLFS_MGMT_NOTIFICATION_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsMgmtLogFullHandlerNotification: CLFS_MGMT_NOTIFICATION_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsMgmtLogUnpinnedNotification: CLFS_MGMT_NOTIFICATION_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsMgmtLogWriteNotification: CLFS_MGMT_NOTIFICATION_TYPE = 3i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct CLFS_MGMT_POLICY {
    pub Version: u32,
    pub LengthInBytes: u32,
    pub PolicyFlags: u32,
    pub PolicyType: CLFS_MGMT_POLICY_TYPE,
    pub PolicyParameters: CLFS_MGMT_POLICY_0,
}
impl ::core::marker::Copy for CLFS_MGMT_POLICY {}
impl ::core::clone::Clone for CLFS_MGMT_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub union CLFS_MGMT_POLICY_0 {
    pub MaximumSize: CLFS_MGMT_POLICY_0_4,
    pub MinimumSize: CLFS_MGMT_POLICY_0_5,
    pub NewContainerSize: CLFS_MGMT_POLICY_0_8,
    pub GrowthRate: CLFS_MGMT_POLICY_0_2,
    pub LogTail: CLFS_MGMT_POLICY_0_3,
    pub AutoShrink: CLFS_MGMT_POLICY_0_1,
    pub AutoGrow: CLFS_MGMT_POLICY_0_0,
    pub NewContainerPrefix: CLFS_MGMT_POLICY_0_7,
    pub NewContainerSuffix: CLFS_MGMT_POLICY_0_9,
    pub NewContainerExtension: CLFS_MGMT_POLICY_0_6,
}
impl ::core::marker::Copy for CLFS_MGMT_POLICY_0 {}
impl ::core::clone::Clone for CLFS_MGMT_POLICY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct CLFS_MGMT_POLICY_0_0 {
    pub Enabled: u32,
}
impl ::core::marker::Copy for CLFS_MGMT_POLICY_0_0 {}
impl ::core::clone::Clone for CLFS_MGMT_POLICY_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct CLFS_MGMT_POLICY_0_1 {
    pub Percentage: u32,
}
impl ::core::marker::Copy for CLFS_MGMT_POLICY_0_1 {}
impl ::core::clone::Clone for CLFS_MGMT_POLICY_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct CLFS_MGMT_POLICY_0_2 {
    pub AbsoluteGrowthInContainers: u32,
    pub RelativeGrowthPercentage: u32,
}
impl ::core::marker::Copy for CLFS_MGMT_POLICY_0_2 {}
impl ::core::clone::Clone for CLFS_MGMT_POLICY_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct CLFS_MGMT_POLICY_0_3 {
    pub MinimumAvailablePercentage: u32,
    pub MinimumAvailableContainers: u32,
}
impl ::core::marker::Copy for CLFS_MGMT_POLICY_0_3 {}
impl ::core::clone::Clone for CLFS_MGMT_POLICY_0_3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct CLFS_MGMT_POLICY_0_4 {
    pub Containers: u32,
}
impl ::core::marker::Copy for CLFS_MGMT_POLICY_0_4 {}
impl ::core::clone::Clone for CLFS_MGMT_POLICY_0_4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct CLFS_MGMT_POLICY_0_5 {
    pub Containers: u32,
}
impl ::core::marker::Copy for CLFS_MGMT_POLICY_0_5 {}
impl ::core::clone::Clone for CLFS_MGMT_POLICY_0_5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct CLFS_MGMT_POLICY_0_6 {
    pub ExtensionLengthInBytes: u16,
    pub ExtensionString: [u16; 1],
}
impl ::core::marker::Copy for CLFS_MGMT_POLICY_0_6 {}
impl ::core::clone::Clone for CLFS_MGMT_POLICY_0_6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct CLFS_MGMT_POLICY_0_7 {
    pub PrefixLengthInBytes: u16,
    pub PrefixString: [u16; 1],
}
impl ::core::marker::Copy for CLFS_MGMT_POLICY_0_7 {}
impl ::core::clone::Clone for CLFS_MGMT_POLICY_0_7 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct CLFS_MGMT_POLICY_0_8 {
    pub SizeInBytes: u32,
}
impl ::core::marker::Copy for CLFS_MGMT_POLICY_0_8 {}
impl ::core::clone::Clone for CLFS_MGMT_POLICY_0_8 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct CLFS_MGMT_POLICY_0_9 {
    pub NextContainerSuffix: u64,
}
impl ::core::marker::Copy for CLFS_MGMT_POLICY_0_9 {}
impl ::core::clone::Clone for CLFS_MGMT_POLICY_0_9 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type CLFS_MGMT_POLICY_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsMgmtPolicyMaximumSize: CLFS_MGMT_POLICY_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsMgmtPolicyMinimumSize: CLFS_MGMT_POLICY_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsMgmtPolicyNewContainerSize: CLFS_MGMT_POLICY_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsMgmtPolicyGrowthRate: CLFS_MGMT_POLICY_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsMgmtPolicyLogTail: CLFS_MGMT_POLICY_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsMgmtPolicyAutoShrink: CLFS_MGMT_POLICY_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsMgmtPolicyAutoGrow: CLFS_MGMT_POLICY_TYPE = 6i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsMgmtPolicyNewContainerPrefix: CLFS_MGMT_POLICY_TYPE = 7i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsMgmtPolicyNewContainerSuffix: CLFS_MGMT_POLICY_TYPE = 8i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsMgmtPolicyNewContainerExtension: CLFS_MGMT_POLICY_TYPE = 9i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsMgmtPolicyInvalid: CLFS_MGMT_POLICY_TYPE = 10i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CLFS_MGMT_POLICY_VERSION: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct CLFS_NODE_ID {
    pub cType: u32,
    pub cbNode: u32,
}
impl ::core::marker::Copy for CLFS_NODE_ID {}
impl ::core::clone::Clone for CLFS_NODE_ID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct CLFS_PHYSICAL_LSN_INFORMATION {
    pub StreamIdentifier: u8,
    pub VirtualLsn: CLS_LSN,
    pub PhysicalLsn: CLS_LSN,
}
impl ::core::marker::Copy for CLFS_PHYSICAL_LSN_INFORMATION {}
impl ::core::clone::Clone for CLFS_PHYSICAL_LSN_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CLFS_SCAN_BACKWARD: u8 = 4u8;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CLFS_SCAN_BUFFERED: u8 = 32u8;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CLFS_SCAN_CLOSE: u8 = 8u8;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CLFS_SCAN_FORWARD: u8 = 2u8;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CLFS_SCAN_INIT: u8 = 1u8;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CLFS_SCAN_INITIALIZED: u8 = 16u8;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct CLFS_STREAM_ID_INFORMATION {
    pub StreamIdentifier: u8,
}
impl ::core::marker::Copy for CLFS_STREAM_ID_INFORMATION {}
impl ::core::clone::Clone for CLFS_STREAM_ID_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CLSID_DiskQuotaControl: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2039002481, data2: 60553, data3: 4559, data4: [156, 0, 0, 170, 0, 161, 79, 86] };
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct CLS_ARCHIVE_DESCRIPTOR {
    pub coffLow: u64,
    pub coffHigh: u64,
    pub infoContainer: CLS_CONTAINER_INFORMATION,
}
impl ::core::marker::Copy for CLS_ARCHIVE_DESCRIPTOR {}
impl ::core::clone::Clone for CLS_ARCHIVE_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct CLS_CONTAINER_INFORMATION {
    pub FileAttributes: u32,
    pub CreationTime: u64,
    pub LastAccessTime: u64,
    pub LastWriteTime: u64,
    pub ContainerSize: i64,
    pub FileNameActualLength: u32,
    pub FileNameLength: u32,
    pub FileName: [u16; 256],
    pub State: u32,
    pub PhysicalContainerId: u32,
    pub LogicalContainerId: u32,
}
impl ::core::marker::Copy for CLS_CONTAINER_INFORMATION {}
impl ::core::clone::Clone for CLS_CONTAINER_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type CLS_CONTEXT_MODE = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClsContextNone: CLS_CONTEXT_MODE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClsContextUndoNext: CLS_CONTEXT_MODE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClsContextPrevious: CLS_CONTEXT_MODE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClsContextForward: CLS_CONTEXT_MODE = 3i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct CLS_INFORMATION {
    pub TotalAvailable: i64,
    pub CurrentAvailable: i64,
    pub TotalReservation: i64,
    pub BaseFileSize: u64,
    pub ContainerSize: u64,
    pub TotalContainers: u32,
    pub FreeContainers: u32,
    pub TotalClients: u32,
    pub Attributes: u32,
    pub FlushThreshold: u32,
    pub SectorSize: u32,
    pub MinArchiveTailLsn: CLS_LSN,
    pub BaseLsn: CLS_LSN,
    pub LastFlushedLsn: CLS_LSN,
    pub LastLsn: CLS_LSN,
    pub RestartLsn: CLS_LSN,
    pub Identity: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for CLS_INFORMATION {}
impl ::core::clone::Clone for CLS_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type CLS_IOSTATS_CLASS = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClsIoStatsDefault: CLS_IOSTATS_CLASS = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClsIoStatsMax: CLS_IOSTATS_CLASS = 65535i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct CLS_IO_STATISTICS {
    pub hdrIoStats: CLS_IO_STATISTICS_HEADER,
    pub cFlush: u64,
    pub cbFlush: u64,
    pub cMetaFlush: u64,
    pub cbMetaFlush: u64,
}
impl ::core::marker::Copy for CLS_IO_STATISTICS {}
impl ::core::clone::Clone for CLS_IO_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct CLS_IO_STATISTICS_HEADER {
    pub ubMajorVersion: u8,
    pub ubMinorVersion: u8,
    pub eStatsClass: CLFS_IOSTATS_CLASS,
    pub cbLength: u16,
    pub coffData: u32,
}
impl ::core::marker::Copy for CLS_IO_STATISTICS_HEADER {}
impl ::core::clone::Clone for CLS_IO_STATISTICS_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type CLS_LOG_INFORMATION_CLASS = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsLogBasicInformation: CLS_LOG_INFORMATION_CLASS = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsLogBasicInformationPhysical: CLS_LOG_INFORMATION_CLASS = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsLogPhysicalNameInformation: CLS_LOG_INFORMATION_CLASS = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsLogStreamIdentifierInformation: CLS_LOG_INFORMATION_CLASS = 3i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsLogSystemMarkingInformation: CLS_LOG_INFORMATION_CLASS = 4i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsLogPhysicalLsnInformation: CLS_LOG_INFORMATION_CLASS = 5i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct CLS_LSN {
    pub Internal: u64,
}
impl ::core::marker::Copy for CLS_LSN {}
impl ::core::clone::Clone for CLS_LSN {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CLS_SCAN_CONTEXT {
    pub cidNode: CLFS_NODE_ID,
    pub hLog: super::super::Foundation::HANDLE,
    pub cIndex: u32,
    pub cContainers: u32,
    pub cContainersReturned: u32,
    pub eScanMode: u8,
    pub pinfoContainer: *mut CLS_CONTAINER_INFORMATION,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CLS_SCAN_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CLS_SCAN_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct CLS_WRITE_ENTRY {
    pub Buffer: *mut ::core::ffi::c_void,
    pub ByteLength: u32,
}
impl ::core::marker::Copy for CLS_WRITE_ENTRY {}
impl ::core::clone::Clone for CLS_WRITE_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct CONNECTION_INFO_0 {
    pub coni0_id: u32,
}
impl ::core::marker::Copy for CONNECTION_INFO_0 {}
impl ::core::clone::Clone for CONNECTION_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct CONNECTION_INFO_1 {
    pub coni1_id: u32,
    pub coni1_type: SHARE_TYPE,
    pub coni1_num_opens: u32,
    pub coni1_num_users: u32,
    pub coni1_time: u32,
    pub coni1_username: ::windows_sys::core::PWSTR,
    pub coni1_netname: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for CONNECTION_INFO_1 {}
impl ::core::clone::Clone for CONNECTION_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type COPYFILE2_COPY_PHASE = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const COPYFILE2_PHASE_NONE: COPYFILE2_COPY_PHASE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const COPYFILE2_PHASE_PREPARE_SOURCE: COPYFILE2_COPY_PHASE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const COPYFILE2_PHASE_PREPARE_DEST: COPYFILE2_COPY_PHASE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const COPYFILE2_PHASE_READ_SOURCE: COPYFILE2_COPY_PHASE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const COPYFILE2_PHASE_WRITE_DESTINATION: COPYFILE2_COPY_PHASE = 4i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const COPYFILE2_PHASE_SERVER_COPY: COPYFILE2_COPY_PHASE = 5i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const COPYFILE2_PHASE_NAMEGRAFT_COPY: COPYFILE2_COPY_PHASE = 6i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const COPYFILE2_PHASE_MAX: COPYFILE2_COPY_PHASE = 7i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct COPYFILE2_EXTENDED_PARAMETERS {
    pub dwSize: u32,
    pub dwCopyFlags: u32,
    pub pfCancel: *mut super::super::Foundation::BOOL,
    pub pProgressRoutine: PCOPYFILE2_PROGRESS_ROUTINE,
    pub pvCallbackContext: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for COPYFILE2_EXTENDED_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for COPYFILE2_EXTENDED_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct COPYFILE2_EXTENDED_PARAMETERS_V2 {
    pub dwSize: u32,
    pub dwCopyFlags: u32,
    pub pfCancel: *mut super::super::Foundation::BOOL,
    pub pProgressRoutine: PCOPYFILE2_PROGRESS_ROUTINE,
    pub pvCallbackContext: *mut ::core::ffi::c_void,
    pub dwCopyFlagsV2: u32,
    pub ioDesiredSize: u32,
    pub ioDesiredRate: u32,
    pub reserved: [*mut ::core::ffi::c_void; 8],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for COPYFILE2_EXTENDED_PARAMETERS_V2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for COPYFILE2_EXTENDED_PARAMETERS_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct COPYFILE2_MESSAGE {
    pub Type: COPYFILE2_MESSAGE_TYPE,
    pub dwPadding: u32,
    pub Info: COPYFILE2_MESSAGE_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for COPYFILE2_MESSAGE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for COPYFILE2_MESSAGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union COPYFILE2_MESSAGE_0 {
    pub ChunkStarted: COPYFILE2_MESSAGE_0_1,
    pub ChunkFinished: COPYFILE2_MESSAGE_0_0,
    pub StreamStarted: COPYFILE2_MESSAGE_0_5,
    pub StreamFinished: COPYFILE2_MESSAGE_0_4,
    pub PollContinue: COPYFILE2_MESSAGE_0_3,
    pub Error: COPYFILE2_MESSAGE_0_2,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for COPYFILE2_MESSAGE_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for COPYFILE2_MESSAGE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct COPYFILE2_MESSAGE_0_0 {
    pub dwStreamNumber: u32,
    pub dwFlags: u32,
    pub hSourceFile: super::super::Foundation::HANDLE,
    pub hDestinationFile: super::super::Foundation::HANDLE,
    pub uliChunkNumber: u64,
    pub uliChunkSize: u64,
    pub uliStreamSize: u64,
    pub uliStreamBytesTransferred: u64,
    pub uliTotalFileSize: u64,
    pub uliTotalBytesTransferred: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for COPYFILE2_MESSAGE_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for COPYFILE2_MESSAGE_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct COPYFILE2_MESSAGE_0_1 {
    pub dwStreamNumber: u32,
    pub dwReserved: u32,
    pub hSourceFile: super::super::Foundation::HANDLE,
    pub hDestinationFile: super::super::Foundation::HANDLE,
    pub uliChunkNumber: u64,
    pub uliChunkSize: u64,
    pub uliStreamSize: u64,
    pub uliTotalFileSize: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for COPYFILE2_MESSAGE_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for COPYFILE2_MESSAGE_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct COPYFILE2_MESSAGE_0_2 {
    pub CopyPhase: COPYFILE2_COPY_PHASE,
    pub dwStreamNumber: u32,
    pub hrFailure: ::windows_sys::core::HRESULT,
    pub dwReserved: u32,
    pub uliChunkNumber: u64,
    pub uliStreamSize: u64,
    pub uliStreamBytesTransferred: u64,
    pub uliTotalFileSize: u64,
    pub uliTotalBytesTransferred: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for COPYFILE2_MESSAGE_0_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for COPYFILE2_MESSAGE_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct COPYFILE2_MESSAGE_0_3 {
    pub dwReserved: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for COPYFILE2_MESSAGE_0_3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for COPYFILE2_MESSAGE_0_3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct COPYFILE2_MESSAGE_0_4 {
    pub dwStreamNumber: u32,
    pub dwReserved: u32,
    pub hSourceFile: super::super::Foundation::HANDLE,
    pub hDestinationFile: super::super::Foundation::HANDLE,
    pub uliStreamSize: u64,
    pub uliStreamBytesTransferred: u64,
    pub uliTotalFileSize: u64,
    pub uliTotalBytesTransferred: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for COPYFILE2_MESSAGE_0_4 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for COPYFILE2_MESSAGE_0_4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct COPYFILE2_MESSAGE_0_5 {
    pub dwStreamNumber: u32,
    pub dwReserved: u32,
    pub hSourceFile: super::super::Foundation::HANDLE,
    pub hDestinationFile: super::super::Foundation::HANDLE,
    pub uliStreamSize: u64,
    pub uliTotalFileSize: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for COPYFILE2_MESSAGE_0_5 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for COPYFILE2_MESSAGE_0_5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type COPYFILE2_MESSAGE_ACTION = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const COPYFILE2_PROGRESS_CONTINUE: COPYFILE2_MESSAGE_ACTION = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const COPYFILE2_PROGRESS_CANCEL: COPYFILE2_MESSAGE_ACTION = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const COPYFILE2_PROGRESS_STOP: COPYFILE2_MESSAGE_ACTION = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const COPYFILE2_PROGRESS_QUIET: COPYFILE2_MESSAGE_ACTION = 3i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const COPYFILE2_PROGRESS_PAUSE: COPYFILE2_MESSAGE_ACTION = 4i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type COPYFILE2_MESSAGE_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const COPYFILE2_CALLBACK_NONE: COPYFILE2_MESSAGE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const COPYFILE2_CALLBACK_CHUNK_STARTED: COPYFILE2_MESSAGE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const COPYFILE2_CALLBACK_CHUNK_FINISHED: COPYFILE2_MESSAGE_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const COPYFILE2_CALLBACK_STREAM_STARTED: COPYFILE2_MESSAGE_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const COPYFILE2_CALLBACK_STREAM_FINISHED: COPYFILE2_MESSAGE_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const COPYFILE2_CALLBACK_POLL_CONTINUE: COPYFILE2_MESSAGE_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const COPYFILE2_CALLBACK_ERROR: COPYFILE2_MESSAGE_TYPE = 6i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const COPYFILE2_CALLBACK_MAX: COPYFILE2_MESSAGE_TYPE = 7i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct CREATEFILE2_EXTENDED_PARAMETERS {
    pub dwSize: u32,
    pub dwFileAttributes: u32,
    pub dwFileFlags: u32,
    pub dwSecurityQosFlags: u32,
    pub lpSecurityAttributes: *mut super::super::Security::SECURITY_ATTRIBUTES,
    pub hTemplateFile: super::super::Foundation::HANDLE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for CREATEFILE2_EXTENDED_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for CREATEFILE2_EXTENDED_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type CREATE_TAPE_PARTITION_METHOD = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TAPE_FIXED_PARTITIONS: CREATE_TAPE_PARTITION_METHOD = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TAPE_INITIATOR_PARTITIONS: CREATE_TAPE_PARTITION_METHOD = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TAPE_SELECT_PARTITIONS: CREATE_TAPE_PARTITION_METHOD = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CRM_PROTOCOL_DYNAMIC_MARSHAL_INFO: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CRM_PROTOCOL_EXPLICIT_MARSHAL_ONLY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CRM_PROTOCOL_MAXIMUM_OPTION: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CSC_CACHE_AUTO_REINT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CSC_CACHE_MANUAL_REINT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CSC_CACHE_NONE: u32 = 48u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CSC_CACHE_VDO: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CSC_MASK: u32 = 48u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CSC_MASK_EXT: u32 = 8240u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CSV_BLOCK_AND_FILE_CACHE_CALLBACK_VERSION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CSV_BLOCK_CACHE_CALLBACK_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsClientRecord: u8 = 3u8;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsContainerActive: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsContainerActivePendingDelete: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsContainerInactive: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsContainerInitializing: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsContainerPendingArchive: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsContainerPendingArchiveAndDelete: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsDataRecord: u8 = 1u8;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsNullRecord: u8 = 0u8;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClfsRestartRecord: u8 = 2u8;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClsContainerActive: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClsContainerActivePendingDelete: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClsContainerInactive: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClsContainerInitializing: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClsContainerPendingArchive: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ClsContainerPendingArchiveAndDelete: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type DEFINE_DOS_DEVICE_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const DDD_RAW_TARGET_PATH: DEFINE_DOS_DEVICE_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const DDD_REMOVE_DEFINITION: DEFINE_DOS_DEVICE_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const DDD_EXACT_MATCH_ON_REMOVE: DEFINE_DOS_DEVICE_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const DDD_NO_BROADCAST_SYSTEM: DEFINE_DOS_DEVICE_FLAGS = 8u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const DDD_LUID_BROADCAST_DRIVE: DEFINE_DOS_DEVICE_FLAGS = 16u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const DISKQUOTA_FILESTATE_INCOMPLETE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const DISKQUOTA_FILESTATE_MASK: u32 = 768u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const DISKQUOTA_FILESTATE_REBUILDING: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const DISKQUOTA_LOGFLAG_USER_LIMIT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const DISKQUOTA_LOGFLAG_USER_THRESHOLD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const DISKQUOTA_STATE_DISABLED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const DISKQUOTA_STATE_ENFORCE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const DISKQUOTA_STATE_MASK: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const DISKQUOTA_STATE_TRACK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type DISKQUOTA_USERNAME_RESOLVE = u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const DISKQUOTA_USERNAME_RESOLVE_ASYNC: DISKQUOTA_USERNAME_RESOLVE = 2u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const DISKQUOTA_USERNAME_RESOLVE_NONE: DISKQUOTA_USERNAME_RESOLVE = 0u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const DISKQUOTA_USERNAME_RESOLVE_SYNC: DISKQUOTA_USERNAME_RESOLVE = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const DISKQUOTA_USER_ACCOUNT_DELETED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const DISKQUOTA_USER_ACCOUNT_INVALID: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const DISKQUOTA_USER_ACCOUNT_RESOLVED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const DISKQUOTA_USER_ACCOUNT_UNAVAILABLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const DISKQUOTA_USER_ACCOUNT_UNKNOWN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const DISKQUOTA_USER_ACCOUNT_UNRESOLVED: u32 = 5u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct DISKQUOTA_USER_INFORMATION {
    pub QuotaUsed: i64,
    pub QuotaThreshold: i64,
    pub QuotaLimit: i64,
}
impl ::core::marker::Copy for DISKQUOTA_USER_INFORMATION {}
impl ::core::clone::Clone for DISKQUOTA_USER_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct DISK_SPACE_INFORMATION {
    pub ActualTotalAllocationUnits: u64,
    pub ActualAvailableAllocationUnits: u64,
    pub ActualPoolUnavailableAllocationUnits: u64,
    pub CallerTotalAllocationUnits: u64,
    pub CallerAvailableAllocationUnits: u64,
    pub CallerPoolUnavailableAllocationUnits: u64,
    pub UsedAllocationUnits: u64,
    pub TotalReservedAllocationUnits: u64,
    pub VolumeStorageReserveAllocationUnits: u64,
    pub AvailableCommittedAllocationUnits: u64,
    pub PoolAvailableAllocationUnits: u64,
    pub SectorsPerAllocationUnit: u32,
    pub BytesPerSector: u32,
}
impl ::core::marker::Copy for DISK_SPACE_INFORMATION {}
impl ::core::clone::Clone for DISK_SPACE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const EA_CONTAINER_NAME: &str = "ContainerName";
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const EA_CONTAINER_SIZE: &str = "ContainerSize";
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct EFS_CERTIFICATE_BLOB {
    pub dwCertEncodingType: u32,
    pub cbData: u32,
    pub pbData: *mut u8,
}
impl ::core::marker::Copy for EFS_CERTIFICATE_BLOB {}
impl ::core::clone::Clone for EFS_CERTIFICATE_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct EFS_COMPATIBILITY_INFO {
    pub EfsVersion: u32,
}
impl ::core::marker::Copy for EFS_COMPATIBILITY_INFO {}
impl ::core::clone::Clone for EFS_COMPATIBILITY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const EFS_COMPATIBILITY_VERSION_NCRYPT_PROTECTOR: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const EFS_COMPATIBILITY_VERSION_PFILE_PROTECTOR: u32 = 6u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct EFS_DECRYPTION_STATUS_INFO {
    pub dwDecryptionError: u32,
    pub dwHashOffset: u32,
    pub cbHash: u32,
}
impl ::core::marker::Copy for EFS_DECRYPTION_STATUS_INFO {}
impl ::core::clone::Clone for EFS_DECRYPTION_STATUS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const EFS_EFS_SUBVER_EFS_CERT: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EFS_ENCRYPTION_STATUS_INFO {
    pub bHasCurrentKey: super::super::Foundation::BOOL,
    pub dwEncryptionError: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EFS_ENCRYPTION_STATUS_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EFS_ENCRYPTION_STATUS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct EFS_HASH_BLOB {
    pub cbData: u32,
    pub pbData: *mut u8,
}
impl ::core::marker::Copy for EFS_HASH_BLOB {}
impl ::core::clone::Clone for EFS_HASH_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct EFS_KEY_INFO {
    pub dwVersion: u32,
    pub Entropy: u32,
    pub Algorithm: u32,
    pub KeyLength: u32,
}
impl ::core::marker::Copy for EFS_KEY_INFO {}
impl ::core::clone::Clone for EFS_KEY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const EFS_METADATA_ADD_USER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const EFS_METADATA_GENERAL_OP: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const EFS_METADATA_REMOVE_USER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const EFS_METADATA_REPLACE_USER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const EFS_PFILE_SUBVER_APPX: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const EFS_PFILE_SUBVER_RMS: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct EFS_PIN_BLOB {
    pub cbPadding: u32,
    pub cbData: u32,
    pub pbData: *mut u8,
}
impl ::core::marker::Copy for EFS_PIN_BLOB {}
impl ::core::clone::Clone for EFS_PIN_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct EFS_RPC_BLOB {
    pub cbData: u32,
    pub pbData: *mut u8,
}
impl ::core::marker::Copy for EFS_RPC_BLOB {}
impl ::core::clone::Clone for EFS_RPC_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const EFS_SUBVER_UNKNOWN: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct EFS_VERSION_INFO {
    pub EfsVersion: u32,
    pub SubVersion: u32,
}
impl ::core::marker::Copy for EFS_VERSION_INFO {}
impl ::core::clone::Clone for EFS_VERSION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
pub struct ENCRYPTED_FILE_METADATA_SIGNATURE {
    pub dwEfsAccessType: u32,
    pub pCertificatesAdded: *mut ENCRYPTION_CERTIFICATE_HASH_LIST,
    pub pEncryptionCertificate: *mut ENCRYPTION_CERTIFICATE,
    pub pEfsStreamSignature: *mut EFS_RPC_BLOB,
}
#[cfg(feature = "Win32_Security")]
impl ::core::marker::Copy for ENCRYPTED_FILE_METADATA_SIGNATURE {}
#[cfg(feature = "Win32_Security")]
impl ::core::clone::Clone for ENCRYPTED_FILE_METADATA_SIGNATURE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
pub struct ENCRYPTION_CERTIFICATE {
    pub cbTotalLength: u32,
    pub pUserSid: *mut super::super::Security::SID,
    pub pCertBlob: *mut EFS_CERTIFICATE_BLOB,
}
#[cfg(feature = "Win32_Security")]
impl ::core::marker::Copy for ENCRYPTION_CERTIFICATE {}
#[cfg(feature = "Win32_Security")]
impl ::core::clone::Clone for ENCRYPTION_CERTIFICATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
pub struct ENCRYPTION_CERTIFICATE_HASH {
    pub cbTotalLength: u32,
    pub pUserSid: *mut super::super::Security::SID,
    pub pHash: *mut EFS_HASH_BLOB,
    pub lpDisplayInformation: ::windows_sys::core::PWSTR,
}
#[cfg(feature = "Win32_Security")]
impl ::core::marker::Copy for ENCRYPTION_CERTIFICATE_HASH {}
#[cfg(feature = "Win32_Security")]
impl ::core::clone::Clone for ENCRYPTION_CERTIFICATE_HASH {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
pub struct ENCRYPTION_CERTIFICATE_HASH_LIST {
    pub nCert_Hash: u32,
    pub pUsers: *mut *mut ENCRYPTION_CERTIFICATE_HASH,
}
#[cfg(feature = "Win32_Security")]
impl ::core::marker::Copy for ENCRYPTION_CERTIFICATE_HASH_LIST {}
#[cfg(feature = "Win32_Security")]
impl ::core::clone::Clone for ENCRYPTION_CERTIFICATE_HASH_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
pub struct ENCRYPTION_CERTIFICATE_LIST {
    pub nUsers: u32,
    pub pUsers: *mut *mut ENCRYPTION_CERTIFICATE,
}
#[cfg(feature = "Win32_Security")]
impl ::core::marker::Copy for ENCRYPTION_CERTIFICATE_LIST {}
#[cfg(feature = "Win32_Security")]
impl ::core::clone::Clone for ENCRYPTION_CERTIFICATE_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
pub struct ENCRYPTION_PROTECTOR {
    pub cbTotalLength: u32,
    pub pUserSid: *mut super::super::Security::SID,
    pub lpProtectorDescriptor: ::windows_sys::core::PWSTR,
}
#[cfg(feature = "Win32_Security")]
impl ::core::marker::Copy for ENCRYPTION_PROTECTOR {}
#[cfg(feature = "Win32_Security")]
impl ::core::clone::Clone for ENCRYPTION_PROTECTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
pub struct ENCRYPTION_PROTECTOR_LIST {
    pub nProtectors: u32,
    pub pProtectors: *mut *mut ENCRYPTION_PROTECTOR,
}
#[cfg(feature = "Win32_Security")]
impl ::core::marker::Copy for ENCRYPTION_PROTECTOR_LIST {}
#[cfg(feature = "Win32_Security")]
impl ::core::clone::Clone for ENCRYPTION_PROTECTOR_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ENLISTMENT_MAXIMUM_OPTION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ENLISTMENT_OBJECT_PATH: &str = "\\Enlistment\\";
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ENLISTMENT_SUPERIOR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type ERASE_TAPE_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TAPE_ERASE_LONG: ERASE_TAPE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TAPE_ERASE_SHORT: ERASE_TAPE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type FCACHE_CREATE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(lpstrname: ::windows_sys::core::PCSTR, lpvdata: *mut ::core::ffi::c_void, cbfilesize: *mut u32, cbfilesizehigh: *mut u32) -> super::super::Foundation::HANDLE>;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type FCACHE_RICHCREATE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(lpstrname: ::windows_sys::core::PCSTR, lpvdata: *mut ::core::ffi::c_void, cbfilesize: *mut u32, cbfilesizehigh: *mut u32, pfdidwescanit: *mut super::super::Foundation::BOOL, pfisstuffed: *mut super::super::Foundation::BOOL, pfstoredwithdots: *mut super::super::Foundation::BOOL, pfstoredwithterminatingdot: *mut super::super::Foundation::BOOL) -> super::super::Foundation::HANDLE>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FH_OVERLAPPED {
    pub Internal: usize,
    pub InternalHigh: usize,
    pub Offset: u32,
    pub OffsetHigh: u32,
    pub hEvent: super::super::Foundation::HANDLE,
    pub pfnCompletion: PFN_IO_COMPLETION,
    pub Reserved1: usize,
    pub Reserved2: usize,
    pub Reserved3: usize,
    pub Reserved4: usize,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FH_OVERLAPPED {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FH_OVERLAPPED {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type FILE_ACCESS_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_READ_DATA: FILE_ACCESS_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_LIST_DIRECTORY: FILE_ACCESS_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_WRITE_DATA: FILE_ACCESS_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_ADD_FILE: FILE_ACCESS_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_APPEND_DATA: FILE_ACCESS_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_ADD_SUBDIRECTORY: FILE_ACCESS_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_CREATE_PIPE_INSTANCE: FILE_ACCESS_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_READ_EA: FILE_ACCESS_FLAGS = 8u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_WRITE_EA: FILE_ACCESS_FLAGS = 16u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_EXECUTE: FILE_ACCESS_FLAGS = 32u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_TRAVERSE: FILE_ACCESS_FLAGS = 32u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_DELETE_CHILD: FILE_ACCESS_FLAGS = 64u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_READ_ATTRIBUTES: FILE_ACCESS_FLAGS = 128u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_WRITE_ATTRIBUTES: FILE_ACCESS_FLAGS = 256u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const READ_CONTROL: FILE_ACCESS_FLAGS = 131072u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SYNCHRONIZE: FILE_ACCESS_FLAGS = 1048576u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const STANDARD_RIGHTS_REQUIRED: FILE_ACCESS_FLAGS = 983040u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const STANDARD_RIGHTS_READ: FILE_ACCESS_FLAGS = 131072u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const STANDARD_RIGHTS_WRITE: FILE_ACCESS_FLAGS = 131072u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const STANDARD_RIGHTS_EXECUTE: FILE_ACCESS_FLAGS = 131072u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const STANDARD_RIGHTS_ALL: FILE_ACCESS_FLAGS = 2031616u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SPECIFIC_RIGHTS_ALL: FILE_ACCESS_FLAGS = 65535u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_ALL_ACCESS: FILE_ACCESS_FLAGS = 2032127u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_GENERIC_READ: FILE_ACCESS_FLAGS = 1179785u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_GENERIC_WRITE: FILE_ACCESS_FLAGS = 1179926u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_GENERIC_EXECUTE: FILE_ACCESS_FLAGS = 1179808u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type FILE_ACTION = u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_ACTION_ADDED: FILE_ACTION = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_ACTION_REMOVED: FILE_ACTION = 2u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_ACTION_MODIFIED: FILE_ACTION = 3u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_ACTION_RENAMED_OLD_NAME: FILE_ACTION = 4u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_ACTION_RENAMED_NEW_NAME: FILE_ACTION = 5u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct FILE_ALIGNMENT_INFO {
    pub AlignmentRequirement: u32,
}
impl ::core::marker::Copy for FILE_ALIGNMENT_INFO {}
impl ::core::clone::Clone for FILE_ALIGNMENT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct FILE_ALLOCATION_INFO {
    pub AllocationSize: i64,
}
impl ::core::marker::Copy for FILE_ALLOCATION_INFO {}
impl ::core::clone::Clone for FILE_ALLOCATION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct FILE_ATTRIBUTE_TAG_INFO {
    pub FileAttributes: u32,
    pub ReparseTag: u32,
}
impl ::core::marker::Copy for FILE_ATTRIBUTE_TAG_INFO {}
impl ::core::clone::Clone for FILE_ATTRIBUTE_TAG_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct FILE_BASIC_INFO {
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub FileAttributes: u32,
}
impl ::core::marker::Copy for FILE_BASIC_INFO {}
impl ::core::clone::Clone for FILE_BASIC_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct FILE_COMPRESSION_INFO {
    pub CompressedFileSize: i64,
    pub CompressionFormat: u16,
    pub CompressionUnitShift: u8,
    pub ChunkShift: u8,
    pub ClusterShift: u8,
    pub Reserved: [u8; 3],
}
impl ::core::marker::Copy for FILE_COMPRESSION_INFO {}
impl ::core::clone::Clone for FILE_COMPRESSION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type FILE_CREATION_DISPOSITION = u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CREATE_NEW: FILE_CREATION_DISPOSITION = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CREATE_ALWAYS: FILE_CREATION_DISPOSITION = 2u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const OPEN_EXISTING: FILE_CREATION_DISPOSITION = 3u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const OPEN_ALWAYS: FILE_CREATION_DISPOSITION = 4u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRUNCATE_EXISTING: FILE_CREATION_DISPOSITION = 5u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type FILE_DEVICE_TYPE = u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_CD_ROM: FILE_DEVICE_TYPE = 2u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_DISK: FILE_DEVICE_TYPE = 7u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_TAPE: FILE_DEVICE_TYPE = 31u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_DEVICE_DVD: FILE_DEVICE_TYPE = 51u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FILE_DISPOSITION_INFO {
    pub DeleteFileA: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FILE_DISPOSITION_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FILE_DISPOSITION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct FILE_END_OF_FILE_INFO {
    pub EndOfFile: i64,
}
impl ::core::marker::Copy for FILE_END_OF_FILE_INFO {}
impl ::core::clone::Clone for FILE_END_OF_FILE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct FILE_EXTENT {
    pub VolumeOffset: u64,
    pub ExtentLength: u64,
}
impl ::core::marker::Copy for FILE_EXTENT {}
impl ::core::clone::Clone for FILE_EXTENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type FILE_FLAGS_AND_ATTRIBUTES = u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_ATTRIBUTE_READONLY: FILE_FLAGS_AND_ATTRIBUTES = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_ATTRIBUTE_HIDDEN: FILE_FLAGS_AND_ATTRIBUTES = 2u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_ATTRIBUTE_SYSTEM: FILE_FLAGS_AND_ATTRIBUTES = 4u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_ATTRIBUTE_DIRECTORY: FILE_FLAGS_AND_ATTRIBUTES = 16u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_ATTRIBUTE_ARCHIVE: FILE_FLAGS_AND_ATTRIBUTES = 32u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_ATTRIBUTE_DEVICE: FILE_FLAGS_AND_ATTRIBUTES = 64u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_ATTRIBUTE_NORMAL: FILE_FLAGS_AND_ATTRIBUTES = 128u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_ATTRIBUTE_TEMPORARY: FILE_FLAGS_AND_ATTRIBUTES = 256u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_ATTRIBUTE_SPARSE_FILE: FILE_FLAGS_AND_ATTRIBUTES = 512u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_ATTRIBUTE_REPARSE_POINT: FILE_FLAGS_AND_ATTRIBUTES = 1024u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_ATTRIBUTE_COMPRESSED: FILE_FLAGS_AND_ATTRIBUTES = 2048u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_ATTRIBUTE_OFFLINE: FILE_FLAGS_AND_ATTRIBUTES = 4096u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_ATTRIBUTE_NOT_CONTENT_INDEXED: FILE_FLAGS_AND_ATTRIBUTES = 8192u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_ATTRIBUTE_ENCRYPTED: FILE_FLAGS_AND_ATTRIBUTES = 16384u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_ATTRIBUTE_INTEGRITY_STREAM: FILE_FLAGS_AND_ATTRIBUTES = 32768u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_ATTRIBUTE_VIRTUAL: FILE_FLAGS_AND_ATTRIBUTES = 65536u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_ATTRIBUTE_NO_SCRUB_DATA: FILE_FLAGS_AND_ATTRIBUTES = 131072u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_ATTRIBUTE_EA: FILE_FLAGS_AND_ATTRIBUTES = 262144u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_ATTRIBUTE_PINNED: FILE_FLAGS_AND_ATTRIBUTES = 524288u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_ATTRIBUTE_UNPINNED: FILE_FLAGS_AND_ATTRIBUTES = 1048576u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_ATTRIBUTE_RECALL_ON_OPEN: FILE_FLAGS_AND_ATTRIBUTES = 262144u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_ATTRIBUTE_RECALL_ON_DATA_ACCESS: FILE_FLAGS_AND_ATTRIBUTES = 4194304u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_FLAG_WRITE_THROUGH: FILE_FLAGS_AND_ATTRIBUTES = 2147483648u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_FLAG_OVERLAPPED: FILE_FLAGS_AND_ATTRIBUTES = 1073741824u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_FLAG_NO_BUFFERING: FILE_FLAGS_AND_ATTRIBUTES = 536870912u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_FLAG_RANDOM_ACCESS: FILE_FLAGS_AND_ATTRIBUTES = 268435456u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_FLAG_SEQUENTIAL_SCAN: FILE_FLAGS_AND_ATTRIBUTES = 134217728u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_FLAG_DELETE_ON_CLOSE: FILE_FLAGS_AND_ATTRIBUTES = 67108864u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_FLAG_BACKUP_SEMANTICS: FILE_FLAGS_AND_ATTRIBUTES = 33554432u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_FLAG_POSIX_SEMANTICS: FILE_FLAGS_AND_ATTRIBUTES = 16777216u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_FLAG_SESSION_AWARE: FILE_FLAGS_AND_ATTRIBUTES = 8388608u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_FLAG_OPEN_REPARSE_POINT: FILE_FLAGS_AND_ATTRIBUTES = 2097152u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_FLAG_OPEN_NO_RECALL: FILE_FLAGS_AND_ATTRIBUTES = 1048576u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_FLAG_FIRST_PIPE_INSTANCE: FILE_FLAGS_AND_ATTRIBUTES = 524288u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const PIPE_ACCESS_DUPLEX: FILE_FLAGS_AND_ATTRIBUTES = 3u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const PIPE_ACCESS_INBOUND: FILE_FLAGS_AND_ATTRIBUTES = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const PIPE_ACCESS_OUTBOUND: FILE_FLAGS_AND_ATTRIBUTES = 2u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SECURITY_ANONYMOUS: FILE_FLAGS_AND_ATTRIBUTES = 0u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SECURITY_IDENTIFICATION: FILE_FLAGS_AND_ATTRIBUTES = 65536u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SECURITY_IMPERSONATION: FILE_FLAGS_AND_ATTRIBUTES = 131072u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SECURITY_DELEGATION: FILE_FLAGS_AND_ATTRIBUTES = 196608u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SECURITY_CONTEXT_TRACKING: FILE_FLAGS_AND_ATTRIBUTES = 262144u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SECURITY_EFFECTIVE_ONLY: FILE_FLAGS_AND_ATTRIBUTES = 524288u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SECURITY_SQOS_PRESENT: FILE_FLAGS_AND_ATTRIBUTES = 1048576u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SECURITY_VALID_SQOS_FLAGS: FILE_FLAGS_AND_ATTRIBUTES = 2031616u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct FILE_FULL_DIR_INFO {
    pub NextEntryOffset: u32,
    pub FileIndex: u32,
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub EndOfFile: i64,
    pub AllocationSize: i64,
    pub FileAttributes: u32,
    pub FileNameLength: u32,
    pub EaSize: u32,
    pub FileName: [u16; 1],
}
impl ::core::marker::Copy for FILE_FULL_DIR_INFO {}
impl ::core::clone::Clone for FILE_FULL_DIR_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct FILE_ID_128 {
    pub Identifier: [u8; 16],
}
impl ::core::marker::Copy for FILE_ID_128 {}
impl ::core::clone::Clone for FILE_ID_128 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct FILE_ID_BOTH_DIR_INFO {
    pub NextEntryOffset: u32,
    pub FileIndex: u32,
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub EndOfFile: i64,
    pub AllocationSize: i64,
    pub FileAttributes: u32,
    pub FileNameLength: u32,
    pub EaSize: u32,
    pub ShortNameLength: i8,
    pub ShortName: [u16; 12],
    pub FileId: i64,
    pub FileName: [u16; 1],
}
impl ::core::marker::Copy for FILE_ID_BOTH_DIR_INFO {}
impl ::core::clone::Clone for FILE_ID_BOTH_DIR_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct FILE_ID_DESCRIPTOR {
    pub dwSize: u32,
    pub Type: FILE_ID_TYPE,
    pub Anonymous: FILE_ID_DESCRIPTOR_0,
}
impl ::core::marker::Copy for FILE_ID_DESCRIPTOR {}
impl ::core::clone::Clone for FILE_ID_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub union FILE_ID_DESCRIPTOR_0 {
    pub FileId: i64,
    pub ObjectId: ::windows_sys::core::GUID,
    pub ExtendedFileId: FILE_ID_128,
}
impl ::core::marker::Copy for FILE_ID_DESCRIPTOR_0 {}
impl ::core::clone::Clone for FILE_ID_DESCRIPTOR_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct FILE_ID_EXTD_DIR_INFO {
    pub NextEntryOffset: u32,
    pub FileIndex: u32,
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub EndOfFile: i64,
    pub AllocationSize: i64,
    pub FileAttributes: u32,
    pub FileNameLength: u32,
    pub EaSize: u32,
    pub ReparsePointTag: u32,
    pub FileId: FILE_ID_128,
    pub FileName: [u16; 1],
}
impl ::core::marker::Copy for FILE_ID_EXTD_DIR_INFO {}
impl ::core::clone::Clone for FILE_ID_EXTD_DIR_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct FILE_ID_INFO {
    pub VolumeSerialNumber: u64,
    pub FileId: FILE_ID_128,
}
impl ::core::marker::Copy for FILE_ID_INFO {}
impl ::core::clone::Clone for FILE_ID_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type FILE_ID_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FileIdType: FILE_ID_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ObjectIdType: FILE_ID_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ExtendedFileIdType: FILE_ID_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const MaximumFileIdType: FILE_ID_TYPE = 3i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct FILE_INFO_2 {
    pub fi2_id: u32,
}
impl ::core::marker::Copy for FILE_INFO_2 {}
impl ::core::clone::Clone for FILE_INFO_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct FILE_INFO_3 {
    pub fi3_id: u32,
    pub fi3_permissions: FILE_INFO_FLAGS_PERMISSIONS,
    pub fi3_num_locks: u32,
    pub fi3_pathname: ::windows_sys::core::PWSTR,
    pub fi3_username: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for FILE_INFO_3 {}
impl ::core::clone::Clone for FILE_INFO_3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type FILE_INFO_BY_HANDLE_CLASS = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FileBasicInfo: FILE_INFO_BY_HANDLE_CLASS = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FileStandardInfo: FILE_INFO_BY_HANDLE_CLASS = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FileNameInfo: FILE_INFO_BY_HANDLE_CLASS = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FileRenameInfo: FILE_INFO_BY_HANDLE_CLASS = 3i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FileDispositionInfo: FILE_INFO_BY_HANDLE_CLASS = 4i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FileAllocationInfo: FILE_INFO_BY_HANDLE_CLASS = 5i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FileEndOfFileInfo: FILE_INFO_BY_HANDLE_CLASS = 6i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FileStreamInfo: FILE_INFO_BY_HANDLE_CLASS = 7i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FileCompressionInfo: FILE_INFO_BY_HANDLE_CLASS = 8i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FileAttributeTagInfo: FILE_INFO_BY_HANDLE_CLASS = 9i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FileIdBothDirectoryInfo: FILE_INFO_BY_HANDLE_CLASS = 10i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FileIdBothDirectoryRestartInfo: FILE_INFO_BY_HANDLE_CLASS = 11i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FileIoPriorityHintInfo: FILE_INFO_BY_HANDLE_CLASS = 12i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FileRemoteProtocolInfo: FILE_INFO_BY_HANDLE_CLASS = 13i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FileFullDirectoryInfo: FILE_INFO_BY_HANDLE_CLASS = 14i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FileFullDirectoryRestartInfo: FILE_INFO_BY_HANDLE_CLASS = 15i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FileStorageInfo: FILE_INFO_BY_HANDLE_CLASS = 16i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FileAlignmentInfo: FILE_INFO_BY_HANDLE_CLASS = 17i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FileIdInfo: FILE_INFO_BY_HANDLE_CLASS = 18i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FileIdExtdDirectoryInfo: FILE_INFO_BY_HANDLE_CLASS = 19i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FileIdExtdDirectoryRestartInfo: FILE_INFO_BY_HANDLE_CLASS = 20i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FileDispositionInfoEx: FILE_INFO_BY_HANDLE_CLASS = 21i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FileRenameInfoEx: FILE_INFO_BY_HANDLE_CLASS = 22i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FileCaseSensitiveInfo: FILE_INFO_BY_HANDLE_CLASS = 23i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FileNormalizedNameInfo: FILE_INFO_BY_HANDLE_CLASS = 24i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const MaximumFileInfoByHandleClass: FILE_INFO_BY_HANDLE_CLASS = 25i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type FILE_INFO_FLAGS_PERMISSIONS = u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const PERM_FILE_READ: FILE_INFO_FLAGS_PERMISSIONS = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const PERM_FILE_WRITE: FILE_INFO_FLAGS_PERMISSIONS = 2u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const PERM_FILE_CREATE: FILE_INFO_FLAGS_PERMISSIONS = 4u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct FILE_IO_PRIORITY_HINT_INFO {
    pub PriorityHint: PRIORITY_HINT,
}
impl ::core::marker::Copy for FILE_IO_PRIORITY_HINT_INFO {}
impl ::core::clone::Clone for FILE_IO_PRIORITY_HINT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type FILE_NAME = u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_NAME_NORMALIZED: FILE_NAME = 0u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_NAME_OPENED: FILE_NAME = 8u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct FILE_NAME_INFO {
    pub FileNameLength: u32,
    pub FileName: [u16; 1],
}
impl ::core::marker::Copy for FILE_NAME_INFO {}
impl ::core::clone::Clone for FILE_NAME_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type FILE_NOTIFY_CHANGE = u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_NOTIFY_CHANGE_FILE_NAME: FILE_NOTIFY_CHANGE = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_NOTIFY_CHANGE_DIR_NAME: FILE_NOTIFY_CHANGE = 2u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_NOTIFY_CHANGE_ATTRIBUTES: FILE_NOTIFY_CHANGE = 4u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_NOTIFY_CHANGE_SIZE: FILE_NOTIFY_CHANGE = 8u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_NOTIFY_CHANGE_LAST_WRITE: FILE_NOTIFY_CHANGE = 16u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_NOTIFY_CHANGE_LAST_ACCESS: FILE_NOTIFY_CHANGE = 32u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_NOTIFY_CHANGE_CREATION: FILE_NOTIFY_CHANGE = 64u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_NOTIFY_CHANGE_SECURITY: FILE_NOTIFY_CHANGE = 256u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct FILE_NOTIFY_EXTENDED_INFORMATION {
    pub NextEntryOffset: u32,
    pub Action: FILE_ACTION,
    pub CreationTime: i64,
    pub LastModificationTime: i64,
    pub LastChangeTime: i64,
    pub LastAccessTime: i64,
    pub AllocatedLength: i64,
    pub FileSize: i64,
    pub FileAttributes: u32,
    pub ReparsePointTag: u32,
    pub FileId: i64,
    pub ParentFileId: i64,
    pub FileNameLength: u32,
    pub FileName: [u16; 1],
}
impl ::core::marker::Copy for FILE_NOTIFY_EXTENDED_INFORMATION {}
impl ::core::clone::Clone for FILE_NOTIFY_EXTENDED_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct FILE_NOTIFY_INFORMATION {
    pub NextEntryOffset: u32,
    pub Action: FILE_ACTION,
    pub FileNameLength: u32,
    pub FileName: [u16; 1],
}
impl ::core::marker::Copy for FILE_NOTIFY_INFORMATION {}
impl ::core::clone::Clone for FILE_NOTIFY_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_PROVIDER_COMPRESSION_LZX: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_PROVIDER_COMPRESSION_XPRESS16K: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_PROVIDER_COMPRESSION_XPRESS4K: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_PROVIDER_COMPRESSION_XPRESS8K: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct FILE_REMOTE_PROTOCOL_INFO {
    pub StructureVersion: u16,
    pub StructureSize: u16,
    pub Protocol: u32,
    pub ProtocolMajorVersion: u16,
    pub ProtocolMinorVersion: u16,
    pub ProtocolRevision: u16,
    pub Reserved: u16,
    pub Flags: u32,
    pub GenericReserved: FILE_REMOTE_PROTOCOL_INFO_0,
    pub ProtocolSpecific: FILE_REMOTE_PROTOCOL_INFO_1,
}
impl ::core::marker::Copy for FILE_REMOTE_PROTOCOL_INFO {}
impl ::core::clone::Clone for FILE_REMOTE_PROTOCOL_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct FILE_REMOTE_PROTOCOL_INFO_0 {
    pub Reserved: [u32; 8],
}
impl ::core::marker::Copy for FILE_REMOTE_PROTOCOL_INFO_0 {}
impl ::core::clone::Clone for FILE_REMOTE_PROTOCOL_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub union FILE_REMOTE_PROTOCOL_INFO_1 {
    pub Smb2: FILE_REMOTE_PROTOCOL_INFO_1_0,
    pub Reserved: [u32; 16],
}
impl ::core::marker::Copy for FILE_REMOTE_PROTOCOL_INFO_1 {}
impl ::core::clone::Clone for FILE_REMOTE_PROTOCOL_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct FILE_REMOTE_PROTOCOL_INFO_1_0 {
    pub Server: FILE_REMOTE_PROTOCOL_INFO_1_0_0,
    pub Share: FILE_REMOTE_PROTOCOL_INFO_1_0_1,
}
impl ::core::marker::Copy for FILE_REMOTE_PROTOCOL_INFO_1_0 {}
impl ::core::clone::Clone for FILE_REMOTE_PROTOCOL_INFO_1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct FILE_REMOTE_PROTOCOL_INFO_1_0_0 {
    pub Capabilities: u32,
}
impl ::core::marker::Copy for FILE_REMOTE_PROTOCOL_INFO_1_0_0 {}
impl ::core::clone::Clone for FILE_REMOTE_PROTOCOL_INFO_1_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct FILE_REMOTE_PROTOCOL_INFO_1_0_1 {
    pub Capabilities: u32,
    pub CachingFlags: u32,
}
impl ::core::marker::Copy for FILE_REMOTE_PROTOCOL_INFO_1_0_1 {}
impl ::core::clone::Clone for FILE_REMOTE_PROTOCOL_INFO_1_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FILE_RENAME_INFO {
    pub Anonymous: FILE_RENAME_INFO_0,
    pub RootDirectory: super::super::Foundation::HANDLE,
    pub FileNameLength: u32,
    pub FileName: [u16; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FILE_RENAME_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FILE_RENAME_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union FILE_RENAME_INFO_0 {
    pub ReplaceIfExists: super::super::Foundation::BOOLEAN,
    pub Flags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FILE_RENAME_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FILE_RENAME_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub union FILE_SEGMENT_ELEMENT {
    pub Buffer: *mut ::core::ffi::c_void,
    pub Alignment: u64,
}
impl ::core::marker::Copy for FILE_SEGMENT_ELEMENT {}
impl ::core::clone::Clone for FILE_SEGMENT_ELEMENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type FILE_SHARE_MODE = u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_SHARE_NONE: FILE_SHARE_MODE = 0u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_SHARE_DELETE: FILE_SHARE_MODE = 4u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_SHARE_READ: FILE_SHARE_MODE = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_SHARE_WRITE: FILE_SHARE_MODE = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FILE_STANDARD_INFO {
    pub AllocationSize: i64,
    pub EndOfFile: i64,
    pub NumberOfLinks: u32,
    pub DeletePending: super::super::Foundation::BOOLEAN,
    pub Directory: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FILE_STANDARD_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FILE_STANDARD_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct FILE_STORAGE_INFO {
    pub LogicalBytesPerSector: u32,
    pub PhysicalBytesPerSectorForAtomicity: u32,
    pub PhysicalBytesPerSectorForPerformance: u32,
    pub FileSystemEffectivePhysicalBytesPerSectorForAtomicity: u32,
    pub Flags: u32,
    pub ByteOffsetForSectorAlignment: u32,
    pub ByteOffsetForPartitionAlignment: u32,
}
impl ::core::marker::Copy for FILE_STORAGE_INFO {}
impl ::core::clone::Clone for FILE_STORAGE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct FILE_STREAM_INFO {
    pub NextEntryOffset: u32,
    pub StreamNameLength: u32,
    pub StreamSize: i64,
    pub StreamAllocationSize: i64,
    pub StreamName: [u16; 1],
}
impl ::core::marker::Copy for FILE_STREAM_INFO {}
impl ::core::clone::Clone for FILE_STREAM_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type FINDEX_INFO_LEVELS = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FindExInfoStandard: FINDEX_INFO_LEVELS = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FindExInfoBasic: FINDEX_INFO_LEVELS = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FindExInfoMaxInfoLevel: FINDEX_INFO_LEVELS = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type FINDEX_SEARCH_OPS = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FindExSearchNameMatch: FINDEX_SEARCH_OPS = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FindExSearchLimitToDirectories: FINDEX_SEARCH_OPS = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FindExSearchLimitToDevices: FINDEX_SEARCH_OPS = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FindExSearchMaxSearchOp: FINDEX_SEARCH_OPS = 3i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type FIND_FIRST_EX_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FIND_FIRST_EX_CASE_SENSITIVE: FIND_FIRST_EX_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FIND_FIRST_EX_LARGE_FETCH: FIND_FIRST_EX_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FIND_FIRST_EX_ON_DISK_ENTRIES_ONLY: FIND_FIRST_EX_FLAGS = 4u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FIO_CONTEXT {
    pub m_dwTempHack: u32,
    pub m_dwSignature: u32,
    pub m_hFile: super::super::Foundation::HANDLE,
    pub m_dwLinesOffset: u32,
    pub m_dwHeaderLength: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FIO_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FIO_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
pub type FindChangeNotificationHandle = isize;
pub type FindFileHandle = isize;
pub type FindFileNameHandle = isize;
pub type FindStreamHandle = isize;
pub type FindVolumeHandle = isize;
pub type FindVolumeMointPointHandle = isize;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type GET_FILEEX_INFO_LEVELS = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const GetFileExInfoStandard: GET_FILEEX_INFO_LEVELS = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const GetFileExMaxInfoLevel: GET_FILEEX_INFO_LEVELS = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type GET_FILE_VERSION_INFO_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_VER_GET_LOCALISED: GET_FILE_VERSION_INFO_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_VER_GET_NEUTRAL: GET_FILE_VERSION_INFO_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_VER_GET_PREFETCHED: GET_FILE_VERSION_INFO_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type GET_TAPE_DRIVE_PARAMETERS_OPERATION = u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const GET_TAPE_DRIVE_INFORMATION: GET_TAPE_DRIVE_PARAMETERS_OPERATION = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const GET_TAPE_MEDIA_INFORMATION: GET_TAPE_DRIVE_PARAMETERS_OPERATION = 0u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct HIORING__ {
    pub unused: i32,
}
impl ::core::marker::Copy for HIORING__ {}
impl ::core::clone::Clone for HIORING__ {
    fn clone(&self) -> Self {
        *self
    }
}
pub type IDiskQuotaControl = *mut ::core::ffi::c_void;
pub type IDiskQuotaEvents = *mut ::core::ffi::c_void;
pub type IDiskQuotaUser = *mut ::core::ffi::c_void;
pub type IDiskQuotaUserBatch = *mut ::core::ffi::c_void;
pub type IEnumDiskQuotaUsers = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const INVALID_FILE_ATTRIBUTES: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const INVALID_SET_FILE_POINTER: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IOCTL_VOLUME_ALLOCATE_BC_STREAM: u32 = 5685312u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IOCTL_VOLUME_BASE: u32 = 86u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IOCTL_VOLUME_BC_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IOCTL_VOLUME_FREE_BC_STREAM: u32 = 5685316u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IOCTL_VOLUME_GET_BC_PROPERTIES: u32 = 5652540u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IOCTL_VOLUME_GET_CSVBLOCKCACHE_CALLBACK: u32 = 5685352u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IOCTL_VOLUME_GET_GPT_ATTRIBUTES: u32 = 5636152u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IOCTL_VOLUME_GET_VOLUME_DISK_EXTENTS: u32 = 5636096u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IOCTL_VOLUME_IS_CLUSTERED: u32 = 5636144u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IOCTL_VOLUME_IS_CSV: u32 = 5636192u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IOCTL_VOLUME_IS_DYNAMIC: u32 = 5636168u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IOCTL_VOLUME_IS_IO_CAPABLE: u32 = 5636116u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IOCTL_VOLUME_IS_OFFLINE: u32 = 5636112u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IOCTL_VOLUME_IS_PARTITION: u32 = 5636136u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IOCTL_VOLUME_LOGICAL_TO_PHYSICAL: u32 = 5636128u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IOCTL_VOLUME_OFFLINE: u32 = 5685260u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IOCTL_VOLUME_ONLINE: u32 = 5685256u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IOCTL_VOLUME_PHYSICAL_TO_LOGICAL: u32 = 5636132u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IOCTL_VOLUME_POST_ONLINE: u32 = 5685348u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IOCTL_VOLUME_PREPARE_FOR_CRITICAL_IO: u32 = 5685324u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IOCTL_VOLUME_PREPARE_FOR_SHRINK: u32 = 5685340u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IOCTL_VOLUME_QUERY_ALLOCATION_HINT: u32 = 5652562u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IOCTL_VOLUME_QUERY_FAILOVER_SET: u32 = 5636120u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IOCTL_VOLUME_QUERY_MINIMUM_SHRINK_SIZE: u32 = 5652568u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IOCTL_VOLUME_QUERY_VOLUME_NUMBER: u32 = 5636124u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IOCTL_VOLUME_READ_PLEX: u32 = 5652526u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IOCTL_VOLUME_SET_GPT_ATTRIBUTES: u32 = 5636148u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IOCTL_VOLUME_SUPPORTS_ONLINE_OFFLINE: u32 = 5636100u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IOCTL_VOLUME_UPDATE_PROPERTIES: u32 = 5636180u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct IORING_BUFFER_INFO {
    pub Address: *mut ::core::ffi::c_void,
    pub Length: u32,
}
impl ::core::marker::Copy for IORING_BUFFER_INFO {}
impl ::core::clone::Clone for IORING_BUFFER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct IORING_BUFFER_REF {
    pub Kind: IORING_REF_KIND,
    pub Buffer: IORING_BUFFER_REF_0,
}
impl ::core::marker::Copy for IORING_BUFFER_REF {}
impl ::core::clone::Clone for IORING_BUFFER_REF {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub union IORING_BUFFER_REF_0 {
    pub Address: *mut ::core::ffi::c_void,
    pub IndexAndOffset: IORING_REGISTERED_BUFFER,
}
impl ::core::marker::Copy for IORING_BUFFER_REF_0 {}
impl ::core::clone::Clone for IORING_BUFFER_REF_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct IORING_CAPABILITIES {
    pub MaxVersion: IORING_VERSION,
    pub MaxSubmissionQueueSize: u32,
    pub MaxCompletionQueueSize: u32,
    pub FeatureFlags: IORING_FEATURE_FLAGS,
}
impl ::core::marker::Copy for IORING_CAPABILITIES {}
impl ::core::clone::Clone for IORING_CAPABILITIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct IORING_CQE {
    pub UserData: usize,
    pub ResultCode: ::windows_sys::core::HRESULT,
    pub Information: usize,
}
impl ::core::marker::Copy for IORING_CQE {}
impl ::core::clone::Clone for IORING_CQE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type IORING_CREATE_ADVISORY_FLAGS = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IORING_CREATE_ADVISORY_FLAGS_NONE: IORING_CREATE_ADVISORY_FLAGS = 0i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct IORING_CREATE_FLAGS {
    pub Required: IORING_CREATE_REQUIRED_FLAGS,
    pub Advisory: IORING_CREATE_ADVISORY_FLAGS,
}
impl ::core::marker::Copy for IORING_CREATE_FLAGS {}
impl ::core::clone::Clone for IORING_CREATE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type IORING_CREATE_REQUIRED_FLAGS = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IORING_CREATE_REQUIRED_FLAGS_NONE: IORING_CREATE_REQUIRED_FLAGS = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type IORING_FEATURE_FLAGS = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IORING_FEATURE_FLAGS_NONE: IORING_FEATURE_FLAGS = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IORING_FEATURE_UM_EMULATION: IORING_FEATURE_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IORING_FEATURE_SET_COMPLETION_EVENT: IORING_FEATURE_FLAGS = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct IORING_HANDLE_REF {
    pub Kind: IORING_REF_KIND,
    pub Handle: IORING_HANDLE_REF_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IORING_HANDLE_REF {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IORING_HANDLE_REF {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union IORING_HANDLE_REF_0 {
    pub Handle: super::super::Foundation::HANDLE,
    pub Index: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IORING_HANDLE_REF_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IORING_HANDLE_REF_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct IORING_INFO {
    pub IoRingVersion: IORING_VERSION,
    pub Flags: IORING_CREATE_FLAGS,
    pub SubmissionQueueSize: u32,
    pub CompletionQueueSize: u32,
}
impl ::core::marker::Copy for IORING_INFO {}
impl ::core::clone::Clone for IORING_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type IORING_OP_CODE = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IORING_OP_NOP: IORING_OP_CODE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IORING_OP_READ: IORING_OP_CODE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IORING_OP_REGISTER_FILES: IORING_OP_CODE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IORING_OP_REGISTER_BUFFERS: IORING_OP_CODE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IORING_OP_CANCEL: IORING_OP_CODE = 4i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type IORING_REF_KIND = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IORING_REF_RAW: IORING_REF_KIND = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IORING_REF_REGISTERED: IORING_REF_KIND = 1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct IORING_REGISTERED_BUFFER {
    pub BufferIndex: u32,
    pub Offset: u32,
}
impl ::core::marker::Copy for IORING_REGISTERED_BUFFER {}
impl ::core::clone::Clone for IORING_REGISTERED_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type IORING_SQE_FLAGS = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IOSQE_FLAGS_NONE: IORING_SQE_FLAGS = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type IORING_VERSION = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IORING_VERSION_INVALID: IORING_VERSION = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IORING_VERSION_1: IORING_VERSION = 1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct KCRM_MARSHAL_HEADER {
    pub VersionMajor: u32,
    pub VersionMinor: u32,
    pub NumProtocols: u32,
    pub Unused: u32,
}
impl ::core::marker::Copy for KCRM_MARSHAL_HEADER {}
impl ::core::clone::Clone for KCRM_MARSHAL_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct KCRM_PROTOCOL_BLOB {
    pub ProtocolId: ::windows_sys::core::GUID,
    pub StaticInfoLength: u32,
    pub TransactionIdInfoLength: u32,
    pub Unused1: u32,
    pub Unused2: u32,
}
impl ::core::marker::Copy for KCRM_PROTOCOL_BLOB {}
impl ::core::clone::Clone for KCRM_PROTOCOL_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct KCRM_TRANSACTION_BLOB {
    pub UOW: ::windows_sys::core::GUID,
    pub TmIdentity: ::windows_sys::core::GUID,
    pub IsolationLevel: u32,
    pub IsolationFlags: u32,
    pub Timeout: u32,
    pub Description: [u16; 64],
}
impl ::core::marker::Copy for KCRM_TRANSACTION_BLOB {}
impl ::core::clone::Clone for KCRM_TRANSACTION_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const KTM_MARSHAL_BLOB_VERSION_MAJOR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const KTM_MARSHAL_BLOB_VERSION_MINOR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type LOCK_FILE_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const LOCKFILE_EXCLUSIVE_LOCK: LOCK_FILE_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const LOCKFILE_FAIL_IMMEDIATELY: LOCK_FILE_FLAGS = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct LOG_MANAGEMENT_CALLBACKS {
    pub CallbackContext: *mut ::core::ffi::c_void,
    pub AdvanceTailCallback: PLOG_TAIL_ADVANCE_CALLBACK,
    pub LogFullHandlerCallback: PLOG_FULL_HANDLER_CALLBACK,
    pub LogUnpinnedCallback: PLOG_UNPINNED_CALLBACK,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LOG_MANAGEMENT_CALLBACKS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LOG_MANAGEMENT_CALLBACKS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const LOG_POLICY_OVERWRITE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const LOG_POLICY_PERSIST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPPROGRESS_ROUTINE = ::core::option::Option<unsafe extern "system" fn(totalfilesize: i64, totalbytestransferred: i64, streamsize: i64, streambytestransferred: i64, dwstreamnumber: u32, dwcallbackreason: LPPROGRESS_ROUTINE_CALLBACK_REASON, hsourcefile: super::super::Foundation::HANDLE, hdestinationfile: super::super::Foundation::HANDLE, lpdata: *const ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type LPPROGRESS_ROUTINE_CALLBACK_REASON = u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CALLBACK_CHUNK_FINISHED: LPPROGRESS_ROUTINE_CALLBACK_REASON = 0u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const CALLBACK_STREAM_SWITCH: LPPROGRESS_ROUTINE_CALLBACK_REASON = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const LZERROR_BADINHANDLE: i32 = -1i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const LZERROR_BADOUTHANDLE: i32 = -2i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const LZERROR_BADVALUE: i32 = -7i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const LZERROR_GLOBALLOC: i32 = -5i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const LZERROR_GLOBLOCK: i32 = -6i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const LZERROR_READ: i32 = -3i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const LZERROR_UNKNOWNALG: i32 = -8i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const LZERROR_WRITE: i32 = -4i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type LZOPENFILE_STYLE = u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const OF_CANCEL: LZOPENFILE_STYLE = 2048u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const OF_CREATE: LZOPENFILE_STYLE = 4096u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const OF_DELETE: LZOPENFILE_STYLE = 512u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const OF_EXIST: LZOPENFILE_STYLE = 16384u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const OF_PARSE: LZOPENFILE_STYLE = 256u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const OF_PROMPT: LZOPENFILE_STYLE = 8192u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const OF_READ: LZOPENFILE_STYLE = 0u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const OF_READWRITE: LZOPENFILE_STYLE = 2u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const OF_REOPEN: LZOPENFILE_STYLE = 32768u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const OF_SHARE_DENY_NONE: LZOPENFILE_STYLE = 64u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const OF_SHARE_DENY_READ: LZOPENFILE_STYLE = 48u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const OF_SHARE_DENY_WRITE: LZOPENFILE_STYLE = 32u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const OF_SHARE_EXCLUSIVE: LZOPENFILE_STYLE = 16u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const OF_WRITE: LZOPENFILE_STYLE = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const OF_SHARE_COMPAT: LZOPENFILE_STYLE = 0u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const OF_VERIFY: LZOPENFILE_STYLE = 1024u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type MAXMEDIALABEL = ::core::option::Option<unsafe extern "system" fn(pmaxsize: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const MAX_RESOURCEMANAGER_DESCRIPTION_LENGTH: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const MAX_SID_SIZE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const MAX_TRANSACTION_DESCRIPTION_LENGTH: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type MOVE_FILE_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const MOVEFILE_COPY_ALLOWED: MOVE_FILE_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const MOVEFILE_CREATE_HARDLINK: MOVE_FILE_FLAGS = 16u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const MOVEFILE_DELAY_UNTIL_REBOOT: MOVE_FILE_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const MOVEFILE_REPLACE_EXISTING: MOVE_FILE_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const MOVEFILE_WRITE_THROUGH: MOVE_FILE_FLAGS = 8u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const MOVEFILE_FAIL_IF_NOT_TRACKABLE: MOVE_FILE_FLAGS = 32u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct MediaLabelInfo {
    pub LabelType: [u16; 64],
    pub LabelIDSize: u32,
    pub LabelID: [u8; 256],
    pub LabelAppDescr: [u16; 256],
}
impl ::core::marker::Copy for MediaLabelInfo {}
impl ::core::clone::Clone for MediaLabelInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct NAME_CACHE_CONTEXT {
    pub m_dwSignature: u32,
}
impl ::core::marker::Copy for NAME_CACHE_CONTEXT {}
impl ::core::clone::Clone for NAME_CACHE_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMSMLI_MAXAPPDESCR: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMSMLI_MAXIDSIZE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMSMLI_MAXTYPE: u32 = 64u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct NTMS_ALLOCATION_INFORMATION {
    pub dwSize: u32,
    pub lpReserved: *mut ::core::ffi::c_void,
    pub AllocatedFrom: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for NTMS_ALLOCATION_INFORMATION {}
impl ::core::clone::Clone for NTMS_ALLOCATION_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_APPLICATIONNAME_LENGTH: u32 = 64u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_ASYNC_IO {
    pub OperationId: ::windows_sys::core::GUID,
    pub EventId: ::windows_sys::core::GUID,
    pub dwOperationType: u32,
    pub dwResult: u32,
    pub dwAsyncState: u32,
    pub hEvent: super::super::Foundation::HANDLE,
    pub bOnStateChange: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NTMS_ASYNC_IO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NTMS_ASYNC_IO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_BARCODE_LENGTH: u32 = 64u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_CHANGERINFORMATIONA {
    pub Number: u32,
    pub ChangerType: ::windows_sys::core::GUID,
    pub szSerialNumber: [super::super::Foundation::CHAR; 32],
    pub szRevision: [super::super::Foundation::CHAR; 32],
    pub szDeviceName: [super::super::Foundation::CHAR; 64],
    pub ScsiPort: u16,
    pub ScsiBus: u16,
    pub ScsiTarget: u16,
    pub ScsiLun: u16,
    pub Library: ::windows_sys::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NTMS_CHANGERINFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NTMS_CHANGERINFORMATIONA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct NTMS_CHANGERINFORMATIONW {
    pub Number: u32,
    pub ChangerType: ::windows_sys::core::GUID,
    pub szSerialNumber: [u16; 32],
    pub szRevision: [u16; 32],
    pub szDeviceName: [u16; 64],
    pub ScsiPort: u16,
    pub ScsiBus: u16,
    pub ScsiTarget: u16,
    pub ScsiLun: u16,
    pub Library: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for NTMS_CHANGERINFORMATIONW {}
impl ::core::clone::Clone for NTMS_CHANGERINFORMATIONW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_CHANGERTYPEINFORMATIONA {
    pub szVendor: [super::super::Foundation::CHAR; 128],
    pub szProduct: [super::super::Foundation::CHAR; 128],
    pub DeviceType: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NTMS_CHANGERTYPEINFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NTMS_CHANGERTYPEINFORMATIONA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct NTMS_CHANGERTYPEINFORMATIONW {
    pub szVendor: [u16; 128],
    pub szProduct: [u16; 128],
    pub DeviceType: u32,
}
impl ::core::marker::Copy for NTMS_CHANGERTYPEINFORMATIONW {}
impl ::core::clone::Clone for NTMS_CHANGERTYPEINFORMATIONW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct NTMS_COMPUTERINFORMATION {
    pub dwLibRequestPurgeTime: u32,
    pub dwOpRequestPurgeTime: u32,
    pub dwLibRequestFlags: u32,
    pub dwOpRequestFlags: u32,
    pub dwMediaPoolPolicy: u32,
}
impl ::core::marker::Copy for NTMS_COMPUTERINFORMATION {}
impl ::core::clone::Clone for NTMS_COMPUTERINFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_COMPUTERNAME_LENGTH: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_DESCRIPTION_LENGTH: u32 = 127u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_DEVICENAME_LENGTH: u32 = 64u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_DRIVEINFORMATIONA {
    pub Number: u32,
    pub State: NtmsDriveState,
    pub DriveType: ::windows_sys::core::GUID,
    pub szDeviceName: [super::super::Foundation::CHAR; 64],
    pub szSerialNumber: [super::super::Foundation::CHAR; 32],
    pub szRevision: [super::super::Foundation::CHAR; 32],
    pub ScsiPort: u16,
    pub ScsiBus: u16,
    pub ScsiTarget: u16,
    pub ScsiLun: u16,
    pub dwMountCount: u32,
    pub LastCleanedTs: super::super::Foundation::SYSTEMTIME,
    pub SavedPartitionId: ::windows_sys::core::GUID,
    pub Library: ::windows_sys::core::GUID,
    pub Reserved: ::windows_sys::core::GUID,
    pub dwDeferDismountDelay: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NTMS_DRIVEINFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NTMS_DRIVEINFORMATIONA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_DRIVEINFORMATIONW {
    pub Number: u32,
    pub State: NtmsDriveState,
    pub DriveType: ::windows_sys::core::GUID,
    pub szDeviceName: [u16; 64],
    pub szSerialNumber: [u16; 32],
    pub szRevision: [u16; 32],
    pub ScsiPort: u16,
    pub ScsiBus: u16,
    pub ScsiTarget: u16,
    pub ScsiLun: u16,
    pub dwMountCount: u32,
    pub LastCleanedTs: super::super::Foundation::SYSTEMTIME,
    pub SavedPartitionId: ::windows_sys::core::GUID,
    pub Library: ::windows_sys::core::GUID,
    pub Reserved: ::windows_sys::core::GUID,
    pub dwDeferDismountDelay: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NTMS_DRIVEINFORMATIONW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NTMS_DRIVEINFORMATIONW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_DRIVETYPEINFORMATIONA {
    pub szVendor: [super::super::Foundation::CHAR; 128],
    pub szProduct: [super::super::Foundation::CHAR; 128],
    pub NumberOfHeads: u32,
    pub DeviceType: FILE_DEVICE_TYPE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NTMS_DRIVETYPEINFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NTMS_DRIVETYPEINFORMATIONA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct NTMS_DRIVETYPEINFORMATIONW {
    pub szVendor: [u16; 128],
    pub szProduct: [u16; 128],
    pub NumberOfHeads: u32,
    pub DeviceType: FILE_DEVICE_TYPE,
}
impl ::core::marker::Copy for NTMS_DRIVETYPEINFORMATIONW {}
impl ::core::clone::Clone for NTMS_DRIVETYPEINFORMATIONW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct NTMS_FILESYSTEM_INFO {
    pub FileSystemType: [u16; 64],
    pub VolumeName: [u16; 256],
    pub SerialNumber: u32,
}
impl ::core::marker::Copy for NTMS_FILESYSTEM_INFO {}
impl ::core::clone::Clone for NTMS_FILESYSTEM_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_I1_LIBRARYINFORMATION {
    pub LibraryType: u32,
    pub CleanerSlot: ::windows_sys::core::GUID,
    pub CleanerSlotDefault: ::windows_sys::core::GUID,
    pub LibrarySupportsDriveCleaning: super::super::Foundation::BOOL,
    pub BarCodeReaderInstalled: super::super::Foundation::BOOL,
    pub InventoryMethod: u32,
    pub dwCleanerUsesRemaining: u32,
    pub FirstDriveNumber: u32,
    pub dwNumberOfDrives: u32,
    pub FirstSlotNumber: u32,
    pub dwNumberOfSlots: u32,
    pub FirstDoorNumber: u32,
    pub dwNumberOfDoors: u32,
    pub FirstPortNumber: u32,
    pub dwNumberOfPorts: u32,
    pub FirstChangerNumber: u32,
    pub dwNumberOfChangers: u32,
    pub dwNumberOfMedia: u32,
    pub dwNumberOfMediaTypes: u32,
    pub dwNumberOfLibRequests: u32,
    pub Reserved: ::windows_sys::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NTMS_I1_LIBRARYINFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NTMS_I1_LIBRARYINFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_I1_LIBREQUESTINFORMATIONA {
    pub OperationCode: u32,
    pub OperationOption: u32,
    pub State: u32,
    pub PartitionId: ::windows_sys::core::GUID,
    pub DriveId: ::windows_sys::core::GUID,
    pub PhysMediaId: ::windows_sys::core::GUID,
    pub Library: ::windows_sys::core::GUID,
    pub SlotId: ::windows_sys::core::GUID,
    pub TimeQueued: super::super::Foundation::SYSTEMTIME,
    pub TimeCompleted: super::super::Foundation::SYSTEMTIME,
    pub szApplication: [super::super::Foundation::CHAR; 64],
    pub szUser: [super::super::Foundation::CHAR; 64],
    pub szComputer: [super::super::Foundation::CHAR; 64],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NTMS_I1_LIBREQUESTINFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NTMS_I1_LIBREQUESTINFORMATIONA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_I1_LIBREQUESTINFORMATIONW {
    pub OperationCode: u32,
    pub OperationOption: u32,
    pub State: u32,
    pub PartitionId: ::windows_sys::core::GUID,
    pub DriveId: ::windows_sys::core::GUID,
    pub PhysMediaId: ::windows_sys::core::GUID,
    pub Library: ::windows_sys::core::GUID,
    pub SlotId: ::windows_sys::core::GUID,
    pub TimeQueued: super::super::Foundation::SYSTEMTIME,
    pub TimeCompleted: super::super::Foundation::SYSTEMTIME,
    pub szApplication: [u16; 64],
    pub szUser: [u16; 64],
    pub szComputer: [u16; 64],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NTMS_I1_LIBREQUESTINFORMATIONW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NTMS_I1_LIBREQUESTINFORMATIONW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_I1_MESSAGE_LENGTH: u32 = 127u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_I1_OBJECTINFORMATIONA {
    pub dwSize: u32,
    pub dwType: u32,
    pub Created: super::super::Foundation::SYSTEMTIME,
    pub Modified: super::super::Foundation::SYSTEMTIME,
    pub ObjectGuid: ::windows_sys::core::GUID,
    pub Enabled: super::super::Foundation::BOOL,
    pub dwOperationalState: u32,
    pub szName: [super::super::Foundation::CHAR; 64],
    pub szDescription: [super::super::Foundation::CHAR; 127],
    pub Info: NTMS_I1_OBJECTINFORMATIONA_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NTMS_I1_OBJECTINFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NTMS_I1_OBJECTINFORMATIONA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union NTMS_I1_OBJECTINFORMATIONA_0 {
    pub Drive: NTMS_DRIVEINFORMATIONA,
    pub DriveType: NTMS_DRIVETYPEINFORMATIONA,
    pub Library: NTMS_I1_LIBRARYINFORMATION,
    pub Changer: NTMS_CHANGERINFORMATIONA,
    pub ChangerType: NTMS_CHANGERTYPEINFORMATIONA,
    pub StorageSlot: NTMS_STORAGESLOTINFORMATION,
    pub IEDoor: NTMS_IEDOORINFORMATION,
    pub IEPort: NTMS_IEPORTINFORMATION,
    pub PhysicalMedia: NTMS_I1_PMIDINFORMATIONA,
    pub LogicalMedia: NTMS_LMIDINFORMATION,
    pub Partition: NTMS_I1_PARTITIONINFORMATIONA,
    pub MediaPool: NTMS_MEDIAPOOLINFORMATION,
    pub MediaType: NTMS_MEDIATYPEINFORMATION,
    pub LibRequest: NTMS_I1_LIBREQUESTINFORMATIONA,
    pub OpRequest: NTMS_I1_OPREQUESTINFORMATIONA,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NTMS_I1_OBJECTINFORMATIONA_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NTMS_I1_OBJECTINFORMATIONA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_I1_OBJECTINFORMATIONW {
    pub dwSize: u32,
    pub dwType: u32,
    pub Created: super::super::Foundation::SYSTEMTIME,
    pub Modified: super::super::Foundation::SYSTEMTIME,
    pub ObjectGuid: ::windows_sys::core::GUID,
    pub Enabled: super::super::Foundation::BOOL,
    pub dwOperationalState: u32,
    pub szName: [u16; 64],
    pub szDescription: [u16; 127],
    pub Info: NTMS_I1_OBJECTINFORMATIONW_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NTMS_I1_OBJECTINFORMATIONW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NTMS_I1_OBJECTINFORMATIONW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union NTMS_I1_OBJECTINFORMATIONW_0 {
    pub Drive: NTMS_DRIVEINFORMATIONW,
    pub DriveType: NTMS_DRIVETYPEINFORMATIONW,
    pub Library: NTMS_I1_LIBRARYINFORMATION,
    pub Changer: NTMS_CHANGERINFORMATIONW,
    pub ChangerType: NTMS_CHANGERTYPEINFORMATIONW,
    pub StorageSlot: NTMS_STORAGESLOTINFORMATION,
    pub IEDoor: NTMS_IEDOORINFORMATION,
    pub IEPort: NTMS_IEPORTINFORMATION,
    pub PhysicalMedia: NTMS_I1_PMIDINFORMATIONW,
    pub LogicalMedia: NTMS_LMIDINFORMATION,
    pub Partition: NTMS_I1_PARTITIONINFORMATIONW,
    pub MediaPool: NTMS_MEDIAPOOLINFORMATION,
    pub MediaType: NTMS_MEDIATYPEINFORMATION,
    pub LibRequest: NTMS_I1_LIBREQUESTINFORMATIONW,
    pub OpRequest: NTMS_I1_OPREQUESTINFORMATIONW,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NTMS_I1_OBJECTINFORMATIONW_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NTMS_I1_OBJECTINFORMATIONW_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_I1_OPREQUESTINFORMATIONA {
    pub Request: u32,
    pub Submitted: super::super::Foundation::SYSTEMTIME,
    pub State: u32,
    pub szMessage: [super::super::Foundation::CHAR; 127],
    pub Arg1Type: u32,
    pub Arg1: ::windows_sys::core::GUID,
    pub Arg2Type: u32,
    pub Arg2: ::windows_sys::core::GUID,
    pub szApplication: [super::super::Foundation::CHAR; 64],
    pub szUser: [super::super::Foundation::CHAR; 64],
    pub szComputer: [super::super::Foundation::CHAR; 64],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NTMS_I1_OPREQUESTINFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NTMS_I1_OPREQUESTINFORMATIONA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_I1_OPREQUESTINFORMATIONW {
    pub Request: u32,
    pub Submitted: super::super::Foundation::SYSTEMTIME,
    pub State: u32,
    pub szMessage: [u16; 127],
    pub Arg1Type: u32,
    pub Arg1: ::windows_sys::core::GUID,
    pub Arg2Type: u32,
    pub Arg2: ::windows_sys::core::GUID,
    pub szApplication: [u16; 64],
    pub szUser: [u16; 64],
    pub szComputer: [u16; 64],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NTMS_I1_OPREQUESTINFORMATIONW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NTMS_I1_OPREQUESTINFORMATIONW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_I1_PARTITIONINFORMATIONA {
    pub PhysicalMedia: ::windows_sys::core::GUID,
    pub LogicalMedia: ::windows_sys::core::GUID,
    pub State: u32,
    pub Side: u16,
    pub dwOmidLabelIdLength: u32,
    pub OmidLabelId: [u8; 255],
    pub szOmidLabelType: [super::super::Foundation::CHAR; 64],
    pub szOmidLabelInfo: [super::super::Foundation::CHAR; 256],
    pub dwMountCount: u32,
    pub dwAllocateCount: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NTMS_I1_PARTITIONINFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NTMS_I1_PARTITIONINFORMATIONA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct NTMS_I1_PARTITIONINFORMATIONW {
    pub PhysicalMedia: ::windows_sys::core::GUID,
    pub LogicalMedia: ::windows_sys::core::GUID,
    pub State: u32,
    pub Side: u16,
    pub dwOmidLabelIdLength: u32,
    pub OmidLabelId: [u8; 255],
    pub szOmidLabelType: [u16; 64],
    pub szOmidLabelInfo: [u16; 256],
    pub dwMountCount: u32,
    pub dwAllocateCount: u32,
}
impl ::core::marker::Copy for NTMS_I1_PARTITIONINFORMATIONW {}
impl ::core::clone::Clone for NTMS_I1_PARTITIONINFORMATIONW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_I1_PMIDINFORMATIONA {
    pub CurrentLibrary: ::windows_sys::core::GUID,
    pub MediaPool: ::windows_sys::core::GUID,
    pub Location: ::windows_sys::core::GUID,
    pub LocationType: u32,
    pub MediaType: ::windows_sys::core::GUID,
    pub HomeSlot: ::windows_sys::core::GUID,
    pub szBarCode: [super::super::Foundation::CHAR; 64],
    pub BarCodeState: u32,
    pub szSequenceNumber: [super::super::Foundation::CHAR; 32],
    pub MediaState: u32,
    pub dwNumberOfPartitions: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NTMS_I1_PMIDINFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NTMS_I1_PMIDINFORMATIONA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct NTMS_I1_PMIDINFORMATIONW {
    pub CurrentLibrary: ::windows_sys::core::GUID,
    pub MediaPool: ::windows_sys::core::GUID,
    pub Location: ::windows_sys::core::GUID,
    pub LocationType: u32,
    pub MediaType: ::windows_sys::core::GUID,
    pub HomeSlot: ::windows_sys::core::GUID,
    pub szBarCode: [u16; 64],
    pub BarCodeState: u32,
    pub szSequenceNumber: [u16; 32],
    pub MediaState: u32,
    pub dwNumberOfPartitions: u32,
}
impl ::core::marker::Copy for NTMS_I1_PMIDINFORMATIONW {}
impl ::core::clone::Clone for NTMS_I1_PMIDINFORMATIONW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct NTMS_IEDOORINFORMATION {
    pub Number: u32,
    pub State: NtmsDoorState,
    pub MaxOpenSecs: u16,
    pub Library: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for NTMS_IEDOORINFORMATION {}
impl ::core::clone::Clone for NTMS_IEDOORINFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct NTMS_IEPORTINFORMATION {
    pub Number: u32,
    pub Content: NtmsPortContent,
    pub Position: NtmsPortPosition,
    pub MaxExtendSecs: u16,
    pub Library: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for NTMS_IEPORTINFORMATION {}
impl ::core::clone::Clone for NTMS_IEPORTINFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_LIBRARYINFORMATION {
    pub LibraryType: NtmsLibraryType,
    pub CleanerSlot: ::windows_sys::core::GUID,
    pub CleanerSlotDefault: ::windows_sys::core::GUID,
    pub LibrarySupportsDriveCleaning: super::super::Foundation::BOOL,
    pub BarCodeReaderInstalled: super::super::Foundation::BOOL,
    pub InventoryMethod: NtmsInventoryMethod,
    pub dwCleanerUsesRemaining: u32,
    pub FirstDriveNumber: u32,
    pub dwNumberOfDrives: u32,
    pub FirstSlotNumber: u32,
    pub dwNumberOfSlots: u32,
    pub FirstDoorNumber: u32,
    pub dwNumberOfDoors: u32,
    pub FirstPortNumber: u32,
    pub dwNumberOfPorts: u32,
    pub FirstChangerNumber: u32,
    pub dwNumberOfChangers: u32,
    pub dwNumberOfMedia: u32,
    pub dwNumberOfMediaTypes: u32,
    pub dwNumberOfLibRequests: u32,
    pub Reserved: ::windows_sys::core::GUID,
    pub AutoRecovery: super::super::Foundation::BOOL,
    pub dwFlags: NtmsLibraryFlags,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NTMS_LIBRARYINFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NTMS_LIBRARYINFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_LIBREQUESTINFORMATIONA {
    pub OperationCode: NtmsLmOperation,
    pub OperationOption: u32,
    pub State: NtmsLmState,
    pub PartitionId: ::windows_sys::core::GUID,
    pub DriveId: ::windows_sys::core::GUID,
    pub PhysMediaId: ::windows_sys::core::GUID,
    pub Library: ::windows_sys::core::GUID,
    pub SlotId: ::windows_sys::core::GUID,
    pub TimeQueued: super::super::Foundation::SYSTEMTIME,
    pub TimeCompleted: super::super::Foundation::SYSTEMTIME,
    pub szApplication: [super::super::Foundation::CHAR; 64],
    pub szUser: [super::super::Foundation::CHAR; 64],
    pub szComputer: [super::super::Foundation::CHAR; 64],
    pub dwErrorCode: u32,
    pub WorkItemId: ::windows_sys::core::GUID,
    pub dwPriority: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NTMS_LIBREQUESTINFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NTMS_LIBREQUESTINFORMATIONA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_LIBREQUESTINFORMATIONW {
    pub OperationCode: NtmsLmOperation,
    pub OperationOption: u32,
    pub State: NtmsLmState,
    pub PartitionId: ::windows_sys::core::GUID,
    pub DriveId: ::windows_sys::core::GUID,
    pub PhysMediaId: ::windows_sys::core::GUID,
    pub Library: ::windows_sys::core::GUID,
    pub SlotId: ::windows_sys::core::GUID,
    pub TimeQueued: super::super::Foundation::SYSTEMTIME,
    pub TimeCompleted: super::super::Foundation::SYSTEMTIME,
    pub szApplication: [u16; 64],
    pub szUser: [u16; 64],
    pub szComputer: [u16; 64],
    pub dwErrorCode: u32,
    pub WorkItemId: ::windows_sys::core::GUID,
    pub dwPriority: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NTMS_LIBREQUESTINFORMATIONW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NTMS_LIBREQUESTINFORMATIONW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct NTMS_LMIDINFORMATION {
    pub MediaPool: ::windows_sys::core::GUID,
    pub dwNumberOfPartitions: u32,
}
impl ::core::marker::Copy for NTMS_LMIDINFORMATION {}
impl ::core::clone::Clone for NTMS_LMIDINFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_MAXATTR_LENGTH: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_MAXATTR_NAMELEN: u32 = 32u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct NTMS_MEDIAPOOLINFORMATION {
    pub PoolType: u32,
    pub MediaType: ::windows_sys::core::GUID,
    pub Parent: ::windows_sys::core::GUID,
    pub AllocationPolicy: u32,
    pub DeallocationPolicy: u32,
    pub dwMaxAllocates: u32,
    pub dwNumberOfPhysicalMedia: u32,
    pub dwNumberOfLogicalMedia: u32,
    pub dwNumberOfMediaPools: u32,
}
impl ::core::marker::Copy for NTMS_MEDIAPOOLINFORMATION {}
impl ::core::clone::Clone for NTMS_MEDIAPOOLINFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct NTMS_MEDIATYPEINFORMATION {
    pub MediaType: u32,
    pub NumberOfSides: u32,
    pub ReadWriteCharacteristics: NtmsReadWriteCharacteristics,
    pub DeviceType: FILE_DEVICE_TYPE,
}
impl ::core::marker::Copy for NTMS_MEDIATYPEINFORMATION {}
impl ::core::clone::Clone for NTMS_MEDIATYPEINFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_MESSAGE_LENGTH: u32 = 256u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct NTMS_MOUNT_INFORMATION {
    pub dwSize: u32,
    pub lpReserved: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for NTMS_MOUNT_INFORMATION {}
impl ::core::clone::Clone for NTMS_MOUNT_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct NTMS_NOTIFICATIONINFORMATION {
    pub dwOperation: NtmsNotificationOperations,
    pub ObjectId: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for NTMS_NOTIFICATIONINFORMATION {}
impl ::core::clone::Clone for NTMS_NOTIFICATIONINFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_OBJECTINFORMATIONA {
    pub dwSize: u32,
    pub dwType: NtmsObjectsTypes,
    pub Created: super::super::Foundation::SYSTEMTIME,
    pub Modified: super::super::Foundation::SYSTEMTIME,
    pub ObjectGuid: ::windows_sys::core::GUID,
    pub Enabled: super::super::Foundation::BOOL,
    pub dwOperationalState: NtmsOperationalState,
    pub szName: [super::super::Foundation::CHAR; 64],
    pub szDescription: [super::super::Foundation::CHAR; 127],
    pub Info: NTMS_OBJECTINFORMATIONA_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NTMS_OBJECTINFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NTMS_OBJECTINFORMATIONA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union NTMS_OBJECTINFORMATIONA_0 {
    pub Drive: NTMS_DRIVEINFORMATIONA,
    pub DriveType: NTMS_DRIVETYPEINFORMATIONA,
    pub Library: NTMS_LIBRARYINFORMATION,
    pub Changer: NTMS_CHANGERINFORMATIONA,
    pub ChangerType: NTMS_CHANGERTYPEINFORMATIONA,
    pub StorageSlot: NTMS_STORAGESLOTINFORMATION,
    pub IEDoor: NTMS_IEDOORINFORMATION,
    pub IEPort: NTMS_IEPORTINFORMATION,
    pub PhysicalMedia: NTMS_PMIDINFORMATIONA,
    pub LogicalMedia: NTMS_LMIDINFORMATION,
    pub Partition: NTMS_PARTITIONINFORMATIONA,
    pub MediaPool: NTMS_MEDIAPOOLINFORMATION,
    pub MediaType: NTMS_MEDIATYPEINFORMATION,
    pub LibRequest: NTMS_LIBREQUESTINFORMATIONA,
    pub OpRequest: NTMS_OPREQUESTINFORMATIONA,
    pub Computer: NTMS_COMPUTERINFORMATION,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NTMS_OBJECTINFORMATIONA_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NTMS_OBJECTINFORMATIONA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_OBJECTINFORMATIONW {
    pub dwSize: u32,
    pub dwType: NtmsObjectsTypes,
    pub Created: super::super::Foundation::SYSTEMTIME,
    pub Modified: super::super::Foundation::SYSTEMTIME,
    pub ObjectGuid: ::windows_sys::core::GUID,
    pub Enabled: super::super::Foundation::BOOL,
    pub dwOperationalState: NtmsOperationalState,
    pub szName: [u16; 64],
    pub szDescription: [u16; 127],
    pub Info: NTMS_OBJECTINFORMATIONW_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NTMS_OBJECTINFORMATIONW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NTMS_OBJECTINFORMATIONW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union NTMS_OBJECTINFORMATIONW_0 {
    pub Drive: NTMS_DRIVEINFORMATIONW,
    pub DriveType: NTMS_DRIVETYPEINFORMATIONW,
    pub Library: NTMS_LIBRARYINFORMATION,
    pub Changer: NTMS_CHANGERINFORMATIONW,
    pub ChangerType: NTMS_CHANGERTYPEINFORMATIONW,
    pub StorageSlot: NTMS_STORAGESLOTINFORMATION,
    pub IEDoor: NTMS_IEDOORINFORMATION,
    pub IEPort: NTMS_IEPORTINFORMATION,
    pub PhysicalMedia: NTMS_PMIDINFORMATIONW,
    pub LogicalMedia: NTMS_LMIDINFORMATION,
    pub Partition: NTMS_PARTITIONINFORMATIONW,
    pub MediaPool: NTMS_MEDIAPOOLINFORMATION,
    pub MediaType: NTMS_MEDIATYPEINFORMATION,
    pub LibRequest: NTMS_LIBREQUESTINFORMATIONW,
    pub OpRequest: NTMS_OPREQUESTINFORMATIONW,
    pub Computer: NTMS_COMPUTERINFORMATION,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NTMS_OBJECTINFORMATIONW_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NTMS_OBJECTINFORMATIONW_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_OBJECTNAME_LENGTH: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_OMIDLABELID_LENGTH: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_OMIDLABELINFO_LENGTH: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_OMIDLABELTYPE_LENGTH: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type NTMS_OMID_TYPE = u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_OMID_TYPE_FILESYSTEM_INFO: NTMS_OMID_TYPE = 2u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_OMID_TYPE_RAW_LABEL: NTMS_OMID_TYPE = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_OPREQUESTINFORMATIONA {
    pub Request: NtmsOpreqCommand,
    pub Submitted: super::super::Foundation::SYSTEMTIME,
    pub State: NtmsOpreqState,
    pub szMessage: [super::super::Foundation::CHAR; 256],
    pub Arg1Type: NtmsObjectsTypes,
    pub Arg1: ::windows_sys::core::GUID,
    pub Arg2Type: NtmsObjectsTypes,
    pub Arg2: ::windows_sys::core::GUID,
    pub szApplication: [super::super::Foundation::CHAR; 64],
    pub szUser: [super::super::Foundation::CHAR; 64],
    pub szComputer: [super::super::Foundation::CHAR; 64],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NTMS_OPREQUESTINFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NTMS_OPREQUESTINFORMATIONA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_OPREQUESTINFORMATIONW {
    pub Request: NtmsOpreqCommand,
    pub Submitted: super::super::Foundation::SYSTEMTIME,
    pub State: NtmsOpreqState,
    pub szMessage: [u16; 256],
    pub Arg1Type: NtmsObjectsTypes,
    pub Arg1: ::windows_sys::core::GUID,
    pub Arg2Type: NtmsObjectsTypes,
    pub Arg2: ::windows_sys::core::GUID,
    pub szApplication: [u16; 64],
    pub szUser: [u16; 64],
    pub szComputer: [u16; 64],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NTMS_OPREQUESTINFORMATIONW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NTMS_OPREQUESTINFORMATIONW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_PARTITIONINFORMATIONA {
    pub PhysicalMedia: ::windows_sys::core::GUID,
    pub LogicalMedia: ::windows_sys::core::GUID,
    pub State: NtmsPartitionState,
    pub Side: u16,
    pub dwOmidLabelIdLength: u32,
    pub OmidLabelId: [u8; 255],
    pub szOmidLabelType: [super::super::Foundation::CHAR; 64],
    pub szOmidLabelInfo: [super::super::Foundation::CHAR; 256],
    pub dwMountCount: u32,
    pub dwAllocateCount: u32,
    pub Capacity: i64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NTMS_PARTITIONINFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NTMS_PARTITIONINFORMATIONA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct NTMS_PARTITIONINFORMATIONW {
    pub PhysicalMedia: ::windows_sys::core::GUID,
    pub LogicalMedia: ::windows_sys::core::GUID,
    pub State: NtmsPartitionState,
    pub Side: u16,
    pub dwOmidLabelIdLength: u32,
    pub OmidLabelId: [u8; 255],
    pub szOmidLabelType: [u16; 64],
    pub szOmidLabelInfo: [u16; 256],
    pub dwMountCount: u32,
    pub dwAllocateCount: u32,
    pub Capacity: i64,
}
impl ::core::marker::Copy for NTMS_PARTITIONINFORMATIONW {}
impl ::core::clone::Clone for NTMS_PARTITIONINFORMATIONW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NTMS_PMIDINFORMATIONA {
    pub CurrentLibrary: ::windows_sys::core::GUID,
    pub MediaPool: ::windows_sys::core::GUID,
    pub Location: ::windows_sys::core::GUID,
    pub LocationType: u32,
    pub MediaType: ::windows_sys::core::GUID,
    pub HomeSlot: ::windows_sys::core::GUID,
    pub szBarCode: [super::super::Foundation::CHAR; 64],
    pub BarCodeState: NtmsBarCodeState,
    pub szSequenceNumber: [super::super::Foundation::CHAR; 32],
    pub MediaState: NtmsMediaState,
    pub dwNumberOfPartitions: u32,
    pub dwMediaTypeCode: u32,
    pub dwDensityCode: u32,
    pub MountedPartition: ::windows_sys::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NTMS_PMIDINFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NTMS_PMIDINFORMATIONA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct NTMS_PMIDINFORMATIONW {
    pub CurrentLibrary: ::windows_sys::core::GUID,
    pub MediaPool: ::windows_sys::core::GUID,
    pub Location: ::windows_sys::core::GUID,
    pub LocationType: u32,
    pub MediaType: ::windows_sys::core::GUID,
    pub HomeSlot: ::windows_sys::core::GUID,
    pub szBarCode: [u16; 64],
    pub BarCodeState: NtmsBarCodeState,
    pub szSequenceNumber: [u16; 32],
    pub MediaState: NtmsMediaState,
    pub dwNumberOfPartitions: u32,
    pub dwMediaTypeCode: u32,
    pub dwDensityCode: u32,
    pub MountedPartition: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for NTMS_PMIDINFORMATIONW {}
impl ::core::clone::Clone for NTMS_PMIDINFORMATIONW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_POOLHIERARCHY_LENGTH: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_PRODUCTNAME_LENGTH: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_REVISION_LENGTH: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_SEQUENCE_LENGTH: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_SERIALNUMBER_LENGTH: u32 = 32u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct NTMS_STORAGESLOTINFORMATION {
    pub Number: u32,
    pub State: u32,
    pub Library: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for NTMS_STORAGESLOTINFORMATION {}
impl ::core::clone::Clone for NTMS_STORAGESLOTINFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_USERNAME_LENGTH: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_VENDORNAME_LENGTH: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type NT_CREATE_FILE_DISPOSITION = u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_SUPERSEDE: NT_CREATE_FILE_DISPOSITION = 0u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_CREATE: NT_CREATE_FILE_DISPOSITION = 2u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_OPEN: NT_CREATE_FILE_DISPOSITION = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_OPEN_IF: NT_CREATE_FILE_DISPOSITION = 3u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_OVERWRITE: NT_CREATE_FILE_DISPOSITION = 4u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_OVERWRITE_IF: NT_CREATE_FILE_DISPOSITION = 5u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type NtmsAccessMask = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_USE_ACCESS: NtmsAccessMask = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_MODIFY_ACCESS: NtmsAccessMask = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_CONTROL_ACCESS: NtmsAccessMask = 4i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type NtmsAllocateOptions = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_ALLOCATE_NEW: NtmsAllocateOptions = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_ALLOCATE_NEXT: NtmsAllocateOptions = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_ALLOCATE_ERROR_IF_UNAVAILABLE: NtmsAllocateOptions = 4i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type NtmsAllocationPolicy = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_ALLOCATE_FROMSCRATCH: NtmsAllocationPolicy = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type NtmsAsyncOperations = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_ASYNCOP_MOUNT: NtmsAsyncOperations = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type NtmsAsyncStatus = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_ASYNCSTATE_QUEUED: NtmsAsyncStatus = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_ASYNCSTATE_WAIT_RESOURCE: NtmsAsyncStatus = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_ASYNCSTATE_WAIT_OPERATOR: NtmsAsyncStatus = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_ASYNCSTATE_INPROCESS: NtmsAsyncStatus = 3i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_ASYNCSTATE_COMPLETE: NtmsAsyncStatus = 4i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type NtmsBarCodeState = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_BARCODESTATE_OK: NtmsBarCodeState = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_BARCODESTATE_UNREADABLE: NtmsBarCodeState = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type NtmsCreateNtmsMediaOptions = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_ERROR_ON_DUPLICATE: NtmsCreateNtmsMediaOptions = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type NtmsCreateOptions = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_OPEN_EXISTING: NtmsCreateOptions = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_CREATE_NEW: NtmsCreateOptions = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_OPEN_ALWAYS: NtmsCreateOptions = 3i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type NtmsDeallocationPolicy = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_DEALLOCATE_TOSCRATCH: NtmsDeallocationPolicy = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type NtmsDismountOptions = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_DISMOUNT_DEFERRED: NtmsDismountOptions = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_DISMOUNT_IMMEDIATE: NtmsDismountOptions = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type NtmsDoorState = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_DOORSTATE_UNKNOWN: NtmsDoorState = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_DOORSTATE_CLOSED: NtmsDoorState = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_DOORSTATE_OPEN: NtmsDoorState = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type NtmsDriveState = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_DRIVESTATE_DISMOUNTED: NtmsDriveState = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_DRIVESTATE_MOUNTED: NtmsDriveState = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_DRIVESTATE_LOADED: NtmsDriveState = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_DRIVESTATE_UNLOADED: NtmsDriveState = 5i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_DRIVESTATE_BEING_CLEANED: NtmsDriveState = 6i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_DRIVESTATE_DISMOUNTABLE: NtmsDriveState = 7i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type NtmsDriveType = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_UNKNOWN_DRIVE: NtmsDriveType = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type NtmsEjectOperation = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_EJECT_START: NtmsEjectOperation = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_EJECT_STOP: NtmsEjectOperation = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_EJECT_QUEUE: NtmsEjectOperation = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_EJECT_FORCE: NtmsEjectOperation = 3i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_EJECT_IMMEDIATE: NtmsEjectOperation = 4i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_EJECT_ASK_USER: NtmsEjectOperation = 5i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type NtmsEnumerateOption = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_ENUM_DEFAULT: NtmsEnumerateOption = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_ENUM_ROOTPOOL: NtmsEnumerateOption = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type NtmsInjectOperation = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_INJECT_START: NtmsInjectOperation = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_INJECT_STOP: NtmsInjectOperation = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_INJECT_RETRACT: NtmsInjectOperation = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_INJECT_STARTMANY: NtmsInjectOperation = 3i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type NtmsInventoryMethod = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_INVENTORY_NONE: NtmsInventoryMethod = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_INVENTORY_FAST: NtmsInventoryMethod = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_INVENTORY_OMID: NtmsInventoryMethod = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_INVENTORY_DEFAULT: NtmsInventoryMethod = 3i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_INVENTORY_SLOT: NtmsInventoryMethod = 4i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_INVENTORY_STOP: NtmsInventoryMethod = 5i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_INVENTORY_MAX: NtmsInventoryMethod = 6i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type NtmsLibRequestFlags = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LIBREQFLAGS_NOAUTOPURGE: NtmsLibRequestFlags = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LIBREQFLAGS_NOFAILEDPURGE: NtmsLibRequestFlags = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type NtmsLibraryFlags = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LIBRARYFLAG_FIXEDOFFLINE: NtmsLibraryFlags = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LIBRARYFLAG_CLEANERPRESENT: NtmsLibraryFlags = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LIBRARYFLAG_AUTODETECTCHANGE: NtmsLibraryFlags = 4i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LIBRARYFLAG_IGNORECLEANERUSESREMAINING: NtmsLibraryFlags = 8i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LIBRARYFLAG_RECOGNIZECLEANERBARCODE: NtmsLibraryFlags = 16i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type NtmsLibraryType = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LIBRARYTYPE_UNKNOWN: NtmsLibraryType = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LIBRARYTYPE_OFFLINE: NtmsLibraryType = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LIBRARYTYPE_ONLINE: NtmsLibraryType = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LIBRARYTYPE_STANDALONE: NtmsLibraryType = 3i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type NtmsLmOperation = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LM_REMOVE: NtmsLmOperation = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LM_DISABLECHANGER: NtmsLmOperation = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LM_DISABLELIBRARY: NtmsLmOperation = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LM_ENABLECHANGER: NtmsLmOperation = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LM_ENABLELIBRARY: NtmsLmOperation = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LM_DISABLEDRIVE: NtmsLmOperation = 3i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LM_ENABLEDRIVE: NtmsLmOperation = 4i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LM_DISABLEMEDIA: NtmsLmOperation = 5i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LM_ENABLEMEDIA: NtmsLmOperation = 6i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LM_UPDATEOMID: NtmsLmOperation = 7i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LM_INVENTORY: NtmsLmOperation = 8i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LM_DOORACCESS: NtmsLmOperation = 9i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LM_EJECT: NtmsLmOperation = 10i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LM_EJECTCLEANER: NtmsLmOperation = 11i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LM_INJECT: NtmsLmOperation = 12i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LM_INJECTCLEANER: NtmsLmOperation = 13i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LM_PROCESSOMID: NtmsLmOperation = 14i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LM_CLEANDRIVE: NtmsLmOperation = 15i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LM_DISMOUNT: NtmsLmOperation = 16i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LM_MOUNT: NtmsLmOperation = 17i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LM_WRITESCRATCH: NtmsLmOperation = 18i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LM_CLASSIFY: NtmsLmOperation = 19i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LM_RESERVECLEANER: NtmsLmOperation = 20i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LM_RELEASECLEANER: NtmsLmOperation = 21i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LM_MAXWORKITEM: NtmsLmOperation = 22i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type NtmsLmState = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LM_QUEUED: NtmsLmState = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LM_INPROCESS: NtmsLmState = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LM_PASSED: NtmsLmState = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LM_FAILED: NtmsLmState = 3i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LM_INVALID: NtmsLmState = 4i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LM_WAITING: NtmsLmState = 5i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LM_DEFERRED: NtmsLmState = 6i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LM_DEFFERED: NtmsLmState = 6i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LM_CANCELLED: NtmsLmState = 7i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LM_STOPPED: NtmsLmState = 8i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type NtmsMediaPoolPolicy = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_POOLPOLICY_PURGEOFFLINESCRATCH: NtmsMediaPoolPolicy = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_POOLPOLICY_KEEPOFFLINEIMPORT: NtmsMediaPoolPolicy = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type NtmsMediaState = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_MEDIASTATE_IDLE: NtmsMediaState = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_MEDIASTATE_INUSE: NtmsMediaState = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_MEDIASTATE_MOUNTED: NtmsMediaState = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_MEDIASTATE_LOADED: NtmsMediaState = 3i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_MEDIASTATE_UNLOADED: NtmsMediaState = 4i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_MEDIASTATE_OPERROR: NtmsMediaState = 5i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_MEDIASTATE_OPREQ: NtmsMediaState = 6i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type NtmsMountOptions = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_MOUNT_READ: NtmsMountOptions = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_MOUNT_WRITE: NtmsMountOptions = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_MOUNT_ERROR_NOT_AVAILABLE: NtmsMountOptions = 4i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_MOUNT_ERROR_IF_UNAVAILABLE: NtmsMountOptions = 4i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_MOUNT_ERROR_OFFLINE: NtmsMountOptions = 8i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_MOUNT_ERROR_IF_OFFLINE: NtmsMountOptions = 8i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_MOUNT_SPECIFIC_DRIVE: NtmsMountOptions = 16i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_MOUNT_NOWAIT: NtmsMountOptions = 32i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type NtmsMountPriority = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_PRIORITY_DEFAULT: NtmsMountPriority = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_PRIORITY_HIGHEST: NtmsMountPriority = 15i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_PRIORITY_HIGH: NtmsMountPriority = 7i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_PRIORITY_NORMAL: NtmsMountPriority = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_PRIORITY_LOW: NtmsMountPriority = -7i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_PRIORITY_LOWEST: NtmsMountPriority = -15i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type NtmsNotificationOperations = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_OBJ_UPDATE: NtmsNotificationOperations = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_OBJ_INSERT: NtmsNotificationOperations = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_OBJ_DELETE: NtmsNotificationOperations = 3i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_EVENT_SIGNAL: NtmsNotificationOperations = 4i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_EVENT_COMPLETE: NtmsNotificationOperations = 5i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type NtmsObjectsTypes = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_UNKNOWN: NtmsObjectsTypes = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_OBJECT: NtmsObjectsTypes = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_CHANGER: NtmsObjectsTypes = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_CHANGER_TYPE: NtmsObjectsTypes = 3i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_COMPUTER: NtmsObjectsTypes = 4i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_DRIVE: NtmsObjectsTypes = 5i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_DRIVE_TYPE: NtmsObjectsTypes = 6i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_IEDOOR: NtmsObjectsTypes = 7i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_IEPORT: NtmsObjectsTypes = 8i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LIBRARY: NtmsObjectsTypes = 9i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LIBREQUEST: NtmsObjectsTypes = 10i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_LOGICAL_MEDIA: NtmsObjectsTypes = 11i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_MEDIA_POOL: NtmsObjectsTypes = 12i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_MEDIA_TYPE: NtmsObjectsTypes = 13i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_PARTITION: NtmsObjectsTypes = 14i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_PHYSICAL_MEDIA: NtmsObjectsTypes = 15i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_STORAGESLOT: NtmsObjectsTypes = 16i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_OPREQUEST: NtmsObjectsTypes = 17i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_UI_DESTINATION: NtmsObjectsTypes = 18i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_NUMBER_OF_OBJECT_TYPES: NtmsObjectsTypes = 19i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type NtmsOpRequestFlags = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_OPREQFLAGS_NOAUTOPURGE: NtmsOpRequestFlags = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_OPREQFLAGS_NOFAILEDPURGE: NtmsOpRequestFlags = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_OPREQFLAGS_NOALERTS: NtmsOpRequestFlags = 16i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_OPREQFLAGS_NOTRAYICON: NtmsOpRequestFlags = 32i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type NtmsOperationalState = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_READY: NtmsOperationalState = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_INITIALIZING: NtmsOperationalState = 10i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_NEEDS_SERVICE: NtmsOperationalState = 20i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_NOT_PRESENT: NtmsOperationalState = 21i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type NtmsOpreqCommand = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_OPREQ_UNKNOWN: NtmsOpreqCommand = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_OPREQ_NEWMEDIA: NtmsOpreqCommand = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_OPREQ_CLEANER: NtmsOpreqCommand = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_OPREQ_DEVICESERVICE: NtmsOpreqCommand = 3i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_OPREQ_MOVEMEDIA: NtmsOpreqCommand = 4i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_OPREQ_MESSAGE: NtmsOpreqCommand = 5i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type NtmsOpreqState = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_OPSTATE_UNKNOWN: NtmsOpreqState = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_OPSTATE_SUBMITTED: NtmsOpreqState = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_OPSTATE_ACTIVE: NtmsOpreqState = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_OPSTATE_INPROGRESS: NtmsOpreqState = 3i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_OPSTATE_REFUSED: NtmsOpreqState = 4i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_OPSTATE_COMPLETE: NtmsOpreqState = 5i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type NtmsPartitionState = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_PARTSTATE_UNKNOWN: NtmsPartitionState = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_PARTSTATE_UNPREPARED: NtmsPartitionState = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_PARTSTATE_INCOMPATIBLE: NtmsPartitionState = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_PARTSTATE_DECOMMISSIONED: NtmsPartitionState = 3i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_PARTSTATE_AVAILABLE: NtmsPartitionState = 4i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_PARTSTATE_ALLOCATED: NtmsPartitionState = 5i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_PARTSTATE_COMPLETE: NtmsPartitionState = 6i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_PARTSTATE_FOREIGN: NtmsPartitionState = 7i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_PARTSTATE_IMPORT: NtmsPartitionState = 8i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_PARTSTATE_RESERVED: NtmsPartitionState = 9i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type NtmsPoolType = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_POOLTYPE_UNKNOWN: NtmsPoolType = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_POOLTYPE_SCRATCH: NtmsPoolType = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_POOLTYPE_FOREIGN: NtmsPoolType = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_POOLTYPE_IMPORT: NtmsPoolType = 3i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_POOLTYPE_APPLICATION: NtmsPoolType = 1000i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type NtmsPortContent = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_PORTCONTENT_UNKNOWN: NtmsPortContent = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_PORTCONTENT_FULL: NtmsPortContent = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_PORTCONTENT_EMPTY: NtmsPortContent = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type NtmsPortPosition = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_PORTPOSITION_UNKNOWN: NtmsPortPosition = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_PORTPOSITION_EXTENDED: NtmsPortPosition = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_PORTPOSITION_RETRACTED: NtmsPortPosition = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type NtmsReadWriteCharacteristics = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_MEDIARW_UNKNOWN: NtmsReadWriteCharacteristics = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_MEDIARW_REWRITABLE: NtmsReadWriteCharacteristics = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_MEDIARW_WRITEONCE: NtmsReadWriteCharacteristics = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_MEDIARW_READONLY: NtmsReadWriteCharacteristics = 3i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type NtmsSessionOptions = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_SESSION_QUERYEXPEDITE: NtmsSessionOptions = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type NtmsSlotState = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_SLOTSTATE_UNKNOWN: NtmsSlotState = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_SLOTSTATE_FULL: NtmsSlotState = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_SLOTSTATE_EMPTY: NtmsSlotState = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_SLOTSTATE_NOTPRESENT: NtmsSlotState = 3i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_SLOTSTATE_NEEDSINVENTORY: NtmsSlotState = 4i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type NtmsUIOperations = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_UIDEST_ADD: NtmsUIOperations = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_UIDEST_DELETE: NtmsUIOperations = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_UIDEST_DELETEALL: NtmsUIOperations = 3i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_UIOPERATION_MAX: NtmsUIOperations = 4i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type NtmsUITypes = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_UITYPE_INVALID: NtmsUITypes = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_UITYPE_INFO: NtmsUITypes = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_UITYPE_REQ: NtmsUITypes = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_UITYPE_ERR: NtmsUITypes = 3i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const NTMS_UITYPE_MAX: NtmsUITypes = 4i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct OFSTRUCT {
    pub cBytes: u8,
    pub fFixedDisk: u8,
    pub nErrCode: u16,
    pub Reserved1: u16,
    pub Reserved2: u16,
    pub szPathName: [super::super::Foundation::CHAR; 128],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OFSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OFSTRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PARTITION_BASIC_DATA_GUID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3956318370, data2: 47589, data3: 17459, data4: [135, 192, 104, 182, 183, 38, 153, 199] };
pub const PARTITION_BSP_GUID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1464029011, data2: 19961, data3: 17849, data4: [142, 158, 35, 112, 240, 6, 69, 124] };
pub const PARTITION_CLUSTER_GUID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3684162473, data2: 2112, data3: 19374, data4: [151, 240, 255, 185, 163, 39, 199, 225] };
pub const PARTITION_DPP_GUID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1464029011, data2: 38091, data3: 17392, data4: [165, 51, 215, 60, 16, 207, 165, 125] };
pub const PARTITION_ENTRY_UNUSED_GUID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 0, data2: 0, data3: 0, data4: [0, 0, 0, 0, 0, 0, 0, 0] };
pub const PARTITION_LDM_DATA_GUID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2946195616, data2: 5169, data3: 20322, data4: [188, 104, 51, 17, 113, 74, 105, 173] };
pub const PARTITION_LDM_METADATA_GUID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1476970666, data2: 32399, data3: 17120, data4: [133, 210, 225, 233, 4, 52, 207, 179] };
pub const PARTITION_LEGACY_BL_GUID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1112318178, data2: 31922, data3: 20409, data4: [129, 67, 197, 42, 153, 57, 139, 198] };
pub const PARTITION_LEGACY_BL_GUID_BACKUP: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1112292972, data2: 55199, data3: 18891, data4: [147, 93, 54, 215, 20, 103, 162, 136] };
pub const PARTITION_MAIN_OS_GUID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1464029011, data2: 36677, data3: 16478, data4: [138, 35, 24, 109, 138, 67, 48, 211] };
pub const PARTITION_MSFT_RECOVERY_GUID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3734289316, data2: 1745, data3: 19776, data4: [161, 106, 191, 213, 1, 121, 214, 172] };
pub const PARTITION_MSFT_RESERVED_GUID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3821658902, data2: 2908, data3: 19896, data4: [129, 125, 249, 45, 240, 2, 21, 174] };
pub const PARTITION_MSFT_SNAPSHOT_GUID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3403541489, data2: 17408, data3: 19944, data4: [177, 3, 18, 17, 125, 207, 60, 207] };
pub const PARTITION_OS_DATA_GUID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1464029011, data2: 9202, data3: 17621, data4: [168, 48, 103, 187, 218, 166, 9, 249] };
pub const PARTITION_PATCH_GUID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2305271430, data2: 38570, data3: 27304, data4: [149, 137, 168, 66, 86, 84, 16, 144] };
pub const PARTITION_PRE_INSTALLED_GUID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1464029011, data2: 32736, data3: 16790, data4: [155, 66, 66, 123, 81, 100, 52, 132] };
pub const PARTITION_SERVICING_FILES_GUID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1464029011, data2: 17198, data3: 16404, data4: [174, 76, 141, 234, 169, 192, 0, 106] };
pub const PARTITION_SERVICING_METADATA_GUID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1464029011, data2: 50833, data3: 18949, data4: [187, 78, 112, 61, 175, 210, 41, 206] };
pub const PARTITION_SERVICING_RESERVE_GUID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1464029011, data2: 19329, data3: 17931, data4: [163, 25, 255, 182, 254, 19, 109, 20] };
pub const PARTITION_SERVICING_STAGING_ROOT_GUID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1464029011, data2: 59469, data3: 20100, data4: [170, 243, 236, 187, 189, 4, 185, 223] };
pub const PARTITION_SPACES_DATA_GUID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3886931124, data2: 56372, data3: 17721, data4: [154, 118, 235, 189, 7, 190, 111, 126] };
pub const PARTITION_SPACES_GUID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3881611151, data2: 63104, data3: 19694, data4: [175, 163, 176, 1, 229, 110, 252, 45] };
pub const PARTITION_SYSTEM_GUID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3240784680, data2: 63519, data3: 4562, data4: [186, 75, 0, 160, 201, 62, 201, 59] };
pub const PARTITION_WINDOWS_SYSTEM_GUID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1464029011, data2: 58339, data3: 17969, data4: [165, 197, 38, 210, 36, 56, 115, 170] };
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type PCLFS_COMPLETION_ROUTINE = ::core::option::Option<unsafe extern "system" fn(pvoverlapped: *mut ::core::ffi::c_void, ulreserved: u32)>;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PCOPYFILE2_PROGRESS_ROUTINE = ::core::option::Option<unsafe extern "system" fn(pmessage: *const COPYFILE2_MESSAGE, pvcallbackcontext: *const ::core::ffi::c_void) -> COPYFILE2_MESSAGE_ACTION>;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type PFE_EXPORT_FUNC = ::core::option::Option<unsafe extern "system" fn(pbdata: *const u8, pvcallbackcontext: *const ::core::ffi::c_void, ullength: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type PFE_IMPORT_FUNC = ::core::option::Option<unsafe extern "system" fn(pbdata: *mut u8, pvcallbackcontext: *const ::core::ffi::c_void, ullength: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_IO_COMPLETION = ::core::option::Option<unsafe extern "system" fn(pcontext: *mut FIO_CONTEXT, lpo: *mut FH_OVERLAPPED, cb: u32, dwcompletionstatus: u32)>;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PLOG_FULL_HANDLER_CALLBACK = ::core::option::Option<unsafe extern "system" fn(hlogfile: super::super::Foundation::HANDLE, dwerror: u32, flogispinned: super::super::Foundation::BOOL, pvclientcontext: *mut ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PLOG_TAIL_ADVANCE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(hlogfile: super::super::Foundation::HANDLE, lsntarget: CLS_LSN, pvclientcontext: *mut ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PLOG_UNPINNED_CALLBACK = ::core::option::Option<unsafe extern "system" fn(hlogfile: super::super::Foundation::HANDLE, pvclientcontext: *mut ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type PREPARE_TAPE_OPERATION = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TAPE_FORMAT: PREPARE_TAPE_OPERATION = 5i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TAPE_LOAD: PREPARE_TAPE_OPERATION = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TAPE_LOCK: PREPARE_TAPE_OPERATION = 3i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TAPE_TENSION: PREPARE_TAPE_OPERATION = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TAPE_UNLOAD: PREPARE_TAPE_OPERATION = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TAPE_UNLOCK: PREPARE_TAPE_OPERATION = 4i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type PRIORITY_HINT = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IoPriorityHintVeryLow: PRIORITY_HINT = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IoPriorityHintLow: PRIORITY_HINT = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const IoPriorityHintNormal: PRIORITY_HINT = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const MaximumIoPriorityHintType: PRIORITY_HINT = 3i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type READ_DIRECTORY_NOTIFY_INFORMATION_CLASS = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ReadDirectoryNotifyInformation: READ_DIRECTORY_NOTIFY_INFORMATION_CLASS = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ReadDirectoryNotifyExtendedInformation: READ_DIRECTORY_NOTIFY_INFORMATION_CLASS = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct REPARSE_GUID_DATA_BUFFER {
    pub ReparseTag: u32,
    pub ReparseDataLength: u16,
    pub Reserved: u16,
    pub ReparseGuid: ::windows_sys::core::GUID,
    pub GenericReparseBuffer: REPARSE_GUID_DATA_BUFFER_0,
}
impl ::core::marker::Copy for REPARSE_GUID_DATA_BUFFER {}
impl ::core::clone::Clone for REPARSE_GUID_DATA_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct REPARSE_GUID_DATA_BUFFER_0 {
    pub DataBuffer: [u8; 1],
}
impl ::core::marker::Copy for REPARSE_GUID_DATA_BUFFER_0 {}
impl ::core::clone::Clone for REPARSE_GUID_DATA_BUFFER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type REPLACE_FILE_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const REPLACEFILE_WRITE_THROUGH: REPLACE_FILE_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const REPLACEFILE_IGNORE_MERGE_ERRORS: REPLACE_FILE_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const REPLACEFILE_IGNORE_ACL_ERRORS: REPLACE_FILE_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const RESOURCE_MANAGER_COMMUNICATION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const RESOURCE_MANAGER_MAXIMUM_OPTION: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const RESOURCE_MANAGER_OBJECT_PATH: &str = "\\ResourceManager\\";
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const RESOURCE_MANAGER_VOLATILE: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVER_ALIAS_INFO_0 {
    pub srvai0_alias: ::windows_sys::core::PWSTR,
    pub srvai0_target: ::windows_sys::core::PWSTR,
    pub srvai0_default: super::super::Foundation::BOOLEAN,
    pub srvai0_reserved: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVER_ALIAS_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVER_ALIAS_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct SERVER_CERTIFICATE_INFO_0 {
    pub srvci0_name: ::windows_sys::core::PWSTR,
    pub srvci0_subject: ::windows_sys::core::PWSTR,
    pub srvci0_issuer: ::windows_sys::core::PWSTR,
    pub srvci0_thumbprint: ::windows_sys::core::PWSTR,
    pub srvci0_friendlyname: ::windows_sys::core::PWSTR,
    pub srvci0_notbefore: ::windows_sys::core::PWSTR,
    pub srvci0_notafter: ::windows_sys::core::PWSTR,
    pub srvci0_storelocation: ::windows_sys::core::PWSTR,
    pub srvci0_storename: ::windows_sys::core::PWSTR,
    pub srvci0_renewalchain: ::windows_sys::core::PWSTR,
    pub srvci0_type: u32,
    pub srvci0_flags: u32,
}
impl ::core::marker::Copy for SERVER_CERTIFICATE_INFO_0 {}
impl ::core::clone::Clone for SERVER_CERTIFICATE_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type SERVER_CERTIFICATE_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const QUIC: SERVER_CERTIFICATE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SESI1_NUM_ELEMENTS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SESI2_NUM_ELEMENTS: u32 = 9u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct SESSION_INFO_0 {
    pub sesi0_cname: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for SESSION_INFO_0 {}
impl ::core::clone::Clone for SESSION_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct SESSION_INFO_1 {
    pub sesi1_cname: ::windows_sys::core::PWSTR,
    pub sesi1_username: ::windows_sys::core::PWSTR,
    pub sesi1_num_opens: u32,
    pub sesi1_time: u32,
    pub sesi1_idle_time: u32,
    pub sesi1_user_flags: SESSION_INFO_USER_FLAGS,
}
impl ::core::marker::Copy for SESSION_INFO_1 {}
impl ::core::clone::Clone for SESSION_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct SESSION_INFO_10 {
    pub sesi10_cname: ::windows_sys::core::PWSTR,
    pub sesi10_username: ::windows_sys::core::PWSTR,
    pub sesi10_time: u32,
    pub sesi10_idle_time: u32,
}
impl ::core::marker::Copy for SESSION_INFO_10 {}
impl ::core::clone::Clone for SESSION_INFO_10 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct SESSION_INFO_2 {
    pub sesi2_cname: ::windows_sys::core::PWSTR,
    pub sesi2_username: ::windows_sys::core::PWSTR,
    pub sesi2_num_opens: u32,
    pub sesi2_time: u32,
    pub sesi2_idle_time: u32,
    pub sesi2_user_flags: SESSION_INFO_USER_FLAGS,
    pub sesi2_cltype_name: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for SESSION_INFO_2 {}
impl ::core::clone::Clone for SESSION_INFO_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct SESSION_INFO_502 {
    pub sesi502_cname: ::windows_sys::core::PWSTR,
    pub sesi502_username: ::windows_sys::core::PWSTR,
    pub sesi502_num_opens: u32,
    pub sesi502_time: u32,
    pub sesi502_idle_time: u32,
    pub sesi502_user_flags: SESSION_INFO_USER_FLAGS,
    pub sesi502_cltype_name: ::windows_sys::core::PWSTR,
    pub sesi502_transport: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for SESSION_INFO_502 {}
impl ::core::clone::Clone for SESSION_INFO_502 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type SESSION_INFO_USER_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SESS_GUEST: SESSION_INFO_USER_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SESS_NOENCRYPTION: SESSION_INFO_USER_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type SET_FILE_POINTER_MOVE_METHOD = u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_BEGIN: SET_FILE_POINTER_MOVE_METHOD = 0u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_CURRENT: SET_FILE_POINTER_MOVE_METHOD = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FILE_END: SET_FILE_POINTER_MOVE_METHOD = 2u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SHARE_CURRENT_USES_PARMNUM: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SHARE_FILE_SD_PARMNUM: u32 = 501u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct SHARE_INFO_0 {
    pub shi0_netname: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for SHARE_INFO_0 {}
impl ::core::clone::Clone for SHARE_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct SHARE_INFO_1 {
    pub shi1_netname: ::windows_sys::core::PWSTR,
    pub shi1_type: SHARE_TYPE,
    pub shi1_remark: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for SHARE_INFO_1 {}
impl ::core::clone::Clone for SHARE_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct SHARE_INFO_1004 {
    pub shi1004_remark: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for SHARE_INFO_1004 {}
impl ::core::clone::Clone for SHARE_INFO_1004 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct SHARE_INFO_1005 {
    pub shi1005_flags: u32,
}
impl ::core::marker::Copy for SHARE_INFO_1005 {}
impl ::core::clone::Clone for SHARE_INFO_1005 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct SHARE_INFO_1006 {
    pub shi1006_max_uses: u32,
}
impl ::core::marker::Copy for SHARE_INFO_1006 {}
impl ::core::clone::Clone for SHARE_INFO_1006 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
pub struct SHARE_INFO_1501 {
    pub shi1501_reserved: u32,
    pub shi1501_security_descriptor: super::super::Security::PSECURITY_DESCRIPTOR,
}
#[cfg(feature = "Win32_Security")]
impl ::core::marker::Copy for SHARE_INFO_1501 {}
#[cfg(feature = "Win32_Security")]
impl ::core::clone::Clone for SHARE_INFO_1501 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct SHARE_INFO_1503 {
    pub shi1503_sharefilter: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for SHARE_INFO_1503 {}
impl ::core::clone::Clone for SHARE_INFO_1503 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct SHARE_INFO_2 {
    pub shi2_netname: ::windows_sys::core::PWSTR,
    pub shi2_type: SHARE_TYPE,
    pub shi2_remark: ::windows_sys::core::PWSTR,
    pub shi2_permissions: SHARE_INFO_PERMISSIONS,
    pub shi2_max_uses: u32,
    pub shi2_current_uses: u32,
    pub shi2_path: ::windows_sys::core::PWSTR,
    pub shi2_passwd: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for SHARE_INFO_2 {}
impl ::core::clone::Clone for SHARE_INFO_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct SHARE_INFO_501 {
    pub shi501_netname: ::windows_sys::core::PWSTR,
    pub shi501_type: SHARE_TYPE,
    pub shi501_remark: ::windows_sys::core::PWSTR,
    pub shi501_flags: u32,
}
impl ::core::marker::Copy for SHARE_INFO_501 {}
impl ::core::clone::Clone for SHARE_INFO_501 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
pub struct SHARE_INFO_502 {
    pub shi502_netname: ::windows_sys::core::PWSTR,
    pub shi502_type: SHARE_TYPE,
    pub shi502_remark: ::windows_sys::core::PWSTR,
    pub shi502_permissions: SHARE_INFO_PERMISSIONS,
    pub shi502_max_uses: u32,
    pub shi502_current_uses: u32,
    pub shi502_path: ::windows_sys::core::PWSTR,
    pub shi502_passwd: ::windows_sys::core::PWSTR,
    pub shi502_reserved: u32,
    pub shi502_security_descriptor: super::super::Security::PSECURITY_DESCRIPTOR,
}
#[cfg(feature = "Win32_Security")]
impl ::core::marker::Copy for SHARE_INFO_502 {}
#[cfg(feature = "Win32_Security")]
impl ::core::clone::Clone for SHARE_INFO_502 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
pub struct SHARE_INFO_503 {
    pub shi503_netname: ::windows_sys::core::PWSTR,
    pub shi503_type: SHARE_TYPE,
    pub shi503_remark: ::windows_sys::core::PWSTR,
    pub shi503_permissions: SHARE_INFO_PERMISSIONS,
    pub shi503_max_uses: u32,
    pub shi503_current_uses: u32,
    pub shi503_path: ::windows_sys::core::PWSTR,
    pub shi503_passwd: ::windows_sys::core::PWSTR,
    pub shi503_servername: ::windows_sys::core::PWSTR,
    pub shi503_reserved: u32,
    pub shi503_security_descriptor: super::super::Security::PSECURITY_DESCRIPTOR,
}
#[cfg(feature = "Win32_Security")]
impl ::core::marker::Copy for SHARE_INFO_503 {}
#[cfg(feature = "Win32_Security")]
impl ::core::clone::Clone for SHARE_INFO_503 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type SHARE_INFO_PERMISSIONS = u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ACCESS_READ: SHARE_INFO_PERMISSIONS = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ACCESS_WRITE: SHARE_INFO_PERMISSIONS = 2u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ACCESS_CREATE: SHARE_INFO_PERMISSIONS = 4u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ACCESS_EXEC: SHARE_INFO_PERMISSIONS = 8u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ACCESS_DELETE: SHARE_INFO_PERMISSIONS = 16u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ACCESS_ATRIB: SHARE_INFO_PERMISSIONS = 32u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ACCESS_PERM: SHARE_INFO_PERMISSIONS = 64u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const ACCESS_ALL: SHARE_INFO_PERMISSIONS = 32768u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SHARE_MAX_USES_PARMNUM: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SHARE_NETNAME_PARMNUM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SHARE_PASSWD_PARMNUM: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SHARE_PATH_PARMNUM: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SHARE_PERMISSIONS_PARMNUM: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SHARE_REMARK_PARMNUM: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SHARE_SERVER_PARMNUM: u32 = 503u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type SHARE_TYPE = u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const STYPE_DISKTREE: SHARE_TYPE = 0u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const STYPE_PRINTQ: SHARE_TYPE = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const STYPE_DEVICE: SHARE_TYPE = 2u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const STYPE_IPC: SHARE_TYPE = 3u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const STYPE_SPECIAL: SHARE_TYPE = 2147483648u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const STYPE_TEMPORARY: SHARE_TYPE = 1073741824u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const STYPE_MASK: SHARE_TYPE = 255u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SHARE_TYPE_PARMNUM: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SHI1005_FLAGS_ACCESS_BASED_DIRECTORY_ENUM: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SHI1005_FLAGS_ALLOW_NAMESPACE_CACHING: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SHI1005_FLAGS_CLUSTER_MANAGED: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SHI1005_FLAGS_COMPRESS_DATA: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SHI1005_FLAGS_DFS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SHI1005_FLAGS_DFS_ROOT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SHI1005_FLAGS_DISABLE_CLIENT_BUFFERING: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SHI1005_FLAGS_ENABLE_CA: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SHI1005_FLAGS_ENABLE_HASH: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SHI1005_FLAGS_ENCRYPT_DATA: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SHI1005_FLAGS_FORCE_LEVELII_OPLOCK: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SHI1005_FLAGS_FORCE_SHARED_DELETE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SHI1005_FLAGS_IDENTITY_REMOTING: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SHI1005_FLAGS_RESERVED: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SHI1005_FLAGS_RESTRICT_EXCLUSIVE_OPENS: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SHI1_NUM_ELEMENTS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SHI2_NUM_ELEMENTS: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SHI_USES_UNLIMITED: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const STATSOPT_CLR: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct STAT_SERVER_0 {
    pub sts0_start: u32,
    pub sts0_fopens: u32,
    pub sts0_devopens: u32,
    pub sts0_jobsqueued: u32,
    pub sts0_sopens: u32,
    pub sts0_stimedout: u32,
    pub sts0_serrorout: u32,
    pub sts0_pwerrors: u32,
    pub sts0_permerrors: u32,
    pub sts0_syserrors: u32,
    pub sts0_bytessent_low: u32,
    pub sts0_bytessent_high: u32,
    pub sts0_bytesrcvd_low: u32,
    pub sts0_bytesrcvd_high: u32,
    pub sts0_avresponse: u32,
    pub sts0_reqbufneed: u32,
    pub sts0_bigbufneed: u32,
}
impl ::core::marker::Copy for STAT_SERVER_0 {}
impl ::core::clone::Clone for STAT_SERVER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct STAT_WORKSTATION_0 {
    pub StatisticsStartTime: i64,
    pub BytesReceived: i64,
    pub SmbsReceived: i64,
    pub PagingReadBytesRequested: i64,
    pub NonPagingReadBytesRequested: i64,
    pub CacheReadBytesRequested: i64,
    pub NetworkReadBytesRequested: i64,
    pub BytesTransmitted: i64,
    pub SmbsTransmitted: i64,
    pub PagingWriteBytesRequested: i64,
    pub NonPagingWriteBytesRequested: i64,
    pub CacheWriteBytesRequested: i64,
    pub NetworkWriteBytesRequested: i64,
    pub InitiallyFailedOperations: u32,
    pub FailedCompletionOperations: u32,
    pub ReadOperations: u32,
    pub RandomReadOperations: u32,
    pub ReadSmbs: u32,
    pub LargeReadSmbs: u32,
    pub SmallReadSmbs: u32,
    pub WriteOperations: u32,
    pub RandomWriteOperations: u32,
    pub WriteSmbs: u32,
    pub LargeWriteSmbs: u32,
    pub SmallWriteSmbs: u32,
    pub RawReadsDenied: u32,
    pub RawWritesDenied: u32,
    pub NetworkErrors: u32,
    pub Sessions: u32,
    pub FailedSessions: u32,
    pub Reconnects: u32,
    pub CoreConnects: u32,
    pub Lanman20Connects: u32,
    pub Lanman21Connects: u32,
    pub LanmanNtConnects: u32,
    pub ServerDisconnects: u32,
    pub HungSessions: u32,
    pub UseCount: u32,
    pub FailedUseCount: u32,
    pub CurrentCommands: u32,
}
impl ::core::marker::Copy for STAT_WORKSTATION_0 {}
impl ::core::clone::Clone for STAT_WORKSTATION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type STORAGE_BUS_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const BusTypeUnknown: STORAGE_BUS_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const BusTypeScsi: STORAGE_BUS_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const BusTypeAtapi: STORAGE_BUS_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const BusTypeAta: STORAGE_BUS_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const BusType1394: STORAGE_BUS_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const BusTypeSsa: STORAGE_BUS_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const BusTypeFibre: STORAGE_BUS_TYPE = 6i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const BusTypeUsb: STORAGE_BUS_TYPE = 7i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const BusTypeRAID: STORAGE_BUS_TYPE = 8i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const BusTypeiScsi: STORAGE_BUS_TYPE = 9i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const BusTypeSas: STORAGE_BUS_TYPE = 10i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const BusTypeSata: STORAGE_BUS_TYPE = 11i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const BusTypeSd: STORAGE_BUS_TYPE = 12i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const BusTypeMmc: STORAGE_BUS_TYPE = 13i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const BusTypeVirtual: STORAGE_BUS_TYPE = 14i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const BusTypeFileBackedVirtual: STORAGE_BUS_TYPE = 15i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const BusTypeSpaces: STORAGE_BUS_TYPE = 16i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const BusTypeNvme: STORAGE_BUS_TYPE = 17i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const BusTypeSCM: STORAGE_BUS_TYPE = 18i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const BusTypeUfs: STORAGE_BUS_TYPE = 19i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const BusTypeMax: STORAGE_BUS_TYPE = 20i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const BusTypeMaxReserved: STORAGE_BUS_TYPE = 127i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type STREAM_INFO_LEVELS = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FindStreamInfoStandard: STREAM_INFO_LEVELS = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const FindStreamInfoMaxInfoLevel: STREAM_INFO_LEVELS = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const STYPE_RESERVED1: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const STYPE_RESERVED2: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const STYPE_RESERVED3: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const STYPE_RESERVED4: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const STYPE_RESERVED5: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const STYPE_RESERVED_ALL: u32 = 1073741568u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type SYMBOLIC_LINK_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SYMBOLIC_LINK_FLAG_DIRECTORY: SYMBOLIC_LINK_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SYMBOLIC_LINK_FLAG_ALLOW_UNPRIVILEGED_CREATE: SYMBOLIC_LINK_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type TAPEMARK_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TAPE_FILEMARKS: TAPEMARK_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TAPE_LONG_FILEMARKS: TAPEMARK_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TAPE_SETMARKS: TAPEMARK_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TAPE_SHORT_FILEMARKS: TAPEMARK_TYPE = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TAPE_ERASE {
    pub Type: ERASE_TAPE_TYPE,
    pub Immediate: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TAPE_ERASE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TAPE_ERASE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct TAPE_GET_POSITION {
    pub Type: TAPE_POSITION_TYPE,
    pub Partition: u32,
    pub Offset: i64,
}
impl ::core::marker::Copy for TAPE_GET_POSITION {}
impl ::core::clone::Clone for TAPE_GET_POSITION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type TAPE_INFORMATION_TYPE = u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SET_TAPE_DRIVE_INFORMATION: TAPE_INFORMATION_TYPE = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const SET_TAPE_MEDIA_INFORMATION: TAPE_INFORMATION_TYPE = 0u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type TAPE_POSITION_METHOD = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TAPE_ABSOLUTE_BLOCK: TAPE_POSITION_METHOD = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TAPE_LOGICAL_BLOCK: TAPE_POSITION_METHOD = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TAPE_REWIND: TAPE_POSITION_METHOD = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TAPE_SPACE_END_OF_DATA: TAPE_POSITION_METHOD = 4i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TAPE_SPACE_FILEMARKS: TAPE_POSITION_METHOD = 6i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TAPE_SPACE_RELATIVE_BLOCKS: TAPE_POSITION_METHOD = 5i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TAPE_SPACE_SEQUENTIAL_FMKS: TAPE_POSITION_METHOD = 7i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TAPE_SPACE_SEQUENTIAL_SMKS: TAPE_POSITION_METHOD = 9i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TAPE_SPACE_SETMARKS: TAPE_POSITION_METHOD = 8i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type TAPE_POSITION_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TAPE_ABSOLUTE_POSITION: TAPE_POSITION_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TAPE_LOGICAL_POSITION: TAPE_POSITION_TYPE = 1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TAPE_PREPARE {
    pub Operation: PREPARE_TAPE_OPERATION,
    pub Immediate: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TAPE_PREPARE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TAPE_PREPARE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TAPE_SET_POSITION {
    pub Method: TAPE_POSITION_METHOD,
    pub Partition: u32,
    pub Offset: i64,
    pub Immediate: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TAPE_SET_POSITION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TAPE_SET_POSITION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TAPE_WRITE_MARKS {
    pub Type: TAPEMARK_TYPE,
    pub Count: u32,
    pub Immediate: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TAPE_WRITE_MARKS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TAPE_WRITE_MARKS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTIONMANAGER_OBJECT_PATH: &str = "\\TransactionManager\\";
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_DO_NOT_PROMOTE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_MANAGER_COMMIT_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_MANAGER_COMMIT_LOWEST: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_MANAGER_COMMIT_SYSTEM_HIVES: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_MANAGER_COMMIT_SYSTEM_VOLUME: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_MANAGER_CORRUPT_FOR_PROGRESS: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_MANAGER_CORRUPT_FOR_RECOVERY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_MANAGER_MAXIMUM_OPTION: u32 = 63u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_MANAGER_VOLATILE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_MAXIMUM_OPTION: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct TRANSACTION_NOTIFICATION {
    pub TransactionKey: *mut ::core::ffi::c_void,
    pub TransactionNotification: u32,
    pub TmVirtualClock: i64,
    pub ArgumentLength: u32,
}
impl ::core::marker::Copy for TRANSACTION_NOTIFICATION {}
impl ::core::clone::Clone for TRANSACTION_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct TRANSACTION_NOTIFICATION_MARSHAL_ARGUMENT {
    pub MarshalCookie: u32,
    pub UOW: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for TRANSACTION_NOTIFICATION_MARSHAL_ARGUMENT {}
impl ::core::clone::Clone for TRANSACTION_NOTIFICATION_MARSHAL_ARGUMENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct TRANSACTION_NOTIFICATION_PROPAGATE_ARGUMENT {
    pub PropagationCookie: u32,
    pub UOW: ::windows_sys::core::GUID,
    pub TmIdentity: ::windows_sys::core::GUID,
    pub BufferLength: u32,
}
impl ::core::marker::Copy for TRANSACTION_NOTIFICATION_PROPAGATE_ARGUMENT {}
impl ::core::clone::Clone for TRANSACTION_NOTIFICATION_PROPAGATE_ARGUMENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct TRANSACTION_NOTIFICATION_RECOVERY_ARGUMENT {
    pub EnlistmentId: ::windows_sys::core::GUID,
    pub UOW: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for TRANSACTION_NOTIFICATION_RECOVERY_ARGUMENT {}
impl ::core::clone::Clone for TRANSACTION_NOTIFICATION_RECOVERY_ARGUMENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct TRANSACTION_NOTIFICATION_SAVEPOINT_ARGUMENT {
    pub SavepointId: u32,
}
impl ::core::marker::Copy for TRANSACTION_NOTIFICATION_SAVEPOINT_ARGUMENT {}
impl ::core::clone::Clone for TRANSACTION_NOTIFICATION_SAVEPOINT_ARGUMENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct TRANSACTION_NOTIFICATION_TM_ONLINE_ARGUMENT {
    pub TmIdentity: ::windows_sys::core::GUID,
    pub Flags: u32,
}
impl ::core::marker::Copy for TRANSACTION_NOTIFICATION_TM_ONLINE_ARGUMENT {}
impl ::core::clone::Clone for TRANSACTION_NOTIFICATION_TM_ONLINE_ARGUMENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_NOTIFICATION_TM_ONLINE_FLAG_IS_CLUSTERED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_NOTIFY_COMMIT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_NOTIFY_COMMIT_COMPLETE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_NOTIFY_COMMIT_FINALIZE: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_NOTIFY_COMMIT_REQUEST: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_NOTIFY_DELEGATE_COMMIT: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_NOTIFY_ENLIST_MASK: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_NOTIFY_ENLIST_PREPREPARE: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_NOTIFY_INDOUBT: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_NOTIFY_LAST_RECOVER: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_NOTIFY_MARSHAL: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_NOTIFY_MASK: u32 = 1073741823u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_NOTIFY_PREPARE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_NOTIFY_PREPARE_COMPLETE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_NOTIFY_PREPREPARE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_NOTIFY_PREPREPARE_COMPLETE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_NOTIFY_PROMOTE: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_NOTIFY_PROMOTE_NEW: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_NOTIFY_PROPAGATE_PULL: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_NOTIFY_PROPAGATE_PUSH: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_NOTIFY_RECOVER: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_NOTIFY_RECOVER_QUERY: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_NOTIFY_REQUEST_OUTCOME: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_NOTIFY_RM_DISCONNECTED: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_NOTIFY_ROLLBACK: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_NOTIFY_ROLLBACK_COMPLETE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_NOTIFY_SINGLE_PHASE_COMMIT: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_NOTIFY_TM_ONLINE: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TRANSACTION_OBJECT_PATH: &str = "\\Transaction\\";
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type TRANSACTION_OUTCOME = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TransactionOutcomeUndetermined: TRANSACTION_OUTCOME = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TransactionOutcomeCommitted: TRANSACTION_OUTCOME = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TransactionOutcomeAborted: TRANSACTION_OUTCOME = 3i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type TXFS_MINIVERSION = u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TXFS_MINIVERSION_COMMITTED_VIEW: TXFS_MINIVERSION = 0u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TXFS_MINIVERSION_DIRTY_VIEW: TXFS_MINIVERSION = 65535u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TXFS_MINIVERSION_DEFAULT_VIEW: TXFS_MINIVERSION = 65534u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct TXF_ID {
    pub Anonymous: TXF_ID_0,
}
impl ::core::marker::Copy for TXF_ID {}
impl ::core::clone::Clone for TXF_ID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct TXF_ID_0 {
    pub LowPart: i64,
    pub HighPart: i64,
}
impl ::core::marker::Copy for TXF_ID_0 {}
impl ::core::clone::Clone for TXF_ID_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct TXF_LOG_RECORD_AFFECTED_FILE {
    pub Version: u16,
    pub RecordLength: u32,
    pub Flags: u32,
    pub TxfFileId: TXF_ID,
    pub KtmGuid: ::windows_sys::core::GUID,
    pub FileNameLength: u32,
    pub FileNameByteOffsetInStructure: u32,
}
impl ::core::marker::Copy for TXF_LOG_RECORD_AFFECTED_FILE {}
impl ::core::clone::Clone for TXF_LOG_RECORD_AFFECTED_FILE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct TXF_LOG_RECORD_BASE {
    pub Version: u16,
    pub RecordType: TXF_LOG_RECORD_TYPE,
    pub RecordLength: u32,
}
impl ::core::marker::Copy for TXF_LOG_RECORD_BASE {}
impl ::core::clone::Clone for TXF_LOG_RECORD_BASE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TXF_LOG_RECORD_GENERIC_TYPE_ABORT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TXF_LOG_RECORD_GENERIC_TYPE_COMMIT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TXF_LOG_RECORD_GENERIC_TYPE_DATA: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TXF_LOG_RECORD_GENERIC_TYPE_PREPARE: u32 = 4u32;
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct TXF_LOG_RECORD_TRUNCATE {
    pub Version: u16,
    pub RecordType: u16,
    pub RecordLength: u32,
    pub Flags: u32,
    pub TxfFileId: TXF_ID,
    pub KtmGuid: ::windows_sys::core::GUID,
    pub NewFileSize: i64,
    pub FileNameLength: u32,
    pub FileNameByteOffsetInStructure: u32,
}
impl ::core::marker::Copy for TXF_LOG_RECORD_TRUNCATE {}
impl ::core::clone::Clone for TXF_LOG_RECORD_TRUNCATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type TXF_LOG_RECORD_TYPE = u16;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TXF_LOG_RECORD_TYPE_AFFECTED_FILE: TXF_LOG_RECORD_TYPE = 4u16;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TXF_LOG_RECORD_TYPE_TRUNCATE: TXF_LOG_RECORD_TYPE = 2u16;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const TXF_LOG_RECORD_TYPE_WRITE: TXF_LOG_RECORD_TYPE = 1u16;
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct TXF_LOG_RECORD_WRITE {
    pub Version: u16,
    pub RecordType: u16,
    pub RecordLength: u32,
    pub Flags: u32,
    pub TxfFileId: TXF_ID,
    pub KtmGuid: ::windows_sys::core::GUID,
    pub ByteOffsetInFile: i64,
    pub NumBytesWritten: u32,
    pub ByteOffsetInStructure: u32,
    pub FileNameLength: u32,
    pub FileNameByteOffsetInStructure: u32,
}
impl ::core::marker::Copy for TXF_LOG_RECORD_WRITE {}
impl ::core::clone::Clone for TXF_LOG_RECORD_WRITE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type VER_FIND_FILE_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VFFF_ISSHAREDFILE: VER_FIND_FILE_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type VER_FIND_FILE_STATUS = u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VFF_CURNEDEST: VER_FIND_FILE_STATUS = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VFF_FILEINUSE: VER_FIND_FILE_STATUS = 2u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VFF_BUFFTOOSMALL: VER_FIND_FILE_STATUS = 4u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type VER_INSTALL_FILE_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VIFF_FORCEINSTALL: VER_INSTALL_FILE_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VIFF_DONTDELETEOLD: VER_INSTALL_FILE_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type VER_INSTALL_FILE_STATUS = u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VIF_TEMPFILE: VER_INSTALL_FILE_STATUS = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VIF_MISMATCH: VER_INSTALL_FILE_STATUS = 2u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VIF_SRCOLD: VER_INSTALL_FILE_STATUS = 4u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VIF_DIFFLANG: VER_INSTALL_FILE_STATUS = 8u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VIF_DIFFCODEPG: VER_INSTALL_FILE_STATUS = 16u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VIF_DIFFTYPE: VER_INSTALL_FILE_STATUS = 32u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VIF_WRITEPROT: VER_INSTALL_FILE_STATUS = 64u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VIF_FILEINUSE: VER_INSTALL_FILE_STATUS = 128u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VIF_OUTOFSPACE: VER_INSTALL_FILE_STATUS = 256u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VIF_ACCESSVIOLATION: VER_INSTALL_FILE_STATUS = 512u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VIF_SHARINGVIOLATION: VER_INSTALL_FILE_STATUS = 1024u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VIF_CANNOTCREATE: VER_INSTALL_FILE_STATUS = 2048u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VIF_CANNOTDELETE: VER_INSTALL_FILE_STATUS = 4096u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VIF_CANNOTRENAME: VER_INSTALL_FILE_STATUS = 8192u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VIF_CANNOTDELETECUR: VER_INSTALL_FILE_STATUS = 16384u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VIF_OUTOFMEMORY: VER_INSTALL_FILE_STATUS = 32768u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VIF_CANNOTREADSRC: VER_INSTALL_FILE_STATUS = 65536u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VIF_CANNOTREADDST: VER_INSTALL_FILE_STATUS = 131072u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VIF_BUFFTOOSMALL: VER_INSTALL_FILE_STATUS = 262144u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VIF_CANNOTLOADLZ32: VER_INSTALL_FILE_STATUS = 524288u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VIF_CANNOTLOADCABINET: VER_INSTALL_FILE_STATUS = 1048576u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct VOLUME_ALLOCATE_BC_STREAM_INPUT {
    pub Version: u32,
    pub RequestsPerPeriod: u32,
    pub Period: u32,
    pub RetryFailures: super::super::Foundation::BOOLEAN,
    pub Discardable: super::super::Foundation::BOOLEAN,
    pub Reserved1: [super::super::Foundation::BOOLEAN; 2],
    pub LowestByteOffset: u64,
    pub HighestByteOffset: u64,
    pub AccessType: u32,
    pub AccessMode: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VOLUME_ALLOCATE_BC_STREAM_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VOLUME_ALLOCATE_BC_STREAM_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct VOLUME_ALLOCATE_BC_STREAM_OUTPUT {
    pub RequestSize: u64,
    pub NumOutStandingRequests: u32,
}
impl ::core::marker::Copy for VOLUME_ALLOCATE_BC_STREAM_OUTPUT {}
impl ::core::clone::Clone for VOLUME_ALLOCATE_BC_STREAM_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct VOLUME_ALLOCATION_HINT_INPUT {
    pub ClusterSize: u32,
    pub NumberOfClusters: u32,
    pub StartingClusterNumber: i64,
}
impl ::core::marker::Copy for VOLUME_ALLOCATION_HINT_INPUT {}
impl ::core::clone::Clone for VOLUME_ALLOCATION_HINT_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct VOLUME_ALLOCATION_HINT_OUTPUT {
    pub Bitmap: [u32; 1],
}
impl ::core::marker::Copy for VOLUME_ALLOCATION_HINT_OUTPUT {}
impl ::core::clone::Clone for VOLUME_ALLOCATION_HINT_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct VOLUME_CRITICAL_IO {
    pub AccessType: u32,
    pub ExtentsCount: u32,
    pub Extents: [FILE_EXTENT; 1],
}
impl ::core::marker::Copy for VOLUME_CRITICAL_IO {}
impl ::core::clone::Clone for VOLUME_CRITICAL_IO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct VOLUME_FAILOVER_SET {
    pub NumberOfDisks: u32,
    pub DiskNumbers: [u32; 1],
}
impl ::core::marker::Copy for VOLUME_FAILOVER_SET {}
impl ::core::clone::Clone for VOLUME_FAILOVER_SET {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct VOLUME_GET_BC_PROPERTIES_INPUT {
    pub Version: u32,
    pub Reserved1: u32,
    pub LowestByteOffset: u64,
    pub HighestByteOffset: u64,
    pub AccessType: u32,
    pub AccessMode: u32,
}
impl ::core::marker::Copy for VOLUME_GET_BC_PROPERTIES_INPUT {}
impl ::core::clone::Clone for VOLUME_GET_BC_PROPERTIES_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct VOLUME_GET_BC_PROPERTIES_OUTPUT {
    pub MaximumRequestsPerPeriod: u32,
    pub MinimumPeriod: u32,
    pub MaximumRequestSize: u64,
    pub EstimatedTimePerRequest: u32,
    pub NumOutStandingRequests: u32,
    pub RequestSize: u64,
}
impl ::core::marker::Copy for VOLUME_GET_BC_PROPERTIES_OUTPUT {}
impl ::core::clone::Clone for VOLUME_GET_BC_PROPERTIES_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct VOLUME_LOGICAL_OFFSET {
    pub LogicalOffset: i64,
}
impl ::core::marker::Copy for VOLUME_LOGICAL_OFFSET {}
impl ::core::clone::Clone for VOLUME_LOGICAL_OFFSET {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct VOLUME_NUMBER {
    pub VolumeNumber: u32,
    pub VolumeManagerName: [u16; 8],
}
impl ::core::marker::Copy for VOLUME_NUMBER {}
impl ::core::clone::Clone for VOLUME_NUMBER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct VOLUME_PHYSICAL_OFFSET {
    pub DiskNumber: u32,
    pub Offset: i64,
}
impl ::core::marker::Copy for VOLUME_PHYSICAL_OFFSET {}
impl ::core::clone::Clone for VOLUME_PHYSICAL_OFFSET {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct VOLUME_PHYSICAL_OFFSETS {
    pub NumberOfPhysicalOffsets: u32,
    pub PhysicalOffset: [VOLUME_PHYSICAL_OFFSET; 1],
}
impl ::core::marker::Copy for VOLUME_PHYSICAL_OFFSETS {}
impl ::core::clone::Clone for VOLUME_PHYSICAL_OFFSETS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct VOLUME_READ_PLEX_INPUT {
    pub ByteOffset: i64,
    pub Length: u32,
    pub PlexNumber: u32,
}
impl ::core::marker::Copy for VOLUME_READ_PLEX_INPUT {}
impl ::core::clone::Clone for VOLUME_READ_PLEX_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct VOLUME_SET_GPT_ATTRIBUTES_INFORMATION {
    pub GptAttributes: u64,
    pub RevertOnClose: super::super::Foundation::BOOLEAN,
    pub ApplyToAllConnectedVolumes: super::super::Foundation::BOOLEAN,
    pub Reserved1: u16,
    pub Reserved2: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VOLUME_SET_GPT_ATTRIBUTES_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VOLUME_SET_GPT_ATTRIBUTES_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct VOLUME_SHRINK_INFO {
    pub VolumeSize: u64,
}
impl ::core::marker::Copy for VOLUME_SHRINK_INFO {}
impl ::core::clone::Clone for VOLUME_SHRINK_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VS_FFI_FILEFLAGSMASK: i32 = 63i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VS_FFI_SIGNATURE: i32 = -17890115i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VS_FFI_STRUCVERSION: i32 = 65536i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct VS_FIXEDFILEINFO {
    pub dwSignature: u32,
    pub dwStrucVersion: u32,
    pub dwFileVersionMS: u32,
    pub dwFileVersionLS: u32,
    pub dwProductVersionMS: u32,
    pub dwProductVersionLS: u32,
    pub dwFileFlagsMask: u32,
    pub dwFileFlags: VS_FIXEDFILEINFO_FILE_FLAGS,
    pub dwFileOS: VS_FIXEDFILEINFO_FILE_OS,
    pub dwFileType: VS_FIXEDFILEINFO_FILE_TYPE,
    pub dwFileSubtype: VS_FIXEDFILEINFO_FILE_SUBTYPE,
    pub dwFileDateMS: u32,
    pub dwFileDateLS: u32,
}
impl ::core::marker::Copy for VS_FIXEDFILEINFO {}
impl ::core::clone::Clone for VS_FIXEDFILEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type VS_FIXEDFILEINFO_FILE_FLAGS = u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VS_FF_DEBUG: VS_FIXEDFILEINFO_FILE_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VS_FF_PRERELEASE: VS_FIXEDFILEINFO_FILE_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VS_FF_PATCHED: VS_FIXEDFILEINFO_FILE_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VS_FF_PRIVATEBUILD: VS_FIXEDFILEINFO_FILE_FLAGS = 8u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VS_FF_INFOINFERRED: VS_FIXEDFILEINFO_FILE_FLAGS = 16u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VS_FF_SPECIALBUILD: VS_FIXEDFILEINFO_FILE_FLAGS = 32u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type VS_FIXEDFILEINFO_FILE_OS = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VOS_UNKNOWN: VS_FIXEDFILEINFO_FILE_OS = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VOS_DOS: VS_FIXEDFILEINFO_FILE_OS = 65536i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VOS_OS216: VS_FIXEDFILEINFO_FILE_OS = 131072i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VOS_OS232: VS_FIXEDFILEINFO_FILE_OS = 196608i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VOS_NT: VS_FIXEDFILEINFO_FILE_OS = 262144i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VOS_WINCE: VS_FIXEDFILEINFO_FILE_OS = 327680i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VOS__BASE: VS_FIXEDFILEINFO_FILE_OS = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VOS__WINDOWS16: VS_FIXEDFILEINFO_FILE_OS = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VOS__PM16: VS_FIXEDFILEINFO_FILE_OS = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VOS__PM32: VS_FIXEDFILEINFO_FILE_OS = 3i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VOS__WINDOWS32: VS_FIXEDFILEINFO_FILE_OS = 4i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VOS_DOS_WINDOWS16: VS_FIXEDFILEINFO_FILE_OS = 65537i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VOS_DOS_WINDOWS32: VS_FIXEDFILEINFO_FILE_OS = 65540i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VOS_OS216_PM16: VS_FIXEDFILEINFO_FILE_OS = 131074i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VOS_OS232_PM32: VS_FIXEDFILEINFO_FILE_OS = 196611i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VOS_NT_WINDOWS32: VS_FIXEDFILEINFO_FILE_OS = 262148i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type VS_FIXEDFILEINFO_FILE_SUBTYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VFT2_UNKNOWN: VS_FIXEDFILEINFO_FILE_SUBTYPE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VFT2_DRV_PRINTER: VS_FIXEDFILEINFO_FILE_SUBTYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VFT2_DRV_KEYBOARD: VS_FIXEDFILEINFO_FILE_SUBTYPE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VFT2_DRV_LANGUAGE: VS_FIXEDFILEINFO_FILE_SUBTYPE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VFT2_DRV_DISPLAY: VS_FIXEDFILEINFO_FILE_SUBTYPE = 4i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VFT2_DRV_MOUSE: VS_FIXEDFILEINFO_FILE_SUBTYPE = 5i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VFT2_DRV_NETWORK: VS_FIXEDFILEINFO_FILE_SUBTYPE = 6i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VFT2_DRV_SYSTEM: VS_FIXEDFILEINFO_FILE_SUBTYPE = 7i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VFT2_DRV_INSTALLABLE: VS_FIXEDFILEINFO_FILE_SUBTYPE = 8i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VFT2_DRV_SOUND: VS_FIXEDFILEINFO_FILE_SUBTYPE = 9i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VFT2_DRV_COMM: VS_FIXEDFILEINFO_FILE_SUBTYPE = 10i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VFT2_DRV_INPUTMETHOD: VS_FIXEDFILEINFO_FILE_SUBTYPE = 11i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VFT2_DRV_VERSIONED_PRINTER: VS_FIXEDFILEINFO_FILE_SUBTYPE = 12i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VFT2_FONT_RASTER: VS_FIXEDFILEINFO_FILE_SUBTYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VFT2_FONT_VECTOR: VS_FIXEDFILEINFO_FILE_SUBTYPE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VFT2_FONT_TRUETYPE: VS_FIXEDFILEINFO_FILE_SUBTYPE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type VS_FIXEDFILEINFO_FILE_TYPE = i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VFT_UNKNOWN: VS_FIXEDFILEINFO_FILE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VFT_APP: VS_FIXEDFILEINFO_FILE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VFT_DLL: VS_FIXEDFILEINFO_FILE_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VFT_DRV: VS_FIXEDFILEINFO_FILE_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VFT_FONT: VS_FIXEDFILEINFO_FILE_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VFT_VXD: VS_FIXEDFILEINFO_FILE_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VFT_STATIC_LIB: VS_FIXEDFILEINFO_FILE_TYPE = 7i32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VS_USER_DEFINED: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const VS_VERSION_INFO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const WIM_BOOT_NOT_OS_WIM: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const WIM_BOOT_OS_WIM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const WIM_ENTRY_FLAG_NOT_ACTIVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const WIM_ENTRY_FLAG_SUSPENDED: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct WIM_ENTRY_INFO {
    pub WimEntryInfoSize: u32,
    pub WimType: u32,
    pub DataSourceId: i64,
    pub WimGuid: ::windows_sys::core::GUID,
    pub WimPath: ::windows_sys::core::PCWSTR,
    pub WimIndex: u32,
    pub Flags: u32,
}
impl ::core::marker::Copy for WIM_ENTRY_INFO {}
impl ::core::clone::Clone for WIM_ENTRY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct WIM_EXTERNAL_FILE_INFO {
    pub DataSourceId: i64,
    pub ResourceHash: [u8; 20],
    pub Flags: u32,
}
impl ::core::marker::Copy for WIM_EXTERNAL_FILE_INFO {}
impl ::core::clone::Clone for WIM_EXTERNAL_FILE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const WIM_EXTERNAL_FILE_INFO_FLAG_NOT_ACTIVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const WIM_EXTERNAL_FILE_INFO_FLAG_SUSPENDED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const WIM_PROVIDER_HASH_SIZE: u32 = 20u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WIN32_FILE_ATTRIBUTE_DATA {
    pub dwFileAttributes: u32,
    pub ftCreationTime: super::super::Foundation::FILETIME,
    pub ftLastAccessTime: super::super::Foundation::FILETIME,
    pub ftLastWriteTime: super::super::Foundation::FILETIME,
    pub nFileSizeHigh: u32,
    pub nFileSizeLow: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WIN32_FILE_ATTRIBUTE_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WIN32_FILE_ATTRIBUTE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WIN32_FIND_DATAA {
    pub dwFileAttributes: u32,
    pub ftCreationTime: super::super::Foundation::FILETIME,
    pub ftLastAccessTime: super::super::Foundation::FILETIME,
    pub ftLastWriteTime: super::super::Foundation::FILETIME,
    pub nFileSizeHigh: u32,
    pub nFileSizeLow: u32,
    pub dwReserved0: u32,
    pub dwReserved1: u32,
    pub cFileName: [super::super::Foundation::CHAR; 260],
    pub cAlternateFileName: [super::super::Foundation::CHAR; 14],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WIN32_FIND_DATAA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WIN32_FIND_DATAA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WIN32_FIND_DATAW {
    pub dwFileAttributes: u32,
    pub ftCreationTime: super::super::Foundation::FILETIME,
    pub ftLastAccessTime: super::super::Foundation::FILETIME,
    pub ftLastWriteTime: super::super::Foundation::FILETIME,
    pub nFileSizeHigh: u32,
    pub nFileSizeLow: u32,
    pub dwReserved0: u32,
    pub dwReserved1: u32,
    pub cFileName: [u16; 260],
    pub cAlternateFileName: [u16; 14],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WIN32_FIND_DATAW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WIN32_FIND_DATAW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct WIN32_FIND_STREAM_DATA {
    pub StreamSize: i64,
    pub cStreamName: [u16; 296],
}
impl ::core::marker::Copy for WIN32_FIND_STREAM_DATA {}
impl ::core::clone::Clone for WIN32_FIND_STREAM_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct WIN32_STREAM_ID {
    pub dwStreamId: WIN_STREAM_ID,
    pub dwStreamAttributes: u32,
    pub Size: i64,
    pub dwStreamNameSize: u32,
    pub cStreamName: [u16; 1],
}
impl ::core::marker::Copy for WIN32_STREAM_ID {}
impl ::core::clone::Clone for WIN32_STREAM_ID {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const WINEFS_SETUSERKEY_SET_CAPABILITIES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub type WIN_STREAM_ID = u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const BACKUP_ALTERNATE_DATA: WIN_STREAM_ID = 4u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const BACKUP_DATA: WIN_STREAM_ID = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const BACKUP_EA_DATA: WIN_STREAM_ID = 2u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const BACKUP_LINK: WIN_STREAM_ID = 5u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const BACKUP_OBJECT_ID: WIN_STREAM_ID = 7u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const BACKUP_PROPERTY_DATA: WIN_STREAM_ID = 6u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const BACKUP_REPARSE_DATA: WIN_STREAM_ID = 8u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const BACKUP_SECURITY_DATA: WIN_STREAM_ID = 3u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const BACKUP_SPARSE_BLOCK: WIN_STREAM_ID = 9u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const BACKUP_TXFS_DATA: WIN_STREAM_ID = 10u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct WOF_FILE_COMPRESSION_INFO_V0 {
    pub Algorithm: u32,
}
impl ::core::marker::Copy for WOF_FILE_COMPRESSION_INFO_V0 {}
impl ::core::clone::Clone for WOF_FILE_COMPRESSION_INFO_V0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub struct WOF_FILE_COMPRESSION_INFO_V1 {
    pub Algorithm: u32,
    pub Flags: u32,
}
impl ::core::marker::Copy for WOF_FILE_COMPRESSION_INFO_V1 {}
impl ::core::clone::Clone for WOF_FILE_COMPRESSION_INFO_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const WOF_PROVIDER_FILE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const WOF_PROVIDER_WIM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type WofEnumEntryProc = ::core::option::Option<unsafe extern "system" fn(entryinfo: *const ::core::ffi::c_void, userdata: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type WofEnumFilesProc = ::core::option::Option<unsafe extern "system" fn(filepath: ::windows_sys::core::PCWSTR, externalfileinfo: *const ::core::ffi::c_void, userdata: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Storage_FileSystem\"`*"]
pub const _FT_TYPES_DEFINITION_: u32 = 1u32;
