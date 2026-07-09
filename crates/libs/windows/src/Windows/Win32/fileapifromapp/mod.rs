#[inline]
pub unsafe fn CopyFileFromAppW<P0, P1>(lpexistingfilename: P0, lpnewfilename: P1, bfailifexists: bool) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-core-file-fromapp-l1-1-0.dll" "system" fn CopyFileFromAppW(lpexistingfilename : windows_core::PCWSTR, lpnewfilename : windows_core::PCWSTR, bfailifexists : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { CopyFileFromAppW(lpexistingfilename.param().abi(), lpnewfilename.param().abi(), bfailifexists.into()) }
}
#[cfg(feature = "Win32_minwinbase")]
#[inline]
pub unsafe fn CreateDirectoryFromAppW<P0>(lppathname: P0, lpsecurityattributes: Option<*const super::minwinbase::SECURITY_ATTRIBUTES>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-core-file-fromapp-l1-1-0.dll" "system" fn CreateDirectoryFromAppW(lppathname : windows_core::PCWSTR, lpsecurityattributes : *const super::minwinbase::SECURITY_ATTRIBUTES) -> windows_core::BOOL);
    unsafe { CreateDirectoryFromAppW(lppathname.param().abi(), lpsecurityattributes.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_fileapi", feature = "Win32_minwinbase", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn CreateFile2FromAppW<P0>(lpfilename: P0, dwdesiredaccess: u32, dwsharemode: u32, dwcreationdisposition: u32, pcreateexparams: Option<*const super::fileapi::CREATEFILE2_EXTENDED_PARAMETERS>) -> super::winnt::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-core-file-fromapp-l1-1-0.dll" "system" fn CreateFile2FromAppW(lpfilename : windows_core::PCWSTR, dwdesiredaccess : u32, dwsharemode : u32, dwcreationdisposition : u32, pcreateexparams : *const super::fileapi::CREATEFILE2_EXTENDED_PARAMETERS) -> super::winnt::HANDLE);
    unsafe { CreateFile2FromAppW(lpfilename.param().abi(), dwdesiredaccess, dwsharemode, dwcreationdisposition, pcreateexparams.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn CreateFileFromAppW<P0>(lpfilename: P0, dwdesiredaccess: u32, dwsharemode: u32, lpsecurityattributes: Option<*const super::minwinbase::SECURITY_ATTRIBUTES>, dwcreationdisposition: u32, dwflagsandattributes: u32, htemplatefile: Option<super::winnt::HANDLE>) -> super::winnt::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-core-file-fromapp-l1-1-0.dll" "system" fn CreateFileFromAppW(lpfilename : windows_core::PCWSTR, dwdesiredaccess : u32, dwsharemode : u32, lpsecurityattributes : *const super::minwinbase::SECURITY_ATTRIBUTES, dwcreationdisposition : u32, dwflagsandattributes : u32, htemplatefile : super::winnt::HANDLE) -> super::winnt::HANDLE);
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
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn FindFirstFileExFromAppW<P0>(lpfilename: P0, finfolevelid: super::minwinbase::FINDEX_INFO_LEVELS, lpfindfiledata: *mut core::ffi::c_void, fsearchop: super::minwinbase::FINDEX_SEARCH_OPS, lpsearchfilter: Option<*const core::ffi::c_void>, dwadditionalflags: u32) -> super::winnt::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-core-file-fromapp-l1-1-0.dll" "system" fn FindFirstFileExFromAppW(lpfilename : windows_core::PCWSTR, finfolevelid : super::minwinbase::FINDEX_INFO_LEVELS, lpfindfiledata : *mut core::ffi::c_void, fsearchop : super::minwinbase::FINDEX_SEARCH_OPS, lpsearchfilter : *const core::ffi::c_void, dwadditionalflags : u32) -> super::winnt::HANDLE);
    unsafe { FindFirstFileExFromAppW(lpfilename.param().abi(), finfolevelid, lpfindfiledata as _, fsearchop, lpsearchfilter.unwrap_or(core::mem::zeroed()) as _, dwadditionalflags) }
}
#[cfg(feature = "Win32_minwinbase")]
#[inline]
pub unsafe fn GetFileAttributesExFromAppW<P0>(lpfilename: P0, finfolevelid: super::minwinbase::GET_FILEEX_INFO_LEVELS, lpfileinformation: *mut core::ffi::c_void) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-core-file-fromapp-l1-1-0.dll" "system" fn GetFileAttributesExFromAppW(lpfilename : windows_core::PCWSTR, finfolevelid : super::minwinbase::GET_FILEEX_INFO_LEVELS, lpfileinformation : *mut core::ffi::c_void) -> windows_core::BOOL);
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
#[inline]
pub unsafe fn ReplaceFileFromAppW<P0, P1, P2>(lpreplacedfilename: P0, lpreplacementfilename: P1, lpbackupfilename: P2, dwreplaceflags: u32, lpexclude: Option<*const core::ffi::c_void>, lpreserved: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-core-file-fromapp-l1-1-0.dll" "system" fn ReplaceFileFromAppW(lpreplacedfilename : windows_core::PCWSTR, lpreplacementfilename : windows_core::PCWSTR, lpbackupfilename : windows_core::PCWSTR, dwreplaceflags : u32, lpexclude : *const core::ffi::c_void, lpreserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { ReplaceFileFromAppW(lpreplacedfilename.param().abi(), lpreplacementfilename.param().abi(), lpbackupfilename.param().abi(), dwreplaceflags, lpexclude.unwrap_or(core::mem::zeroed()) as _, lpreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetFileAttributesFromAppW<P0>(lpfilename: P0, dwfileattributes: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-core-file-fromapp-l1-1-0.dll" "system" fn SetFileAttributesFromAppW(lpfilename : windows_core::PCWSTR, dwfileattributes : u32) -> windows_core::BOOL);
    unsafe { SetFileAttributesFromAppW(lpfilename.param().abi(), dwfileattributes) }
}
