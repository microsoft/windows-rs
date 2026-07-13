windows_link::link!("api-ms-win-core-file-fromapp-l1-1-0.dll" "system" fn CopyFileFromAppW(lpexistingfilename : windows_sys::core::PCWSTR, lpnewfilename : windows_sys::core::PCWSTR, bfailifexists : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "minwinbase")]
windows_link::link!("api-ms-win-core-file-fromapp-l1-1-0.dll" "system" fn CreateDirectoryFromAppW(lppathname : windows_sys::core::PCWSTR, lpsecurityattributes : *const super::minwinbase::SECURITY_ATTRIBUTES) -> windows_sys::core::BOOL);
#[cfg(all(feature = "fileapi", feature = "minwinbase", feature = "winnt"))]
windows_link::link!("api-ms-win-core-file-fromapp-l1-1-0.dll" "system" fn CreateFile2FromAppW(lpfilename : windows_sys::core::PCWSTR, dwdesiredaccess : u32, dwsharemode : u32, dwcreationdisposition : u32, pcreateexparams : *const super::fileapi::CREATEFILE2_EXTENDED_PARAMETERS) -> super::winnt::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("api-ms-win-core-file-fromapp-l1-1-0.dll" "system" fn CreateFileFromAppW(lpfilename : windows_sys::core::PCWSTR, dwdesiredaccess : u32, dwsharemode : u32, lpsecurityattributes : *const super::minwinbase::SECURITY_ATTRIBUTES, dwcreationdisposition : u32, dwflagsandattributes : u32, htemplatefile : super::winnt::HANDLE) -> super::winnt::HANDLE);
windows_link::link!("api-ms-win-core-file-fromapp-l1-1-0.dll" "system" fn DeleteFileFromAppW(lpfilename : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("api-ms-win-core-file-fromapp-l1-1-0.dll" "system" fn FindFirstFileExFromAppW(lpfilename : windows_sys::core::PCWSTR, finfolevelid : super::minwinbase::FINDEX_INFO_LEVELS, lpfindfiledata : *mut core::ffi::c_void, fsearchop : super::minwinbase::FINDEX_SEARCH_OPS, lpsearchfilter : *const core::ffi::c_void, dwadditionalflags : u32) -> super::winnt::HANDLE);
#[cfg(feature = "minwinbase")]
windows_link::link!("api-ms-win-core-file-fromapp-l1-1-0.dll" "system" fn GetFileAttributesExFromAppW(lpfilename : windows_sys::core::PCWSTR, finfolevelid : super::minwinbase::GET_FILEEX_INFO_LEVELS, lpfileinformation : *mut core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("api-ms-win-core-file-fromapp-l1-1-0.dll" "system" fn MoveFileFromAppW(lpexistingfilename : windows_sys::core::PCWSTR, lpnewfilename : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("api-ms-win-core-file-fromapp-l1-1-0.dll" "system" fn RemoveDirectoryFromAppW(lppathname : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("api-ms-win-core-file-fromapp-l1-1-0.dll" "system" fn ReplaceFileFromAppW(lpreplacedfilename : windows_sys::core::PCWSTR, lpreplacementfilename : windows_sys::core::PCWSTR, lpbackupfilename : windows_sys::core::PCWSTR, dwreplaceflags : u32, lpexclude : *const core::ffi::c_void, lpreserved : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("api-ms-win-core-file-fromapp-l1-1-0.dll" "system" fn SetFileAttributesFromAppW(lpfilename : windows_sys::core::PCWSTR, dwfileattributes : u32) -> windows_sys::core::BOOL);
