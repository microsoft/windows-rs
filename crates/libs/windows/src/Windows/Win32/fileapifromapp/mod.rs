#[inline]
pub unsafe fn CopyFileFromAppW<P0, P1>(lpexistingfilename: P0, lpnewfilename: P1, bfailifexists: bool) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-core-file-fromapp-l1-1-0.dll" "system" fn CopyFileFromAppW(lpexistingfilename : windows_core::PCWSTR, lpnewfilename : windows_core::PCWSTR, bfailifexists : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { CopyFileFromAppW(lpexistingfilename.param().abi(), lpnewfilename.param().abi(), bfailifexists.into()) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CreateDirectoryFromAppW<P0>(lppathname: P0, lpsecurityattributes: Option<super::LPSECURITY_ATTRIBUTES>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-core-file-fromapp-l1-1-0.dll" "system" fn CreateDirectoryFromAppW(lppathname : windows_core::PCWSTR, lpsecurityattributes : super::LPSECURITY_ATTRIBUTES) -> windows_core::BOOL);
    unsafe { CreateDirectoryFromAppW(lppathname.param().abi(), lpsecurityattributes.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "fileapi", feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateFile2FromAppW<P0>(lpfilename: P0, dwdesiredaccess: super::DWORD, dwsharemode: super::DWORD, dwcreationdisposition: super::DWORD, pcreateexparams: Option<super::LPCREATEFILE2_EXTENDED_PARAMETERS>) -> super::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-core-file-fromapp-l1-1-0.dll" "system" fn CreateFile2FromAppW(lpfilename : windows_core::PCWSTR, dwdesiredaccess : super::DWORD, dwsharemode : super::DWORD, dwcreationdisposition : super::DWORD, pcreateexparams : super::LPCREATEFILE2_EXTENDED_PARAMETERS) -> super::HANDLE);
    unsafe { CreateFile2FromAppW(lpfilename.param().abi(), dwdesiredaccess, dwsharemode, dwcreationdisposition, pcreateexparams.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateFileFromAppW<P0>(lpfilename: P0, dwdesiredaccess: super::DWORD, dwsharemode: super::DWORD, lpsecurityattributes: Option<super::LPSECURITY_ATTRIBUTES>, dwcreationdisposition: super::DWORD, dwflagsandattributes: super::DWORD, htemplatefile: Option<super::HANDLE>) -> super::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-core-file-fromapp-l1-1-0.dll" "system" fn CreateFileFromAppW(lpfilename : windows_core::PCWSTR, dwdesiredaccess : super::DWORD, dwsharemode : super::DWORD, lpsecurityattributes : super::LPSECURITY_ATTRIBUTES, dwcreationdisposition : super::DWORD, dwflagsandattributes : super::DWORD, htemplatefile : super::HANDLE) -> super::HANDLE);
    unsafe { CreateFileFromAppW(lpfilename.param().abi(), dwdesiredaccess, dwsharemode, lpsecurityattributes.unwrap_or(core::mem::zeroed()) as _, dwcreationdisposition, dwflagsandattributes, htemplatefile.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn DeleteFileFromAppW<P0>(lpfilename: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-core-file-fromapp-l1-1-0.dll" "system" fn DeleteFileFromAppW(lpfilename : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { DeleteFileFromAppW(lpfilename.param().abi()) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FindFirstFileExFromAppW<P0>(lpfilename: P0, finfolevelid: super::FINDEX_INFO_LEVELS, lpfindfiledata: super::LPVOID, fsearchop: super::FINDEX_SEARCH_OPS, lpsearchfilter: Option<super::LPVOID>, dwadditionalflags: super::DWORD) -> super::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-core-file-fromapp-l1-1-0.dll" "system" fn FindFirstFileExFromAppW(lpfilename : windows_core::PCWSTR, finfolevelid : super::FINDEX_INFO_LEVELS, lpfindfiledata : super::LPVOID, fsearchop : super::FINDEX_SEARCH_OPS, lpsearchfilter : super::LPVOID, dwadditionalflags : super::DWORD) -> super::HANDLE);
    unsafe { FindFirstFileExFromAppW(lpfilename.param().abi(), finfolevelid, lpfindfiledata as _, fsearchop, lpsearchfilter.unwrap_or(core::mem::zeroed()) as _, dwadditionalflags) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef"))]
#[inline]
pub unsafe fn GetFileAttributesExFromAppW<P0>(lpfilename: P0, finfolevelid: super::GET_FILEEX_INFO_LEVELS, lpfileinformation: super::LPVOID) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-core-file-fromapp-l1-1-0.dll" "system" fn GetFileAttributesExFromAppW(lpfilename : windows_core::PCWSTR, finfolevelid : super::GET_FILEEX_INFO_LEVELS, lpfileinformation : super::LPVOID) -> windows_core::BOOL);
    unsafe { GetFileAttributesExFromAppW(lpfilename.param().abi(), finfolevelid, lpfileinformation as _) }
}
#[inline]
pub unsafe fn MoveFileFromAppW<P0, P1>(lpexistingfilename: P0, lpnewfilename: P1) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-core-file-fromapp-l1-1-0.dll" "system" fn MoveFileFromAppW(lpexistingfilename : windows_core::PCWSTR, lpnewfilename : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { MoveFileFromAppW(lpexistingfilename.param().abi(), lpnewfilename.param().abi()) }
}
#[inline]
pub unsafe fn RemoveDirectoryFromAppW<P0>(lppathname: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-core-file-fromapp-l1-1-0.dll" "system" fn RemoveDirectoryFromAppW(lppathname : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { RemoveDirectoryFromAppW(lppathname.param().abi()) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn ReplaceFileFromAppW<P0, P1, P2>(lpreplacedfilename: P0, lpreplacementfilename: P1, lpbackupfilename: P2, dwreplaceflags: super::DWORD, lpexclude: Option<super::LPVOID>, lpreserved: Option<super::LPVOID>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-core-file-fromapp-l1-1-0.dll" "system" fn ReplaceFileFromAppW(lpreplacedfilename : windows_core::PCWSTR, lpreplacementfilename : windows_core::PCWSTR, lpbackupfilename : windows_core::PCWSTR, dwreplaceflags : super::DWORD, lpexclude : super::LPVOID, lpreserved : super::LPVOID) -> windows_core::BOOL);
    unsafe { ReplaceFileFromAppW(lpreplacedfilename.param().abi(), lpreplacementfilename.param().abi(), lpbackupfilename.param().abi(), dwreplaceflags, lpexclude.unwrap_or(core::mem::zeroed()) as _, lpreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn SetFileAttributesFromAppW<P0>(lpfilename: P0, dwfileattributes: super::DWORD) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-core-file-fromapp-l1-1-0.dll" "system" fn SetFileAttributesFromAppW(lpfilename : windows_core::PCWSTR, dwfileattributes : super::DWORD) -> windows_core::BOOL);
    unsafe { SetFileAttributesFromAppW(lpfilename.param().abi(), dwfileattributes) }
}
