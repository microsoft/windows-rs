#[inline]
pub unsafe fn AreFileApisANSI() -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn AreFileApisANSI() -> windows_core::BOOL);
    unsafe { AreFileApisANSI() }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn AreShortNamesEnabled(handle: super::HANDLE, enabled: *mut windows_core::BOOL) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn AreShortNamesEnabled(handle : super::HANDLE, enabled : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { AreShortNamesEnabled(handle, enabled as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CompareFileTime(lpfiletime1: *const super::FILETIME, lpfiletime2: *const super::FILETIME) -> super::LONG {
    windows_core::link!("kernel32.dll" "system" fn CompareFileTime(lpfiletime1 : *const super::FILETIME, lpfiletime2 : *const super::FILETIME) -> super::LONG);
    unsafe { CompareFileTime(lpfiletime1, lpfiletime2) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateDirectory2A<P0>(lppathname: P0, dwdesiredaccess: super::DWORD, dwsharemode: super::DWORD, directoryflags: DIRECTORY_FLAGS, lpsecurityattributes: Option<super::LPSECURITY_ATTRIBUTES>) -> super::HANDLE
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateDirectory2A(lppathname : windows_core::PCSTR, dwdesiredaccess : super::DWORD, dwsharemode : super::DWORD, directoryflags : DIRECTORY_FLAGS, lpsecurityattributes : super::LPSECURITY_ATTRIBUTES) -> super::HANDLE);
    unsafe { CreateDirectory2A(lppathname.param().abi(), dwdesiredaccess, dwsharemode, directoryflags, lpsecurityattributes.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateDirectory2W<P0>(lppathname: P0, dwdesiredaccess: super::DWORD, dwsharemode: super::DWORD, directoryflags: DIRECTORY_FLAGS, lpsecurityattributes: Option<super::LPSECURITY_ATTRIBUTES>) -> super::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateDirectory2W(lppathname : windows_core::PCWSTR, dwdesiredaccess : super::DWORD, dwsharemode : super::DWORD, directoryflags : DIRECTORY_FLAGS, lpsecurityattributes : super::LPSECURITY_ATTRIBUTES) -> super::HANDLE);
    unsafe { CreateDirectory2W(lppathname.param().abi(), dwdesiredaccess, dwsharemode, directoryflags, lpsecurityattributes.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CreateDirectoryA<P0>(lppathname: P0, lpsecurityattributes: Option<super::LPSECURITY_ATTRIBUTES>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateDirectoryA(lppathname : windows_core::PCSTR, lpsecurityattributes : super::LPSECURITY_ATTRIBUTES) -> windows_core::BOOL);
    unsafe { CreateDirectoryA(lppathname.param().abi(), lpsecurityattributes.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CreateDirectoryW<P0>(lppathname: P0, lpsecurityattributes: Option<super::LPSECURITY_ATTRIBUTES>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateDirectoryW(lppathname : windows_core::PCWSTR, lpsecurityattributes : super::LPSECURITY_ATTRIBUTES) -> windows_core::BOOL);
    unsafe { CreateDirectoryW(lppathname.param().abi(), lpsecurityattributes.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateFile2<P0>(lpfilename: P0, dwdesiredaccess: super::DWORD, dwsharemode: super::DWORD, dwcreationdisposition: super::DWORD, pcreateexparams: Option<LPCREATEFILE2_EXTENDED_PARAMETERS>) -> super::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateFile2(lpfilename : windows_core::PCWSTR, dwdesiredaccess : super::DWORD, dwsharemode : super::DWORD, dwcreationdisposition : super::DWORD, pcreateexparams : LPCREATEFILE2_EXTENDED_PARAMETERS) -> super::HANDLE);
    unsafe { CreateFile2(lpfilename.param().abi(), dwdesiredaccess, dwsharemode, dwcreationdisposition, pcreateexparams.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateFile3<P0>(lpfilename: P0, dwdesiredaccess: super::DWORD, dwsharemode: super::DWORD, dwcreationdisposition: super::DWORD, pcreateexparams: Option<LPCREATEFILE3_EXTENDED_PARAMETERS>) -> super::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateFile3(lpfilename : windows_core::PCWSTR, dwdesiredaccess : super::DWORD, dwsharemode : super::DWORD, dwcreationdisposition : super::DWORD, pcreateexparams : LPCREATEFILE3_EXTENDED_PARAMETERS) -> super::HANDLE);
    unsafe { CreateFile3(lpfilename.param().abi(), dwdesiredaccess, dwsharemode, dwcreationdisposition, pcreateexparams.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateFileA<P0>(lpfilename: P0, dwdesiredaccess: super::DWORD, dwsharemode: super::DWORD, lpsecurityattributes: Option<super::LPSECURITY_ATTRIBUTES>, dwcreationdisposition: super::DWORD, dwflagsandattributes: super::DWORD, htemplatefile: Option<super::HANDLE>) -> super::HANDLE
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateFileA(lpfilename : windows_core::PCSTR, dwdesiredaccess : super::DWORD, dwsharemode : super::DWORD, lpsecurityattributes : super::LPSECURITY_ATTRIBUTES, dwcreationdisposition : super::DWORD, dwflagsandattributes : super::DWORD, htemplatefile : super::HANDLE) -> super::HANDLE);
    unsafe { CreateFileA(lpfilename.param().abi(), dwdesiredaccess, dwsharemode, lpsecurityattributes.unwrap_or(core::mem::zeroed()) as _, dwcreationdisposition, dwflagsandattributes, htemplatefile.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateFileW<P0>(lpfilename: P0, dwdesiredaccess: super::DWORD, dwsharemode: super::DWORD, lpsecurityattributes: Option<super::LPSECURITY_ATTRIBUTES>, dwcreationdisposition: super::DWORD, dwflagsandattributes: super::DWORD, htemplatefile: Option<super::HANDLE>) -> super::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateFileW(lpfilename : windows_core::PCWSTR, dwdesiredaccess : super::DWORD, dwsharemode : super::DWORD, lpsecurityattributes : super::LPSECURITY_ATTRIBUTES, dwcreationdisposition : super::DWORD, dwflagsandattributes : super::DWORD, htemplatefile : super::HANDLE) -> super::HANDLE);
    unsafe { CreateFileW(lpfilename.param().abi(), dwdesiredaccess, dwsharemode, lpsecurityattributes.unwrap_or(core::mem::zeroed()) as _, dwcreationdisposition, dwflagsandattributes, htemplatefile.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn DefineDosDeviceW<P1, P2>(dwflags: super::DWORD, lpdevicename: P1, lptargetpath: P2) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn DefineDosDeviceW(dwflags : super::DWORD, lpdevicename : windows_core::PCWSTR, lptargetpath : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { DefineDosDeviceW(dwflags, lpdevicename.param().abi(), lptargetpath.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn DeleteFile2A<P0>(lpfilename: P0, flags: super::DWORD) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn DeleteFile2A(lpfilename : windows_core::PCSTR, flags : super::DWORD) -> windows_core::BOOL);
    unsafe { DeleteFile2A(lpfilename.param().abi(), flags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn DeleteFile2W<P0>(lpfilename: P0, flags: super::DWORD) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn DeleteFile2W(lpfilename : windows_core::PCWSTR, flags : super::DWORD) -> windows_core::BOOL);
    unsafe { DeleteFile2W(lpfilename.param().abi(), flags) }
}
#[inline]
pub unsafe fn DeleteFileA<P0>(lpfilename: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn DeleteFileA(lpfilename : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { DeleteFileA(lpfilename.param().abi()) }
}
#[inline]
pub unsafe fn DeleteFileW<P0>(lpfilename: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn DeleteFileW(lpfilename : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { DeleteFileW(lpfilename.param().abi()) }
}
#[inline]
pub unsafe fn DeleteVolumeMountPointW<P0>(lpszvolumemountpoint: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn DeleteVolumeMountPointW(lpszvolumemountpoint : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { DeleteVolumeMountPointW(lpszvolumemountpoint.param().abi()) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn FileTimeToLocalFileTime(lpfiletime: *const super::FILETIME, lplocalfiletime: super::LPFILETIME) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn FileTimeToLocalFileTime(lpfiletime : *const super::FILETIME, lplocalfiletime : super::LPFILETIME) -> windows_core::BOOL);
    unsafe { FileTimeToLocalFileTime(lpfiletime, lplocalfiletime as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn FindClose(hfindfile: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn FindClose(hfindfile : super::HANDLE) -> windows_core::BOOL);
    unsafe { FindClose(hfindfile as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn FindCloseChangeNotification(hchangehandle: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn FindCloseChangeNotification(hchangehandle : super::HANDLE) -> windows_core::BOOL);
    unsafe { FindCloseChangeNotification(hchangehandle) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FindFirstChangeNotificationA<P0>(lppathname: P0, bwatchsubtree: bool, dwnotifyfilter: super::DWORD) -> super::HANDLE
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn FindFirstChangeNotificationA(lppathname : windows_core::PCSTR, bwatchsubtree : windows_core::BOOL, dwnotifyfilter : super::DWORD) -> super::HANDLE);
    unsafe { FindFirstChangeNotificationA(lppathname.param().abi(), bwatchsubtree.into(), dwnotifyfilter) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FindFirstChangeNotificationW<P0>(lppathname: P0, bwatchsubtree: bool, dwnotifyfilter: super::DWORD) -> super::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn FindFirstChangeNotificationW(lppathname : windows_core::PCWSTR, bwatchsubtree : windows_core::BOOL, dwnotifyfilter : super::DWORD) -> super::HANDLE);
    unsafe { FindFirstChangeNotificationW(lppathname.param().abi(), bwatchsubtree.into(), dwnotifyfilter) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FindFirstFileA<P0>(lpfilename: P0, lpfindfiledata: super::LPWIN32_FIND_DATAA) -> super::HANDLE
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn FindFirstFileA(lpfilename : windows_core::PCSTR, lpfindfiledata : super::LPWIN32_FIND_DATAA) -> super::HANDLE);
    unsafe { FindFirstFileA(lpfilename.param().abi(), lpfindfiledata as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FindFirstFileExA<P0>(lpfilename: P0, finfolevelid: super::FINDEX_INFO_LEVELS, lpfindfiledata: super::LPVOID, fsearchop: super::FINDEX_SEARCH_OPS, lpsearchfilter: super::LPVOID, dwadditionalflags: super::DWORD) -> super::HANDLE
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn FindFirstFileExA(lpfilename : windows_core::PCSTR, finfolevelid : super::FINDEX_INFO_LEVELS, lpfindfiledata : super::LPVOID, fsearchop : super::FINDEX_SEARCH_OPS, lpsearchfilter : super::LPVOID, dwadditionalflags : super::DWORD) -> super::HANDLE);
    unsafe { FindFirstFileExA(lpfilename.param().abi(), finfolevelid, lpfindfiledata as _, fsearchop, lpsearchfilter as _, dwadditionalflags) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FindFirstFileExW<P0>(lpfilename: P0, finfolevelid: super::FINDEX_INFO_LEVELS, lpfindfiledata: super::LPVOID, fsearchop: super::FINDEX_SEARCH_OPS, lpsearchfilter: super::LPVOID, dwadditionalflags: super::DWORD) -> super::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn FindFirstFileExW(lpfilename : windows_core::PCWSTR, finfolevelid : super::FINDEX_INFO_LEVELS, lpfindfiledata : super::LPVOID, fsearchop : super::FINDEX_SEARCH_OPS, lpsearchfilter : super::LPVOID, dwadditionalflags : super::DWORD) -> super::HANDLE);
    unsafe { FindFirstFileExW(lpfilename.param().abi(), finfolevelid, lpfindfiledata as _, fsearchop, lpsearchfilter as _, dwadditionalflags) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FindFirstFileNameW<P0>(lpfilename: P0, dwflags: super::DWORD, stringlength: super::LPDWORD, linkname: windows_core::PWSTR) -> super::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn FindFirstFileNameW(lpfilename : windows_core::PCWSTR, dwflags : super::DWORD, stringlength : super::LPDWORD, linkname : windows_core::PWSTR) -> super::HANDLE);
    unsafe { FindFirstFileNameW(lpfilename.param().abi(), dwflags, stringlength as _, linkname) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FindFirstFileW<P0>(lpfilename: P0, lpfindfiledata: super::LPWIN32_FIND_DATAW) -> super::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn FindFirstFileW(lpfilename : windows_core::PCWSTR, lpfindfiledata : super::LPWIN32_FIND_DATAW) -> super::HANDLE);
    unsafe { FindFirstFileW(lpfilename.param().abi(), lpfindfiledata as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FindFirstStreamW<P0>(lpfilename: P0, infolevel: STREAM_INFO_LEVELS, lpfindstreamdata: super::LPVOID, dwflags: super::DWORD) -> super::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn FindFirstStreamW(lpfilename : windows_core::PCWSTR, infolevel : STREAM_INFO_LEVELS, lpfindstreamdata : super::LPVOID, dwflags : super::DWORD) -> super::HANDLE);
    unsafe { FindFirstStreamW(lpfilename.param().abi(), infolevel, lpfindstreamdata as _, dwflags) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FindFirstVolumeW(lpszvolumename: windows_core::PWSTR, cchbufferlength: super::DWORD) -> super::HANDLE {
    windows_core::link!("kernel32.dll" "system" fn FindFirstVolumeW(lpszvolumename : windows_core::PWSTR, cchbufferlength : super::DWORD) -> super::HANDLE);
    unsafe { FindFirstVolumeW(lpszvolumename, cchbufferlength) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn FindNextChangeNotification(hchangehandle: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn FindNextChangeNotification(hchangehandle : super::HANDLE) -> windows_core::BOOL);
    unsafe { FindNextChangeNotification(hchangehandle) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FindNextFileA(hfindfile: super::HANDLE, lpfindfiledata: super::LPWIN32_FIND_DATAA) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn FindNextFileA(hfindfile : super::HANDLE, lpfindfiledata : super::LPWIN32_FIND_DATAA) -> windows_core::BOOL);
    unsafe { FindNextFileA(hfindfile, lpfindfiledata as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn FindNextFileNameW(hfindstream: super::HANDLE, stringlength: super::LPDWORD, linkname: windows_core::PWSTR) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn FindNextFileNameW(hfindstream : super::HANDLE, stringlength : super::LPDWORD, linkname : windows_core::PWSTR) -> windows_core::BOOL);
    unsafe { FindNextFileNameW(hfindstream, stringlength as _, linkname) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FindNextFileW(hfindfile: super::HANDLE, lpfindfiledata: super::LPWIN32_FIND_DATAW) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn FindNextFileW(hfindfile : super::HANDLE, lpfindfiledata : super::LPWIN32_FIND_DATAW) -> windows_core::BOOL);
    unsafe { FindNextFileW(hfindfile, lpfindfiledata as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn FindNextStreamW(hfindstream: super::HANDLE, lpfindstreamdata: super::LPVOID) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn FindNextStreamW(hfindstream : super::HANDLE, lpfindstreamdata : super::LPVOID) -> windows_core::BOOL);
    unsafe { FindNextStreamW(hfindstream, lpfindstreamdata as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FindNextVolumeW(hfindvolume: super::HANDLE, lpszvolumename: windows_core::PWSTR, cchbufferlength: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn FindNextVolumeW(hfindvolume : super::HANDLE, lpszvolumename : windows_core::PWSTR, cchbufferlength : super::DWORD) -> windows_core::BOOL);
    unsafe { FindNextVolumeW(hfindvolume as _, lpszvolumename, cchbufferlength) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn FindVolumeClose(hfindvolume: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn FindVolumeClose(hfindvolume : super::HANDLE) -> windows_core::BOOL);
    unsafe { FindVolumeClose(hfindvolume) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn FlushFileBuffers(hfile: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn FlushFileBuffers(hfile : super::HANDLE) -> windows_core::BOOL);
    unsafe { FlushFileBuffers(hfile) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn GetCompressedFileSizeA<P0>(lpfilename: P0, lpfilesizehigh: Option<super::LPDWORD>) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetCompressedFileSizeA(lpfilename : windows_core::PCSTR, lpfilesizehigh : super::LPDWORD) -> super::DWORD);
    unsafe { GetCompressedFileSizeA(lpfilename.param().abi(), lpfilesizehigh.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn GetCompressedFileSizeW<P0>(lpfilename: P0, lpfilesizehigh: Option<super::LPDWORD>) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetCompressedFileSizeW(lpfilename : windows_core::PCWSTR, lpfilesizehigh : super::LPDWORD) -> super::DWORD);
    unsafe { GetCompressedFileSizeW(lpfilename.param().abi(), lpfilesizehigh.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn GetDiskFreeSpaceA<P0>(lprootpathname: P0, lpsectorspercluster: Option<super::LPDWORD>, lpbytespersector: Option<super::LPDWORD>, lpnumberoffreeclusters: Option<super::LPDWORD>, lptotalnumberofclusters: Option<super::LPDWORD>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetDiskFreeSpaceA(lprootpathname : windows_core::PCSTR, lpsectorspercluster : super::LPDWORD, lpbytespersector : super::LPDWORD, lpnumberoffreeclusters : super::LPDWORD, lptotalnumberofclusters : super::LPDWORD) -> windows_core::BOOL);
    unsafe { GetDiskFreeSpaceA(lprootpathname.param().abi(), lpsectorspercluster.unwrap_or(core::mem::zeroed()) as _, lpbytespersector.unwrap_or(core::mem::zeroed()) as _, lpnumberoffreeclusters.unwrap_or(core::mem::zeroed()) as _, lptotalnumberofclusters.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetDiskFreeSpaceExA<P0>(lpdirectoryname: P0, lpfreebytesavailabletocaller: Option<super::PULARGE_INTEGER>, lptotalnumberofbytes: Option<super::PULARGE_INTEGER>, lptotalnumberoffreebytes: Option<super::PULARGE_INTEGER>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetDiskFreeSpaceExA(lpdirectoryname : windows_core::PCSTR, lpfreebytesavailabletocaller : super::PULARGE_INTEGER, lptotalnumberofbytes : super::PULARGE_INTEGER, lptotalnumberoffreebytes : super::PULARGE_INTEGER) -> windows_core::BOOL);
    unsafe { GetDiskFreeSpaceExA(lpdirectoryname.param().abi(), lpfreebytesavailabletocaller.unwrap_or(core::mem::zeroed()) as _, lptotalnumberofbytes.unwrap_or(core::mem::zeroed()) as _, lptotalnumberoffreebytes.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetDiskFreeSpaceExW<P0>(lpdirectoryname: P0, lpfreebytesavailabletocaller: Option<super::PULARGE_INTEGER>, lptotalnumberofbytes: Option<super::PULARGE_INTEGER>, lptotalnumberoffreebytes: Option<super::PULARGE_INTEGER>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetDiskFreeSpaceExW(lpdirectoryname : windows_core::PCWSTR, lpfreebytesavailabletocaller : super::PULARGE_INTEGER, lptotalnumberofbytes : super::PULARGE_INTEGER, lptotalnumberoffreebytes : super::PULARGE_INTEGER) -> windows_core::BOOL);
    unsafe { GetDiskFreeSpaceExW(lpdirectoryname.param().abi(), lpfreebytesavailabletocaller.unwrap_or(core::mem::zeroed()) as _, lptotalnumberofbytes.unwrap_or(core::mem::zeroed()) as _, lptotalnumberoffreebytes.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn GetDiskFreeSpaceW<P0>(lprootpathname: P0, lpsectorspercluster: Option<super::LPDWORD>, lpbytespersector: Option<super::LPDWORD>, lpnumberoffreeclusters: Option<super::LPDWORD>, lptotalnumberofclusters: Option<super::LPDWORD>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetDiskFreeSpaceW(lprootpathname : windows_core::PCWSTR, lpsectorspercluster : super::LPDWORD, lpbytespersector : super::LPDWORD, lpnumberoffreeclusters : super::LPDWORD, lptotalnumberofclusters : super::LPDWORD) -> windows_core::BOOL);
    unsafe { GetDiskFreeSpaceW(lprootpathname.param().abi(), lpsectorspercluster.unwrap_or(core::mem::zeroed()) as _, lpbytespersector.unwrap_or(core::mem::zeroed()) as _, lpnumberoffreeclusters.unwrap_or(core::mem::zeroed()) as _, lptotalnumberofclusters.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetDiskSpaceInformationA<P0>(rootpath: P0, diskspaceinfo: *mut DISK_SPACE_INFORMATION) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetDiskSpaceInformationA(rootpath : windows_core::PCSTR, diskspaceinfo : *mut DISK_SPACE_INFORMATION) -> windows_core::HRESULT);
    unsafe { GetDiskSpaceInformationA(rootpath.param().abi(), diskspaceinfo as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetDiskSpaceInformationW<P0>(rootpath: P0, diskspaceinfo: *mut DISK_SPACE_INFORMATION) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetDiskSpaceInformationW(rootpath : windows_core::PCWSTR, diskspaceinfo : *mut DISK_SPACE_INFORMATION) -> windows_core::HRESULT);
    unsafe { GetDiskSpaceInformationW(rootpath.param().abi(), diskspaceinfo as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetDriveTypeA<P0>(lprootpathname: P0) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetDriveTypeA(lprootpathname : windows_core::PCSTR) -> super::UINT);
    unsafe { GetDriveTypeA(lprootpathname.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetDriveTypeW<P0>(lprootpathname: P0) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetDriveTypeW(lprootpathname : windows_core::PCWSTR) -> super::UINT);
    unsafe { GetDriveTypeW(lprootpathname.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetFileAttributesA<P0>(lpfilename: P0) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetFileAttributesA(lpfilename : windows_core::PCSTR) -> super::DWORD);
    unsafe { GetFileAttributesA(lpfilename.param().abi()) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef"))]
#[inline]
pub unsafe fn GetFileAttributesExA<P0>(lpfilename: P0, finfolevelid: super::GET_FILEEX_INFO_LEVELS, lpfileinformation: super::LPVOID) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetFileAttributesExA(lpfilename : windows_core::PCSTR, finfolevelid : super::GET_FILEEX_INFO_LEVELS, lpfileinformation : super::LPVOID) -> windows_core::BOOL);
    unsafe { GetFileAttributesExA(lpfilename.param().abi(), finfolevelid, lpfileinformation as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef"))]
#[inline]
pub unsafe fn GetFileAttributesExW<P0>(lpfilename: P0, finfolevelid: super::GET_FILEEX_INFO_LEVELS, lpfileinformation: super::LPVOID) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetFileAttributesExW(lpfilename : windows_core::PCWSTR, finfolevelid : super::GET_FILEEX_INFO_LEVELS, lpfileinformation : super::LPVOID) -> windows_core::BOOL);
    unsafe { GetFileAttributesExW(lpfilename.param().abi(), finfolevelid, lpfileinformation as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetFileAttributesW<P0>(lpfilename: P0) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetFileAttributesW(lpfilename : windows_core::PCWSTR) -> super::DWORD);
    unsafe { GetFileAttributesW(lpfilename.param().abi()) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetFileInformationByHandle(hfile: super::HANDLE, lpfileinformation: LPBY_HANDLE_FILE_INFORMATION) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetFileInformationByHandle(hfile : super::HANDLE, lpfileinformation : LPBY_HANDLE_FILE_INFORMATION) -> windows_core::BOOL);
    unsafe { GetFileInformationByHandle(hfile, lpfileinformation as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetFileSize(hfile: super::HANDLE, lpfilesizehigh: Option<super::LPDWORD>) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn GetFileSize(hfile : super::HANDLE, lpfilesizehigh : super::LPDWORD) -> super::DWORD);
    unsafe { GetFileSize(hfile, lpfilesizehigh.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetFileSizeEx(hfile: super::HANDLE, lpfilesize: super::PLARGE_INTEGER) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetFileSizeEx(hfile : super::HANDLE, lpfilesize : super::PLARGE_INTEGER) -> windows_core::BOOL);
    unsafe { GetFileSizeEx(hfile, lpfilesize as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetFileTime(hfile: super::HANDLE, lpcreationtime: Option<super::LPFILETIME>, lplastaccesstime: Option<super::LPFILETIME>, lplastwritetime: Option<super::LPFILETIME>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetFileTime(hfile : super::HANDLE, lpcreationtime : super::LPFILETIME, lplastaccesstime : super::LPFILETIME, lplastwritetime : super::LPFILETIME) -> windows_core::BOOL);
    unsafe { GetFileTime(hfile, lpcreationtime.unwrap_or(core::mem::zeroed()) as _, lplastaccesstime.unwrap_or(core::mem::zeroed()) as _, lplastwritetime.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetFileType(hfile: super::HANDLE) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn GetFileType(hfile : super::HANDLE) -> super::DWORD);
    unsafe { GetFileType(hfile) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetFinalPathNameByHandleA(hfile: super::HANDLE, lpszfilepath: windows_core::PSTR, cchfilepath: super::DWORD, dwflags: super::DWORD) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn GetFinalPathNameByHandleA(hfile : super::HANDLE, lpszfilepath : windows_core::PSTR, cchfilepath : super::DWORD, dwflags : super::DWORD) -> super::DWORD);
    unsafe { GetFinalPathNameByHandleA(hfile, lpszfilepath, cchfilepath, dwflags) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetFinalPathNameByHandleW(hfile: super::HANDLE, lpszfilepath: windows_core::PWSTR, cchfilepath: super::DWORD, dwflags: super::DWORD) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn GetFinalPathNameByHandleW(hfile : super::HANDLE, lpszfilepath : windows_core::PWSTR, cchfilepath : super::DWORD, dwflags : super::DWORD) -> super::DWORD);
    unsafe { GetFinalPathNameByHandleW(hfile, lpszfilepath, cchfilepath, dwflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetFullPathNameA<P0>(lpfilename: P0, nbufferlength: super::DWORD, lpbuffer: Option<windows_core::PSTR>, lpfilepart: Option<*mut windows_core::PSTR>) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetFullPathNameA(lpfilename : windows_core::PCSTR, nbufferlength : super::DWORD, lpbuffer : windows_core::PSTR, lpfilepart : *mut windows_core::PSTR) -> super::DWORD);
    unsafe { GetFullPathNameA(lpfilename.param().abi(), nbufferlength, lpbuffer.unwrap_or(core::mem::zeroed()) as _, lpfilepart.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetFullPathNameW<P0>(lpfilename: P0, nbufferlength: super::DWORD, lpbuffer: Option<windows_core::PWSTR>, lpfilepart: Option<*mut windows_core::PWSTR>) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetFullPathNameW(lpfilename : windows_core::PCWSTR, nbufferlength : super::DWORD, lpbuffer : windows_core::PWSTR, lpfilepart : *mut windows_core::PWSTR) -> super::DWORD);
    unsafe { GetFullPathNameW(lpfilename.param().abi(), nbufferlength, lpbuffer.unwrap_or(core::mem::zeroed()) as _, lpfilepart.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetLogicalDriveStringsW(nbufferlength: super::DWORD, lpbuffer: Option<windows_core::PWSTR>) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn GetLogicalDriveStringsW(nbufferlength : super::DWORD, lpbuffer : windows_core::PWSTR) -> super::DWORD);
    unsafe { GetLogicalDriveStringsW(nbufferlength, lpbuffer.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetLogicalDrives() -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn GetLogicalDrives() -> super::DWORD);
    unsafe { GetLogicalDrives() }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetLongPathNameA<P0>(lpszshortpath: P0, lpszlongpath: Option<windows_core::PSTR>, cchbuffer: super::DWORD) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetLongPathNameA(lpszshortpath : windows_core::PCSTR, lpszlongpath : windows_core::PSTR, cchbuffer : super::DWORD) -> super::DWORD);
    unsafe { GetLongPathNameA(lpszshortpath.param().abi(), lpszlongpath.unwrap_or(core::mem::zeroed()) as _, cchbuffer) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetLongPathNameW<P0>(lpszshortpath: P0, lpszlongpath: Option<windows_core::PWSTR>, cchbuffer: super::DWORD) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetLongPathNameW(lpszshortpath : windows_core::PCWSTR, lpszlongpath : windows_core::PWSTR, cchbuffer : super::DWORD) -> super::DWORD);
    unsafe { GetLongPathNameW(lpszshortpath.param().abi(), lpszlongpath.unwrap_or(core::mem::zeroed()) as _, cchbuffer) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetShortPathNameW<P0>(lpszlongpath: P0, lpszshortpath: Option<windows_core::PWSTR>, cchbuffer: super::DWORD) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetShortPathNameW(lpszlongpath : windows_core::PCWSTR, lpszshortpath : windows_core::PWSTR, cchbuffer : super::DWORD) -> super::DWORD);
    unsafe { GetShortPathNameW(lpszlongpath.param().abi(), lpszshortpath.unwrap_or(core::mem::zeroed()) as _, cchbuffer) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetTempFileNameA<P0, P1>(lppathname: P0, lpprefixstring: P1, uunique: super::UINT, lptempfilename: windows_core::PSTR) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetTempFileNameA(lppathname : windows_core::PCSTR, lpprefixstring : windows_core::PCSTR, uunique : super::UINT, lptempfilename : windows_core::PSTR) -> super::UINT);
    unsafe { GetTempFileNameA(lppathname.param().abi(), lpprefixstring.param().abi(), uunique, lptempfilename) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetTempFileNameW<P0, P1>(lppathname: P0, lpprefixstring: P1, uunique: super::UINT, lptempfilename: windows_core::PWSTR) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetTempFileNameW(lppathname : windows_core::PCWSTR, lpprefixstring : windows_core::PCWSTR, uunique : super::UINT, lptempfilename : windows_core::PWSTR) -> super::UINT);
    unsafe { GetTempFileNameW(lppathname.param().abi(), lpprefixstring.param().abi(), uunique, lptempfilename) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetTempPath2A(bufferlength: super::DWORD, buffer: Option<windows_core::PSTR>) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn GetTempPath2A(bufferlength : super::DWORD, buffer : windows_core::PSTR) -> super::DWORD);
    unsafe { GetTempPath2A(bufferlength, buffer.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetTempPath2W(bufferlength: super::DWORD, buffer: Option<windows_core::PWSTR>) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn GetTempPath2W(bufferlength : super::DWORD, buffer : windows_core::PWSTR) -> super::DWORD);
    unsafe { GetTempPath2W(bufferlength, buffer.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetTempPathA(nbufferlength: super::DWORD, lpbuffer: Option<windows_core::PSTR>) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn GetTempPathA(nbufferlength : super::DWORD, lpbuffer : windows_core::PSTR) -> super::DWORD);
    unsafe { GetTempPathA(nbufferlength, lpbuffer.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetTempPathW(nbufferlength: super::DWORD, lpbuffer: Option<windows_core::PWSTR>) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn GetTempPathW(nbufferlength : super::DWORD, lpbuffer : windows_core::PWSTR) -> super::DWORD);
    unsafe { GetTempPathW(nbufferlength, lpbuffer.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn GetVolumeInformationA<P0>(lprootpathname: P0, lpvolumenamebuffer: Option<windows_core::PSTR>, nvolumenamesize: super::DWORD, lpvolumeserialnumber: Option<super::LPDWORD>, lpmaximumcomponentlength: Option<super::LPDWORD>, lpfilesystemflags: Option<super::LPDWORD>, lpfilesystemnamebuffer: Option<windows_core::PSTR>, nfilesystemnamesize: super::DWORD) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetVolumeInformationA(lprootpathname : windows_core::PCSTR, lpvolumenamebuffer : windows_core::PSTR, nvolumenamesize : super::DWORD, lpvolumeserialnumber : super::LPDWORD, lpmaximumcomponentlength : super::LPDWORD, lpfilesystemflags : super::LPDWORD, lpfilesystemnamebuffer : windows_core::PSTR, nfilesystemnamesize : super::DWORD) -> windows_core::BOOL);
    unsafe { GetVolumeInformationA(lprootpathname.param().abi(), lpvolumenamebuffer.unwrap_or(core::mem::zeroed()) as _, nvolumenamesize, lpvolumeserialnumber.unwrap_or(core::mem::zeroed()) as _, lpmaximumcomponentlength.unwrap_or(core::mem::zeroed()) as _, lpfilesystemflags.unwrap_or(core::mem::zeroed()) as _, lpfilesystemnamebuffer.unwrap_or(core::mem::zeroed()) as _, nfilesystemnamesize) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetVolumeInformationByHandleW(hfile: super::HANDLE, lpvolumenamebuffer: Option<windows_core::PWSTR>, nvolumenamesize: super::DWORD, lpvolumeserialnumber: Option<super::LPDWORD>, lpmaximumcomponentlength: Option<super::LPDWORD>, lpfilesystemflags: Option<super::LPDWORD>, lpfilesystemnamebuffer: Option<windows_core::PWSTR>, nfilesystemnamesize: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetVolumeInformationByHandleW(hfile : super::HANDLE, lpvolumenamebuffer : windows_core::PWSTR, nvolumenamesize : super::DWORD, lpvolumeserialnumber : super::LPDWORD, lpmaximumcomponentlength : super::LPDWORD, lpfilesystemflags : super::LPDWORD, lpfilesystemnamebuffer : windows_core::PWSTR, nfilesystemnamesize : super::DWORD) -> windows_core::BOOL);
    unsafe { GetVolumeInformationByHandleW(hfile, lpvolumenamebuffer.unwrap_or(core::mem::zeroed()) as _, nvolumenamesize, lpvolumeserialnumber.unwrap_or(core::mem::zeroed()) as _, lpmaximumcomponentlength.unwrap_or(core::mem::zeroed()) as _, lpfilesystemflags.unwrap_or(core::mem::zeroed()) as _, lpfilesystemnamebuffer.unwrap_or(core::mem::zeroed()) as _, nfilesystemnamesize) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn GetVolumeInformationW<P0>(lprootpathname: P0, lpvolumenamebuffer: Option<windows_core::PWSTR>, nvolumenamesize: super::DWORD, lpvolumeserialnumber: Option<super::LPDWORD>, lpmaximumcomponentlength: Option<super::LPDWORD>, lpfilesystemflags: Option<super::LPDWORD>, lpfilesystemnamebuffer: Option<windows_core::PWSTR>, nfilesystemnamesize: super::DWORD) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetVolumeInformationW(lprootpathname : windows_core::PCWSTR, lpvolumenamebuffer : windows_core::PWSTR, nvolumenamesize : super::DWORD, lpvolumeserialnumber : super::LPDWORD, lpmaximumcomponentlength : super::LPDWORD, lpfilesystemflags : super::LPDWORD, lpfilesystemnamebuffer : windows_core::PWSTR, nfilesystemnamesize : super::DWORD) -> windows_core::BOOL);
    unsafe { GetVolumeInformationW(lprootpathname.param().abi(), lpvolumenamebuffer.unwrap_or(core::mem::zeroed()) as _, nvolumenamesize, lpvolumeserialnumber.unwrap_or(core::mem::zeroed()) as _, lpmaximumcomponentlength.unwrap_or(core::mem::zeroed()) as _, lpfilesystemflags.unwrap_or(core::mem::zeroed()) as _, lpfilesystemnamebuffer.unwrap_or(core::mem::zeroed()) as _, nfilesystemnamesize) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetVolumeNameForVolumeMountPointW<P0>(lpszvolumemountpoint: P0, lpszvolumename: windows_core::PWSTR, cchbufferlength: super::DWORD) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetVolumeNameForVolumeMountPointW(lpszvolumemountpoint : windows_core::PCWSTR, lpszvolumename : windows_core::PWSTR, cchbufferlength : super::DWORD) -> windows_core::BOOL);
    unsafe { GetVolumeNameForVolumeMountPointW(lpszvolumemountpoint.param().abi(), lpszvolumename, cchbufferlength) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetVolumePathNameW<P0>(lpszfilename: P0, lpszvolumepathname: windows_core::PWSTR, cchbufferlength: super::DWORD) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetVolumePathNameW(lpszfilename : windows_core::PCWSTR, lpszvolumepathname : windows_core::PWSTR, cchbufferlength : super::DWORD) -> windows_core::BOOL);
    unsafe { GetVolumePathNameW(lpszfilename.param().abi(), lpszvolumepathname, cchbufferlength) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetVolumePathNamesForVolumeNameW<P0>(lpszvolumename: P0, lpszvolumepathnames: Option<super::LPWCH>, cchbufferlength: super::DWORD, lpcchreturnlength: super::PDWORD) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetVolumePathNamesForVolumeNameW(lpszvolumename : windows_core::PCWSTR, lpszvolumepathnames : super::LPWCH, cchbufferlength : super::DWORD, lpcchreturnlength : super::PDWORD) -> windows_core::BOOL);
    unsafe { GetVolumePathNamesForVolumeNameW(lpszvolumename.param().abi(), lpszvolumepathnames.unwrap_or(core::mem::zeroed()) as _, cchbufferlength, lpcchreturnlength as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn LocalFileTimeToFileTime(lplocalfiletime: *const super::FILETIME, lpfiletime: super::LPFILETIME) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn LocalFileTimeToFileTime(lplocalfiletime : *const super::FILETIME, lpfiletime : super::LPFILETIME) -> windows_core::BOOL);
    unsafe { LocalFileTimeToFileTime(lplocalfiletime, lpfiletime as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn LockFile(hfile: super::HANDLE, dwfileoffsetlow: super::DWORD, dwfileoffsethigh: super::DWORD, nnumberofbytestolocklow: super::DWORD, nnumberofbytestolockhigh: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn LockFile(hfile : super::HANDLE, dwfileoffsetlow : super::DWORD, dwfileoffsethigh : super::DWORD, nnumberofbytestolocklow : super::DWORD, nnumberofbytestolockhigh : super::DWORD) -> windows_core::BOOL);
    unsafe { LockFile(hfile, dwfileoffsetlow, dwfileoffsethigh, nnumberofbytestolocklow, nnumberofbytestolockhigh) }
}
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn LockFileEx(hfile: super::HANDLE, dwflags: super::DWORD, dwreserved: super::DWORD, nnumberofbytestolocklow: super::DWORD, nnumberofbytestolockhigh: super::DWORD, lpoverlapped: super::LPOVERLAPPED) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn LockFileEx(hfile : super::HANDLE, dwflags : super::DWORD, dwreserved : super::DWORD, nnumberofbytestolocklow : super::DWORD, nnumberofbytestolockhigh : super::DWORD, lpoverlapped : super::LPOVERLAPPED) -> windows_core::BOOL);
    unsafe { LockFileEx(hfile, dwflags, dwreserved, nnumberofbytestolocklow, nnumberofbytestolockhigh, lpoverlapped as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn QueryDosDeviceW<P0>(lpdevicename: P0, lptargetpath: Option<windows_core::PWSTR>, ucchmax: super::DWORD) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn QueryDosDeviceW(lpdevicename : windows_core::PCWSTR, lptargetpath : windows_core::PWSTR, ucchmax : super::DWORD) -> super::DWORD);
    unsafe { QueryDosDeviceW(lpdevicename.param().abi(), lptargetpath.unwrap_or(core::mem::zeroed()) as _, ucchmax) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn ReadFile(hfile: super::HANDLE, lpbuffer: Option<super::LPVOID>, nnumberofbytestoread: super::DWORD, lpnumberofbytesread: Option<super::LPDWORD>, lpoverlapped: Option<super::LPOVERLAPPED>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn ReadFile(hfile : super::HANDLE, lpbuffer : super::LPVOID, nnumberofbytestoread : super::DWORD, lpnumberofbytesread : super::LPDWORD, lpoverlapped : super::LPOVERLAPPED) -> windows_core::BOOL);
    unsafe { ReadFile(hfile, lpbuffer.unwrap_or(core::mem::zeroed()) as _, nnumberofbytestoread, lpnumberofbytesread.unwrap_or(core::mem::zeroed()) as _, lpoverlapped.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn ReadFileEx(hfile: super::HANDLE, lpbuffer: Option<super::LPVOID>, nnumberofbytestoread: super::DWORD, lpoverlapped: super::LPOVERLAPPED, lpcompletionroutine: super::LPOVERLAPPED_COMPLETION_ROUTINE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn ReadFileEx(hfile : super::HANDLE, lpbuffer : super::LPVOID, nnumberofbytestoread : super::DWORD, lpoverlapped : super::LPOVERLAPPED, lpcompletionroutine : super::LPOVERLAPPED_COMPLETION_ROUTINE) -> windows_core::BOOL);
    unsafe { ReadFileEx(hfile, lpbuffer.unwrap_or(core::mem::zeroed()) as _, nnumberofbytestoread, lpoverlapped as _, lpcompletionroutine) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn ReadFileScatter(hfile: super::HANDLE, asegmentarray: *const super::FILE_SEGMENT_ELEMENT, nnumberofbytestoread: super::DWORD, lpreserved: super::LPDWORD, lpoverlapped: super::LPOVERLAPPED) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn ReadFileScatter(hfile : super::HANDLE, asegmentarray : *const super::FILE_SEGMENT_ELEMENT, nnumberofbytestoread : super::DWORD, lpreserved : super::LPDWORD, lpoverlapped : super::LPOVERLAPPED) -> windows_core::BOOL);
    unsafe { ReadFileScatter(hfile, asegmentarray, nnumberofbytestoread, lpreserved as _, lpoverlapped as _) }
}
#[inline]
pub unsafe fn RemoveDirectory2A<P0>(lppathname: P0, directoryflags: DIRECTORY_FLAGS) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn RemoveDirectory2A(lppathname : windows_core::PCSTR, directoryflags : DIRECTORY_FLAGS) -> windows_core::BOOL);
    unsafe { RemoveDirectory2A(lppathname.param().abi(), directoryflags) }
}
#[inline]
pub unsafe fn RemoveDirectory2W<P0>(lppathname: P0, directoryflags: DIRECTORY_FLAGS) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn RemoveDirectory2W(lppathname : windows_core::PCWSTR, directoryflags : DIRECTORY_FLAGS) -> windows_core::BOOL);
    unsafe { RemoveDirectory2W(lppathname.param().abi(), directoryflags) }
}
#[inline]
pub unsafe fn RemoveDirectoryA<P0>(lppathname: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn RemoveDirectoryA(lppathname : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { RemoveDirectoryA(lppathname.param().abi()) }
}
#[inline]
pub unsafe fn RemoveDirectoryW<P0>(lppathname: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn RemoveDirectoryW(lppathname : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { RemoveDirectoryW(lppathname.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SetEndOfFile(hfile: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetEndOfFile(hfile : super::HANDLE) -> windows_core::BOOL);
    unsafe { SetEndOfFile(hfile) }
}
#[inline]
pub unsafe fn SetFileApisToANSI() {
    windows_core::link!("kernel32.dll" "system" fn SetFileApisToANSI());
    unsafe { SetFileApisToANSI() }
}
#[inline]
pub unsafe fn SetFileApisToOEM() {
    windows_core::link!("kernel32.dll" "system" fn SetFileApisToOEM());
    unsafe { SetFileApisToOEM() }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn SetFileAttributesA<P0>(lpfilename: P0, dwfileattributes: super::DWORD) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn SetFileAttributesA(lpfilename : windows_core::PCSTR, dwfileattributes : super::DWORD) -> windows_core::BOOL);
    unsafe { SetFileAttributesA(lpfilename.param().abi(), dwfileattributes) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn SetFileAttributesW<P0>(lpfilename: P0, dwfileattributes: super::DWORD) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn SetFileAttributesW(lpfilename : windows_core::PCWSTR, dwfileattributes : super::DWORD) -> windows_core::BOOL);
    unsafe { SetFileAttributesW(lpfilename.param().abi(), dwfileattributes) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetFileInformationByHandle(hfile: super::HANDLE, fileinformationclass: super::FILE_INFO_BY_HANDLE_CLASS, lpfileinformation: super::LPVOID, dwbuffersize: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetFileInformationByHandle(hfile : super::HANDLE, fileinformationclass : super::FILE_INFO_BY_HANDLE_CLASS, lpfileinformation : super::LPVOID, dwbuffersize : super::DWORD) -> windows_core::BOOL);
    unsafe { SetFileInformationByHandle(hfile, fileinformationclass, lpfileinformation, dwbuffersize) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetFileIoOverlappedRange(filehandle: super::HANDLE, overlappedrangestart: super::PUCHAR, length: super::ULONG) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetFileIoOverlappedRange(filehandle : super::HANDLE, overlappedrangestart : super::PUCHAR, length : super::ULONG) -> windows_core::BOOL);
    unsafe { SetFileIoOverlappedRange(filehandle, overlappedrangestart, length) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetFilePointer(hfile: super::HANDLE, ldistancetomove: super::LONG, lpdistancetomovehigh: Option<super::PLONG>, dwmovemethod: super::DWORD) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn SetFilePointer(hfile : super::HANDLE, ldistancetomove : super::LONG, lpdistancetomovehigh : super::PLONG, dwmovemethod : super::DWORD) -> super::DWORD);
    unsafe { SetFilePointer(hfile, ldistancetomove, lpdistancetomovehigh.unwrap_or(core::mem::zeroed()) as _, dwmovemethod) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetFilePointerEx(hfile: super::HANDLE, lidistancetomove: super::LARGE_INTEGER, lpnewfilepointer: Option<super::PLARGE_INTEGER>, dwmovemethod: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetFilePointerEx(hfile : super::HANDLE, lidistancetomove : super::LARGE_INTEGER, lpnewfilepointer : super::PLARGE_INTEGER, dwmovemethod : super::DWORD) -> windows_core::BOOL);
    unsafe { SetFilePointerEx(hfile, lidistancetomove, lpnewfilepointer.unwrap_or(core::mem::zeroed()) as _, dwmovemethod) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetFileTime(hfile: super::HANDLE, lpcreationtime: Option<*const super::FILETIME>, lplastaccesstime: Option<*const super::FILETIME>, lplastwritetime: Option<*const super::FILETIME>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetFileTime(hfile : super::HANDLE, lpcreationtime : *const super::FILETIME, lplastaccesstime : *const super::FILETIME, lplastwritetime : *const super::FILETIME) -> windows_core::BOOL);
    unsafe { SetFileTime(hfile, lpcreationtime.unwrap_or(core::mem::zeroed()) as _, lplastaccesstime.unwrap_or(core::mem::zeroed()) as _, lplastwritetime.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetFileValidData(hfile: super::HANDLE, validdatalength: super::LONGLONG) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetFileValidData(hfile : super::HANDLE, validdatalength : super::LONGLONG) -> windows_core::BOOL);
    unsafe { SetFileValidData(hfile, validdatalength) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn UnlockFile(hfile: super::HANDLE, dwfileoffsetlow: super::DWORD, dwfileoffsethigh: super::DWORD, nnumberofbytestounlocklow: super::DWORD, nnumberofbytestounlockhigh: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn UnlockFile(hfile : super::HANDLE, dwfileoffsetlow : super::DWORD, dwfileoffsethigh : super::DWORD, nnumberofbytestounlocklow : super::DWORD, nnumberofbytestounlockhigh : super::DWORD) -> windows_core::BOOL);
    unsafe { UnlockFile(hfile, dwfileoffsetlow, dwfileoffsethigh, nnumberofbytestounlocklow, nnumberofbytestounlockhigh) }
}
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn UnlockFileEx(hfile: super::HANDLE, dwreserved: super::DWORD, nnumberofbytestounlocklow: super::DWORD, nnumberofbytestounlockhigh: super::DWORD, lpoverlapped: super::LPOVERLAPPED) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn UnlockFileEx(hfile : super::HANDLE, dwreserved : super::DWORD, nnumberofbytestounlocklow : super::DWORD, nnumberofbytestounlockhigh : super::DWORD, lpoverlapped : super::LPOVERLAPPED) -> windows_core::BOOL);
    unsafe { UnlockFileEx(hfile, dwreserved, nnumberofbytestounlocklow, nnumberofbytestounlockhigh, lpoverlapped as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn WriteFile(hfile: super::HANDLE, lpbuffer: Option<super::LPCVOID>, nnumberofbytestowrite: super::DWORD, lpnumberofbyteswritten: Option<super::LPDWORD>, lpoverlapped: Option<super::LPOVERLAPPED>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn WriteFile(hfile : super::HANDLE, lpbuffer : super::LPCVOID, nnumberofbytestowrite : super::DWORD, lpnumberofbyteswritten : super::LPDWORD, lpoverlapped : super::LPOVERLAPPED) -> windows_core::BOOL);
    unsafe { WriteFile(hfile, lpbuffer.unwrap_or(core::mem::zeroed()) as _, nnumberofbytestowrite, lpnumberofbyteswritten.unwrap_or(core::mem::zeroed()) as _, lpoverlapped.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn WriteFileEx(hfile: super::HANDLE, lpbuffer: Option<super::LPCVOID>, nnumberofbytestowrite: super::DWORD, lpoverlapped: super::LPOVERLAPPED, lpcompletionroutine: super::LPOVERLAPPED_COMPLETION_ROUTINE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn WriteFileEx(hfile : super::HANDLE, lpbuffer : super::LPCVOID, nnumberofbytestowrite : super::DWORD, lpoverlapped : super::LPOVERLAPPED, lpcompletionroutine : super::LPOVERLAPPED_COMPLETION_ROUTINE) -> windows_core::BOOL);
    unsafe { WriteFileEx(hfile, lpbuffer.unwrap_or(core::mem::zeroed()) as _, nnumberofbytestowrite, lpoverlapped as _, lpcompletionroutine) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn WriteFileGather(hfile: super::HANDLE, asegmentarray: *const super::FILE_SEGMENT_ELEMENT, nnumberofbytestowrite: super::DWORD, lpreserved: super::LPDWORD, lpoverlapped: super::LPOVERLAPPED) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn WriteFileGather(hfile : super::HANDLE, asegmentarray : *const super::FILE_SEGMENT_ELEMENT, nnumberofbytestowrite : super::DWORD, lpreserved : super::LPDWORD, lpoverlapped : super::LPOVERLAPPED) -> windows_core::BOOL);
    unsafe { WriteFileGather(hfile, asegmentarray, nnumberofbytestowrite, lpreserved as _, lpoverlapped as _) }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
