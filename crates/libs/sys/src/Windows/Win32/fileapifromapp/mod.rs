windows_link::link!("api-ms-win-core-file-fromapp-l1-1-0.dll" "system" fn CopyFileFromAppW(lpexistingfilename : windows_sys::core::PCWSTR, lpnewfilename : windows_sys::core::PCWSTR, bfailifexists : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared"))]
windows_link::link!("api-ms-win-core-file-fromapp-l1-1-0.dll" "system" fn CreateDirectoryFromAppW(lppathname : windows_sys::core::PCWSTR, lpsecurityattributes : super::LPSECURITY_ATTRIBUTES) -> windows_sys::core::BOOL);
#[cfg(all(feature = "fileapi", feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("api-ms-win-core-file-fromapp-l1-1-0.dll" "system" fn CreateFile2FromAppW(lpfilename : windows_sys::core::PCWSTR, dwdesiredaccess : super::DWORD, dwsharemode : super::DWORD, dwcreationdisposition : super::DWORD, pcreateexparams : super::LPCREATEFILE2_EXTENDED_PARAMETERS) -> super::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("api-ms-win-core-file-fromapp-l1-1-0.dll" "system" fn CreateFileFromAppW(lpfilename : windows_sys::core::PCWSTR, dwdesiredaccess : super::DWORD, dwsharemode : super::DWORD, lpsecurityattributes : super::LPSECURITY_ATTRIBUTES, dwcreationdisposition : super::DWORD, dwflagsandattributes : super::DWORD, htemplatefile : super::HANDLE) -> super::HANDLE);
windows_link::link!("api-ms-win-core-file-fromapp-l1-1-0.dll" "system" fn DeleteFileFromAppW(lpfilename : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("api-ms-win-core-file-fromapp-l1-1-0.dll" "system" fn FindFirstFileExFromAppW(lpfilename : windows_sys::core::PCWSTR, finfolevelid : super::FINDEX_INFO_LEVELS, lpfindfiledata : super::LPVOID, fsearchop : super::FINDEX_SEARCH_OPS, lpsearchfilter : super::LPVOID, dwadditionalflags : super::DWORD) -> super::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "minwindef"))]
windows_link::link!("api-ms-win-core-file-fromapp-l1-1-0.dll" "system" fn GetFileAttributesExFromAppW(lpfilename : windows_sys::core::PCWSTR, finfolevelid : super::GET_FILEEX_INFO_LEVELS, lpfileinformation : super::LPVOID) -> windows_sys::core::BOOL);
windows_link::link!("api-ms-win-core-file-fromapp-l1-1-0.dll" "system" fn MoveFileFromAppW(lpexistingfilename : windows_sys::core::PCWSTR, lpnewfilename : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("api-ms-win-core-file-fromapp-l1-1-0.dll" "system" fn RemoveDirectoryFromAppW(lppathname : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("api-ms-win-core-file-fromapp-l1-1-0.dll" "system" fn ReplaceFileFromAppW(lpreplacedfilename : windows_sys::core::PCWSTR, lpreplacementfilename : windows_sys::core::PCWSTR, lpbackupfilename : windows_sys::core::PCWSTR, dwreplaceflags : super::DWORD, lpexclude : super::LPVOID, lpreserved : super::LPVOID) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("api-ms-win-core-file-fromapp-l1-1-0.dll" "system" fn SetFileAttributesFromAppW(lpfilename : windows_sys::core::PCWSTR, dwfileattributes : super::DWORD) -> windows_sys::core::BOOL);
