windows_link::link!("kernel32.dll" "system" fn AreFileApisANSI() -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn AreShortNamesEnabled(handle : super::HANDLE, enabled : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("kernel32.dll" "system" fn CompareFileTime(lpfiletime1 : *const super::FILETIME, lpfiletime2 : *const super::FILETIME) -> super::LONG);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateDirectory2A(lppathname : windows_sys::core::PCSTR, dwdesiredaccess : super::DWORD, dwsharemode : super::DWORD, directoryflags : DIRECTORY_FLAGS, lpsecurityattributes : super::LPSECURITY_ATTRIBUTES) -> super::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateDirectory2W(lppathname : windows_sys::core::PCWSTR, dwdesiredaccess : super::DWORD, dwsharemode : super::DWORD, directoryflags : DIRECTORY_FLAGS, lpsecurityattributes : super::LPSECURITY_ATTRIBUTES) -> super::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared"))]
windows_link::link!("kernel32.dll" "system" fn CreateDirectoryA(lppathname : windows_sys::core::PCSTR, lpsecurityattributes : super::LPSECURITY_ATTRIBUTES) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared"))]
windows_link::link!("kernel32.dll" "system" fn CreateDirectoryW(lppathname : windows_sys::core::PCWSTR, lpsecurityattributes : super::LPSECURITY_ATTRIBUTES) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateFile2(lpfilename : windows_sys::core::PCWSTR, dwdesiredaccess : super::DWORD, dwsharemode : super::DWORD, dwcreationdisposition : super::DWORD, pcreateexparams : LPCREATEFILE2_EXTENDED_PARAMETERS) -> super::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateFile3(lpfilename : windows_sys::core::PCWSTR, dwdesiredaccess : super::DWORD, dwsharemode : super::DWORD, dwcreationdisposition : super::DWORD, pcreateexparams : LPCREATEFILE3_EXTENDED_PARAMETERS) -> super::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateFileA(lpfilename : windows_sys::core::PCSTR, dwdesiredaccess : super::DWORD, dwsharemode : super::DWORD, lpsecurityattributes : super::LPSECURITY_ATTRIBUTES, dwcreationdisposition : super::DWORD, dwflagsandattributes : super::DWORD, htemplatefile : super::HANDLE) -> super::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateFileW(lpfilename : windows_sys::core::PCWSTR, dwdesiredaccess : super::DWORD, dwsharemode : super::DWORD, lpsecurityattributes : super::LPSECURITY_ATTRIBUTES, dwcreationdisposition : super::DWORD, dwflagsandattributes : super::DWORD, htemplatefile : super::HANDLE) -> super::HANDLE);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn DefineDosDeviceW(dwflags : super::DWORD, lpdevicename : windows_sys::core::PCWSTR, lptargetpath : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn DeleteFile2A(lpfilename : windows_sys::core::PCSTR, flags : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn DeleteFile2W(lpfilename : windows_sys::core::PCWSTR, flags : super::DWORD) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn DeleteFileA(lpfilename : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn DeleteFileW(lpfilename : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn DeleteVolumeMountPointW(lpszvolumemountpoint : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("kernel32.dll" "system" fn FileTimeToLocalFileTime(lpfiletime : *const super::FILETIME, lplocalfiletime : super::LPFILETIME) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn FindClose(hfindfile : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn FindCloseChangeNotification(hchangehandle : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn FindFirstChangeNotificationA(lppathname : windows_sys::core::PCSTR, bwatchsubtree : windows_sys::core::BOOL, dwnotifyfilter : super::DWORD) -> super::HANDLE);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn FindFirstChangeNotificationW(lppathname : windows_sys::core::PCWSTR, bwatchsubtree : windows_sys::core::BOOL, dwnotifyfilter : super::DWORD) -> super::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn FindFirstFileA(lpfilename : windows_sys::core::PCSTR, lpfindfiledata : super::LPWIN32_FIND_DATAA) -> super::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn FindFirstFileExA(lpfilename : windows_sys::core::PCSTR, finfolevelid : super::FINDEX_INFO_LEVELS, lpfindfiledata : super::LPVOID, fsearchop : super::FINDEX_SEARCH_OPS, lpsearchfilter : super::LPVOID, dwadditionalflags : super::DWORD) -> super::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn FindFirstFileExW(lpfilename : windows_sys::core::PCWSTR, finfolevelid : super::FINDEX_INFO_LEVELS, lpfindfiledata : super::LPVOID, fsearchop : super::FINDEX_SEARCH_OPS, lpsearchfilter : super::LPVOID, dwadditionalflags : super::DWORD) -> super::HANDLE);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn FindFirstFileNameW(lpfilename : windows_sys::core::PCWSTR, dwflags : super::DWORD, stringlength : super::LPDWORD, linkname : windows_sys::core::PWSTR) -> super::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn FindFirstFileW(lpfilename : windows_sys::core::PCWSTR, lpfindfiledata : super::LPWIN32_FIND_DATAW) -> super::HANDLE);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn FindFirstStreamW(lpfilename : windows_sys::core::PCWSTR, infolevel : STREAM_INFO_LEVELS, lpfindstreamdata : super::LPVOID, dwflags : super::DWORD) -> super::HANDLE);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn FindFirstVolumeW(lpszvolumename : windows_sys::core::PWSTR, cchbufferlength : super::DWORD) -> super::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn FindNextChangeNotification(hchangehandle : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn FindNextFileA(hfindfile : super::HANDLE, lpfindfiledata : super::LPWIN32_FIND_DATAA) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn FindNextFileNameW(hfindstream : super::HANDLE, stringlength : super::LPDWORD, linkname : windows_sys::core::PWSTR) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn FindNextFileW(hfindfile : super::HANDLE, lpfindfiledata : super::LPWIN32_FIND_DATAW) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn FindNextStreamW(hfindstream : super::HANDLE, lpfindstreamdata : super::LPVOID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn FindNextVolumeW(hfindvolume : super::HANDLE, lpszvolumename : windows_sys::core::PWSTR, cchbufferlength : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn FindVolumeClose(hfindvolume : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn FlushFileBuffers(hfile : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("kernel32.dll" "system" fn GetCompressedFileSizeA(lpfilename : windows_sys::core::PCSTR, lpfilesizehigh : super::LPDWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("kernel32.dll" "system" fn GetCompressedFileSizeW(lpfilename : windows_sys::core::PCWSTR, lpfilesizehigh : super::LPDWORD) -> super::DWORD);
#[cfg(feature = "minwindef")]
windows_link::link!("kernel32.dll" "system" fn GetDiskFreeSpaceA(lprootpathname : windows_sys::core::PCSTR, lpsectorspercluster : super::LPDWORD, lpbytespersector : super::LPDWORD, lpnumberoffreeclusters : super::LPDWORD, lptotalnumberofclusters : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetDiskFreeSpaceExA(lpdirectoryname : windows_sys::core::PCSTR, lpfreebytesavailabletocaller : super::PULARGE_INTEGER, lptotalnumberofbytes : super::PULARGE_INTEGER, lptotalnumberoffreebytes : super::PULARGE_INTEGER) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetDiskFreeSpaceExW(lpdirectoryname : windows_sys::core::PCWSTR, lpfreebytesavailabletocaller : super::PULARGE_INTEGER, lptotalnumberofbytes : super::PULARGE_INTEGER, lptotalnumberoffreebytes : super::PULARGE_INTEGER) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("kernel32.dll" "system" fn GetDiskFreeSpaceW(lprootpathname : windows_sys::core::PCWSTR, lpsectorspercluster : super::LPDWORD, lpbytespersector : super::LPDWORD, lpnumberoffreeclusters : super::LPDWORD, lptotalnumberofclusters : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn GetDiskSpaceInformationA(rootpath : windows_sys::core::PCSTR, diskspaceinfo : *mut DISK_SPACE_INFORMATION) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn GetDiskSpaceInformationW(rootpath : windows_sys::core::PCWSTR, diskspaceinfo : *mut DISK_SPACE_INFORMATION) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn GetDriveTypeA(lprootpathname : windows_sys::core::PCSTR) -> super::UINT);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn GetDriveTypeW(lprootpathname : windows_sys::core::PCWSTR) -> super::UINT);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn GetFileAttributesA(lpfilename : windows_sys::core::PCSTR) -> super::DWORD);
#[cfg(all(feature = "minwinbase", feature = "minwindef"))]
windows_link::link!("kernel32.dll" "system" fn GetFileAttributesExA(lpfilename : windows_sys::core::PCSTR, finfolevelid : super::GET_FILEEX_INFO_LEVELS, lpfileinformation : super::LPVOID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "minwindef"))]
windows_link::link!("kernel32.dll" "system" fn GetFileAttributesExW(lpfilename : windows_sys::core::PCWSTR, finfolevelid : super::GET_FILEEX_INFO_LEVELS, lpfileinformation : super::LPVOID) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn GetFileAttributesW(lpfilename : windows_sys::core::PCWSTR) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetFileInformationByHandle(hfile : super::HANDLE, lpfileinformation : LPBY_HANDLE_FILE_INFORMATION) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetFileSize(hfile : super::HANDLE, lpfilesizehigh : super::LPDWORD) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetFileSizeEx(hfile : super::HANDLE, lpfilesize : super::PLARGE_INTEGER) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetFileTime(hfile : super::HANDLE, lpcreationtime : super::LPFILETIME, lplastaccesstime : super::LPFILETIME, lplastwritetime : super::LPFILETIME) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetFileType(hfile : super::HANDLE) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetFinalPathNameByHandleA(hfile : super::HANDLE, lpszfilepath : windows_sys::core::PSTR, cchfilepath : super::DWORD, dwflags : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetFinalPathNameByHandleW(hfile : super::HANDLE, lpszfilepath : windows_sys::core::PWSTR, cchfilepath : super::DWORD, dwflags : super::DWORD) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn GetFullPathNameA(lpfilename : windows_sys::core::PCSTR, nbufferlength : super::DWORD, lpbuffer : windows_sys::core::PSTR, lpfilepart : *mut windows_sys::core::PSTR) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn GetFullPathNameW(lpfilename : windows_sys::core::PCWSTR, nbufferlength : super::DWORD, lpbuffer : windows_sys::core::PWSTR, lpfilepart : *mut windows_sys::core::PWSTR) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn GetLogicalDriveStringsW(nbufferlength : super::DWORD, lpbuffer : windows_sys::core::PWSTR) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn GetLogicalDrives() -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn GetLongPathNameA(lpszshortpath : windows_sys::core::PCSTR, lpszlongpath : windows_sys::core::PSTR, cchbuffer : super::DWORD) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn GetLongPathNameW(lpszshortpath : windows_sys::core::PCWSTR, lpszlongpath : windows_sys::core::PWSTR, cchbuffer : super::DWORD) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn GetShortPathNameW(lpszlongpath : windows_sys::core::PCWSTR, lpszshortpath : windows_sys::core::PWSTR, cchbuffer : super::DWORD) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn GetTempFileNameA(lppathname : windows_sys::core::PCSTR, lpprefixstring : windows_sys::core::PCSTR, uunique : super::UINT, lptempfilename : windows_sys::core::PSTR) -> super::UINT);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn GetTempFileNameW(lppathname : windows_sys::core::PCWSTR, lpprefixstring : windows_sys::core::PCWSTR, uunique : super::UINT, lptempfilename : windows_sys::core::PWSTR) -> super::UINT);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn GetTempPath2A(bufferlength : super::DWORD, buffer : windows_sys::core::PSTR) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn GetTempPath2W(bufferlength : super::DWORD, buffer : windows_sys::core::PWSTR) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn GetTempPathA(nbufferlength : super::DWORD, lpbuffer : windows_sys::core::PSTR) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn GetTempPathW(nbufferlength : super::DWORD, lpbuffer : windows_sys::core::PWSTR) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("kernel32.dll" "system" fn GetVolumeInformationA(lprootpathname : windows_sys::core::PCSTR, lpvolumenamebuffer : windows_sys::core::PSTR, nvolumenamesize : super::DWORD, lpvolumeserialnumber : super::LPDWORD, lpmaximumcomponentlength : super::LPDWORD, lpfilesystemflags : super::LPDWORD, lpfilesystemnamebuffer : windows_sys::core::PSTR, nfilesystemnamesize : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetVolumeInformationByHandleW(hfile : super::HANDLE, lpvolumenamebuffer : windows_sys::core::PWSTR, nvolumenamesize : super::DWORD, lpvolumeserialnumber : super::LPDWORD, lpmaximumcomponentlength : super::LPDWORD, lpfilesystemflags : super::LPDWORD, lpfilesystemnamebuffer : windows_sys::core::PWSTR, nfilesystemnamesize : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("kernel32.dll" "system" fn GetVolumeInformationW(lprootpathname : windows_sys::core::PCWSTR, lpvolumenamebuffer : windows_sys::core::PWSTR, nvolumenamesize : super::DWORD, lpvolumeserialnumber : super::LPDWORD, lpmaximumcomponentlength : super::LPDWORD, lpfilesystemflags : super::LPDWORD, lpfilesystemnamebuffer : windows_sys::core::PWSTR, nfilesystemnamesize : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn GetVolumeNameForVolumeMountPointW(lpszvolumemountpoint : windows_sys::core::PCWSTR, lpszvolumename : windows_sys::core::PWSTR, cchbufferlength : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn GetVolumePathNameW(lpszfilename : windows_sys::core::PCWSTR, lpszvolumepathname : windows_sys::core::PWSTR, cchbufferlength : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetVolumePathNamesForVolumeNameW(lpszvolumename : windows_sys::core::PCWSTR, lpszvolumepathnames : super::LPWCH, cchbufferlength : super::DWORD, lpcchreturnlength : super::PDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("kernel32.dll" "system" fn LocalFileTimeToFileTime(lplocalfiletime : *const super::FILETIME, lpfiletime : super::LPFILETIME) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn LockFile(hfile : super::HANDLE, dwfileoffsetlow : super::DWORD, dwfileoffsethigh : super::DWORD, nnumberofbytestolocklow : super::DWORD, nnumberofbytestolockhigh : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn LockFileEx(hfile : super::HANDLE, dwflags : super::DWORD, dwreserved : super::DWORD, nnumberofbytestolocklow : super::DWORD, nnumberofbytestolockhigh : super::DWORD, lpoverlapped : super::LPOVERLAPPED) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn QueryDosDeviceW(lpdevicename : windows_sys::core::PCWSTR, lptargetpath : windows_sys::core::PWSTR, ucchmax : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn ReadFile(hfile : super::HANDLE, lpbuffer : super::LPVOID, nnumberofbytestoread : super::DWORD, lpnumberofbytesread : super::LPDWORD, lpoverlapped : super::LPOVERLAPPED) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn ReadFileEx(hfile : super::HANDLE, lpbuffer : super::LPVOID, nnumberofbytestoread : super::DWORD, lpoverlapped : super::LPOVERLAPPED, lpcompletionroutine : super::LPOVERLAPPED_COMPLETION_ROUTINE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn ReadFileScatter(hfile : super::HANDLE, asegmentarray : *const super::FILE_SEGMENT_ELEMENT, nnumberofbytestoread : super::DWORD, lpreserved : super::LPDWORD, lpoverlapped : super::LPOVERLAPPED) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn RemoveDirectory2A(lppathname : windows_sys::core::PCSTR, directoryflags : DIRECTORY_FLAGS) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn RemoveDirectory2W(lppathname : windows_sys::core::PCWSTR, directoryflags : DIRECTORY_FLAGS) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn RemoveDirectoryA(lppathname : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn RemoveDirectoryW(lppathname : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetEndOfFile(hfile : super::HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetFileApisToANSI());
windows_link::link!("kernel32.dll" "system" fn SetFileApisToOEM());
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn SetFileAttributesA(lpfilename : windows_sys::core::PCSTR, dwfileattributes : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn SetFileAttributesW(lpfilename : windows_sys::core::PCWSTR, dwfileattributes : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn SetFileInformationByHandle(hfile : super::HANDLE, fileinformationclass : super::FILE_INFO_BY_HANDLE_CLASS, lpfileinformation : super::LPVOID, dwbuffersize : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn SetFileIoOverlappedRange(filehandle : super::HANDLE, overlappedrangestart : super::PUCHAR, length : super::ULONG) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn SetFilePointer(hfile : super::HANDLE, ldistancetomove : super::LONG, lpdistancetomovehigh : super::PLONG, dwmovemethod : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn SetFilePointerEx(hfile : super::HANDLE, lidistancetomove : super::LARGE_INTEGER, lpnewfilepointer : super::PLARGE_INTEGER, dwmovemethod : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn SetFileTime(hfile : super::HANDLE, lpcreationtime : *const super::FILETIME, lplastaccesstime : *const super::FILETIME, lplastwritetime : *const super::FILETIME) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn SetFileValidData(hfile : super::HANDLE, validdatalength : super::LONGLONG) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn UnlockFile(hfile : super::HANDLE, dwfileoffsetlow : super::DWORD, dwfileoffsethigh : super::DWORD, nnumberofbytestounlocklow : super::DWORD, nnumberofbytestounlockhigh : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn UnlockFileEx(hfile : super::HANDLE, dwreserved : super::DWORD, nnumberofbytestounlocklow : super::DWORD, nnumberofbytestounlockhigh : super::DWORD, lpoverlapped : super::LPOVERLAPPED) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn WriteFile(hfile : super::HANDLE, lpbuffer : super::LPCVOID, nnumberofbytestowrite : super::DWORD, lpnumberofbyteswritten : super::LPDWORD, lpoverlapped : super::LPOVERLAPPED) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn WriteFileEx(hfile : super::HANDLE, lpbuffer : super::LPCVOID, nnumberofbytestowrite : super::DWORD, lpoverlapped : super::LPOVERLAPPED, lpcompletionroutine : super::LPOVERLAPPED_COMPLETION_ROUTINE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn WriteFileGather(hfile : super::HANDLE, asegmentarray : *const super::FILE_SEGMENT_ELEMENT, nnumberofbytestowrite : super::DWORD, lpreserved : super::LPDWORD, lpoverlapped : super::LPOVERLAPPED) -> windows_sys::core::BOOL);
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct BY_HANDLE_FILE_INFORMATION {
    pub dwFileAttributes: super::DWORD,
    pub ftCreationTime: super::FILETIME,
    pub ftLastAccessTime: super::FILETIME,
    pub ftLastWriteTime: super::FILETIME,
    pub dwVolumeSerialNumber: super::DWORD,
    pub nFileSizeHigh: super::DWORD,
    pub nFileSizeLow: super::DWORD,
    pub nNumberOfLinks: super::DWORD,
    pub nFileIndexHigh: super::DWORD,
    pub nFileIndexLow: super::DWORD,
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct CREATEFILE2_EXTENDED_PARAMETERS {
    pub dwSize: super::DWORD,
    pub dwFileAttributes: super::DWORD,
    pub dwFileFlags: super::DWORD,
    pub dwSecurityQosFlags: super::DWORD,
    pub lpSecurityAttributes: super::LPSECURITY_ATTRIBUTES,
    pub hTemplateFile: super::HANDLE,
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct CREATEFILE3_EXTENDED_PARAMETERS {
    pub dwSize: super::DWORD,
    pub dwFileAttributes: super::DWORD,
    pub dwFileFlags: super::DWORD,
    pub dwSecurityQosFlags: super::DWORD,
    pub lpSecurityAttributes: super::LPSECURITY_ATTRIBUTES,
    pub hTemplateFile: super::HANDLE,
}
pub const CREATE_ALWAYS: i32 = 2;
pub const CREATE_NEW: i32 = 1;
pub type DIRECTORY_FLAGS = i32;
pub const DIRECTORY_FLAGS_DISALLOW_PATH_REDIRECTS: DIRECTORY_FLAGS = 1;
pub const DIRECTORY_FLAGS_NONE: DIRECTORY_FLAGS = 0;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DISK_SPACE_INFORMATION {
    pub ActualTotalAllocationUnits: super::ULONGLONG,
    pub ActualAvailableAllocationUnits: super::ULONGLONG,
    pub ActualPoolUnavailableAllocationUnits: super::ULONGLONG,
    pub CallerTotalAllocationUnits: super::ULONGLONG,
    pub CallerAvailableAllocationUnits: super::ULONGLONG,
    pub CallerPoolUnavailableAllocationUnits: super::ULONGLONG,
    pub UsedAllocationUnits: super::ULONGLONG,
    pub TotalReservedAllocationUnits: super::ULONGLONG,
    pub VolumeStorageReserveAllocationUnits: super::ULONGLONG,
    pub AvailableCommittedAllocationUnits: super::ULONGLONG,
    pub PoolAvailableAllocationUnits: super::ULONGLONG,
    pub SectorsPerAllocationUnit: super::DWORD,
    pub BytesPerSector: super::DWORD,
}
pub const FindStreamInfoMaxInfoLevel: STREAM_INFO_LEVELS = 1;
pub const FindStreamInfoStandard: STREAM_INFO_LEVELS = 0;
#[cfg(feature = "shared")]
pub const INVALID_FILE_ATTRIBUTES: super::DWORD = 4294967295;
#[cfg(feature = "shared")]
pub const INVALID_FILE_SIZE: super::DWORD = 4294967295;
#[cfg(feature = "shared")]
pub const INVALID_SET_FILE_POINTER: super::DWORD = 4294967295;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type LPBY_HANDLE_FILE_INFORMATION = *mut BY_HANDLE_FILE_INFORMATION;
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type LPCREATEFILE2_EXTENDED_PARAMETERS = *mut CREATEFILE2_EXTENDED_PARAMETERS;
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type LPCREATEFILE3_EXTENDED_PARAMETERS = *mut CREATEFILE3_EXTENDED_PARAMETERS;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type LPWIN32_FILE_ATTRIBUTE_DATA = *mut WIN32_FILE_ATTRIBUTE_DATA;
pub const OPEN_ALWAYS: i32 = 4;
pub const OPEN_EXISTING: i32 = 3;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PBY_HANDLE_FILE_INFORMATION = *mut BY_HANDLE_FILE_INFORMATION;
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PCREATEFILE2_EXTENDED_PARAMETERS = *mut CREATEFILE2_EXTENDED_PARAMETERS;
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PCREATEFILE3_EXTENDED_PARAMETERS = *mut CREATEFILE3_EXTENDED_PARAMETERS;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PWIN32_FIND_STREAM_DATA = *mut WIN32_FIND_STREAM_DATA;
pub type STREAM_INFO_LEVELS = i32;
pub const TRUNCATE_EXISTING: i32 = 5;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct WIN32_FILE_ATTRIBUTE_DATA {
    pub dwFileAttributes: super::DWORD,
    pub ftCreationTime: super::FILETIME,
    pub ftLastAccessTime: super::FILETIME,
    pub ftLastWriteTime: super::FILETIME,
    pub nFileSizeHigh: super::DWORD,
    pub nFileSizeLow: super::DWORD,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct WIN32_FIND_STREAM_DATA {
    pub StreamSize: super::LARGE_INTEGER,
    pub cStreamName: [super::WCHAR; 296],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for WIN32_FIND_STREAM_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
