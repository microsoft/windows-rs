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
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn CompareFileTime(lpfiletime1: *const super::FILETIME, lpfiletime2: *const super::FILETIME) -> i32 {
    windows_core::link!("kernel32.dll" "system" fn CompareFileTime(lpfiletime1 : *const super::FILETIME, lpfiletime2 : *const super::FILETIME) -> i32);
    unsafe { CompareFileTime(lpfiletime1, lpfiletime2) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn CreateDirectory2A<P0>(lppathname: P0, dwdesiredaccess: u32, dwsharemode: u32, directoryflags: DIRECTORY_FLAGS, lpsecurityattributes: Option<*const super::SECURITY_ATTRIBUTES>) -> super::HANDLE
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateDirectory2A(lppathname : windows_core::PCSTR, dwdesiredaccess : u32, dwsharemode : u32, directoryflags : DIRECTORY_FLAGS, lpsecurityattributes : *const super::SECURITY_ATTRIBUTES) -> super::HANDLE);
    unsafe { CreateDirectory2A(lppathname.param().abi(), dwdesiredaccess, dwsharemode, directoryflags, lpsecurityattributes.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn CreateDirectory2W<P0>(lppathname: P0, dwdesiredaccess: u32, dwsharemode: u32, directoryflags: DIRECTORY_FLAGS, lpsecurityattributes: Option<*const super::SECURITY_ATTRIBUTES>) -> super::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateDirectory2W(lppathname : windows_core::PCWSTR, dwdesiredaccess : u32, dwsharemode : u32, directoryflags : DIRECTORY_FLAGS, lpsecurityattributes : *const super::SECURITY_ATTRIBUTES) -> super::HANDLE);
    unsafe { CreateDirectory2W(lppathname.param().abi(), dwdesiredaccess, dwsharemode, directoryflags, lpsecurityattributes.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "minwinbase")]
#[inline]
pub unsafe fn CreateDirectoryA<P0>(lppathname: P0, lpsecurityattributes: Option<*const super::SECURITY_ATTRIBUTES>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateDirectoryA(lppathname : windows_core::PCSTR, lpsecurityattributes : *const super::SECURITY_ATTRIBUTES) -> windows_core::BOOL);
    unsafe { CreateDirectoryA(lppathname.param().abi(), lpsecurityattributes.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "minwinbase")]
#[inline]
pub unsafe fn CreateDirectoryW<P0>(lppathname: P0, lpsecurityattributes: Option<*const super::SECURITY_ATTRIBUTES>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateDirectoryW(lppathname : windows_core::PCWSTR, lpsecurityattributes : *const super::SECURITY_ATTRIBUTES) -> windows_core::BOOL);
    unsafe { CreateDirectoryW(lppathname.param().abi(), lpsecurityattributes.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn CreateFile2<P0>(lpfilename: P0, dwdesiredaccess: u32, dwsharemode: u32, dwcreationdisposition: u32, pcreateexparams: Option<*const CREATEFILE2_EXTENDED_PARAMETERS>) -> super::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateFile2(lpfilename : windows_core::PCWSTR, dwdesiredaccess : u32, dwsharemode : u32, dwcreationdisposition : u32, pcreateexparams : *const CREATEFILE2_EXTENDED_PARAMETERS) -> super::HANDLE);
    unsafe { CreateFile2(lpfilename.param().abi(), dwdesiredaccess, dwsharemode, dwcreationdisposition, pcreateexparams.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn CreateFile3<P0>(lpfilename: P0, dwdesiredaccess: u32, dwsharemode: u32, dwcreationdisposition: u32, pcreateexparams: Option<*const CREATEFILE3_EXTENDED_PARAMETERS>) -> super::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateFile3(lpfilename : windows_core::PCWSTR, dwdesiredaccess : u32, dwsharemode : u32, dwcreationdisposition : u32, pcreateexparams : *const CREATEFILE3_EXTENDED_PARAMETERS) -> super::HANDLE);
    unsafe { CreateFile3(lpfilename.param().abi(), dwdesiredaccess, dwsharemode, dwcreationdisposition, pcreateexparams.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn CreateFileA<P0>(lpfilename: P0, dwdesiredaccess: u32, dwsharemode: u32, lpsecurityattributes: Option<*const super::SECURITY_ATTRIBUTES>, dwcreationdisposition: u32, dwflagsandattributes: u32, htemplatefile: Option<super::HANDLE>) -> super::HANDLE
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateFileA(lpfilename : windows_core::PCSTR, dwdesiredaccess : u32, dwsharemode : u32, lpsecurityattributes : *const super::SECURITY_ATTRIBUTES, dwcreationdisposition : u32, dwflagsandattributes : u32, htemplatefile : super::HANDLE) -> super::HANDLE);
    unsafe { CreateFileA(lpfilename.param().abi(), dwdesiredaccess, dwsharemode, lpsecurityattributes.unwrap_or(core::mem::zeroed()) as _, dwcreationdisposition, dwflagsandattributes, htemplatefile.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn CreateFileW<P0>(lpfilename: P0, dwdesiredaccess: u32, dwsharemode: u32, lpsecurityattributes: Option<*const super::SECURITY_ATTRIBUTES>, dwcreationdisposition: u32, dwflagsandattributes: u32, htemplatefile: Option<super::HANDLE>) -> super::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateFileW(lpfilename : windows_core::PCWSTR, dwdesiredaccess : u32, dwsharemode : u32, lpsecurityattributes : *const super::SECURITY_ATTRIBUTES, dwcreationdisposition : u32, dwflagsandattributes : u32, htemplatefile : super::HANDLE) -> super::HANDLE);
    unsafe { CreateFileW(lpfilename.param().abi(), dwdesiredaccess, dwsharemode, lpsecurityattributes.unwrap_or(core::mem::zeroed()) as _, dwcreationdisposition, dwflagsandattributes, htemplatefile.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn DefineDosDeviceW<P1, P2>(dwflags: u32, lpdevicename: P1, lptargetpath: P2) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn DefineDosDeviceW(dwflags : u32, lpdevicename : windows_core::PCWSTR, lptargetpath : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { DefineDosDeviceW(dwflags, lpdevicename.param().abi(), lptargetpath.param().abi()) }
}
#[inline]
pub unsafe fn DeleteFile2A<P0>(lpfilename: P0, flags: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn DeleteFile2A(lpfilename : windows_core::PCSTR, flags : u32) -> windows_core::BOOL);
    unsafe { DeleteFile2A(lpfilename.param().abi(), flags) }
}
#[inline]
pub unsafe fn DeleteFile2W<P0>(lpfilename: P0, flags: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn DeleteFile2W(lpfilename : windows_core::PCWSTR, flags : u32) -> windows_core::BOOL);
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
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn FileTimeToLocalFileTime(lpfiletime: *const super::FILETIME, lplocalfiletime: *mut super::FILETIME) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn FileTimeToLocalFileTime(lpfiletime : *const super::FILETIME, lplocalfiletime : *mut super::FILETIME) -> windows_core::BOOL);
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
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn FindFirstChangeNotificationA<P0>(lppathname: P0, bwatchsubtree: bool, dwnotifyfilter: u32) -> super::HANDLE
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn FindFirstChangeNotificationA(lppathname : windows_core::PCSTR, bwatchsubtree : windows_core::BOOL, dwnotifyfilter : u32) -> super::HANDLE);
    unsafe { FindFirstChangeNotificationA(lppathname.param().abi(), bwatchsubtree.into(), dwnotifyfilter) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn FindFirstChangeNotificationW<P0>(lppathname: P0, bwatchsubtree: bool, dwnotifyfilter: u32) -> super::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn FindFirstChangeNotificationW(lppathname : windows_core::PCWSTR, bwatchsubtree : windows_core::BOOL, dwnotifyfilter : u32) -> super::HANDLE);
    unsafe { FindFirstChangeNotificationW(lppathname.param().abi(), bwatchsubtree.into(), dwnotifyfilter) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn FindFirstFileA<P0>(lpfilename: P0, lpfindfiledata: *mut super::WIN32_FIND_DATAA) -> super::HANDLE
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn FindFirstFileA(lpfilename : windows_core::PCSTR, lpfindfiledata : *mut super::WIN32_FIND_DATAA) -> super::HANDLE);
    unsafe { FindFirstFileA(lpfilename.param().abi(), lpfindfiledata as _) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn FindFirstFileExA<P0>(lpfilename: P0, finfolevelid: super::FINDEX_INFO_LEVELS, lpfindfiledata: *mut core::ffi::c_void, fsearchop: super::FINDEX_SEARCH_OPS, lpsearchfilter: Option<*const core::ffi::c_void>, dwadditionalflags: u32) -> super::HANDLE
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn FindFirstFileExA(lpfilename : windows_core::PCSTR, finfolevelid : super::FINDEX_INFO_LEVELS, lpfindfiledata : *mut core::ffi::c_void, fsearchop : super::FINDEX_SEARCH_OPS, lpsearchfilter : *const core::ffi::c_void, dwadditionalflags : u32) -> super::HANDLE);
    unsafe { FindFirstFileExA(lpfilename.param().abi(), finfolevelid, lpfindfiledata as _, fsearchop, lpsearchfilter.unwrap_or(core::mem::zeroed()) as _, dwadditionalflags) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn FindFirstFileExW<P0>(lpfilename: P0, finfolevelid: super::FINDEX_INFO_LEVELS, lpfindfiledata: *mut core::ffi::c_void, fsearchop: super::FINDEX_SEARCH_OPS, lpsearchfilter: Option<*const core::ffi::c_void>, dwadditionalflags: u32) -> super::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn FindFirstFileExW(lpfilename : windows_core::PCWSTR, finfolevelid : super::FINDEX_INFO_LEVELS, lpfindfiledata : *mut core::ffi::c_void, fsearchop : super::FINDEX_SEARCH_OPS, lpsearchfilter : *const core::ffi::c_void, dwadditionalflags : u32) -> super::HANDLE);
    unsafe { FindFirstFileExW(lpfilename.param().abi(), finfolevelid, lpfindfiledata as _, fsearchop, lpsearchfilter.unwrap_or(core::mem::zeroed()) as _, dwadditionalflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn FindFirstFileNameW<P0>(lpfilename: P0, dwflags: u32, stringlength: *mut u32, linkname: windows_core::PWSTR) -> super::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn FindFirstFileNameW(lpfilename : windows_core::PCWSTR, dwflags : u32, stringlength : *mut u32, linkname : windows_core::PWSTR) -> super::HANDLE);
    unsafe { FindFirstFileNameW(lpfilename.param().abi(), dwflags, stringlength as _, linkname) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn FindFirstFileW<P0>(lpfilename: P0, lpfindfiledata: *mut super::WIN32_FIND_DATAW) -> super::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn FindFirstFileW(lpfilename : windows_core::PCWSTR, lpfindfiledata : *mut super::WIN32_FIND_DATAW) -> super::HANDLE);
    unsafe { FindFirstFileW(lpfilename.param().abi(), lpfindfiledata as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn FindFirstStreamW<P0>(lpfilename: P0, infolevel: STREAM_INFO_LEVELS, lpfindstreamdata: *mut core::ffi::c_void, dwflags: Option<u32>) -> super::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn FindFirstStreamW(lpfilename : windows_core::PCWSTR, infolevel : STREAM_INFO_LEVELS, lpfindstreamdata : *mut core::ffi::c_void, dwflags : u32) -> super::HANDLE);
    unsafe { FindFirstStreamW(lpfilename.param().abi(), infolevel, lpfindstreamdata as _, dwflags.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn FindFirstVolumeW(lpszvolumename: &mut [u16]) -> super::HANDLE {
    windows_core::link!("kernel32.dll" "system" fn FindFirstVolumeW(lpszvolumename : windows_core::PWSTR, cchbufferlength : u32) -> super::HANDLE);
    unsafe { FindFirstVolumeW(core::mem::transmute(lpszvolumename.as_mut_ptr()), lpszvolumename.len().try_into().unwrap()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn FindNextChangeNotification(hchangehandle: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn FindNextChangeNotification(hchangehandle : super::HANDLE) -> windows_core::BOOL);
    unsafe { FindNextChangeNotification(hchangehandle) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn FindNextFileA(hfindfile: super::HANDLE, lpfindfiledata: *mut super::WIN32_FIND_DATAA) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn FindNextFileA(hfindfile : super::HANDLE, lpfindfiledata : *mut super::WIN32_FIND_DATAA) -> windows_core::BOOL);
    unsafe { FindNextFileA(hfindfile, lpfindfiledata as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn FindNextFileNameW(hfindstream: super::HANDLE, stringlength: *mut u32, linkname: windows_core::PWSTR) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn FindNextFileNameW(hfindstream : super::HANDLE, stringlength : *mut u32, linkname : windows_core::PWSTR) -> windows_core::BOOL);
    unsafe { FindNextFileNameW(hfindstream, stringlength as _, linkname) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn FindNextFileW(hfindfile: super::HANDLE, lpfindfiledata: *mut super::WIN32_FIND_DATAW) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn FindNextFileW(hfindfile : super::HANDLE, lpfindfiledata : *mut super::WIN32_FIND_DATAW) -> windows_core::BOOL);
    unsafe { FindNextFileW(hfindfile, lpfindfiledata as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn FindNextStreamW(hfindstream: super::HANDLE, lpfindstreamdata: *mut core::ffi::c_void) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn FindNextStreamW(hfindstream : super::HANDLE, lpfindstreamdata : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { FindNextStreamW(hfindstream, lpfindstreamdata as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn FindNextVolumeW(hfindvolume: super::HANDLE, lpszvolumename: &mut [u16]) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn FindNextVolumeW(hfindvolume : super::HANDLE, lpszvolumename : windows_core::PWSTR, cchbufferlength : u32) -> windows_core::BOOL);
    unsafe { FindNextVolumeW(hfindvolume as _, core::mem::transmute(lpszvolumename.as_mut_ptr()), lpszvolumename.len().try_into().unwrap()) }
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
#[inline]
pub unsafe fn GetCompressedFileSizeA<P0>(lpfilename: P0, lpfilesizehigh: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetCompressedFileSizeA(lpfilename : windows_core::PCSTR, lpfilesizehigh : *mut u32) -> u32);
    unsafe { GetCompressedFileSizeA(lpfilename.param().abi(), lpfilesizehigh.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GetCompressedFileSizeW<P0>(lpfilename: P0, lpfilesizehigh: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetCompressedFileSizeW(lpfilename : windows_core::PCWSTR, lpfilesizehigh : *mut u32) -> u32);
    unsafe { GetCompressedFileSizeW(lpfilename.param().abi(), lpfilesizehigh.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GetDiskFreeSpaceA<P0>(lprootpathname: P0, lpsectorspercluster: Option<*mut u32>, lpbytespersector: Option<*mut u32>, lpnumberoffreeclusters: Option<*mut u32>, lptotalnumberofclusters: Option<*mut u32>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetDiskFreeSpaceA(lprootpathname : windows_core::PCSTR, lpsectorspercluster : *mut u32, lpbytespersector : *mut u32, lpnumberoffreeclusters : *mut u32, lptotalnumberofclusters : *mut u32) -> windows_core::BOOL);
    unsafe { GetDiskFreeSpaceA(lprootpathname.param().abi(), lpsectorspercluster.unwrap_or(core::mem::zeroed()) as _, lpbytespersector.unwrap_or(core::mem::zeroed()) as _, lpnumberoffreeclusters.unwrap_or(core::mem::zeroed()) as _, lptotalnumberofclusters.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GetDiskFreeSpaceExA<P0>(lpdirectoryname: P0, lpfreebytesavailabletocaller: Option<*mut u64>, lptotalnumberofbytes: Option<*mut u64>, lptotalnumberoffreebytes: Option<*mut u64>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetDiskFreeSpaceExA(lpdirectoryname : windows_core::PCSTR, lpfreebytesavailabletocaller : *mut u64, lptotalnumberofbytes : *mut u64, lptotalnumberoffreebytes : *mut u64) -> windows_core::BOOL);
    unsafe { GetDiskFreeSpaceExA(lpdirectoryname.param().abi(), lpfreebytesavailabletocaller.unwrap_or(core::mem::zeroed()) as _, lptotalnumberofbytes.unwrap_or(core::mem::zeroed()) as _, lptotalnumberoffreebytes.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GetDiskFreeSpaceExW<P0>(lpdirectoryname: P0, lpfreebytesavailabletocaller: Option<*mut u64>, lptotalnumberofbytes: Option<*mut u64>, lptotalnumberoffreebytes: Option<*mut u64>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetDiskFreeSpaceExW(lpdirectoryname : windows_core::PCWSTR, lpfreebytesavailabletocaller : *mut u64, lptotalnumberofbytes : *mut u64, lptotalnumberoffreebytes : *mut u64) -> windows_core::BOOL);
    unsafe { GetDiskFreeSpaceExW(lpdirectoryname.param().abi(), lpfreebytesavailabletocaller.unwrap_or(core::mem::zeroed()) as _, lptotalnumberofbytes.unwrap_or(core::mem::zeroed()) as _, lptotalnumberoffreebytes.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GetDiskFreeSpaceW<P0>(lprootpathname: P0, lpsectorspercluster: Option<*mut u32>, lpbytespersector: Option<*mut u32>, lpnumberoffreeclusters: Option<*mut u32>, lptotalnumberofclusters: Option<*mut u32>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetDiskFreeSpaceW(lprootpathname : windows_core::PCWSTR, lpsectorspercluster : *mut u32, lpbytespersector : *mut u32, lpnumberoffreeclusters : *mut u32, lptotalnumberofclusters : *mut u32) -> windows_core::BOOL);
    unsafe { GetDiskFreeSpaceW(lprootpathname.param().abi(), lpsectorspercluster.unwrap_or(core::mem::zeroed()) as _, lpbytespersector.unwrap_or(core::mem::zeroed()) as _, lpnumberoffreeclusters.unwrap_or(core::mem::zeroed()) as _, lptotalnumberofclusters.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GetDiskSpaceInformationA<P0>(rootpath: P0, diskspaceinfo: *mut DISK_SPACE_INFORMATION) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetDiskSpaceInformationA(rootpath : windows_core::PCSTR, diskspaceinfo : *mut DISK_SPACE_INFORMATION) -> windows_core::HRESULT);
    unsafe { GetDiskSpaceInformationA(rootpath.param().abi(), diskspaceinfo as _) }
}
#[inline]
pub unsafe fn GetDiskSpaceInformationW<P0>(rootpath: P0, diskspaceinfo: *mut DISK_SPACE_INFORMATION) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetDiskSpaceInformationW(rootpath : windows_core::PCWSTR, diskspaceinfo : *mut DISK_SPACE_INFORMATION) -> windows_core::HRESULT);
    unsafe { GetDiskSpaceInformationW(rootpath.param().abi(), diskspaceinfo as _) }
}
#[inline]
pub unsafe fn GetDriveTypeA<P0>(lprootpathname: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetDriveTypeA(lprootpathname : windows_core::PCSTR) -> u32);
    unsafe { GetDriveTypeA(lprootpathname.param().abi()) }
}
#[inline]
pub unsafe fn GetDriveTypeW<P0>(lprootpathname: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetDriveTypeW(lprootpathname : windows_core::PCWSTR) -> u32);
    unsafe { GetDriveTypeW(lprootpathname.param().abi()) }
}
#[inline]
pub unsafe fn GetFileAttributesA<P0>(lpfilename: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetFileAttributesA(lpfilename : windows_core::PCSTR) -> u32);
    unsafe { GetFileAttributesA(lpfilename.param().abi()) }
}
#[cfg(feature = "minwinbase")]
#[inline]
pub unsafe fn GetFileAttributesExA<P0>(lpfilename: P0, finfolevelid: super::GET_FILEEX_INFO_LEVELS, lpfileinformation: *mut core::ffi::c_void) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetFileAttributesExA(lpfilename : windows_core::PCSTR, finfolevelid : super::GET_FILEEX_INFO_LEVELS, lpfileinformation : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { GetFileAttributesExA(lpfilename.param().abi(), finfolevelid, lpfileinformation as _) }
}
#[cfg(feature = "minwinbase")]
#[inline]
pub unsafe fn GetFileAttributesExW<P0>(lpfilename: P0, finfolevelid: super::GET_FILEEX_INFO_LEVELS, lpfileinformation: *mut core::ffi::c_void) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetFileAttributesExW(lpfilename : windows_core::PCWSTR, finfolevelid : super::GET_FILEEX_INFO_LEVELS, lpfileinformation : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { GetFileAttributesExW(lpfilename.param().abi(), finfolevelid, lpfileinformation as _) }
}
#[inline]
pub unsafe fn GetFileAttributesW<P0>(lpfilename: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetFileAttributesW(lpfilename : windows_core::PCWSTR) -> u32);
    unsafe { GetFileAttributesW(lpfilename.param().abi()) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn GetFileInformationByHandle(hfile: super::HANDLE, lpfileinformation: *mut BY_HANDLE_FILE_INFORMATION) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetFileInformationByHandle(hfile : super::HANDLE, lpfileinformation : *mut BY_HANDLE_FILE_INFORMATION) -> windows_core::BOOL);
    unsafe { GetFileInformationByHandle(hfile, lpfileinformation as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetFileSize(hfile: super::HANDLE, lpfilesizehigh: Option<*mut u32>) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetFileSize(hfile : super::HANDLE, lpfilesizehigh : *mut u32) -> u32);
    unsafe { GetFileSize(hfile, lpfilesizehigh.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetFileSizeEx(hfile: super::HANDLE, lpfilesize: *mut i64) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetFileSizeEx(hfile : super::HANDLE, lpfilesize : *mut i64) -> windows_core::BOOL);
    unsafe { GetFileSizeEx(hfile, lpfilesize as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn GetFileTime(hfile: super::HANDLE, lpcreationtime: Option<*mut super::FILETIME>, lplastaccesstime: Option<*mut super::FILETIME>, lplastwritetime: Option<*mut super::FILETIME>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetFileTime(hfile : super::HANDLE, lpcreationtime : *mut super::FILETIME, lplastaccesstime : *mut super::FILETIME, lplastwritetime : *mut super::FILETIME) -> windows_core::BOOL);
    unsafe { GetFileTime(hfile, lpcreationtime.unwrap_or(core::mem::zeroed()) as _, lplastaccesstime.unwrap_or(core::mem::zeroed()) as _, lplastwritetime.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetFileType(hfile: super::HANDLE) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetFileType(hfile : super::HANDLE) -> u32);
    unsafe { GetFileType(hfile) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetFinalPathNameByHandleA(hfile: super::HANDLE, lpszfilepath: &mut [u8], dwflags: u32) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetFinalPathNameByHandleA(hfile : super::HANDLE, lpszfilepath : windows_core::PSTR, cchfilepath : u32, dwflags : u32) -> u32);
    unsafe { GetFinalPathNameByHandleA(hfile, core::mem::transmute(lpszfilepath.as_mut_ptr()), lpszfilepath.len().try_into().unwrap(), dwflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetFinalPathNameByHandleW(hfile: super::HANDLE, lpszfilepath: &mut [u16], dwflags: u32) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetFinalPathNameByHandleW(hfile : super::HANDLE, lpszfilepath : windows_core::PWSTR, cchfilepath : u32, dwflags : u32) -> u32);
    unsafe { GetFinalPathNameByHandleW(hfile, core::mem::transmute(lpszfilepath.as_mut_ptr()), lpszfilepath.len().try_into().unwrap(), dwflags) }
}
#[inline]
pub unsafe fn GetFullPathNameA<P0>(lpfilename: P0, lpbuffer: Option<&mut [u8]>, lpfilepart: *mut windows_core::PSTR) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetFullPathNameA(lpfilename : windows_core::PCSTR, nbufferlength : u32, lpbuffer : windows_core::PSTR, lpfilepart : *mut windows_core::PSTR) -> u32);
    unsafe { GetFullPathNameA(lpfilename.param().abi(), lpbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(lpbuffer.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut())), lpfilepart as _) }
}
#[inline]
pub unsafe fn GetFullPathNameW<P0>(lpfilename: P0, lpbuffer: Option<&mut [u16]>, lpfilepart: *mut windows_core::PWSTR) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetFullPathNameW(lpfilename : windows_core::PCWSTR, nbufferlength : u32, lpbuffer : windows_core::PWSTR, lpfilepart : *mut windows_core::PWSTR) -> u32);
    unsafe { GetFullPathNameW(lpfilename.param().abi(), lpbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(lpbuffer.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut())), lpfilepart as _) }
}
#[inline]
pub unsafe fn GetLogicalDriveStringsW(lpbuffer: Option<&mut [u16]>) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetLogicalDriveStringsW(nbufferlength : u32, lpbuffer : windows_core::PWSTR) -> u32);
    unsafe { GetLogicalDriveStringsW(lpbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(lpbuffer.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()))) }
}
#[inline]
pub unsafe fn GetLogicalDrives() -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetLogicalDrives() -> u32);
    unsafe { GetLogicalDrives() }
}
#[inline]
pub unsafe fn GetLongPathNameA<P0>(lpszshortpath: P0, lpszlongpath: Option<&mut [u8]>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetLongPathNameA(lpszshortpath : windows_core::PCSTR, lpszlongpath : windows_core::PSTR, cchbuffer : u32) -> u32);
    unsafe { GetLongPathNameA(lpszshortpath.param().abi(), core::mem::transmute(lpszlongpath.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut())), lpszlongpath.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn GetLongPathNameW<P0>(lpszshortpath: P0, lpszlongpath: Option<&mut [u16]>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetLongPathNameW(lpszshortpath : windows_core::PCWSTR, lpszlongpath : windows_core::PWSTR, cchbuffer : u32) -> u32);
    unsafe { GetLongPathNameW(lpszshortpath.param().abi(), core::mem::transmute(lpszlongpath.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut())), lpszlongpath.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn GetShortPathNameW<P0>(lpszlongpath: P0, lpszshortpath: Option<&mut [u16]>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetShortPathNameW(lpszlongpath : windows_core::PCWSTR, lpszshortpath : windows_core::PWSTR, cchbuffer : u32) -> u32);
    unsafe { GetShortPathNameW(lpszlongpath.param().abi(), core::mem::transmute(lpszshortpath.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut())), lpszshortpath.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn GetTempFileNameA<P0, P1>(lppathname: P0, lpprefixstring: P1, uunique: u32, lptempfilename: windows_core::PSTR) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetTempFileNameA(lppathname : windows_core::PCSTR, lpprefixstring : windows_core::PCSTR, uunique : u32, lptempfilename : windows_core::PSTR) -> u32);
    unsafe { GetTempFileNameA(lppathname.param().abi(), lpprefixstring.param().abi(), uunique, lptempfilename) }
}
#[inline]
pub unsafe fn GetTempFileNameW<P0, P1>(lppathname: P0, lpprefixstring: P1, uunique: u32, lptempfilename: windows_core::PWSTR) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetTempFileNameW(lppathname : windows_core::PCWSTR, lpprefixstring : windows_core::PCWSTR, uunique : u32, lptempfilename : windows_core::PWSTR) -> u32);
    unsafe { GetTempFileNameW(lppathname.param().abi(), lpprefixstring.param().abi(), uunique, lptempfilename) }
}
#[inline]
pub unsafe fn GetTempPath2A(buffer: Option<&mut [u8]>) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetTempPath2A(bufferlength : u32, buffer : windows_core::PSTR) -> u32);
    unsafe { GetTempPath2A(buffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(buffer.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()))) }
}
#[inline]
pub unsafe fn GetTempPath2W(buffer: Option<&mut [u16]>) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetTempPath2W(bufferlength : u32, buffer : windows_core::PWSTR) -> u32);
    unsafe { GetTempPath2W(buffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(buffer.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()))) }
}
#[inline]
pub unsafe fn GetTempPathA(lpbuffer: Option<&mut [u8]>) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetTempPathA(nbufferlength : u32, lpbuffer : windows_core::PSTR) -> u32);
    unsafe { GetTempPathA(lpbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(lpbuffer.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()))) }
}
#[inline]
pub unsafe fn GetTempPathW(lpbuffer: Option<&mut [u16]>) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetTempPathW(nbufferlength : u32, lpbuffer : windows_core::PWSTR) -> u32);
    unsafe { GetTempPathW(lpbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(lpbuffer.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()))) }
}
#[inline]
pub unsafe fn GetVolumeInformationA<P0>(lprootpathname: P0, lpvolumenamebuffer: Option<&mut [u8]>, lpvolumeserialnumber: Option<*mut u32>, lpmaximumcomponentlength: Option<*mut u32>, lpfilesystemflags: Option<*mut u32>, lpfilesystemnamebuffer: Option<&mut [u8]>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetVolumeInformationA(lprootpathname : windows_core::PCSTR, lpvolumenamebuffer : windows_core::PSTR, nvolumenamesize : u32, lpvolumeserialnumber : *mut u32, lpmaximumcomponentlength : *mut u32, lpfilesystemflags : *mut u32, lpfilesystemnamebuffer : windows_core::PSTR, nfilesystemnamesize : u32) -> windows_core::BOOL);
    unsafe {
        GetVolumeInformationA(
            lprootpathname.param().abi(),
            core::mem::transmute(lpvolumenamebuffer.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut())),
            lpvolumenamebuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
            lpvolumeserialnumber.unwrap_or(core::mem::zeroed()) as _,
            lpmaximumcomponentlength.unwrap_or(core::mem::zeroed()) as _,
            lpfilesystemflags.unwrap_or(core::mem::zeroed()) as _,
            core::mem::transmute(lpfilesystemnamebuffer.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut())),
            lpfilesystemnamebuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        )
    }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetVolumeInformationByHandleW(hfile: super::HANDLE, lpvolumenamebuffer: Option<&mut [u16]>, lpvolumeserialnumber: Option<*mut u32>, lpmaximumcomponentlength: Option<*mut u32>, lpfilesystemflags: Option<*mut u32>, lpfilesystemnamebuffer: Option<&mut [u16]>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetVolumeInformationByHandleW(hfile : super::HANDLE, lpvolumenamebuffer : windows_core::PWSTR, nvolumenamesize : u32, lpvolumeserialnumber : *mut u32, lpmaximumcomponentlength : *mut u32, lpfilesystemflags : *mut u32, lpfilesystemnamebuffer : windows_core::PWSTR, nfilesystemnamesize : u32) -> windows_core::BOOL);
    unsafe {
        GetVolumeInformationByHandleW(
            hfile,
            core::mem::transmute(lpvolumenamebuffer.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut())),
            lpvolumenamebuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
            lpvolumeserialnumber.unwrap_or(core::mem::zeroed()) as _,
            lpmaximumcomponentlength.unwrap_or(core::mem::zeroed()) as _,
            lpfilesystemflags.unwrap_or(core::mem::zeroed()) as _,
            core::mem::transmute(lpfilesystemnamebuffer.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut())),
            lpfilesystemnamebuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        )
    }
}
#[inline]
pub unsafe fn GetVolumeInformationW<P0>(lprootpathname: P0, lpvolumenamebuffer: Option<&mut [u16]>, lpvolumeserialnumber: Option<*mut u32>, lpmaximumcomponentlength: Option<*mut u32>, lpfilesystemflags: Option<*mut u32>, lpfilesystemnamebuffer: Option<&mut [u16]>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetVolumeInformationW(lprootpathname : windows_core::PCWSTR, lpvolumenamebuffer : windows_core::PWSTR, nvolumenamesize : u32, lpvolumeserialnumber : *mut u32, lpmaximumcomponentlength : *mut u32, lpfilesystemflags : *mut u32, lpfilesystemnamebuffer : windows_core::PWSTR, nfilesystemnamesize : u32) -> windows_core::BOOL);
    unsafe {
        GetVolumeInformationW(
            lprootpathname.param().abi(),
            core::mem::transmute(lpvolumenamebuffer.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut())),
            lpvolumenamebuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
            lpvolumeserialnumber.unwrap_or(core::mem::zeroed()) as _,
            lpmaximumcomponentlength.unwrap_or(core::mem::zeroed()) as _,
            lpfilesystemflags.unwrap_or(core::mem::zeroed()) as _,
            core::mem::transmute(lpfilesystemnamebuffer.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut())),
            lpfilesystemnamebuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        )
    }
}
#[inline]
pub unsafe fn GetVolumeNameForVolumeMountPointW<P0>(lpszvolumemountpoint: P0, lpszvolumename: &mut [u16]) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetVolumeNameForVolumeMountPointW(lpszvolumemountpoint : windows_core::PCWSTR, lpszvolumename : windows_core::PWSTR, cchbufferlength : u32) -> windows_core::BOOL);
    unsafe { GetVolumeNameForVolumeMountPointW(lpszvolumemountpoint.param().abi(), core::mem::transmute(lpszvolumename.as_mut_ptr()), lpszvolumename.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn GetVolumePathNameW<P0>(lpszfilename: P0, lpszvolumepathname: &mut [u16]) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetVolumePathNameW(lpszfilename : windows_core::PCWSTR, lpszvolumepathname : windows_core::PWSTR, cchbufferlength : u32) -> windows_core::BOOL);
    unsafe { GetVolumePathNameW(lpszfilename.param().abi(), core::mem::transmute(lpszvolumepathname.as_mut_ptr()), lpszvolumepathname.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn GetVolumePathNamesForVolumeNameW<P0>(lpszvolumename: P0, lpszvolumepathnames: Option<&mut [u16]>, lpcchreturnlength: *mut u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetVolumePathNamesForVolumeNameW(lpszvolumename : windows_core::PCWSTR, lpszvolumepathnames : *mut u16, cchbufferlength : u32, lpcchreturnlength : *mut u32) -> windows_core::BOOL);
    unsafe { GetVolumePathNamesForVolumeNameW(lpszvolumename.param().abi(), lpszvolumepathnames.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()), lpszvolumepathnames.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), lpcchreturnlength as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn LocalFileTimeToFileTime(lplocalfiletime: *const super::FILETIME, lpfiletime: *mut super::FILETIME) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn LocalFileTimeToFileTime(lplocalfiletime : *const super::FILETIME, lpfiletime : *mut super::FILETIME) -> windows_core::BOOL);
    unsafe { LocalFileTimeToFileTime(lplocalfiletime, lpfiletime as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn LockFile(hfile: super::HANDLE, dwfileoffsetlow: u32, dwfileoffsethigh: u32, nnumberofbytestolocklow: u32, nnumberofbytestolockhigh: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn LockFile(hfile : super::HANDLE, dwfileoffsetlow : u32, dwfileoffsethigh : u32, nnumberofbytestolocklow : u32, nnumberofbytestolockhigh : u32) -> windows_core::BOOL);
    unsafe { LockFile(hfile, dwfileoffsetlow, dwfileoffsethigh, nnumberofbytestolocklow, nnumberofbytestolockhigh) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn LockFileEx(hfile: super::HANDLE, dwflags: u32, dwreserved: Option<u32>, nnumberofbytestolocklow: u32, nnumberofbytestolockhigh: u32, lpoverlapped: *mut super::OVERLAPPED) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn LockFileEx(hfile : super::HANDLE, dwflags : u32, dwreserved : u32, nnumberofbytestolocklow : u32, nnumberofbytestolockhigh : u32, lpoverlapped : *mut super::OVERLAPPED) -> windows_core::BOOL);
    unsafe { LockFileEx(hfile, dwflags, dwreserved.unwrap_or(core::mem::zeroed()) as _, nnumberofbytestolocklow, nnumberofbytestolockhigh, lpoverlapped as _) }
}
#[inline]
pub unsafe fn QueryDosDeviceW<P0>(lpdevicename: P0, lptargetpath: Option<&mut [u16]>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn QueryDosDeviceW(lpdevicename : windows_core::PCWSTR, lptargetpath : windows_core::PWSTR, ucchmax : u32) -> u32);
    unsafe { QueryDosDeviceW(lpdevicename.param().abi(), core::mem::transmute(lptargetpath.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut())), lptargetpath.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn ReadFile(hfile: super::HANDLE, lpbuffer: Option<*mut core::ffi::c_void>, nnumberofbytestoread: u32, lpnumberofbytesread: Option<*mut u32>, lpoverlapped: Option<*mut super::OVERLAPPED>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn ReadFile(hfile : super::HANDLE, lpbuffer : *mut core::ffi::c_void, nnumberofbytestoread : u32, lpnumberofbytesread : *mut u32, lpoverlapped : *mut super::OVERLAPPED) -> windows_core::BOOL);
    unsafe { ReadFile(hfile, lpbuffer.unwrap_or(core::mem::zeroed()) as _, nnumberofbytestoread, lpnumberofbytesread.unwrap_or(core::mem::zeroed()) as _, lpoverlapped.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn ReadFileEx(hfile: super::HANDLE, lpbuffer: Option<*mut core::ffi::c_void>, nnumberofbytestoread: u32, lpoverlapped: *mut super::OVERLAPPED, lpcompletionroutine: super::LPOVERLAPPED_COMPLETION_ROUTINE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn ReadFileEx(hfile : super::HANDLE, lpbuffer : *mut core::ffi::c_void, nnumberofbytestoread : u32, lpoverlapped : *mut super::OVERLAPPED, lpcompletionroutine : super::LPOVERLAPPED_COMPLETION_ROUTINE) -> windows_core::BOOL);
    unsafe { ReadFileEx(hfile, lpbuffer.unwrap_or(core::mem::zeroed()) as _, nnumberofbytestoread, lpoverlapped as _, lpcompletionroutine) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn ReadFileScatter(hfile: super::HANDLE, asegmentarray: *const super::FILE_SEGMENT_ELEMENT, nnumberofbytestoread: u32, lpreserved: Option<*const u32>, lpoverlapped: *mut super::OVERLAPPED) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn ReadFileScatter(hfile : super::HANDLE, asegmentarray : *const super::FILE_SEGMENT_ELEMENT, nnumberofbytestoread : u32, lpreserved : *const u32, lpoverlapped : *mut super::OVERLAPPED) -> windows_core::BOOL);
    unsafe { ReadFileScatter(hfile, asegmentarray, nnumberofbytestoread, lpreserved.unwrap_or(core::mem::zeroed()) as _, lpoverlapped as _) }
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
#[inline]
pub unsafe fn SetFileAttributesA<P0>(lpfilename: P0, dwfileattributes: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn SetFileAttributesA(lpfilename : windows_core::PCSTR, dwfileattributes : u32) -> windows_core::BOOL);
    unsafe { SetFileAttributesA(lpfilename.param().abi(), dwfileattributes) }
}
#[inline]
pub unsafe fn SetFileAttributesW<P0>(lpfilename: P0, dwfileattributes: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn SetFileAttributesW(lpfilename : windows_core::PCWSTR, dwfileattributes : u32) -> windows_core::BOOL);
    unsafe { SetFileAttributesW(lpfilename.param().abi(), dwfileattributes) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn SetFileInformationByHandle(hfile: super::HANDLE, fileinformationclass: super::FILE_INFO_BY_HANDLE_CLASS, lpfileinformation: *const core::ffi::c_void, dwbuffersize: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetFileInformationByHandle(hfile : super::HANDLE, fileinformationclass : super::FILE_INFO_BY_HANDLE_CLASS, lpfileinformation : *const core::ffi::c_void, dwbuffersize : u32) -> windows_core::BOOL);
    unsafe { SetFileInformationByHandle(hfile, fileinformationclass, lpfileinformation, dwbuffersize) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SetFileIoOverlappedRange(filehandle: super::HANDLE, overlappedrangestart: *const u8, length: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetFileIoOverlappedRange(filehandle : super::HANDLE, overlappedrangestart : *const u8, length : u32) -> windows_core::BOOL);
    unsafe { SetFileIoOverlappedRange(filehandle, overlappedrangestart, length) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SetFilePointer(hfile: super::HANDLE, ldistancetomove: i32, lpdistancetomovehigh: Option<*mut i32>, dwmovemethod: u32) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn SetFilePointer(hfile : super::HANDLE, ldistancetomove : i32, lpdistancetomovehigh : *mut i32, dwmovemethod : u32) -> u32);
    unsafe { SetFilePointer(hfile, ldistancetomove, lpdistancetomovehigh.unwrap_or(core::mem::zeroed()) as _, dwmovemethod) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SetFilePointerEx(hfile: super::HANDLE, lidistancetomove: i64, lpnewfilepointer: Option<*mut i64>, dwmovemethod: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetFilePointerEx(hfile : super::HANDLE, lidistancetomove : i64, lpnewfilepointer : *mut i64, dwmovemethod : u32) -> windows_core::BOOL);
    unsafe { SetFilePointerEx(hfile, lidistancetomove, lpnewfilepointer.unwrap_or(core::mem::zeroed()) as _, dwmovemethod) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn SetFileTime(hfile: super::HANDLE, lpcreationtime: Option<*const super::FILETIME>, lplastaccesstime: Option<*const super::FILETIME>, lplastwritetime: Option<*const super::FILETIME>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetFileTime(hfile : super::HANDLE, lpcreationtime : *const super::FILETIME, lplastaccesstime : *const super::FILETIME, lplastwritetime : *const super::FILETIME) -> windows_core::BOOL);
    unsafe { SetFileTime(hfile, lpcreationtime.unwrap_or(core::mem::zeroed()) as _, lplastaccesstime.unwrap_or(core::mem::zeroed()) as _, lplastwritetime.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SetFileValidData(hfile: super::HANDLE, validdatalength: i64) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetFileValidData(hfile : super::HANDLE, validdatalength : i64) -> windows_core::BOOL);
    unsafe { SetFileValidData(hfile, validdatalength) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn UnlockFile(hfile: super::HANDLE, dwfileoffsetlow: u32, dwfileoffsethigh: u32, nnumberofbytestounlocklow: u32, nnumberofbytestounlockhigh: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn UnlockFile(hfile : super::HANDLE, dwfileoffsetlow : u32, dwfileoffsethigh : u32, nnumberofbytestounlocklow : u32, nnumberofbytestounlockhigh : u32) -> windows_core::BOOL);
    unsafe { UnlockFile(hfile, dwfileoffsetlow, dwfileoffsethigh, nnumberofbytestounlocklow, nnumberofbytestounlockhigh) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn UnlockFileEx(hfile: super::HANDLE, dwreserved: Option<u32>, nnumberofbytestounlocklow: u32, nnumberofbytestounlockhigh: u32, lpoverlapped: *mut super::OVERLAPPED) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn UnlockFileEx(hfile : super::HANDLE, dwreserved : u32, nnumberofbytestounlocklow : u32, nnumberofbytestounlockhigh : u32, lpoverlapped : *mut super::OVERLAPPED) -> windows_core::BOOL);
    unsafe { UnlockFileEx(hfile, dwreserved.unwrap_or(core::mem::zeroed()) as _, nnumberofbytestounlocklow, nnumberofbytestounlockhigh, lpoverlapped as _) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn WriteFile(hfile: super::HANDLE, lpbuffer: Option<*const core::ffi::c_void>, nnumberofbytestowrite: u32, lpnumberofbyteswritten: Option<*mut u32>, lpoverlapped: Option<*mut super::OVERLAPPED>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn WriteFile(hfile : super::HANDLE, lpbuffer : *const core::ffi::c_void, nnumberofbytestowrite : u32, lpnumberofbyteswritten : *mut u32, lpoverlapped : *mut super::OVERLAPPED) -> windows_core::BOOL);
    unsafe { WriteFile(hfile, lpbuffer.unwrap_or(core::mem::zeroed()) as _, nnumberofbytestowrite, lpnumberofbyteswritten.unwrap_or(core::mem::zeroed()) as _, lpoverlapped.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn WriteFileEx(hfile: super::HANDLE, lpbuffer: Option<*const core::ffi::c_void>, nnumberofbytestowrite: u32, lpoverlapped: *mut super::OVERLAPPED, lpcompletionroutine: super::LPOVERLAPPED_COMPLETION_ROUTINE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn WriteFileEx(hfile : super::HANDLE, lpbuffer : *const core::ffi::c_void, nnumberofbytestowrite : u32, lpoverlapped : *mut super::OVERLAPPED, lpcompletionroutine : super::LPOVERLAPPED_COMPLETION_ROUTINE) -> windows_core::BOOL);
    unsafe { WriteFileEx(hfile, lpbuffer.unwrap_or(core::mem::zeroed()) as _, nnumberofbytestowrite, lpoverlapped as _, lpcompletionroutine) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn WriteFileGather(hfile: super::HANDLE, asegmentarray: *const super::FILE_SEGMENT_ELEMENT, nnumberofbytestowrite: u32, lpreserved: Option<*const u32>, lpoverlapped: *mut super::OVERLAPPED) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn WriteFileGather(hfile : super::HANDLE, asegmentarray : *const super::FILE_SEGMENT_ELEMENT, nnumberofbytestowrite : u32, lpreserved : *const u32, lpoverlapped : *mut super::OVERLAPPED) -> windows_core::BOOL);
    unsafe { WriteFileGather(hfile, asegmentarray, nnumberofbytestowrite, lpreserved.unwrap_or(core::mem::zeroed()) as _, lpoverlapped as _) }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BY_HANDLE_FILE_INFORMATION {
    pub dwFileAttributes: u32,
    pub ftCreationTime: super::FILETIME,
    pub ftLastAccessTime: super::FILETIME,
    pub ftLastWriteTime: super::FILETIME,
    pub dwVolumeSerialNumber: u32,
    pub nFileSizeHigh: u32,
    pub nFileSizeLow: u32,
    pub nNumberOfLinks: u32,
    pub nFileIndexHigh: u32,
    pub nFileIndexLow: u32,
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CREATEFILE2_EXTENDED_PARAMETERS {
    pub dwSize: u32,
    pub dwFileAttributes: u32,
    pub dwFileFlags: u32,
    pub dwSecurityQosFlags: u32,
    pub lpSecurityAttributes: super::LPSECURITY_ATTRIBUTES,
    pub hTemplateFile: super::HANDLE,
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CREATEFILE3_EXTENDED_PARAMETERS {
    pub dwSize: u32,
    pub dwFileAttributes: u32,
    pub dwFileFlags: u32,
    pub dwSecurityQosFlags: u32,
    pub lpSecurityAttributes: super::LPSECURITY_ATTRIBUTES,
    pub hTemplateFile: super::HANDLE,
}
pub const CREATE_ALWAYS: u32 = 2;
pub const CREATE_NEW: u32 = 1;
pub type DIRECTORY_FLAGS = u32;
pub const DIRECTORY_FLAGS_DISALLOW_PATH_REDIRECTS: DIRECTORY_FLAGS = 1;
pub const DIRECTORY_FLAGS_NONE: DIRECTORY_FLAGS = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WIN32_FILE_ATTRIBUTE_DATA {
    pub dwFileAttributes: u32,
    pub ftCreationTime: super::FILETIME,
    pub ftLastAccessTime: super::FILETIME,
    pub ftLastWriteTime: super::FILETIME,
    pub nFileSizeHigh: u32,
    pub nFileSizeLow: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WIN32_FIND_STREAM_DATA {
    pub StreamSize: i64,
    pub cStreamName: [u16; 296],
}
impl Default for WIN32_FIND_STREAM_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
