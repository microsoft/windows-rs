windows_link::link!("kernel32.dll" "system" fn AreFileApisANSI() -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn AreShortNamesEnabled(handle : super::winnt::HANDLE, enabled : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("kernel32.dll" "system" fn CompareFileTime(lpfiletime1 : *const super::minwindef::FILETIME, lpfiletime2 : *const super::minwindef::FILETIME) -> i32);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateDirectory2A(lppathname : windows_sys::core::PCSTR, dwdesiredaccess : u32, dwsharemode : u32, directoryflags : DIRECTORY_FLAGS, lpsecurityattributes : *const super::minwinbase::SECURITY_ATTRIBUTES) -> super::winnt::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateDirectory2W(lppathname : windows_sys::core::PCWSTR, dwdesiredaccess : u32, dwsharemode : u32, directoryflags : DIRECTORY_FLAGS, lpsecurityattributes : *const super::minwinbase::SECURITY_ATTRIBUTES) -> super::winnt::HANDLE);
#[cfg(feature = "minwinbase")]
windows_link::link!("kernel32.dll" "system" fn CreateDirectoryA(lppathname : windows_sys::core::PCSTR, lpsecurityattributes : *const super::minwinbase::SECURITY_ATTRIBUTES) -> windows_sys::core::BOOL);
#[cfg(feature = "minwinbase")]
windows_link::link!("kernel32.dll" "system" fn CreateDirectoryW(lppathname : windows_sys::core::PCWSTR, lpsecurityattributes : *const super::minwinbase::SECURITY_ATTRIBUTES) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateFile2(lpfilename : windows_sys::core::PCWSTR, dwdesiredaccess : u32, dwsharemode : u32, dwcreationdisposition : u32, pcreateexparams : *const CREATEFILE2_EXTENDED_PARAMETERS) -> super::winnt::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateFile3(lpfilename : windows_sys::core::PCWSTR, dwdesiredaccess : u32, dwsharemode : u32, dwcreationdisposition : u32, pcreateexparams : *const CREATEFILE3_EXTENDED_PARAMETERS) -> super::winnt::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateFileA(lpfilename : windows_sys::core::PCSTR, dwdesiredaccess : u32, dwsharemode : u32, lpsecurityattributes : *const super::minwinbase::SECURITY_ATTRIBUTES, dwcreationdisposition : u32, dwflagsandattributes : u32, htemplatefile : super::winnt::HANDLE) -> super::winnt::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateFileW(lpfilename : windows_sys::core::PCWSTR, dwdesiredaccess : u32, dwsharemode : u32, lpsecurityattributes : *const super::minwinbase::SECURITY_ATTRIBUTES, dwcreationdisposition : u32, dwflagsandattributes : u32, htemplatefile : super::winnt::HANDLE) -> super::winnt::HANDLE);
windows_link::link!("kernel32.dll" "system" fn DefineDosDeviceW(dwflags : u32, lpdevicename : windows_sys::core::PCWSTR, lptargetpath : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn DeleteFile2A(lpfilename : windows_sys::core::PCSTR, flags : u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn DeleteFile2W(lpfilename : windows_sys::core::PCWSTR, flags : u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn DeleteFileA(lpfilename : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn DeleteFileW(lpfilename : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn DeleteVolumeMountPointW(lpszvolumemountpoint : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("kernel32.dll" "system" fn FileTimeToLocalFileTime(lpfiletime : *const super::minwindef::FILETIME, lplocalfiletime : *mut super::minwindef::FILETIME) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn FindClose(hfindfile : super::winnt::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn FindCloseChangeNotification(hchangehandle : super::winnt::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn FindFirstChangeNotificationA(lppathname : windows_sys::core::PCSTR, bwatchsubtree : windows_sys::core::BOOL, dwnotifyfilter : u32) -> super::winnt::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn FindFirstChangeNotificationW(lppathname : windows_sys::core::PCWSTR, bwatchsubtree : windows_sys::core::BOOL, dwnotifyfilter : u32) -> super::winnt::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn FindFirstFileA(lpfilename : windows_sys::core::PCSTR, lpfindfiledata : *mut super::minwinbase::WIN32_FIND_DATAA) -> super::winnt::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn FindFirstFileExA(lpfilename : windows_sys::core::PCSTR, finfolevelid : super::minwinbase::FINDEX_INFO_LEVELS, lpfindfiledata : *mut core::ffi::c_void, fsearchop : super::minwinbase::FINDEX_SEARCH_OPS, lpsearchfilter : *const core::ffi::c_void, dwadditionalflags : u32) -> super::winnt::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn FindFirstFileExW(lpfilename : windows_sys::core::PCWSTR, finfolevelid : super::minwinbase::FINDEX_INFO_LEVELS, lpfindfiledata : *mut core::ffi::c_void, fsearchop : super::minwinbase::FINDEX_SEARCH_OPS, lpsearchfilter : *const core::ffi::c_void, dwadditionalflags : u32) -> super::winnt::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn FindFirstFileNameW(lpfilename : windows_sys::core::PCWSTR, dwflags : u32, stringlength : *mut u32, linkname : windows_sys::core::PWSTR) -> super::winnt::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn FindFirstFileW(lpfilename : windows_sys::core::PCWSTR, lpfindfiledata : *mut super::minwinbase::WIN32_FIND_DATAW) -> super::winnt::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn FindFirstStreamW(lpfilename : windows_sys::core::PCWSTR, infolevel : STREAM_INFO_LEVELS, lpfindstreamdata : *mut core::ffi::c_void, dwflags : u32) -> super::winnt::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn FindFirstVolumeW(lpszvolumename : windows_sys::core::PWSTR, cchbufferlength : u32) -> super::winnt::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn FindNextChangeNotification(hchangehandle : super::winnt::HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn FindNextFileA(hfindfile : super::winnt::HANDLE, lpfindfiledata : *mut super::minwinbase::WIN32_FIND_DATAA) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn FindNextFileNameW(hfindstream : super::winnt::HANDLE, stringlength : *mut u32, linkname : windows_sys::core::PWSTR) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn FindNextFileW(hfindfile : super::winnt::HANDLE, lpfindfiledata : *mut super::minwinbase::WIN32_FIND_DATAW) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn FindNextStreamW(hfindstream : super::winnt::HANDLE, lpfindstreamdata : *mut core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn FindNextVolumeW(hfindvolume : super::winnt::HANDLE, lpszvolumename : windows_sys::core::PWSTR, cchbufferlength : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn FindVolumeClose(hfindvolume : super::winnt::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn FlushFileBuffers(hfile : super::winnt::HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetCompressedFileSizeA(lpfilename : windows_sys::core::PCSTR, lpfilesizehigh : *mut u32) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetCompressedFileSizeW(lpfilename : windows_sys::core::PCWSTR, lpfilesizehigh : *mut u32) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetDiskFreeSpaceA(lprootpathname : windows_sys::core::PCSTR, lpsectorspercluster : *mut u32, lpbytespersector : *mut u32, lpnumberoffreeclusters : *mut u32, lptotalnumberofclusters : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetDiskFreeSpaceExA(lpdirectoryname : windows_sys::core::PCSTR, lpfreebytesavailabletocaller : *mut u64, lptotalnumberofbytes : *mut u64, lptotalnumberoffreebytes : *mut u64) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetDiskFreeSpaceExW(lpdirectoryname : windows_sys::core::PCWSTR, lpfreebytesavailabletocaller : *mut u64, lptotalnumberofbytes : *mut u64, lptotalnumberoffreebytes : *mut u64) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetDiskFreeSpaceW(lprootpathname : windows_sys::core::PCWSTR, lpsectorspercluster : *mut u32, lpbytespersector : *mut u32, lpnumberoffreeclusters : *mut u32, lptotalnumberofclusters : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetDiskSpaceInformationA(rootpath : windows_sys::core::PCSTR, diskspaceinfo : *mut DISK_SPACE_INFORMATION) -> windows_sys::core::HRESULT);
windows_link::link!("kernel32.dll" "system" fn GetDiskSpaceInformationW(rootpath : windows_sys::core::PCWSTR, diskspaceinfo : *mut DISK_SPACE_INFORMATION) -> windows_sys::core::HRESULT);
windows_link::link!("kernel32.dll" "system" fn GetDriveTypeA(lprootpathname : windows_sys::core::PCSTR) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetDriveTypeW(lprootpathname : windows_sys::core::PCWSTR) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetFileAttributesA(lpfilename : windows_sys::core::PCSTR) -> u32);
#[cfg(feature = "minwinbase")]
windows_link::link!("kernel32.dll" "system" fn GetFileAttributesExA(lpfilename : windows_sys::core::PCSTR, finfolevelid : super::minwinbase::GET_FILEEX_INFO_LEVELS, lpfileinformation : *mut core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "minwinbase")]
windows_link::link!("kernel32.dll" "system" fn GetFileAttributesExW(lpfilename : windows_sys::core::PCWSTR, finfolevelid : super::minwinbase::GET_FILEEX_INFO_LEVELS, lpfileinformation : *mut core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetFileAttributesW(lpfilename : windows_sys::core::PCWSTR) -> u32);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetFileInformationByHandle(hfile : super::winnt::HANDLE, lpfileinformation : *mut BY_HANDLE_FILE_INFORMATION) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetFileSize(hfile : super::winnt::HANDLE, lpfilesizehigh : *mut u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetFileSizeEx(hfile : super::winnt::HANDLE, lpfilesize : *mut i64) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetFileTime(hfile : super::winnt::HANDLE, lpcreationtime : *mut super::minwindef::FILETIME, lplastaccesstime : *mut super::minwindef::FILETIME, lplastwritetime : *mut super::minwindef::FILETIME) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetFileType(hfile : super::winnt::HANDLE) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetFinalPathNameByHandleA(hfile : super::winnt::HANDLE, lpszfilepath : windows_sys::core::PSTR, cchfilepath : u32, dwflags : u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetFinalPathNameByHandleW(hfile : super::winnt::HANDLE, lpszfilepath : windows_sys::core::PWSTR, cchfilepath : u32, dwflags : u32) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetFullPathNameA(lpfilename : windows_sys::core::PCSTR, nbufferlength : u32, lpbuffer : windows_sys::core::PSTR, lpfilepart : *mut windows_sys::core::PSTR) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetFullPathNameW(lpfilename : windows_sys::core::PCWSTR, nbufferlength : u32, lpbuffer : windows_sys::core::PWSTR, lpfilepart : *mut windows_sys::core::PWSTR) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetLogicalDriveStringsW(nbufferlength : u32, lpbuffer : windows_sys::core::PWSTR) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetLogicalDrives() -> u32);
windows_link::link!("kernel32.dll" "system" fn GetLongPathNameA(lpszshortpath : windows_sys::core::PCSTR, lpszlongpath : windows_sys::core::PSTR, cchbuffer : u32) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetLongPathNameW(lpszshortpath : windows_sys::core::PCWSTR, lpszlongpath : windows_sys::core::PWSTR, cchbuffer : u32) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetShortPathNameW(lpszlongpath : windows_sys::core::PCWSTR, lpszshortpath : windows_sys::core::PWSTR, cchbuffer : u32) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetTempFileNameA(lppathname : windows_sys::core::PCSTR, lpprefixstring : windows_sys::core::PCSTR, uunique : u32, lptempfilename : windows_sys::core::PSTR) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetTempFileNameW(lppathname : windows_sys::core::PCWSTR, lpprefixstring : windows_sys::core::PCWSTR, uunique : u32, lptempfilename : windows_sys::core::PWSTR) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetTempPath2A(bufferlength : u32, buffer : windows_sys::core::PSTR) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetTempPath2W(bufferlength : u32, buffer : windows_sys::core::PWSTR) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetTempPathA(nbufferlength : u32, lpbuffer : windows_sys::core::PSTR) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetTempPathW(nbufferlength : u32, lpbuffer : windows_sys::core::PWSTR) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetVolumeInformationA(lprootpathname : windows_sys::core::PCSTR, lpvolumenamebuffer : windows_sys::core::PSTR, nvolumenamesize : u32, lpvolumeserialnumber : *mut u32, lpmaximumcomponentlength : *mut u32, lpfilesystemflags : *mut u32, lpfilesystemnamebuffer : windows_sys::core::PSTR, nfilesystemnamesize : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetVolumeInformationByHandleW(hfile : super::winnt::HANDLE, lpvolumenamebuffer : windows_sys::core::PWSTR, nvolumenamesize : u32, lpvolumeserialnumber : *mut u32, lpmaximumcomponentlength : *mut u32, lpfilesystemflags : *mut u32, lpfilesystemnamebuffer : windows_sys::core::PWSTR, nfilesystemnamesize : u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetVolumeInformationW(lprootpathname : windows_sys::core::PCWSTR, lpvolumenamebuffer : windows_sys::core::PWSTR, nvolumenamesize : u32, lpvolumeserialnumber : *mut u32, lpmaximumcomponentlength : *mut u32, lpfilesystemflags : *mut u32, lpfilesystemnamebuffer : windows_sys::core::PWSTR, nfilesystemnamesize : u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetVolumeNameForVolumeMountPointW(lpszvolumemountpoint : windows_sys::core::PCWSTR, lpszvolumename : windows_sys::core::PWSTR, cchbufferlength : u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetVolumePathNameW(lpszfilename : windows_sys::core::PCWSTR, lpszvolumepathname : windows_sys::core::PWSTR, cchbufferlength : u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetVolumePathNamesForVolumeNameW(lpszvolumename : windows_sys::core::PCWSTR, lpszvolumepathnames : *mut u16, cchbufferlength : u32, lpcchreturnlength : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("kernel32.dll" "system" fn LocalFileTimeToFileTime(lplocalfiletime : *const super::minwindef::FILETIME, lpfiletime : *mut super::minwindef::FILETIME) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn LockFile(hfile : super::winnt::HANDLE, dwfileoffsetlow : u32, dwfileoffsethigh : u32, nnumberofbytestolocklow : u32, nnumberofbytestolockhigh : u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn LockFileEx(hfile : super::winnt::HANDLE, dwflags : u32, dwreserved : u32, nnumberofbytestolocklow : u32, nnumberofbytestolockhigh : u32, lpoverlapped : *mut super::minwinbase::OVERLAPPED) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn QueryDosDeviceW(lpdevicename : windows_sys::core::PCWSTR, lptargetpath : windows_sys::core::PWSTR, ucchmax : u32) -> u32);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn ReadFile(hfile : super::winnt::HANDLE, lpbuffer : *mut core::ffi::c_void, nnumberofbytestoread : u32, lpnumberofbytesread : *mut u32, lpoverlapped : *mut super::minwinbase::OVERLAPPED) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn ReadFileEx(hfile : super::winnt::HANDLE, lpbuffer : *mut core::ffi::c_void, nnumberofbytestoread : u32, lpoverlapped : *mut super::minwinbase::OVERLAPPED, lpcompletionroutine : super::minwinbase::LPOVERLAPPED_COMPLETION_ROUTINE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn ReadFileScatter(hfile : super::winnt::HANDLE, asegmentarray : *const super::winnt::FILE_SEGMENT_ELEMENT, nnumberofbytestoread : u32, lpreserved : *const u32, lpoverlapped : *mut super::minwinbase::OVERLAPPED) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn RemoveDirectory2A(lppathname : windows_sys::core::PCSTR, directoryflags : DIRECTORY_FLAGS) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn RemoveDirectory2W(lppathname : windows_sys::core::PCWSTR, directoryflags : DIRECTORY_FLAGS) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn RemoveDirectoryA(lppathname : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn RemoveDirectoryW(lppathname : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetEndOfFile(hfile : super::winnt::HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetFileApisToANSI());
windows_link::link!("kernel32.dll" "system" fn SetFileApisToOEM());
windows_link::link!("kernel32.dll" "system" fn SetFileAttributesA(lpfilename : windows_sys::core::PCSTR, dwfileattributes : u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetFileAttributesW(lpfilename : windows_sys::core::PCWSTR, dwfileattributes : u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn SetFileInformationByHandle(hfile : super::winnt::HANDLE, fileinformationclass : super::minwinbase::FILE_INFO_BY_HANDLE_CLASS, lpfileinformation : *const core::ffi::c_void, dwbuffersize : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetFileIoOverlappedRange(filehandle : super::winnt::HANDLE, overlappedrangestart : *const u8, length : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetFilePointer(hfile : super::winnt::HANDLE, ldistancetomove : i32, lpdistancetomovehigh : *mut i32, dwmovemethod : u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetFilePointerEx(hfile : super::winnt::HANDLE, lidistancetomove : i64, lpnewfilepointer : *mut i64, dwmovemethod : u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn SetFileTime(hfile : super::winnt::HANDLE, lpcreationtime : *const super::minwindef::FILETIME, lplastaccesstime : *const super::minwindef::FILETIME, lplastwritetime : *const super::minwindef::FILETIME) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetFileValidData(hfile : super::winnt::HANDLE, validdatalength : i64) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn UnlockFile(hfile : super::winnt::HANDLE, dwfileoffsetlow : u32, dwfileoffsethigh : u32, nnumberofbytestounlocklow : u32, nnumberofbytestounlockhigh : u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn UnlockFileEx(hfile : super::winnt::HANDLE, dwreserved : u32, nnumberofbytestounlocklow : u32, nnumberofbytestounlockhigh : u32, lpoverlapped : *mut super::minwinbase::OVERLAPPED) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn WriteFile(hfile : super::winnt::HANDLE, lpbuffer : *const core::ffi::c_void, nnumberofbytestowrite : u32, lpnumberofbyteswritten : *mut u32, lpoverlapped : *mut super::minwinbase::OVERLAPPED) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn WriteFileEx(hfile : super::winnt::HANDLE, lpbuffer : *const core::ffi::c_void, nnumberofbytestowrite : u32, lpoverlapped : *mut super::minwinbase::OVERLAPPED, lpcompletionroutine : super::minwinbase::LPOVERLAPPED_COMPLETION_ROUTINE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn WriteFileGather(hfile : super::winnt::HANDLE, asegmentarray : *const super::winnt::FILE_SEGMENT_ELEMENT, nnumberofbytestowrite : u32, lpreserved : *const u32, lpoverlapped : *mut super::minwinbase::OVERLAPPED) -> windows_sys::core::BOOL);
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct BY_HANDLE_FILE_INFORMATION {
    pub dwFileAttributes: u32,
    pub ftCreationTime: super::minwindef::FILETIME,
    pub ftLastAccessTime: super::minwindef::FILETIME,
    pub ftLastWriteTime: super::minwindef::FILETIME,
    pub dwVolumeSerialNumber: u32,
    pub nFileSizeHigh: u32,
    pub nFileSizeLow: u32,
    pub nNumberOfLinks: u32,
    pub nFileIndexHigh: u32,
    pub nFileIndexLow: u32,
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct CREATEFILE2_EXTENDED_PARAMETERS {
    pub dwSize: u32,
    pub dwFileAttributes: u32,
    pub dwFileFlags: u32,
    pub dwSecurityQosFlags: u32,
    pub lpSecurityAttributes: super::minwinbase::LPSECURITY_ATTRIBUTES,
    pub hTemplateFile: super::winnt::HANDLE,
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
impl Default for CREATEFILE2_EXTENDED_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct CREATEFILE3_EXTENDED_PARAMETERS {
    pub dwSize: u32,
    pub dwFileAttributes: u32,
    pub dwFileFlags: u32,
    pub dwSecurityQosFlags: u32,
    pub lpSecurityAttributes: super::minwinbase::LPSECURITY_ATTRIBUTES,
    pub hTemplateFile: super::winnt::HANDLE,
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
impl Default for CREATEFILE3_EXTENDED_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CREATE_ALWAYS: u32 = 2;
pub const CREATE_NEW: u32 = 1;
pub type DIRECTORY_FLAGS = u32;
pub const DIRECTORY_FLAGS_DISALLOW_PATH_REDIRECTS: DIRECTORY_FLAGS = 1;
pub const DIRECTORY_FLAGS_NONE: DIRECTORY_FLAGS = 0;
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
pub const FindStreamInfoMaxInfoLevel: STREAM_INFO_LEVELS = 1;
pub const FindStreamInfoStandard: STREAM_INFO_LEVELS = 0;
pub const INVALID_FILE_ATTRIBUTES: u32 = 4294967295;
pub const INVALID_FILE_SIZE: u32 = 4294967295;
pub const INVALID_SET_FILE_POINTER: u32 = 4294967295;
#[cfg(feature = "minwindef")]
pub type LPBY_HANDLE_FILE_INFORMATION = *mut BY_HANDLE_FILE_INFORMATION;
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
pub type LPCREATEFILE2_EXTENDED_PARAMETERS = *mut CREATEFILE2_EXTENDED_PARAMETERS;
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
pub type LPCREATEFILE3_EXTENDED_PARAMETERS = *mut CREATEFILE3_EXTENDED_PARAMETERS;
#[cfg(feature = "minwindef")]
pub type LPWIN32_FILE_ATTRIBUTE_DATA = *mut WIN32_FILE_ATTRIBUTE_DATA;
pub const OPEN_ALWAYS: u32 = 4;
pub const OPEN_EXISTING: u32 = 3;
#[cfg(feature = "minwindef")]
pub type PBY_HANDLE_FILE_INFORMATION = *mut BY_HANDLE_FILE_INFORMATION;
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
pub type PCREATEFILE2_EXTENDED_PARAMETERS = *mut CREATEFILE2_EXTENDED_PARAMETERS;
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
pub type PCREATEFILE3_EXTENDED_PARAMETERS = *mut CREATEFILE3_EXTENDED_PARAMETERS;
pub type PWIN32_FIND_STREAM_DATA = *mut WIN32_FIND_STREAM_DATA;
pub type STREAM_INFO_LEVELS = i32;
pub const TRUNCATE_EXISTING: u32 = 5;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct WIN32_FILE_ATTRIBUTE_DATA {
    pub dwFileAttributes: u32,
    pub ftCreationTime: super::minwindef::FILETIME,
    pub ftLastAccessTime: super::minwindef::FILETIME,
    pub ftLastWriteTime: super::minwindef::FILETIME,
    pub nFileSizeHigh: u32,
    pub nFileSizeLow: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WIN32_FIND_STREAM_DATA {
    pub StreamSize: i64,
    pub cStreamName: [u16; 296],
}
impl Default for WIN32_FIND_STREAM_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
